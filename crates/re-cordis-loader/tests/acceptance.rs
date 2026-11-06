//! Bounded loader integration acceptance for middleware, scopes and release.
use re_cordis_core::{BoxError, Event, Listener, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{
    Catalog, Config, EntrySpec, EntryState, Group, Loader, Origin, Scope, SourceDocument, SourceId,
};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    ops::ControlFlow,
    rc::Rc,
};
use tokio::sync::oneshot;

#[tokio::test(flavor = "current_thread")]
async fn direct_update_middleware_blocks_settlement_of_nested_grandchild() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let (entered_send, entered) = oneshot::channel();
        let (release, gate) = oneshot::channel();
        let gates = Rc::new(RefCell::new(Some((entered_send, gate))));
        let applied = Rc::new(RefCell::new(Vec::new()));
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", {
                    let applied = applied.clone();
                    move |ctx, value: u32| {
                        applied.borrow_mut().push(value);
                        let gates = gates.borrow_mut().take();
                        async move {
                            let gates = Rc::new(RefCell::new(gates));
                            ctx.on_update::<u32, _, _, BoxError>(move |_, _, next| {
                                let pause = gates.borrow_mut().take();
                                async move {
                                    if let Some((entered, gate)) = pause {
                                        entered.send(()).unwrap();
                                        gate.await.unwrap();
                                    }
                                    Ok(next.call().await?)
                                }
                            })?;
                            Ok::<_, BoxError>(())
                        }
                    }
                }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        loader
            .create(
                EntrySpec::group(Group::new(vec![
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(1_u32)).id("worker"),
                    ]))
                    .id("inner"),
                ]))
                .id("outer"),
            )
            .unwrap();
        let report = loader.settle().unwrap().await.unwrap();
        assert_eq!(report.entries.len(), 3);
        assert!(report.failures.is_empty());
        let id = loader.resolve("outer:inner:worker").unwrap();
        drop(
            loader
                .update(&id, EntrySpec::new("worker", Config::new(2_u32)))
                .unwrap(),
        );
        entered.await.unwrap();
        let mut settlement = Box::pin(loader.settle().unwrap());
        assert!(futures_util::poll!(&mut settlement).is_pending());
        assert_eq!(*applied.borrow(), [1]);
        release.send(()).unwrap();
        assert!(settlement.await.unwrap().failures.is_empty());
        assert_eq!(*applied.borrow(), [1, 2]);
        assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Active);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn verdict_is_invalidated_by_dependency_republication_and_explicit_update() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let key = ServiceKey::<bool>::new("enabled");
        let calls = Rc::new(Cell::new(0));
        let applies = Rc::new(Cell::new(0));
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", {
                    let applies = applies.clone();
                    move |_, (): ()| {
                        applies.set(applies.get() + 1);
                        async { Ok::<_, Infallible>(()) }
                    }
                }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let spec = EntrySpec::new("worker", Config::new(()))
            .injecting([key.required()])
            .condition({
                let calls = calls.clone();
                let key = key.clone();
                move |ctx| {
                    calls.set(calls.get() + 1);
                    Ok::<_, BoxError>(*ctx.get(&key)?.unwrap())
                }
            });
        let id = loader.create(spec.clone()).unwrap();
        loader.settle().unwrap().await.unwrap();
        assert_eq!(calls.get(), 0);
        let provider = root.provide(&key, false).unwrap();
        loader.settle().unwrap().await.unwrap();
        assert_eq!(calls.get(), 1);
        assert_eq!(applies.get(), 0);
        provider.withdraw().unwrap().await.unwrap();
        loader.settle().unwrap().await.unwrap();
        root.provide(&key, true).unwrap();
        loader.settle().unwrap().await.unwrap();
        assert_eq!(calls.get(), 2);
        assert_eq!(applies.get(), 1);
        loader
            .update(&id, spec.disabled(true))
            .unwrap()
            .await
            .unwrap();
        assert_eq!(calls.get(), 2);
        let enabled = loader.snapshot(&id).unwrap().options.disabled(false);
        loader.update(&id, enabled).unwrap().await.unwrap();
        assert_eq!(calls.get(), 3);
        assert_eq!(applies.get(), 2);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn repeated_realm_moves_join_self_provider_cleanup_in_original_view() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let dependency = ServiceKey::<()>::new("dependency");
        root.provide(&dependency, ()).unwrap();
        let own = ServiceKey::<u32>::new("self");
        let observed = Rc::new(RefCell::new(Vec::new()));
        let cleaned = Rc::new(RefCell::new(Vec::new()));
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", {
                    let own = own.clone();
                    let observed = observed.clone();
                    let cleaned = cleaned.clone();
                    move |ctx, value: u32| {
                        let own = own.clone();
                        let observed = observed.clone();
                        let cleaned = cleaned.clone();
                        async move {
                            ctx.provide(&own, value)?;
                            let slot = ctx.isolation(&own)?;
                            observed
                                .borrow_mut()
                                .push((slot.clone(), ctx.fiber_id().clone()));
                            let view = ctx.clone();
                            ctx.on_dispose(move || async move {
                                assert_eq!(view.isolation(&own)?, slot);
                                cleaned.borrow_mut().push(*view.get(&own)?.unwrap());
                                Ok::<_, BoxError>(())
                            })?;
                            Ok::<_, BoxError>(())
                        }
                    }
                })
                .injecting([dependency.required()]),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let a = loader
            .create(
                EntrySpec::group(Group::new(vec![]))
                    .scope(Scope::new().isolate(&own))
                    .id("a"),
            )
            .unwrap();
        let b = loader
            .create(
                EntrySpec::group(Group::new(vec![]))
                    .scope(Scope::new().isolate(&own))
                    .id("b"),
            )
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        let id = loader
            .create_in(
                Some(&a),
                0,
                EntrySpec::new("worker", Config::new(7_u32)).id("worker"),
            )
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        for destination in [Some(&b), None, Some(&a), Some(&b), None] {
            let old = loader.snapshot(&id).unwrap().fiber_id;
            loader
                .move_entry(&id, destination, 0)
                .unwrap()
                .await
                .unwrap();
            assert_ne!(loader.snapshot(&id).unwrap().fiber_id, old);
            assert_eq!(cleaned.borrow().len() + 1, observed.borrow().len());
        }
        loader.dispose().unwrap().await.unwrap();
        assert_eq!(*cleaned.borrow(), vec![7; 6]);
        assert!(root.registry().is_empty().unwrap());
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn exact_commit_payloads_cover_create_update_move_remove_and_source_suppression() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, _: u32| async { Ok::<_, Infallible>(()) }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let group = loader
            .create(EntrySpec::group(Group::new(vec![])).id("group"))
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        let records = Rc::new(RefCell::new(Vec::new()));
        root.on(
            &loader.commits(),
            Listener::sync({
                let records = records.clone();
                move |_, record| {
                    records.borrow_mut().push(record);
                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                }
            }),
        )
        .unwrap();
        let id = loader
            .create(EntrySpec::new("worker", Config::new(1_u32)).id("worker"))
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        loader
            .update(&id, EntrySpec::new("worker", Config::new(2_u32)))
            .unwrap()
            .await
            .unwrap();
        loader
            .move_entry(&id, Some(&group), 0)
            .unwrap()
            .await
            .unwrap();
        loader.remove(&id).unwrap().await.unwrap();
        loader
            .reconcile(SourceDocument::typed(
                SourceId::new("read-only"),
                vec![EntrySpec::new("worker", Config::new(3_u32)).id("source")],
            ))
            .unwrap()
            .await
            .unwrap();
        let records = records.borrow().clone();
        assert_eq!(records.len(), 4);
        assert!(
            records
                .iter()
                .all(|record| record.id == id && record.origin == Origin::Runtime)
        );
        assert!(records[0].old.is_none());
        assert!(records[0].group.is_none());
        assert_eq!(
            *records[0]
                .new
                .as_ref()
                .unwrap()
                .source
                .get::<u32>()
                .unwrap(),
            1
        );
        assert_eq!(
            *records[1]
                .old
                .as_ref()
                .unwrap()
                .source
                .get::<u32>()
                .unwrap(),
            1
        );
        assert_eq!(
            *records[1]
                .new
                .as_ref()
                .unwrap()
                .source
                .get::<u32>()
                .unwrap(),
            2
        );
        assert_eq!(records[2].old.as_ref().unwrap().parent, None);
        assert_eq!(records[2].group, Some(group.clone()));
        assert_eq!(records[2].new.as_ref().unwrap().parent, Some(group.clone()));
        assert_eq!(records[3].group, Some(group));
        assert!(records[3].new.is_none());
        drop(records);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn hundred_cycles_restore_registry_effect_baselines_and_release_native_captures() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let event = Event::<()>::named("cycle");
        let calls = Rc::new(Cell::new(0));
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", {
                    let event = event.clone();
                    let calls = calls.clone();
                    move |ctx, capture: Rc<()>| {
                        let event = event.clone();
                        let calls = calls.clone();
                        async move {
                            ctx.on(
                                &event,
                                Listener::sync(move |_, _| {
                                    calls.set(calls.get() + 1);
                                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                                }),
                            )?;
                            ctx.on_dispose_sync(move || {
                                drop(capture);
                                Ok::<_, Infallible>(())
                            })?;
                            Ok::<_, BoxError>(())
                        }
                    }
                }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        loader.settle().unwrap().await.unwrap();
        let effects = root.get_effects().unwrap().len();
        let groups = root.registry().len().unwrap();
        for _ in 0..100 {
            let capture = Rc::new(());
            let weak = Rc::downgrade(&capture);
            let id = loader
                .create(EntrySpec::new("worker", Config::new(capture.clone())))
                .unwrap();
            drop(capture);
            loader.settle().unwrap().await.unwrap();
            root.emit(&event, (), Default::default())
                .unwrap()
                .await
                .unwrap();
            loader.remove(&id).unwrap().await.unwrap();
            loader.settle().unwrap().await.unwrap();
            assert!(weak.upgrade().is_none());
            assert_eq!(root.get_effects().unwrap().len(), effects);
            assert_eq!(root.registry().len().unwrap(), groups);
        }
        assert_eq!(calls.get(), 100);
        root.emit(&event, (), Default::default())
            .unwrap()
            .await
            .unwrap();
        assert_eq!(calls.get(), 100);
        loader.dispose().unwrap().await.unwrap();
        drop(loader);
        root.drain().unwrap().await.unwrap();
        assert!(root.get_effects().unwrap().is_empty());
        assert!(root.registry().is_empty().unwrap());
    })
    .await;
}

async fn bounded<T>(runtime: &Runtime, future: impl std::future::Future<Output = T>) {
    tokio::time::timeout(std::time::Duration::from_secs(5), runtime.run(future))
        .await
        .expect("loader scenario exceeded five seconds")
        .expect("Cordis host failed");
}
