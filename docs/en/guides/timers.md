# Timers

[Tiếng Việt](../../vi/guides/timers.md) | [English home](../README.md)

Import `re_cordis_timer::TimerExt` and call methods on a live Context under
its original `Runtime::run`. The six operations use owned local callbacks;
no timer service installation or Send bound is required.

## One-shot and repeating work

`timeout(delay, callback)` returns an inert-drop Effect; use
`effect.dispose()?.await?` to cancel and join that exact manual cleanup.
A completed timeout retires its registration before invoking its callback.
`sleep(delay)` admits immediately and returns an awaited result. Its first
poll can be delayed without restarting the declared delay. Owner cancellation
returns `Disposed`.

`interval(period, callback)` first fires after one period; missed ticks skip
without shifting phase. Owner disposal joins callback capture release.
Callbacks can register nested Context effects/timers.

## Stream ticks and cancellation

`interval_stream(period)` is a single-consumer Stream with at most one pending
coalesced tick. `control().stop()` ends cleanly; `fail(reason)` emits one error
then stable None. Delivered owner cancellation emits Disposed once. The first
committed terminal gate wins and discards pending ticks; the earlier lifecycle
request has not itself committed that gate.

Drop of Sleep, IntervalStream and Scheduled requests cancellation. Their
task destruction and effect retirement need original host progress; await the
owner disposal/drain barrier to join owner cancellation. Core Effect Drop stays
inert. Keep separately claimed manual completion waiters when their barrier matters.

## Throttle and debounce

`throttle(delay, no_trailing, callback)` fires immediately on the leading edge;
suppressed calls optionally coalesce at the fixed window boundary.
`debounce(delay, callback)` replaces the trailing deadline on each call.
Use `scheduled.call()?` and `scheduled.dispose()?`. Active calls validate their
original live host. Disposed calls are inert; cancellation is safe outside the host.

Self-disposal and positive-delay recursive calls are supported without borrowing
framework mutable state during user code. Recursive zero-delay calls are ignored.
A callback panic stops its operation, reports once through core diagnostics, and
leaves siblings usable. Self/ancestor lifecycle waits are refused.

## Clock boundaries and deterministic checks

Deadline validation happens at admission. Zero timeout/sleep/debounce run on a
later host turn; zero throttle leading calls are immediate. Interval/stream zero
periods and unrepresentable deadlines are rejected. Positive sub-millisecond
periods use Tokio millisecond precision. Enable Tokio time on the executor;
disabled-driver construction returns TimeDriverUnavailable, with Tokio's normal
synchronous panic hook still visible and no detached task.

Use `start_paused(true)` and `tokio::time::advance` for deterministic tests.
Deadlines are already armed when construction returns. Yield after advance to
drive delivery; yield after cancellation to drive retirement.
See [timer API](../api/timer.md) and the all-six-operation
[plugin lifecycle example](../../../crates/re-cordis-timer/examples/timer_lifecycle.rs).

## Complete example

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
