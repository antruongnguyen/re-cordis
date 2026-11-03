//! Public regressions for final independent include review IR1–IR5.
mod support;
use re_cordis_core::Runtime;
use re_cordis_include::{Include, IncludeOptions, JsonEdit};
use serde_json::{Value, json};
use support::*;

#[tokio::test(flavor = "current_thread")]
async fn created_child_survives_pending_parent_and_failed_flush() {
    let d = Dir::new();
    d.put("a.json", "[]");
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        i.create(json!({"id":"g","group":true,"config":[]}), None, 0)
            .unwrap()
            .await
            .unwrap();
        i.create(
            json!({"id":"child","name":"worker","config":7}),
            Some("g"),
            0,
        )
        .unwrap()
        .await
        .unwrap();
        d.put("a.json", "broken");
        assert!(i.flush().is_err());
        assert_eq!(i.pending(), 2);
        d.put("a.json", "[]");
        let r = i.refresh().unwrap().await.unwrap();
        assert!(r.conflicts.is_empty(), "{:?}", r.conflicts);
        assert_eq!(i.data()[0]["config"][0]["config"], 7);
        assert_eq!(i.data(), read(&d.path("a.json")));
        assert_eq!(i.pending(), 0);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn move_and_edit_into_pending_parent_survive_both_orders() {
    for edit_first in [true, false] {
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
            i.create(json!({"id":"g","group":true,"config":[]}), None, 0)
                .unwrap()
                .await
                .unwrap();
            if edit_first {
                i.edit("a", JsonEdit::new().set("config", json!(8)))
                    .unwrap()
                    .await
                    .unwrap();
            }
            i.move_entry("a", Some("g"), 0).unwrap().await.unwrap();
            if !edit_first {
                i.edit("a", JsonEdit::new().set("config", json!(8)))
                    .unwrap()
                    .await
                    .unwrap();
            }
            let r = i.flush().unwrap().await.unwrap();
            assert!(r.conflicts.is_empty(), "{:?}", r.conflicts);
            assert_eq!(
                i.data(),
                json!([{"id":"g","group":true,"config":[{"id":"a","name":"worker","config":8}]}])
            );
            assert_eq!(read(&d.path("a.json")), i.data());
            i.dispose().unwrap().await.unwrap();
        })
        .await;
    }
}

#[tokio::test(flavor = "current_thread")]
async fn removed_group_deletes_moved_baseline_child_and_cancels_created_child() {
    for created_group in [false, true] {
        let d = Dir::new();
        let entries = if created_group {
            json!([{"id":"a","name":"worker","config":1}])
        } else {
            json!([{"id":"a","name":"worker","config":1},{"id":"g","group":true,"config":[]}])
        };
        d.put("a.json", &entries.to_string());
        let rt = Runtime::new();
        run(&rt, async {
            let i = Include::new(
                &rt.context(),
                catalog(),
                IncludeOptions::new(d.path("a.json")),
            )
            .unwrap();
            i.refresh().unwrap().await.unwrap();
            if created_group {
                i.create(json!({"id":"g","group":true,"config":[]}), None, 0)
                    .unwrap()
                    .await
                    .unwrap();
            }
            i.move_entry("a", Some("g"), 0).unwrap().await.unwrap();
            i.create(json!({"id":"new","name":"worker","config":2}), Some("g"), 1)
                .unwrap()
                .await
                .unwrap();
            i.remove("g").unwrap().await.unwrap();
            let r = i.flush().unwrap().await.unwrap();
            assert!(r.conflicts.is_empty());
            assert_eq!(i.data(), json!([]));
            assert_eq!(read(&d.path("a.json")), json!([]));
            assert_eq!(i.pending(), 0);
            i.dispose().unwrap().await.unwrap();
        })
        .await;
    }
}

#[tokio::test(flavor = "current_thread")]
async fn opaque_plugin_arrays_are_unchanged_in_source_patch_and_runtime_create() {
    let config = json!([{"name":"Alice"},{"group":true,"config":[{"name":"include","config":{"path":"does-not-exist.json"}}]},{"id":"include-anon-1"}]);
    let d = Dir::new();
    d.put("a.json", &json!({"entries":[{"id":"a","name":"worker","config":config},{"name":"worker"}],"patches":[{"id":"a","config":config}]}).to_string());
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
        assert_eq!(i.data()[0]["config"], config);
        assert_eq!(i.data()[1]["id"], "include-anon-1");
        i.create(json!({"id":"new","name":"worker","config":config}), None, 2)
            .unwrap()
            .await
            .unwrap();
        assert_eq!(i.data()[2]["config"], config);
        i.edit("a", JsonEdit::new().set("disabled", json!(true)))
            .unwrap()
            .await
            .unwrap();
        i.flush().unwrap().await.unwrap();
        assert_eq!(read(&d.path("a.json"))["entries"][0]["config"], config);
        assert_eq!(read(&d.path("a.json"))["patches"][0]["config"], config);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn replacement_patch_loads_relative_include_and_preserves_child_file_owner() {
    let d = Dir::new();
    std::fs::create_dir(d.path("nested")).unwrap();
    d.put(
        "nested/child.json",
        r#"[{"id":"a","name":"worker","config":1}]"#,
    );
    d.put("nested/patch.json", r#"{"entries":[{"id":"g","group":true,"config":[]}],"patches":[{"id":"g","config":[{"id":"inc","name":"include","config":{"path":"child.json"}}]}]}"#);
    d.put(
        "a.json",
        r#"[{"id":"outer","name":"include","config":{"path":"nested/patch.json"}}]"#,
    );
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
        assert_eq!(
            i.ownership("a").unwrap().entry.file,
            std::fs::canonicalize(d.path("nested/child.json")).unwrap()
        );
        i.edit("a", JsonEdit::new().set("config", json!(9)))
            .unwrap()
            .await
            .unwrap();
        i.flush().unwrap().await.unwrap();
        assert_eq!(read(&d.path("nested/child.json"))[0]["config"], 9);
        assert_eq!(
            read(&d.path("nested/patch.json"))["patches"][0]["config"][0]["config"]["path"],
            "child.json"
        );
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn patch_owned_removal_preserves_effective_absence_and_hidden_base() {
    let d = Dir::new();
    let base = json!({"id":"a","name":"worker","config":1,"disabled":true});
    d.put(
        "a.json",
        &json!({"entries":[base],"patches":[{"id":"a","config":2,"disabled":false}]}).to_string(),
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
        i.edit("a", JsonEdit::new().remove("disabled").remove("config"))
            .unwrap()
            .await
            .unwrap();
        assert!(i.data()[0].get("config").is_none());
        let r = i.flush().unwrap().await.unwrap();
        assert!(r.conflicts.is_empty());
        assert!(i.data()[0].get("disabled").is_none());
        assert!(i.data()[0].get("config").is_none());
        assert_eq!(read(&d.path("a.json"))["entries"][0], base);
        assert!(i.ownership("a").unwrap().keys.contains_key("config"));
        i.refresh().unwrap().await.unwrap();
        assert!(i.data()[0].get("config").is_none());
        i.edit(
            "a",
            JsonEdit::new()
                .set("config", Value::Null)
                .set("disabled", json!(false)),
        )
        .unwrap()
        .await
        .unwrap();
        i.flush().unwrap().await.unwrap();
        assert_eq!(i.data()[0].get("config"), Some(&Value::Null));
        assert_eq!(i.data()[0]["disabled"], false);
        assert_eq!(read(&d.path("a.json"))["entries"][0], base);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn earlier_patch_config_becomes_structural_only_when_group_conversion_applies() {
    let d = Dir::new();
    d.put("child.json", r#"[{"id":"include-anon-2","name":"worker"}]"#);
    d.put("a.json", r#"{"entries":[{"name":"worker"},{"id":"a","name":"worker","config":[]}],"patches":[{"id":"a","config":[{"id":"include-anon-1","name":"include","config":{"path":"child.json"}}]},{"id":"a","group":true}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        let report = i.refresh().unwrap().await.unwrap();
        assert!(report.failures.is_empty(), "{:?}", report.failures);
        assert_eq!(i.data()[0]["id"], "include-anon-3");
        assert_eq!(i.data()[1]["config"][0]["group"], true);
        assert_eq!(
            i.data()[1]["config"][0]["config"][0]["id"],
            "include-anon-2"
        );
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn skipped_opaque_and_unset_patch_configs_do_not_load_sources_or_reserve_ids() {
    let d = Dir::new();
    let opaque = json!([{"id":"include-anon-1","name":"include","config":{"path":"missing.json"}}]);
    d.put("a.json", &json!({"entries":[{"name":"worker"},{"id":"a","name":"worker"},{"id":"g","group":true,"config":[]}],"patches":[
        {"id":"missing","insert":opaque},
        {"id":"a","name":"other","group":true,"config":opaque},
        {"id":"a","config":opaque},
        {"id":"g","config":opaque,"unset":["config"]},
        {"id":"g","group":true,"config":opaque,"unset":["group"]}
    ]}).to_string());
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        let report = i.refresh().unwrap().await.unwrap();
        assert_eq!(i.data()[0]["id"], "include-anon-1");
        assert_eq!(i.data()[1]["config"], opaque);
        assert_eq!(i.data()[2]["config"], opaque);
        assert!(i.data()[2].get("group").is_none());
        assert_eq!(report.failures.len(), 3); // Two skipped patches and final invalid plugin g.
        assert!(i.ownership("g").unwrap().keys.contains_key("group"));
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn inserted_plugin_config_is_not_loaded_before_later_group_replacement() {
    let d = Dir::new();
    d.put("child.json", r#"[{"id":"child","name":"worker"}]"#);
    d.put("a.json", r#"{"entries":[],"patches":[{"insert":[{"id":"a","name":"worker","config":[{"name":"include","config":{"path":"missing.json"}}]}]},{"id":"a","group":true,"config":[{"name":"include","config":{"path":"child.json"}}]}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
        assert_eq!(i.data()[0]["config"][0]["config"][0]["id"], "child");
        assert!(
            read(&d.path("a.json"))["patches"][0]["insert"][0]["config"][0]
                .get("id")
                .is_none()
        );
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn patch_unset_rejects_control_keys_and_preserves_name_guard() {
    let d = Dir::new();
    d.put("a.json", r#"{"entries":[{"id":"a","name":"worker","config":1,"disabled":true}],"patches":[{"id":"a","unset":["id"]},{"id":"a","name":"other","unset":["disabled"]},{"id":"a","name":"worker","disabled":false,"unset":["disabled"]}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(&rt.context(), catalog(), IncludeOptions::new(d.path("a.json"))).unwrap();
        let report = i.refresh().unwrap().await.unwrap();
        assert_eq!(report.failures.len(), 2);
        assert_eq!(i.data()[0]["id"], "a");
        assert!(i.data()[0].get("disabled").is_none());
        i.edit("a", JsonEdit::new().set("disabled", json!(true))).unwrap().await.unwrap();
        i.flush().unwrap().await.unwrap();
        assert!(read(&d.path("a.json"))["patches"][2].get("unset").is_none());
        d.put("a.json", r#"{"entries":[{"id":"a","name":"worker","config":1,"disabled":true}],"patches":[{"id":"a","name":"other","unset":["disabled"]}]}"#);
        i.refresh().unwrap().await.unwrap();
        assert_eq!(i.data()[0]["disabled"], true);
        assert!(!i.ownership("a").unwrap().keys.contains_key("disabled"));
        i.dispose().unwrap().await.unwrap();
    }).await;
}

#[tokio::test(flavor = "current_thread")]
async fn effective_removal_routes_to_last_patch_and_direct_embedded_owner() {
    let d = Dir::new();
    d.put("a.json", r#"{"entries":[{"id":"a","name":"worker","config":1,"disabled":true}],"patches":[{"id":"a","config":2},{"id":"a","config":3},{"insert":[{"id":"b","name":"worker","config":4}]}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        for id in ["a", "b"] {
            i.edit(id, JsonEdit::new().remove("config"))
                .unwrap()
                .await
                .unwrap();
        }
        i.flush().unwrap().await.unwrap();
        let raw = read(&d.path("a.json"));
        assert_eq!(raw["entries"][0]["config"], 1);
        assert_eq!(raw["patches"][0]["config"], 2);
        assert_eq!(raw["patches"][1]["unset"], json!(["config"]));
        assert!(raw["patches"][2]["insert"][0].get("config").is_none());
        assert!(raw["patches"][2]["insert"][0].get("unset").is_none());
        assert!(i.data()[0].get("config").is_none());
        assert!(i.data()[1].get("config").is_none());
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn redundant_group_patch_keeps_existing_include_provenance() {
    let d = Dir::new();
    d.put("child.json", r#"[{"name":"worker","config":1}]"#);
    d.put("a.json", r#"{"entries":[{"id":"g","group":true,"config":[{"name":"include","config":{"path":"child.json"}}]}],"patches":[{"id":"g","group":true}]}"#);
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
        let data = i.data();
        let child = data[0]["config"][0]["config"][0]["id"].as_str().unwrap();
        assert_eq!(
            i.ownership(child).unwrap().entry.file,
            std::fs::canonicalize(d.path("child.json")).unwrap()
        );
        i.edit(child, JsonEdit::new().set("config", json!(3)))
            .unwrap()
            .await
            .unwrap();
        i.flush().unwrap().await.unwrap();
        assert_eq!(read(&d.path("child.json"))[0]["config"], 3);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn nested_pending_groups_keep_created_and_moved_descendants_after_retry() {
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
        for (id, parent) in [("g", None), ("inner", Some("g"))] {
            i.create(json!({"id":id,"group":true,"config":[]}), parent, 0)
                .unwrap()
                .await
                .unwrap();
        }
        i.move_entry("a", Some("inner"), 0).unwrap().await.unwrap();
        i.edit("a", JsonEdit::new().set("config", json!(6)))
            .unwrap()
            .await
            .unwrap();
        i.create(
            json!({"id":"new","name":"worker","config":7}),
            Some("inner"),
            1,
        )
        .unwrap()
        .await
        .unwrap();
        d.put("a.json", "broken");
        assert!(i.flush().is_err());
        assert_eq!(i.pending(), 4);
        d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
        assert!(i.flush().unwrap().await.unwrap().conflicts.is_empty());
        assert_eq!(
            i.data()[0]["config"][0]["config"],
            json!([{"id":"a","name":"worker","config":6},{"id":"new","name":"worker","config":7}])
        );
        assert_eq!(i.data(), read(&d.path("a.json")));
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn actual_file_parent_removal_wins_creation_and_move() {
    let d = Dir::new();
    d.put(
        "a.json",
        r#"[{"id":"a","name":"worker","config":1},{"id":"g","group":true,"config":[]}]"#,
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
        i.move_entry("a", Some("g"), 0).unwrap().await.unwrap();
        i.create(json!({"id":"new","name":"worker"}), Some("g"), 1)
            .unwrap()
            .await
            .unwrap();
        d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
        let report = i.flush().unwrap().await.unwrap();
        assert_eq!(report.conflicts.len(), 2);
        assert_eq!(i.data(), json!([{"id":"a","name":"worker","config":1}]));
        assert_eq!(i.pending(), 0);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn replacement_patch_includes_keep_cycle_and_duplicate_mount_checks() {
    let d = Dir::new();
    d.put("a.json", r#"[{"id":"g","group":true,"config":[]}]"#);
    d.put(
        "child.json",
        r#"[{"id":"back","name":"include","config":{"path":"a.json"}}]"#,
    );
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(&rt.context(), catalog(), IncludeOptions::new(d.path("a.json"))).unwrap();
        i.refresh().unwrap().await.unwrap();
        let before = i.data();
        let descriptor = json!({"id":"inc","name":"include","config":{"path":"child.json"}});
        d.put("a.json", &json!({"entries":before,"patches":[{"id":"g","config":[descriptor]}]}).to_string());
        assert!(i.refresh().err().unwrap().message.contains("cycle"));
        assert_eq!(i.data(), before);
        d.put("child.json", r#"[{"id":"child","name":"worker"}]"#);
        d.put("a.json", &json!({"entries":before,"patches":[{"id":"g","config":[descriptor,{"id":"second","name":"include","config":{"path":"child.json"}}]}]}).to_string());
        assert!(i.refresh().err().unwrap().message.contains("mounted more than once"));
        assert_eq!(i.data(), before);
        d.put("a.json", &json!({"entries":before,"patches":[{"id":"g","config":[descriptor]}]}).to_string());
        assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
        assert_eq!(i.data()[0]["config"][0]["config"][0]["id"], "child");
        i.dispose().unwrap().await.unwrap();
    }).await;
}
