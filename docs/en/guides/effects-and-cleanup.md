# Effects, resource ownership, and cleanup

[Tiếng Việt](../../vi/guides/effects-and-cleanup.md) | [English home](../README.md)

## Register cleanup with the owner

Use `on_dispose`/`on_dispose_sync` for an already acquired resource.
Use `effect`/`effect_async` when the framework should reserve ownership before
setup runs. A `Disposer` is a cleanup value: register or yield it; dropping it
does not execute cleanup.

```rust
use re_cordis_core::{BoxError, Disposer, Runtime};
use std::{cell::RefCell, convert::Infallible, rc::Rc};

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let order = Rc::new(RefCell::new(Vec::new()));
            let result = async {
                let first = order.clone();
                let second = order.clone();
                let resource = root.effect(move || {
                    Ok::<_, Infallible>(Disposer::compose(vec![
                        Disposer::sync(move || {
                            first.borrow_mut().push(1);
                            Ok::<_, Infallible>(())
                        }),
                        Disposer::sync(move || {
                            second.borrow_mut().push(2);
                            Ok::<_, Infallible>(())
                        }),
                    ]))
                })?;
                let report = resource.dispose()?.await?;
                if let Some(failure) = report.failures().first() {
                    return Err(failure.clone().into());
                }
                assert_eq!(*order.borrow(), [2, 1]);
                Ok::<_, BoxError>(())
            }.await;
            let cleanup = root.drain()?.await?;
            if let Some(failure) = cleanup.failures().first() {
                return Err(failure.clone().into());
            }
            result
        }).await?
    })
}
```

## Select the right acquisition API

| API | Acquisition shape |
|---|---|
| `effect` | Synchronous setup returning one disposer |
| `effect_iter` | Synchronous iterator of disposer results |
| `effect_async` | Factory returning a future producing one disposer |
| `effect_stream` | Factory producing a stream of disposer results |

Async setup starts on the host; await `Effect::ready` to observe setup and
failure rollback. Disposal does not drop an in-flight acquisition: it collects
the final acquired disposer and joins its cleanup before finishing. Invalidated
streams stop before advancing to the next acquisition.

An iterator's all-synchronous rollback completes immediately. If rollback must
await async cleanup, it continues on the host; use stream setup and its readiness
barrier when that rollback must be explicitly joined.

## Ordering is not one global LIFO stack

Independent top-level cleanup is invoked in reverse registration order and
progresses concurrently. Do not depend on sibling completion order.

`Disposer::compose` instead awaits children sequentially in reverse order.
A child failure stops that composed chain; independent owner cleanups are still
attempted. Use composition only when the dependency/order is intentional.

`Disposer::from_effect` transfers an unclaimed registration into composition.
Listeners, exporters, and service registrations expose `into_effect`.
Transfer removes the original journal claim, preventing duplicate cleanup.
Already-claimed and recursive self/ancestor transfers are explicit errors.

## Shutdown protocol

Stop your application's intake, join its owned workers, and await the relevant
Fiber disposal/root drain while the runtime is still driven. Framework effects
do not automatically own every user-spawned task: an effect can capture a task's
stop signal and await its join handle in its disposer.

`Completion` yields `Result<CleanupReport, CompletionError>`.
Inspect `report.failures()`; outer success means the barrier finished, not that
all callbacks succeeded. Lifecycle cleanup failures are diagnosed and retained;
inspect `Runtime::take_failures` for failures not returned as cleanup reports.

Root drain covers a reusable snapshot. Registrations after that snapshot need
another drain. A manual cleanup already claimed elsewhere has its own completion
handle; retain and await it. A zero-attempt repeat claim is not a join of another
operation's work.

Do not wait for your own or an owning ancestor's lifecycle from attributed
setup/cleanup. Such waits are refused to avoid deadlock. This does not detect
arbitrary user-created task cycles.

Reference: [effects API](../api/effects.md).
