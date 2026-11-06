//! Independent source, format, and admission boundary acceptance.
mod support;
use re_cordis_core::{PathSegment, Plugin, Runtime};
use re_cordis_include::{
    Include, IncludeOptions, MAX_BYTES, MAX_DEPTH, MAX_ENTRIES, MAX_FILES, MAX_NODES, MAX_PATCHES,
    MAX_STRING_BYTES, parse_document,
};
use re_cordis_loader::Catalog;
use serde_json::{Value, json};
use std::{cell::RefCell, convert::Infallible, path::Path, rc::Rc};
use support::*;

#[test]
fn exact_parser_bounds_accept_the_limit_and_reject_the_next_unit() {
    let p = Path::new("bounds.json");
    let bytes = format!("[]{}", " ".repeat(MAX_BYTES - 2));
    assert!(parse_document(p, &bytes).is_ok());
    assert!(parse_document(p, &(bytes + " ")).is_err());
    for depth in [MAX_DEPTH, MAX_DEPTH + 1] {
        let text = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
        assert_eq!(parse_document(p, &text).is_ok(), depth == MAX_DEPTH);
    }
    // UTF-8 bytes, rather than character count, bound both keys and strings.
    let exact = "é".repeat(MAX_STRING_BYTES / 2);
    for text in [
        serde_json::to_string(&exact).unwrap(),
        serde_json::to_string(&json!({exact.clone(): 0})).unwrap(),
    ] {
        assert!(parse_document(p, &text).is_ok());
    }
    for text in [
        serde_json::to_string(&(exact.clone() + "a")).unwrap(),
        serde_json::to_string(&json!({(exact + "a"): 0})).unwrap(),
    ] {
        assert!(parse_document(p, &text).is_err());
    }
    // YAML marked events avoid a quadratic JSON-position scan for this large fixture.
    for nodes in [MAX_NODES, MAX_NODES + 1] {
        let text = format!("[{}]", vec!["0"; nodes - 1].join(","));
        assert_eq!(
            parse_document(Path::new("nodes.yaml"), &text).is_ok(),
            nodes == MAX_NODES
        );
    }
}

#[test]
fn restricted_yaml_and_real_unicode_locations_preserve_canonical_values() {
    let p = Path::new("values.yaml");
    let text = "a: yes\nb: 0x10\nc: 0o10\nd: -12\ne: 1.5\nf: 1e2\ng: null\nh: FALSE\ni: 'true'\nj: [é, 😀, x]\n";
    let parsed = parse_document(p, text).unwrap();
    assert_eq!(
        parsed.value,
        json!({"a":"yes","b":16,"c":8,"d":-12,"e":1.5,"f":100.0,"g":null,
                "h":false,"i":"true","j":["é","😀","x"]})
    );
    let span = parsed
        .location(&[PathSegment::Key("j".into()), PathSegment::Index(2)])
        .unwrap();
    assert_eq!((span.line, span.column), (10, 11));
    let canonical = serde_json::to_string_pretty(&parsed.value).unwrap();
    assert_eq!(parse_document(p, &canonical).unwrap().value, parsed.value);
    let json_text = "[\"é\",\"😀\",0]";
    let parsed = parse_document(Path::new("unicode.json"), json_text).unwrap();
    let span = parsed.location(&[PathSegment::Index(2)]).unwrap();
    assert_eq!((span.line, span.column), (1, 10));
    for text in [
        "a: !!str x\n",
        "a: !js 1+1\n",
        "a: &x [1]\nb: *x\n",
        "a: 1\na: 2\n",
        "a: .inf\n",
        "a: -.inf\n",
        "a: +.inf\n",
        "a: .Inf\n",
        "a: -.Inf\n",
        "a: +.Inf\n",
        "a: .INF\n",
        "a: -.INF\n",
        "a: +.INF\n",
        "a: .nan\n",
        "a: .NaN\n",
        "a: .NAN\n",
        "---\na: 1\n---\na: 2\n",
    ] {
        let error = parse_document(p, text).unwrap_err();
        assert!(
            error
                .location
                .as_ref()
                .and_then(|l| l.span.as_ref())
                .is_some(),
            "{text}"
        );
    }
    assert!(parse_document(Path::new("unknown.txt"), "[]").is_err());
    assert!(parse_document(Path::new("malformed.json"), "[{]").is_err());
}

#[tokio::test(flavor = "current_thread")]
async fn read_failure_retains_state_but_parsed_invalid_entries_remove_only_their_ids() {
    let d = Dir::new();
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("root.json")),
        )
        .unwrap();
        assert!(i.refresh().is_err());
        assert!(i.entries().is_empty());
        assert!(!d.path("root.json").exists());
        d.put(
            "root.json",
            r#"[{"id":"bad","name":"worker"},{"id":"good","name":"worker"}]"#,
        );
        i.refresh().unwrap().await.unwrap();
        let good = i
            .entries()
            .iter()
            .find(|e| e.id.as_str() == "good")
            .unwrap()
            .fiber_id
            .clone();
        for text in [
            "null",
            "{}",
            r#"{"entries":false}"#,
            r#"{"entries":[],"unknown":0}"#,
        ] {
            d.put("root.json", text);
            assert!(i.refresh().is_err(), "{text}");
            assert_eq!(i.entries().len(), 2);
        }
        d.put(
            "root.json",
            r#"[null,7,{"id":"bad","name":"unknown"},{"id":"good","name":"worker"}]"#,
        );
        let report = i.refresh().unwrap().await.unwrap();
        assert_eq!(report.failures.len(), 3);
        assert_eq!(i.entries().len(), 1);
        assert_eq!(i.entries()[0].id.as_str(), "good");
        assert_eq!(i.entries()[0].fiber_id, good);
        for failure in &report.failures {
            assert_eq!(
                failure
                    .location
                    .as_ref()
                    .unwrap_or_else(|| panic!("missing source location: {failure:?}"))
                    .file,
                std::fs::canonicalize(d.path("root.json")).unwrap()
            );
        }
        d.put(
            "root.json",
            r#"[{"id":"bad","name":"worker"},{"id":"good","name":"worker"}]"#,
        );
        assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
        assert_eq!(i.entries().len(), 2);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn changing_a_file_factory_name_retires_the_old_instance() {
    let d = Dir::new();
    d.put("root.json", r#"[{"id":"entry","name":"first"}]"#);
    let rt = Runtime::new();
    run(&rt, async {
        let applied = Rc::new(RefCell::new(Vec::new()));
        let cleaned = Rc::new(RefCell::new(Vec::new()));
        let mut c = Catalog::new();
        for name in ["first", "second"] {
            let applied = applied.clone();
            let cleaned = cleaned.clone();
            c.register_json(
                name,
                Plugin::new(name, move |ctx, _: ()| {
                    applied.borrow_mut().push(name);
                    let cleaned = cleaned.clone();
                    async move {
                        ctx.on_dispose_sync(move || {
                            cleaned.borrow_mut().push(name);
                            Ok::<_, Infallible>(())
                        })?;
                        Ok::<_, re_cordis_core::BoxError>(())
                    }
                }),
                |_| Ok::<_, Infallible>(()),
            )
            .unwrap();
        }
        let i = Include::new(&rt.context(), c, IncludeOptions::new(d.path("root.json"))).unwrap();
        i.refresh().unwrap().await.unwrap();
        let old = i.entries()[0].fiber_id.clone();
        d.put("root.json", r#"[{"id":"entry","name":"second"}]"#);
        let report = i.refresh().unwrap().await.unwrap();
        assert!(report.failures.is_empty());
        assert_eq!(i.entries()[0].id.as_str(), "entry");
        assert_ne!(i.entries()[0].fiber_id, old);
        assert_eq!(&*applied.borrow(), &["first", "second"]);
        assert_eq!(&*cleaned.borrow(), &["first"]);
        i.dispose().unwrap().await.unwrap();
        assert_eq!(&*cleaned.borrow(), &["first", "second"]);
    })
    .await;
}

#[cfg(unix)]
#[tokio::test(flavor = "current_thread")]
async fn native_absolute_and_symlink_paths_use_the_actual_target_directory() {
    let d = Dir::new();
    std::fs::create_dir(d.path("native é\\dir")).unwrap();
    let child = d.path("native é\\dir/child 😀\\file.yaml");
    std::fs::write(&child, "- id: child\n  name: worker\n").unwrap();
    let target = d.path("native é\\dir/root.json");
    std::fs::write(
        &target,
        serde_json::to_vec(&json!([
            {"id":"include","name":"include","config":{"path":"child 😀\\file.yaml"}}
        ]))
        .unwrap(),
    )
    .unwrap();
    std::os::unix::fs::symlink(&target, d.path("root-link.json")).unwrap();
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("root-link.json")),
        )
        .unwrap();
        assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
        assert_eq!(
            i.ownership("child").unwrap().entry.file,
            std::fs::canonicalize(&child).unwrap()
        );
        assert_eq!(
            i.ownership("include").unwrap().entry.file,
            std::fs::canonicalize(&target).unwrap()
        );
        std::fs::write(
            &target,
            serde_json::to_vec(&json!([
                {"id":"include","name":"include","config":{"path":child}}
            ]))
            .unwrap(),
        )
        .unwrap();
        assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
        assert_eq!(i.entries().len(), 2);
        i.dispose().unwrap().await.unwrap();
        assert!(
            std::fs::symlink_metadata(d.path("root-link.json"))
                .unwrap()
                .file_type()
                .is_symlink()
        );
    })
    .await;
}

#[tokio::test(flavor = "current_thread")]
async fn exact_entry_patch_and_file_graph_limits_reject_without_replacing_state() {
    let d = Dir::new();
    let rt = Runtime::new();
    run(&rt, async {
            let i = Include::new(&rt.context(), catalog(), IncludeOptions::new(d.path("root.json"))).unwrap();
            let entries = |count: usize| Value::Array((0..count).map(|n| json!({"id":format!("e{n}"),"name":"worker","disabled":true})).collect());
            d.put("root.json", &entries(MAX_ENTRIES).to_string());
            assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
            assert_eq!(i.entries().len(), MAX_ENTRIES);
            d.put("root.json", &entries(MAX_ENTRIES + 1).to_string());
            assert!(i.refresh().is_err());
            assert_eq!(i.entries().len(), MAX_ENTRIES);
            let patches = |count: usize| json!({"entries":[],"patches":vec![json!({"id":"missing","disabled":true});count]});
            d.put("root.json", &patches(MAX_PATCHES).to_string());
            assert_eq!(i.refresh().unwrap().await.unwrap().failures.len(), MAX_PATCHES);
            assert!(i.entries().is_empty());
            d.put("root.json", &patches(MAX_PATCHES + 1).to_string());
            assert!(i.refresh().is_err());
            assert!(i.entries().is_empty());
            for n in 0..MAX_FILES {
                d.put(&format!("child{n}.json"), "[]");
            }
            let graph = |children: usize| Value::Array((0..children).map(|n| json!({"id":format!("g{n}"),"name":"include","config":{"path":format!("child{n}.json")}})).collect());
            d.put("root.json", &graph(MAX_FILES - 1).to_string());
            assert!(i.refresh().unwrap().await.unwrap().failures.is_empty());
            assert_eq!(i.entries().len(), MAX_FILES - 1);
            d.put("root.json", &graph(MAX_FILES).to_string());
            assert!(i.refresh().is_err());
            assert_eq!(i.entries().len(), MAX_FILES - 1);
            d.put("root.json", &graph(MAX_FILES - 1).to_string());
            i.dispose().unwrap().await.unwrap();
        }).await;
}
