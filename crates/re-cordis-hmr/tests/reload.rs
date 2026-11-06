//! Explicit application graphs and linked replacement lifecycle acceptance.
use re_cordis_core::{BoxError, Event, Listener, Plugin, Runtime, ServiceKey};
use re_cordis_hmr::{FactoryReplacement, ReloadNode, ReloadPlan};
use re_cordis_loader::{Catalog, Config, EntrySpec, Loader};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    io,
    ops::ControlFlow,
    rc::Rc,
    time::Duration,
};
use tokio::sync::oneshot;

fn node(name: &str, factory: Option<&str>, dependencies: &[&str]) -> ReloadNode {
    ReloadNode {
        name: name.into(),
        factory: factory.map(Into::into),
        dependencies: dependencies.iter().map(|name| (*name).into()).collect(),
    }
}
fn inert(name: &str) -> Plugin<u32> {
    Plugin::new(name, |_, _: u32| async { Ok::<_, Infallible>(()) })
}
#[test]
fn explicit_reverse_closure_preserves_unchanged_shared_dependencies_and_cycles() {
    let plan = ReloadPlan::new(vec![
        node("left", Some("a"), &["changed", "shared"]),
        node("right", Some("b"), &["shared"]),
        node("changed", None, &["cycle"]),
        node("cycle", None, &["changed"]),
        node("shared", None, &[]),
    ])
    .unwrap();
    assert_eq!(
        plan.affected(&["changed".into(), "changed".into()])
            .unwrap(),
        vec!["a"]
    );
    assert_eq!(plan.affected(&["shared".into()]).unwrap(), vec!["a", "b"]);
    assert!(plan.affected(&[]).unwrap().is_empty());
    assert!(plan.affected(&["unknown".into()]).is_err());
}
#[test]
fn malformed_graphs_and_bounds_reject_explicitly() {
    assert!(ReloadPlan::new(vec![node("", None, &[])]).is_err());
    assert!(ReloadPlan::new(vec![node("a", None, &["missing"])]).is_err());
    assert!(ReloadPlan::new(vec![node("a", None, &[]), node("a", None, &[])]).is_err());
    assert!(
        ReloadPlan::new(vec![
            node("a", Some("same"), &[]),
            node("b", Some("same"), &[])
        ])
        .is_err()
    );
    assert!(ReloadPlan::new(vec![node("a", Some(""), &[])]).is_err());
    assert!(ReloadPlan::new((0..257).map(|i| node(&i.to_string(), None, &[])).collect()).is_err());
    assert!(
        ReloadPlan::new(vec![ReloadNode {
            name: "a".into(),
            factory: None,
            dependencies: vec!["a".into(); 1025]
        }])
        .is_err()
    );
    assert!(ReloadPlan::new(vec![node(&"x".repeat(257), None, &[])]).is_err());
    let plan = ReloadPlan::new(vec![node("a", None, &[])]).unwrap();
    assert!(plan.affected(&vec!["a".into(); 257]).is_err());
}
#[tokio::test(flavor = "current_thread")]
async fn instances_keep_configs_and_candidate_coverage_is_exact() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", inert("old")).unwrap();
            catalog.register("sibling", inert("sibling")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let a = loader
                .create(EntrySpec::new("worker", Config::new(1u32)))
                .unwrap();
            loader
                .create(EntrySpec::new("worker", Config::new(2u32)))
                .unwrap();
            let sibling = loader
                .create(EntrySpec::new("sibling", Config::new(7u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let original = loader.snapshot(&a).unwrap().factory_id;
            let sibling_fiber = loader.snapshot(&sibling).unwrap().fiber_id;
            let plan = ReloadPlan::new(vec![
                node("worker", Some("worker"), &["dep", "shared"]),
                node("sibling", Some("sibling"), &["shared"]),
                node("dep", None, &[]),
                node("shared", None, &[]),
            ])
            .unwrap();
            assert!(plan.replace(&loader, &["dep".into()], vec![]).is_err());
            assert!(
                plan.replace(
                    &loader,
                    &["dep".into()],
                    vec![FactoryReplacement::new("sibling", inert("wrong"))]
                )
                .is_err()
            );
            assert!(
                plan.replace(
                    &loader,
                    &["dep".into()],
                    vec![
                        FactoryReplacement::new("worker", inert("new")),
                        FactoryReplacement::new("worker", inert("again"))
                    ]
                )
                .is_err()
            );
            assert_eq!(loader.snapshot(&a).unwrap().factory_id, original);
            let seen = Rc::new(RefCell::new(Vec::new()));
            let replacement = Plugin::new("new", {
                let seen = seen.clone();
                move |_, config: u32| {
                    seen.borrow_mut().push(config);
                    async { Ok::<_, Infallible>(()) }
                }
            });
            assert!(
                plan.replace(
                    &loader,
                    &["dep".into()],
                    vec![FactoryReplacement::new("worker", replacement)]
                )
                .unwrap()
                .await
                .unwrap()
                .failures
                .is_empty()
            );
            let mut actual = seen.borrow().clone();
            actual.sort();
            assert_eq!(actual, [1, 2]);
            assert_eq!(loader.snapshot(&sibling).unwrap().fiber_id, sibling_fiber);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn exact_provider_survives_gated_consumer_cleanup_while_unrelated_reload_progresses() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = ServiceKey::<u32>::new("version");
            let old_alive = Rc::new(Cell::new(false));
            let observed = Rc::new(RefCell::new(Vec::new()));
            let (entered_tx, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let pause = Rc::new(RefCell::new(Some((entered_tx, gate))));
            let provider = Plugin::new("provider", {
                let key = key.clone();
                let alive = old_alive.clone();
                move |ctx, v: u32| {
                    let key = key.clone();
                    let alive = alive.clone();
                    async move {
                        alive.set(true);
                        ctx.provide(&key, v)?;
                        ctx.on_dispose_sync(move || {
                            alive.set(false);
                            Ok::<_, Infallible>(())
                        })?;
                        Ok::<_, BoxError>(())
                    }
                }
            });
            let consumer = Plugin::new("consumer", {
                let key = key.clone();
                let pause = pause.clone();
                let alive = old_alive.clone();
                move |ctx, _: u32| {
                    let key = key.clone();
                    let pause = pause.borrow_mut().take();
                    let alive = alive.clone();
                    async move {
                        let cleanup = ctx.clone();
                        ctx.on_dispose(move || async move {
                            let before = cleanup.get(&key)?.unwrap();
                            assert_eq!(*before, 1);
                            assert!(alive.get());
                            if let Some((entered, gate)) = pause {
                                entered.send(()).unwrap();
                                gate.await.unwrap();
                            }
                            let after = cleanup.get(&key)?.unwrap();
                            assert!(Rc::ptr_eq(&before, &after));
                            assert!(alive.get());
                            Ok::<_, BoxError>(())
                        })?;
                        Ok::<_, BoxError>(())
                    }
                }
            })
            .injecting([key.required()]);
            let mut catalog = Catalog::new();
            catalog.register("provider", provider).unwrap();
            catalog.register("consumer", consumer).unwrap();
            catalog.register("fast", inert("old-fast")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            loader
                .create(EntrySpec::new("provider", Config::new(1u32)))
                .unwrap();
            loader
                .create(EntrySpec::new("consumer", Config::new(4u32)))
                .unwrap();
            loader
                .create(EntrySpec::new("fast", Config::new(8u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let (fast_tx, fast) = oneshot::channel();
            let fast_tx = Rc::new(RefCell::new(Some(fast_tx)));
            let fast_plugin = Plugin::new("new-fast", move |_, _: u32| {
                fast_tx.borrow_mut().take().unwrap().send(()).unwrap();
                async { Ok::<_, Infallible>(()) }
            });
            let next_provider = Plugin::new("new-provider", {
                let key = key.clone();
                let alive = old_alive.clone();
                move |ctx, _: u32| {
                    let key = key.clone();
                    let alive = alive.clone();
                    async move {
                        assert!(!alive.get());
                        ctx.provide(&key, 2)?;
                        Ok::<_, BoxError>(())
                    }
                }
            });
            let next_consumer = Plugin::new("new-consumer", {
                let key = key.clone();
                let observed = observed.clone();
                move |ctx, v: u32| {
                    observed
                        .borrow_mut()
                        .push((v, *ctx.get(&key).unwrap().unwrap()));
                    async { Ok::<_, Infallible>(()) }
                }
            })
            .injecting([key.required()]);
            let plan = ReloadPlan::new(vec![
                node("provider", Some("provider"), &[]),
                node("consumer", Some("consumer"), &["provider"]),
                node("fast", Some("fast"), &[]),
            ])
            .unwrap();
            let task = plan
                .replace(
                    &loader,
                    &["provider".into(), "fast".into()],
                    vec![
                        FactoryReplacement::new("provider", next_provider),
                        FactoryReplacement::new("consumer", next_consumer),
                        FactoryReplacement::new("fast", fast_plugin),
                    ],
                )
                .unwrap();
            entered.await.unwrap();
            tokio::time::timeout(Duration::from_secs(2), fast)
                .await
                .unwrap()
                .unwrap();
            assert!(old_alive.get());
            assert!(observed.borrow().is_empty());
            drop(task);
            release.send(()).unwrap();
            assert!(loader.settle().unwrap().await.unwrap().failures.is_empty());
            assert!(!old_alive.get());
            assert_eq!(*observed.borrow(), [(4, 2)]);
            loader.dispose().unwrap().await.unwrap();
            assert!(runtime.take_failures().is_empty());
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn post_unload_failure_keeps_source_and_later_candidate_recovers() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let entry = loader
                .create(EntrySpec::new("worker", Config::new(5u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let plan = ReloadPlan::new(vec![node("worker", Some("worker"), &[])]).unwrap();
            let failing = Plugin::new("failing", |_, _: u32| async {
                Err::<(), _>(io::Error::other("apply failed"))
            });
            let report = plan
                .replace(
                    &loader,
                    &["worker".into()],
                    vec![FactoryReplacement::new("worker", failing)],
                )
                .unwrap()
                .await
                .unwrap();
            assert_eq!(report.failures.len(), 1);
            assert!(
                report.failures[0]
                    .error
                    .to_string()
                    .contains("apply failed")
            );
            assert_eq!(
                *loader
                    .snapshot(&entry)
                    .unwrap()
                    .source
                    .get::<u32>()
                    .unwrap(),
                5
            );
            assert!(
                plan.replace(
                    &loader,
                    &["worker".into()],
                    vec![FactoryReplacement::new("worker", inert("recovered"))]
                )
                .unwrap()
                .await
                .unwrap()
                .failures
                .is_empty()
            );
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn one_hundred_replacements_restore_resource_and_capture_baselines() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let before = root.get_effects().unwrap().len();
            let event = Event::<()>::symbol("reload-tick");
            let applied = Rc::new(Cell::new(0));
            let released = Rc::new(Cell::new(0));
            let make = |capture: Rc<()>| {
                let applied = applied.clone();
                let released = released.clone();
                let event = event.clone();
                Plugin::new("resource", move |ctx, _: u32| {
                    let _ = &capture;
                    let applied = applied.clone();
                    let released = released.clone();
                    let event = event.clone();
                    async move {
                        applied.set(applied.get() + 1);
                        ctx.on(
                            &event,
                            Listener::sync(|_, _| Ok::<_, Infallible>(ControlFlow::Continue(()))),
                        )?;
                        ctx.on_dispose_sync(move || {
                            released.set(released.get() + 1);
                            Ok::<_, Infallible>(())
                        })?;
                        Ok::<_, BoxError>(())
                    }
                })
            };
            let mut catalog = Catalog::new();
            catalog.register("worker", make(Rc::new(()))).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            loader
                .create(EntrySpec::new("worker", Config::new(0u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let plan = ReloadPlan::new(vec![node("worker", Some("worker"), &[])]).unwrap();
            let mut weak = None;
            for _ in 0..100 {
                let marker = Rc::new(());
                weak = Some(Rc::downgrade(&marker));
                assert!(
                    plan.replace(
                        &loader,
                        &["worker".into()],
                        vec![FactoryReplacement::new("worker", make(marker))]
                    )
                    .unwrap()
                    .await
                    .unwrap()
                    .failures
                    .is_empty()
                );
                assert_eq!(applied.get() - released.get(), 1);
            }
            loader.dispose().unwrap().await.unwrap();
            drop(loader);
            assert_eq!(applied.get(), 101);
            assert_eq!(released.get(), 101);
            assert!(weak.unwrap().upgrade().is_none());
            assert!(root.registry().is_empty().unwrap());
            assert_eq!(root.get_effects().unwrap().len(), before);
            assert!(runtime.take_failures().is_empty());
        })
        .await
        .unwrap();
}

#[cfg(feature = "include")]
#[tokio::test(flavor = "current_thread")]
async fn include_graph_replaces_the_same_file_managed_instances_without_writing() {
    use re_cordis_include::{Include, IncludeOptions};
    let path = std::env::temp_dir().join(format!(
        "cordis-hmr-linked-{}-{}.json",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let bytes = r#"[{"id":"a","name":"worker","config":6}]"#;
    std::fs::write(&path, bytes).unwrap();
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog
                .register_json("worker", inert("old"), |v| {
                    Ok::<_, Infallible>(v.unwrap().as_u64().unwrap() as u32)
                })
                .unwrap();
            let include = Include::new(&root, catalog, IncludeOptions::new(&path)).unwrap();
            include.refresh().unwrap().await.unwrap();
            let plan = ReloadPlan::new(vec![
                node("worker", Some("worker"), &["dep"]),
                node("dep", None, &[]),
            ])
            .unwrap();
            assert!(
                plan.replace_include(&include, &["dep".into()], vec![])
                    .is_err()
            );
            let seen = Rc::new(Cell::new(0));
            let next = Plugin::new("next", {
                let seen = seen.clone();
                move |_, v: u32| {
                    seen.set(v);
                    async { Ok::<_, Infallible>(()) }
                }
            });
            let report = plan
                .replace_include(
                    &include,
                    &["dep".into()],
                    vec![FactoryReplacement::new("worker", next)],
                )
                .unwrap()
                .await
                .unwrap();
            assert!(!report.changed);
            assert!(report.failures.is_empty());
            assert_eq!(seen.get(), 6);
            assert_eq!(include.pending(), 0);
            assert_eq!(std::fs::read_to_string(&path).unwrap(), bytes);
            include.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
    std::fs::remove_file(path).unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn candidate_preflight_is_separate_from_dependency_ready_core_validation() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = ServiceKey::<u32>::new("required");
            let mut catalog = Catalog::new();
            catalog.register("worker", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            loader
                .create(EntrySpec::new("worker", Config::new(7u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let preflight = Rc::new(Cell::new(0));
            let validation = Rc::new(Cell::new(0));
            let candidate = Plugin::validated(
                "validated",
                {
                    let validation = validation.clone();
                    move |value: &u32| {
                        validation.set(validation.get() + 1);
                        Err::<u32, _>(io::Error::other(format!("core rejects {value}")))
                    }
                },
                |_, _: u32| async { Ok::<_, Infallible>(()) },
            )
            .injecting([key.required()]);
            let plan = ReloadPlan::new(vec![node("worker", Some("worker"), &[])]).unwrap();
            let replacement = FactoryReplacement::new("worker", candidate).with_preflight({
                let preflight = preflight.clone();
                move |config: &Config| {
                    assert_eq!(*config.get::<u32>().unwrap(), 7);
                    preflight.set(preflight.get() + 1);
                    Ok::<_, Infallible>(())
                }
            });
            let report = plan
                .replace(&loader, &["worker".into()], vec![replacement])
                .unwrap()
                .await
                .unwrap();
            assert!(report.failures.is_empty());
            assert_eq!(preflight.get(), 1);
            assert_eq!(validation.get(), 0);
            root.provide(&key, 1).unwrap();
            let report = loader.settle().unwrap().await.unwrap();
            assert_eq!(report.failures.len(), 1);
            assert_eq!(validation.get(), 1);
            loader
                .replace_factory("worker", inert("recovery"))
                .unwrap()
                .await
                .unwrap();
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
