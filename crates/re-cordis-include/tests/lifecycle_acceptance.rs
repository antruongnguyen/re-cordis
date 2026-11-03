//! Independent host, replacement, and joined ownership acceptance.
mod support;
use futures_util::FutureExt;
use re_cordis_core::{BoxError, DispatchOptions, Event, Listener, Plugin, Runtime, ServiceKey};
use re_cordis_include::{Include, IncludeOptions};
use re_cordis_loader::Catalog;
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    ops::ControlFlow,
    rc::Rc,
};
use support::*;
use tokio::sync::oneshot;

#[tokio::test(flavor = "current_thread")]
async fn wrong_host_and_inactive_owner_are_refused_before_file_access() {
    let d = Dir::new();
    let rt = Runtime::new();
    let other = Runtime::new();
    let include = rt
        .run(async {
            Include::new(
                &rt.context(),
                catalog(),
                IncludeOptions::new(d.path("missing.json")).initial(serde_json::json!([])),
            )
            .unwrap()
        })
        .await
        .unwrap();
    let error = include.refresh().err().unwrap();
    assert!(error.location.is_none());
    assert!(!d.path("missing.json").exists());
    other
        .run(async {
            assert!(
                Include::new(
                    &rt.context(),
                    catalog(),
                    IncludeOptions::new(d.path("missing.json"))
                )
                .is_err()
            );
            let error = include.refresh().err().unwrap();
            assert!(error.location.is_none());
            assert!(!error.message.contains("No such file"));
            assert!(include.flush().is_err());
            assert!(!d.path("missing.json").exists());
        })
        .await
        .unwrap();
    run(&rt, async {
        let owner = rt
            .context()
            .plugin(
                &Plugin::new("parent", |_, ()| async { Ok::<_, Infallible>(()) }),
                (),
            )
            .unwrap();
        owner.ready().unwrap().await.unwrap();
        let stale = owner.context();
        owner.dispose().unwrap().await.unwrap();
        assert!(
            Include::new(
                &stale,
                catalog(),
                IncludeOptions::new(d.path("missing.json"))
            )
            .is_err()
        );
        include.dispose().unwrap().await.unwrap();
        assert!(include.refresh().is_err());
        assert!(include.flush().is_err());
        assert!(!d.path("missing.json").exists());
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn decoder_reentry_is_refused_and_file_replacement_during_apply_is_refreshable() {
    let d = Dir::new();
    d.put(
        "root.json",
        r#"[{"id":"worker","name":"worker","config":1}]"#,
    );
    let rt = Runtime::new();
    run(&rt, async {
        let slot = Rc::new(RefCell::new(None::<Include>));
        let refused = Rc::new(Cell::new(0));
        let applied = Rc::new(RefCell::new(Vec::new()));
        let path = d.path("root.json");
        let mut c = Catalog::new();
        c.register_json(
            "worker",
            Plugin::new("worker", {
                let path = path.clone();
                let applied = applied.clone();
                move |_, value: u32| {
                    applied.borrow_mut().push(value);
                    if value == 1 {
                        std::fs::write(&path, r#"[{"id":"worker","name":"worker","config":2}]"#)
                            .unwrap();
                    }
                    async { Ok::<_, Infallible>(()) }
                }
            }),
            {
                let slot = slot.clone();
                let refused = refused.clone();
                move |v| {
                    let include = slot.borrow().as_ref().unwrap().clone();
                    let error = include
                        .refresh()
                        .err()
                        .expect("outer decode must exclude reentry");
                    assert!(error.message.contains("progress"));
                    refused.set(refused.get() + 1);
                    serde_json::from_value::<u32>(v.cloned().unwrap())
                }
            },
        )
        .unwrap();
        let include = Include::new(&rt.context(), c, IncludeOptions::new(path)).unwrap();
        *slot.borrow_mut() = Some(include.clone());
        assert!(
            include
                .refresh()
                .unwrap()
                .await
                .unwrap()
                .failures
                .is_empty()
        );
        assert_eq!(&*applied.borrow(), &[1]);
        assert!(include.refresh().unwrap().await.unwrap().changed);
        assert_eq!(&*applied.borrow(), &[1, 2]);
        assert!(refused.get() >= 2);
        slot.borrow_mut().take();
        include.dispose().unwrap().await.unwrap();
        assert!(rt.take_failures().is_empty());
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn repeated_nested_source_removal_and_restore_release_each_runtime_subtree() {
    let d = Dir::new();
    d.put("child.json", r#"[{"id":"child","name":"worker"}]"#);
    let graph = r#"[{"id":"group","group":true,"config":[{"id":"include","name":"include","config":{"path":"child.json"}}]}]"#;
    d.put("root.json", graph);
    let rt = Runtime::new();
    run(&rt, async {
        let root = rt.context();
        let include =
            Include::new(&root, catalog(), IncludeOptions::new(d.path("root.json"))).unwrap();
        for _ in 0..10 {
            assert!(
                include
                    .refresh()
                    .unwrap()
                    .await
                    .unwrap()
                    .failures
                    .is_empty()
            );
            assert_eq!(include.entries().len(), 3);
            assert!(!include.refresh().unwrap().await.unwrap().changed);
            std::fs::remove_file(d.path("child.json")).unwrap();
            assert!(include.refresh().is_err());
            assert_eq!(include.entries().len(), 3);
            d.put("root.json", "[]");
            assert!(
                include
                    .refresh()
                    .unwrap()
                    .await
                    .unwrap()
                    .failures
                    .is_empty()
            );
            assert!(include.entries().is_empty());
            assert!(include.ownership("child").is_none());
            d.put("child.json", r#"[{"id":"child","name":"worker"}]"#);
            d.put("root.json", graph);
        }
        include.dispose().unwrap().await.unwrap();
        assert!(root.registry().is_empty().unwrap());
        assert!(root.get_effects().unwrap().is_empty());
        assert!(rt.take_failures().is_empty());
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn slow_cleanup_survives_dropped_waiter_and_releases_owned_captures() {
    let d = Dir::new();
    d.put("root.json", r#"[{"id":"worker","name":"worker"}]"#);
    let rt = Runtime::new();
    run(&rt, async {
        let (send_entered, entered) = oneshot::channel();
        let (release, gate) = oneshot::channel();
        let gates = Rc::new(RefCell::new(Some((send_entered, gate))));
        let weak = Rc::new(RefCell::new(None));
        let cleaned = Rc::new(Cell::new(false));
        let mut c = Catalog::new();
        c.register_json(
            "worker",
            Plugin::new("worker", {
                let gates = gates.clone();
                let weak = weak.clone();
                let cleaned = cleaned.clone();
                move |ctx, _: ()| {
                    let gates = gates.borrow_mut().take().unwrap();
                    let capture = Rc::new(());
                    *weak.borrow_mut() = Some(Rc::downgrade(&capture));
                    let cleaned = cleaned.clone();
                    async move {
                        ctx.on_dispose(move || async move {
                            gates.0.send(()).unwrap();
                            gates.1.await.unwrap();
                            drop(capture);
                            cleaned.set(true);
                            Ok::<_, Infallible>(())
                        })?;
                        Ok::<_, BoxError>(())
                    }
                }
            }),
            |_| Ok::<_, Infallible>(()),
        )
        .unwrap();
        let include =
            Include::new(&rt.context(), c, IncludeOptions::new(d.path("root.json"))).unwrap();
        include.refresh().unwrap().await.unwrap();
        let mut joined = include.dispose().unwrap();
        entered.await.unwrap();
        assert!((&mut joined).now_or_never().is_none());
        assert!(!cleaned.get());
        assert!(weak.borrow().as_ref().unwrap().upgrade().is_some());
        assert!(include.refresh().is_err());
        drop(joined);
        release.send(()).unwrap();
        rt.context().drain().unwrap().await.unwrap();
        assert!(cleaned.get());
        assert!(weak.borrow().as_ref().unwrap().upgrade().is_none());
        assert!(rt.context().registry().is_empty().unwrap());
        assert!(rt.context().get_effects().unwrap().is_empty());
        assert!(rt.take_failures().is_empty());
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn one_hundred_create_use_dispose_cycles_restore_all_public_baselines() {
    let d = Dir::new();
    d.put("root.json", r#"[{"id":"worker","name":"worker"}]"#);
    let rt = Runtime::new();
    run(&rt, async {
        let root = rt.context();
        let event = Event::<()>::symbol("include-owned");
        let key = ServiceKey::<()>::new("include-owned");
        let calls = Rc::new(Cell::new(0));
        let weak = Rc::new(RefCell::new(None));
        for cycle in 1..=100 {
            let mut c = Catalog::new();
            c.register_json(
                "worker",
                Plugin::new("worker", {
                    let event = event.clone();
                    let key = key.clone();
                    let calls = calls.clone();
                    let weak = weak.clone();
                    move |ctx, _: ()| {
                        let event = event.clone();
                        let key = key.clone();
                        let calls = calls.clone();
                        let capture = Rc::new(());
                        *weak.borrow_mut() = Some(Rc::downgrade(&capture));
                        async move {
                            ctx.provide(&key, ())?;
                            ctx.on(
                                &event,
                                Listener::sync(move |_, ()| {
                                    let _keep = &capture;
                                    calls.set(calls.get() + 1);
                                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                                }),
                            )?;
                            Ok::<_, BoxError>(())
                        }
                    }
                }),
                |_| Ok::<_, Infallible>(()),
            )
            .unwrap();
            let include = Include::new(&root, c, IncludeOptions::new(d.path("root.json"))).unwrap();
            assert!(
                include
                    .refresh()
                    .unwrap()
                    .await
                    .unwrap()
                    .failures
                    .is_empty()
            );
            root.emit(&event, (), DispatchOptions::default())
                .unwrap()
                .await
                .unwrap();
            assert_eq!(calls.get(), cycle);
            assert!(root.get_direct(&key).unwrap().is_some());
            include.dispose().unwrap().await.unwrap();
            assert!(root.registry().is_empty().unwrap(), "cycle {cycle}");
            assert!(root.get_effects().unwrap().is_empty(), "cycle {cycle}");
            assert!(root.get_direct(&key).unwrap().is_none(), "cycle {cycle}");
            assert!(
                weak.borrow().as_ref().unwrap().upgrade().is_none(),
                "cycle {cycle}"
            );
            root.emit(&event, (), DispatchOptions::default())
                .unwrap()
                .await
                .unwrap();
            assert_eq!(calls.get(), cycle);
        }
        assert!(rt.take_failures().is_empty());
    })
    .await;
}
