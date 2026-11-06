//! File reading and retained-state regressions.
mod support;
use re_cordis_core::Runtime;
use re_cordis_include::{Include, IncludeOptions};
use serde_json::json;
use support::*;
#[tokio::test(flavor = "current_thread")]
async fn missing_initial_is_not_written_on_read_and_existing_invalid_never_overwritten() {
    let d = Dir::new();
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")).initial(json!([{"id":"a","name":"worker"}])),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        assert!(!d.path("a.json").exists());
        assert_eq!(i.entries().len(), 1);
        i.dispose().unwrap().await.unwrap();
        assert_eq!(read(&d.path("a.json"))[0]["id"], "a");
        d.put("bad.json", "broken");
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("bad.json")).initial(json!([])),
        )
        .unwrap();
        assert!(i.refresh().is_err());
        i.dispose().unwrap().await.unwrap();
        assert_eq!(
            std::fs::read_to_string(d.path("bad.json")).unwrap(),
            "broken"
        );
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn refresh_deduplicates_preserves_last_valid_and_recovers() {
    let d = Dir::new();
    d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        assert!(i.refresh().unwrap().await.unwrap().changed);
        let before = i.entries()[0].fiber_id.clone();
        assert!(!i.refresh().unwrap().await.unwrap().changed);
        assert_eq!(i.entries()[0].fiber_id, before);
        d.put("a.json", "{");
        assert!(i.refresh().is_err());
        assert_eq!(i.entries().len(), 1);
        d.put(
            "a.json",
            r#"[{"id":"b","name":"worker","config":2},{"id":"bad","name":"unknown"}]"#,
        );
        let r = i.refresh().unwrap().await.unwrap();
        assert_eq!(r.failures.len(), 1);
        assert_eq!(i.entries()[0].id.as_str(), "b");
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn nested_sources_groups_cycles_and_native_relative_paths() {
    let d = Dir::new();
    d.put("child.json", r#"[{"id":"child","name":"worker"}]"#);
    d.put(
        "a.json",
        r#"[{"id":"inc","name":"include","config":{"path":"child.json"}}]"#,
    );
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        assert_eq!(i.entries().len(), 2);
        assert!(
            i.entries()
                .iter()
                .any(|e| e.id.as_str() == "child" && e.parent.as_ref().unwrap().as_str() == "inc")
        );
        d.put(
            "child.json",
            r#"[{"name":"include","config":{"path":"a.json"}}]"#,
        );
        assert!(i.refresh().is_err());
        assert_eq!(i.entries().len(), 2);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
