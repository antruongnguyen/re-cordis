//! Joined final flush, source replacement and wrapper reentry.
mod support;
use re_cordis_core::{Plugin, Runtime};
use re_cordis_include::{Include, IncludeOptions, JsonEdit, parse_document};
use re_cordis_loader::Catalog;
use serde_json::json;
use std::{cell::RefCell, convert::Infallible, rc::Rc};
use support::*;
#[test]
fn parse_errors_retain_original_parser_causes_and_actual_locations() {
    for (file, text) in [("a.json", "["), ("a.yaml", "- [\n")] {
        let e = parse_document(std::path::Path::new(file), text).unwrap_err();
        assert!(std::error::Error::source(&e).is_some());
        assert!(e.location.as_ref().unwrap().span.is_some());
    }
}
#[tokio::test(flavor = "current_thread")]
async fn final_io_failure_is_visible_and_disposal_is_idempotent() {
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
        let first = i.dispose().unwrap().await.unwrap_err();
        assert!(std::error::Error::source(&first).is_some());
        assert!(i.final_error().is_some());
        assert_eq!(i.pending(), 1);
        assert_eq!(
            i.dispose().unwrap().await.unwrap_err().message,
            first.message
        );
        assert!(
            i.edit("a", JsonEdit::new().set("config", json!(3)))
                .is_err()
        );
        assert!(
            i.entries()
                .iter()
                .all(|e| e.state == re_cordis_loader::EntryState::Disposed)
        );
        assert!(rt.context().registry().is_empty().unwrap());
        assert_eq!(std::fs::read_to_string(d.path("a.json")).unwrap(), "broken");
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn parent_cleanup_flushes_before_child_stop_and_dropped_waiter_still_joins() {
    let d = Dir::new();
    d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
    let (start_tx, start_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let start = Rc::new(RefCell::new(Some(start_tx)));
    let release = Rc::new(RefCell::new(Some(release_rx)));
    let p = d.path("a.json");
    let mut c = Catalog::new();
    c.register_json(
        "worker",
        Plugin::new("worker", move |ctx, value: Option<serde_json::Value>| {
            let start = start.clone();
            let release = release.clone();
            let p = p.clone();
            async move {
                if value == Some(json!(1)) {
                    ctx.on_dispose(move || {
                        assert_eq!(read(&p)[1]["id"], "pending");
                        start.borrow_mut().take().unwrap().send(()).unwrap();
                        let receiver = release.borrow_mut().take().unwrap();
                        async move {
                            receiver.await.unwrap();
                            Ok::<_, Infallible>(())
                        }
                    })
                    .unwrap();
                }
                Ok::<_, Infallible>(())
            }
        }),
        |v| Ok::<_, Infallible>(v.cloned()),
    )
    .unwrap();
    let rt = Runtime::new();
    run(&rt, async {
        let root = rt.context();
        let parent = root
            .plugin(
                &Plugin::new("parent", |_, ()| async { Ok::<_, Infallible>(()) }),
                (),
            )
            .unwrap();
        let i = Include::new(&parent.context(), c, IncludeOptions::new(d.path("a.json"))).unwrap();
        i.refresh().unwrap().await.unwrap();
        i.create(json!({"id":"pending","name":"worker","config":2}), None, 1)
            .unwrap()
            .await
            .unwrap();
        assert_eq!(i.pending(), 1);
        drop(parent.dispose().unwrap());
        start_rx.await.unwrap();
        assert_eq!(read(&d.path("a.json"))[1]["id"], "pending");
        assert!(
            i.create(json!({"id":"late","name":"worker"}), None, 2)
                .is_err()
        );
        release_tx.send(()).unwrap();
        parent.dispose().unwrap().await.unwrap();
        assert!(root.registry().is_empty().unwrap());
        assert!(i.final_error().is_none());
        assert_eq!(i.pending(), 0);
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn decoder_reentry_is_refused_without_overwriting_outer_source_state() {
    let d = Dir::new();
    d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
    let slot: Rc<RefCell<Option<Include>>> = Rc::new(RefCell::new(None));
    let count = Rc::new(RefCell::new(0));
    let mut c = Catalog::new();
    let s = slot.clone();
    let n = count.clone();
    c.register_json(
        "worker",
        Plugin::new("worker", |_, _: Option<serde_json::Value>| async {
            Ok::<_, Infallible>(())
        }),
        move |v| {
            if let Some(i) = s.borrow().as_ref() {
                let e = i.refresh().err().expect("reentry refused");
                assert!(e.message.contains("in progress"));
                *n.borrow_mut() += 1;
            }
            Ok::<_, Infallible>(v.cloned())
        },
    )
    .unwrap();
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(&rt.context(), c, IncludeOptions::new(d.path("a.json"))).unwrap();
        *slot.borrow_mut() = Some(i.clone());
        i.refresh().unwrap().await.unwrap();
        assert_eq!(i.data()[0]["config"], 1);
        assert!(*count.borrow() > 0);
        d.put("a.json", r#"[{"id":"a","name":"worker","config":2}]"#);
        i.refresh().unwrap().await.unwrap();
        assert_eq!(i.data()[0]["config"], 2);
        slot.borrow_mut().take();
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn source_factory_replacement_keeps_latest_intent_during_slow_cleanup() {
    let d = Dir::new();
    d.put("a.json", r#"[{"id":"a","name":"first","config":1}]"#);
    let (start_tx, start_rx) = tokio::sync::oneshot::channel();
    let (release_tx, release_rx) = tokio::sync::oneshot::channel();
    let start = Rc::new(RefCell::new(Some(start_tx)));
    let release = Rc::new(RefCell::new(Some(release_rx)));
    let seen = Rc::new(RefCell::new(vec![]));
    let mut c = Catalog::new();
    for name in ["first", "second", "third"] {
        let start = start.clone();
        let release = release.clone();
        let seen = seen.clone();
        c.register_json(
            name,
            Plugin::new(name, move |ctx, value: u32| {
                let start = start.clone();
                let release = release.clone();
                let seen = seen.clone();
                async move {
                    seen.borrow_mut().push((name, value));
                    if name == "first" {
                        ctx.on_dispose(move || {
                            start.borrow_mut().take().unwrap().send(()).unwrap();
                            let receiver = release.borrow_mut().take().unwrap();
                            async move {
                                receiver.await.unwrap();
                                Ok::<_, Infallible>(())
                            }
                        })
                        .unwrap();
                    }
                    Ok::<_, Infallible>(())
                }
            }),
            |v| serde_json::from_value::<u32>(v.cloned().unwrap_or(json!(0))),
        )
        .unwrap();
    }
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(&rt.context(), c, IncludeOptions::new(d.path("a.json"))).unwrap();
        i.refresh().unwrap().await.unwrap();
        d.put("a.json", r#"[{"id":"a","name":"second","config":2}]"#);
        drop(i.refresh().unwrap());
        start_rx.await.unwrap();
        d.put("a.json", r#"[{"id":"a","name":"third","config":3}]"#);
        let latest = i.refresh().unwrap();
        assert_eq!(i.data()[0]["name"], "third");
        release_tx.send(()).unwrap();
        latest.await.unwrap();
        assert_eq!(*seen.borrow(), [("first", 1), ("third", 3)]);
        assert_eq!(i.entries()[0].factory, "third");
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
#[tokio::test(flavor = "current_thread")]
async fn moving_nested_include_descriptors_rebases_paths_and_preserves_raw_structure() {
    let d = Dir::new();
    std::fs::create_dir(d.path("sub")).unwrap();
    d.put(
        "sub/grand.json",
        r#"[{"id":"grand","name":"worker","config":1}]"#,
    );
    d.put("sub/child.json",r#"[{"id":"g","group":true,"config":[{"id":"inner","name":"include","config":{"path":"grand.json"}}]}]"#);
    d.put(
        "root.json",
        r#"[{"id":"outer","name":"include","config":{"path":"sub/child.json"}}]"#,
    );
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("root.json")),
        )
        .unwrap();
        i.refresh().unwrap().await.unwrap();
        i.move_entry("g", None, 1).unwrap().await.unwrap();
        assert!(i.ownership("g").is_some());
        i.flush().unwrap().await.unwrap();
        let value = read(&d.path("root.json"));
        assert_eq!(value[1]["config"][0]["name"], "include");
        assert_eq!(
            value[1]["config"][0]["config"]["path"],
            std::fs::canonicalize(d.path("sub/grand.json"))
                .unwrap()
                .to_str()
                .unwrap()
        );
        assert_eq!(i.entries().len(), 4);
        i.refresh().unwrap().await.unwrap();
        assert_eq!(i.entries().len(), 4);
        i.move_entry("inner", None, 2).unwrap().await.unwrap();
        i.flush().unwrap().await.unwrap();
        let value = read(&d.path("root.json"));
        assert_eq!(value[2]["name"], "include");
        assert_eq!(
            value[2]["config"]["path"],
            std::fs::canonicalize(d.path("sub/grand.json"))
                .unwrap()
                .to_str()
                .unwrap()
        );
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}
