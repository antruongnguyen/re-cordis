# API reference

[Tiếng Việt](../../vi/api/README.md) | [English home](../README.md)

This reference covers the public exports of `cordis-core`, not its private
storage or example-private handles. It describes usage and behavioral contracts;
generated Rustdoc is authoritative for exact signatures, generic bounds,
trait implementations, and public fields.

## Generate exact API documentation

From the repository root:

```sh
cargo doc -p cordis-core --no-deps --locked
```

Open `target/doc/cordis_core/index.html`, or the equivalent path under your
`CARGO_TARGET_DIR`. Nothing is published to docs.rs by this command.
An unpublished `0.0.0` package is not a supported crates.io installation target.

## Public export map

Every root export from [lib.rs](../../../crates/cordis-core/src/lib.rs) belongs
to one of these chapters. Context extension methods are documented by concern.

| Chapter | Public types / module |
|---|---|
| [Runtime and Context](runtime-context.md) | `Runtime`, `Context`, `RuntimeId`, `FiberId`, `PublicationId`, `IsolationId` |
| [Plugins and Registry](plugins-registry.md) | `Plugin`, `FiberHandle`, `FiberState`, `LifecycleTask`, `UpdateOptions`, `Registry`, `LifecycleError` |
| [Services and reflection](services-reflection.md) | `ServiceKey`, `Dependency`, `ServiceRegistration`, `BoundService`, `ServiceInvoke`, `AccessorKey`, `Accessor`, `ServiceError`, `ReflectError` |
| [Effects and cleanup](effects.md) | `Disposer`, `Effect`, `EffectMeta`, `Completion`, `CleanupReport`, `CleanupFailure`, `LocalFuture`, `BoxError`, `SetupError` |
| [Events](events.md) | `Event`, `Listener`, `ListenerOptions`, `DispatchOptions`, `ListenerRegistration`, `Next`, `AsyncNext`, `EventTask`, `EventError` |
| [Logging](logging.md) | `Logger`, `LoggerConfig`, `Level`, `LogRecord`, `WeakFiber`, `ErrorValue`, `ColorDepth`, `ExporterOptions`, `ExporterRegistration`, `LogError` |
| [Errors and hooks](errors-hooks.md) | `HostError`, `AdmissionError`, `CompletionError`, `ValidationError`, `ValidationIssue`, `PathSegment`, public `hooks` module |

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

All APIs are currently unstable. Many errors are `#[non_exhaustive]`; downstream
matches must allow future variants. IDs are opaque and comparable/hashable;
there is no numeric serialization or ordering contract.

The semantic reference is pinned TypeScript Cordis, with explicit Rust
representations and safety deviations. Historical milestone documents are
evidence, not a substitute for checking current Rust signatures.
