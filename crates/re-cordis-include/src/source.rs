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
    load_missing: bool,
    mounted: BTreeSet<PathBuf>,
    entry_paths: BTreeMap<(PathBuf, String), Vec<PathSegment>>,
    known_ids: BTreeMap<(PathBuf, String), String>,
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
fn load_document(path: &Path, initial: Option<&Value>) -> Result<Document, IncludeError> {
    let (bytes, text) = match read(path) {
        Ok(b) => {
            let t = String::from_utf8(b.clone()).map_err(|e| IncludeError::new(e.to_string()))?;
            (Some(b), t)
        }
        Err(e)
            if e.cause
                .as_deref()
                .and_then(|e| e.downcast_ref::<std::io::Error>())
                .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound)
                && initial.is_some() =>
        {
            (
                None,
                serde_json::to_string(initial.unwrap())
                    .map_err(|e| IncludeError::new(e.to_string()))?,
            )
        }
        Err(e) => return Err(e),
    };
    let parsed = parse_document(path, &text)?;
    entries(&parsed.value)?;
    Ok(Document {
        value: parsed.value,
        bytes,
        spans: parsed.locations,
    })
}
pub(crate) fn reserve(value: &Value, used: &mut BTreeSet<String>) {
    if let Some(id) = value.get("id").and_then(Value::as_str) {
        used.insert(id.into());
    }
    if let Some(a) = value.as_array() {
        for v in a {
            reserve(v, used);
        }
    } else if value.get("group") == Some(&Value::Bool(true)) {
        if let Some(c) = value.get("config") {
            reserve(c, used);
        }
    }
}
pub(crate) fn assign(
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
    if value.get("group") == Some(&Value::Bool(true)) {
        if let Some(c) = value.get_mut("config") {
            assign(c, field(path, "config"), file, ids, used, next);
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
        files.insert(
            root.clone(),
            load_document(&root, options.initial.as_ref())?,
        );
        // Expand once without generated IDs to identify actual entry addresses.
        // This pass also loads only sources encountered in applied structural data.
        let mut model = Self::build(root.clone(), files, true, BTreeMap::new())?;
        let explicit = model.explicit_ids();
        let known_ids: BTreeMap<_, _> = ids
            .iter()
            .filter(|(_, id)| !explicit.contains(*id))
            .map(|(path, id)| (path.clone(), id.clone()))
            .collect();
        if !known_ids.is_empty() {
            // Exposed positional IDs can make a new structural patch applicable.
            // Restore them only in effective entries; raw values stay unchanged.
            model = Self::build(root.clone(), model.files, true, known_ids)?;
        }
        let mut used = model.explicit_ids();
        let mut files = model.files;
        for ((file, pointer), path) in model.entry_paths {
            let value = files
                .get_mut(&file)
                .unwrap()
                .value
                .pointer_mut(&pointer)
                .unwrap();
            assign(value, path, &file, ids, &mut used, next);
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
        Self::build(root, files, false, BTreeMap::new())
    }
    fn explicit_ids(&self) -> BTreeSet<String> {
        self.entry_paths
            .keys()
            .filter_map(|(file, path)| {
                self.files[file]
                    .value
                    .pointer(path)?
                    .get("id")?
                    .as_str()
                    .map(str::to_owned)
            })
            .collect()
    }
    fn build(
        root: PathBuf,
        files: BTreeMap<PathBuf, Document>,
        load_missing: bool,
        known_ids: BTreeMap<(PathBuf, String), String>,
    ) -> Result<Self, IncludeError> {
        let mut model = Self {
            root: root.clone(),
            files,
            data: vec![],
            owners: BTreeMap::new(),
            locations: BTreeMap::new(),
            warnings: vec![],
            load_missing,
            mounted: BTreeSet::new(),
            entry_paths: BTreeMap::new(),
            known_ids,
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
        if self.load_missing && !self.mounted.insert(file.into()) {
            return Err(IncludeError::new(format!(
                "include cycle or source mounted more than once: {}",
                file.display()
            )));
        }
        if !self.files.contains_key(file) && self.load_missing {
            if self.files.len() >= MAX_FILES {
                return Err(IncludeError::new("source file limit exceeded"));
            }
            self.files.insert(file.into(), load_document(file, None)?);
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
        let address = (file.into(), path_key(&path));
        self.entry_paths.insert(address.clone(), path.clone());
        if v.get("id").is_none()
            && (v.get("name").is_some() || v.get("group") == Some(&Value::Bool(true)))
        {
            if let Some(id) = self.known_ids.get(&address) {
                v.as_object_mut()
                    .unwrap()
                    .insert("id".into(), Value::String(id.clone()));
            }
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
            effective: Vec<PathSegment>,
            file: PathBuf,
            rawbase: Vec<PathSegment>,
            owners: &BTreeMap<String, Ownership>,
            files: &BTreeMap<PathBuf, Document>,
            out: &mut BTreeMap<String, Location>,
        ) {
            for (i, v) in a.iter().enumerate() {
                let ep = index(effective.clone(), i);
                let fallback = index(rawbase.clone(), i);
                let own = v
                    .get("id")
                    .and_then(Value::as_str)
                    .and_then(|id| owners.get(id));
                let (entry_file, entry_path) = own
                    .map(|o| (o.entry.file.clone(), o.entry.path.clone()))
                    .unwrap_or((file.clone(), fallback));
                let span = files
                    .get(&entry_file)
                    .and_then(|d| d.spans.get(&path_key(&entry_path)))
                    .cloned();
                out.insert(
                    path_key(&ep),
                    Location {
                        file: entry_file.clone(),
                        path: entry_path.clone(),
                        span,
                    },
                );
                if let (Some(object), Some(own)) = (v.as_object(), own) {
                    for k in object.keys() {
                        let owner = own.keys.get(k).unwrap_or(&own.entry);
                        let raw = field(owner.path.clone(), k);
                        out.insert(
                            path_key(&field(ep.clone(), k)),
                            Location {
                                file: owner.file.clone(),
                                path: raw.clone(),
                                span: files
                                    .get(&owner.file)
                                    .and_then(|d| d.spans.get(&path_key(&raw)))
                                    .cloned(),
                            },
                        );
                    }
                }
                if v.get("group") == Some(&Value::Bool(true)) {
                    if let Some(children) = v.get("config").and_then(Value::as_array) {
                        let mut child_file = entry_file.clone();
                        let mut child_path = field(entry_path.clone(), "config");
                        if let Some(own) = own {
                            if let Some(o) = own.keys.get("config") {
                                child_file = o.file.clone();
                                child_path = field(o.path.clone(), "config");
                            } else if let Some(raw) = files
                                .get(&entry_file)
                                .and_then(|d| d.value.pointer(&path_key(&entry_path)))
                            {
                                if include_path(raw).is_some() {
                                    if let Some(target) = include_path(raw).and_then(|p| {
                                        canonical(&entry_file.parent().unwrap().join(p)).ok()
                                    }) {
                                        if let Some(d) = files.get(&target) {
                                            child_file = target;
                                            child_path = entries_path(&d.value);
                                        }
                                    }
                                }
                            }
                        }
                        walk(
                            children,
                            field(ep, "config"),
                            child_file,
                            child_path,
                            owners,
                            files,
                            out,
                        );
                    }
                }
            }
        }
        let base = entries_path(&self.files[&self.root].value);
        walk(
            &self.data,
            vec![],
            self.root.clone(),
            base,
            &self.owners,
            &self.files,
            &mut self.locations,
        );
    }
}
