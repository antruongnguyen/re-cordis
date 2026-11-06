//! Latest source, liveness and nested-host intent during joined reload cleanup.
use re_cordis_core::{BoxError, Plugin, Runtime};
use re_cordis_hmr::{FactoryReplacement, ReloadNode, ReloadPlan};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Group, Loader};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    rc::Rc,
};
use tokio::sync::oneshot;

enum Change {
    Config,
    Disable,
    Remove,
    Reenable,
    ParentReplacement,
    Dispose,
}
fn plan(factory: &str) -> ReloadPlan {
    ReloadPlan::new(vec![ReloadNode {
        name: factory.into(),
        factory: Some(factory.into()),
        dependencies: vec![],
    }])
    .unwrap()
}
async fn during_unload(change: Change) {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let (started, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let pause = Rc::new(RefCell::new(Some((started, gate))));
            let closed = Rc::new(Cell::new(false));
            let old = Plugin::new("old", {
                let pause = pause.clone();
                let closed = closed.clone();
                move |ctx, _: u32| {
                    let pause = pause.borrow_mut().take();
                    let closed = closed.clone();
                    async move {
                        ctx.on_dispose(move || async move {
                            if let Some((started, gate)) = pause {
                                started.send(()).unwrap();
                                gate.await.unwrap();
                            }
                            closed.set(true);
                            Ok::<_, Infallible>(())
                        })?;
                        Ok::<_, BoxError>(())
                    }
                }
            });
            let mut catalog = Catalog::new();
            catalog.register("worker", old).unwrap();
            catalog
                .register(
                    "host",
                    Plugin::new("host", |_, _: u32| async { Ok::<_, Infallible>(()) }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let parent = loader
                .create(
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(1u32)).id("child"),
                    ]))
                    .id("parent"),
                )
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let child = loader.resolve("parent:child").unwrap();
            let applied = Rc::new(RefCell::new(Vec::new()));
            let new = Plugin::new("new", {
                let applied = applied.clone();
                let closed = closed.clone();
                move |_, v: u32| {
                    assert!(closed.get(), "old physical release precedes replacement");
                    applied.borrow_mut().push(v);
                    async { Ok::<_, Infallible>(()) }
                }
            });
            let reload = plan("worker")
                .replace(
                    &loader,
                    &["worker".into()],
                    vec![FactoryReplacement::new("worker", new)],
                )
                .unwrap();
            entered.await.unwrap();
            let mut disposal = None;
            match change {
                Change::Config => drop(
                    loader
                        .update(&child, EntrySpec::new("worker", Config::new(9u32)))
                        .unwrap(),
                ),
                Change::Disable => drop(
                    loader
                        .update(
                            &child,
                            EntrySpec::new("worker", Config::new(9u32)).disabled(true),
                        )
                        .unwrap(),
                ),
                Change::Remove => drop(loader.remove(&child).unwrap()),
                Change::Reenable => {
                    drop(
                        loader
                            .update(
                                &child,
                                EntrySpec::new("worker", Config::new(7u32)).disabled(true),
                            )
                            .unwrap(),
                    );
                    drop(
                        loader
                            .update(&child, EntrySpec::new("worker", Config::new(9u32)))
                            .unwrap(),
                    );
                }
                Change::ParentReplacement => drop(
                    loader
                        .update(&parent, EntrySpec::new("host", Config::new(8u32)))
                        .unwrap(),
                ),
                Change::Dispose => disposal = Some(loader.dispose().unwrap()),
            }
            assert!(applied.borrow().is_empty());
            assert!(!closed.get());
            release.send(()).unwrap();
            if let Some(disposal) = disposal {
                drop(reload);
                disposal.await.unwrap();
                assert!(applied.borrow().is_empty());
                assert!(root.registry().is_empty().unwrap());
                return;
            }
            assert!(reload.await.unwrap().failures.is_empty());
            let current = loader.snapshot(&child);
            match change {
                Change::Config | Change::Reenable => {
                    assert_eq!(*applied.borrow(), [9]);
                    assert_eq!(current.unwrap().state, EntryState::Active);
                }
                Change::Disable => {
                    assert!(applied.borrow().is_empty());
                    assert_eq!(current.unwrap().state, EntryState::Disabled);
                }
                Change::Remove | Change::ParentReplacement => {
                    assert!(applied.borrow().is_empty());
                    assert!(current.is_none());
                }
                Change::Dispose => unreachable!(),
            }
            loader.dispose().unwrap().await.unwrap();
            assert!(runtime.take_failures().is_empty());
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn latest_config_wins_during_unload() {
    during_unload(Change::Config).await;
}
#[tokio::test(flavor = "current_thread")]
async fn disabled_entry_is_not_resurrected() {
    during_unload(Change::Disable).await;
}
#[tokio::test(flavor = "current_thread")]
async fn removed_entry_is_not_resurrected() {
    during_unload(Change::Remove).await;
}
#[tokio::test(flavor = "current_thread")]
async fn reenable_uses_latest_config_once() {
    during_unload(Change::Reenable).await;
}
#[tokio::test(flavor = "current_thread")]
async fn replaced_parent_does_not_resurrect_old_child() {
    during_unload(Change::ParentReplacement).await;
}
#[tokio::test(flavor = "current_thread")]
async fn owner_disposal_supersedes_started_replacement() {
    during_unload(Change::Dispose).await;
}

#[tokio::test(flavor = "current_thread")]
async fn recreated_identity_is_not_reloaded_a_second_time() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let (started, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let pause = Rc::new(RefCell::new(Some((started, gate))));
            let old = Plugin::new("old", move |ctx, _: u32| {
                let pause = pause.borrow_mut().take();
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
            });
            let mut catalog = Catalog::new();
            catalog.register("worker", old).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let child = loader
                .create(EntrySpec::new("worker", Config::new(1u32)).id("child"))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let seen = Rc::new(RefCell::new(Vec::new()));
            let next = Plugin::new("next", {
                let seen = seen.clone();
                move |_, v: u32| {
                    seen.borrow_mut().push(v);
                    async { Ok::<_, Infallible>(()) }
                }
            });
            let reload = plan("worker")
                .replace(
                    &loader,
                    &["worker".into()],
                    vec![FactoryReplacement::new("worker", next)],
                )
                .unwrap();
            entered.await.unwrap();
            drop(loader.remove(&child).unwrap());
            assert!(
                loader
                    .create(EntrySpec::new("worker", Config::new(9u32)).id("child"))
                    .is_err()
            );
            release.send(()).unwrap();
            reload.await.unwrap();
            loader.settle().unwrap().await.unwrap();
            let current = loader
                .create(EntrySpec::new("worker", Config::new(9u32)).id("child"))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            assert_eq!(current, child);
            assert_eq!(*seen.borrow(), [9]);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn explicit_host_candidate_rebuilds_nested_core_plugin_once() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let seen = Rc::new(RefCell::new(Vec::new()));
            let make = |revision| {
                let seen = seen.clone();
                Plugin::new("host", move |ctx, v: u32| {
                    let seen = seen.clone();
                    async move {
                        let child = Plugin::new("child", move |_, _: ()| {
                            seen.borrow_mut().push((revision, v));
                            async { Ok::<_, Infallible>(()) }
                        });
                        let middle = Plugin::new("middle", move |ctx, _: ()| {
                            let child = child.clone();
                            async move {
                                ctx.plugin(&child, ())?.ready()?.await?;
                                Ok::<_, BoxError>(())
                            }
                        });
                        ctx.plugin(&middle, ())?.ready()?.await?;
                        Ok::<_, BoxError>(())
                    }
                })
            };
            let mut catalog = Catalog::new();
            catalog.register("host", make(1)).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            loader
                .create(EntrySpec::new("host", Config::new(7u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let graph = ReloadPlan::new(vec![
                ReloadNode {
                    name: "host".into(),
                    factory: Some("host".into()),
                    dependencies: vec!["nested-unit".into()],
                },
                ReloadNode {
                    name: "nested-unit".into(),
                    factory: None,
                    dependencies: vec![],
                },
            ])
            .unwrap();
            graph
                .replace(
                    &loader,
                    &["nested-unit".into()],
                    vec![FactoryReplacement::new("host", make(2))],
                )
                .unwrap()
                .await
                .unwrap();
            assert_eq!(*seen.borrow(), [(1, 7), (2, 7)]);
            loader.dispose().unwrap().await.unwrap();
            assert!(root.registry().is_empty().unwrap());
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn second_reenable_init_supersedes_the_first_without_hmr_duplicate() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let (old_tx, old_entered) = oneshot::channel();
            let (old_release, old_gate) = oneshot::channel();
            let old_pause = Rc::new(RefCell::new(Some((old_tx, old_gate))));
            let old = Plugin::new("old", move |ctx, _: u32| {
                let pause = old_pause.borrow_mut().take();
                async move {
                    ctx.on_dispose(move || async move {
                        if let Some((tx, gate)) = pause {
                            tx.send(()).unwrap();
                            gate.await.unwrap();
                        }
                        Ok::<_, Infallible>(())
                    })?;
                    Ok::<_, BoxError>(())
                }
            });
            let mut catalog = Catalog::new();
            catalog.register("worker", old).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let entry = loader
                .create(EntrySpec::new("worker", Config::new(1u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let (new_tx, new_entered) = oneshot::channel();
            let (new_release, new_gate) = oneshot::channel();
            let new_pause = Rc::new(RefCell::new(Some((new_tx, new_gate))));
            let seen = Rc::new(RefCell::new(Vec::new()));
            let new = Plugin::new("new", {
                let seen = seen.clone();
                move |_, v: u32| {
                    let pause = new_pause.borrow_mut().take();
                    let seen = seen.clone();
                    async move {
                        seen.borrow_mut().push(v);
                        if let Some((tx, gate)) = pause {
                            tx.send(()).unwrap();
                            gate.await.unwrap();
                        }
                        Ok::<_, Infallible>(())
                    }
                }
            });
            let reload = plan("worker")
                .replace(
                    &loader,
                    &["worker".into()],
                    vec![FactoryReplacement::new("worker", new)],
                )
                .unwrap();
            old_entered.await.unwrap();
            drop(
                loader
                    .update(
                        &entry,
                        EntrySpec::new("worker", Config::new(7u32)).disabled(true),
                    )
                    .unwrap(),
            );
            drop(
                loader
                    .update(&entry, EntrySpec::new("worker", Config::new(7u32)))
                    .unwrap(),
            );
            old_release.send(()).unwrap();
            new_entered.await.unwrap();
            drop(
                loader
                    .update(
                        &entry,
                        EntrySpec::new("worker", Config::new(9u32)).disabled(true),
                    )
                    .unwrap(),
            );
            drop(
                loader
                    .update(&entry, EntrySpec::new("worker", Config::new(9u32)))
                    .unwrap(),
            );
            new_release.send(()).unwrap();
            assert!(reload.await.unwrap().failures.is_empty());
            assert_eq!(*seen.borrow(), [7, 9]);
            assert_eq!(
                *loader
                    .snapshot(&entry)
                    .unwrap()
                    .source
                    .get::<u32>()
                    .unwrap(),
                9
            );
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
