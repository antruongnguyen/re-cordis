//! Journal reconciliation and actual-owner persistence.
mod support;
use re_cordis_core::Runtime;
use re_cordis_include::{Include, IncludeOptions, JsonEdit};
use serde_json::json;
use support::*;
#[tokio::test(flavor = "current_thread")]
async fn coalesced_update_create_remove_and_dropped_waiter_are_durable() {
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
        i.refresh().unwrap().await.unwrap();
        drop(
            i.edit("a", JsonEdit::new().set("config", json!(2)))
                .unwrap(),
        );
        i.edit("a", JsonEdit::new().set("config", json!(3)))
            .unwrap()
            .await
            .unwrap();
        assert_eq!(i.pending(), 1);
        i.create(json!({"id":"new","name":"worker"}), None, 0)
            .unwrap()
            .await
            .unwrap();
        i.remove("new").unwrap().await.unwrap();
        assert_eq!(i.pending(), 1);
        i.flush().unwrap().await.unwrap();
        assert_eq!(read(&d.path("a.json"))[0]["config"], 3);
        assert_eq!(i.pending(), 0);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn file_wins_same_key_but_keeps_independent_runtime_key() {
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
        i.refresh().unwrap().await.unwrap();
        i.edit(
            "a",
            JsonEdit::new()
                .set("config", json!(2))
                .set("disabled", json!(true)),
        )
        .unwrap()
        .await
        .unwrap();
        d.put("a.json", r#"[{"id":"a","name":"worker","config":3}]"#);
        let r = i.flush().unwrap().await.unwrap();
        assert_eq!(r.conflicts.len(), 1);
        assert_eq!(r.conflicts[0].key.as_deref(), Some("config"));
        let v = read(&d.path("a.json"));
        assert_eq!(v[0]["config"], 3);
        assert_eq!(v[0]["disabled"], true);
        assert_eq!(i.data(), v);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn refresh_before_update_and_update_before_refresh_use_current_snapshot() {
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
        i.refresh().unwrap().await.unwrap();
        d.put("a.json", r#"[{"id":"a","name":"worker","config":3}]"#);
        i.refresh().unwrap().await.unwrap();
        i.edit("a", JsonEdit::new().set("config", json!(2)))
            .unwrap()
            .await
            .unwrap();
        assert!(i.flush().unwrap().await.unwrap().conflicts.is_empty());
        assert_eq!(read(&d.path("a.json"))[0]["config"], 2);
        i.edit("a", JsonEdit::new().set("config", json!(4)))
            .unwrap()
            .await
            .unwrap();
        d.put("a.json", r#"[{"id":"a","name":"worker","config":5}]"#);
        assert_eq!(i.refresh().unwrap().await.unwrap().conflicts.len(), 1);
        assert_eq!(i.data()[0]["config"], 5);
        assert_eq!(i.pending(), 0);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn file_removal_and_modified_entry_defeat_runtime_update_and_removal() {
    let d = Dir::new();
    d.put(
        "a.json",
        r#"[{"id":"a","name":"worker","config":1},{"id":"b","name":"worker","config":1}]"#,
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
        i.edit("a", JsonEdit::new().set("config", json!(2)))
            .unwrap()
            .await
            .unwrap();
        i.remove("b").unwrap().await.unwrap();
        d.put("a.json", r#"[{"id":"b","name":"worker","config":3}]"#);
        let r = i.flush().unwrap().await.unwrap();
        assert_eq!(r.conflicts.len(), 2);
        assert_eq!(i.data(), json!([{"id":"b","name":"worker","config":3}]));
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn route_file_patch_insert_embedded_group_and_moves_plus_updates() {
    let d = Dir::new();
    d.put(
        "child.json",
        r#"[{"id":"file","name":"worker","config":1}]"#,
    );
    d.put("a.json",r#"{"entries":[{"id":"inc","name":"include","config":{"path":"child.json"}},{"id":"g","group":true,"config":[]}],"patches":[{"id":"file","disabled":true},{"insert":[{"id":"insert","name":"worker","config":1}]},{"id":"g","config":[{"id":"embedded","name":"worker","config":1}]}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        for id in ["file", "insert", "embedded"] {
            i.edit(id, JsonEdit::new().set("config", json!(2)))
                .unwrap()
                .await
                .unwrap();
        }
        i.edit("file", JsonEdit::new().set("disabled", json!(false)))
            .unwrap()
            .await
            .unwrap();
        i.flush().unwrap().await.unwrap();
        assert_eq!(read(&d.path("child.json"))[0]["config"], 2);
        let v = read(&d.path("a.json"));
        assert_eq!(v["entries"][0]["name"], "include");
        assert_eq!(v["entries"][0]["config"], json!({"path":"child.json"}));
        assert_eq!(v["patches"][0]["disabled"], false);
        assert_eq!(v["patches"][1]["insert"][0]["config"], 2);
        assert_eq!(v["patches"][2]["config"][0]["config"], 2);
        i.move_entry("insert", Some("g"), 0).unwrap().await.unwrap();
        i.edit("insert", JsonEdit::new().set("config", json!(4)))
            .unwrap()
            .await
            .unwrap();
        i.flush().unwrap().await.unwrap();
        let v = read(&d.path("a.json"));
        assert!(v["patches"][1]["insert"].as_array().unwrap().is_empty());
        assert_eq!(v["patches"][2]["config"][0]["id"], "insert");
        assert_eq!(v["patches"][2]["config"][0]["config"], 4);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn name_guard_transition_routes_later_config_to_new_effective_owner() {
    use re_cordis_core::Plugin;
    use std::convert::Infallible;
    let d = Dir::new();
    d.put("a.json",r#"{"entries":[{"id":"a","name":"worker","config":1}],"patches":[{"id":"a","name":"worker","config":5}]}"#);
    let mut c = catalog();
    c.register_json(
        "other",
        Plugin::new("other", |_, _: Option<serde_json::Value>| async {
            Ok::<_, Infallible>(())
        }),
        |v| Ok::<_, Infallible>(v.cloned()),
    )
    .unwrap();
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(&rt.context(), c, IncludeOptions::new(d.path("a.json"))).unwrap();
        i.refresh().unwrap().await.unwrap();
        i.edit("a", JsonEdit::new().set("name", json!("other")))
            .unwrap()
            .await
            .unwrap();
        i.edit("a", JsonEdit::new().set("config", json!(2)))
            .unwrap()
            .await
            .unwrap();
        i.flush().unwrap().await.unwrap();
        let raw = read(&d.path("a.json"));
        assert_eq!(raw["entries"][0]["config"], 2);
        assert_eq!(raw["patches"][0]["config"], 5);
        assert_eq!(i.data()[0]["config"], 2);
        assert!(!i.ownership("a").unwrap().keys.contains_key("config"));
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
