//! Whole-batch linked declaration admission and reentry regressions.
use re_cordis_core::{BoxError, Listener, Plugin, Runtime};
use re_cordis_loader::{
    Catalog, Config, EntryPatch, EntrySpec, FactoryReplacement, FieldPatch, Loader, LoaderError,
    SourceDocument, SourceId,
};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    io,
    ops::ControlFlow,
    rc::Rc,
};

fn inert(name: &str) -> Plugin<u32> {
    Plugin::new(name, |_, _: u32| async { Ok::<_, Infallible>(()) })
}
fn counted(name: &str, released: Rc<Cell<u32>>) -> Plugin<u32> {
    Plugin::new(name, move |ctx, _: u32| {
        let released = released.clone();
        async move {
            ctx.on_dispose_sync(move || {
                released.set(released.get() + 1);
                Ok::<_, Infallible>(())
            })?;
            Ok::<_, BoxError>(())
        }
    })
}
#[tokio::test(flavor = "current_thread")]
async fn every_candidate_is_checked_before_catalog_or_teardown() {
    let runtime = Runtime::new();
    runtime.run(async {
        let root=runtime.context(); let released=Rc::new(Cell::new(0));
        let mut catalog=Catalog::new();
        catalog.register("a",counted("a",released.clone())).unwrap();
        catalog.register("b",counted("b",released.clone())).unwrap();
        let loader=Loader::new(&root,catalog).unwrap();
        let a=loader.create(EntrySpec::new("a",Config::new(1u32))).unwrap();
        let b=loader.create(EntrySpec::new("b",Config::new(2u32)).disabled(true)).unwrap();
        loader.settle().unwrap().await.unwrap();
        let before=loader.snapshot(&a).unwrap();
        let invalid=FactoryReplacement::new("b",Plugin::new("wrong",|_, _:String| async {Ok::<_,Infallible>(())}));
        assert!(loader.replace_factories(vec![FactoryReplacement::new("a",inert("new")),invalid]).is_err());
        assert_eq!(loader.snapshot(&a).unwrap().factory_id,before.factory_id);
        assert_eq!(loader.snapshot(&a).unwrap().fiber_id,before.fiber_id);
        assert_eq!(released.get(),0);
        assert!(loader.replace_factories(vec![FactoryReplacement::new("a",inert("new")),FactoryReplacement::new("missing",inert("new"))]).is_err());
        assert!(loader.replace_factories(vec![FactoryReplacement::new("a",inert("new")),FactoryReplacement::new("a",inert("new"))]).is_err());
        let original=io::Error::other("candidate rejected");
        let error=loader.replace_factories(vec![FactoryReplacement::new("a",inert("new")),FactoryReplacement::new("b",inert("new")).with_preflight(move |config| {
            assert_eq!(*config.get::<u32>().unwrap(),2);
            Err::<(),_>(io::Error::new(original.kind(),original.to_string()))
        })]).err().unwrap();
        assert!(matches!(error,LoaderError::Decode{source,..} if source.downcast_ref::<io::Error>().unwrap().to_string()=="candidate rejected"));
        assert_eq!(loader.snapshot(&a).unwrap().factory_id,before.factory_id);
        assert_eq!(released.get(),0);
        assert_eq!(loader.snapshot(&b).unwrap().source.get::<u32>().unwrap().as_ref(),&2);
        loader.dispose().unwrap().await.unwrap();
    }).await.unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn preflight_blocks_same_loader_mutation_and_dispose() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("a", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let a = loader
                .create(EntrySpec::new("a", Config::new(1u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let calls = Rc::new(Cell::new(0));
            let replacement = FactoryReplacement::new("a", inert("new")).with_preflight({
                let loader = loader.clone();
                let a = a.clone();
                let calls = calls.clone();
                move |_: &Config| {
                    assert!(
                        loader
                            .create(EntrySpec::new("a", Config::new(3u32)))
                            .is_err()
                    );
                    assert!(
                        loader
                            .update(&a, EntrySpec::new("a", Config::new(3u32)))
                            .is_err()
                    );
                    assert!(
                        loader
                            .patch(
                                &a,
                                EntryPatch {
                                    disabled: FieldPatch::Set(true),
                                    ..EntryPatch::default()
                                }
                            )
                            .is_err()
                    );
                    assert!(loader.remove(&a).is_err());
                    assert!(
                        loader
                            .reconcile(SourceDocument::typed(SourceId::new("preflight"), vec![]))
                            .is_err()
                    );
                    assert!(loader.replace_factory("a", inert("recursive")).is_err());
                    assert!(loader.dispose().is_err());
                    calls.set(calls.get() + 1);
                    Ok::<_, Infallible>(())
                }
            });
            assert!(
                loader
                    .replace_factories(vec![replacement])
                    .unwrap()
                    .await
                    .unwrap()
                    .failures
                    .is_empty()
            );
            assert_eq!(calls.get(), 1);
            assert_eq!(
                *loader.snapshot(&a).unwrap().source.get::<u32>().unwrap(),
                1
            );
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn preflight_panic_releases_guard_without_accepting_candidate() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("a", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let a = loader
                .create(EntrySpec::new("a", Config::new(1u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let before = loader.snapshot(&a).unwrap().factory_id;
            let candidate = FactoryReplacement::new("a", inert("panic")).with_preflight(
                |_: &Config| -> Result<(), Infallible> { panic!("preflight boom") },
            );
            assert!(matches!(
                loader.replace_factories(vec![candidate]),
                Err(LoaderError::Callback(_))
            ));
            assert_eq!(loader.snapshot(&a).unwrap().factory_id, before);
            loader
                .replace_factory("a", inert("recovery"))
                .unwrap()
                .await
                .unwrap();
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn replacement_skips_removed_sibling_and_keeps_current_config() {
    for remove in [true, false] {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                let mut catalog = Catalog::new();
                catalog.register("worker", inert("old")).unwrap();
                let loader = Loader::new(&root, catalog).unwrap();
                let a = loader
                    .create(EntrySpec::new("worker", Config::new(1u32)).id("a"))
                    .unwrap();
                let b = loader
                    .create(EntrySpec::new("worker", Config::new(2u32)).id("b"))
                    .unwrap();
                loader.settle().unwrap().await.unwrap();
                let once = Rc::new(Cell::new(false));
                root.on(
                    &loader.commits(),
                    Listener::sync({
                        let loader = loader.clone();
                        let a = a.clone();
                        let b = b.clone();
                        let once = once.clone();
                        move |_, commit: re_cordis_loader::CommitRecord| {
                            if commit.id == a && !once.replace(true) {
                                if remove {
                                    drop(loader.remove(&b).unwrap());
                                } else {
                                    drop(
                                        loader
                                            .update(&b, EntrySpec::new("worker", Config::new(9u32)))
                                            .unwrap(),
                                    );
                                }
                            }
                            Ok::<_, Infallible>(ControlFlow::Continue(()))
                        }
                    }),
                )
                .unwrap();
                let applied = Rc::new(RefCell::new(Vec::new()));
                let new = Plugin::new("new", {
                    let applied = applied.clone();
                    move |_, v: u32| {
                        applied.borrow_mut().push(v);
                        async { Ok::<_, Infallible>(()) }
                    }
                });
                assert!(
                    loader
                        .replace_factory("worker", new)
                        .unwrap()
                        .await
                        .unwrap()
                        .failures
                        .is_empty()
                );
                let mut actual = applied.borrow().clone();
                actual.sort();
                assert_eq!(actual, if remove { vec![1] } else { vec![1, 9] });
                assert_eq!(loader.snapshot(&b).is_none(), remove);
                loader.dispose().unwrap().await.unwrap();
            })
            .await
            .unwrap();
    }
}
type DropCallback = Rc<RefCell<Option<Box<dyn FnOnce()>>>>;
struct Reenter(DropCallback);
impl Drop for Reenter {
    fn drop(&mut self) {
        let callback = self.0.borrow_mut().take();
        if let Some(callback) = callback {
            callback();
        }
    }
}
#[tokio::test(flavor = "current_thread")]
async fn displaced_factory_drop_sees_the_whole_new_catalog_without_borrow() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let slot = Rc::new(RefCell::new(None::<Box<dyn FnOnce()>>));
            let capture = Reenter(slot.clone());
            let old = Plugin::new("old", move |_, _: u32| {
                let _ = &capture;
                async { Ok::<_, Infallible>(()) }
            });
            let mut catalog = Catalog::new();
            catalog.register("a", old).unwrap();
            catalog.register("b", inert("old-b")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let seen = Rc::new(Cell::new(0));
            *slot.borrow_mut() = Some(Box::new({
                let loader = loader.clone();
                move || {
                    loader
                        .create(EntrySpec::new("b", Config::new(4u32)))
                        .unwrap();
                }
            }));
            let b = Plugin::new("new-b", {
                let seen = seen.clone();
                move |_, v: u32| {
                    seen.set(v);
                    async { Ok::<_, Infallible>(()) }
                }
            });
            loader
                .replace_factories(vec![
                    FactoryReplacement::new("a", inert("new-a")),
                    FactoryReplacement::new("b", b),
                ])
                .unwrap()
                .await
                .unwrap();
            assert_eq!(seen.get(), 4);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn commit_observer_failure_does_not_stop_unrelated_batch_work() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let a = loader
                .create(EntrySpec::new("worker", Config::new(1u32)).id("a"))
                .unwrap();
            loader
                .create(EntrySpec::new("worker", Config::new(2u32)).id("b"))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            root.on(
                &loader.commits(),
                Listener::sync({
                    let a = a.clone();
                    move |_, commit: re_cordis_loader::CommitRecord| {
                        if commit.id == a {
                            return Err(io::Error::other("commit observer failed"));
                        }
                        Ok(ControlFlow::Continue(()))
                    }
                }),
            )
            .unwrap();
            let seen = Rc::new(RefCell::new(Vec::new()));
            let new = Plugin::new("new", {
                let seen = seen.clone();
                move |_, v: u32| {
                    seen.borrow_mut().push(v);
                    async { Ok::<_, Infallible>(()) }
                }
            });
            let outcome = loader.replace_factory("worker", new);
            let admitted = outcome.is_ok();
            let report = if let Ok(task) = outcome {
                Some(task.await.unwrap())
            } else {
                None
            };
            loader.settle().unwrap().await.unwrap();
            let mut actual = seen.borrow().clone();
            actual.sort();
            loader.dispose().unwrap().await.unwrap();
            assert!(
                admitted,
                "catalog was admitted; observer failure belongs in Settlement"
            );
            assert_eq!(actual, [1, 2]);
            let report = report.unwrap();
            assert_eq!(report.failures.len(), 1);
            assert_eq!(report.failures[0].id, a);
            let mut cause: &dyn std::error::Error = &report.failures[0].error;
            while let Some(source) = cause.source() {
                cause = source;
            }
            assert_eq!(
                cause.downcast_ref::<io::Error>().unwrap().to_string(),
                "commit observer failed"
            );
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn rejected_preflight_capture_drop_runs_after_guard_release() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let entry = loader
                .create(EntrySpec::new("worker", Config::new(1u32)))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let before = loader.snapshot(&entry).unwrap().factory_id;
            let slot = Rc::new(RefCell::new(None::<Box<dyn FnOnce()>>));
            let called = Rc::new(Cell::new(false));
            *slot.borrow_mut() = Some(Box::new({
                let loader = loader.clone();
                let called = called.clone();
                move || {
                    loader
                        .create(EntrySpec::new("worker", Config::new(2u32)))
                        .unwrap();
                    called.set(true);
                }
            }));
            let capture = Reenter(slot);
            let candidate = FactoryReplacement::new("worker", inert("rejected")).with_preflight(
                move |_: &Config| {
                    let _ = &capture;
                    Err::<(), _>(io::Error::other("reject"))
                },
            );
            assert!(loader.replace_factories(vec![candidate]).is_err());
            assert!(called.get());
            assert_eq!(loader.snapshot(&entry).unwrap().factory_id, before);
            loader.settle().unwrap().await.unwrap();
            assert_eq!(loader.entries().len(), 2);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn commit_factory_change_preserves_newer_sibling_intent_and_decoder() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let seen = Rc::new(RefCell::new(Vec::new()));
            let mut catalog = Catalog::new();
            catalog
                .register_json("worker", inert("old"), |v| {
                    Ok::<_, Infallible>(v.unwrap().as_u64().unwrap() as u32)
                })
                .unwrap();
            catalog
                .register(
                    "other",
                    Plugin::new("other", {
                        let seen = seen.clone();
                        move |_, v: u32| {
                            seen.borrow_mut().push(("other", v));
                            async { Ok::<_, Infallible>(()) }
                        }
                    }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let a = loader
                .create(EntrySpec::new("worker", Config::new(1u32)).id("a"))
                .unwrap();
            let b = loader
                .create(EntrySpec::new("worker", Config::new(2u32)).id("b"))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let once = Rc::new(Cell::new(false));
            root.on(
                &loader.commits(),
                Listener::sync({
                    let loader = loader.clone();
                    let a = a.clone();
                    let b = b.clone();
                    move |_, commit: re_cordis_loader::CommitRecord| {
                        if commit.id == a && !once.replace(true) {
                            drop(
                                loader
                                    .update(&b, EntrySpec::new("other", Config::new(9u32)))
                                    .unwrap(),
                            );
                        }
                        Ok::<_, Infallible>(ControlFlow::Continue(()))
                    }
                }),
            )
            .unwrap();
            let replacement = Plugin::new("new", {
                let seen = seen.clone();
                move |_, v: u32| {
                    seen.borrow_mut().push(("new", v));
                    async { Ok::<_, Infallible>(()) }
                }
            });
            loader
                .replace_factory("worker", replacement)
                .unwrap()
                .await
                .unwrap();
            assert_eq!(loader.snapshot(&b).unwrap().factory, "other");
            loader
                .create_json(r#"{"name":"worker","config":3}"#)
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let mut actual = seen.borrow().clone();
            actual.sort();
            assert_eq!(actual, [("new", 1), ("new", 3), ("other", 9)]);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn dropped_batch_observer_error_remains_observable() {
    for remove in [false, true] {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let a = loader.create(EntrySpec::new("worker", Config::new(1u32))).unwrap();
            loader.settle().unwrap().await.unwrap();
            let once = Rc::new(Cell::new(false));
            root.on(&loader.commits(), Listener::sync({
                let loader = loader.clone(); let a = a.clone();
                move |_, commit: re_cordis_loader::CommitRecord| {
                    if commit.id != a || once.replace(true) { return Ok(ControlFlow::Continue(())); }
                    if remove { drop(loader.remove(&a).unwrap()); }
                    Err(io::Error::other("dropped batch observer"))
                }
            })).unwrap();
            drop(loader.replace_factory("worker", inert("new")).unwrap());
            let report = loader.settle().unwrap().await.unwrap();
            let diagnostics = runtime.take_failures();
            let original = |error: &dyn std::error::Error| {
                let mut cause = error;
                while let Some(source) = cause.source() { cause = source; }
                cause.to_string() == "dropped batch observer"
            };
            let in_report = report.failures.iter().any(|failure| failure.id == a && original(&failure.error));
            let in_runtime = diagnostics.iter().any(|failure| original(failure));
            eprintln!("removed={remove} later_settlement_has_original={in_report} runtime_has_original={in_runtime}");
            assert!(in_report || in_runtime, "dropped batch hides original observer cause");
            assert!(loader.settle().unwrap().await.unwrap().failures.is_empty(), "delivered settlement acknowledges retained error");
            loader.dispose().unwrap().await.unwrap();
        }).await.unwrap();
    }
}
fn original_io_error<'a>(error: &'a (dyn std::error::Error + 'static)) -> &'a io::Error {
    let mut cause = error;
    while let Some(source) = cause.source() {
        cause = source;
    }
    cause
        .downcast_ref::<io::Error>()
        .expect("original I/O error")
}

#[tokio::test(flavor = "current_thread")]
async fn exact_batch_retains_both_observer_and_apply_errors_for_each_entry() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            for (id, config) in [("a", 1u32), ("b", 2u32)] {
                loader
                    .create(EntrySpec::new("worker", Config::new(config)).id(id))
                    .unwrap();
            }
            loader.settle().unwrap().await.unwrap();
            root.on(
                &loader.commits(),
                Listener::sync(|_, commit: re_cordis_loader::CommitRecord| {
                    Err::<ControlFlow<()>, _>(io::Error::other(format!(
                        "observer-{}",
                        commit.id.as_str()
                    )))
                }),
            )
            .unwrap();
            let failed = Plugin::new("failed", |_, config: u32| async move {
                Err::<(), _>(io::Error::other(format!("apply-{config}")))
            });
            let report = loader
                .replace_factory("worker", failed)
                .unwrap()
                .await
                .unwrap();
            let mut actual: Vec<_> = report
                .failures
                .iter()
                .map(|failure| {
                    (
                        failure.id.as_str(),
                        original_io_error(&failure.error).to_string(),
                    )
                })
                .collect();
            actual.sort();
            eprintln!("exact batch original causes: {actual:?}");
            assert_eq!(
                actual,
                [
                    ("a", "apply-1".into()),
                    ("a", "observer-a".into()),
                    ("b", "apply-2".into()),
                    ("b", "observer-b".into()),
                ]
            );
            let later = loader.settle().unwrap().await.unwrap();
            assert_eq!(later.failures.len(), 2);
            for failure in later.failures {
                assert!(
                    original_io_error(&failure.error)
                        .to_string()
                        .starts_with("apply-")
                );
            }
            let mut diagnostics: Vec<_> = runtime
                .take_failures()
                .iter()
                .map(|error| original_io_error(error).to_string())
                .collect();
            diagnostics.sort();
            assert_eq!(diagnostics, ["apply-1", "apply-2"]);
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn dropped_batch_keeps_first_unobserved_observer_policy() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", inert("old")).unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            for (id, config) in [("a", 1u32), ("b", 2u32)] {
                loader
                    .create(EntrySpec::new("worker", Config::new(config)).id(id))
                    .unwrap();
            }
            loader.settle().unwrap().await.unwrap();
            root.on(
                &loader.commits(),
                Listener::sync(|_, commit: re_cordis_loader::CommitRecord| {
                    Err::<ControlFlow<()>, _>(io::Error::other(format!(
                        "observer-{}",
                        commit.id.as_str()
                    )))
                }),
            )
            .unwrap();
            let failed = Plugin::new("failed", |_, config: u32| async move {
                Err::<(), _>(io::Error::other(format!("apply-{config}")))
            });
            drop(loader.replace_factory("worker", failed).unwrap());
            let report = loader.settle().unwrap().await.unwrap();
            let mut actual: Vec<_> = report
                .failures
                .iter()
                .map(|failure| {
                    (
                        failure.id.as_str(),
                        original_io_error(&failure.error).to_string(),
                    )
                })
                .collect();
            actual.sort();
            eprintln!("dropped batch original causes: {actual:?}");
            assert_eq!(
                actual,
                [("a", "observer-a".into()), ("b", "apply-2".into())]
            );
            let later = loader.settle().unwrap().await.unwrap();
            assert_eq!(later.failures.len(), 2);
            for failure in later.failures {
                assert!(
                    original_io_error(&failure.error)
                        .to_string()
                        .starts_with("apply-")
                );
            }
            loader.dispose().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

async fn exact_batch_with_older_observer_slot(current_observers_fail: bool) {
    let runtime = Runtime::new();
    runtime
            .run(async {
                let root = runtime.context();
                let mut catalog = Catalog::new();
                catalog.register("worker", inert("old")).unwrap();
                let loader = Loader::new(&root, catalog).unwrap();
                for (id, config) in [("a", 1u32), ("b", 2u32)] {
                    loader
                        .create(EntrySpec::new("worker", Config::new(config)).id(id))
                        .unwrap();
                }
                loader.settle().unwrap().await.unwrap();
                let phase = Rc::new(Cell::new(1));
                root.on(
                    &loader.commits(),
                    Listener::sync({
                        let phase = phase.clone();
                        move |_, commit: re_cordis_loader::CommitRecord| {
                            if phase.get() == 2 && !current_observers_fail {
                                return Ok(ControlFlow::Continue(()));
                            }
                            Err::<ControlFlow<()>, _>(io::Error::other(format!(
                                "observer-{}-{}",
                                phase.get(),
                                commit.id.as_str()
                            )))
                        }
                    }),
                )
                .unwrap();
                drop(loader.replace_factory("worker", inert("first")).unwrap());
                phase.set(2);
                let failed = Plugin::new("failed", |_, config: u32| async move {
                    Err::<(), _>(io::Error::other(format!("apply-{config}")))
                });
                let report = loader
                    .replace_factory("worker", failed)
                    .unwrap()
                    .await
                    .unwrap();
                let mut actual: Vec<_> = report
                    .failures
                    .iter()
                    .map(|failure| {
                        (
                            failure.id.as_str(),
                            original_io_error(&failure.error).to_string(),
                        )
                    })
                    .collect();
                actual.sort();
                let mut expected = vec![
                    ("a", "apply-1".into()),
                    ("a", "observer-1-a".into()),
                    ("b", "apply-2".into()),
                ];
                if current_observers_fail {
                    expected.extend([
                        ("a", "observer-2-a".into()),
                        ("b", "observer-2-b".into()),
                    ]);
                }
                expected.sort();
                eprintln!("exact batch with older slot, current_observers_fail={current_observers_fail}: {actual:?}");
                assert_eq!(actual, expected);
                let later = loader.settle().unwrap().await.unwrap();
                let mut later_causes: Vec<_> = later
                    .failures
                    .iter()
                    .map(|failure| {
                        (
                            failure.id.as_str(),
                            original_io_error(&failure.error).to_string(),
                        )
                    })
                    .collect();
                later_causes.sort();
                assert_eq!(
                    later_causes,
                    [("a", "apply-1".into()), ("b", "apply-2".into())]
                );
                let mut diagnostics: Vec<_> = runtime
                    .take_failures()
                    .iter()
                    .map(|error| original_io_error(error).to_string())
                    .collect();
                diagnostics.sort();
                assert_eq!(diagnostics, ["apply-1", "apply-2"]);
                loader.dispose().unwrap().await.unwrap();
            })
            .await
            .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn exact_batch_keeps_apply_and_observer_causes_with_an_older_slot() {
    exact_batch_with_older_observer_slot(true).await;
}

#[tokio::test(flavor = "current_thread")]
async fn exact_batch_keeps_apply_causes_with_an_older_slot_and_successful_observers() {
    exact_batch_with_older_observer_slot(false).await;
}
