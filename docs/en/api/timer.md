# Timer satellite

[Tiếng Việt](../../vi/api/timer.md) | [API index](README.md)

Source: [timer lib.rs](../../../crates/re-cordis-timer/src/lib.rs).
Guide: [timers](../guides/timers.md).
Runnable example: [timer_lifecycle.rs](../../../crates/re-cordis-timer/examples/timer_lifecycle.rs).

`re-cordis-timer` is unpublished (`0.0.0`, Rust 2024, MSRV 1.85).
Import sealed `TimerExt` on `Context`; no service installation is required.
Timer tasks use the original Cordis host through
[`Context::task`](effects.md#hosted-tasks-and-synchronous-callbacks).
Callbacks can register nested timers/effects. Raw Tokio tasks do not inherit
admission. Owner disposal joins timer destruction before completing.

## TimerExt

| Method | Result | Contract |
|---|---|---|
| `timeout` | `Result<Effect, TimerError>` | One callback; its exact registration retires before invocation |
| `sleep` | `Sleep` | Eager admission; await `Ok(())` or `Disposed` |
| `interval` | `Result<Effect, TimerError>` | First tick after one period; missed ticks skip without shifting phase |
| `interval_stream` | `Result<IntervalStream, TimerError>` | Single-consumer stream; at most one pending coalesced tick |
| `throttle` | `Result<Scheduled, TimerError>` | Immediate leading call, optional trailing call at the window boundary |
| `debounce` | `Result<Scheduled, TimerError>` | Calls replace the pending deadline; fire after the quiet window |

Callbacks are owned `FnOnce` (timeout) or `FnMut` (other callback forms), with
`'static` captures and no `Send` requirement. Construction checks host/owner
admission before touching Tokio time or spawning work. Refusal releases captures.

Deadlines are checked and pinned at successful construction; Sleep admission is
eager even if its first poll is delayed. Scheduled trailing deadlines are pinned
at each successful call. Zero timeout/sleep/debounce run asynchronously; zero
throttle fires its leading call immediately. Zero interval/stream periods and
unrepresentable deadlines return `InvalidDelay`. Positive sub-millisecond
periods are admitted, with Tokio's millisecond precision.

## Sleep

Future with output `Result<(), TimerError>`. Success retires its registration
before delivery; owner cancellation yields `Disposed`. Construction errors
are stored and returned on its first poll. Polling after completion panics.
Drop requests cancellation. Task destruction and retirement finish when the
original host is driven or the owner barrier is awaited.

## IntervalStream

Implements `futures_util::Stream<Item = Result<(), TimerError>>`.
It retains at most one coalesced tick, with no unbounded unread history.
Terminal state discards pending ticks and takes priority over tick delivery:

- `control().stop()`: clean `None`.
- `control().fail(reason)`: one final `Err(reason)`, then stable `None`.
- Owner cancellation: one final `Err(TimerError::Disposed)`, then stable `None`.

The first committed terminal gate wins; a lifecycle disposal request precedes
delivered owner cancellation. Drop acts as stop. Producer cancellation does
not wait for another tick; joined destruction requires original host progress.
The JS concurrent-reader return-value/throw protocol is omitted; see
[limitations](../../limitation.md#timer-satellite).

## IntervalControl

Cloneable `stop()`/`fail(reason)` authority obtained from `control()`.
Terminal requests are safe outside the host and never admit new work.

## Scheduled

Owned throttle/debounce handle.

| Method | Contract |
|---|---|
| `call() -> Result<(), TimerError>` | Active calls require the original live host; disposed calls are inert |
| `dispose() -> Result<(), TimerError>` | Capability cancellation, safe outside the host; later calls are inert |

Callbacks run outside mutable framework borrows. Self-disposal, disposal of
another operation, and nested timer registration are supported. Recursive
positive-delay calls coalesce through the ordinary trailing window; recursive
zero-delay calls are ignored to bound recursion and zero-delay loops.
A panic stops the affected operation. Leading throttle returns its retained
callback failure; asynchronous callback failures reach core diagnostics once.
Sibling operations and the host remain usable.

Drop requests cancellation and releases idle callback captures synchronously;
an executing callback releases after returning. The owner barrier joins task
destruction and effect retirement. This does not change core Effect's inert Drop.
A separately claimed manual Effect completion remains its own waiter.

## TimerError

- `Disposed`: owner cancellation before delivery.
- `InvalidDelay`: zero repeat period or unrepresentable monotonic deadline.
- `TimeDriverUnavailable`: the executor has no enabled Tokio time driver.
- `Admission(AdmissionError)`: wrong/non-running host, stale owner, or other refusal.
- `Callback(CleanupFailure)`: original reported scheduled callback panic.

Admission/callback variants preserve their error source. Disabled-driver detection
catches Tokio's synchronous construction panic; its normal panic hook still
runs. No detached timer task or retained core callback diagnostic is created.

## Clock and testing

Use monotonic `tokio::time`, rather than TS wall-clock `Date.now()`.
Constructors arm deadlines before returning; a yield before advancing time is
no longer needed to set the deadline. After advancing a paused clock, yield to
drive task delivery/cancellation before asserting its outcome.
Self/ancestor lifecycle waits from callbacks/tasks are refused by the core guard.

## Example

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
                let debounced = root.debounce(Duration::ZERO, || println!("quiet"))?;
                debounced.call()?;
                root.sleep(Duration::from_millis(1)).await?;
                debounced.dispose()?;
                root.drain()?.await?;
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}
```
