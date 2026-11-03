use crate::paths;
use futures_util::FutureExt;
use re_cordis_core::{
    AdmissionError, BoxError, CallbackError, Context, FiberHandle, LifecycleError, LifecycleTask,
    LocalFuture, Plugin, TaskControl,
};
use re_cordis_timer::{Scheduled, TimerError, TimerExt};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet, VecDeque},
    convert::Infallible,
    error::Error,
    ffi::OsString,
    fmt,
    fs::File,
    hash::{DefaultHasher, Hasher},
    io::{self, Read},
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, SystemTime},
};
use tokio::sync::Notify;
/// Bounds and clocks for exact registered-file watching.
#[derive(Clone, Debug)]
pub struct WatchOptions {
    /// Relative paths resolve against this fixed native base.
    pub base: PathBuf,
    /// Delay between bounded scans; must be nonzero.
    pub poll_interval: Duration,
    /// Trailing coalescing window, using the accepted timer.
    pub debounce: Duration,
    /// Resident registrations, including cancelled work awaiting destruction.
    pub max_watches: usize,
    /// Maximum cross-thread messages.
    pub queue_capacity: usize,
    /// Maximum retained events; oldest entries are evicted.
    pub event_capacity: usize,
    /// Maximum bytes fingerprinted per file.
    pub max_file_bytes: u64,
    /// Exact ignored component names for observe methods only.
    pub ignored: Vec<OsString>,
}
impl WatchOptions {
    /// Defaults: 100ms polling, 50ms debounce, 256 watches, 128 queue/events, 4MiB files.
    pub fn new(base: impl Into<PathBuf>) -> Self {
        Self {
            base: base.into(),
            poll_interval: Duration::from_millis(100),
            debounce: Duration::from_millis(50),
            max_watches: 256,
            queue_capacity: 128,
            event_capacity: 128,
            max_file_bytes: 4 * 1024 * 1024,
            ignored: vec![".git".into(), "target".into(), "node_modules".into()],
        }
    }
    fn validate(&self) -> Result<(), WatchError> {
        if self.poll_interval.is_zero()
            || std::time::Instant::now()
                .checked_add(self.poll_interval)
                .is_none()
            || self.max_watches == 0
            || self.queue_capacity == 0
            || self.event_capacity == 0
            || self.max_file_bytes == 0
            || self.max_file_bytes == u64::MAX
        {
            return Err(WatchError::InvalidOptions(
                "positive bounded capacities and poll interval are required",
            ));
        }
        if self.ignored.len() > 256 || self.ignored.iter().any(|s| s.len() > paths::MAX_PATH_BYTES)
        {
            return Err(WatchError::InvalidOptions(
                "ignored components exceed bounds",
            ));
        }
        Ok(())
    }
}
/// Producer used by this watcher.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchBackend {
    /// Joined standard-library polling.
    OwnedPolling,
    /// Explicit deterministic inputs, without a producer thread.
    Synthetic,
}
/// Honest watcher capabilities; reading this emits no warning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WatchCapabilities {
    /// Selected backend.
    pub backend: WatchBackend,
    /// Local configuration callbacks are available.
    pub file_callbacks: bool,
    /// Source registrations report a required external restart.
    pub source_restart_required: bool,
}
/// Exact, non-reused callback registration identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WatchId(u64);
/// Observed registered-file transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeKind {
    /// A missing file became present.
    Created,
    /// Present file fingerprint changed.
    Modified,
    /// Present file became missing.
    Removed,
    /// Initial observation or overflow requires a refresh.
    Rescan,
}
/// One coalesced native path transition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    /// Absolute sticky logical path.
    pub path: PathBuf,
    /// Latest transition, or forced rescan.
    pub kind: ChangeKind,
}
/// Origin of an observable failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchFailureStage {
    /// Bounded producer file sampling.
    Read,
    /// Local callback creation or polling.
    Callback,
    /// Producer shutdown or thread failure.
    Producer,
}
/// Retained original callback/I/O error; inspect through Error::source.
#[derive(Clone, Debug)]
pub struct WatchFailure {
    /// Callback identity, absent for producer failures.
    pub watch: Option<WatchId>,
    /// Native source path when known.
    pub path: Option<PathBuf>,
    /// Failed stage.
    pub stage: WatchFailureStage,
    source: Rc<dyn Error>,
}
impl fmt::Display for WatchFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.source.fmt(f)
    }
}
impl Error for WatchFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.source)
    }
}
/// Bounded public event history.
#[derive(Clone, Debug)]
pub enum WatchEvent {
    /// One path batch, before callbacks.
    Change(Change),
    /// The application must rebuild/restart for changed source.
    RestartRequired(Change),
    /// A retained failure, without disabling later changes.
    Failure(WatchFailure),
}
/// Value-only bounded-state and cumulative-loss counters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WatchStats {
    /// Active plus retiring resident registrations.
    pub watches: usize,
    /// Currently active registrations.
    pub active: usize,
    /// Unique paths sampled.
    pub paths: usize,
    /// Messages currently queued across threads.
    pub queued: usize,
    /// Registrations with one pending coalesced callback.
    pub pending: usize,
    /// Callback futures not yet destroyed.
    pub callbacks_running: usize,
    /// Retained public events.
    pub events: usize,
    /// Ingress overflow occurrences.
    pub queue_overflows: u64,
    /// Old public events evicted at capacity, including failures.
    pub events_evicted: u64,
    /// Producer has started and has not finished.
    pub producer_running: bool,
}
/// Admission/configuration/path failure before committing watch work.
#[derive(Debug)]
pub enum WatchError {
    /// Core callback/task admission was refused.
    Admission(AdmissionError),
    /// Core plugin/lifecycle failure, including refused self-join.
    Lifecycle(LifecycleError),
    /// Timer admission or checked deadline failed.
    Timer(TimerError),
    /// Original native I/O error.
    Io {
        /// Failing path.
        path: PathBuf,
        /// Original source.
        source: io::Error,
    },
    /// Invalid positive-capacity/clock settings.
    InvalidOptions(&'static str),
    /// Unsupported or oversized native path.
    InvalidPath {
        /// Refused path.
        path: PathBuf,
        /// Reason.
        reason: &'static str,
    },
    /// Resident quota or identity range exhausted.
    LimitExceeded(&'static str),
    /// Watcher admission has closed.
    Disposed,
}
impl fmt::Display for WatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission(e) => e.fmt(f),
            Self::Lifecycle(e) => e.fmt(f),
            Self::Timer(e) => e.fmt(f),
            Self::Io { source, .. } => source.fmt(f),
            Self::InvalidOptions(s) | Self::LimitExceeded(s) => f.write_str(s),
            Self::InvalidPath { reason, .. } => f.write_str(reason),
            Self::Disposed => f.write_str("watcher disposed"),
        }
    }
}
impl Error for WatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Admission(e) => Some(e),
            Self::Lifecycle(e) => Some(e),
            Self::Timer(e) => Some(e),
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}
impl From<AdmissionError> for WatchError {
    fn from(e: AdmissionError) -> Self {
        Self::Admission(e)
    }
}
impl From<LifecycleError> for WatchError {
    fn from(e: LifecycleError) -> Self {
        Self::Lifecycle(e)
    }
}
impl From<TimerError> for WatchError {
    fn from(e: TimerError) -> Self {
        Self::Timer(e)
    }
}
fn admitted(ctx: &Context) -> Result<(), WatchError> {
    ctx.invoke_callback(|| ()).map_err(|e| match e {
        CallbackError::Admission(e) => WatchError::Admission(e),
        CallbackError::Callback(e) => WatchError::Lifecycle(LifecycleError::Callback(e)),
    })
}

type Callback = Box<dyn FnMut(Change) -> LocalFuture<Result<(), BoxError>>>;
struct Entry {
    id: WatchId,
    path: PathBuf,
    active: Cell<bool>,
    source: bool,
    callback: RefCell<Option<Callback>>,
    pending: Cell<Option<ChangeKind>>,
    running: Cell<bool>,
    control: RefCell<Option<TaskControl>>,
}
enum Message {
    Change {
        change: Change,
        generation: u64,
    },
    Failure {
        path: PathBuf,
        generation: u64,
        error: io::Error,
    },
}
struct Shared {
    state: Mutex<SharedState>,
    notify: Notify,
    changed: Condvar,
    running: AtomicBool,
}
struct SharedState {
    paths: BTreeMap<PathBuf, u64>,
    queue: VecDeque<Message>,
    capacity: usize,
    overflow: bool,
    overflows: u64,
    stop: bool,
}
impl Shared {
    fn send(&self, message: Message) -> Result<SyntheticDelivery, SyntheticError> {
        let mut s = self.state.lock().unwrap();
        if s.stop {
            return Err(SyntheticError::Closed);
        }
        if s.queue.len() == s.capacity {
            s.overflow = true;
            s.overflows = s.overflows.saturating_add(1);
            drop(s);
            self.notify.notify_one();
            Ok(SyntheticDelivery::RescanRequested)
        } else {
            s.queue.push_back(message);
            drop(s);
            self.notify.notify_one();
            Ok(SyntheticDelivery::Queued)
        }
    }
    fn stop(&self) {
        let mut s = self.state.lock().unwrap();
        s.stop = true;
        s.paths.clear();
        s.queue.clear();
        s.overflow = false;
        drop(s);
        self.changed.notify_all();
        self.notify.notify_one();
    }
}
struct State {
    ctx: Context,
    options: WatchOptions,
    backend: WatchBackend,
    closed: Cell<bool>,
    entries: RefCell<BTreeMap<WatchId, Rc<Entry>>>,
    next: Cell<u64>,
    shared: Arc<Shared>,
    schedule: RefCell<Option<Scheduled>>,
    ready: Cell<bool>,
    receiver: RefCell<Option<TaskControl>>,
    producer: RefCell<Option<JoinHandle<()>>>,
    events: RefCell<VecDeque<WatchEvent>>,
    evicted: Cell<u64>,
}
impl State {
    fn admit(&self) -> Result<(), WatchError> {
        admitted(&self.ctx)?;
        if self.closed.get() {
            return Err(WatchError::Disposed);
        }
        Ok(())
    }
    fn event(&self, event: WatchEvent) {
        let displaced = {
            let mut es = self.events.borrow_mut();
            let old = if es.len() == self.options.event_capacity {
                self.evicted.set(self.evicted.get().saturating_add(1));
                es.pop_front()
            } else {
                None
            };
            es.push_back(event);
            old
        };
        drop(displaced);
    }
    fn failure(
        &self,
        id: Option<WatchId>,
        path: Option<PathBuf>,
        stage: WatchFailureStage,
        error: Rc<dyn Error>,
    ) {
        self.event(WatchEvent::Failure(WatchFailure {
            watch: id,
            path,
            stage,
            source: error,
        }));
    }
    fn stop_entry(&self, id: WatchId) {
        let entry = self.entries.borrow().get(&id).cloned();
        let Some(e) = entry else {
            return;
        };
        if !e.active.replace(false) {
            return;
        }
        e.pending.set(None);
        let callback = e.callback.borrow_mut().take();
        if let Some(c) = e.control.borrow().as_ref() {
            c.cancel();
        }
        if !self
            .entries
            .borrow()
            .values()
            .any(|v| v.active.get() && v.path == e.path)
        {
            self.shared.state.lock().unwrap().paths.remove(&e.path);
            self.shared.changed.notify_all();
        }
        if !e.running.get() {
            self.entries.borrow_mut().remove(&id);
        }
        drop(callback);
    }
    fn stop(&self) {
        if self.closed.replace(true) {
            return;
        }
        let ids = self.entries.borrow().keys().copied().collect::<Vec<_>>();
        for id in ids {
            self.stop_entry(id);
        }
        if let Some(s) = self.schedule.borrow_mut().take() {
            let _ = s.dispose();
        }
        if let Some(c) = self.receiver.borrow().as_ref() {
            c.cancel();
        }
        self.shared.stop();
    }
    fn changed(&self, c: &Change, g: u64) {
        if self.closed.get() || self.shared.state.lock().unwrap().paths.get(&c.path) != Some(&g) {
            return;
        }
        for e in self.entries.borrow().values() {
            if e.active.get() && e.path == c.path {
                e.pending.set(Some(c.kind));
            }
        }
    }
    fn burst(&self) {
        self.ready.set(false);
        if let Some(s) = self.schedule.borrow().as_ref() {
            if let Err(e) = s.call() {
                self.failure(None, None, WatchFailureStage::Callback, Rc::new(e));
            }
        }
    }
    fn dispatch(self: &Rc<Self>) {
        if self.closed.get() || !self.ready.get() {
            return;
        }
        let entries = self.entries.borrow().values().cloned().collect::<Vec<_>>();
        let mut announced = BTreeSet::new();
        for e in entries {
            if !e.active.get() || e.running.get() {
                continue;
            }
            let Some(kind) = e.pending.take() else {
                continue;
            };
            let change = Change {
                path: e.path.clone(),
                kind,
            };
            if announced.insert(e.path.clone()) {
                self.event(WatchEvent::Change(change.clone()));
                if self
                    .entries
                    .borrow()
                    .values()
                    .any(|v| v.active.get() && v.source && v.path == e.path)
                {
                    self.event(WatchEvent::RestartRequired(change.clone()));
                }
            }
            if e.source {
                continue;
            }
            let Some(mut callback) = e.callback.borrow_mut().take() else {
                continue;
            };
            e.running.set(true);
            let guard = RunningGuard {
                state: Rc::downgrade(self),
                entry: e.clone(),
            };
            let future = self.ctx.invoke_callback(|| callback(change));
            if e.active.get() {
                *e.callback.borrow_mut() = Some(callback);
            }
            match future {
                Err(error) => {
                    self.failure(
                        Some(e.id),
                        Some(e.path.clone()),
                        WatchFailureStage::Callback,
                        Rc::new(error),
                    );
                    drop(guard);
                }
                Ok(future) if !e.active.get() => {
                    drop(future);
                    drop(guard);
                }
                Ok(future) => {
                    let weak = Rc::downgrade(self);
                    let id = e.id;
                    let path = e.path.clone();
                    match self.ctx.task(async move {
                        let result = std::panic::AssertUnwindSafe(future).catch_unwind().await;
                        drop(guard);
                        move || {
                            if let Some(s) = weak.upgrade() {
                                match result {
                                    Ok(Err(error)) => s.failure(
                                        Some(id),
                                        Some(path),
                                        WatchFailureStage::Callback,
                                        Rc::from(error),
                                    ),
                                    Err(panic) => s.failure(
                                        Some(id),
                                        Some(path),
                                        WatchFailureStage::Callback,
                                        Rc::new(PanicMessage(panic_text(panic))),
                                    ),
                                    Ok(Ok(())) => {}
                                }
                            }
                        }
                    }) {
                        Ok((_, control)) => *e.control.borrow_mut() = Some(control),
                        Err(error) => self.failure(
                            Some(e.id),
                            Some(e.path.clone()),
                            WatchFailureStage::Callback,
                            Rc::new(error),
                        ),
                    }
                }
            }
        }
    }
    async fn receive(self: Rc<Self>) -> fn() {
        loop {
            self.shared.notify.notified().await;
            if self.closed.get() {
                break;
            }
            let (messages, rescan) = {
                let mut s = self.shared.state.lock().unwrap();
                (
                    std::mem::take(&mut s.queue),
                    std::mem::take(&mut s.overflow),
                )
            };
            let changed = rescan || !messages.is_empty();
            for message in messages {
                match message {
                    Message::Change { change, generation } => self.changed(&change, generation),
                    Message::Failure {
                        path,
                        generation,
                        error,
                    } => {
                        if self.shared.state.lock().unwrap().paths.get(&path) == Some(&generation) {
                            self.failure(None, Some(path), WatchFailureStage::Read, Rc::new(error));
                        }
                    }
                }
            }
            if rescan {
                for e in self.entries.borrow().values() {
                    if e.active.get() {
                        e.pending.set(Some(ChangeKind::Rescan));
                    }
                }
            }
            if changed {
                self.burst();
            }
            self.dispatch();
        }
        || {}
    }
}
struct HostGuard(Weak<State>);
impl Drop for HostGuard {
    fn drop(&mut self) {
        if let Some(s) = self.0.upgrade() {
            s.stop();
        }
    }
}
struct RunningGuard {
    state: Weak<State>,
    entry: Rc<Entry>,
}
impl Drop for RunningGuard {
    fn drop(&mut self) {
        self.entry.running.set(false);
        self.entry.control.borrow_mut().take();
        if let Some(s) = self.state.upgrade() {
            if !self.entry.active.get() {
                s.entries.borrow_mut().remove(&self.entry.id);
            }
            s.shared.notify.notify_one();
        }
    }
}
#[derive(Debug)]
struct PanicMessage(String);
impl fmt::Display for PanicMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl Error for PanicMessage {}
fn panic_text(p: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = p.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "non-string callback panic".into()
    }
}

/// Exact registered-file watcher hosted by one dedicated owner.
///
/// Drop requests cancellation. Original-host progress and owner disposal join
/// cleanup; directories are never traversed automatically.
pub struct FileWatcher {
    state: Rc<State>,
    owner: FiberHandle<()>,
}
impl FileWatcher {
    /// Admit a watcher and start its joined polling producer.
    pub fn new(owner: &Context, options: WatchOptions) -> Result<Self, WatchError> {
        Self::create(owner, options, WatchBackend::OwnedPolling)
    }
    /// Admit deterministic bounded input without a producer thread.
    pub fn synthetic(
        owner: &Context,
        options: WatchOptions,
    ) -> Result<(Self, SyntheticEvents), WatchError> {
        let w = Self::create(owner, options, WatchBackend::Synthetic)?;
        let input = SyntheticEvents(w.state.shared.clone());
        Ok((w, input))
    }
    fn create(
        parent: &Context,
        mut options: WatchOptions,
        backend: WatchBackend,
    ) -> Result<Self, WatchError> {
        admitted(parent)?;
        options.validate()?;
        let cwd = std::env::current_dir().map_err(|source| WatchError::Io {
            path: options.base.clone(),
            source,
        })?;
        options.base = paths::absolute(&cwd, &options.base);
        if let Ok(canonical) = std::fs::canonicalize(&options.base) {
            options.base = canonical;
        }
        let plugin = Plugin::new("hmr-watch", |_, ()| async { Ok::<_, Infallible>(()) });
        let owner = parent.plugin(&plugin, ())?;
        let shared = Arc::new(Shared {
            state: Mutex::new(SharedState {
                paths: BTreeMap::new(),
                queue: VecDeque::new(),
                capacity: options.queue_capacity,
                overflow: false,
                overflows: 0,
                stop: false,
            }),
            notify: Notify::new(),
            changed: Condvar::new(),
            running: AtomicBool::new(false),
        });
        let state = Rc::new(State {
            ctx: owner.context(),
            options,
            backend,
            closed: Cell::new(false),
            entries: RefCell::new(BTreeMap::new()),
            next: Cell::new(0),
            shared,
            schedule: RefCell::new(None),
            ready: Cell::new(false),
            receiver: RefCell::new(None),
            producer: RefCell::new(None),
            events: RefCell::new(VecDeque::new()),
            evicted: Cell::new(0),
        });
        let w = Self {
            state: state.clone(),
            owner,
        };
        let weak = Rc::downgrade(&state);
        let schedule = match state.ctx.debounce(state.options.debounce, move || {
            if let Some(s) = weak.upgrade() {
                s.ready.set(true);
                s.shared.notify.notify_one();
            }
        }) {
            Ok(s) => s,
            Err(e) => {
                drop(w.owner.dispose());
                return Err(e.into());
            }
        };
        *state.schedule.borrow_mut() = Some(schedule);
        let host_guard = HostGuard(Rc::downgrade(&state));
        let receiving = state.clone();
        let (_, control) = state.ctx.task(async move {
            let _guard = host_guard;
            receiving.receive().await
        })?;
        *state.receiver.borrow_mut() = Some(control);
        let cleanup = state.clone();
        state.ctx.on_dispose(move || {
            cleanup.stop();
            let handle = cleanup.producer.borrow_mut().take();
            async move {
                if let Some(handle) = handle {
                    let result = tokio::task::spawn_blocking(move || handle.join())
                        .await
                        .map_err(|e| Box::new(e) as BoxError)?;
                    if let Err(p) = result {
                        let message = panic_text(p);
                        cleanup.failure(
                            None,
                            None,
                            WatchFailureStage::Producer,
                            Rc::new(PanicMessage(message.clone())),
                        );
                        return Err(Box::new(PanicMessage(message)) as BoxError);
                    }
                }
                Ok::<_, BoxError>(())
            }
        })?;
        if backend == WatchBackend::OwnedPolling {
            let shared = state.shared.clone();
            let interval = state.options.poll_interval;
            let limit = state.options.max_file_bytes;
            shared.running.store(true, Ordering::Release);
            match thread::Builder::new()
                .name("re-cordis-hmr poll".into())
                .spawn(move || producer(shared, interval, limit))
            {
                Ok(handle) => *state.producer.borrow_mut() = Some(handle),
                Err(source) => {
                    state.shared.running.store(false, Ordering::Release);
                    drop(w.owner.dispose());
                    return Err(WatchError::Io {
                        path: state.options.base.clone(),
                        source,
                    });
                }
            }
        }
        Ok(w)
    }
    fn register(
        &self,
        path: &Path,
        callback: Option<Callback>,
        source: bool,
        observed: bool,
    ) -> Result<Option<WatchToken>, WatchError> {
        self.state.admit()?;
        if observed {
            let full = paths::absolute(&self.state.options.base, path);
            if full.components().any(|c| {
                self.state
                    .options
                    .ignored
                    .iter()
                    .any(|i| i == c.as_os_str())
            }) {
                return Ok(None);
            }
        }
        if self.state.entries.borrow().len() >= self.state.options.max_watches {
            return Err(WatchError::LimitExceeded("resident watch quota exceeded"));
        }
        let path = paths::resolve(&self.state.options.base, path)?;
        let next = self
            .state
            .next
            .get()
            .checked_add(1)
            .ok_or(WatchError::LimitExceeded("watch identity exhausted"))?;
        self.state.next.set(next);
        let id = WatchId(next);
        let e = Rc::new(Entry {
            id,
            path: path.clone(),
            active: Cell::new(true),
            source,
            callback: RefCell::new(callback),
            pending: Cell::new(None),
            running: Cell::new(false),
            control: RefCell::new(None),
        });
        self.state.entries.borrow_mut().insert(id, e.clone());
        {
            let mut s = self.state.shared.state.lock().unwrap();
            s.paths.entry(path.clone()).or_insert(next);
        }
        self.state.shared.changed.notify_all();
        if self.state.backend == WatchBackend::OwnedPolling {
            e.pending.set(Some(ChangeKind::Rescan));
            self.state.burst();
        }
        Ok(Some(WatchToken {
            id,
            path,
            state: Rc::downgrade(&self.state),
        }))
    }
    /// Watch an exact file, overriding ignored component names.
    pub fn watch<F>(&self, path: impl AsRef<Path>, callback: F) -> Result<WatchToken, WatchError>
    where
        F: FnMut(Change) -> LocalFuture<Result<(), BoxError>> + 'static,
    {
        Ok(self
            .register(path.as_ref(), Some(Box::new(callback)), false, false)?
            .unwrap())
    }
    /// Watch an exact file only if no ignored component matches.
    pub fn observe<F>(
        &self,
        path: impl AsRef<Path>,
        callback: F,
    ) -> Result<Option<WatchToken>, WatchError>
    where
        F: FnMut(Change) -> LocalFuture<Result<(), BoxError>> + 'static,
    {
        self.register(path.as_ref(), Some(Box::new(callback)), false, true)
    }
    /// Report source changes as restart-required, overriding ignores.
    pub fn watch_source(&self, path: impl AsRef<Path>) -> Result<WatchToken, WatchError> {
        Ok(self.register(path.as_ref(), None, true, false)?.unwrap())
    }
    /// Report source changes only when no ignored component matches.
    pub fn observe_source(&self, path: impl AsRef<Path>) -> Result<Option<WatchToken>, WatchError> {
        self.register(path.as_ref(), None, true, true)
    }
    /// Read supported capabilities without emitting a warning.
    pub fn capabilities(&self) -> WatchCapabilities {
        WatchCapabilities {
            backend: self.state.backend,
            file_callbacks: true,
            source_restart_required: true,
        }
    }
    /// Snapshot active/retiring bounds and cumulative loss counters.
    pub fn stats(&self) -> WatchStats {
        let es = self.state.entries.borrow();
        let s = self.state.shared.state.lock().unwrap();
        WatchStats {
            watches: es.len(),
            active: es.values().filter(|e| e.active.get()).count(),
            paths: s.paths.len(),
            queued: s.queue.len(),
            pending: es.values().filter(|e| e.pending.get().is_some()).count(),
            callbacks_running: es.values().filter(|e| e.running.get()).count(),
            events: self.state.events.borrow().len(),
            queue_overflows: s.overflows,
            events_evicted: self.state.evicted.get(),
            producer_running: self.state.shared.running.load(Ordering::Acquire),
        }
    }
    /// Drain bounded public events and their retained original causes.
    pub fn take_events(&self) -> Vec<WatchEvent> {
        self.state.events.borrow_mut().drain(..).collect()
    }
    /// Start owner disposal and join producer, timer and callback destruction.
    ///
    /// Core refuses self/ancestor joins before stopping any dispatch.
    pub fn dispose(&self) -> Result<LifecycleTask, WatchError> {
        let task = self.owner.dispose()?;
        self.state.stop();
        Ok(task)
    }
}
impl Drop for FileWatcher {
    fn drop(&mut self) {
        self.state.stop();
    }
}
/// Exact non-clone local registration. Drop requests cancellation.
pub struct WatchToken {
    id: WatchId,
    path: PathBuf,
    state: Weak<State>,
}
impl WatchToken {
    /// Exact registration identity.
    pub fn id(&self) -> WatchId {
        self.id
    }
    /// Sticky absolute native file identity.
    pub fn path(&self) -> &Path {
        &self.path
    }
    /// Suppress follow-up and request cancellation; owner disposal joins retirement.
    pub fn stop(&self) {
        if let Some(s) = self.state.upgrade() {
            s.stop_entry(self.id);
        }
    }
}
impl Drop for WatchToken {
    fn drop(&mut self) {
        self.stop();
    }
}
/// Send + Sync bounded input without any local host handles.
#[derive(Clone)]
pub struct SyntheticEvents(Arc<Shared>);
/// Whether input fit or forced a full registered-path rescan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntheticDelivery {
    /// Input queued.
    Queued,
    /// Full queue forced rescan.
    RescanRequested,
}
/// Refused synthetic input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntheticError {
    /// Watcher input has closed.
    Closed,
    /// Input must be a bounded absolute token path.
    InvalidPath,
}
impl fmt::Display for SyntheticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Closed => "synthetic input closed",
            Self::InvalidPath => "synthetic input requires a bounded absolute path",
        })
    }
}
impl Error for SyntheticError {}
impl SyntheticEvents {
    /// Queue a transition at an absolute token path; unknown paths are ignored.
    pub fn emit(&self, change: Change) -> Result<SyntheticDelivery, SyntheticError> {
        if !change.path.is_absolute() || change.path.as_os_str().len() > paths::MAX_PATH_BYTES {
            return Err(SyntheticError::InvalidPath);
        }
        let s = self.0.state.lock().unwrap();
        if s.stop {
            return Err(SyntheticError::Closed);
        }
        let Some(&generation) = s.paths.get(&change.path) else {
            return Ok(SyntheticDelivery::Queued);
        };
        drop(s);
        self.0.send(Message::Change { change, generation })
    }
    /// Force all current registrations dirty without blocking on queue capacity.
    pub fn rescan(&self) -> Result<(), SyntheticError> {
        let mut s = self.0.state.lock().unwrap();
        if s.stop {
            return Err(SyntheticError::Closed);
        }
        s.overflow = true;
        drop(s);
        self.0.notify.notify_one();
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Sample {
    Missing,
    Present {
        bytes: u64,
        modified: Option<SystemTime>,
        digest: u64,
    },
}
fn sample(path: &Path, limit: u64) -> io::Result<Sample> {
    if paths::symlink_component(path)? {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "symlink components are unsupported",
        ));
    }
    let m = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Sample::Missing),
        Err(e) => return Err(e),
    };
    if !m.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "registered path is not a regular file",
        ));
    }
    let mut input = File::open(path)?.take(limit + 1);
    let mut buffer = [0u8; 8192];
    let mut bytes = 0u64;
    let mut hash = DefaultHasher::new();
    loop {
        let n = input.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        bytes += n as u64;
        if bytes > limit {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "file exceeds watcher byte limit",
            ));
        }
        hash.write(&buffer[..n]);
    }
    Ok(Sample::Present {
        bytes,
        modified: m.modified().ok(),
        digest: hash.finish(),
    })
}
struct ProducerGuard(Arc<Shared>);
impl Drop for ProducerGuard {
    fn drop(&mut self) {
        self.0.running.store(false, Ordering::Release);
        self.0.notify.notify_one();
    }
}
type PreviousSample = (u64, Option<Sample>, Option<(io::ErrorKind, String)>);
fn producer(shared: Arc<Shared>, interval: Duration, limit: u64) {
    let _guard = ProducerGuard(shared.clone());
    let mut previous: BTreeMap<PathBuf, PreviousSample> = BTreeMap::new();
    loop {
        let paths = {
            let s = shared.state.lock().unwrap();
            if s.stop {
                break;
            }
            s.paths.clone()
        };
        previous.retain(|p, (g, _, _)| paths.get(p) == Some(g));
        for (path, generation) in &paths {
            if shared.state.lock().unwrap().stop {
                return;
            }
            let entry = previous
                .entry(path.clone())
                .or_insert((*generation, None, None));
            match sample(path, limit) {
                Ok(next) => {
                    let kind = if entry.1.is_none() {
                        Some(ChangeKind::Rescan)
                    } else if entry.1.as_ref() != Some(&next) || entry.2.is_some() {
                        Some(match (&entry.1, &next) {
                            (_, Sample::Missing) => ChangeKind::Removed,
                            (Some(Sample::Missing), _) => ChangeKind::Created,
                            _ => ChangeKind::Modified,
                        })
                    } else {
                        None
                    };
                    entry.1 = Some(next);
                    entry.2 = None;
                    if let Some(kind) = kind {
                        let _ = shared.send(Message::Change {
                            change: Change {
                                path: path.clone(),
                                kind,
                            },
                            generation: *generation,
                        });
                    }
                }
                Err(error) => {
                    let key = (error.kind(), error.to_string());
                    if entry.2.as_ref() != Some(&key) {
                        entry.2 = Some(key);
                        let _ = shared.send(Message::Failure {
                            path: path.clone(),
                            generation: *generation,
                            error,
                        });
                    }
                }
            }
        }
        let s = shared.state.lock().unwrap();
        if s.stop {
            break;
        }
        let _ = shared.changed.wait_timeout(s, interval).unwrap();
    }
}
