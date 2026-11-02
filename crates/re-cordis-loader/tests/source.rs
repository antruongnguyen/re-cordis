//! Bounded JSON, source reconciliation and commit acceptance.
use re_cordis_core::{Listener, Plugin, Runtime};
use re_cordis_loader::{
    Catalog, Config, EntryPatch, EntrySpec, FieldPatch, Loader, LoaderError, SourceDocument,
    SourceId,
};
use serde_json::json;
use std::{cell::RefCell, convert::Infallible, ops::ControlFlow, rc::Rc};
#[tokio::test(flavor = "current_thread")]
async fn json_preserves_absence_null_and_original_decode_errors_and_source_siblings() {
    let runtime = Runtime::new();
    runtime.run(async{
  let root=runtime.context();let observed=Rc::new(RefCell::new(Vec::new()));
  let mut catalog=Catalog::new();
  catalog.register_json("worker",Plugin::new("worker",{let observed=observed.clone();move|_,v:Option<serde_json::Value>|{observed.borrow_mut().push(v);async{Ok::<_,Infallible>(())}}}),|v|Ok::<_,Infallible>(v.cloned())).unwrap();
  let loader=Loader::new(&root,catalog).unwrap();let commits=Rc::new(RefCell::new(Vec::new()));
  root.on(&loader.commits(),Listener::sync({let commits=commits.clone();move|_,c|{commits.borrow_mut().push(c);Ok::<_,Infallible>(ControlFlow::Continue(()))}})).unwrap();
  let source=SourceId::new("memory");
  let doc=SourceDocument::json(source.clone(),r#"[{"id":"a","name":"worker"},{"id":"bad","name":"unknown"},{"id":"b","name":"worker","config":null}]"#).unwrap();
  let report=loader.reconcile(doc).unwrap().await.unwrap();assert_eq!(report.failures.len(),1);assert_eq!(loader.entries().len(),2);assert!(commits.borrow().is_empty());
  assert_eq!(*observed.borrow(),[None,Some(json!(null))]);
  let a=loader.resolve("a").unwrap();loader.patch(&a,EntryPatch{disabled:FieldPatch::Set(true),..EntryPatch::default()}).unwrap().await.unwrap();
  assert_eq!(commits.borrow().len(),1);assert_eq!(commits.borrow()[0].id,a);
  assert!(matches!(loader.decode_json(r#"[{"name":"worker","disabled":{"__jsExpr":"1"}}]"#),Err(LoaderError::Located{..})));
  loader.dispose().unwrap().await.unwrap();
 }).await.unwrap();
}
#[tokio::test(flavor = "current_thread")]
async fn runtime_json_group_preflights_all_children_and_replacement_catalog_keeps_identity() {
    let runtime = Runtime::new();
    runtime.run(async{
  let root=runtime.context();let applied=Rc::new(RefCell::new(Vec::new()));
  let first=Plugin::new("first",{let a=applied.clone();move|_,v:u32|{a.borrow_mut().push(("first",v));async{Ok::<_,Infallible>(())}}});
  let mut catalog=Catalog::new();catalog.register_json("worker",first,|v|serde_json::from_value::<u32>(v.cloned().unwrap_or(json!(0)))).unwrap();
  let loader=Loader::new(&root,catalog).unwrap();
  assert!(loader.create_json(r#"{"group":true,"config":[{"name":"worker","config":1},{"name":"worker","config":"bad"}]}"#).is_err());assert!(loader.entries().is_empty());
  let id=loader.create(EntrySpec::new("worker",Config::new(1_u32))).unwrap();loader.settle().unwrap().await.unwrap();
  let factory=loader.snapshot(&id).unwrap().factory_id;
  let new=Plugin::new("second",{let a=applied.clone();move|_,v:u32|{a.borrow_mut().push(("second",v));async{Ok::<_,Infallible>(())}}});
  loader.replace_factory("worker",new).unwrap().await.unwrap();
  assert_eq!(loader.snapshot(&id).unwrap().id,id);assert_ne!(loader.snapshot(&id).unwrap().factory_id,factory);
  assert_eq!(*applied.borrow(),[("first",1),("second",1)]);
  loader.dispose().unwrap().await.unwrap();assert!(root.registry().is_empty().unwrap());
 }).await.unwrap();
}
#[test]
fn json_document_limits_remain_global_and_entry_ids_are_deferred() {
    assert!(SourceDocument::json(SourceId::new("test"), &" ".repeat(1024 * 1024 + 1)).is_err());
    assert!(
        SourceDocument::json(SourceId::new("test"), r#"[{"id":"a:b","name":"worker"}]"#).is_ok()
    );
}
