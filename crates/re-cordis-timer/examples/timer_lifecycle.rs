//! Owner-scoped timers over a deterministic (paused) clock.
//!
//! Shows `timeout`, `interval_stream`, `throttle`, and `debounce`, then
//! disposes the owning fiber and confirms no further callbacks fire. The clock
//! is paused and advanced explicitly so the run is fast and reproducible.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use futures_util::StreamExt;
use re_cordis_core::{BoxError, Plugin, Runtime};
use re_cordis_timer::TimerExt;

pub(crate) fn run() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                let log = Rc::new(Cell::new(0u32));

                // A one-shot timeout.
                let hit = log.clone();
                root.timeout(Duration::from_millis(100), move || {
                    hit.set(hit.get() + 1);
                    println!("timeout fired");
                })?;
                tokio::task::yield_now().await;
                tokio::time::advance(Duration::from_millis(100)).await;
                tokio::task::yield_now().await;

                // Three interval ticks via the Stream form.
                let stream = root.interval_stream(Duration::from_millis(50))?;
                let control = stream.control();
                tokio::task::spawn_local(async move {
                    futures_util::pin_mut!(stream);
                    let mut ticks = 0;
                    while let Some(Ok(())) = stream.next().await {
                        ticks += 1;
                        println!("interval tick {ticks}");
                        if ticks == 3 {
                            control.stop();
                        }
                    }
                });
                for _ in 0..3 {
                    tokio::task::yield_now().await;
                    tokio::time::advance(Duration::from_millis(50)).await;
                    tokio::task::yield_now().await;
                }

                // Everything scheduled on the root is cancelled when the owner
                // (the whole runtime, here) drains. Run one more scheduled op
                // under a child fiber and dispose it to show cancellation.
                let fired = Rc::new(Cell::new(false));
                let flag = fired.clone();
                let child = root.plugin(
                    &Plugin::new("worker", move |ctx: re_cordis_core::Context, (): ()| {
                        let flag = flag.clone();
                        async move {
                            ctx.timeout(Duration::from_millis(1000), move || {
                                flag.set(true);
                                println!("this must NOT print");
                            })?;
                            Ok::<_, BoxError>(())
                        }
                    }),
                    (),
                )?;
                child.ready()?.await?;
                tokio::task::yield_now().await;
                child.dispose()?.await?;
                tokio::time::advance(Duration::from_millis(2000)).await;
                tokio::task::yield_now().await;
                assert!(!fired.get(), "child timeout cancelled by owner disposal");

                println!(
                    "done: {} immediate callback(s), interval + child cancelled cleanly",
                    log.get()
                );
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}

fn main() -> Result<(), BoxError> {
    run()
}
