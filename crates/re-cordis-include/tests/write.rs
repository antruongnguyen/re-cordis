//! Persistence safety and bounded admission.
mod support;
use re_cordis_core::Runtime;
use re_cordis_include::{Include, IncludeOptions, JsonEdit};
use serde_json::json;
use support::*;
#[tokio::test(flavor = "current_thread")]
async fn invalid_latest_is_never_overwritten_pending_survives_and_refresh_retries() {
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
        i.edit("a", JsonEdit::new().set("config", json!(2)))
            .unwrap()
            .await
            .unwrap();
        d.put("a.json", "broken");
        assert!(i.flush().is_err());
        assert_eq!(i.pending(), 1);
        assert_eq!(std::fs::read_to_string(d.path("a.json")).unwrap(), "broken");
        d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
        i.refresh().unwrap().await.unwrap();
        assert_eq!(read(&d.path("a.json"))[0]["config"], 2);
        assert_eq!(i.pending(), 0);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn initial_file_appearance_wins_without_overwrite() {
    let d = Dir::new();
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json"))
                .initial(json!([{"id":"initial","name":"worker"}])),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        d.put("a.json", r#"[{"id":"external","name":"worker"}]"#);
        i.flush().unwrap().await.unwrap();
        assert_eq!(read(&d.path("a.json"))[0]["id"], "external");
        assert_eq!(i.data()[0]["id"], "external");
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn json_admission_rejects_before_journal_or_running_state_changes() {
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
        let before = i.data();
        assert!(
            i.edit("a", JsonEdit::new().set("id", json!("other")))
                .is_err()
        );
        assert!(
            i.edit("a", JsonEdit::new().set("config", json!("x".repeat(65537))))
                .is_err()
        );
        assert!(
            i.create(json!({"id":"bad","name":"unknown"}), None, 0)
                .is_err()
        );
        assert_eq!(i.pending(), 0);
        assert_eq!(i.data(), before);
        i.dispose().unwrap().await.unwrap();
        assert!(
            i.edit("a", JsonEdit::new().set("config", json!(2)))
                .is_err()
        );
    })
    .await;
}
#[cfg(unix)]
#[tokio::test(flavor = "current_thread")]
async fn replacement_preserves_private_permissions_and_symlink_target() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let d = Dir::new();
    d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
    std::fs::set_permissions(d.path("a.json"), std::fs::Permissions::from_mode(0o600)).unwrap();
    symlink(d.path("a.json"), d.path("link.json")).unwrap();
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("link.json")),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        i.edit("a", JsonEdit::new().set("config", json!(2)))
            .unwrap()
            .await
            .unwrap();
        i.flush().unwrap().await.unwrap();
        assert_eq!(
            std::fs::metadata(d.path("a.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
        assert!(
            std::fs::symlink_metadata(d.path("link.json"))
                .unwrap()
                .file_type()
                .is_symlink()
        );
        assert_eq!(read(&d.path("a.json"))[0]["config"], 2);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn loaded_plugin_validation_rejection_never_enters_persistence_journal() {
    use re_cordis_core::Plugin;
    use re_cordis_loader::Catalog;
    use std::convert::Infallible;
    let d = Dir::new();
    d.put("a.json", r#"[{"id":"a","name":"checked","config":1}]"#);
    let mut c = Catalog::new();
    c.register_json(
        "checked",
        Plugin::validated(
            "checked",
            |v: &u32| {
                if *v < 3 {
                    Ok(*v)
                } else {
                    Err(std::io::Error::other("rejected"))
                }
            },
            |_, _: u32| async { Ok::<_, Infallible>(()) },
        ),
        |v| serde_json::from_value::<u32>(v.cloned().unwrap_or(json!(0))),
    )
    .unwrap();
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(&rt.context(), c, IncludeOptions::new(d.path("a.json"))).unwrap();
        i.refresh().unwrap().await.unwrap();
        let before = i.data();
        let e = i
            .edit("a", JsonEdit::new().set("config", json!(3)))
            .err()
            .expect("loaded validator rejects before journal");
        assert!(e.location.is_none());
        assert!(std::error::Error::source(&e).is_some());
        assert_eq!(i.pending(), 0);
        assert_eq!(i.data(), before);
        i.flush().unwrap().await.unwrap();
        assert_eq!(read(&d.path("a.json"))[0]["config"], 1);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
