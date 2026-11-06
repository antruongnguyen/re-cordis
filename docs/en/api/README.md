# API reference

[Tiếng Việt](../../vi/api/README.md) | [English home](../README.md)

This reference covers the public exports of `re-cordis-core` and the
`re-cordis-timer`, `re-cordis-logger`, `re-cordis-loader`, `re-cordis-include`,
`re-cordis-hmr` and selected offline `re-cordis-create` satellites, not their private storage or example-private
handles. It describes usage and behavioral contracts; generated Rustdoc is
authoritative for exact signatures, generic bounds, trait implementations, and
public fields.

The [whole-program guide](../guides/system-lifecycle.md) combines these APIs.
Its application types remain example-private and introduce no public exports.

The [public development website](https://antruongnguyen.github.io/re-cordis/en/docs/api/)
also bundles exact Rustdoc from its deployed build revision.
See [documentation channels](../guides/documentation.md) for public and local output,
HMR's optional Include API and future exact-version docs.rs policy.

## Generate exact API documentation

From the repository root:

```sh
cargo doc -p re-cordis-core --no-deps --locked
```

Open `target/doc/re_cordis_core/index.html`, or the equivalent path under your
`CARGO_TARGET_DIR`. Nothing is published to docs.rs by this command.
An unpublished `0.0.0` package is not a supported crates.io installation target.

## Public export map

Every root export from [lib.rs](../../../crates/re-cordis-core/src/lib.rs) belongs
to one of these chapters, and the `re-cordis-timer` satellite's exports are
covered by the [timer chapter](timer.md); console exports belong to
[console logger](console-logger.md). Context extension methods are
documented by concern.

| Chapter | Public types / module |
|---|---|
| [Runtime and Context](runtime-context.md) | `Runtime`, `Context`, `RuntimeId`, `FiberId`, `PublicationId`, `IsolationId` |
| [Plugins and Registry](plugins-registry.md) | `Plugin`, `FiberHandle`, `FiberState`, `LifecycleTask`, `UpdateOptions`, `Registry`, `LifecycleError` |
| [Services and reflection](services-reflection.md) | `ServiceKey`, `Dependency`, `ServiceRegistration`, `BoundService`, `ServiceInvoke`, `AccessorKey`, `Accessor`, `ServiceError`, `ReflectError` |
| [Effects and cleanup](effects.md) | `Disposer`, `Effect`, `EffectMeta`, `Completion`, `CleanupReport`, `CleanupFailure`, `LocalFuture`, `BoxError`, `SetupError`, `TaskControl`, `CallbackError` |
| [Events](events.md) | `Event`, `Listener`, `ListenerOptions`, `DispatchOptions`, `ListenerRegistration`, `Next`, `AsyncNext`, `EventTask`, `EventError` |
| [Logging](logging.md) | `Logger`, `LoggerConfig`, `Level`, `LogRecord`, `WeakFiber`, `ErrorValue`, `ColorDepth`, `ExporterOptions`, `ExporterRegistration`, `LogError` |
| [Errors and hooks](errors-hooks.md) | `HostError`, `AdmissionError`, `CompletionError`, `ValidationError`, `ValidationIssue`, `PathSegment`, public `hooks` module |
| [Timer satellite](timer.md) | `TimerExt`, `Sleep`, `IntervalStream`, `IntervalControl`, `Scheduled`, `TimerError` |
| [Console satellite](console-logger.md) | `ColorMode`, `Alignment`, `LabelStyle`, `Timestamp`, `ConsoleOptions`, `render`, `register`, `register_stdout` |
| [Loader](loader.md) | `Catalog`, `Config`, `EntrySpec`, `EntryKind`, `EntryId`, `FactoryId`, `FactoryReplacement`, `Loader`, `LoaderTask`, `LoaderError`, `EntrySnapshot`, `EntryState`, `EntryFailure`, `Settlement`, `Scope`, `Origin`, `CommitRecord`, `SourceId`, `SourceDocument`, `SourceLocation`, `SourceSpan`, `FieldPatch`, `EntryPatch`, `MAX_ENTRIES`, `MAX_JSON_BYTES`, `MAX_JSON_DEPTH`, `MAX_JSON_NODES`, `MAX_STRING_BYTES` |
| [Groups](groups.md) | `Group` |
| [Include](include.md) | `Include`, `IncludeOptions`, `IncludeTask`, `Report`, `JsonEdit`, `Conflict`, `IncludeError`, `Location`, `Owner`, `OwnerKind`, `Ownership`, `ParsedDocument`, `parse_document`, `MAX_BYTES`, `MAX_DEPTH`, `MAX_NODES`, `MAX_STRING_BYTES`, `MAX_FILES`, `MAX_ENTRIES`, `MAX_PATCHES`, `MAX_RETRIES` |
| [HMR](hmr.md) | `Change`, `ChangeKind`, `FactoryReplacement`, `FileWatcher`, `ReloadNode`, `ReloadPlan`, `SyntheticDelivery`, `SyntheticError`, `SyntheticEvents`, `WatchBackend`, `WatchCapabilities`, `WatchError`, `WatchEvent`, `WatchFailure`, `WatchFailureStage`, `WatchId`, `WatchOptions`, `WatchStats`, `WatchToken` |
| [Create](create.md) | `create`, `CreateError` |

## Read method results in two stages

An operation such as `fiber.update(source)?` can fail immediately before
admission; its returned task can fail later while awaited. `Completion`,
`LifecycleTask`, and `EventTask` are wait-only handles, not commands that start
on first poll. Keep the host alive and driven through their completion.

`Effect::ready` is an async setup waiter, not a cleanup command.
Logging and pure reads have different admission requirements from registering
new work; see the relevant chapter rather than assuming every method needs
`run` or every method works outside it.

## Stability and compatibility

R1 reviews this current API surface without adding a semver promise. The
[release proposal](../../release-readiness.md) describes a possible coordinated
0.1.0-alpha.1; all current crates remain 0.0.0/publish=false. Public fields,
trait implementations and dependency re-exports also affect compatibility.


All APIs are currently unstable. Many errors are `#[non_exhaustive]`; downstream
matches must allow future variants. IDs are opaque and comparable/hashable;
there is no numeric serialization or ordering contract.

The semantic reference is pinned TypeScript Cordis, with explicit Rust
representations and safety deviations. Historical milestone documents are
evidence, not a substitute for checking current Rust signatures.
