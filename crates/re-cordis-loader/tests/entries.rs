//! Catalog and eager entry foundation.
use re_cordis_core::{Plugin, Runtime};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Loader, LoaderError};
use std::{cell::Cell, convert::Infallible, rc::Rc};

#[tokio::test(flavor = "current_thread")]
async fn eager_admission_identity_checked_configs_and_joined_removal() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let releases = Rc::new(Cell::new(0));
            let worker = Plugin::new("worker", {
                let releases = releases.clone();
                move |ctx, value: u32| {
                    let releases = releases.clone();
                    async move {
                        assert_eq!(value, 3);
                        ctx.on_dispose_sync(move || {
                            releases.set(releases.get() + 1);
                            Ok::<_, Infallible>(())
                        })?;
                        Ok::<_, re_cordis_core::AdmissionError>(())
                    }
                }
            });
            let mut catalog = Catalog::new();
            catalog.register("worker", worker.clone()).unwrap();
            assert!(matches!(
                catalog.register("worker", worker.clone()),
                Err(LoaderError::DuplicateFactory(_))
            ));
            let loader = Loader::new(&root, catalog).unwrap();
            assert!(matches!(
                loader.create(EntrySpec::new("missing", Config::new(()))),
                Err(LoaderError::UnknownFactory(_))
            ));
            assert!(matches!(
                loader.create(EntrySpec::new("worker", Config::new("wrong"))),
                Err(LoaderError::Decode { .. })
            ));
            let a = loader
                .create(EntrySpec::new("worker", Config::new(3_u32)).id("entry-1"))
                .unwrap();
            assert!(matches!(
                loader.create(EntrySpec::new("worker", Config::new(3_u32)).id("entry-1")),
                Err(LoaderError::DuplicateEntry(_))
            ));
            let b = loader
                .create(EntrySpec::new("worker", Config::new(3_u32)))
                .unwrap();
            assert_ne!(a, b);
            let report = loader.settle().unwrap().await.unwrap();
            assert!(report.failures.is_empty());
            assert!(report.entries.iter().all(|e| e.state == EntryState::Active));
            assert_eq!(root.registry().instances(&worker).unwrap().len(), 2);
            assert_eq!(
                loader.snapshot(&a).unwrap().factory_id,
                loader.snapshot(&b).unwrap().factory_id
            );
            drop(loader.remove(&a).unwrap());
            loader.settle().unwrap().await.unwrap();
            assert_eq!(releases.get(), 1);
            loader.dispose().unwrap().await.unwrap();
            assert_eq!(releases.get(), 2);
            assert!(root.registry().is_empty().unwrap());
        })
        .await
        .unwrap();
}
