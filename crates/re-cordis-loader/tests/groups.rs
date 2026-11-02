//! Structural groups, moves and entry evaluation.
use re_cordis_core::{BoxError, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Group, Loader, Scope};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    rc::Rc,
};
#[tokio::test(flavor = "current_thread")]
async fn disabled_groups_are_structural_moves_adopt_children_and_reorder_is_inert() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let applied = Rc::new(Cell::new(0));
            let releases = Rc::new(Cell::new(0));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", {
                        let applied = applied.clone();
                        let releases = releases.clone();
                        move |ctx, (): ()| {
                            applied.set(applied.get() + 1);
                            let releases = releases.clone();
                            async move {
                                ctx.on_dispose_sync(move || {
                                    releases.set(releases.get() + 1);
                                    Ok::<_, Infallible>(())
                                })?;
                                Ok::<_, BoxError>(())
                            }
                        }
                    }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let a = loader
                .create(
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(())).id("child"),
                    ]))
                    .id("a")
                    .disabled(true),
                )
                .unwrap();
            let b = loader
                .create(EntrySpec::group(Group::new(vec![])).id("b"))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let child = loader.resolve("a:child").unwrap();
            assert_eq!(loader.snapshot(&a).unwrap().state, EntryState::Active);
            assert_eq!(loader.snapshot(&child).unwrap().state, EntryState::Disabled);
            assert_eq!(applied.get(), 0);
            loader
                .move_entry(&child, Some(&b), 0)
                .unwrap()
                .await
                .unwrap();
            assert_eq!(applied.get(), 1);
            let before = loader.snapshot(&child).unwrap().fiber_id;
            loader
                .move_entry(&child, Some(&b), 0)
                .unwrap()
                .await
                .unwrap();
            assert_eq!(loader.snapshot(&child).unwrap().fiber_id, before);
            loader.remove(&a).unwrap().await.unwrap();
            assert!(loader.snapshot(&child).is_some());
            loader.dispose().unwrap().await.unwrap();
            assert_eq!(releases.get(), 1);
            assert!(root.registry().is_empty().unwrap());
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn entry_injection_and_plugin_mapping_use_their_own_checked_context() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let entry_key = ServiceKey::<u32>::new("entry");
            let plugin_key = ServiceKey::<u32>::new("plugin");
            let mapped = Rc::new(RefCell::new(Vec::new()));
            let seen = Rc::new(RefCell::new(Vec::new()));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", {
                        let seen = seen.clone();
                        move |_, v: u32| {
                            seen.borrow_mut().push(v);
                            async { Ok::<_, Infallible>(()) }
                        }
                    })
                    .injecting([plugin_key.required()]),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let condition_count = Rc::new(Cell::new(0));
            let spec = EntrySpec::new("worker", Config::new(1_u32))
                .injecting([entry_key.required()])
                .condition({
                    let key = entry_key.clone();
                    let n = condition_count.clone();
                    move |ctx| {
                        n.set(n.get() + 1);
                        Ok::<_, BoxError>(*ctx.get(&key)?.unwrap() > 0)
                    }
                })
                .map_config::<u32, _, BoxError>({
                    let key = plugin_key.clone();
                    let mapped = mapped.clone();
                    move |ctx, v| {
                        mapped.borrow_mut().push(ctx.fiber_id().clone());
                        Ok(v + *ctx.get(&key)?.unwrap())
                    }
                });
            let id = loader.create(spec).unwrap();
            loader.settle().unwrap().await.unwrap();
            assert_eq!(condition_count.get(), 0);
            root.provide(&entry_key, 1).unwrap();
            loader.settle().unwrap().await.unwrap();
            assert_eq!(condition_count.get(), 1);
            assert!(seen.borrow().is_empty());
            root.provide(&plugin_key, 4).unwrap();
            loader.settle().unwrap().await.unwrap();
            assert_eq!(*seen.borrow(), [5]);
            assert_eq!(
                mapped.borrow()[0],
                loader.snapshot(&id).unwrap().fiber_id.unwrap()
            );
            assert_eq!(
                *loader.snapshot(&id).unwrap().source.get::<u32>().unwrap(),
                1
            );
            assert_eq!(
                *loader
                    .snapshot(&id)
                    .unwrap()
                    .evaluated
                    .unwrap()
                    .get::<u32>()
                    .unwrap(),
                5
            );
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn typed_isolation_moves_preserve_entry_identity_and_rebuild_fiber() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = ServiceKey::<u32>::new("value");
            let scope = Scope::new().isolate(&key);
            let values = Rc::new(RefCell::new(Vec::new()));
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
                        move |ctx, (): ()| {
                            values.borrow_mut().push(*ctx.get(&key).unwrap().unwrap());
                            async { Ok::<_, Infallible>(()) }
                        }
                    })
                    .injecting([key.required()]),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            loader
                .create(EntrySpec::new("provider", Config::new(7_u32)))
                .unwrap();
            let group = loader
                .create(
                    EntrySpec::group(Group::new(vec![EntrySpec::new(
                        "provider",
                        Config::new(9_u32),
                    )]))
                    .scope(scope),
                )
                .unwrap();
            let consumer = loader
                .create(EntrySpec::new("consumer", Config::new(())))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let before = loader.snapshot(&consumer).unwrap().fiber_id;
            loader
                .move_entry(&consumer, Some(&group), 0)
                .unwrap()
                .await
                .unwrap();
            assert_eq!(*values.borrow(), [7, 9]);
            assert_ne!(before, loader.snapshot(&consumer).unwrap().fiber_id);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
