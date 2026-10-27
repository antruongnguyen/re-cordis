# Timers

[Tiếng Việt](../../vi/guides/timers.md) | [English home](../README.md)

The `re-cordis-timer` satellite adds owner-scoped timers over the core effect
system. Every operation is cancelled when its owning context is disposed, so
timers never outlive the component that created them. Reach the operations
through the sealed `TimerExt` trait — import it and call methods on any live
`Context`. No service installation is needed. See the
[timer API reference](../api/timer.md) for the full surface.

## One-shot and repeating callbacks

`timeout` fires once; `interval` fires every period (first tick at the period,
not zero). Both return an `Effect` — dispose it to cancel. Owner disposal
cancels them too. This complete program awaits a one-shot `sleep`:

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
                // Resolves at the deadline; Err(Disposed) if the owner is
                // disposed first.
                root.sleep(Duration::from_millis(1)).await?;
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}
```

For a repeating callback, `root.interval(period, || { ... })?` returns an
`Effect`; dispose it to stop. The first tick is at `period`, and missed ticks
coalesce.

## Awaiting a delay

`sleep` is the awaited form of `timeout`. It resolves `Ok(())` at the deadline,
or `Err(TimerError::Disposed)` if the owning context is disposed first — so a
component shutting down mid-wait observes the cancellation rather than hanging.

## Streaming ticks

When you want to pull ticks instead of pushing to a callback, `interval_stream`
returns a `futures_util::Stream` of `Result<(), TimerError>`. Its `control()`
handle ends the stream with `stop()` (clean) or `fail(reason)` (one final
error); owner disposal ends it with `Err(TimerError::Disposed)`. Unlike the
TypeScript async iterator, a stream has one logical consumer — for fan-out, use
the `interval` callback form.

## Rate limiting: throttle and debounce

`throttle` fires on the leading edge and, unless `no_trailing` is set, once more
at the end of the window. `debounce` fires once calls go quiet for the full
delay. Both return a `Scheduled` handle you invoke with `call()` and cancel with
`dispose()`. The windows are monotonic (`tokio::time`), not wall-clock.

## Testing timers deterministically

Build the runtime with `start_paused(true)` and advance the clock explicitly
with `tokio::time::advance`. Poll once with `tokio::task::yield_now().await`
after creating or calling an operation, so its task registers on the paused
timer driver before you advance — otherwise the advance moves past a deadline
the task has not yet armed. The example above follows this pattern.
