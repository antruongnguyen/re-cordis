//! Linked replacement keeps Include's source ownership and pending journal intact.
mod support;
use re_cordis_core::{Plugin, Runtime};
use re_cordis_include::{Include, IncludeOptions, JsonEdit};
use re_cordis_loader::{Catalog, Config, FactoryReplacement};
use serde_json::json;
use std::{cell::RefCell, convert::Infallible, io, rc::Rc};
use support::*;

fn observed(seen: Rc<RefCell<Vec<u32>>>) -> Plugin<u32> {
    Plugin::new("linked", move |_, v: u32| {
        seen.borrow_mut().push(v);
        async { Ok::<_, Infallible>(()) }
    })
}
fn catalog(seen: Rc<RefCell<Vec<u32>>>) -> Catalog {
    let mut catalog = Catalog::new();
    catalog
        .register_json("worker", observed(seen), |v| {
            serde_json::from_value::<u32>(v.cloned().unwrap_or(json!(0)))
        })
        .unwrap();
    catalog
}
#[tokio::test(flavor = "current_thread")]
async fn replacement_preserves_source_bytes_data_provenance_and_pending_journal() {
    let dir = Dir::new();
    dir.put(
        "a.json",
        r#"[{"id":"a","name":"worker","config":1},{"id":"b","name":"worker","config":2}]"#,
    );
    let runtime = Runtime::new();
    run(&runtime, async {
        let initial = Rc::new(RefCell::new(Vec::new()));
        let include = Include::new(
            &runtime.context(),
            catalog(initial),
            IncludeOptions::new(dir.path("a.json")),
        )
        .unwrap();
        include.refresh().unwrap().await.unwrap();
        include
            .edit("a", JsonEdit::new().set("config", json!(9)))
            .unwrap()
            .await
            .unwrap();
        let before = include.entries();
        let data = include.data();
        let pending = include.pending();
        let provenance = format!("{:?}", include.ownership("a"));
        let bytes = std::fs::read(dir.path("a.json")).unwrap();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let report = include
            .replace_factories(vec![FactoryReplacement::new(
                "worker",
                observed(seen.clone()),
            )])
            .unwrap()
            .await
            .unwrap();
        assert!(!report.changed);
        assert!(report.failures.is_empty());
        assert_eq!(include.data(), data);
        assert_eq!(include.pending(), pending);
        assert_eq!(format!("{:?}", include.ownership("a")), provenance);
        assert_eq!(std::fs::read(dir.path("a.json")).unwrap(), bytes);
        let after = include.entries();
        for (old, new) in before.iter().zip(&after) {
            assert_eq!(old.id, new.id);
            assert_eq!(old.origin, new.origin);
            assert_eq!(old.transport, new.transport);
            assert_ne!(old.factory_id, new.factory_id);
        }
        let mut actual = seen.borrow().clone();
        actual.sort();
        assert_eq!(actual, [2, 9]);
        include.flush().unwrap().await.unwrap();
        seen.borrow_mut().clear();
        dir.put(
            "a.json",
            r#"[{"id":"a","name":"worker","config":3},{"id":"b","name":"worker","config":4}]"#,
        );
        include.refresh().unwrap().await.unwrap();
        let mut actual = seen.borrow().clone();
        actual.sort();
        assert_eq!(actual, [3, 4]);
        include.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn preflight_rejection_keeps_old_instances_and_include_remains_usable() {
    let dir = Dir::new();
    dir.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
    let runtime = Runtime::new();
    run(&runtime, async {
        let include = Include::new(
            &runtime.context(),
            catalog(Rc::new(RefCell::new(Vec::new()))),
            IncludeOptions::new(dir.path("a.json")),
        )
        .unwrap();
        include.refresh().unwrap().await.unwrap();
        let before = include.entries()[0].clone();
        let candidate =
            FactoryReplacement::new("worker", observed(Rc::new(RefCell::new(Vec::new()))))
                .with_preflight({
                    let include = include.clone();
                    move |config: &Config| {
                        assert_eq!(*config.get::<u32>().unwrap(), 1);
                        assert!(
                            include
                                .edit("a", JsonEdit::new().set("config", json!(8)))
                                .is_err()
                        );
                        assert!(include.dispose().is_err());
                        Err::<(), _>(io::Error::other("preflight rejected"))
                    }
                });
        let error = include.replace_factories(vec![candidate]).err().unwrap();
        let mut cause: &dyn std::error::Error = &error;
        while let Some(source) = cause.source() {
            cause = source;
        }
        assert_eq!(
            cause.downcast_ref::<io::Error>().unwrap().to_string(),
            "preflight rejected"
        );
        assert_eq!(include.entries()[0].fiber_id, before.fiber_id);
        assert_eq!(include.entries()[0].factory_id, before.factory_id);
        assert_eq!(include.pending(), 0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        include
            .replace_factories(vec![FactoryReplacement::new(
                "worker",
                observed(seen.clone()),
            )])
            .unwrap()
            .await
            .unwrap();
        assert_eq!(*seen.borrow(), [1]);
        include.dispose().unwrap().await.unwrap();
    })
    .await;
}
