# Events and listeners

[Tiếng Việt](../../vi/api/events.md) | [API index](README.md)

Source: [events.rs](../../../crates/re-cordis-core/src/events.rs).
Runnable guide: [events](../guides/events.md).

## Contracts and registration

| API | Contract |
|---|---|
| `Event::<A, R>::named(name)` | Shared string identity with checked live argument/result types |
| `Event::symbol(description)` | Fresh opaque identity; clone to share |
| `Listener::sync` | `(Context, A) -> Result<ControlFlow<R>, E>` |
| `Listener::asynchronous` | Factory receiving `(Context, A)` returning an async responder |
| `Listener::around` | `(Context, A, Next<R>) -> Result<R, E>` |
| `Listener::around_async` | Factory receiving `(Context, A, AsyncNext<R>)` returning an async result |
| `Context::on` | Register using default options |
| `Context::on_with` | Register with explicit `ListenerOptions` |
| `Context::once` | Register removal-before-invocation behavior |

`R` defaults to `()`. Dispatch requires cloneable arguments; listener contracts
and futures are owned/`'static`. Registration callbacks receive their selected
registration Context, not an implicit JavaScript `this`.

`ListenerOptions` has public booleans `prepend` and `global`, both false by
default. `DispatchOptions::default` selects unfiltered listeners;
`DispatchOptions::filtered` tests registration Contexts. Global bypasses filters.

`ListenerRegistration::remove` synchronously claims this registration and
returns `Result<bool, AdmissionError>`. `into_effect` transfers scoped removal
into composition. Dropping the token does not unregister.

## Dispatch results

All methods take `(&event, args, DispatchOptions)`; waterfall forms also take an
owned terminal callback with no arguments.

| Context method | Successful outer result | Allowed listener |
|---|---|---|
| `emit` | `EventTask<()>` | sync / asynchronous |
| `parallel` | `EventTask<()>` | sync / asynchronous |
| `serial` | `EventTask<ControlFlow<R>>` | sync / asynchronous |
| `bail` | `ControlFlow<R>` | sync |
| `waterfall` | `R` | around |
| `waterfall_async` | `EventTask<R>` | around_async |

Outer errors are `EventError`. `EventTask<T>` awaits
`Result<T, EventError>` from started work.

Emit runs sync callbacks/factories immediately, ignoring Break answers and
stopping on synchronous errors. Previously started continuations survive a
later immediate failure. Parallel invokes all and joins all outcomes; failures
remain in snapshot order. Serial/bail stop at first Break, including zero/empty
values. Waterfall is around-middleware, not reduction.

`Next::call(self)` returns `Result<R, EventError>`.
`AsyncNext::call(self)` returns a local future of that result. Both consume
themselves, so duplicate next calls are a type error. An async continuation
future still needs to be awaited; invoking the adapter does not make arbitrary
native future bodies eager.

## Snapshot and error behavior

The registered-listener snapshot is captured before dispatch filters run.
Filters select from that snapshot before the selected event callbacks run.
Removing a selected callback does not revoke the in-flight snapshot. Once
removes before invocation but does not add a second occurrence-claim gate across
already selected snapshots. Final listener removal releases its named event's
live type bucket.

Unsupported listener/mode combinations fail before invoking the snapshot.
Emit async failures are always diagnosed. Serial/parallel/async-waterfall
callback errors are returned to retained waiters, or diagnosed on abandoned
waiters. Dropping EventTask does not cancel started workers.

`EventError` distinguishes admission, contract mismatch, unsupported mode,
callback failures, HostStopped, and DriverPanicked.
`Callbacks` exposes event label and `Vec<CleanupFailure>`; its Error source is
the first failure, not a substitute for inspecting all aggregate members.

For framework-internal interception, `hooks::ListenerRequest::replace` and
`replace_with` preserve registration ownership without recursively invoking
the interception protocol. See [hooks](errors-hooks.md).
