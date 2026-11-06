//! Actual parser positions and unsupported syntax.
mod support;
use re_cordis_core::{PathSegment, Runtime};
use re_cordis_include::{Include, IncludeOptions, parse_document};
use support::*;
#[test]
fn format_capabilities_are_explicit() {
    let p = std::path::Path::new("a.yaml");
    let v = parse_document(p, "a: yes\nb: 012\nc: true\nd: ~\ne: 'false'\n").unwrap();
    assert_eq!(v.value["a"], "yes");
    assert_eq!(v.value["b"], 12);
    assert_eq!(v.value["c"], true);
    assert_eq!(v.value["d"], serde_json::Value::Null);
    assert_eq!(v.value["e"], "false");
    for text in [
        "a: 1\na: 2\n",
        "a: &x [1]\nb: *x\n",
        "a: !js 1+1\n",
        "a: 1\n---\na: 2\n",
    ] {
        assert!(parse_document(p, text).is_err(), "{text}");
    }
    assert!(parse_document(std::path::Path::new("a.json"), r#"{"a":1,"a":2}"#).is_err());
}
#[test]
fn precise_unicode_scalar_columns_and_node_bounds() {
    let p = std::path::Path::new("a.yaml");
    let v = parse_document(p, "[é, 😀, x]").unwrap();
    let m = v.location(&[PathSegment::Index(2)]).unwrap();
    assert_eq!((m.line, m.column), (1, 8));
    assert!(parse_document(p, &"a".repeat(1_048_577)).is_err());
    assert!(parse_document(p, &format!("{}0{}", "[".repeat(33), "]".repeat(33))).is_err());
}
#[tokio::test(flavor = "current_thread")]
async fn nested_failure_points_at_owning_file() {
    let d = Dir::new();
    d.put("child.yaml", "- id: bad\n  name: missing\n");
    d.put(
        "a.json",
        r#"[{"id":"inc","name":"include","config":{"path":"child.yaml"}}]"#,
    );
    let rt = Runtime::new();
    run(&rt, async {
        let i = Include::new(
            &rt.context(),
            catalog(),
            IncludeOptions::new(d.path("a.json")),
        )
        .unwrap();
        let r = i.refresh().unwrap().await.unwrap();
        let loc = r.failures[0].location.as_ref().unwrap();
        assert_eq!(
            loc.file,
            std::fs::canonicalize(d.path("child.yaml")).unwrap()
        );
        assert_eq!(loc.span.as_ref().unwrap().line, 2);
        i.dispose().unwrap().await.unwrap();
    })
    .await;
}

#[test]
fn json_astral_escapes_and_literals_are_accepted() {
    let p = std::path::Path::new("a.json");
    for text in [r#"["\ud83d\ude00",1]"#, r#"["😀",1]"#] {
        let v = parse_document(p, text).unwrap();
        assert_eq!(v.value[0], "😀");
        assert!(v.location(&[PathSegment::Index(1)]).is_some());
    }
}
