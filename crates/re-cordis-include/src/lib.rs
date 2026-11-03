//! Bounded JSON/YAML file sources over the typed re-cordis loader.
//!
//! File reads and writes are synchronous and bounded. Keep the original core
//! Runtime driven until the returned lifecycle barriers finish.
//!
//! Construction itself performs no file I/O; refresh is explicit. This also
//! permits joining an unused include owner without touching a source file.
//!
//! ```
//! use re_cordis_core::{BoxError, Runtime};
//! use re_cordis_include::{Include, IncludeOptions};
//! use re_cordis_loader::Catalog;
//!
//! fn main() -> Result<(), BoxError> {
//!     tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
//!         let runtime = Runtime::new();
//!         runtime.run(async {
//!             let include = Include::new(&runtime.context(), Catalog::new(),
//!                 IncludeOptions::new("unused.json"))?;
//!             include.dispose()?.await?;
//!             Ok::<_, BoxError>(())
//!         }).await??;
//!         Ok::<_, BoxError>(())
//!     })
//! }
//! ```
//!
//! Include handles stay on their owning Runtime host.
//!
//! ```compile_fail
//! use re_cordis_include::Include;
//! fn move_to_thread(include: Include) {
//!     std::thread::spawn(move || drop(include));
//! }
//! ```
mod format;
mod journal;
mod patch;
mod persist;
mod source;
pub use format::{ParsedDocument, parse_document};
use futures_util::future::{FutureExt, LocalBoxFuture, Shared};
use re_cordis_core::{Context, FiberHandle, Plugin};
use re_cordis_loader::{
    Catalog, EntrySnapshot, Loader, LoaderError, SourceDocument, SourceId, SourceSpan,
};
use serde_json::Value;
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    convert::Infallible,
    error::Error,
    fmt,
    future::Future,
    path::PathBuf,
    pin::Pin,
    rc::Rc,
    task::{Context as TaskContext, Poll},
};
/// Maximum source or journal bytes.
pub const MAX_BYTES: usize = 1_048_576;
/// Maximum value/include/group nesting.
pub const MAX_DEPTH: usize = 32;
/// Maximum parsed nodes.
pub const MAX_NODES: usize = 65_536;
/// Maximum string/key bytes.
pub const MAX_STRING_BYTES: usize = 65_536;
/// Maximum canonical source files.
pub const MAX_FILES: usize = 64;
/// Maximum effective entries and journal records.
pub const MAX_ENTRIES: usize = 1024;
/// Maximum patches in a file.
pub const MAX_PATCHES: usize = 1024;
/// Maximum replacement attempts.
pub const MAX_RETRIES: usize = 3;
/// Actual source path and position.
#[derive(Clone, Debug)]
pub struct Location {
    /// Canonical native source path.
    pub file: PathBuf,
    /// Raw structured path within that file.
    pub path: Vec<re_cordis_core::PathSegment>,
    /// Actual one-based parser position, when available.
    pub span: Option<SourceSpan>,
}
/// Source failure retaining its original cause.
#[derive(Clone, Debug)]
pub struct IncludeError {
    /// Human-readable reason.
    pub message: String,
    /// Actual source location, absent for runtime admission.
    pub location: Option<Rc<Location>>,
    /// File-winning conflicts observed before an operation failed.
    pub conflicts: Vec<Conflict>,
    cause: Option<Rc<dyn Error>>,
}
impl IncludeError {
    fn caused<E: Error + 'static>(error: E) -> Self {
        Self {
            message: error.to_string(),
            location: None,
            conflicts: vec![],
            cause: Some(Rc::new(error)),
        }
    }
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            location: None,
            conflicts: vec![],
            cause: None,
        }
    }
    fn located_cause<E: Error + 'static>(error: E, location: Location) -> Self {
        let mut error = Self::caused(error);
        error.location = Some(Rc::new(location));
        error
    }
    fn located(message: impl Into<String>, location: Location) -> Self {
        Self {
            message: message.into(),
            location: Some(Rc::new(location)),
            conflicts: vec![],
            cause: None,
        }
    }
}
impl fmt::Display for IncludeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}
impl Error for IncludeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_deref()
    }
}
impl From<std::io::Error> for IncludeError {
    fn from(e: std::io::Error) -> Self {
        Self {
            message: e.to_string(),
            location: None,
            conflicts: vec![],
            cause: Some(Rc::new(e)),
        }
    }
}
impl From<LoaderError> for IncludeError {
    fn from(e: LoaderError) -> Self {
        Self {
            message: e.to_string(),
            location: None,
            conflicts: vec![],
            cause: Some(Rc::new(e)),
        }
    }
}
/// Effective ownership category.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OwnerKind {
    /// Entry written directly in a file.
    File,
    /// One overridden entry key in a patch.
    Patch,
    /// Entry embedded in a patch insert or group config.
    Embedded,
}
/// Raw write-back address of an entry or overridden key.
#[derive(Clone, Debug)]
pub struct Owner {
    /// Actual canonical owning file.
    pub file: PathBuf,
    /// Raw owning entry or patch object path.
    pub path: Vec<re_cordis_core::PathSegment>,
    /// Ownership category.
    pub kind: OwnerKind,
}
/// Entry and per-key provenance.
#[derive(Clone, Debug)]
pub struct Ownership {
    /// Owner of the entry itself.
    pub entry: Owner,
    /// Overrides; absent keys inherit the entry owner.
    pub keys: BTreeMap<String, Owner>,
}
/// Include source options.
#[derive(Clone, Debug)]
pub struct IncludeOptions {
    /// Native root path, relative to the caller's working directory.
    pub path: PathBuf,
    /// Used only if the root is missing; persisted on flush/disposal.
    pub initial: Option<Value>,
}
impl IncludeOptions {
    /// Read a JSON/YAML source.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            initial: None,
        }
    }
    /// Supply missing-only initial content.
    pub fn initial(mut self, value: Value) -> Self {
        self.initial = Some(value);
        self
    }
}
/// Completed read/apply report.
#[derive(Clone, Debug, Default)]
pub struct Report {
    /// Whether accepted source bytes changed.
    pub changed: bool,
    /// Independent schema/lifecycle/source failures.
    pub failures: Vec<IncludeError>,
    /// File-winning decisions from three-way reconciliation.
    pub conflicts: Vec<Conflict>,
}
/// Wait-only barrier for already admitted loader work.
#[must_use = "include work has started; await its joined barrier"]
pub struct IncludeTask<T>(LocalBoxFuture<'static, Result<T, IncludeError>>);
impl<T> Future for IncludeTask<T> {
    type Output = Result<T, IncludeError>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

/// One file-winning key or structural decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conflict {
    /// Stable entry ID.
    pub id: String,
    /// Conflicting option key, or None for structural edits.
    pub key: Option<String>,
    /// Human-readable resolution.
    pub reason: String,
}
/// Explicit JSON option changes; config null differs from removing the config key.
#[derive(Clone, Debug, Default)]
pub struct JsonEdit {
    /// Keys to set.
    pub set: serde_json::Map<String, Value>,
    /// Keys to remove.
    pub remove: Vec<String>,
}
impl JsonEdit {
    /// Empty edit.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set an option key.
    pub fn set(mut self, key: impl Into<String>, value: Value) -> Self {
        self.set.insert(key.into(), value);
        self
    }
    /// Remove an option key.
    pub fn remove(mut self, key: impl Into<String>) -> Self {
        self.remove.push(key.into());
        self
    }
}

type Disposal = Shared<LocalBoxFuture<'static, Result<(), IncludeError>>>;
struct State {
    owner: FiberHandle<()>,
    parent: Context,
    loader: Loader,
    options: IncludeOptions,
    closed: Cell<bool>,
    busy: Cell<bool>,
    model: RefCell<Option<source::Model>>,
    live: RefCell<Vec<Value>>,
    provenance: RefCell<BTreeMap<String, Ownership>>,
    journal: RefCell<Vec<journal::Record>>,
    conflicts: RefCell<Vec<Conflict>>,
    cleanup_started: Cell<bool>,
    disposal: RefCell<Option<Disposal>>,
    ids: RefCell<BTreeMap<(PathBuf, String), String>>,
    next: Cell<u64>,
    final_error: RefCell<Option<IncludeError>>,
}
/// Owner-scoped file-backed configuration. Native mutable loader access is not exposed.
#[derive(Clone)]
pub struct Include(Rc<State>);
impl Include {
    /// Admit an owner before any filesystem access; use refresh to read/apply.
    pub fn new(
        owner: &Context,
        catalog: Catalog,
        options: IncludeOptions,
    ) -> Result<Self, IncludeError> {
        let plugin = Plugin::new("include", |_, ()| async { Ok::<_, Infallible>(()) });
        let fiber = owner.plugin(&plugin, ()).map_err(IncludeError::caused)?;
        let loader = Loader::new(&fiber.context(), catalog)?;
        let state = Rc::new(State {
            owner: fiber,
            parent: owner.clone(),
            loader,
            options,
            closed: Cell::new(false),
            busy: Cell::new(false),
            model: RefCell::new(None),
            live: RefCell::new(vec![]),
            provenance: RefCell::new(BTreeMap::new()),
            journal: RefCell::new(vec![]),
            conflicts: RefCell::new(vec![]),
            cleanup_started: Cell::new(false),
            disposal: RefCell::new(None),
            ids: RefCell::new(BTreeMap::new()),
            next: Cell::new(0),
            final_error: RefCell::new(None),
        });
        let retained = state.clone();
        state
            .owner
            .context()
            .on_dispose(move || {
                retained.closed.set(true);
                let result = if retained.cleanup_started.replace(true) {
                    retained
                        .final_error
                        .borrow()
                        .clone()
                        .map_or(Ok(Report::default()), Err)
                } else {
                    Include(retained.clone()).final_flush()
                };
                if let Err(e) = &result {
                    *retained.final_error.borrow_mut() = Some(e.clone());
                }
                async move { result.map(|_| ()) }
            })
            .map_err(IncludeError::caused)?;
        Ok(Self(state))
    }
    fn admit(&self) -> Result<(), IncludeError> {
        if self.0.busy.get() {
            return Err(IncludeError::new("include operation in progress"));
        }
        self.0
            .owner
            .context()
            .invoke_callback(|| ())
            .map_err(IncludeError::caused)?;
        if self.0.closed.get() {
            return Err(IncludeError::new("include disposed"));
        }
        Ok(())
    }
    /// Immutable native loader snapshots.
    pub fn entries(&self) -> Vec<EntrySnapshot> {
        self.0.loader.entries()
    }
    /// Effective source JSON, including in-memory anonymous IDs.
    pub fn data(&self) -> Value {
        Value::Array(self.0.live.borrow().clone())
    }
    /// Current entry/key write-back provenance.
    pub fn ownership(&self, id: &str) -> Option<Ownership> {
        self.0.provenance.borrow().get(id).cloned()
    }
    /// Final bounded cleanup I/O error, retained independently of waiter drop.
    pub fn final_error(&self) -> Option<IncludeError> {
        self.0.final_error.borrow().clone()
    }

    fn begin(&self) -> Result<OperationGuard<'_>, IncludeError> {
        self.admit()?;
        self.0.busy.set(true);
        Ok(OperationGuard(&self.0.busy))
    }
    fn id(&self, id: &str) -> Result<re_cordis_loader::EntryId, IncludeError> {
        self.0
            .loader
            .entries()
            .into_iter()
            .find(|e| e.id.as_str() == id)
            .map(|e| e.id)
            .ok_or_else(|| IncludeError::new("runtime entry missing"))
    }
    fn check_data(&self, data: &[Value]) -> Result<(), IncludeError> {
        SourceDocument::json(
            SourceId::new("runtime"),
            &serde_json::to_string(data).map_err(IncludeError::caused)?,
        )?;
        Ok(())
    }
    fn apply(&self, mut report: Report) -> Result<IncludeTask<Report>, IncludeError> {
        let data = self.0.live.borrow().clone();
        let model = self
            .0
            .model
            .borrow()
            .clone()
            .ok_or_else(|| IncludeError::new("refresh a source before mutating"))?;
        let locations = model.locations.clone();
        let spans = locations.clone();
        let doc = SourceDocument::json(
            SourceId::new("include"),
            &serde_json::to_string(&data).map_err(IncludeError::caused)?,
        )?
        .with_locations(move |p| spans.get(&source::path_key(p)).and_then(|l| l.span.clone()));
        let task = self.0.loader.reconcile(doc)?;
        Ok(IncludeTask(
            async move {
                let settled = task.await?;
                report
                    .failures
                    .extend(settled.failures.into_iter().map(|f| {
                        let mut e = IncludeError::from(f.error.clone());
                        if let LoaderError::Located { location, .. } = &f.error {
                            let mut p = location.path.clone();
                            loop {
                                if let Some(l) = locations.get(&source::path_key(&p)) {
                                    e.location = Some(Rc::new(l.clone()));
                                    break;
                                }
                                if p.pop().is_none() {
                                    break;
                                }
                            }
                        }
                        e
                    }));
                Ok(report)
            }
            .boxed_local(),
        ))
    }
    /// Number of coalesced pending entry records.
    pub fn pending(&self) -> usize {
        self.0.journal.borrow().len()
    }
    /// Reread/reconcile sources, retry pending writes, then join loader work.
    pub fn refresh(&self) -> Result<IncludeTask<Report>, IncludeError> {
        let _guard = self.begin()?;
        let mut next = self.0.next.get();
        let mut ids = self.0.ids.borrow().clone();
        let mut current = source::Model::load(&self.0.options, &mut ids, &mut next)?;
        current.rebuild_locations();
        let old = self.0.model.borrow().clone();
        let changed = old.as_ref().is_none_or(|m| !m.same_bytes(&current));
        let mut records = self.0.journal.borrow().clone();
        let conflicts = old
            .as_ref()
            .map(|old| journal::reconcile(&mut records, &old.data, &current.data))
            .unwrap_or_default();
        let routed = journal::route(current.clone(), &records)?;
        self.check_data(&routed.data)?;
        let same_live = *self.0.live.borrow() == routed.data;
        let warnings = current.warnings.clone();
        *self.0.model.borrow_mut() = Some(current);
        *self.0.provenance.borrow_mut() = routed.owners.clone();
        *self.0.live.borrow_mut() = routed.data;
        *self.0.journal.borrow_mut() = records;
        *self.0.ids.borrow_mut() = ids;
        self.0.next.set(next);
        self.remember_conflicts(conflicts);
        let needs_flush = self.pending() != 0
            || self
                .0
                .model
                .borrow()
                .as_ref()
                .is_some_and(|m| m.files.values().any(|d| d.bytes.is_none()));
        let mut report = Report {
            changed,
            failures: warnings,
            conflicts: self.take_conflicts(),
        };
        // Missing-only initial content remains read-only until explicit flush/disposal.
        if self.pending() != 0 {
            match self.flush_internal() {
                Ok(r) => {
                    report.conflicts.extend(r.conflicts);
                    report.failures.extend(r.failures);
                }
                Err(e) => report.failures.push(e),
            }
        }
        if !changed
            && same_live
            && !needs_flush
            && report.failures.is_empty()
            && report.conflicts.is_empty()
        {
            return Ok(IncludeTask(async { Ok(Report::default()) }.boxed_local()));
        }
        self.apply(report)
    }
    fn remember_conflicts(&self, conflicts: Vec<Conflict>) {
        let mut all = self.0.conflicts.borrow_mut();
        for c in conflicts {
            if !all.iter().any(|v| v == &c) {
                all.push(c);
            }
        }
    }
    fn take_conflicts(&self) -> Vec<Conflict> {
        self.0.conflicts.borrow_mut().drain(..).collect()
    }
    fn flush_with<F: FnMut(persist::Stage, &std::path::Path) -> std::io::Result<()>>(
        &self,
        hook: &mut F,
    ) -> Result<Report, IncludeError> {
        if self.0.model.borrow().is_none() {
            return Ok(Report {
                conflicts: self.take_conflicts(),
                ..Report::default()
            });
        }
        let mut last = None;
        for _ in 0..MAX_RETRIES {
            let old = self.0.model.borrow().clone().unwrap();
            let mut next = self.0.next.get();
            let mut ids = self.0.ids.borrow().clone();
            let mut current = match source::Model::load(&self.0.options, &mut ids, &mut next) {
                Ok(m) => m,
                Err(mut e) => {
                    e.conflicts = self.0.conflicts.borrow().clone();
                    return Err(e);
                }
            };
            current.rebuild_locations();
            let mut records = self.0.journal.borrow().clone();
            self.remember_conflicts(journal::reconcile(&mut records, &old.data, &current.data));
            let routed = journal::route(current.clone(), &records)?;
            self.check_data(&routed.data)?;
            *self.0.journal.borrow_mut() = records;
            *self.0.model.borrow_mut() = Some(current.clone());
            *self.0.provenance.borrow_mut() = routed.owners.clone();
            *self.0.live.borrow_mut() = routed.data.clone();
            *self.0.ids.borrow_mut() = ids;
            self.0.next.set(next);
            let mut failed = None;
            let mut changed = false;
            for (p, d) in &routed.files {
                let existing = current
                    .files
                    .get(p)
                    .ok_or_else(|| IncludeError::new("current source missing"))?;
                if d.value == existing.value && existing.bytes.is_some() {
                    continue;
                }
                let bytes = serde_json::to_vec_pretty(&d.value).map_err(IncludeError::caused)?;
                if bytes.len() > MAX_BYTES {
                    return Err(IncludeError::new("canonical file byte limit exceeded"));
                }
                match persist::replace_with(p, &bytes, existing.bytes.as_deref(), hook) {
                    Ok(()) => {
                        changed = true;
                        let mut saved = d.clone();
                        saved.spans = format::parse_document(
                            p,
                            std::str::from_utf8(&bytes).map_err(IncludeError::caused)?,
                        )?
                        .locations;
                        saved.bytes = Some(bytes);
                        current.files.insert(p.clone(), saved);
                        let mut baseline =
                            source::Model::from_files(current.root.clone(), current.files.clone())?;
                        baseline.rebuild_locations();
                        *self.0.model.borrow_mut() = Some(baseline);
                    }
                    Err(e) => {
                        failed = Some(e);
                        break;
                    }
                }
            }
            if let Some(e) = failed {
                last = Some(e);
                continue;
            }
            let mut saved = source::Model::from_files(current.root, current.files)?;
            saved.rebuild_locations();
            *self.0.provenance.borrow_mut() = saved.owners.clone();
            *self.0.live.borrow_mut() = saved.data.clone();
            *self.0.model.borrow_mut() = Some(saved);
            self.0.journal.borrow_mut().clear();
            return Ok(Report {
                changed,
                conflicts: self.take_conflicts(),
                failures: vec![],
            });
        }
        let mut e = last.unwrap_or_else(|| IncludeError::new("replacement retry limit exceeded"));
        e.conflicts = self.0.conflicts.borrow().clone();
        Err(e)
    }
    fn flush_internal(&self) -> Result<Report, IncludeError> {
        self.flush_with(&mut |_, _| Ok(()))
    }
    fn final_flush(&self) -> Result<Report, IncludeError> {
        if self.pending() == 0
            && !self
                .0
                .model
                .borrow()
                .as_ref()
                .is_some_and(|m| m.files.values().any(|d| d.bytes.is_none()))
        {
            Ok(Report {
                conflicts: self.take_conflicts(),
                ..Report::default()
            })
        } else {
            self.flush_internal()
        }
    }
    /// Write admitted JSON edits eagerly, then join reconciled loader work.
    pub fn flush(&self) -> Result<IncludeTask<Report>, IncludeError> {
        let _guard = self.begin()?;
        let report = self.flush_internal()?;
        self.apply(report)
    }
    fn runtime_task(&self, task: re_cordis_loader::LoaderTask<()>) -> IncludeTask<Report> {
        IncludeTask(
            async move {
                task.await?;
                Ok(Report::default())
            }
            .boxed_local(),
        )
    }
    /// Admit a bounded JSON key edit; rejected input changes neither journal nor state.
    pub fn edit(&self, id: &str, edit: JsonEdit) -> Result<IncludeTask<Report>, IncludeError> {
        let _guard = self.begin()?;
        if edit.set.contains_key("id") || edit.remove.iter().any(|k| k == "id") {
            return Err(IncludeError::new("entry ID cannot be edited"));
        }
        let entry_id = self.id(id)?;
        let mut live = self.0.live.borrow().clone();
        let old = patch::find(&live, id)
            .cloned()
            .ok_or_else(|| IncludeError::new("source entry missing"))?;
        let mut value = old.clone();
        let object = value
            .as_object_mut()
            .ok_or_else(|| IncludeError::new("entry must be an object"))?;
        for (k, v) in &edit.set {
            object.insert(k.clone(), v.clone());
        }
        for k in &edit.remove {
            object.remove(k);
        }
        let model = self
            .0
            .model
            .borrow()
            .clone()
            .ok_or_else(|| IncludeError::new("refresh required"))?;
        if let Some(owner) = model.owners.get(id) {
            let raw = model.files[&owner.entry.file]
                .value
                .pointer(&source::path_key(&owner.entry.path));
            if raw.and_then(|v| v.get("name")).and_then(Value::as_str) == Some("include")
                && (edit
                    .set
                    .keys()
                    .chain(edit.remove.iter())
                    .any(|k| k == "name" || k == "config" || k == "group"))
            {
                return Err(IncludeError::new(
                    "include descriptor paths change through source refresh",
                ));
            }
        }
        let mut spec = self
            .0
            .loader
            .decode_json(
                &serde_json::to_string(&vec![value.clone()]).map_err(IncludeError::caused)?,
            )?
            .remove(0);
        spec = spec.id(id);
        *patch::find_mut(&mut live, id).unwrap() = value.clone();
        self.check_data(&live)?;
        let mut changes = BTreeMap::new();
        for k in edit.set.keys().chain(edit.remove.iter()) {
            if old.get(k) != value.get(k) {
                changes.insert(k.clone(), value.get(k).cloned());
            }
        }
        if changes.is_empty() {
            return Ok(IncludeTask(async { Ok(Report::default()) }.boxed_local()));
        }
        let mut records = self.0.journal.borrow().clone();
        journal::merge(
            &mut records,
            journal::Record {
                snapshot: Some(journal::snapshot(&model, id, &value)?),
                original_parent: self
                    .0
                    .loader
                    .snapshot(&entry_id)
                    .and_then(|e| e.parent)
                    .map(|p| p.as_str().to_owned()),
                id: id.into(),
                created: false,
                remove: false,
                parent: None,
                position: journal::position(&live, id).map(|v| v.1).unwrap_or(0),
                changes,
            },
        )?;
        let planned = journal::route(
            self.0
                .model
                .borrow()
                .clone()
                .ok_or_else(|| IncludeError::new("refresh required"))?,
            &records,
        )?;
        let task = self.0.loader.update(&entry_id, spec)?;
        *self.0.live.borrow_mut() = live;
        *self.0.provenance.borrow_mut() = planned.owners;
        *self.0.journal.borrow_mut() = records;
        Ok(self.runtime_task(task))
    }
    /// Create JSON plugin/group options under a group or at the root.
    pub fn create(
        &self,
        mut value: Value,
        parent: Option<&str>,
        position: usize,
    ) -> Result<IncludeTask<Report>, IncludeError> {
        let _guard = self.begin()?;
        let mut next = self.0.next.get();
        let mut used = std::collections::BTreeSet::new();
        for e in self.entries() {
            used.insert(e.id.as_str().to_owned());
        }
        source::reserve(&value, &mut used);
        source::assign(
            &mut value,
            vec![],
            &self.0.options.path,
            &mut BTreeMap::new(),
            &mut used,
            &mut next,
        );
        let id = value
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| IncludeError::new("entry ID required"))?
            .to_owned();
        let spec = self
            .0
            .loader
            .decode_json(
                &serde_json::to_string(&vec![value.clone()]).map_err(IncludeError::caused)?,
            )?
            .remove(0);
        let mut live = self.0.live.borrow().clone();
        journal::place(&mut live, value.clone(), parent, position)?;
        self.check_data(&live)?;
        let changes = value
            .as_object()
            .unwrap()
            .iter()
            .filter(|(k, _)| *k != "id")
            .map(|(k, v)| (k.clone(), Some(v.clone())))
            .collect();
        let mut records = self.0.journal.borrow().clone();
        journal::merge(
            &mut records,
            journal::Record {
                snapshot: Some(value.clone()),
                original_parent: parent.map(str::to_owned),
                id,
                created: true,
                remove: false,
                parent: Some(parent.map(str::to_owned)),
                position,
                changes,
            },
        )?;
        let planned = journal::route(
            self.0
                .model
                .borrow()
                .clone()
                .ok_or_else(|| IncludeError::new("refresh required"))?,
            &records,
        )?;
        let parent_id = parent.map(|p| self.id(p)).transpose()?;
        self.0
            .loader
            .create_in(parent_id.as_ref(), position, spec)?;
        *self.0.live.borrow_mut() = live;
        *self.0.provenance.borrow_mut() = planned.owners;
        *self.0.journal.borrow_mut() = records;
        self.0.next.set(next);
        self.settle_internal()
    }
    /// Remove an entry/subtree; a pending create-then-remove leaves no file trace.
    pub fn remove(&self, id: &str) -> Result<IncludeTask<Report>, IncludeError> {
        let _guard = self.begin()?;
        let entry_id = self.id(id)?;
        let mut live = self.0.live.borrow().clone();
        let descendants = journal::children_ids(&live, id);
        journal::detach(&mut live, id).ok_or_else(|| IncludeError::new("entry missing"))?;
        let mut records = self.0.journal.borrow().clone();
        let model = self
            .0
            .model
            .borrow()
            .clone()
            .ok_or_else(|| IncludeError::new("refresh required"))?;
        for child in descendants {
            journal::prune(&mut records, &child);
            let pending = records.iter().position(|r| r.id == child);
            if patch::find(&model.data, &child).is_none() {
                if let Some(pos) = pending {
                    records.remove(pos);
                }
                continue;
            }
            journal::merge(
                &mut records,
                journal::Record {
                    snapshot: None,
                    original_parent: None,
                    id: child,
                    created: false,
                    remove: true,
                    parent: None,
                    position: 0,
                    changes: BTreeMap::new(),
                },
            )?;
        }
        let planned = journal::route(
            self.0
                .model
                .borrow()
                .clone()
                .ok_or_else(|| IncludeError::new("refresh required"))?,
            &records,
        )?;
        let task = self.0.loader.remove(&entry_id)?;
        *self.0.live.borrow_mut() = live;
        *self.0.provenance.borrow_mut() = planned.owners;
        *self.0.journal.borrow_mut() = records;
        Ok(self.runtime_task(task))
    }
    /// Relocate/reorder an entry; later JSON edits coalesce with that move.
    pub fn move_entry(
        &self,
        id: &str,
        parent: Option<&str>,
        position: usize,
    ) -> Result<IncludeTask<Report>, IncludeError> {
        let _guard = self.begin()?;
        let entry_id = self.id(id)?;
        let parent_id = parent.map(|p| self.id(p)).transpose()?;
        let mut live = self.0.live.borrow().clone();
        let v = journal::detach(&mut live, id).ok_or_else(|| IncludeError::new("entry missing"))?;
        let snapshot = journal::snapshot(
            &self
                .0
                .model
                .borrow()
                .clone()
                .ok_or_else(|| IncludeError::new("refresh required"))?,
            id,
            &v,
        )?;
        journal::place(&mut live, v, parent, position)?;
        let mut records = self.0.journal.borrow().clone();
        journal::merge(
            &mut records,
            journal::Record {
                snapshot: Some(snapshot),
                original_parent: self
                    .0
                    .loader
                    .snapshot(&entry_id)
                    .and_then(|e| e.parent)
                    .map(|p| p.as_str().to_owned()),
                id: id.into(),
                created: false,
                remove: false,
                parent: Some(parent.map(str::to_owned)),
                position,
                changes: BTreeMap::new(),
            },
        )?;
        let planned = journal::route(
            self.0
                .model
                .borrow()
                .clone()
                .ok_or_else(|| IncludeError::new("refresh required"))?,
            &records,
        )?;
        let task = self
            .0
            .loader
            .move_entry(&entry_id, parent_id.as_ref(), position)?;
        *self.0.live.borrow_mut() = live;
        *self.0.provenance.borrow_mut() = planned.owners;
        *self.0.journal.borrow_mut() = records;
        Ok(self.runtime_task(task))
    }
    fn settle_internal(&self) -> Result<IncludeTask<Report>, IncludeError> {
        let task = self.0.loader.settle()?;
        Ok(IncludeTask(
            async move {
                let s = task.await?;
                Ok(Report {
                    failures: s.failures.into_iter().map(|f| f.error.into()).collect(),
                    ..Report::default()
                })
            }
            .boxed_local(),
        ))
    }
    /// Join all admitted loader work without writing files.
    pub fn settle(&self) -> Result<IncludeTask<Report>, IncludeError> {
        let _guard = self.begin()?;
        self.settle_internal()
    }
    /// Stop admissions, perform one final bounded flush, then join the dedicated owner.
    pub fn dispose(&self) -> Result<IncludeTask<()>, IncludeError> {
        self.0
            .parent
            .invoke_callback(|| ())
            .map_err(IncludeError::caused)?;
        if let Some(work) = self.0.disposal.borrow().clone() {
            return Ok(IncludeTask(work.boxed_local()));
        }
        let _guard = self.begin()?;
        self.0.closed.set(true);
        self.0.cleanup_started.set(true);
        let result = self.final_flush();
        if let Err(e) = &result {
            *self.0.final_error.borrow_mut() = Some(e.clone());
        }
        let task = self.0.owner.dispose().map_err(IncludeError::caused)?;
        let work = async move {
            let joined = task.await.map_err(IncludeError::caused);
            result?;
            joined?;
            Ok(())
        }
        .boxed_local()
        .shared();
        *self.0.disposal.borrow_mut() = Some(work.clone());
        Ok(IncludeTask(work.boxed_local()))
    }
}

struct OperationGuard<'a>(&'a Cell<bool>);
impl Drop for OperationGuard<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod test_support;
#[cfg(test)]
mod journal_tests {
    use super::*;
    use re_cordis_core::Runtime;
    use serde_json::json;
    use test_support::*;
    #[tokio::test(flavor = "current_thread")]
    async fn gates_retain_pending_clean_staging_and_bound_attempts() {
        let d = Dir::new();
        d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
        let runtime = Runtime::new();
        run(&runtime, async {
            let i = Include::new(
                &runtime.context(),
                catalog(),
                IncludeOptions::new(d.path("a.json")),
            )
            .unwrap();
            i.refresh().unwrap().await.unwrap();
            i.edit("a", JsonEdit::new().set("config", json!(2)))
                .unwrap()
                .await
                .unwrap();
            for stage in [
                persist::Stage::Opened,
                persist::Stage::Written,
                persist::Stage::BeforeRename,
            ] {
                let mut attempts = 0;
                let result = i.flush_with(&mut |s, _| {
                    if s == stage {
                        attempts += 1;
                        Err(std::io::Error::new(
                            std::io::ErrorKind::PermissionDenied,
                            "injected",
                        ))
                    } else {
                        Ok(())
                    }
                });
                assert!(result.is_err());
                assert_eq!(attempts, MAX_RETRIES);
                assert_eq!(i.pending(), 1);
                assert_eq!(read(&d.path("a.json"))[0]["config"], 1);
                assert_eq!(
                    std::fs::read_dir(d.path("a.json").parent().unwrap())
                        .unwrap()
                        .count(),
                    1
                );
            }
            i.flush().unwrap().await.unwrap();
            assert_eq!(read(&d.path("a.json"))[0]["config"], 2);
            i.dispose().unwrap().await.unwrap();
        })
        .await;
    }
    #[tokio::test(flavor = "current_thread")]
    async fn retry_rereads_external_edit_and_conflict_is_observable() {
        let d = Dir::new();
        d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
        let runtime = Runtime::new();
        run(&runtime, async {
            let i = Include::new(
                &runtime.context(),
                catalog(),
                IncludeOptions::new(d.path("a.json")),
            )
            .unwrap();
            i.refresh().unwrap().await.unwrap();
            i.edit("a", JsonEdit::new().set("config", json!(2)))
                .unwrap()
                .await
                .unwrap();
            let mut first = true;
            let report = i
                .flush_with(&mut |stage, path| {
                    if first && stage == persist::Stage::BeforeCheck {
                        first = false;
                        std::fs::write(path, r#"[{"id":"a","name":"worker","config":3}]"#)?;
                    }
                    Ok(())
                })
                .unwrap();
            assert_eq!(report.conflicts.len(), 1);
            assert_eq!(read(&d.path("a.json"))[0]["config"], 3);
            assert_eq!(i.pending(), 0);
            i.dispose().unwrap().await.unwrap();
        })
        .await;
    }
    #[tokio::test(flavor = "current_thread")]
    async fn retry_does_not_overwrite_invalid_latest_and_preserves_changes() {
        let d = Dir::new();
        d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
        let runtime = Runtime::new();
        run(&runtime, async {
            let i = Include::new(
                &runtime.context(),
                catalog(),
                IncludeOptions::new(d.path("a.json")),
            )
            .unwrap();
            i.refresh().unwrap().await.unwrap();
            i.edit("a", JsonEdit::new().set("config", json!(2)))
                .unwrap()
                .await
                .unwrap();
            let mut first = true;
            assert!(
                i.flush_with(&mut |stage, path| {
                    if first && stage == persist::Stage::BeforeCheck {
                        first = false;
                        std::fs::write(path, "broken")?;
                    }
                    Ok(())
                })
                .is_err()
            );
            assert_eq!(i.pending(), 1);
            assert_eq!(std::fs::read_to_string(d.path("a.json")).unwrap(), "broken");
            d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
            i.flush().unwrap().await.unwrap();
            i.dispose().unwrap().await.unwrap();
        })
        .await;
    }
    #[tokio::test(flavor = "current_thread")]
    async fn conflicts_survive_a_later_io_failure_until_report_delivery() {
        let d = Dir::new();
        d.put("a.json", r#"[{"id":"a","name":"worker","config":1}]"#);
        let runtime = Runtime::new();
        run(&runtime, async {
            let i = Include::new(
                &runtime.context(),
                catalog(),
                IncludeOptions::new(d.path("a.json")),
            )
            .unwrap();
            i.refresh().unwrap().await.unwrap();
            i.edit(
                "a",
                JsonEdit::new()
                    .set("config", json!(2))
                    .set("disabled", json!(true)),
            )
            .unwrap()
            .await
            .unwrap();
            d.put("a.json", r#"[{"id":"a","name":"worker","config":3}]"#);
            let e = i
                .flush_with(&mut |_, _| {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "injected",
                    ))
                })
                .unwrap_err();
            assert_eq!(e.conflicts.len(), 1);
            assert_eq!(i.pending(), 1);
            let report = i.flush().unwrap().await.unwrap();
            assert_eq!(report.conflicts.len(), 1);
            assert_eq!(read(&d.path("a.json"))[0]["config"], 3);
            i.dispose().unwrap().await.unwrap();
        })
        .await;
    }
    #[test]
    fn record_capacity_and_byte_budget_reject_without_unbounded_backlog() {
        let mut records = vec![];
        for n in 0..MAX_ENTRIES {
            journal::merge(
                &mut records,
                journal::Record {
                    snapshot: None,
                    original_parent: None,
                    id: format!("id{n}"),
                    created: false,
                    remove: false,
                    parent: None,
                    position: 0,
                    changes: BTreeMap::new(),
                },
            )
            .unwrap();
        }
        let mut candidate = records.clone();
        assert!(
            journal::merge(
                &mut candidate,
                journal::Record {
                    snapshot: None,
                    original_parent: None,
                    id: "overflow".into(),
                    created: false,
                    remove: false,
                    parent: None,
                    position: 0,
                    changes: BTreeMap::new()
                }
            )
            .is_err()
        );
        assert_eq!(records.len(), MAX_ENTRIES);
        let mut candidate = records.clone();
        assert!(
            journal::merge(
                &mut candidate,
                journal::Record {
                    snapshot: None,
                    original_parent: None,
                    id: "id0".into(),
                    created: false,
                    remove: false,
                    parent: None,
                    position: 0,
                    changes: BTreeMap::from([(
                        "config".into(),
                        Some(Value::String("x".repeat(MAX_BYTES)))
                    )])
                }
            )
            .is_err()
        );
        assert_eq!(records[0].changes.len(), 0);
    }
    #[tokio::test(flavor = "current_thread")]
    async fn partial_cross_file_move_recovers_full_entry_in_both_write_orders() {
        for destination_first in [false, true] {
            let d = Dir::new();
            let (root, child) = if destination_first {
                ("a-root.json", "z-child.json")
            } else {
                ("z-root.json", "a-child.json")
            };
            d.put(
                child,
                r#"[{"id":"child","name":"worker","config":{"unchanged":7},"disabled":true}]"#,
            );
            d.put(
                root,
                &format!(r#"[{{"id":"inc","name":"include","config":{{"path":"{child}"}}}}]"#),
            );
            let runtime = Runtime::new();
            run(&runtime, async {
                let i = Include::new(
                    &runtime.context(),
                    catalog(),
                    IncludeOptions::new(d.path(root)),
                )
                .unwrap();
                i.refresh().unwrap().await.unwrap();
                i.move_entry("child", None, 1).unwrap().await.unwrap();
                let fail = d.path(if destination_first { child } else { root });
                let canonical = std::fs::canonicalize(&fail).unwrap();
                let result = i.flush_with(&mut |stage, path| {
                    if stage == persist::Stage::BeforeRename && path == canonical {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::PermissionDenied,
                            "second file",
                        ))
                    } else {
                        Ok(())
                    }
                });
                assert!(result.is_err());
                assert_eq!(i.pending(), 1);
                i.flush().unwrap().await.unwrap();
                assert!(read(&d.path(child)).as_array().unwrap().is_empty());
                let saved = read(&d.path(root));
                assert_eq!(saved[1]["id"], "child");
                assert_eq!(saved[1]["name"], "worker");
                assert_eq!(saved[1]["config"], json!({"unchanged":7}));
                assert_eq!(saved[1]["disabled"], true);
                assert_eq!(i.pending(), 0);
                i.dispose().unwrap().await.unwrap();
            })
            .await;
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn partial_parent_patch_then_child_file_failure_retains_remaining_edits() {
        let d = Dir::new();
        d.put(
            "z-child.json",
            r#"[{"id":"child","name":"worker","config":1}]"#,
        );
        d.put("a-root.json",r#"{"entries":[{"id":"inc","name":"include","config":{"path":"z-child.json"}}],"patches":[{"id":"child","disabled":true}]}"#);
        let runtime = Runtime::new();
        run(&runtime, async {
            let i = Include::new(
                &runtime.context(),
                catalog(),
                IncludeOptions::new(d.path("a-root.json")),
            )
            .unwrap();
            i.refresh().unwrap().await.unwrap();
            i.edit(
                "child",
                JsonEdit::new()
                    .set("disabled", json!(false))
                    .set("config", json!(2)),
            )
            .unwrap()
            .await
            .unwrap();
            let child = std::fs::canonicalize(d.path("z-child.json")).unwrap();
            assert!(
                i.flush_with(&mut |stage, path| if stage == persist::Stage::BeforeRename
                    && path == child
                {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "child",
                    ))
                } else {
                    Ok(())
                })
                .is_err()
            );
            assert_eq!(
                read(&d.path("a-root.json"))["patches"][0]["disabled"],
                false
            );
            assert_eq!(read(&d.path("z-child.json"))[0]["config"], 1);
            assert_eq!(i.pending(), 1);
            i.flush().unwrap().await.unwrap();
            assert_eq!(read(&d.path("z-child.json"))[0]["config"], 2);
            assert_eq!(i.pending(), 0);
            i.dispose().unwrap().await.unwrap();
        })
        .await;
    }
    #[tokio::test(flavor = "current_thread")]
    async fn child_edit_before_parent_move_survives_partial_source_removal() {
        let d = Dir::new();
        d.put(
            "a-child.json",
            r#"[{"id":"g","group":true,"config":[{"id":"child","name":"worker","config":1}]}]"#,
        );
        d.put(
            "z-root.json",
            r#"[{"id":"inc","name":"include","config":{"path":"a-child.json"}}]"#,
        );
        let runtime = Runtime::new();
        run(&runtime, async {
            let i = Include::new(
                &runtime.context(),
                catalog(),
                IncludeOptions::new(d.path("z-root.json")),
            )
            .unwrap();
            i.refresh().unwrap().await.unwrap();
            i.edit("child", JsonEdit::new().set("config", json!(2)))
                .unwrap()
                .await
                .unwrap();
            i.move_entry("g", None, 1).unwrap().await.unwrap();
            let root = std::fs::canonicalize(d.path("z-root.json")).unwrap();
            assert!(
                i.flush_with(&mut |stage, path| if stage == persist::Stage::BeforeRename
                    && path == root
                {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "root",
                    ))
                } else {
                    Ok(())
                })
                .is_err()
            );
            assert_eq!(i.pending(), 2);
            i.flush().unwrap().await.unwrap();
            let v = read(&d.path("z-root.json"));
            assert_eq!(v[1]["id"], "g");
            assert_eq!(v[1]["config"][0]["name"], "worker");
            assert_eq!(v[1]["config"][0]["config"], 2);
            assert!(read(&d.path("a-child.json")).as_array().unwrap().is_empty());
            i.dispose().unwrap().await.unwrap();
        })
        .await;
    }
}
