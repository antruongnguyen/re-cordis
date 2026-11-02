//! Source adoption and ordering through read-only reconciliation.
use re_cordis_core::{Plugin, Runtime};
use re_cordis_loader::{Catalog, Config, EntrySpec, Group, Loader, SourceDocument, SourceId};
use std::convert::Infallible;
#[tokio::test(flavor = "current_thread")]
async fn source_adoption_keeps_global_identity_and_old_group_cannot_remove_it() {
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
            let a = SourceId::new("a");
            let b = SourceId::new("b");
            loader
                .reconcile(SourceDocument::typed(
                    a.clone(),
                    vec![
                        EntrySpec::group(Group::new(vec![
                            EntrySpec::new("worker", Config::new(())).id("child"),
                        ]))
                        .id("group"),
                    ],
                ))
                .unwrap()
                .await
                .unwrap();
            let child = loader.resolve("group:child").unwrap();
            loader
                .reconcile(SourceDocument::typed(
                    b,
                    vec![EntrySpec::new("worker", Config::new(())).id("child")],
                ))
                .unwrap()
                .await
                .unwrap();
            assert_eq!(loader.resolve("child").unwrap(), child);
            loader
                .reconcile(SourceDocument::typed(a, vec![]))
                .unwrap()
                .await
                .unwrap();
            assert!(loader.snapshot(&child).is_some());
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn reconciliation_reorders_explicit_root_entries_without_changing_entry_ids() {
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
            let source = SourceId::new("source");
            let spec = |id: &str| EntrySpec::new("worker", Config::new(())).id(id);
            loader
                .reconcile(SourceDocument::typed(
                    source.clone(),
                    vec![spec("a"), spec("b")],
                ))
                .unwrap()
                .await
                .unwrap();
            let a = loader.resolve("a").unwrap();
            let b = loader.resolve("b").unwrap();
            loader
                .reconcile(SourceDocument::typed(source, vec![spec("b"), spec("a")]))
                .unwrap()
                .await
                .unwrap();
            assert_eq!(
                loader
                    .entries()
                    .into_iter()
                    .map(|e| e.id)
                    .collect::<Vec<_>>(),
                [b, a]
            );
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
