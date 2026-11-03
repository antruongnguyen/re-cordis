//! Ordered patch and provenance regressions.
mod support;
use re_cordis_core::Runtime;
use re_cordis_include::{Include, IncludeOptions, OwnerKind};
use serde_json::json;
use support::*;
#[tokio::test(flavor = "current_thread")]
async fn ordered_insert_override_disable_name_guard_and_patch_removal() {
    let d = Dir::new();
    d.put("a.json",r#"{"entries":[{"id":"a","name":"worker","config":1},{"id":"g","group":true,"config":[]}],"patches":[{"id":"a","name":"worker","config":2,"disabled":true},{"id":"a","name":"wrong","config":9},{"insert":[{"id":"new","name":"worker","config":3}]},{"id":"new","config":4},{"id":"g","insert":[{"id":"child","name":"worker"}]},{"id":"a","insert":[{"name":"worker"}]},{"id":"missing","config":0},{}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        let r = i.refresh().unwrap().await.unwrap();
        assert_eq!(r.failures.len(), 4);
        assert_eq!(i.data()[0]["config"], 2);
        assert_eq!(i.data()[0]["disabled"], true);
        assert_eq!(i.data()[2]["config"], 4);
        assert_eq!(
            i.ownership("a").unwrap().keys["config"].kind,
            OwnerKind::Patch
        );
        assert_eq!(i.ownership("new").unwrap().entry.kind, OwnerKind::Embedded);
        assert!(i.ownership("child").is_some());
        d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
        i.refresh().unwrap().await.unwrap();
        assert_eq!(i.data(), json!([{"id":"a","name":"worker","config":1}]));
        assert!(i.ownership("new").is_none());
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn group_replacement_clears_stale_children_and_later_patch_targets_replacement() {
    let d = Dir::new();
    d.put("a.yaml","entries:\n- id: g\n  group: true\n  config:\n  - id: old\n    name: worker\npatches:\n- id: g\n  config:\n  - id: fresh\n    name: worker\n    config: 1\n- id: old\n  config: 7\n- id: fresh\n  config: 2\n");
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.yaml")),
        )
        .unwrap();
        let r = i.refresh().unwrap().await.unwrap();
        assert_eq!(r.failures.len(), 1);
        assert!(i.ownership("old").is_none());
        assert_eq!(
            i.ownership("fresh").unwrap().entry.kind,
            OwnerKind::Embedded
        );
        assert_eq!(i.data()[0]["config"][0]["config"], 2);
        assert_eq!(i.ownership("fresh").unwrap().keys["config"].path.len(), 2);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn anonymous_ids_stay_stable_and_skip_explicit_ids_without_eager_write() {
    let d = Dir::new();
    let text = r#"{"entries":[{"name":"worker"},{"id":"include-anon-1","group":true,"config":[{"name":"worker"}]}],"patches":[{"insert":[{"id":"include-anon-2","name":"worker"},{"name":"worker"}]}]}"#;
    d.put("a.json", text);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        let first = i.data();
        assert_ne!(first[0]["id"], "include-anon-1");
        assert_ne!(first[0]["id"], "include-anon-2");
        d.put("a.json", &format!("{text}\n"));
        i.refresh().unwrap().await.unwrap();
        assert_eq!(i.data(), first);
        assert_eq!(
            std::fs::read_to_string(d.path("a.json")).unwrap(),
            format!("{text}\n")
        );
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn parent_patch_owns_nested_child_key_at_actual_parent_file() {
    let d = Dir::new();
    d.put(
        "child.json",
        r#"[{"id":"child","name":"worker","config":1}]"#,
    );
    d.put("a.json",r#"{"entries":[{"id":"inc","name":"include","config":{"path":"child.json"}}],"patches":[{"id":"child","config":2}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        let o = i.ownership("child").unwrap();
        assert_eq!(
            o.entry.file,
            std::fs::canonicalize(d.path("child.json")).unwrap()
        );
        assert_eq!(
            o.keys["config"].file,
            std::fs::canonicalize(d.path("a.json")).unwrap()
        );
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn group_replacement_collision_skips_patch_and_preserves_first_source_owner() {
    let d = Dir::new();
    d.put("a.json",r#"{"entries":[{"id":"a","name":"worker","config":1},{"id":"g","group":true,"config":[{"id":"old","name":"worker"}]}],"patches":[{"id":"g","config":[{"id":"a","name":"worker","config":9}]}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        let report = i.refresh().unwrap().await.unwrap();
        assert_eq!(report.failures.len(), 1);
        assert!(i.ownership("old").is_some());
        assert_eq!(i.ownership("a").unwrap().entry.kind, OwnerKind::File);
        assert_eq!(i.data()[0]["config"], 1);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
