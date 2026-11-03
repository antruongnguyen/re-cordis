use crate::{
    IncludeError, Location, Owner, OwnerKind, Ownership,
    source::{Model, field, index},
};
use re_cordis_core::PathSegment;
use serde_json::Value;
use std::{collections::BTreeSet, path::Path};
pub(crate) fn find<'a>(data: &'a [Value], id: &str) -> Option<&'a Value> {
    for v in data {
        if v.get("id").and_then(Value::as_str) == Some(id) {
            return Some(v);
        }
        if v.get("group") == Some(&Value::Bool(true)) {
            if let Some(c) = v.get("config").and_then(Value::as_array) {
                if let Some(v) = find(c, id) {
                    return Some(v);
                }
            }
        }
    }
    None
}
pub(crate) fn find_mut<'a>(data: &'a mut [Value], id: &str) -> Option<&'a mut Value> {
    for v in data {
        if v.get("id").and_then(Value::as_str) == Some(id) {
            return Some(v);
        }
        if v.get("group") == Some(&Value::Bool(true)) {
            if let Some(c) = v.get_mut("config").and_then(Value::as_array_mut) {
                if let Some(v) = find_mut(c, id) {
                    return Some(v);
                }
            }
        }
    }
    None
}
fn ids(data: &[Value], out: &mut Vec<String>) {
    for v in data {
        if let Some(id) = v.get("id").and_then(Value::as_str) {
            out.push(id.into());
        }
        if v.get("group") == Some(&Value::Bool(true)) {
            if let Some(c) = v.get("config").and_then(Value::as_array) {
                ids(c, out);
            }
        }
    }
}
impl Model {
    pub(crate) fn apply_patches(
        &mut self,
        data: &mut Vec<Value>,
        file: &Path,
        patches: &[Value],
        depth: usize,
    ) -> Result<(), IncludeError> {
        for (i, patch) in patches.iter().enumerate() {
            let path = index(vec![PathSegment::Key("patches".into())], i);
            let warn = |model: &mut Model, message: &str| {
                model.warnings.push(IncludeError::located(
                    message,
                    Location {
                        file: file.into(),
                        path: path.clone(),
                        span: model
                            .files
                            .get(file)
                            .and_then(|d| d.spans.get(&crate::source::path_key(&path)))
                            .cloned(),
                    },
                ))
            };
            let Some(object) = patch.as_object() else {
                warn(self, "patch must be an object");
                continue;
            };
            let id = patch.get("id").and_then(Value::as_str);
            let target = id.and_then(|id| find(data, id));
            if let Some(insert) = patch.get("insert") {
                let Some(a) = insert.as_array() else {
                    warn(self, "patch insert must be an array");
                    continue;
                };
                if id.is_some() && target.is_none() {
                    warn(self, "patch insert target missing");
                    continue;
                }
                if target.is_some_and(|v| v.get("group") != Some(&Value::Bool(true))) {
                    warn(self, "patch insert target must be a group");
                    continue;
                }
                let mut names = Vec::new();
                ids(a, &mut names);
                let mut seen = BTreeSet::new();
                if names
                    .iter()
                    .any(|id| self.owners.contains_key(id) || !seen.insert(id))
                {
                    warn(self, "patch insert ID collision");
                    continue;
                }
                let mut added = Vec::new();
                for (j, v) in a.iter().cloned().enumerate() {
                    let p = index(field(path.clone(), "insert"), j);
                    let v = self.expand(v, file, p.clone(), depth + 1)?;
                    self.embedded(&v, file, p);
                    added.push(v);
                }
                if let Some(id) = id {
                    let t = find_mut(data, id).unwrap().as_object_mut().unwrap();
                    let list = t.entry("config").or_insert_with(|| Value::Array(vec![]));
                    if !list.is_array() {
                        *list = Value::Array(vec![]);
                    }
                    list.as_array_mut().unwrap().extend(added);
                } else {
                    data.extend(added);
                }
                continue;
            }
            let Some(id) = id else {
                warn(self, "patch ID required");
                continue;
            };
            let Some(target) = target else {
                warn(self, "patch target missing");
                continue;
            };
            if patch
                .get("name")
                .is_some_and(|n| target.get("name") != Some(n))
            {
                warn(self, "patch name mismatch");
                continue;
            }
            let replacing = target.get("group") == Some(&Value::Bool(true))
                && (object.contains_key("config")
                    || object.get("group").is_some_and(|g| g != &Value::Bool(true)));
            if replacing {
                if let Some(old) = target.get("config").and_then(Value::as_array) {
                    let mut stale = Vec::new();
                    ids(old, &mut stale);
                    for id in stale {
                        self.owners.remove(&id);
                    }
                }
            }
            let target = find_mut(data, id).unwrap();
            let target_obj = target.as_object_mut().unwrap();
            for (k, v) in object {
                if k == "id" || k == "name" {
                    continue;
                }
                if v.is_null() && k != "config" {
                    target_obj.remove(k);
                } else {
                    target_obj.insert(k.clone(), v.clone());
                }
                if let Some(o) = self.owners.get_mut(id) {
                    o.keys.insert(
                        k.clone(),
                        Owner {
                            file: file.into(),
                            path: path.clone(),
                            kind: OwnerKind::Patch,
                        },
                    );
                }
            }
            if object.contains_key("config") && target.get("group") == Some(&Value::Bool(true)) {
                if let Some(a) = target.get("config").and_then(Value::as_array) {
                    let raw = a.clone();
                    let mut children = Vec::new();
                    for (j, v) in raw.into_iter().enumerate() {
                        let p = index(field(path.clone(), "config"), j);
                        let v = self.expand(v, file, p.clone(), depth + 1)?;
                        self.embedded(&v, file, p);
                        children.push(v);
                    }
                    target
                        .as_object_mut()
                        .unwrap()
                        .insert("config".into(), Value::Array(children));
                }
            }
        }
        Ok(())
    }
    fn embedded(&mut self, v: &Value, file: &Path, path: Vec<PathSegment>) {
        if let Some(id) = v.get("id").and_then(Value::as_str) {
            self.owners.insert(
                id.into(),
                Ownership {
                    entry: Owner {
                        file: file.into(),
                        path: path.clone(),
                        kind: OwnerKind::Embedded,
                    },
                    keys: std::collections::BTreeMap::new(),
                },
            );
        }
        let nested = self
            .files
            .get(file)
            .and_then(|d| d.value.pointer(&crate::source::path_key(&path)))
            .and_then(|v| v.get("name"))
            .and_then(Value::as_str)
            == Some("include");
        if !nested && v.get("group") == Some(&Value::Bool(true)) {
            if let Some(c) = v.get("config").and_then(Value::as_array) {
                for (i, v) in c.iter().enumerate() {
                    self.embedded(v, file, index(field(path.clone(), "config"), i));
                }
            }
        }
    }
}
