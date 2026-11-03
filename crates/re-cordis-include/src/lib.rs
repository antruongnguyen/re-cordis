//! Bounded JSON/YAML file sources over the typed re-cordis loader.
//!
//! File reads and writes are synchronous and bounded. Keep the original core
//! Runtime driven until the returned lifecycle barriers finish.
mod format;
mod patch;
mod source;
pub use format::{ParsedDocument, parse_document};
use futures_util::future::{FutureExt, LocalBoxFuture};
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
    pub location: Option<Location>,
    cause: Option<Rc<dyn Error>>,
}
impl IncludeError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            location: None,
            cause: None,
        }
    }
    fn located(message: impl Into<String>, location: Location) -> Self {
        Self {
            message: message.into(),
            location: Some(location),
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
            cause: Some(Rc::new(e)),
        }
    }
}
impl From<LoaderError> for IncludeError {
    fn from(e: LoaderError) -> Self {
        Self {
            message: e.to_string(),
            location: None,
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
struct State {
    owner: FiberHandle<()>,
    loader: Loader,
    options: IncludeOptions,
    closed: Cell<bool>,
    busy: Cell<bool>,
    model: RefCell<Option<source::Model>>,
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
        let fiber = owner
            .plugin(&plugin, ())
            .map_err(|e| IncludeError::new(e.to_string()))?;
        let loader = Loader::new(&fiber.context(), catalog)?;
        let state = Rc::new(State {
            owner: fiber,
            loader,
            options,
            closed: Cell::new(false),
            busy: Cell::new(false),
            model: RefCell::new(None),
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
                let result = Include(retained.clone()).flush_internal();
                if let Err(e) = &result {
                    *retained.final_error.borrow_mut() = Some(e.clone());
                }
                async move { result.map(|_| ()) }
            })
            .map_err(|e| IncludeError::new(e.to_string()))?;
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
            .map_err(|e| IncludeError::new(e.to_string()))?;
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
        Value::Array(
            self.0
                .model
                .borrow()
                .as_ref()
                .map(|m| m.data.clone())
                .unwrap_or_default(),
        )
    }
    /// Current entry/key write-back provenance.
    pub fn ownership(&self, id: &str) -> Option<Ownership> {
        self.0.model.borrow().as_ref()?.owners.get(id).cloned()
    }
    /// Final bounded cleanup I/O error, retained independently of waiter drop.
    pub fn final_error(&self) -> Option<IncludeError> {
        self.0.final_error.borrow().clone()
    }
    /// Reread the bounded source graph, retaining the last valid state on read/parse errors.
    pub fn refresh(&self) -> Result<IncludeTask<Report>, IncludeError> {
        self.admit()?;
        self.0.busy.set(true);
        let _guard = OperationGuard(&self.0.busy);
        let mut next = self.0.next.get();
        let mut ids = self.0.ids.borrow().clone();
        let mut model = source::Model::load(&self.0.options, &mut ids, &mut next)?;
        model.rebuild_locations();
        let changed = self
            .0
            .model
            .borrow()
            .as_ref()
            .is_none_or(|old| !old.same_bytes(&model));
        if !changed {
            return Ok(IncludeTask(async { Ok(Report::default()) }.boxed_local()));
        }
        let warnings = model.warnings.clone();
        let locations = model.locations.clone();
        let spans = locations.clone();
        let text =
            serde_json::to_string(&model.data).map_err(|e| IncludeError::new(e.to_string()))?;
        let doc = SourceDocument::json(
            SourceId::new(format!("include:{}", model.root.display())),
            &text,
        )?
        .with_locations(move |p| spans.get(&source::path_key(p)).and_then(|l| l.span.clone()));
        let task = self.0.loader.reconcile(doc)?;
        *self.0.model.borrow_mut() = Some(model);
        *self.0.ids.borrow_mut() = ids;
        self.0.next.set(next);
        Ok(IncludeTask(
            async move {
                let report = task.await?;
                let mut failures: Vec<_> = report
                    .failures
                    .into_iter()
                    .map(|f| {
                        let mut e = IncludeError::from(f.error.clone());
                        if let LoaderError::Located { location, .. } = &f.error {
                            let mut p = location.path.clone();
                            loop {
                                if let Some(l) = locations.get(&source::path_key(&p)) {
                                    e.location = Some(l.clone());
                                    break;
                                }
                                if p.pop().is_none() {
                                    break;
                                }
                            }
                        }
                        e
                    })
                    .collect();
                failures.extend(warnings);
                Ok(Report { changed, failures })
            }
            .boxed_local(),
        ))
    }
    fn flush_internal(&self) -> Result<Report, IncludeError> {
        let model = self.0.model.borrow().clone();
        let Some(mut model) = model else {
            return Ok(Report::default());
        };
        for (p, d) in &mut model.files {
            if d.bytes.is_none() {
                let text = serde_json::to_vec_pretty(&d.value)
                    .map_err(|e| IncludeError::new(e.to_string()))?;
                let mut f = match std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(p)
                {
                    Ok(f) => f,
                    Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(e) => return Err(e.into()),
                };
                use std::io::Write;
                f.write_all(&text)?;
                d.bytes = Some(text);
            }
        }
        *self.0.model.borrow_mut() = Some(model);
        Ok(Report::default())
    }
    /// Flush pending missing-only initial content.
    pub fn flush(&self) -> Result<Report, IncludeError> {
        self.admit()?;
        self.flush_internal()
    }
    /// Stop admission, flush admitted data, and join the loader subtree.
    pub fn dispose(&self) -> Result<IncludeTask<()>, IncludeError> {
        self.admit()?;
        self.0.closed.set(true);
        let result = self.flush_internal();
        if let Err(e) = &result {
            *self.0.final_error.borrow_mut() = Some(e.clone());
        }
        let task = self
            .0
            .owner
            .dispose()
            .map_err(|e| IncludeError::new(e.to_string()))?;
        Ok(IncludeTask(
            async move {
                let joined = task.await.map_err(|e| IncludeError::new(e.to_string()));
                result?;
                joined?;
                Ok(())
            }
            .boxed_local(),
        ))
    }
}

struct OperationGuard<'a>(&'a Cell<bool>);
impl Drop for OperationGuard<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}
