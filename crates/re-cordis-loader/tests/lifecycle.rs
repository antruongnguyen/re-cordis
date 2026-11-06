//! Exact lifecycle attribution and retained provider move regressions.
use re_cordis_core::{BoxError, Listener, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, Group, Loader, Scope};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    ops::ControlFlow,
    rc::Rc,
};
use tokio::sync::oneshot;
#[tokio::test(flavor = "current_thread")]
async fn nested_child_disposal_is_not_entry_disposal_but_exact_inner_disposal_commits_disabled() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let child = Plugin::new("child", |_, (): ()| async { Ok::<_, Infallible>(()) });
            let worker = Plugin::new("worker", {
                let child = child.clone();
                move |ctx, (): ()| {
                    let child = child.clone();
                    async move {
                        ctx.plugin(&child, ())?.ready()?.await?;
                        Ok::<_, BoxError>(())
                    }
                }
            });
            let mut catalog = Catalog::new();
            catalog.register("worker", worker.clone()).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let id = loader
                .create(EntrySpec::new("worker", Config::new(())))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let changes = Rc::new(RefCell::new(Vec::new()));
            root.on(
                &loader.commits(),
                Listener::sync({
                    let changes = changes.clone();
                    move |_, record| {
                        changes.borrow_mut().push(record);
                        Ok::<_, Infallible>(ControlFlow::Continue(()))
                    }
                }),
            )
            .unwrap();
            for task in root.registry().remove(&child).unwrap() {
                task.await.unwrap();
            }
            loader.settle().unwrap().await.unwrap();
            assert!(!loader.snapshot(&id).unwrap().options.disabled);
            assert!(changes.borrow().is_empty());
            for task in root.registry().remove(&worker).unwrap() {
                task.await.unwrap();
            }
            tokio::task::yield_now().await;
            assert!(loader.snapshot(&id).unwrap().options.disabled);
            assert_eq!(changes.borrow().len(), 1);
            assert_eq!(changes.borrow()[0].id, id);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn consumer_retains_old_provider_until_cleanup_finishes_across_scope_move() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = ServiceKey::<u32>::new("value");
            let values = Rc::new(RefCell::new(Vec::new()));
            let cleaned = Rc::new(RefCell::new(Vec::new()));
            let (started, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let gates = Rc::new(RefCell::new(Some((started, gate))));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "provider",
                    Plugin::new("provider", {
                        let key = key.clone();
                        move |ctx, v: u32| {
                            let key = key.clone();
                            async move {
                                ctx.provide(&key, v)?;
                                Ok::<_, BoxError>(())
                            }
                        }
                    }),
                )
                .unwrap();
            catalog
                .register(
                    "consumer",
                    Plugin::new("consumer", {
                        let key = key.clone();
                        let values = values.clone();
                        let cleaned = cleaned.clone();
                        move |ctx, (): ()| {
                            let key = key.clone();
                            let values = values.clone();
                            let cleaned = cleaned.clone();
                            let pause = gates.borrow_mut().take();
                            async move {
                                let old = *ctx.get(&key)?.unwrap();
                                values.borrow_mut().push(old);
                                let cleanup_ctx = ctx.clone();
                                ctx.on_dispose(move || async move {
                                    if let Some((started, gate)) = pause {
                                        started.send(()).unwrap();
                                        gate.await.unwrap();
                                    }
                                    cleaned.borrow_mut().push(*cleanup_ctx.get(&key)?.unwrap());
                                    Ok::<_, BoxError>(())
                                })?;
                                Ok::<_, BoxError>(())
                            }
                        }
                    })
                    .injecting([key.required()]),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            loader
                .create(EntrySpec::new("provider", Config::new(1_u32)))
                .unwrap();
            let group = loader
                .create(
                    EntrySpec::group(Group::new(vec![EntrySpec::new(
                        "provider",
                        Config::new(2_u32),
                    )]))
                    .scope(Scope::new().isolate(&key)),
                )
                .unwrap();
            let id = loader
                .create(EntrySpec::new("consumer", Config::new(())))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let moved = loader.move_entry(&id, Some(&group), 0).unwrap();
            entered.await.unwrap();
            assert_eq!(*values.borrow(), [1]);
            release.send(()).unwrap();
            moved.await.unwrap();
            assert_eq!(*values.borrow(), [1, 2]);
            assert_eq!(*cleaned.borrow(), [1]);
            loader.dispose().unwrap().await.unwrap();
            assert_eq!(*cleaned.borrow(), [1, 2]);
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn removal_from_create_commit_callback_cannot_leave_resurrected_work() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
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
            root.on(
                &loader.commits(),
                Listener::sync({
                    let loader = loader.clone();
                    move |_, record: re_cordis_loader::CommitRecord| {
                        if record.old.is_none() {
                            drop(loader.remove(&record.id).unwrap());
                        }
                        Ok::<_, Infallible>(ControlFlow::Continue(()))
                    }
                }),
            )
            .unwrap();
            let id = loader
                .create(EntrySpec::new("worker", Config::new(())))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            assert!(loader.snapshot(&id).is_none());
            assert_eq!(applies.get(), 0);
            loader.dispose().unwrap().await.unwrap();
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
