//! Nested structural verdicts and latest entry-injection rebuild.
use re_cordis_core::{BoxError, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Group, Loader, Scope};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    rc::Rc,
};
use tokio::sync::oneshot;
#[tokio::test(flavor = "current_thread")]
async fn all_nested_parent_child_disabled_combinations_keep_groups_structural() {
    for parent_disabled in [false, true] {
        for group_disabled in [false, true] {
            for child_disabled in [false, true] {
                let runtime = Runtime::new();
                runtime
                    .run(async {
                        let root = runtime.context();
                        let mut catalog = Catalog::new();
                        catalog
                            .register(
                                "worker",
                                Plugin::new("worker", |_, (): ()| async {
                                    Ok::<_, Infallible>(())
                                }),
                            )
                            .unwrap();
                        let loader = Loader::new(&root, catalog).unwrap();
                        let inner = EntrySpec::group(Group::new(vec![
                            EntrySpec::new("worker", Config::new(()))
                                .id("worker")
                                .disabled(child_disabled),
                        ]))
                        .id("inner")
                        .disabled(group_disabled);
                        let outer = loader
                            .create(
                                EntrySpec::group(Group::new(vec![inner]))
                                    .id("outer")
                                    .disabled(parent_disabled),
                            )
                            .unwrap();
                        loader.settle().unwrap().await.unwrap();
                        let inner = loader.resolve("outer:inner").unwrap();
                        let child = loader.resolve("outer:inner:worker").unwrap();
                        assert_eq!(loader.snapshot(&outer).unwrap().state, EntryState::Active);
                        assert_eq!(loader.snapshot(&inner).unwrap().state, EntryState::Active);
                        assert_eq!(
                            loader.snapshot(&child).unwrap().state,
                            if parent_disabled || group_disabled || child_disabled {
                                EntryState::Disabled
                            } else {
                                EntryState::Active
                            }
                        );
                        loader.dispose().unwrap().await.unwrap();
                        assert!(root.registry().is_empty().unwrap());
                    })
                    .await
                    .unwrap();
            }
        }
    }
}
#[tokio::test(flavor = "current_thread")]
async fn latest_entry_injection_wins_during_gated_old_cleanup() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = ServiceKey::<()>::new("settings");
            let applied = Rc::new(RefCell::new(Vec::new()));
            let (started, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let gates = Rc::new(RefCell::new(Some((started, gate))));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", {
                        let applied = applied.clone();
                        move |ctx, v: u32| {
                            applied.borrow_mut().push(v);
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
                        }
                    }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let id = loader
                .create(EntrySpec::new("worker", Config::new(0_u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let first = loader
                .update(
                    &id,
                    EntrySpec::new("worker", Config::new(1_u32)).injecting([key.required()]),
                )
                .unwrap();
            entered.await.unwrap();
            let second = loader
                .update(
                    &id,
                    EntrySpec::new("worker", Config::new(2_u32)).injecting([key.required()]),
                )
                .unwrap();
            let latest = loader
                .update(&id, EntrySpec::new("worker", Config::new(3_u32)))
                .unwrap();
            release.send(()).unwrap();
            first.await.unwrap();
            second.await.unwrap();
            latest.await.unwrap();
            assert_eq!(*applied.borrow(), [0, 3]);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn intercept_failure_waits_for_entry_injection_and_explicit_update_recovers() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = ServiceKey::<u32>::new("settings");
            let count = Rc::new(Cell::new(0));
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
            let intercept = Scope::new().intercept_with::<u32, u32, _, BoxError>(&key, {
                let count = count.clone();
                let key = key.clone();
                move |ctx| {
                    assert_eq!(*ctx.get(&key)?.unwrap(), 7);
                    count.set(count.get() + 1);
                    Err("intercept failure".into())
                }
            });
            let id = loader
                .create(
                    EntrySpec::new("worker", Config::new(()))
                        .injecting([key.required()])
                        .intercept(intercept),
                )
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            assert_eq!(count.get(), 0);
            root.provide(&key, 7).unwrap();
            let report = loader.settle().unwrap().await.unwrap();
            assert_eq!(report.failures.len(), 1);
            assert_eq!(applies.get(), 0);
            assert_eq!(count.get(), 1);
            loader
                .update(&id, EntrySpec::new("worker", Config::new(())))
                .unwrap()
                .await
                .unwrap();
            assert_eq!(applies.get(), 1);
            loader.dispose().unwrap().await.unwrap();
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
