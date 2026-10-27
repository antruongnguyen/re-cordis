# Events and around-middleware

[Tiếng Việt](../../vi/guides/events.md) | [English home](../README.md)

## Define a contract

`Event<A, R>` has owned argument type `A` and answer type `R` (default `()`).
Named events share identity by name and check live type contracts. Symbol events
allocate unique identity; cloning shares it, repeating the description does not.

```rust
use cordis_core::{BoxError, DispatchOptions, Event, Listener, Runtime};
use std::{convert::Infallible, ops::ControlFlow};

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                let query = Event::<u32, u32>::symbol("double");
                root.on(&query, Listener::sync(|_, value| {
                    Ok::<_, Infallible>(ControlFlow::Break(value * 2))
                }))?;
                assert_eq!(
                    root.serial(&query, 3, DispatchOptions::default())?.await?,
                    ControlFlow::Break(6),
                );
                let chain = Event::<(), u32>::symbol("around");
                root.on(&chain, Listener::around(|_, (), next| {
                    Ok::<_, BoxError>(next.call()? + 1)
                }))?;
                let answer = root.waterfall(&chain, (), DispatchOptions::default(), || {
                    Ok::<_, Infallible>(10_u32)
                })?;
                assert_eq!(answer, 11);
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

## Choose a mode and matching listener

| Mode | Listener shape | Result |
|---|---|---|
| `emit` | `sync`, `asynchronous` | Immediate invocation; wait handle for deferred outcomes |
| `parallel` | `sync`, `asynchronous` | All outcomes joined, all failures retained |
| `serial` | `sync`, `asynchronous` | Ordered first `ControlFlow::Break` |
| `bail` | `sync` only | Synchronous first Break |
| `waterfall` | `around` | Synchronous around-chain result |
| `waterfall_async` | `around_async` | Async around-chain result |

The last two are explicit Rust forms of Cordis's waterfall mode, not a
value-reduction loop. Middleware calls `next.call()` to continue, or returns
without calling it to veto the rest. Next is owned and one-use; calling twice
is a compile error. Sync and async around-listeners are not implicitly mixed.

Continue means no answer. `Break(0)` and `Break(String::new())` are still answers.
Emit/parallel do not use Break to short-circuit. A mode-incompatible snapshot is
rejected before callbacks run.

## Async timing

`Listener::asynchronous` invokes its factory during dispatch. Work outside the
returned async block is the eager prefix; work inside the block runs when
driven. Emit stops on synchronous responder failure, but previously started
async continuations remain scheduled.

Await returned event tasks when completion matters. Removing a listener or
draining its owner does not cancel a dispatch that already selected it.
Host destruction does cancel outstanding framework tasks.

## Selection and lifetime

`on`, `on_with`, and `once` register on the selected owner. `ListenerOptions`
controls prepend/global. `DispatchOptions::filtered` tests listener registration
Contexts; no automatic ancestor-only routing is implied. Global listeners bypass
filters. Use service-realm filters explicitly for service-originated events.

Once removes before invocation, protecting recursive dispatch. It does not
revoke callbacks already selected by another in-flight snapshot. Manual
`remove` synchronously removes one exact registration; dropping its token is
inert.

Emitted async failures are diagnosed even with retained waiters. For
serial/parallel/async-waterfall, callback errors belong to the awaiting caller,
or are diagnosed when their waiter is abandoned. Propagate errors rather than
discard results.

Reference: [events API](../api/events.md).
