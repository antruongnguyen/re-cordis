//! Loader update and latest-intent acceptance.
use re_cordis_core::{BoxError, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Loader};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    rc::Rc,
};
use tokio::sync::oneshot;

#[tokio::test(flavor = "current_thread")]
async fn pending_validation_failed_recovery_and_invalid_loaded_source() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = ServiceKey::<()>::new("settings");
            let validations = Rc::new(Cell::new(0));
            let applied = Rc::new(RefCell::new(Vec::new()));
            let plugin = Plugin::validated(
                "worker",
                {
                    let n = validations.clone();
                    move |v: &i32| {
                        n.set(n.get() + 1);
                        u32::try_from(*v)
                    }
                },
                {
                    let applied = applied.clone();
                    move |_, v: u32| {
                        applied.borrow_mut().push(v);
                        async { Ok::<_, Infallible>(()) }
                    }
                },
            )
            .injecting([key.required()]);
            let mut catalog = Catalog::new();
            catalog.register("worker", plugin).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let id = loader
                .create(EntrySpec::new("worker", Config::new(-1_i32)))
                .unwrap();
            assert_eq!(
                loader.settle().unwrap().await.unwrap().entries[0].state,
                EntryState::Pending
            );
            assert_eq!(validations.get(), 0);
            root.provide(&key, ()).unwrap();
            assert_eq!(loader.settle().unwrap().await.unwrap().failures.len(), 1);
            loader
                .update(&id, EntrySpec::new("worker", Config::new(2_i32)))
                .unwrap()
                .await
                .unwrap();
            assert_eq!(*applied.borrow(), [2]);
            assert!(
                loader
                    .update(&id, EntrySpec::new("worker", Config::new(-2_i32)))
                    .is_err()
            );
            assert_eq!(
                *loader.snapshot(&id).unwrap().source.get::<i32>().unwrap(),
                2
            );
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn latest_rename_and_remove_while_cleanup_is_gated() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let names = Rc::new(RefCell::new(Vec::new()));
            let (started, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let gates = Rc::new(RefCell::new(Some((started, gate))));
            let mut catalog = Catalog::new();
            for name in ["a", "b", "c"] {
                let names = names.clone();
                let gates = gates.clone();
                catalog
                    .register(
                        name,
                        Plugin::new(name, move |ctx, (): ()| {
                            names.borrow_mut().push(name);
                            let pause = if name == "a" {
                                gates.borrow_mut().take()
                            } else {
                                None
                            };
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
            }
            let loader = Loader::new(&root, catalog).unwrap();
            let id = loader.create(EntrySpec::new("a", Config::new(()))).unwrap();
            loader.settle().unwrap().await.unwrap();
            let first = loader
                .update(&id, EntrySpec::new("b", Config::new(())))
                .unwrap();
            entered.await.unwrap();
            let second = loader
                .update(&id, EntrySpec::new("c", Config::new(())))
                .unwrap();
            release.send(()).unwrap();
            first.await.unwrap();
            second.await.unwrap();
            assert_eq!(*names.borrow(), ["a", "c"]);
            let third = loader
                .update(&id, EntrySpec::new("b", Config::new(())))
                .unwrap();
            drop(loader.remove(&id).unwrap());
            third.await.unwrap();
            loader.settle().unwrap().await.unwrap();
            assert!(loader.snapshot(&id).is_none());
            loader.dispose().unwrap().await.unwrap();
            assert!(root.registry().is_empty().unwrap());
        })
        .await
        .unwrap();
}
