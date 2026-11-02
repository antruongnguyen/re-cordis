//! Independently reviewed loader lifecycle and source regressions.
use re_cordis_core::{BoxError, Listener, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{
    Catalog, Config, EntrySpec, EntryState, Group, Loader, SourceDocument, SourceId,
};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    ops::ControlFlow,
    rc::Rc,
};

#[tokio::test(flavor = "current_thread")]
async fn entry_dependency_withdrawal_recovers_without_disabled_commit() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let key = ServiceKey::<()>::new("entry-dependency");
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
        let provider = root.provide(&key, ()).unwrap();
        let id = loader
            .create(EntrySpec::new("worker", Config::new(())).injecting([key.required()]))
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        let commits = Rc::new(RefCell::new(Vec::new()));
        root.on(
            &loader.commits(),
            Listener::sync({
                let commits = commits.clone();
                move |_, record| {
                    commits.borrow_mut().push(record);
                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                }
            }),
        )
        .unwrap();
        provider.withdraw().unwrap().await.unwrap();
        loader.settle().unwrap().await.unwrap();
        assert!(!loader.snapshot(&id).unwrap().options.disabled);
        assert!(commits.borrow().is_empty());
        root.provide(&key, ()).unwrap();
        let report = loader.settle().unwrap().await.unwrap();
        assert!(report.failures.is_empty());
        assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Active);
        assert_eq!(applies.get(), 2);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn pending_group_update_defers_condition_until_entry_dependencies_commit() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let key = ServiceKey::<()>::new("group-dependency");
        let calls = Rc::new(Cell::new(0));
        let loader = Loader::new(&root, Catalog::new()).unwrap();
        let spec = EntrySpec::group(Group::new(vec![]))
            .injecting([key.required()])
            .condition({
                let calls = calls.clone();
                move |_| {
                    calls.set(calls.get() + 1);
                    Ok::<_, Infallible>(true)
                }
            });
        let id = loader.create(spec.clone()).unwrap();
        loader.settle().unwrap().await.unwrap();
        loader.update(&id, spec).unwrap().await.unwrap();
        assert_eq!(calls.get(), 0);
        assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Pending);
        root.provide(&key, ()).unwrap();
        loader.settle().unwrap().await.unwrap();
        assert_eq!(calls.get(), 1);
        assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Active);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn same_failed_group_update_explicitly_recovers_outer() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let failed = Rc::new(Cell::new(true));
        let loader = Loader::new(&root, Catalog::new()).unwrap();
        let spec = EntrySpec::group(Group::new(vec![])).condition({
            let failed = failed.clone();
            move |_| {
                if failed.get() {
                    Err::<bool, BoxError>("verdict failed".into())
                } else {
                    Ok(true)
                }
            }
        });
        let id = loader.create(spec.clone()).unwrap();
        assert_eq!(loader.settle().unwrap().await.unwrap().failures.len(), 1);
        failed.set(false);
        loader.update(&id, spec).unwrap().await.unwrap();
        assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Active);
        assert!(loader.settle().unwrap().await.unwrap().failures.is_empty());
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn source_adoption_moves_nested_identity_to_root_and_preserves_runtime_edit_owner() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, _: u32| async { Ok::<_, Infallible>(()) }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let a = SourceId::new("A");
        let b = SourceId::new("B");
        loader
            .reconcile(SourceDocument::typed(
                a.clone(),
                vec![
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(1_u32)).id("child"),
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
                b.clone(),
                vec![EntrySpec::new("worker", Config::new(2_u32)).id("child")],
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
        loader
            .update(&child, EntrySpec::new("worker", Config::new(3_u32)))
            .unwrap()
            .await
            .unwrap();
        loader
            .reconcile(SourceDocument::typed(b, vec![]))
            .unwrap()
            .await
            .unwrap();
        assert!(loader.snapshot(&child).is_none());
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn runtime_group_options_preflight_is_atomic_but_loaded_validation_is_independent() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::validated(
                    "worker",
                    |v: &i32| u32::try_from(*v),
                    |_, _: u32| async { Ok::<_, Infallible>(()) },
                ),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let child = |id: &str, v| EntrySpec::new("worker", Config::new(v)).id(id);
        let group = loader
            .create(
                EntrySpec::group(Group::new(vec![
                    child("first", 1_i32),
                    child("bad", 2_i32),
                    child("last", 3_i32),
                ]))
                .id("group"),
            )
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        let records = Rc::new(RefCell::new(Vec::new()));
        root.on(
            &loader.commits(),
            Listener::sync({
                let records = records.clone();
                move |_, record| {
                    records.borrow_mut().push(record);
                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                }
            }),
        )
        .unwrap();
        assert!(
            loader
                .update(
                    &group,
                    EntrySpec::group(Group::new(vec![
                        child("first", 4_i32),
                        EntrySpec::new("unknown", Config::new(())).id("bad")
                    ]))
                )
                .is_err()
        );
        assert!(records.borrow().is_empty());
        assert_eq!(
            *loader
                .snapshot(&loader.resolve("group:first").unwrap())
                .unwrap()
                .source
                .get::<i32>()
                .unwrap(),
            1
        );
        let task = loader
            .update(
                &group,
                EntrySpec::group(Group::new(vec![
                    child("first", 4_i32),
                    child("bad", -1_i32),
                    child("last", 5_i32),
                ])),
            )
            .unwrap();
        drop(task);
        let report = loader.settle().unwrap().await.unwrap();
        assert_eq!(report.failures.len(), 1);
        assert_eq!(report.failures[0].id.as_str(), "bad");
        assert_eq!(
            *loader
                .snapshot(&loader.resolve("group:first").unwrap())
                .unwrap()
                .source
                .get::<i32>()
                .unwrap(),
            4
        );
        assert_eq!(
            *loader
                .snapshot(&loader.resolve("group:bad").unwrap())
                .unwrap()
                .source
                .get::<i32>()
                .unwrap(),
            2
        );
        assert_eq!(
            *loader
                .snapshot(&loader.resolve("group:last").unwrap())
                .unwrap()
                .source
                .get::<i32>()
                .unwrap(),
            5
        );
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn typed_source_group_keeps_valid_siblings_and_locates_contract_failure() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, _: u32| async { Ok::<_, Infallible>(()) }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let source = SourceId::new("typed-memory");
        let document = SourceDocument::typed(
            source.clone(),
            vec![
                EntrySpec::group(Group::new(vec![
                    EntrySpec::new("worker", Config::new(1_u32)).id("first"),
                    EntrySpec::new("worker", Config::new("wrong contract")).id("bad"),
                    EntrySpec::new("worker", Config::new(2_u32)).id("last"),
                ]))
                .id("group"),
            ],
        )
        .with_locations(|_| Some(re_cordis_loader::SourceSpan { line: 9, column: 4 }));
        let report = loader.reconcile(document).unwrap().await.unwrap();
        assert!(loader.resolve("group:first").is_ok());
        assert!(loader.resolve("group:last").is_ok());
        assert!(loader.resolve("group:bad").is_err());
        assert_eq!(report.failures.len(), 1);
        assert_eq!(report.failures[0].id.as_str(), "bad");
        let re_cordis_loader::LoaderError::Located { location, .. } = &report.failures[0].error
        else {
            panic!("missing source provenance")
        };
        assert_eq!(location.source, Some(source));
        assert_eq!(
            location.path,
            vec![
                re_cordis_core::PathSegment::Index(0),
                re_cordis_core::PathSegment::Key("config".into()),
                re_cordis_core::PathSegment::Index(1)
            ]
        );
        assert_eq!(
            location.span,
            Some(re_cordis_loader::SourceSpan { line: 9, column: 4 })
        );
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[derive(Debug)]
struct ValidationFailure;
impl std::fmt::Display for ValidationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("validation rejected")
    }
}
impl std::error::Error for ValidationFailure {}
fn has_validation_cause(mut error: &(dyn std::error::Error + 'static)) -> bool {
    loop {
        if error.downcast_ref::<ValidationFailure>().is_some() {
            return true;
        }
        match error.source() {
            Some(source) => error = source,
            None => return false,
        }
    }
}
#[tokio::test(flavor = "current_thread")]
async fn source_initial_and_loaded_validation_failures_keep_original_cause_and_location() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::validated(
                    "worker",
                    |v: &i32| {
                        if *v < 0 {
                            Err(ValidationFailure)
                        } else {
                            Ok(*v)
                        }
                    },
                    |_, _: i32| async { Ok::<_, Infallible>(()) },
                ),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let source = SourceId::new("validation-memory");
        let document = |value| {
            SourceDocument::typed(
                source.clone(),
                vec![EntrySpec::new("worker", Config::new(value)).id("worker")],
            )
            .with_locations(|_| {
                Some(re_cordis_loader::SourceSpan {
                    line: 7,
                    column: 11,
                })
            })
        };
        let report = loader.reconcile(document(-1_i32)).unwrap().await.unwrap();
        assert_eq!(report.failures.len(), 1);
        let check = |error: &re_cordis_loader::LoaderError| {
            let re_cordis_loader::LoaderError::Located { location, .. } = error else {
                panic!("missing source provenance: {error:?}")
            };
            assert_eq!(location.source, Some(source.clone()));
            assert_eq!(location.path, vec![re_cordis_core::PathSegment::Index(0)]);
            assert_eq!(
                location.span,
                Some(re_cordis_loader::SourceSpan {
                    line: 7,
                    column: 11
                })
            );
            assert!(has_validation_cause(error));
        };
        check(&report.failures[0].error);
        loader.reconcile(document(1_i32)).unwrap().await.unwrap();
        let report = loader.reconcile(document(-2_i32)).unwrap().await.unwrap();
        assert_eq!(report.failures.len(), 1);
        check(&report.failures[0].error);
        let id = loader.resolve("worker").unwrap();
        assert_eq!(
            *loader.snapshot(&id).unwrap().source.get::<i32>().unwrap(),
            1
        );
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn failing_detached_cleanup_keeps_original_diagnostics_through_admit_and_shutdown() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let (entered_send, entered) = tokio::sync::oneshot::channel();
        let (release, gate) = tokio::sync::oneshot::channel();
        let gates = Rc::new(RefCell::new(Some((entered_send, gate))));
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", move |ctx, (): ()| {
                    let gates = gates.borrow_mut().take();
                    async move {
                        ctx.on_dispose(move || async move {
                            if let Some((entered, gate)) = gates {
                                entered.send(()).unwrap();
                                gate.await.unwrap();
                                return Err::<(), BoxError>(Box::new(ValidationFailure));
                            }
                            Ok(())
                        })?;
                        Ok::<_, BoxError>(())
                    }
                }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let id = loader
            .create(EntrySpec::new("worker", Config::new(())))
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        drop(loader.remove(&id).unwrap());
        entered.await.unwrap();
        release.send(()).unwrap();
        loader.settle().unwrap().await.unwrap();
        loader
            .create(EntrySpec::new("worker", Config::new(())))
            .unwrap();
        drop(loader);
        root.drain().unwrap().await.unwrap();
        assert!(
            runtime
                .take_failures()
                .iter()
                .any(|failure| has_validation_cause(failure))
        );
        assert!(root.registry().is_empty().unwrap());
    })
    .await;
}

#[derive(Clone)]
struct SnapshotOnDrop {
    state: Rc<RefCell<Option<(Loader, re_cordis_loader::EntryId)>>>,
    reads: Rc<Cell<usize>>,
    mapped: bool,
}
impl Drop for SnapshotOnDrop {
    fn drop(&mut self) {
        if self.mapped {
            if let Some((loader, id)) = self.state.borrow().as_ref() {
                assert!(loader.snapshot(id).is_some());
                self.reads.set(self.reads.get() + 1);
            }
        }
    }
}
#[tokio::test(flavor = "current_thread")]
async fn mapped_config_destructor_can_read_snapshot_during_update() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let state = Rc::new(RefCell::new(None));
        let reads = Rc::new(Cell::new(0));
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, _: SnapshotOnDrop| async {
                    Ok::<_, Infallible>(())
                }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let spec = EntrySpec::new(
            "worker",
            Config::new(SnapshotOnDrop {
                state: state.clone(),
                reads: reads.clone(),
                mapped: false,
            }),
        )
        .map_config::<SnapshotOnDrop, _, Infallible>(|_, source| {
            Ok(SnapshotOnDrop {
                mapped: true,
                state: source.state.clone(),
                reads: source.reads.clone(),
            })
        });
        let id = loader.create(spec.clone()).unwrap();
        loader.settle().unwrap().await.unwrap();
        *state.borrow_mut() = Some((loader.clone(), id.clone()));
        loader.update(&id, spec).unwrap().await.unwrap();
        assert!(reads.get() > 0);
        assert!(runtime.take_failures().is_empty());
        let previous = state.borrow_mut().take();
        drop(previous);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn child_commit_observer_can_reconcile_parent_without_origin_borrow_panic() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, _: u32| async { Ok::<_, Infallible>(()) }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let group_spec = |value| {
            EntrySpec::group(Group::new(vec![
                EntrySpec::new("worker", Config::new(value)).id("child"),
            ]))
            .id("group")
        };
        let id = loader.create(group_spec(1_u32)).unwrap();
        loader.settle().unwrap().await.unwrap();
        let once = Rc::new(Cell::new(true));
        root.on(
            &loader.commits(),
            Listener::sync({
                let loader = loader.clone();
                let once = once.clone();
                move |_, record: re_cordis_loader::CommitRecord| {
                    if record.id.as_str() == "child" && once.replace(false) {
                        drop(
                            loader
                                .reconcile(SourceDocument::typed(
                                    SourceId::new("observer"),
                                    vec![
                                        EntrySpec::group(Group::new(vec![
                                            EntrySpec::new("worker", Config::new(3_u32))
                                                .id("child"),
                                        ]))
                                        .id("group"),
                                    ],
                                ))
                                .unwrap(),
                        );
                    }
                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                }
            }),
        )
        .unwrap();
        loader
            .update(&id, group_spec(2_u32))
            .unwrap()
            .await
            .unwrap();
        let report = loader.settle().unwrap().await.unwrap();
        assert!(report.failures.is_empty());
        assert!(runtime.take_failures().is_empty());
        assert_eq!(
            *loader
                .snapshot(&loader.resolve("group:child").unwrap())
                .unwrap()
                .source
                .get::<u32>()
                .unwrap(),
            3
        );
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn factory_replacement_rejects_changed_config_contract_without_live_entries() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register_json(
                "worker",
                Plugin::new("worker", |_, _: u32| async { Ok::<_, Infallible>(()) }),
                |value| {
                    serde_json::from_value::<u32>(value.cloned().unwrap_or(serde_json::json!(0)))
                },
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        assert!(
            loader
                .replace_factory(
                    "worker",
                    Plugin::new("replacement", |_, _: String| async {
                        Ok::<_, Infallible>(())
                    })
                )
                .is_err()
        );
        let id = loader
            .create_json(r#"{"name":"worker","config":5}"#)
            .unwrap();
        assert!(loader.settle().unwrap().await.unwrap().failures.is_empty());
        assert_eq!(
            *loader.snapshot(&id).unwrap().source.get::<u32>().unwrap(),
            5
        );
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn nested_json_async_failure_keeps_original_uncompacted_source_path_and_transport() {
    let runtime = Runtime::new();
    bounded(&runtime, async{
        let root=runtime.context();let mut catalog=Catalog::new();
        catalog.register_json("worker",Plugin::validated("worker",|v:&i32|if *v<0 {Err(ValidationFailure)}else{Ok(*v)},|_,_:i32|async{Ok::<_,Infallible>(())}),|value|serde_json::from_value::<i32>(value.cloned().unwrap())).unwrap();
        let loader=Loader::new(&root,catalog).unwrap();let source=SourceId::new("nested");
        let report=loader.reconcile(SourceDocument::json(source.clone(),r#"[{"id":"group","group":true,"config":[{"id":"decode-bad","name":"worker","config":"bad"},{"id":"load-bad","name":"worker","config":-1}]}]"#).unwrap().with_locations(|path|Some(re_cordis_loader::SourceSpan{line:3,column:path.len()}))).unwrap().await.unwrap();
        assert_eq!(report.failures.len(),2);
        let error=&report.failures.iter().find(|failure|failure.id.as_str()=="load-bad").unwrap().error;
        let re_cordis_loader::LoaderError::Located{location,..}=error else{panic!("missing location")};
        assert_eq!(location.source,Some(source));
        assert_eq!(location.path,vec![re_cordis_core::PathSegment::Index(0),re_cordis_core::PathSegment::Key("config".into()),re_cordis_core::PathSegment::Index(1)]);
        assert!(has_validation_cause(error));
        let id=loader.resolve("group:load-bad").unwrap();
        assert_eq!(loader.snapshot(&id).unwrap().transport.unwrap()["config"],-1);
        loader.dispose().unwrap().await.unwrap();
    }).await;
}
#[tokio::test(flavor = "current_thread")]
async fn same_source_root_adoption_into_group_survives_previous_root_cleanup() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let source = SourceId::new("memory");
        loader
            .reconcile(SourceDocument::typed(
                source.clone(),
                vec![EntrySpec::new("worker", Config::new(())).id("child")],
            ))
            .unwrap()
            .await
            .unwrap();
        let child = loader.resolve("child").unwrap();
        loader
            .reconcile(SourceDocument::typed(
                source,
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
        assert_eq!(loader.resolve("group:child").unwrap(), child);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn same_source_root_adoption_joins_gated_cleanup_without_losing_child() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let (started, entered) = tokio::sync::oneshot::channel();
        let (release, gate) = tokio::sync::oneshot::channel();
        let gates = Rc::new(RefCell::new(Some((started, gate))));
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", move |ctx, (): ()| {
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
                }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let source = SourceId::new("adoption");
        loader
            .reconcile(SourceDocument::typed(
                source.clone(),
                vec![EntrySpec::new("worker", Config::new(())).id("child")],
            ))
            .unwrap()
            .await
            .unwrap();
        let child = loader.resolve("child").unwrap();
        let task = loader
            .reconcile(SourceDocument::typed(
                source,
                vec![
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(())).id("child"),
                    ]))
                    .id("group"),
                ],
            ))
            .unwrap();
        entered.await.unwrap();
        release.send(()).unwrap();
        let report = task.await.unwrap();
        assert!(report.failures.is_empty(), "{:?}", report.failures);
        assert_eq!(loader.resolve("group:child").unwrap(), child);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

async fn bounded<T>(runtime: &Runtime, future: impl std::future::Future<Output = T>) {
    tokio::time::timeout(std::time::Duration::from_secs(5), runtime.run(future))
        .await
        .expect("loader scenario exceeded five seconds")
        .expect("Cordis host failed");
}

#[tokio::test(flavor = "current_thread")]
async fn invalid_old_source_cannot_remove_identity_adopted_by_another_source() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
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
        let spec = || EntrySpec::new("worker", Config::new(())).id("child");
        loader
            .reconcile(SourceDocument::typed(a.clone(), vec![spec()]))
            .unwrap()
            .await
            .unwrap();
        let id = loader.resolve("child").unwrap();
        loader
            .reconcile(SourceDocument::typed(b, vec![spec()]))
            .unwrap()
            .await
            .unwrap();
        let report = loader
            .reconcile(SourceDocument::typed(
                a,
                vec![EntrySpec::new("unknown", Config::new(())).id("child")],
            ))
            .unwrap()
            .await
            .unwrap();
        assert_eq!(report.failures.len(), 1);
        assert!(loader.snapshot(&id).is_some());
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn malformed_json_source_ids_fail_per_entry_while_valid_siblings_survive() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register_json(
                "worker",
                Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
                |_| Ok::<_, Infallible>(()),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let source = SourceId::new("id-errors");
        let text = serde_json::json!([
            {"id":"bad:reserved","name":"worker"},
            {"id":"x".repeat(129),"name":"worker"},
            {"id":"good","name":"worker"}
        ])
        .to_string();
        let report = loader
            .reconcile(SourceDocument::json(source.clone(), &text).unwrap())
            .unwrap()
            .await
            .unwrap();
        assert_eq!(report.failures.len(), 2);
        assert_eq!(loader.entries().len(), 1);
        assert!(loader.resolve("good").is_ok());
        for (index, failure) in report.failures.iter().enumerate() {
            let re_cordis_loader::LoaderError::Located { location, .. } = &failure.error else {
                panic!("missing location")
            };
            assert_eq!(location.source, Some(source.clone()));
            assert_eq!(
                location.path,
                vec![
                    re_cordis_core::PathSegment::Index(index),
                    re_cordis_core::PathSegment::Key("id".into())
                ]
            );
        }
        assert!(
            loader
                .create_json(r#"{"id":"bad:reserved","name":"worker"}"#)
                .is_err()
        );
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn group_update_rejects_effective_root_id_collision_before_commit() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let id = loader
            .create(EntrySpec::group(Group::new(vec![])).id("group"))
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        let records = Rc::new(RefCell::new(Vec::new()));
        root.on(
            &loader.commits(),
            Listener::sync({
                let records = records.clone();
                move |_, record| {
                    records.borrow_mut().push(record);
                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                }
            }),
        )
        .unwrap();
        assert!(
            loader
                .update(
                    &id,
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(())).id("group")
                    ]))
                )
                .is_err()
        );
        assert!(records.borrow().is_empty());
        assert!(loader.snapshot(&id).unwrap().children.is_empty());
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn autogenerated_group_identity_skips_requested_descendant_ids() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let group = loader
            .create(EntrySpec::group(Group::new(vec![
                EntrySpec::new("worker", Config::new(())).id("entry-1"),
            ])))
            .unwrap();
        assert_ne!(group.as_str(), "entry-1");
        assert!(loader.settle().unwrap().await.unwrap().failures.is_empty());
        assert!(loader.resolve(&format!("{group}:entry-1")).is_ok());
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn duplicate_source_root_id_reports_one_failure_and_preserves_first_and_healthy_sibling() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", |_, _: u32| async { Ok::<_, Infallible>(()) }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let source = SourceId::new("duplicates");
        let spec = |id: &str, value| EntrySpec::new("worker", Config::new(value)).id(id);
        let document = || {
            SourceDocument::typed(
                source.clone(),
                vec![spec("x", 1_u32), spec("x", 2_u32), spec("y", 3_u32)],
            )
        };
        for _ in 0..2 {
            let report = loader.reconcile(document()).unwrap().await.unwrap();
            assert_eq!(report.failures.len(), 1);
            assert_eq!(report.failures[0].id.as_str(), "x");
            assert_eq!(
                loader
                    .entries()
                    .iter()
                    .map(|entry| entry.id.as_str())
                    .collect::<Vec<_>>(),
                ["x", "y"]
            );
            assert_eq!(
                *loader
                    .snapshot(&loader.resolve("x").unwrap())
                    .unwrap()
                    .source
                    .get::<u32>()
                    .unwrap(),
                1
            );
        }
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}
