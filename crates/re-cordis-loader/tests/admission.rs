//! Admission, bounded input and callback containment.
use re_cordis_core::{BoxError, Event, Listener, Plugin, Runtime};
use re_cordis_loader::{
    Catalog, Config, EntrySpec, Group, Loader, MAX_ENTRIES, Scope, SourceDocument, SourceId,
};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    ops::ControlFlow,
    rc::Rc,
};
use tokio::sync::oneshot;
#[tokio::test(flavor = "current_thread")]
async fn retiring_entry_counts_toward_capacity_and_parent_disposal_keeps_shared_barrier() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let (started, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let gates = Rc::new(RefCell::new(Some((started, gate))));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", move |ctx, (): ()| {
                        let pause = gates.borrow_mut().take();
                        async move {
                            ctx.on_dispose(move || async move {
                                if let Some((started, gate)) = pause {
                                    started.send(()).unwrap();
                                    gate.await.unwrap();
                                }
                                Ok::<_, Infallible>(())
                            })?;
                            Ok::<_, BoxError>(())
                        }
                    }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let id = loader
                .create(EntrySpec::new("worker", Config::new(())))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            for _ in 1..MAX_ENTRIES {
                loader
                    .create(EntrySpec::new("worker", Config::new(())).disabled(true))
                    .unwrap();
            }
            let removed = loader.remove(&id).unwrap();
            entered.await.unwrap();
            assert!(
                loader
                    .create(EntrySpec::new("worker", Config::new(())).disabled(true))
                    .is_err()
            );
            release.send(()).unwrap();
            removed.await.unwrap();
            loader
                .create(EntrySpec::new("worker", Config::new(())).disabled(true))
                .unwrap();
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn runtime_group_capacity_preflight_preserves_all_existing_sources() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let children = (0..MAX_ENTRIES)
                .map(|_| EntrySpec::new("worker", Config::new(())).disabled(true))
                .collect();
            assert!(
                loader
                    .create(EntrySpec::group(Group::new(children)))
                    .is_err()
            );
            assert!(loader.entries().is_empty());
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn failing_scope_callback_is_contained_and_does_not_admit_a_partial_entry() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = re_cordis_core::ServiceKey::<()>::new("value");
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let scope =
                Scope::new().intercept_with::<(), (), _, BoxError>(&key, |_| panic!("scope panic"));
            let result = loader.create(EntrySpec::new("worker", Config::new(())).scope(scope));
            assert!(result.is_err());
            assert!(loader.entries().is_empty());
            assert_eq!(runtime.take_failures().len(), 1);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn failed_partial_apply_releases_listener_and_healthy_sibling_continues() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let hits = Rc::new(Cell::new(0));
            let event = Event::<()>::named("tick");
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "bad",
                    Plugin::new("bad", {
                        let event = event.clone();
                        let hits = hits.clone();
                        move |ctx, (): ()| {
                            let event = event.clone();
                            let hits = hits.clone();
                            async move {
                                ctx.on(
                                    &event,
                                    Listener::sync(move |_, _| {
                                        hits.set(hits.get() + 100);
                                        Ok::<_, Infallible>(ControlFlow::Continue(()))
                                    }),
                                )?;
                                Err::<(), BoxError>("bad setup".into())
                            }
                        }
                    }),
                )
                .unwrap();
            catalog
                .register(
                    "good",
                    Plugin::new("good", {
                        let event = event.clone();
                        let hits = hits.clone();
                        move |ctx, (): ()| {
                            let event = event.clone();
                            let hits = hits.clone();
                            async move {
                                ctx.on(
                                    &event,
                                    Listener::sync(move |_, _| {
                                        hits.set(hits.get() + 1);
                                        Ok::<_, Infallible>(ControlFlow::Continue(()))
                                    }),
                                )?;
                                Ok::<_, BoxError>(())
                            }
                        }
                    }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            loader
                .create(EntrySpec::new("bad", Config::new(())))
                .unwrap();
            loader
                .create(EntrySpec::new("good", Config::new(())))
                .unwrap();
            let report = loader.settle().unwrap().await.unwrap();
            assert_eq!(report.failures.len(), 1);
            root.emit(&event, (), Default::default())
                .unwrap()
                .await
                .unwrap();
            assert_eq!(hits.get(), 1);
            loader.dispose().unwrap().await.unwrap();
            root.emit(&event, (), Default::default())
                .unwrap()
                .await
                .unwrap();
            assert_eq!(hits.get(), 1);
        })
        .await
        .unwrap();
}
#[test]
fn json_depth_nodes_string_and_unknown_field_bounds_are_explicit() {
    let id = SourceId::new("memory");
    let text = format!("[{}]", "[".repeat(34) + "0" + &"]".repeat(34));
    assert!(SourceDocument::json(id.clone(), &text).is_err());
    let text = format!(r#"[{{"name":"worker","config":"{}"}}]"#, "a".repeat(65537));
    assert!(SourceDocument::json(id.clone(), &text).is_err());
    let text = format!("[{}]", "null,".repeat(65536) + "null");
    assert!(SourceDocument::json(id, &text).is_err());
}
