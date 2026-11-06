//! Typed heterogeneous catalog acceptance.
use re_cordis_core::{Plugin, Runtime, ServiceError};
use re_cordis_loader::{Catalog, Config, EntrySpec, Loader, LoaderError};
use std::{cell::Cell, convert::Infallible, rc::Rc};
#[derive(Clone)]
struct NativeConfig(Rc<Cell<u32>>);
#[tokio::test(flavor = "current_thread")]
async fn heterogeneous_native_configs_need_clone_but_not_serialize_or_send() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let value = Rc::new(Cell::new(0));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "number",
                    Plugin::new("number", |_, v: u32| async move {
                        assert_eq!(v, 3);
                        Ok::<_, Infallible>(())
                    }),
                )
                .unwrap();
            catalog
                .register(
                    "native",
                    Plugin::new("native", |_, cfg: NativeConfig| async move {
                        cfg.0.set(9);
                        Ok::<_, Infallible>(())
                    }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            loader
                .create(EntrySpec::new("number", Config::new(3_u32)))
                .unwrap();
            loader
                .create(EntrySpec::new(
                    "native",
                    Config::new(NativeConfig(value.clone())),
                ))
                .unwrap();
            let error = loader
                .create(EntrySpec::new("native", Config::new(3_u32)))
                .unwrap_err();
            let LoaderError::Decode { source, .. } = error else {
                panic!("wrong failure")
            };
            assert!(source.downcast_ref::<ServiceError>().is_some());
            assert!(loader.settle().unwrap().await.unwrap().failures.is_empty());
            assert_eq!(value.get(), 9);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[test]
fn catalog_name_limits_use_utf8_bytes_and_duplicate_registration_is_atomic() {
    let plugin = Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) });
    let mut catalog = Catalog::new();
    assert!(catalog.register("", plugin.clone()).is_err());
    assert!(catalog.register("x".repeat(257), plugin.clone()).is_err());
    let name = "λ".repeat(128);
    catalog.register(name.clone(), plugin.clone()).unwrap();
    assert!(
        matches!(catalog.register(name.clone(),plugin),Err(LoaderError::DuplicateFactory(actual))if actual==name)
    );
}
