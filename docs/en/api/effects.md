# Effects and cleanup

[Tiếng Việt](../../vi/api/effects.md) | [API index](README.md)

Sources: [effect.rs](../../../crates/re-cordis-core/src/effect.rs),
[setup.rs](../../../crates/re-cordis-core/src/setup.rs).
Runnable guide: [effects and cleanup](../guides/effects-and-cleanup.md).

## Owned cleanup values

| Operation | Contract |
|---|---|
| `Disposer::new` | One-shot callback returning a future of `Result<(), E>` |
| `Disposer::sync` | One-shot synchronous `Result<(), E>` callback |
| `Disposer::compose` | Reverse sequential cleanup of children; stop the chain on failure |
| `Disposer::from_effect` | Transfer an unclaimed registration out of its journal |
| `Context::on_dispose` | Register already-acquired resource's async cleanup |
| `Context::on_dispose_sync` | Register explicitly synchronous cleanup |

Disposer and registration token Drop is inert. Yield/register acquired cleanup
values. Transferred effects have one owner/claim, not duplicated disposal.
From-effect errors include already claimed work and recursive self/ancestor
composition.

## Acquisition

| Context method | Setup shape |
|---|---|
| `effect` | `FnOnce() -> Result<Disposer, E>` |
| `effect_iter` | Iterator of `Result<Disposer, E>` |
| `effect_async` | Factory returning a future of `Result<Disposer, E>` |
| `effect_stream` | Factory returning a stream of disposer results |

Each method returns `Result<Effect, SetupError>`. Ownership is reserved before
setup runs. Async factories have an eager prefix; future bodies are driven
locally. Partial failures roll back acquired children. Disposal/invalidation
stops later stream advancement but retains the in-flight final acquisition and
joins its cleanup.

All-sync iterator rollback completes before return. Async rollback may continue
on the host; stream readiness exposes its completion explicitly.

## Effect and introspection

| Operation | Contract |
|---|---|
| `Effect::ready().await` | `Result<(), SetupError>` after setup/rollback; does not dispose |
| `with_label(label)` | Set introspection label without changing ownership |
| `dispose` | Claim this exact cleanup now; return `Result<Completion, AdmissionError>` |
| `Context::get_effects` | Return value-only `Vec<EffectMeta>` snapshots in registration order |

`EffectMeta` has public `label: String` and `children: Vec<EffectMeta>`.
Metadata does not own resources. Transferred registrations retain their labels
and become nested children; raw disposers do not invent effect nodes.

An all-sync manual disposal finishes immediately. An async cleanup's factory
prefix starts immediately, but its body remains async. If another operation
already claimed the registration, manual disposal reports zero attempts; that
does not join the other operation's work.

## Drain and completion

`Context::drain` claims the current journal snapshot and returns a Completion.
Independent callbacks progress concurrently after reverse-order invocation.
Requests during one drain share that pass; later registrations need a new pass.
Root remains reusable. Non-root lifecycle disposal also joins an existing owner
drain and later admitted resources before releasing providers.

`Completion` implements Future with output
`Result<CleanupReport, CompletionError>`. `CleanupReport::attempted` counts
claimed callbacks; `failures` borrows all retained callback failures.
`CleanupFailure::is_panic` distinguishes unwinds; `Error::source` retains the
original source. A successful outer result is not an error-free cleanup report.

Self/owning-ancestor waits are refused. Host destruction yields HostStopped,
not successful cleanup. Keep separately claimed manual completions when needed.

## Supporting types and errors

`BoxError` is `Box<dyn std::error::Error>`, without a Send/Sync bound.
`LocalFuture<T>` is a boxed `'static` local future, not an executor-independent
or thread-safe task.

`SetupError` distinguishes admission, callback/rollback failure, `AlreadyClaimed`,
`RecursiveComposition`, `HostStopped`, and `DriverPanicked`.
Immediate refusal never implies the supplied acquisition succeeded.
See [errors and hooks](errors-hooks.md).

## Hosted tasks and synchronous callbacks

Source: [task.rs](../../../crates/re-cordis-core/src/task.rs).

`Context::task(future)` returns `Result<(Effect, TaskControl), AdmissionError>`.
The local `'static` future returns its final synchronous `FnOnce()` callback.
Core polls it under the admitted host and owner, contains panics, retires its
exact registration, then invokes the callback without a suspension between
retirement and invocation. A transferred task still completes normally; a
completed transferred disposer becomes a no-op at later cleanup.

Cloneable `TaskControl::cancel()` is cancellation authority only. It marks and
requests abort safely outside Runtime::run, without admitting or invoking code.
The supervisor joins task destruction before retirement. Owner cleanup begins
cancellation synchronously and awaits that shared join. Core Effect/TaskControl
Drop is inert; timer operation wrappers explicitly request cancellation.
A separately claimed Effect completion retains the existing independent-waiter
contract. Keep the original host alive and driven through joined cleanup.

`Context::invoke_callback(callback)` checks the current host and live owner,
then invokes a synchronous callback with panic containment. It does not create
host authority for user-spawned Tokio tasks. `CallbackError` distinguishes
`Admission(AdmissionError)` from `Callback(CleanupFailure)`, preserves sources,
and reports the original panic exactly once through normal diagnostics.
Ordinary returned values, including nested Results, are returned unchanged.

Task future polling and terminal/synchronous callbacks use the existing
self/ancestor lifecycle guard. Waiting for the task's own owner lifecycle or
drain is refused; nested registrations remain supported. Cancelling a task
waiting for its own manually claimed Effect completion aborts the task so its
supervisor can finish.
