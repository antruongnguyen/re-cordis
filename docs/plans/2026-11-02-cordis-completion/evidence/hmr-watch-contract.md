# HMR H1 watcher contract and dependency gate

Date: 2026-11-03. Status: **reviewed exact Rust contract for H1 implementation**.
This file is a design and dependency investigation, not H1 delivery evidence.
Source pin: TypeScript Cordis 35fdc969e4b6c48c48ed44717c7a018324e4dd88.
Inherited Rust baseline: 47042d9 and accepted timer/core task, loader L5 and Include I4.

## Scope and selected backend

Watch only registered regular file paths. There is no directory recursion,
automatic Rust/Cargo dependency graph, native binary loading, or Node module cache.
An owned standard-library polling producer provides portable file watching with
an explicit JoinHandle. Poll interval and timer debounce are separate clocks.
Source registrations report restart-required; config registrations execute local
callbacks. Explicit callbacks remain useful when code replacement is unavailable.

All Context, Fiber, Scheduled, callback, and local error handles stay on the
original local host. Only owned PathBuf values, numeric generations, standard I/O
errors, bounded queue state and Arc cancellation state cross to the producer.

## Proposed exact public surface

The implementation may use the core BoxError and LocalFuture aliases below.
Public names are settled here so the H0 reviewer can accept or amend them.

~~~rust
pub struct WatchOptions {
    pub base: PathBuf,
    pub poll_interval: Duration,
    pub debounce: Duration,
    pub max_watches: usize,
    pub queue_capacity: usize,
    pub event_capacity: usize,
    pub max_file_bytes: u64,
    pub ignored: Vec<OsString>,
}
impl WatchOptions {
    pub fn new(base: impl Into<PathBuf>) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchBackend { OwnedPolling, Synthetic }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WatchCapabilities {
    pub backend: WatchBackend,
    pub file_callbacks: bool,
    pub source_restart_required: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WatchId(/* private exact monotonic identity */);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeKind { Created, Modified, Removed, Rescan }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    pub path: PathBuf,
    pub kind: ChangeKind,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchFailureStage { Read, Callback, Producer }
#[derive(Clone, Debug)]
pub struct WatchFailure {
    pub watch: Option<WatchId>,
    pub path: Option<PathBuf>,
    pub stage: WatchFailureStage,
    // Private Rc<dyn Error>; Error::source preserves the actual original cause.
}
impl std::fmt::Display for WatchFailure { /* ... */ }
impl std::error::Error for WatchFailure { /* original source */ }

#[derive(Clone, Debug)]
pub enum WatchEvent {
    Change(Change),
    RestartRequired(Change),
    Failure(WatchFailure),
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WatchStats {
    pub watches: usize, // resident active + retiring registrations
    pub active: usize,
    pub paths: usize,
    pub queued: usize,
    pub pending: usize,
    pub callbacks_running: usize,
    pub events: usize,
    pub queue_overflows: u64,
    pub events_evicted: u64,
    pub producer_running: bool,
}

pub enum WatchError {
    Admission(AdmissionError),
    Lifecycle(LifecycleError),
    Timer(TimerError),
    Io { path: PathBuf, source: std::io::Error },
    InvalidOptions(&'static str),
    InvalidPath { path: PathBuf, reason: &'static str },
    LimitExceeded(&'static str),
    Disposed,
}
impl std::fmt::Display for WatchError { /* ... */ }
impl std::error::Error for WatchError { /* retains source where present */ }

pub struct FileWatcher(/* private local owner state */);
impl FileWatcher {
    pub fn new(owner: &Context, options: WatchOptions) -> Result<Self, WatchError>;
    pub fn synthetic(
        owner: &Context, options: WatchOptions,
    ) -> Result<(Self, SyntheticEvents), WatchError>;

    pub fn watch<F>(
        &self, path: impl AsRef<Path>, callback: F,
    ) -> Result<WatchToken, WatchError>
    where F: FnMut(Change) -> LocalFuture<Result<(), BoxError>> + 'static;

    pub fn observe<F>(
        &self, path: impl AsRef<Path>, callback: F,
    ) -> Result<Option<WatchToken>, WatchError>
    where F: FnMut(Change) -> LocalFuture<Result<(), BoxError>> + 'static;

    pub fn watch_source(&self, path: impl AsRef<Path>) -> Result<WatchToken, WatchError>;
    pub fn observe_source(&self, path: impl AsRef<Path>)
        -> Result<Option<WatchToken>, WatchError>;
    pub fn capabilities(&self) -> WatchCapabilities;
    pub fn stats(&self) -> WatchStats;
    pub fn take_events(&self) -> Vec<WatchEvent>;
    pub fn dispose(&self) -> Result<LifecycleTask, WatchError>;
}
pub struct WatchToken(/* exact non-clone local registration */);
impl WatchToken {
    pub fn id(&self) -> WatchId;
    pub fn path(&self) -> &Path;
    pub fn stop(&self); // idempotent cancellation, allowed outside run
}
impl Drop for WatchToken { /* calls stop */ }
impl Drop for FileWatcher { /* requests stop; owner barrier joins */ }

#[derive(Clone)]
pub struct SyntheticEvents(/* Send + Sync bounded producer capability */);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntheticDelivery { Queued, RescanRequested }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntheticError { Closed, InvalidPath }
impl SyntheticEvents {
    pub fn emit(&self, change: Change) -> Result<SyntheticDelivery, SyntheticError>;
    pub fn rescan(&self) -> Result<(), SyntheticError>;
}
~~~

FileWatcher and WatchToken are not Send/Sync. SyntheticEvents is Send + Sync
and contains no local handles. Input paths must be absolute token.path() values;
unknown paths are ignored. A real producer is not started in synthetic mode.
Synthetic emit uses the same queue, identity checks, overflow, debounce, callbacks
and event history as production; only filesystem sampling is replaced.

Defaults: poll_interval 100 ms, debounce 50 ms, max_watches 256,
queue_capacity 128, event_capacity 128, max_file_bytes 4 MiB; ignored component
names are .git, target and node_modules. Registration count includes duplicate
callbacks and retired registrations whose callback destruction or admitted Include
barrier has not joined. Quota is released only when retirement has completed,
preventing unbounded stop/re-register accumulation. stats.watches includes those
resident registrations; stats.active reports the live subset. Capacities must be positive; poll interval must be positive and representable by checked Instant addition; checked
timer admission rejects unavailable clocks/unrepresentable deadlines before a
producer is started. Zero debounce is supported using accepted timer semantics.
base is resolved once at construction and need not be UTF-8. An existing base
is canonicalized once, accepting native base aliases such as macOS /var;
symlink components below the fixed base and explicit path symlinks are refused.

capabilities() is a pure value snapshot. Construction does not produce repeated
source-replacement warnings. take_events() drains the current bounded history;
it neither admits new work nor requires an active host. History evicts the oldest
event at capacity and increments events_evicted, including evicted failures.
Error causes remain original and inspectable while retained. Counter arithmetic
saturates rather than wrapping. Successful source events require a restart
supervisor supplied by the application; the watcher does not restart a process.

## Path identity and ignored paths

watch/watch_source explicitly override ignored names, including outside base.
observe/observe_source use native path components to filter ignored names;
a filtered result is Ok(None) and creates no token, timer, task or OS work.
These are exact component names, not a glob language. There is no automatic
directory root traversal, so excluded directories cost no scans.

Resolve relative inputs against the fixed base; normalize . and .. lexically.
Canonicalize the nearest existing parent/ancestor once, then append the unresolved
suffix and final file name. Keep that absolute logical path for the registration.
The final file inode is not canonical identity: atomic rename replacing its
inode still changes the same registered path. Missing files and missing parent
directories are valid registrations; remove/recreate of the logical parent
recovers under the same registration. Do not relocate a watch to a renamed
parent. A second registered destination sees its own create/modify transition.

Reject existing symlink components and a final symlink on admission. During
sampling, a path changed into a symlink or directory reports a read failure and
does not follow the replacement; later regular-file recovery remains usable.
This policy is ordinary filesystem behavior, not a security confinement boundary:
portable stdlib path APIs cannot atomically exclude hostile symlink substitutions
between metadata and open. No content bytes leave the producer.

Windows uses host Path/OsString operations and native canonical parent identity;
preserve filename case and do not invent Unicode lowercasing or JS file URLs.
Case variants may have distinct logical registrations that sample the same
native file; callbacks on both still run. Windows/Linux runtime verification is
not established by macOS smoke tests and must remain an explicit evidence gap.

## Sampling, queue and debounce bounds

The real backend's first observation enqueues Rescan; there is no quiet baseline.
A write immediately after watch() returns is therefore refreshed even if it
preceded the producer's first sample. Each duplicate real registration receives
an initial dirty Rescan too. Synthetic registrations produce only explicit
synthetic inputs. Scan only currently
registered unique paths; there is no read_dir or recursive traversal. Each
sample reads at most max_file_bytes + 1, in bounded chunks, to identify an
oversized file without an unbounded allocation. Compare length, modified time
and a streaming standard-library content digest to detect ordinary same-size
rewrites even when timestamp granularity masks them. The digest is process-local,
not persisted and not cryptographic; collision-free byte change detection is
not promised. A metadata/read failure keeps the previous accepted sample and
reports its actual I/O cause; unchanged repeated failures are coalesced.
Oversized files fail visibly and resume after a later valid bounded rewrite.
Changes entirely between polling samples may be missed.

One bounded ingress queue carries one path/generation change per message.
A unique path generation changes when its last token is removed and it is later
registered again. A queued event from an old generation cannot invoke a new
registration at that path. Duplicate live callbacks have distinct WatchIds.

try_send never blocks the filesystem producer. Full ingress sets an atomic
rescan flag and wakes the local receiver even when no message can be enqueued.
On rescan, every currently active registered path is marked dirty; invoke its
callbacks with ChangeKind::Rescan after the debounce window. This deliberately
refreshes unchanged files to recover missed events. It is a full rescan of the
bounded registered set, not an inferred filesystem or source dependency graph.

Pending state is bounded by active registrations, not burst length. One accepted
Scheduled debounce controls the current burst; its local callback wakes the
receiver/dispatcher. One callback future per registration runs at a time;
events arriving during it collapse into one follow-up dirty state. Unrelated
registrations may progress independently. Drop/stop removes the exact callback,
and no callback starts for a stopped WatchId. Change emits once per active path
batch; RestartRequired emits once for that batch when any active source token
uses the path. Callback failures become retained Failure events and do not
disable later valid refreshes.

## Admission, cancellation and completion

FileWatcher owns a dedicated core Fiber. Constructor validates options, host,
owner and timer before spawning the producer. All registration methods admit
the watcher owner and fail before committing new work when disposed/wrong-host.
Reading stats/events/capabilities and token cancellation are authority-free.

The receiver and each callback future use Context::task on the dedicated owner.
Call the user's synchronous FnMut through Context::invoke_callback; polling its
returned future remains inside the hosted task. Per-callback failure/panic
containment must leave the registration usable, clear the running marker with
a destruction guard, and retain returned BoxError causes. Core's contained sync
panic remains visible through Runtime::take_failures; asynchronous panic
messages also produce watcher Failure events. Avoid holding RefCell borrows
across user callbacks or awaits.

stop()/token Drop synchronously marks its WatchId inactive, removes exact map
state and producer membership when last for the path, drops idle captures,
cancels an active callback via TaskControl and suppresses all pending dispatch.
This is a cancellation request, not a blocking join. Captures in cancelled tasks
retire with original-host progress or the watcher/owner disposal barrier.
Dropping a token must not remove sibling callbacks on the same path.

Watcher disposal has a synchronous prefix: close admission/dispatch, clear dirty
state and idle callbacks, dispose Scheduled, cancel receiver/callback controls,
set producer stop and notify its Condvar. Its cleanup future moves only the
std::thread::JoinHandle into tokio::task::spawn_blocking and awaits join; no
local handle enters that blocking job. The dedicated owner's LifecycleTask
joins core task destruction/retirement and producer cleanup before completing.
Ancestor disposal must execute the same stop prefix and join barrier. Dropping
the returned LifecycleTask does not revoke already-started cleanup. Repeated
explicit disposal returns the existing joined lifecycle outcome.

The Condvar wait is interruptible; a filesystem read already blocked inside the
OS is not. Disposal guarantees joining when that operation returns, not a hard
wall-clock shutdown deadline. Do not claim that cancelling an arbitrary callback
rolls back external effects it already started.

For Include integration, refresh() admits work before returning IncludeTask.
Its waiter Drop is inert and may leave separately owned loader work progressing.
The H3 adapter retains the already-admitted IncludeTask as a Shared local
completion outside the cancelable callback task. Token stop prevents follow-up,
and watcher/ancestor disposal joins the retained completion even after token Drop
or callback abort. No new Include::settle/refresh admission is attempted during
cleanup, because Include's owner admission may already have closed. Returned
Include errors and every Report.failures cause remain visible in bounded events.
The exact optional adapter signature is:

~~~rust
#[cfg(feature = "include")]
pub fn watch_include(&self, path: impl AsRef<Path>, include: Include)
    -> Result<WatchToken, WatchError>;
~~~
Include continues to have no HMR dependency. An optional HMR-side dependency
can add watch_include(path, include), with explicit chosen path registrations;
it must not imply an automatically discovered include graph without a reviewed
Include path-snapshot seam. Invalid JSON/YAML remains visible and a later valid
file change refreshes the same Include.

## Dependency gate, executed on aarch64 macOS

Reviewed versions:

| Version | Declared Rust minimum | Executed 1.85.0 result | Shutdown finding |
|---|---|---|---|
| notify 7.0.0 | 1.72 | Metadata/source review only | PollWatcher drops a stop flag; detached thread |
| notify 8.2.0 | 1.77 | check, strict all-target Clippy, no-deps Rustdoc pass | Same detached PollWatcher; native backend differs by OS |
| notify 8.2.1 | 1.85 | check, strict all-target Clippy, no-deps Rustdoc pass | Same detached PollWatcher; native backend differs by OS |

8.2.0 resolved notify-types 2.1.0 (Rust 1.85) using resolver 3's MSRV-aware
selection; 2.2.0 requires Rust 1.88. 8.2.1 likewise resolved 2.1.0 in the
isolated MSRV probe. No dependency from these probes is added to the workspace.

Source findings at notify 8.2.1: poll.rs:507-518 has no JoinHandle;
poll.rs:582 starts a worker whose result is discarded; poll.rs:673-676 Drop only
stores want_to_stop. Even manual poll mode uses this worker. inotify.rs:125
starts a detached loop and Drop:632 signals shutdown without retaining/joining
its handle. windows.rs:104 starts a detached loop and Drop:590 signals quit.
macOS fsevent.rs:330 does join its owned thread. Thus neither PollWatcher nor a
cross-platform RecommendedWatcher establishes the required joined shutdown.
notify 7.0.0 and 8.2.0 have the same relevant PollWatcher limitation. We reject
these backends for this contract despite successful dependency compilation.

Probe roots/logs, all ignored by Git:
target/hmr-h1-probe/{Cargo.toml,Cargo.lock,metadata.json,check-1.85.0.log,
clippy-1.85.0.log,doc-1.85.0.log,notify-7.0.0-info.log,notify-8.2.1-info.log};
target/hmr-h1-probe-821/{Cargo.toml,Cargo.lock,check-1.85.0.log,
clippy-1.85.0.log,doc-1.85.0.log}. Build output: target/hmr-h1-probe-build.

Every Cargo/rustc/rustdoc/cargo-clippy/clippy-driver path was resolved with
rustup which --toolchain 1.85.0. The resolved toolchain bin led PATH;
RUSTUP_TOOLCHAIN=1.85.0, RUSTC and RUSTDOC were explicitly exported.
No Homebrew tool counted toward the gate.

Executed versions: cargo 1.85.0 (d73d2caf9 2024-12-31);
rustc/rustdoc 1.85.0 (4d91de4e4 2025-02-17);
cargo-clippy/clippy-driver 0.1.85 (4d91de4e48 2025-02-17).

## H1 implementation oracles

Deterministic synthetic tests must establish exact token removal, duplicate
coalescing, multiple callbacks, ignored observe versus explicit override,
bounded queue overflow recovery, callback error/panic recovery, dropped-token
generation suppression, one concurrent callback per registration, owner shutdown
and destruction before the disposal barrier completes. Prove SyntheticEvents
is Send + Sync and local watcher/token remain non-Send through compile checks.

Real tests supplement those oracles with equal-length rewrites, atomic rename,
remove/recreate (including parent removal), missing initial paths, over-limit
recovery and symlink refusal/recovery. A blocked callback future must release
its capture before joined disposal completes; producer completion must be
observable before the same barrier. File/poll tests use bounded deadlines and
always join shutdown. H3 additionally proves Include parse recovery and joins
separately admitted Include work. No tracked production code has been edited
for this proposal.

## Closed H0 choices
    
    Keep the four explicit/observed registration methods. Bounded streaming digest
    with the stated collision caveat is accepted. Token stop remains request-only;
    watcher/ancestor lifecycle completion joins retirement. Attempting watcher
    disposal from its own callback or receiver must preserve
    LifecycleError::Admission(ReentrantDrain) before any stop side effect. H3 must
    retain and join already-admitted IncludeTask completions as described above.
    These choices need implementation tests; this contract does not claim delivery.
    