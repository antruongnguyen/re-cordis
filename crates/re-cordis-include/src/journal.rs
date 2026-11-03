use crate::{
    Conflict, IncludeError, MAX_BYTES, MAX_ENTRIES,
    patch::{find, find_mut},
    source::{Model, entries_path, path_key},
};
use serde_json::{Map, Value, json};
use std::{collections::BTreeMap, path::PathBuf};
#[derive(Clone, Debug)]
pub(crate) struct Record {
    pub id: String,
    pub created: bool,
    pub remove: bool,
    pub parent: Option<Option<String>>,
    pub position: usize,
    pub snapshot: Option<Value>,
    pub original_parent: Option<String>,
    pub changes: BTreeMap<String, Option<Value>>,
}
#[derive(Clone)]
struct Flat {
    value: Value,
    parent: Option<String>,
    position: usize,
}
fn flatten(a: &[Value], parent: Option<String>, out: &mut BTreeMap<String, Flat>) {
    for (i, v) in a.iter().enumerate() {
        if let Some(id) = v.get("id").and_then(Value::as_str) {
            out.insert(
                id.into(),
                Flat {
                    value: v.clone(),
                    parent: parent.clone(),
                    position: i,
                },
            );
            if v.get("group") == Some(&Value::Bool(true)) {
                if let Some(c) = v.get("config").and_then(Value::as_array) {
                    flatten(c, Some(id.into()), out);
                }
            }
        }
    }
}
pub(crate) fn children_ids(a: &[Value], id: &str) -> Vec<String> {
    let mut out = BTreeMap::new();
    if let Some(v) = find(a, id) {
        flatten(std::slice::from_ref(v), None, &mut out);
    }
    out.into_keys().collect()
}
pub(crate) fn merge(records: &mut Vec<Record>, new: Record) -> Result<(), IncludeError> {
    if let Some(pos) = records.iter().position(|r| r.id == new.id) {
        let old = &mut records[pos];
        if new.remove {
            if old.created {
                records.remove(pos);
            } else {
                *old = new;
            }
        } else if old.remove {
            let mut new = new;
            new.created = false;
            *old = new;
        } else {
            old.changes.extend(new.changes);
            if new.snapshot.is_some() {
                old.snapshot = new.snapshot;
                old.original_parent = new.original_parent;
            }
            if new.parent.is_some() {
                old.parent = new.parent;
                old.position = new.position;
            }
        }
    } else {
        records.push(new);
    }
    if records.len() > MAX_ENTRIES {
        return Err(IncludeError::new("journal record limit exceeded"));
    }
    let values:Vec<_>=records.iter().map(|r|json!({"id":r.id,"created":r.created,"remove":r.remove,"parent":r.parent,"position":r.position,"snapshot":r.snapshot,"original_parent":r.original_parent,"changes":r.changes.iter().map(|(k,v)|(k.clone(),json!({"present":v.is_some(),"value":v}))).collect::<Map<String,Value>>()})).collect();
    if serde_json::to_vec(&values)
        .map_err(|e| IncludeError::new(e.to_string()))?
        .len()
        > MAX_BYTES
    {
        return Err(IncludeError::new("journal byte limit exceeded"));
    }
    Ok(())
}
pub(crate) fn reconcile(
    records: &mut Vec<Record>,
    base: &[Value],
    theirs: &[Value],
) -> Vec<Conflict> {
    let mut b = BTreeMap::new();
    flatten(base, None, &mut b);
    let mut t = BTreeMap::new();
    flatten(theirs, None, &mut t);
    let mut conflicts = Vec::new();
    records.retain_mut(|r| {
        let old = b.get(&r.id);
        let now = t.get(&r.id);
        let conflict = |out: &mut Vec<Conflict>, key: Option<String>, reason: &str| {
            out.push(Conflict {
                id: r.id.clone(),
                key,
                reason: reason.into(),
            })
        };
        if r.remove {
            match now {
                None => return false,
                Some(now) if old.is_none_or(|old| old.value != now.value) => {
                    conflict(
                        &mut conflicts,
                        None,
                        "file entry changed; runtime removal discarded",
                    );
                    return false;
                }
                _ => return true,
            }
        }
        let Some(now) = now else {
            if old.is_some() {
                conflict(
                    &mut conflicts,
                    None,
                    "file entry removed; runtime update discarded",
                );
                return false;
            }
            if r.parent
                .as_ref()
                .and_then(Option::as_ref)
                .is_some_and(|p| !t.contains_key(p))
            {
                conflict(
                    &mut conflicts,
                    None,
                    "file parent removed; runtime creation discarded",
                );
                return false;
            }
            return true;
        };
        r.created = false;
        r.changes.retain(|key, ours| {
            let bv = old.and_then(|old| old.value.get(key));
            let tv = now.value.get(key);
            if bv != tv && tv != ours.as_ref() {
                conflict(
                    &mut conflicts,
                    Some(key.clone()),
                    "file and runtime changed the same key; file wins",
                );
                false
            } else {
                true
            }
        });
        if let Some(parent) = &r.parent {
            if parent.as_ref().is_some_and(|p| !t.contains_key(p))
                || old.is_some_and(|old| old.parent != now.parent && now.parent != *parent)
            {
                conflict(
                    &mut conflicts,
                    None,
                    "file and runtime parent move conflict; file wins",
                );
                r.parent = None;
            }
        }
        let moved = r
            .parent
            .as_ref()
            .is_some_and(|p| *p != now.parent || r.position != now.position);
        !r.changes.is_empty() || moved || r.parent.is_some()
    });
    conflicts
}
pub(crate) fn detach(a: &mut Vec<Value>, id: &str) -> Option<Value> {
    if let Some(i) = a
        .iter()
        .position(|v| v.get("id").and_then(Value::as_str) == Some(id))
    {
        return Some(a.remove(i));
    }
    for v in a {
        if v.get("group") == Some(&Value::Bool(true)) {
            if let Some(c) = v.get_mut("config").and_then(Value::as_array_mut) {
                if let Some(v) = detach(c, id) {
                    return Some(v);
                }
            }
        }
    }
    None
}
pub(crate) fn place(
    a: &mut Vec<Value>,
    v: Value,
    parent: Option<&str>,
    position: usize,
) -> Result<(), IncludeError> {
    let list = if let Some(p) = parent {
        let group = find_mut(a, p).ok_or_else(|| IncludeError::new("target group missing"))?;
        if group.get("group") != Some(&Value::Bool(true)) {
            return Err(IncludeError::new("target must be a group"));
        }
        group
            .as_object_mut()
            .unwrap()
            .entry("config")
            .or_insert_with(|| Value::Array(vec![]))
            .as_array_mut()
            .ok_or_else(|| IncludeError::new("group config must be an array"))?
    } else {
        a
    };
    list.insert(position.min(list.len()), v);
    Ok(())
}
fn detach_raw(
    model: &mut Model,
    file: &PathBuf,
    path: &[re_cordis_core::PathSegment],
) -> Result<Value, IncludeError> {
    let Some((re_cordis_core::PathSegment::Index(i), parent)) = path.split_last() else {
        return Err(IncludeError::new("entry has no array address"));
    };
    let list = model
        .files
        .get_mut(file)
        .and_then(|d| d.value.pointer_mut(&path_key(parent)))
        .and_then(Value::as_array_mut)
        .ok_or_else(|| IncludeError::new("entry owner array missing"))?;
    if *i >= list.len() {
        return Err(IncludeError::new("entry owner index missing"));
    }
    Ok(list.remove(*i))
}
fn destination(model: &mut Model, parent: Option<&str>) -> Result<(PathBuf, String), IncludeError> {
    let Some(parent) = parent else {
        let root = model.root.clone();
        return Ok((
            root.clone(),
            path_key(&entries_path(&model.files[&root].value)),
        ));
    };
    let v =
        find(&model.data, parent).ok_or_else(|| IncludeError::new("destination parent missing"))?;
    if v.get("group") != Some(&Value::Bool(true)) {
        return Err(IncludeError::new("destination must be a group"));
    }
    let own = model
        .owners
        .get(parent)
        .ok_or_else(|| IncludeError::new("destination owner missing"))?
        .clone();
    if !own.keys.contains_key("config") {
        let raw = model.files[&own.entry.file]
            .value
            .pointer(&path_key(&own.entry.path))
            .ok_or_else(|| IncludeError::new("destination raw entry missing"))?;
        if raw.get("name").and_then(Value::as_str) == Some("include") {
            let p = raw
                .get("config")
                .and_then(|v| v.get("path"))
                .and_then(Value::as_str)
                .ok_or_else(|| IncludeError::new("include path missing"))?;
            let file = crate::source::canonical(&own.entry.file.parent().unwrap().join(p))?;
            let d = model
                .files
                .get(&file)
                .ok_or_else(|| IncludeError::new("included destination missing"))?;
            return Ok((file, path_key(&entries_path(&d.value))));
        }
    }
    let owner = own.keys.get("config").unwrap_or(&own.entry);
    let entry = model
        .files
        .get_mut(&owner.file)
        .and_then(|d| d.value.pointer_mut(&path_key(&owner.path)))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| IncludeError::new("destination owner missing"))?;
    entry
        .entry("config")
        .or_insert_with(|| Value::Array(vec![]));
    let mut p = owner.path.clone();
    p.push(re_cordis_core::PathSegment::Key("config".into()));
    Ok((owner.file.clone(), path_key(&p)))
}

pub(crate) fn position(data: &[Value], id: &str) -> Option<(Option<String>, usize)> {
    let mut flat = BTreeMap::new();
    flatten(data, None, &mut flat);
    flat.remove(id).map(|v| (v.parent, v.position))
}
pub(crate) fn snapshot(model: &Model, id: &str, effective: &Value) -> Result<Value, IncludeError> {
    let mut result = effective.clone();
    if let Some(o) = model.owners.get(id) {
        if let Some(raw) = model.files[&o.entry.file]
            .value
            .pointer(&path_key(&o.entry.path))
        {
            if raw.get("name").and_then(Value::as_str) == Some("include") {
                result = raw.clone();
                for (k, v) in effective.as_object().into_iter().flat_map(|m| m.iter()) {
                    if k != "name" && k != "group" && k != "config" {
                        result.as_object_mut().unwrap().insert(k.clone(), v.clone());
                    }
                }
                if let Some(p) = raw
                    .get("config")
                    .and_then(|v| v.get("path"))
                    .and_then(Value::as_str)
                {
                    let target = crate::source::canonical(&o.entry.file.parent().unwrap().join(p))?;
                    result["config"]["path"] = Value::String(
                        target
                            .to_str()
                            .ok_or_else(|| {
                                IncludeError::new("JSON path cannot express non-UTF8 native path")
                            })?
                            .into(),
                    );
                }
                return Ok(result);
            }
        }
    }
    if result.get("group") == Some(&Value::Bool(true)) {
        if let Some(children) = result.get_mut("config").and_then(Value::as_array_mut) {
            for child in children {
                if let Some(id) = child.get("id").and_then(Value::as_str).map(str::to_owned) {
                    *child = snapshot(model, &id, child)?;
                }
            }
        }
    }
    Ok(result)
}
fn remove_definitions(model: &mut Model, id: &str) {
    fn groups(a: &[Value], out: &mut std::collections::BTreeSet<String>) {
        for v in a {
            if v.get("group") == Some(&Value::Bool(true)) {
                if let Some(id) = v.get("id").and_then(Value::as_str) {
                    out.insert(id.into());
                }
                if let Some(c) = v.get("config").and_then(Value::as_array) {
                    groups(c, out);
                }
            }
        }
    }
    fn entries(a: &mut Vec<Value>, id: &str) {
        a.retain(|v| v.get("id").and_then(Value::as_str) != Some(id));
        for v in a {
            if v.get("group") == Some(&Value::Bool(true)) {
                if let Some(c) = v.get_mut("config").and_then(Value::as_array_mut) {
                    entries(c, id);
                }
            }
        }
    }
    let mut group_ids = std::collections::BTreeSet::new();
    groups(&model.data, &mut group_ids);
    for d in model.files.values_mut() {
        if let Some(a) = d.value.as_array_mut() {
            entries(a, id);
            continue;
        }
        if let Some(a) = d.value.get_mut("entries").and_then(Value::as_array_mut) {
            entries(a, id);
        }
        if let Some(patches) = d.value.get_mut("patches").and_then(Value::as_array_mut) {
            for p in patches {
                if let Some(a) = p.get_mut("insert").and_then(Value::as_array_mut) {
                    entries(a, id);
                }
                if p.get("id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| group_ids.contains(id))
                {
                    if let Some(a) = p.get_mut("config").and_then(Value::as_array_mut) {
                        entries(a, id);
                    }
                }
            }
        }
    }
}
pub(crate) fn route(mut model: Model, records: &[Record]) -> Result<Model, IncludeError> {
    let mut remaining: Vec<_> = records.iter().collect();
    while !remaining.is_empty() {
        let mut selected = None;
        for (i, r) in remaining.iter().enumerate() {
            let exists = find(&model.data, &r.id).is_some();
            let parent = r
                .parent
                .as_ref()
                .map(Option::as_deref)
                .unwrap_or(r.original_parent.as_deref());
            if !r.remove
                && (!exists || r.parent.is_some())
                && parent.is_some_and(|p| find(&model.data, p).is_none())
                && remaining.iter().any(|v| Some(v.id.as_str()) == parent)
            {
                continue;
            }
            selected = Some(i);
            break;
        }
        let r = remaining.remove(
            selected.ok_or_else(|| IncludeError::new("pending placement dependency cycle"))?,
        );
        if r.remove {
            if let Some(o) = model.owners.get(&r.id).cloned() {
                detach_raw(&mut model, &o.entry.file, &o.entry.path)?;
                model = Model::from_files(model.root, model.files)?;
            }
            continue;
        }
        let exists = find(&model.data, &r.id).is_some();
        if exists {
            let mut keys: Vec<_> = r.changes.keys().collect();
            keys.sort_by_key(|k| match k.as_str() {
                "name" => 0,
                "group" => 1,
                _ => 2,
            });
            for k in keys {
                let own = model
                    .owners
                    .get(&r.id)
                    .cloned()
                    .ok_or_else(|| IncludeError::new("entry owner missing"))?;
                let owner = own.keys.get(k).unwrap_or(&own.entry);
                let raw = model
                    .files
                    .get_mut(&owner.file)
                    .and_then(|d| d.value.pointer_mut(&path_key(&owner.path)))
                    .and_then(Value::as_object_mut)
                    .ok_or_else(|| IncludeError::new("key owner missing"))?;
                if raw.get("name").and_then(Value::as_str) == Some("include")
                    && (k == "config" || k == "group" || k == "name")
                {
                    return Err(IncludeError::new(
                        "include descriptor paths change through source refresh",
                    ));
                }
                if let Some(v) = &r.changes[k] {
                    raw.insert(k.clone(), v.clone());
                } else {
                    raw.remove(k);
                }
                model = Model::from_files(model.root, model.files)?;
            }
            if r.parent.is_none() {
                continue;
            }
        }
        let mut carried = if exists {
            snapshot(&model, &r.id, find(&model.data, &r.id).unwrap())?
        } else {
            r.snapshot.clone().unwrap_or_else(|| json!({"id":r.id}))
        };
        if !exists {
            let object = carried
                .as_object_mut()
                .ok_or_else(|| IncludeError::new("pending entry snapshot must be an object"))?;
            for (k, v) in &r.changes {
                if let Some(v) = v {
                    object.insert(k.clone(), v.clone());
                } else {
                    object.remove(k);
                }
            }
        }
        remove_definitions(&mut model, &r.id);
        model = Model::from_files(model.root, model.files)?;
        let parent = r
            .parent
            .as_ref()
            .map(Option::as_deref)
            .unwrap_or(r.original_parent.as_deref());
        let (file, path) = destination(&mut model, parent)?;
        let list = model
            .files
            .get_mut(&file)
            .and_then(|d| d.value.pointer_mut(&path))
            .and_then(Value::as_array_mut)
            .ok_or_else(|| IncludeError::new("destination list missing"))?;
        list.insert(r.position.min(list.len()), carried);
        model = Model::from_files(model.root, model.files)?;
    }
    model.rebuild_locations();
    Ok(model)
}
