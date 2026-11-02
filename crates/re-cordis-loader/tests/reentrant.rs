//! Reentrant mutation and admission boundaries.
use re_cordis_core::{BoxError, Listener, Plugin, Runtime, hooks};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Loader};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    rc::Rc,
};
#[tokio::test(flavor = "current_thread")]
async fn inner_config_hook_removal_is_refused_before_entry_unlink() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let applied = Rc::new(Cell::new(0));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", {
                        let applied = applied.clone();
                        move |_, v: u32| {
                            assert_eq!(v, 1);
                            applied.set(applied.get() + 1);
                            async { Ok::<_, Infallible>(()) }
                        }
                    }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let id = loader
                .create(EntrySpec::new("worker", Config::new(1_u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let fiber = loader.snapshot(&id).unwrap().fiber_id.unwrap();
            root.on(
                &hooks::config(),
                Listener::around({
                    let loader = loader.clone();
                    let id = id.clone();
                    move |_, request: hooks::Config, next: re_cordis_core::Next<Config>| {
                        if request.fiber.id == fiber {
                            assert!(matches!(
                                loader.remove(&id),
                                Err(re_cordis_loader::LoaderError::Lifecycle(
                                    re_cordis_core::LifecycleError::Admission(
                                        re_cordis_core::AdmissionError::ReentrantDrain
                                    )
                                ))
                            ));
                        }
                        next.call()
                    }
                }),
            )
            .unwrap();
            drop(
                loader
                    .update(&id, EntrySpec::new("worker", Config::new(1_u32)))
                    .unwrap(),
            );
            loader.settle().unwrap().await.unwrap();
            assert!(loader.snapshot(&id).is_some());
            assert_eq!(applied.get(), 2);
            loader.dispose().unwrap().await.unwrap();
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn wrong_host_and_unhosted_mutations_are_typed_refusals() {
    let runtime = Runtime::new();
    let other = Runtime::new();
    let stored = Rc::new(RefCell::new(None));
    runtime
        .run(async {
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
                )
                .unwrap();
            *stored.borrow_mut() = Some(Loader::new(&runtime.context(), catalog).unwrap());
        })
        .await
        .unwrap();
    let loader = stored.borrow().clone().unwrap();
    assert!(
        loader
            .create(EntrySpec::new("worker", Config::new(())))
            .is_err()
    );
    other
        .run(async {
            assert!(
                loader
                    .create(EntrySpec::new("worker", Config::new(())))
                    .is_err()
            );
        })
        .await
        .unwrap();
    runtime
        .run(async {
            assert!(loader.entries().is_empty());
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn disabled_entry_never_evaluates_and_failed_condition_recovers_on_explicit_update() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let evaluations = Rc::new(Cell::new(0));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let spec = EntrySpec::new("worker", Config::new(()))
                .disabled(true)
                .condition({
                    let n = evaluations.clone();
                    move |_| {
                        n.set(n.get() + 1);
                        Err::<bool, BoxError>("condition failure".into())
                    }
                });
            let id = loader.create(spec.clone()).unwrap();
            loader.settle().unwrap().await.unwrap();
            assert_eq!(evaluations.get(), 0);
            let failed = loader.update(&id, spec.disabled(false)).unwrap();
            assert!(failed.await.is_err());
            assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Failed);
            loader
                .update(&id, EntrySpec::new("worker", Config::new(())))
                .unwrap()
                .await
                .unwrap();
            assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Active);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
