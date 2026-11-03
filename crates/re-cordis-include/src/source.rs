use crate::{
    IncludeError, IncludeOptions, Location, MAX_BYTES, MAX_DEPTH, MAX_ENTRIES, MAX_FILES,
    MAX_PATCHES, Owner, OwnerKind, Ownership, format::parse_document,
};
use re_cordis_core::PathSegment;
use re_cordis_loader::SourceSpan;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
#[derive(Clone)]
pub(crate) struct Document {
    pub value: Value,
    pub bytes: Option<Vec<u8>>,
    pub spans: BTreeMap<String, SourceSpan>,
}
#[derive(Clone)]
pub(crate) struct Model {
    pub root: PathBuf,
    pub files: BTreeMap<PathBuf, Document>,
    pub data: Vec<Value>,
    pub owners: BTreeMap<String, Ownership>,
    pub locations: BTreeMap<String, Location>,
    pub warnings: Vec<IncludeError>,
}
pub(crate) fn canonical(path: &Path) -> Result<PathBuf, IncludeError> {
    match std::fs::canonicalize(path) {
        Ok(p) => Ok(p),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let parent = path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let name = path
                .file_name()
                .ok_or_else(|| IncludeError::new("source filename required"))?;
            Ok(std::fs::canonicalize(parent)
                .map_err(IncludeError::from)?
                .join(name))
        }
        Err(e) => Err(e.into()),
    }
}
pub(crate) fn path_key(path: &[PathSegment]) -> String {
    path.iter()
        .map(|p| match p {
            PathSegment::Key(k) => format!("/{}", k.replace('~', "~0").replace('/', "~1")),
            PathSegment::Index(i) => format!("/{i}"),
        })
        .collect()
}
pub(crate) fn read(path: &Path) -> Result<Vec<u8>, IncludeError> {
    if !std::fs::metadata(path)?.is_file() {
        return Err(IncludeError::new("source must be a regular file"));
    }
    let mut bytes = Vec::new();
    File::open(path)?
        .take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_BYTES {
        return Err(IncludeError::new("file byte limit exceeded"));
    }
    Ok(bytes)
}
pub(crate) fn entries(value: &Value) -> Result<&Vec<Value>, IncludeError> {
    if let Some(a) = value.as_array() {
        return Ok(a);
    }
    let o = value.as_object().ok_or_else(|| {
        IncludeError::new("source must be an entry array or entries/patches object")
    })?;
    if o.keys().any(|k| k != "entries" && k != "patches") {
        return Err(IncludeError::new("unknown source document field"));
    }
    if let Some(p) = o.get("patches") {
        if !p.is_array() || p.as_array().unwrap().len() > MAX_PATCHES {
            return Err(IncludeError::new("patch array limit exceeded"));
        }
    }
    o.get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| IncludeError::new("entries must be an array"))
}
pub(crate) fn entries_path(value: &Value) -> Vec<PathSegment> {
    if value.is_array() {
        vec![]
    } else {
        vec![PathSegment::Key("entries".into())]
    }
}
pub(crate) fn field(mut p: Vec<PathSegment>, k: &str) -> Vec<PathSegment> {
    p.push(PathSegment::Key(k.into()));
    p
}
pub(crate) fn index(mut p: Vec<PathSegment>, i: usize) -> Vec<PathSegment> {
    p.push(PathSegment::Index(i));
    p
}
fn include_path(v: &Value) -> Option<&str> {
    (v.get("name")?.as_str()? == "include")
        .then(|| v.get("config")?.get("path")?.as_str())
        .flatten()
}
fn discover(value: &Value, base: &Path, out: &mut Vec<PathBuf>) {
    if let Some(p) = include_path(value) {
        out.push(base.join(p));
        return;
    }
    if let Some(a) = value.as_array() {
        for v in a {
            discover(v, base, out);
        }
    } else if let Some(o) = value.as_object() {
        for (k, v) in o {
            if k == "entries"
                || k == "patches"
                || k == "insert"
                || (k == "config" && value.get("group") == Some(&Value::Bool(true)))
            {
                discover(v, base, out);
            }
        }
    }
}
fn load(
    path: PathBuf,
    initial: Option<&Value>,
    files: &mut BTreeMap<PathBuf, Document>,
    stack: &mut BTreeSet<PathBuf>,
    depth: usize,
) -> Result<(), IncludeError> {
    if depth > MAX_DEPTH {
        return Err(IncludeError::new("include depth limit exceeded"));
    }
    let path = canonical(&path)?;
    if stack.contains(&path) {
        return Err(IncludeError::new(format!(
            "include cycle: {}",
            path.display()
        )));
    }
    if files.contains_key(&path) {
        return Err(IncludeError::new(format!(
            "source mounted more than once: {}",
            path.display()
        )));
    }
    if files.len() >= MAX_FILES {
        return Err(IncludeError::new("source file limit exceeded"));
    }
    let (bytes, text) = match read(&path) {
        Ok(b) => {
            let t = String::from_utf8(b.clone()).map_err(|e| IncludeError::new(e.to_string()))?;
            (Some(b), t)
        }
        Err(_e) if !path.exists() && initial.is_some() => (
            None,
            serde_json::to_string(initial.unwrap())
                .map_err(|e| IncludeError::new(e.to_string()))?,
        ),
        Err(e) => return Err(e),
    };
    let parsed = parse_document(&path, &text)?;
    entries(&parsed.value)?;
    let mut includes = Vec::new();
    discover(&parsed.value, path.parent().unwrap(), &mut includes);
    stack.insert(path.clone());
    files.insert(
        path.clone(),
        Document {
            value: parsed.value,
            bytes,
            spans: parsed.locations,
        },
    );
    for child in includes {
        load(child, None, files, stack, depth + 1)?;
    }
    stack.remove(&path);
    Ok(())
}
fn reserve(value: &Value, used: &mut BTreeSet<String>) {
    if let Some(id) = value.get("id").and_then(Value::as_str) {
        used.insert(id.into());
    }
    if let Some(a) = value.as_array() {
        for v in a {
            reserve(v, used);
        }
    } else if let Some(o) = value.as_object() {
        for (k, v) in o {
            if k == "entries" || k == "patches" || k == "insert" || k == "config" && v.is_array() {
                reserve(v, used);
            }
        }
    }
}
fn assign(
    value: &mut Value,
    path: Vec<PathSegment>,
    file: &Path,
    ids: &mut BTreeMap<(PathBuf, String), String>,
    used: &mut BTreeSet<String>,
    next: &mut u64,
) {
    if let Some(a) = value.as_array_mut() {
        for (i, v) in a.iter_mut().enumerate() {
            assign(v, index(path.clone(), i), file, ids, used, next);
        }
        return;
    }
    if !value.is_object() {
        return;
    }
    let is_patch =
        path.len() >= 2 && path.get(path.len() - 2) == Some(&PathSegment::Key("patches".into()));
    let is_entry = !is_patch
        && (value.get("name").is_some() || value.get("group") == Some(&Value::Bool(true)));
    if is_entry && value.get("id").is_none() {
        let key = (file.into(), path_key(&path));
        let old = ids.get(&key).filter(|id| !used.contains(*id)).cloned();
        let id = old.unwrap_or_else(|| {
            loop {
                *next += 1;
                let id = format!("include-anon-{next}");
                if !used.contains(&id) {
                    break id;
                }
            }
        });
        used.insert(id.clone());
        ids.insert(key, id.clone());
        value
            .as_object_mut()
            .unwrap()
            .insert("id".into(), Value::String(id));
    }
    let keys: Vec<_> = value.as_object().unwrap().keys().cloned().collect();
    for k in keys {
        if k == "entries"
            || k == "patches"
            || k == "insert"
            || k == "config" && value.get(&k).is_some_and(Value::is_array)
        {
            assign(
                value.get_mut(&k).unwrap(),
                field(path.clone(), &k),
                file,
                ids,
                used,
                next,
            );
        }
    }
}
impl Model {
    pub fn load(
        options: &IncludeOptions,
        ids: &mut BTreeMap<(PathBuf, String), String>,
        next: &mut u64,
    ) -> Result<Self, IncludeError> {
        let root = canonical(&options.path)?;
        let mut files = BTreeMap::new();
        load(
            root.clone(),
            options.initial.as_ref(),
            &mut files,
            &mut BTreeSet::new(),
            0,
        )?;
        let mut used = BTreeSet::new();
        for d in files.values() {
            reserve(&d.value, &mut used);
        }
        for (p, d) in &mut files {
            assign(&mut d.value, vec![], p, ids, &mut used, next);
        }
        ids.retain(|(file, path), id| {
            files
                .get(file)
                .and_then(|d| d.value.pointer(path))
                .and_then(|v| v.get("id"))
                .and_then(Value::as_str)
                == Some(id.as_str())
        });
        if ids.len() > MAX_ENTRIES {
            return Err(IncludeError::new("anonymous identity limit exceeded"));
        }
        Self::from_files(root, files)
    }
    pub fn from_files(
        root: PathBuf,
        files: BTreeMap<PathBuf, Document>,
    ) -> Result<Self, IncludeError> {
        let mut model = Self {
            root: root.clone(),
            files,
            data: vec![],
            owners: BTreeMap::new(),
            locations: BTreeMap::new(),
            warnings: vec![],
        };
        model.data = model.expand_file(&root, 0)?;
        if model.owners.len() > MAX_ENTRIES {
            return Err(IncludeError::new("effective entry limit exceeded"));
        }
        Ok(model)
    }
    fn expand_file(&mut self, file: &Path, depth: usize) -> Result<Vec<Value>, IncludeError> {
        if depth > MAX_DEPTH {
            return Err(IncludeError::new("group depth limit exceeded"));
        }
        let value = self
            .files
            .get(file)
            .ok_or_else(|| IncludeError::new("missing included source"))?
            .value
            .clone();
        let path = entries_path(&value);
        let raw = entries(&value)?.clone();
        let mut data = Vec::new();
        for (i, v) in raw.into_iter().enumerate() {
            data.push(self.expand(v, file, index(path.clone(), i), depth)?);
        }
        if let Some(patches) = value.get("patches").and_then(Value::as_array) {
            self.apply_patches(&mut data, file, patches, depth)?;
        }
        Ok(data)
    }
    pub(crate) fn expand(
        &mut self,
        mut v: Value,
        file: &Path,
        path: Vec<PathSegment>,
        depth: usize,
    ) -> Result<Value, IncludeError> {
        if depth > MAX_DEPTH {
            return Err(IncludeError::new("group depth limit exceeded"));
        }
        if let Some(id) = v.get("id").and_then(Value::as_str) {
            let owner = Owner {
                file: file.into(),
                path: path.clone(),
                kind: OwnerKind::File,
            };
            self.owners.entry(id.into()).or_insert(Ownership {
                entry: owner,
                keys: BTreeMap::new(),
            });
        }
        if let Some(p) = include_path(&v) {
            let target = canonical(&file.parent().unwrap().join(p))?;
            let children = self.expand_file(&target, depth + 1)?;
            v.as_object_mut().unwrap().remove("name");
            v.as_object_mut()
                .unwrap()
                .insert("group".into(), Value::Bool(true));
            v.as_object_mut()
                .unwrap()
                .insert("config".into(), Value::Array(children));
        } else if v.get("group") == Some(&Value::Bool(true)) {
            if let Some(a) = v.get("config").and_then(Value::as_array) {
                let mut children = Vec::new();
                for (i, c) in a.iter().cloned().enumerate() {
                    children.push(self.expand(
                        c,
                        file,
                        index(field(path.clone(), "config"), i),
                        depth + 1,
                    )?);
                }
                v.as_object_mut()
                    .unwrap()
                    .insert("config".into(), Value::Array(children));
            }
        }
        Ok(v)
    }
    pub fn same_bytes(&self, other: &Self) -> bool {
        self.files.len() == other.files.len()
            && self
                .files
                .iter()
                .all(|(p, d)| other.files.get(p).is_some_and(|o| d.bytes == o.bytes))
    }
    pub fn rebuild_locations(&mut self) {
        self.locations.clear();
        fn walk(
            a: &[Value],
            p: Vec<PathSegment>,
            owners: &BTreeMap<String, Ownership>,
            files: &BTreeMap<PathBuf, Document>,
            out: &mut BTreeMap<String, Location>,
        ) {
            for (i, v) in a.iter().enumerate() {
                let path = index(p.clone(), i);
                if let Some(o) = v
                    .get("id")
                    .and_then(Value::as_str)
                    .and_then(|id| owners.get(id))
                {
                    let d = files.get(&o.entry.file).unwrap();
                    out.insert(
                        path_key(&path),
                        Location {
                            file: o.entry.file.clone(),
                            path: o.entry.path.clone(),
                            span: d.spans.get(&path_key(&o.entry.path)).cloned(),
                        },
                    );
                    if let Some(m) = v.as_object() {
                        for k in m.keys() {
                            let own = o.keys.get(k).unwrap_or(&o.entry);
                            let raw = field(own.path.clone(), k);
                            out.insert(
                                path_key(&field(path.clone(), k)),
                                Location {
                                    file: own.file.clone(),
                                    path: raw.clone(),
                                    span: files
                                        .get(&own.file)
                                        .and_then(|d| d.spans.get(&path_key(&raw)))
                                        .cloned(),
                                },
                            );
                        }
                    }
                }
                if v.get("group") == Some(&Value::Bool(true)) {
                    if let Some(c) = v.get("config").and_then(Value::as_array) {
                        walk(c, field(path, "config"), owners, files, out);
                    }
                }
            }
        }
        walk(
            &self.data,
            vec![],
            &self.owners,
            &self.files,
            &mut self.locations,
        );
    }
}
