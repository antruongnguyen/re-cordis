# Timer satellite

[Tiếng Việt](../../vi/api/timer.md) | [API index](README.md)

Source: [re-cordis-timer/src/lib.rs](../../../crates/re-cordis-timer/src/lib.rs).
Runnable guide: [timers](../guides/timers.md).
Runnable example:
[timer_lifecycle.rs](../../../crates/re-cordis-timer/examples/timer_lifecycle.rs).

`re-cordis-timer` is an unpublished satellite crate (`0.0.0`, Rust 2024, MSRV
1.85) providing owner-scoped timers over the existing core effect system. It
does not add a timing core: every operation schedules a `tokio::time` task on
the runtime's local task set and registers a `Context::on_dispose_sync`
cleanup, so disposing the owning context cancels the operation and no callback
runs afterward.

Operations are reached through the sealed [`TimerExt`](#timerext) trait, which
is implemented only for `Context`. No service registration or installation step
is required — importing the trait is enough.

## `TimerExt`

Sealed extension trait on `Context`. Import it to call the methods.

| Method | Result | Behavior |
|---|---|---|
| `timeout` | `Result<Effect, TimerError>` | Fire `callback` once after `delay`, then self-dispose. Dispose the `Effect` to cancel early. At most one call. |
| `sleep` | [`Sleep`](#sleep) | Awaited form: resolves `Ok(())` once at the deadline, or `Err(TimerError::Disposed)` if the owner is disposed first. |
| `interval` | `Result<Effect, TimerError>` | Fire `callback` every `delay`; first tick at `delay`. Dispose the `Effect` to stop. Missed ticks coalesce (skip). |
| `interval_stream` | `Result<IntervalStream, TimerError>` | A [`Stream`](#intervalstream) of ticks; see its termination contract. |
| `throttle` | `Result<Scheduled, TimerError>` | Leading-edge rate limit with an optional trailing call; monotonic window. |
| `debounce` | `Result<Scheduled, TimerError>` | Trailing-edge coalescing; each call restarts the window. |

The callback forms take `FnOnce` (`timeout`) or `FnMut` (`interval`,
`throttle`, `debounce`) with a `'static` bound. A zero `delay` is admitted and
fires on the next runtime turn.

## `Sleep`

Future returned by `TimerExt::sleep`. Yields `Result<(), TimerError>`: `Ok(())`
exactly once at the deadline, or `Err(TimerError::Disposed)` if the owning
context is disposed before the deadline. It holds its disposal registration
alive for its lifetime, so the cancellation wiring stays connected until the
race settles.

## `IntervalStream`

Implements `futures_util::Stream<Item = Result<(), TimerError>>`. Each tick
yields `Ok(())`. Termination:

- [`control().stop()`](#intervalcontrol) — ends the stream cleanly (`None`),
  no trailing item.
- `control().fail(reason)` — delivers one final `Err(reason)`, then `None`.
- Owner disposal — delivers one final `Err(TimerError::Disposed)`, then `None`.

A `Stream` has a single logical consumer. The TypeScript async iterator's
concurrent multi-reader `return(value)`/`throw(reason)` protocol is **not**
reproduced; use `interval` with a callback when you need fan-out. See
[limitations](../../limitation.md#timer-satellite).

## `IntervalControl`

Cloneable handle from `IntervalStream::control()` with `stop()` and
`fail(reason)`, as described above.

## `Scheduled`

Callable, cancellable handle from `throttle` and `debounce`.

| Method | Behavior |
|---|---|
| `call()` | Invoke the operation. Throttle fires on the leading edge and (unless `no_trailing`) once at the window's end; debounce fires once calls go quiet for the full delay. A disposed handle does nothing. |
| `dispose()` | Cancel: abort any pending trailing fire and ignore later calls. Owner disposal does the same. |

## `TimerError`

`#[derive(Debug)]` error with variants:

- `Disposed` — the owning context was disposed before the operation completed.
- `InvalidDelay` — the requested delay exceeds the supported range.
- `Admission(AdmissionError)` — the context could not admit the registration.

## Clock and testing

Timers use the monotonic `tokio::time` clock the local runtime drives, not
wall-clock time. This differs from the TypeScript `Date.now()`-based
throttle/debounce windows; see [decisions](../../decision.md#timer-satellite).
Drive deterministic tests with a paused runtime (`start_paused`) and
`tokio::time::advance`, polling once (`tokio::task::yield_now().await`) after
creating or calling an operation so its task registers before the clock moves.

## Example

A complete program that awaits a short real delay through `sleep`:

```rust
use std::time::Duration;

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                root.sleep(Duration::from_millis(1)).await?;
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}
```

For deterministic tests, use a paused runtime (`start_paused(true)`, which needs
tokio's `test-util` feature) and `tokio::time::advance`; see the
[timers guide](../guides/timers.md).
