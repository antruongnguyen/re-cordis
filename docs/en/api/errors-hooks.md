# Errors, validation, and internal hooks

[Tiếng Việt](../../vi/api/errors-hooks.md) | [API index](README.md)

Sources: [error.rs](../../../crates/re-cordis-core/src/error.rs),
[hooks.rs](../../../crates/re-cordis-core/src/hooks.rs).

## Error families

| Type | Meaning / principal variants |
|---|---|
| `HostError` | MissingExecutor, NestedHost, AlreadyRunning, SpawnedTask |
| `AdmissionError` | RuntimeDropped, HostNotRunning, WrongHost, ReentrantDrain, IdentityExhausted, InactiveContext |
| `CompletionError` | HostStopped, DriverPanicked |
| `LifecycleError` | Admission, Callback, Hook, HostStopped, DriverPanicked |
| `SetupError` | Admission, Callback, AlreadyClaimed, RecursiveComposition, HostStopped, DriverPanicked |
| `ServiceError` | Admission/Hook, DuplicateProvider, TypeMismatch, Undeclared, Inactive, NotProvided, NotOwner |
| `ReflectError` | Admission, Duplicate, Missing, Contract, ReadOnly, Callback |
| `EventError` | Admission, ContractMismatch, UnsupportedMode, Callbacks, HostStopped, DriverPanicked |
| `LogError` | RuntimeDropped, Formatting, Configuration, Exporters, SequenceExhausted |
| `CleanupFailure` | Preserved original callback error or caught unwind |

Most error enums above are non-exhaustive; check Rustdoc before exhaustive
matching. `ReflectError` is currently exhaustive, but the API remains unstable.
Use `Error::source` to inspect original errors rather than parsing Display.
For aggregate event/exporter errors, inspect all members, not just the first
source. `CleanupFailure::is_panic` distinguishes caught unwinds.

Immediate refusal and eventual callback failure are different stages.
HostStopped/DriverPanicked never mean successful cleanup. Admission failures do
not run setup; resources acquired before refused registration remain the
application's responsibility.

## Structured validation

`ValidationError::new(Vec<ValidationIssue>)` retains public `issues`.
Each `ValidationIssue` has `message: String` and `path: Vec<PathSegment>`.
`PathSegment::Key(String)` and `Index(usize)` preserve field/array locations;
an empty path means the whole configuration.

Return this error from a synchronous validator when structured issues are
useful. Any validator error convertible into BoxError is accepted.
Serde, async validation, and Standard Schema discovery are not automatic APIs.

## Typed hook payloads

`hooks::Value` wraps checked heterogeneous data. `new`, `from_rc`, `is::<T>`,
and `get::<T>` preserve/check the Rust contract; `get` returns `Rc<T>` or a
ServiceError. Use `is` to distinguish unrelated payload contracts without
treating every mismatch as operational failure.

| Hook function | Payload / public fields | Mode / purpose |
|---|---|---|
| `hooks::plugin()` | `FiberInfo { id, name, state, live }` | Admission/removal observation |
| `hooks::status()` | `Status { fiber, previous }` | State transition observation |
| `hooks::commit()` | `Commit { fiber, source }` | Source commit; suppressed by no_save |
| `hooks::config()` | `Config { fiber, source }` | Synchronous around-transform, result Value |
| `hooks::update()` | `Update { fiber, resolved }` | Async around-update, result bool |
| `hooks::service()` | `ServiceChange { name, slot, value }` | Exact-realm availability observation |
| `hooks::get()` | `Get { context, name }` | Synchronous around-get, result Option<Value> |
| `hooks::set()` | `Set { context, name, value }` | Synchronous around-set, result () |
| `hooks::listener()` | `ListenerRequest` | Synchronous responder; Break supplies replacement registration |
| `hooks::dispatch()` | `Dispatch { mode, event, arguments, context }` | Non-internal dispatch observation |

Observation hooks use synchronous responders returning Continue.
`FiberInfo::live` describes admission, not cleanup completion. Its fields are
value-only snapshots; retaining them does not retain a Fiber.
Source hook values and resolved update values represent different stages.
Their types `C` and `R` may differ, but can also be the same.

`ListenerRequest` exposes `context`, `event`, `listener`, and `options` methods.
`replace`/`replace_with` substitute an exact-contract typed Listener while
preserving original ownership/once behavior and avoiding recursive interception.
Recover typed Event/Listener values through checked `Value::get`.

Prefer `Context::on_update`/`on_update_sync` for ordinary per-Fiber update logic.
Local update middleware precedes ordinary global hooks; prepended global hooks
precede the local router. Hooks remain scoped registrations, while built-in
protocol implementations survive reusable root drains.

## Safety boundaries

Observer failures are diagnosed without stranding cleanup; transformation/update
middleware errors still reach the operation's caller. Framework-invariant
panics, aborts, panicking destructors, and arbitrary user task/Rc cycles are
not automatically repaired.

Do not treat hook payloads as mutable runtime records or use direct access to
bypass dependency-lifetime requirements. See
[compatibility restrictions](../../m6.md#remaining-restrictions-and-safety-deviations).
