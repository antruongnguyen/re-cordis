//! Typed service transport, source tree and ownership regressions.
use re_cordis_core::{BoxError, Event, Listener, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{
    Catalog, Config, EntrySpec, EntryState, Group, Loader, SourceDocument, SourceId,
};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    ops::ControlFlow,
    rc::Rc,
};
use tokio::sync::oneshot;
#[tokio::test(flavor = "current_thread")]
async fn typed_serialized_inject_isolate_and_intercept_are_checked() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let key = ServiceKey::<u32>::new("value");
            let mut catalog = Catalog::new();
            catalog
                .register_service_config(&key, |v| serde_json::from_value::<u32>(v.clone()))
                .unwrap();
            let seen = Rc::new(RefCell::new(Vec::new()));
            catalog
                .register_json(
                    "consumer",
                    Plugin::new("consumer", {
                        let seen = seen.clone();
                        let key = key.clone();
                        move |ctx, (): ()| {
                            let binding = ctx.bind(&key).unwrap();
                            let config = *binding.intercepts::<u32>().unwrap()[0];
                            seen.borrow_mut().push(config);
                            async { Ok::<_, Infallible>(()) }
                        }
                    }),
                    |_| Ok::<_, Infallible>(()),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            root.provide(&key, 1).unwrap();
            loader
                .create_json(r#"{"name":"consumer","inject":["value"],"intercept":{"value":8}}"#)
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            assert_eq!(*seen.borrow(), [8]);
            assert!(
                loader
                    .create_json(r#"{"name":"consumer","inject":["unknown"]}"#)
                    .is_err()
            );
            assert!(
                loader
                    .create_json(r#"{"name":"consumer","intercept":{"value":"bad"}}"#)
                    .is_err()
            );
            loader.dispose().unwrap().await.unwrap();
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn group_reconfigure_disable_enable_preserves_children_identity_for_100_cycles() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let applied = Rc::new(Cell::new(0));
            let closed = Rc::new(Cell::new(0));
            let event = Event::<()>::named("tick");
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", {
                        let a = applied.clone();
                        let c = closed.clone();
                        move |ctx, v: u32| {
                            let a = a.clone();
                            let c = c.clone();
                            let event = event.clone();
                            async move {
                                assert!(v <= 100);
                                a.set(a.get() + 1);
                                ctx.on(
                                    &event,
                                    Listener::sync(|_, _| {
                                        Ok::<_, Infallible>(ControlFlow::Continue(()))
                                    }),
                                )?;
                                ctx.on_dispose_sync(move || {
                                    c.set(c.get() + 1);
                                    Ok::<_, Infallible>(())
                                })?;
                                Ok::<_, BoxError>(())
                            }
                        }
                    }),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let group = loader
                .create(
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(0_u32)).id("child"),
                    ]))
                    .id("group"),
                )
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let child = loader.resolve("group:child").unwrap();
            for value in 1_u32..=100 {
                loader
                    .update(
                        &group,
                        EntrySpec::group(Group::new(vec![
                            EntrySpec::new("worker", Config::new(value)).id("child"),
                        ])),
                    )
                    .unwrap()
                    .await
                    .unwrap();
                let report = loader.settle().unwrap().await.unwrap();
                assert!(report.failures.is_empty());
                assert_eq!(loader.resolve("group:child").unwrap(), child);
            }
            loader
                .update(
                    &group,
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(100_u32)).id("child"),
                    ]))
                    .disabled(true),
                )
                .unwrap()
                .await
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            assert_eq!(loader.snapshot(&child).unwrap().state, EntryState::Disabled);
            loader
                .update(
                    &group,
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(100_u32)).id("child"),
                    ])),
                )
                .unwrap()
                .await
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            loader.dispose().unwrap().await.unwrap();
            assert_eq!(applied.get(), closed.get());
            assert!(root.registry().is_empty().unwrap());
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn parent_disposal_joins_manually_detached_cleanup_after_loader_handles_drop() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let (started, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let gates = Rc::new(RefCell::new(Some((started, gate))));
            let closed = Rc::new(Cell::new(false));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", {
                        let gates = gates.clone();
                        let closed = closed.clone();
                        move |ctx, (): ()| {
                            let pause = gates.borrow_mut().take();
                            let closed = closed.clone();
                            async move {
                                ctx.on_dispose(move || async move {
                                    if let Some((started, gate)) = pause {
                                        started.send(()).unwrap();
                                        gate.await.unwrap();
                                    }
                                    closed.set(true);
                                    Ok::<_, Infallible>(())
                                })?;
                                Ok::<_, BoxError>(())
                            }
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
            drop(loader);
            let disposal = root.drain().unwrap();
            let mut disposal = Box::pin(disposal);
            assert!(futures_util::poll!(&mut disposal).is_pending());
            assert!(!closed.get());
            release.send(()).unwrap();
            disposal.await.unwrap();
            assert!(closed.get());
            assert!(root.registry().is_empty().unwrap());
        })
        .await
        .unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn source_nested_invalid_child_preserves_healthy_sibling_and_source_location() {
    let runtime = Runtime::new();
    runtime.run(async{
  let root=runtime.context();let mut catalog=Catalog::new();catalog.register_json("worker",Plugin::new("worker",|_,v:u32|async move{assert_eq!(v,1);Ok::<_,Infallible>(())}),|v|serde_json::from_value::<u32>(v.cloned().unwrap())).unwrap();
  let loader=Loader::new(&root,catalog).unwrap();
  let report=loader.reconcile(SourceDocument::json(SourceId::new("memory"),r#"[{"id":"group","group":true,"config":[{"id":"good","name":"worker","config":1},{"id":"bad","name":"worker","config":"bad"}]}]"#).unwrap()).unwrap().await.unwrap();
  assert_eq!(report.failures.len(),1);assert!(loader.resolve("group:good").is_ok());assert!(loader.resolve("group:bad").is_err());
  loader.dispose().unwrap().await.unwrap();
 }).await.unwrap();
}
