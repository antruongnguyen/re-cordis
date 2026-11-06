# HMR satellite

[Tiếng Việt](../../vi/api/hmr.md) | [API index](README.md)

Source: [lib.rs](../../../crates/re-cordis-hmr/src/lib.rs),
[watch.rs](../../../crates/re-cordis-hmr/src/watch.rs) and
[reload.rs](../../../crates/re-cordis-hmr/src/reload.rs).
Guide: [file changes and replacement](../guides/hmr.md).

`re-cordis-hmr` is unpublished (`0.0.0`, Rust 2024, MSRV 1.85).
This page describes the implemented Rust API. H0–H3 is independently accepted;
see the [shared verification](../../plans/2026-11-02-cordis-completion/evidence/hmr-acceptance.md). File watching, explicit linked replacement and source restart
notification are separate capabilities. No root recursion, module discovery,
Node import/cache, native hot loading or process supervisor is provided.

## WatchOptions and capabilities

Construct options with `WatchOptions::new(base)`. Fields and defaults:

| Field | Default / contract |
|---|---|
| `base: PathBuf` | Supplied native base, resolved once; non-UTF-8 paths supported |
| `poll_interval: Duration` | 100 ms, positive |
| `debounce: Duration` | 50 ms; zero supported |
| `max_watches: usize` | 256 active and unfinished retired registrations |
| `queue_capacity: usize` | 128 ingress messages |
| `event_capacity: usize` | 128 retained events, oldest evicted when full |
| `max_file_bytes: u64` | 4 MiB; sampling reads at most bound + 1 in chunks |
| `ignored: Vec<OsString>` | Exact component names `.git`, `target`, `node_modules` |

Positive capacities and checked timer admission are required before producer
startup. max_file_bytes must be below u64::MAX; paths are bounded to 4096
native bytes. ignored accepts at most 256 names, each at most 4096 native bytes. Polling uses metadata plus a bounded streaming digest; between-sample
changes and digest collisions can be missed. This is ordinary file watching,
not hostile-filesystem confinement. Windows/Linux runtime acceptance is pending.

`WatchCapabilities` reports `backend: WatchBackend` (`OwnedPolling` or
`Synthetic`), `file_callbacks: bool` and `source_restart_required: bool`.
Reading it admits no work and emits no warnings.

## FileWatcher

FileWatcher owns a dedicated core Fiber and stays local, without Send/Sync.
Construct/register on the original live Cordis host with Tokio time enabled.

| Method | Result | Contract |
|---|---|---|
| `new(owner, options)` | `Result<FileWatcher, WatchError>` | Owned polling producer and joined shutdown |
| `synthetic(owner, options)` | `Result<(FileWatcher, SyntheticEvents), WatchError>` | Same bounded dispatch; no filesystem producer |
| `watch(path, callback)` | `Result<WatchToken, WatchError>` | Explicit regular-file path; overrides ignored components |
| `observe(path, callback)` | `Result<Option<WatchToken>, WatchError>` | Ignored component returns None before admitting work |
| `watch_source(path)` | `Result<WatchToken, WatchError>` | Explicit path emits RestartRequired |
| `observe_source(path)` | `Result<Option<WatchToken>, WatchError>` | Filtered source registration |
| `capabilities()` | `WatchCapabilities` | Pure snapshot |
| `stats()` | `WatchStats` | Resident/active work and bounded queue/history counters |
| `take_events()` | `Vec<WatchEvent>` | Drains bounded history; safe outside the host |
| `dispose()` | `Result<LifecycleTask, WatchError>` | Stops admission/dispatch, then joins tasks and producer |

Callbacks are `FnMut(Change) -> LocalFuture<Result<(), BoxError>> + 'static`.
Invocation and returned-future polling use accepted core hosting. Each
registration has one active callback and one coalesced follow-up; unrelated
registrations progress independently. Original returned errors are retained;
panic containment keeps later events usable and preserves core diagnostics.

Paths use nearest existing parent identity plus the logical file suffix, not
the final inode. Relative paths resolve against base; missing files/parents
are allowed. Final/ancestor symlinks are refused on admission, sampled
symlinks/directories fail visibly, and regular-file recovery is allowed.
Real registrations receive initial Rescan. Last-token removal changes path
generation, so queued stale events cannot invoke a new token at that path.

Self/ancestor lifecycle drains from a hosted callback are refused before stop
side effects. Disposal joins after an already-blocked OS read returns; no hard
wall-clock shutdown deadline is promised. Drop requests stop; the owner barrier
observes completion. Dropping a LifecycleTask does not revoke admitted cleanup.

## Tokens, synthetic input and observations

`WatchToken` is an exact, non-clone local registration with `id() -> WatchId`,
`path() -> &Path` and idempotent `stop()`. Stop/Drop is cancellation, safe
outside the host; completion needs host progress or owner/watcher disposal.
Stopping one token does not remove sibling callbacks.

`SyntheticEvents` is cloneable and Send + Sync. `emit(Change)` returns
`Result<SyntheticDelivery, SyntheticError>`; `rescan()` returns
`Result<(), SyntheticError>`. Use absolute token paths. Unknown paths are
ignored. Full queue returns `RescanRequested`, recovering through the bounded
registered set; `Queued` means ingress admission, not callback completion.
SyntheticError is `Closed` or `InvalidPath`.

`Change { path: PathBuf, kind: ChangeKind }` uses Created, Modified, Removed or
Rescan. `WatchEvent` is Change, RestartRequired or Failure.
`WatchFailure` has optional watch/path, `WatchFailureStage` Read/Callback/Producer
and an original error source. History eviction can remove failures too.

WatchStats exposes `watches` (active plus unfinished retirement), `active`,
`paths`, `queued`, `pending`, `callbacks_running`, `events`,
`queue_overflows`, `events_evicted` and `producer_running`.
Counters saturate. Pending work is bounded by registrations, not burst length.

WatchError variants are Admission, Lifecycle, Timer, Io (path/source),
InvalidOptions, InvalidPath (path/reason), LimitExceeded and Disposed.
Source-bearing variants preserve their causes.

## Optional Include adapter

With feature `include`, call
`watch_include(path, include) -> Result<WatchToken, WatchError>`.
It registers one explicit trigger path and refreshes the supplied Include,
including its source graph. Paths are not discovered automatically; explicit
registration overrides ignored components. Include has no HMR dependency.

The adapter retains the admitted IncludeTask independently from callback
cancellation, starts at most one refresh plus one coalesced follow-up per
registration, and joins retained completion on watcher/ancestor disposal.
No new Include admission occurs during cleanup. Returned Include errors and
each Report failure enter bounded watcher history once per completed refresh,
preserving original causes and locations. Ordinary core cleanup failures use
Runtime diagnostics; inspect `Runtime::take_failures` too. Unfinished retirement
counts against max_watches; this adapter does not dispose the supplied Include.

## ReloadNode, ReloadPlan and FactoryReplacement

`ReloadNode { name: String, factory: Option<String>, dependencies: Vec<String> }`
is an explicit application declaration. None marks a dependency-only unit.
Node/factory names are unique and contain 1–256 UTF-8 bytes. Every edge names
an existing node. Limits are 256 nodes, 1024 edges and 256 changed units/candidates.
Cycles converge through a bounded fixed point.

| Method | Contract |
|---|---|
| `ReloadPlan::new(nodes) -> Result<ReloadPlan, LoaderError>` | Check graph names, mappings, edges and bounds |
| `affected(changed) -> Result<Vec<String>, LoaderError>` | Reverse dependency closure, factory output in node order; unknown unit fails |
| `replace(loader, changed, replacements) -> Result<LoaderTask<Settlement>, LoaderError>` | Exact candidate coverage and one loader batch |
| `replace_include(include, changed, replacements) -> Result<IncludeTask<Report>, IncludeError>` | Optional include feature: the same selection checks and exact Include/loader batch completion |

The optional `replace_include` forwards through `Include::replace_factories`
for entries managed by that same Include. It preserves journal, data,
provenance and file bytes, performs no flush, and returns the exact loader
batch report through IncludeTask. Include still has no HMR dependency.

The plan owns no resource or worker. Empty changes select no factories;
extra candidates still fail. Unchanged shared dependencies do not select
unrelated consumers. Filenames and Cargo metadata imply no graph.

HMR re-exports loader `FactoryReplacement`. Use `new(name, Plugin<C,R>)`,
`name()` and optional `with_preflight(Fn(&Config) -> Result<(), E>)`.
Loader checks the entire candidate batch before catalog publication or unload:
existence, uniqueness, retained source/config types, bounds and optional
hosted raw-config preflight. Preflight must be pure; same-loader mutation and
dispose admission are guarded. It does not replace dependency-ready core
Plugin validation or make external effects transactional.

Loader retains each entry's config and exact old providers through cleanup,
then uses latest live entry/parent intent. Independent siblings progress.
A supplied host candidate rebuilds its nested core plugins; no arbitrary nested
declaration replacement inside an unchanged host is offered.
Settlement reports post-unload failures. A later explicit candidate/update
recovers them; completed teardown is not rolled back.
