//! Bounded loader observations paired with the pinned TypeScript loader.
use re_cordis_core::{BoxError, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, Group, Loader};
use std::{cell::Cell, cell::RefCell, convert::Infallible, rc::Rc, time::Duration};

fn assert_trace(name: &str, expected: &str, trace: &[String]) {
    let actual = trace.join("\n") + "\n";
    assert_eq!(actual, expected, "{name} loader observations");
    if let Some(output) = std::env::var_os("CORDIS_LOADER_PARITY_OUTPUT") {
        std::fs::write(
            std::path::Path::new(&output).join(format!("{name}.txt")),
            actual,
        )
        .unwrap();
    }
}

#[tokio::test(flavor = "current_thread")]
async fn dependency_arrival_failure_and_explicit_recovery() {
    let runtime = Runtime::new();
    tokio::time::timeout(
        Duration::from_secs(10),
        runtime.run(async {
            let root = runtime.context();
            let key = ServiceKey::<()>::new("settings");
            let values = Rc::new(RefCell::new(Vec::new()));
            let releases = Rc::new(Cell::new(0));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", {
                        let values = values.clone();
                        let releases = releases.clone();
                        move |ctx, value: i32| {
                            let values = values.clone();
                            let releases = releases.clone();
                            async move {
                                values.borrow_mut().push(value);
                                if value < 0 {
                                    return Err::<(), BoxError>("rejected source".into());
                                }
                                ctx.on_dispose_sync(move || {
                                    releases.set(releases.get() + 1);
                                    Ok::<_, Infallible>(())
                                })?;
                                Ok::<_, BoxError>(())
                            }
                        }
                    })
                    .injecting([key.required()]),
                )
                .unwrap();
            let loader = Loader::new(&root, catalog).unwrap();
            let id = loader
                .create(EntrySpec::new("worker", Config::new(-1_i32)).id("worker"))
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let mut trace = vec![format!(
                "pending state={:?} values={:?}",
                loader.snapshot(&id).unwrap().state,
                values.borrow()
            )];
            root.provide(&key, ()).unwrap();
            let failed = loader.settle().unwrap().await.unwrap();
            assert_eq!(failed.failures.len(), 1);
            trace.push(format!(
                "arrived state={:?} values={:?}",
                loader.snapshot(&id).unwrap().state,
                values.borrow()
            ));
            loader
                .update(&id, EntrySpec::new("worker", Config::new(2_i32)))
                .unwrap()
                .await
                .unwrap();
            let recovered = loader.settle().unwrap().await.unwrap();
            assert!(recovered.failures.is_empty());
            trace.push(format!(
                "recovered state={:?} values={:?}",
                loader.snapshot(&id).unwrap().state,
                values.borrow()
            ));
            loader.remove(&id).unwrap().await.unwrap();
            trace.push(format!(
                "removed entries={} releases={}",
                loader.entries().len(),
                releases.get()
            ));
            loader.dispose().unwrap().await.unwrap();
            assert!(root.registry().is_empty().unwrap());
            assert_trace(
                "recovery",
                include_str!("fixtures/loader_parity/recovery.txt"),
                &trace,
            );
        }),
    )
    .await
    .expect("loader recovery scenario timed out")
    .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn structural_disabled_group_controls_child_cleanup() {
    let runtime = Runtime::new();
    tokio::time::timeout(
        Duration::from_secs(10),
        runtime.run(async {
            let root = runtime.context();
            let applies = Rc::new(Cell::new(0));
            let releases = Rc::new(Cell::new(0));
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", {
                        let applies = applies.clone();
                        let releases = releases.clone();
                        move |ctx, (): ()| {
                            applies.set(applies.get() + 1);
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
            let spec = || {
                EntrySpec::group(Group::new(vec![
                    EntrySpec::new("worker", Config::new(())).id("child"),
                ]))
                .id("group")
            };
            let group = Loader::new(&root, catalog).unwrap();
            let id = group.create(spec().disabled(true)).unwrap();
            group.settle().unwrap().await.unwrap();
            let child = group.resolve("group:child").unwrap();
            let mut trace = Vec::new();
            for (phase, disabled) in [
                ("created", true),
                ("enabled", false),
                ("disabled", true),
                ("reenabled", false),
            ] {
                if phase != "created" {
                    group
                        .update(&id, spec().disabled(disabled))
                        .unwrap()
                        .await
                        .unwrap();
                    group.settle().unwrap().await.unwrap();
                }
                assert_eq!(group.resolve("group:child").unwrap(), child);
                trace.push(format!(
                    "{phase} group={:?} child={:?} entries={} applies={} releases={}",
                    group.snapshot(&id).unwrap().state,
                    group.snapshot(&child).unwrap().state,
                    group.entries().len(),
                    applies.get(),
                    releases.get()
                ));
            }
            group.dispose().unwrap().await.unwrap();
            trace.push(format!(
                "disposed applies={} releases={}",
                applies.get(),
                releases.get()
            ));
            assert!(root.registry().is_empty().unwrap());
            assert_trace(
                "group",
                include_str!("fixtures/loader_parity/group.txt"),
                &trace,
            );
        }),
    )
    .await
    .expect("loader group scenario timed out")
    .unwrap();
}
