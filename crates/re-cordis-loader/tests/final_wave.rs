//! Regression oracles from the independent final loader review.
use re_cordis_core::{BoxError, Listener, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{
    Catalog, Config, EntrySpec, EntryState, Group, Loader, LoaderError, Origin, Scope,
    SourceDocument, SourceId, SourceSpan,
};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    ops::ControlFlow,
    rc::Rc,
};
use tokio::sync::oneshot;
async fn bounded(runtime: &Runtime, future: impl std::future::Future<Output = ()>) {
    tokio::time::timeout(std::time::Duration::from_secs(5), runtime.run(future))
        .await
        .expect("five-second loader regression deadline")
        .unwrap();
}
fn group(value: u32, id: &str) -> EntrySpec {
    EntrySpec::group(Group::new(vec![
        EntrySpec::new("worker", Config::new(value)).id(id),
    ]))
    .id("group")
}
fn number_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    catalog
        .register(
            "worker",
            Plugin::new("worker", |_, _: u32| async { Ok::<_, Infallible>(()) }),
        )
        .unwrap();
    catalog
}
#[tokio::test(flavor = "current_thread")]
async fn reentrant_membership_replacement_keeps_newest_tree_visible() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let loader = Loader::new(&root, number_catalog()).unwrap();
        let parent = loader.create(group(1, "child")).unwrap();
        loader.settle().unwrap().await.unwrap();
        let once = Rc::new(Cell::new(true));
        root.on(
            &loader.commits(),
            Listener::sync({
                let loader = loader.clone();
                move |_, record: re_cordis_loader::CommitRecord| {
                    if record.id.as_str() == "child" && once.replace(false) {
                        drop(
                            loader
                                .reconcile(SourceDocument::typed(
                                    SourceId::new("newest"),
                                    vec![group(3, "new")],
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
            .update(&parent, group(2, "child"))
            .unwrap()
            .await
            .unwrap();
        let report = loader.settle().unwrap().await.unwrap();
        assert!(report.failures.is_empty());
        let new = loader.resolve("group:new").unwrap();
        assert!(loader.resolve("group:child").is_err());
        assert_eq!(loader.snapshot(&parent).unwrap().children, [new]);
        assert_eq!(loader.entries().len(), 2);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn active_group_condition_and_intercept_failures_cleanup_then_explicitly_recover() {
    for intercept in [false, true] {
        let runtime = Runtime::new();
        bounded(&runtime, async {
            let root = runtime.context();
            let loader = Loader::new(&root, number_catalog()).unwrap();
            let enabled = Rc::new(Cell::new(true));
            let key = ServiceKey::<()>::new("unused");
            let spec = if intercept {
                group(1, "child").intercept(Scope::new().intercept_with::<(), u32, _, BoxError>(
                    &key,
                    {
                        let enabled = enabled.clone();
                        move |_| {
                            if enabled.get() {
                                Ok(1)
                            } else {
                                Err("intercept failed".into())
                            }
                        }
                    },
                ))
            } else {
                group(1, "child").condition({
                    let enabled = enabled.clone();
                    move |_| {
                        if enabled.get() {
                            Ok(true)
                        } else {
                            Err::<bool, BoxError>("condition failed".into())
                        }
                    }
                })
            };
            let parent = loader.create(spec.clone()).unwrap();
            loader.settle().unwrap().await.unwrap();
            enabled.set(false);
            let failed = match loader.update(&parent, spec.clone()) {
                Ok(task) => task.await.is_err(),
                Err(_) => true,
            };
            assert!(failed);
            assert_eq!(loader.snapshot(&parent).unwrap().state, EntryState::Failed);
            assert!(
                loader
                    .entries()
                    .iter()
                    .all(|entry| entry.state != EntryState::Active)
            );
            assert!(!loader.settle().unwrap().await.unwrap().failures.is_empty());
            enabled.set(true);
            loader.update(&parent, spec).unwrap().await.unwrap();
            let report = loader.settle().unwrap().await.unwrap();
            assert!(report.failures.is_empty());
            assert_eq!(loader.snapshot(&parent).unwrap().state, EntryState::Active);
            assert_eq!(
                loader
                    .snapshot(&loader.resolve("group:child").unwrap())
                    .unwrap()
                    .state,
                EntryState::Active
            );
            loader.dispose().unwrap().await.unwrap();
        })
        .await;
    }
}

#[tokio::test(flavor = "current_thread")]
async fn nested_typed_and_json_source_duplicates_preserve_first_and_healthy_siblings() {
    for json in [false, true] {
        let runtime = Runtime::new();
        bounded(&runtime,async{
  let root=runtime.context();let mut catalog=number_catalog();catalog.register_json("json",Plugin::new("json",|_,_:u32|async{Ok::<_,Infallible>(())}),|v|serde_json::from_value::<u32>(v.cloned().unwrap())).unwrap();
  let loader=Loader::new(&root,catalog).unwrap();let source=SourceId::new("duplicates");
  for _ in 0..2{
   let doc=if json{SourceDocument::json(source.clone(),r#"[{"id":"group","group":true,"config":[{"id":"x","name":"json","config":1},{"id":"x","name":"json","config":2},{"id":"y","name":"json","config":3}]}]"#).unwrap()}
   else{SourceDocument::typed(source.clone(),vec![EntrySpec::group(Group::new(vec![EntrySpec::new("worker",Config::new(1_u32)).id("x"),EntrySpec::new("worker",Config::new(2_u32)).id("x"),EntrySpec::new("worker",Config::new(3_u32)).id("y")])).id("group")])};
   let report=loader.reconcile(doc.with_locations(|_|Some(SourceSpan{line:7,column:9}))).unwrap().await.unwrap();assert_eq!(report.entries.len(),3);assert_eq!(report.failures.len(),1);
   assert!(loader.resolve("group:y").is_ok());assert_eq!(*loader.snapshot(&loader.resolve("group:x").unwrap()).unwrap().source.get::<u32>().unwrap(),1);
   let LoaderError::Located{location,..}=&report.failures[0].error else{panic!("unlocated duplicate")};
   assert_eq!(location.source,Some(source.clone()));assert_eq!(location.path,vec![re_cordis_core::PathSegment::Index(0),re_cordis_core::PathSegment::Key("config".into()),re_cordis_core::PathSegment::Index(1)]);
  }
  loader.dispose().unwrap().await.unwrap();
 }).await;
    }
}

#[tokio::test(flavor = "current_thread")]
async fn group_to_plugin_replacement_removes_children_and_joins_gated_capture_cleanup() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let (started, entered) = oneshot::channel();
        let (release, gate) = oneshot::channel();
        let pause = Rc::new(RefCell::new(Some((started, gate))));
        let mut catalog = Catalog::new();
        catalog
            .register(
                "worker",
                Plugin::new("worker", move |ctx, capture: Rc<()>| {
                    let pause = pause.borrow_mut().take();
                    async move {
                        ctx.on_dispose(move || async move {
                            if let Some((started, gate)) = pause {
                                started.send(()).unwrap();
                                gate.await.unwrap();
                            }
                            drop(capture);
                            Ok::<_, Infallible>(())
                        })?;
                        Ok::<_, BoxError>(())
                    }
                }),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let capture = Rc::new(());
        let weak = Rc::downgrade(&capture);
        let parent = loader
            .create(
                EntrySpec::group(Group::new(vec![
                    EntrySpec::new("worker", Config::new(capture.clone())).id("child"),
                ]))
                .id("parent"),
            )
            .unwrap();
        drop(capture);
        loader.settle().unwrap().await.unwrap();
        let child = loader.resolve("parent:child").unwrap();
        let task = loader
            .update(&parent, EntrySpec::new("worker", Config::new(Rc::new(()))))
            .unwrap();
        entered.await.unwrap();
        let mut task = Box::pin(task);
        assert!(futures_util::poll!(&mut task).is_pending());
        release.send(()).unwrap();
        task.await.unwrap();
        let report = loader.settle().unwrap().await.unwrap();
        assert!(report.failures.is_empty());
        assert_eq!(report.entries.len(), 1);
        assert!(loader.snapshot(&child).is_none());
        assert!(loader.resolve("parent:child").is_err());
        assert!(loader.snapshot(&parent).unwrap().children.is_empty());
        assert!(weak.upgrade().is_none());
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn existing_ancestor_cycle_is_rejected_before_source_or_commit_changes() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let loader = Loader::new(&root, number_catalog()).unwrap();
        let outer = loader
            .create(
                EntrySpec::group(Group::new(vec![
                    EntrySpec::group(Group::new(vec![])).id("inner"),
                ]))
                .id("outer"),
            )
            .unwrap();
        loader.settle().unwrap().await.unwrap();
        let inner = loader.resolve("outer:inner").unwrap();
        let commits = Rc::new(Cell::new(0));
        root.on(
            &loader.commits(),
            Listener::sync({
                let commits = commits.clone();
                move |_, _| {
                    commits.set(commits.get() + 1);
                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                }
            }),
        )
        .unwrap();
        assert!(
            loader
                .update(
                    &inner,
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::group(Group::new(vec![])).id("outer")
                    ]))
                )
                .is_err()
        );
        assert_eq!(commits.get(), 0);
        assert!(loader.snapshot(&inner).unwrap().children.is_empty());
        let EntrySpec {
            kind: re_cordis_loader::EntryKind::Group(group),
            ..
        } = loader.snapshot(&inner).unwrap().options
        else {
            panic!("wrong kind")
        };
        assert!(group.entries.is_empty());
        assert_eq!(loader.snapshot(&inner).unwrap().parent, Some(outer.clone()));
        assert_eq!(loader.snapshot(&outer).unwrap().children, [inner]);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn malformed_nested_json_group_retains_source_config_path_and_span() {
    let runtime = Runtime::new();
    bounded(&runtime,async{
  let root=runtime.context();let loader=Loader::new(&root,Catalog::new()).unwrap();let source=SourceId::new("broken");
  let report=loader.reconcile(SourceDocument::json(source.clone(),r#"[{"id":"outer","group":true,"config":[{"id":"broken","group":true,"config":42}]}]"#).unwrap().with_locations(|_|Some(SourceSpan{line:7,column:9}))).unwrap().await.unwrap();
  assert_eq!(report.failures.len(),1);assert_eq!(report.failures[0].id.as_str(),"broken");let LoaderError::Located{location,source:cause}=&report.failures[0].error else{panic!("unlocated malformed group")};
  assert_eq!(location.source,Some(source));assert_eq!(location.span,Some(SourceSpan{line:7,column:9}));assert_eq!(location.path,vec![re_cordis_core::PathSegment::Index(0),re_cordis_core::PathSegment::Key("config".into()),re_cordis_core::PathSegment::Index(0),re_cordis_core::PathSegment::Key("config".into())]);assert!(cause.downcast_ref::<LoaderError>().is_some());
  loader.dispose().unwrap().await.unwrap();
 }).await;
}

#[tokio::test(flavor = "current_thread")]
async fn anonymous_positional_source_update_does_not_reclaim_another_sources_adoption() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let loader = Loader::new(&root, number_catalog()).unwrap();
        loader
            .reconcile(SourceDocument::typed(
                SourceId::new("A"),
                vec![EntrySpec::new("worker", Config::new(1_u32))],
            ))
            .unwrap()
            .await
            .unwrap();
        let id = loader.entries()[0].id.clone();
        loader
            .reconcile(SourceDocument::typed(
                SourceId::new("B"),
                vec![EntrySpec::new("worker", Config::new(2_u32)).id(id.as_str())],
            ))
            .unwrap()
            .await
            .unwrap();
        loader
            .reconcile(SourceDocument::typed(
                SourceId::new("A"),
                vec![EntrySpec::new("worker", Config::new(3_u32))],
            ))
            .unwrap()
            .await
            .unwrap();
        assert_eq!(loader.entries().len(), 2);
        let adopted = loader.snapshot(&id).unwrap();
        assert_eq!(adopted.origin, Origin::Source(SourceId::new("B")));
        assert_eq!(*adopted.source.get::<u32>().unwrap(), 2);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn named_realm_identity_survives_live_scopes_and_snapshots_then_recycles() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let key = ServiceKey::<u32>::new("value");
        let seen = Rc::new(RefCell::new(Vec::new()));
        let views = Rc::new(RefCell::new(Vec::new()));
        let mut catalog = Catalog::new();
        catalog.register_service(&key).unwrap();
        catalog
            .register_json(
                "worker",
                Plugin::new("worker", {
                    let key = key.clone();
                    let seen = seen.clone();
                    let views = views.clone();
                    move |ctx, (): ()| {
                        seen.borrow_mut().push(ctx.isolation(&key).unwrap());
                        views.borrow_mut().push(ctx);
                        async { Ok::<_, Infallible>(()) }
                    }
                }),
                |_| Ok::<_, Infallible>(()),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let text = r#"{"name":"worker","isolate":{"value":"shared"}}"#;
        let held = loader.decode_json(&format!("[{text}]")).unwrap();
        let first = loader.create(held[0].clone()).unwrap();
        loader.settle().unwrap().await.unwrap();
        let snapshot = loader.snapshot(&first).unwrap();
        loader.remove(&first).unwrap().await.unwrap();
        loader.settle().unwrap().await.unwrap();
        let second = loader.create_json(text).unwrap();
        loader.settle().unwrap().await.unwrap();
        assert_eq!(seen.borrow()[0], seen.borrow()[1]);
        drop(held);
        loader.remove(&second).unwrap().await.unwrap();
        loader.settle().unwrap().await.unwrap();
        let third = loader.create_json(text).unwrap();
        loader.settle().unwrap().await.unwrap();
        assert_eq!(seen.borrow()[0], seen.borrow()[2]);
        drop(snapshot);
        loader.remove(&third).unwrap().await.unwrap();
        loader.settle().unwrap().await.unwrap();
        loader.create_json(text).unwrap();
        loader.settle().unwrap().await.unwrap();
        assert_ne!(seen.borrow()[0], seen.borrow()[3]);
        assert_eq!(views.borrow()[0].isolation(&key).unwrap(), seen.borrow()[0]);
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn named_realm_live_definition_limit_releases_after_scopes_drop() {
    let runtime = Runtime::new();
    bounded(&runtime, async {
        let root = runtime.context();
        let key = ServiceKey::<u32>::new("value");
        let mut catalog = Catalog::new();
        catalog.register_service(&key).unwrap();
        catalog
            .register_json(
                "worker",
                Plugin::new("worker", |_, (): ()| async { Ok::<_, Infallible>(()) }),
                |_| Ok::<_, Infallible>(()),
            )
            .unwrap();
        let loader = Loader::new(&root, catalog).unwrap();
        let mut held = Vec::new();
        for index in 0..re_cordis_loader::MAX_ENTRIES {
            held.extend(
                loader
                    .decode_json(&format!(
                        r#"[{{"name":"worker","isolate":{{"value":"realm-{index}"}}}}]"#
                    ))
                    .unwrap(),
            );
        }
        assert!(
            loader
                .decode_json(r#"[{"name":"worker","isolate":{"value":"overflow"}}]"#)
                .is_err()
        );
        drop(held);
        assert!(
            loader
                .decode_json(r#"[{"name":"worker","isolate":{"value":"after-release"}}]"#)
                .is_ok()
        );
        loader.dispose().unwrap().await.unwrap();
    })
    .await;
}
