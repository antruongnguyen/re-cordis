//! All six timer operations follow plugin configuration and owner disposal.
//! Run with cargo run -p re-cordis-timer --example timer_lifecycle --locked.

use futures_util::StreamExt;
use re_cordis_core::{BoxError, Context, Plugin, Runtime};
use re_cordis_timer::TimerExt;
use std::{cell::RefCell, rc::Rc, time::Duration};

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
                let events = Rc::new(RefCell::new(Vec::<String>::new()));
                let schedules = Rc::new(RefCell::new(None));
                let output = events.clone();
                let handles = schedules.clone();
                let worker = Plugin::new("timer-worker", move |ctx: Context, milliseconds: u64| {
                    let events = output.clone();
                    let handles = handles.clone();
                    async move {
                        let period = Duration::from_millis(milliseconds);
                        let log = events.clone();
                        ctx.timeout(period, move || log.borrow_mut().push("timeout".into()))?;
                        let log = events.clone();
                        ctx.interval(period, move || log.borrow_mut().push("interval".into()))?;

                        let wait = ctx.sleep(period);
                        let log = events.clone();
                        ctx.task(async move {
                            let result = wait.await;
                            move || log.borrow_mut().push(format!("sleep: {result:?}"))
                        })?;

                        let mut stream = ctx.interval_stream(period)?;
                        let log = events.clone();
                        ctx.task(async move {
                            let tick = stream.next().await;
                            move || log.borrow_mut().push(format!("stream: {tick:?}"))
                        })?;

                        let log = events.clone();
                        let throttle = ctx.throttle(period, false, move || {
                            log.borrow_mut().push("throttle".into())
                        })?;
                        throttle.call()?;
                        throttle.call()?;
                        let debounce = ctx.debounce(period, move || {
                            events.borrow_mut().push("debounce".into())
                        })?;
                        debounce.call()?;
                        *handles.borrow_mut() = Some((throttle, debounce));
                        Ok::<_, BoxError>(())
                    }
                });
                let child = root.plugin(&worker, 10)?;
                child.ready()?.await?;
                tokio::time::advance(Duration::from_millis(10)).await;
                tokio::task::yield_now().await;

                // Reconfigure through the ordinary plugin lifecycle. The old
                // generation's timers are joined before the new period is installed.
                child.update(20)?.await?;
                tokio::time::advance(Duration::from_millis(20)).await;
                tokio::task::yield_now().await;
                child.dispose()?.await?;

                let (throttle, debounce) = schedules.borrow_mut().take().unwrap();
                throttle.call()?;
                debounce.call()?;
                let count = events.borrow().len();
                tokio::time::advance(Duration::from_secs(1)).await;
                tokio::task::yield_now().await;
                assert_eq!(events.borrow().len(), count);
                assert!(root.get_effects()?.is_empty());
                assert!(root.registry().is_empty()?);
                println!("joined timer lifecycle: {}", events.borrow().join(", "));
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}

fn main() -> Result<(), BoxError> {
    run()
}
