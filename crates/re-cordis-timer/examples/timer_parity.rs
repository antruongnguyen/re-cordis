//! Four deterministic timer traces paired with the pinned TypeScript fixture.
//! Run with `python3 -B tests/timer_parity/run.py`.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;

fn assert_trace(name: &str, expected: &str, trace: &[String]) -> Result<(), BoxError> {
    let actual = trace.join("\n") + "\n";
    assert_eq!(actual, expected, "{name} timer observations");
    if let Some(output) = std::env::var_os("CORDIS_TIMER_PARITY_OUTPUT") {
        std::fs::write(
            std::path::Path::new(&output).join(format!("{name}.txt")),
            actual,
        )?;
    }
    Ok(())
}

async fn advance(milliseconds: u64) {
    tokio::time::advance(Duration::from_millis(milliseconds)).await;
    tokio::task::yield_now().await;
}

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                let count = Rc::new(Cell::new(0));
                let sink = count.clone();
                root.timeout(Duration::from_millis(10), move || sink.set(sink.get() + 1))?;
                let mut trace = vec![format!("scheduled count={}", count.get())];
                // Poll once before advancing, matching the ordinary TS registration trace.
                // Admission-time anchoring is covered by Rust unit tests, not this fixture.
                tokio::task::yield_now().await;
                advance(5).await;
                trace.push(format!("advance 5 count={}", count.get()));
                advance(5).await;
                trace.push(format!("advance 10 count={}", count.get()));
                advance(10).await;
                trace.push(format!("advance 20 count={}", count.get()));
                assert_trace(
                    "timeout",
                    include_str!("../../../tests/timer_parity/timeout.txt"),
                    &trace,
                )?;

                let count = Rc::new(Cell::new(0));
                let sink = count.clone();
                let timer =
                    root.timeout(Duration::from_millis(10), move || sink.set(sink.get() + 1))?;
                let mut trace = vec![format!("scheduled count={}", count.get())];
                timer.dispose()?.await?;
                trace.push("cancelled".into());
                tokio::task::yield_now().await;
                advance(20).await;
                trace.push(format!("advance 20 count={}", count.get()));
                assert_trace(
                    "cancel",
                    include_str!("../../../tests/timer_parity/cancel.txt"),
                    &trace,
                )?;

                let sleep = tokio::task::spawn_local(root.sleep(Duration::from_millis(10)));
                let mut trace = vec!["scheduled pending".into()];
                tokio::task::yield_now().await;
                advance(5).await;
                trace.push(format!(
                    "advance 5 {}",
                    if sleep.is_finished() {
                        "resolved"
                    } else {
                        "pending"
                    }
                ));
                advance(5).await;
                trace.push(format!(
                    "advance 10 {}",
                    if sleep.is_finished() {
                        "resolved"
                    } else {
                        "pending"
                    }
                ));
                sleep.await??;
                trace.push("await resolved".into());
                assert_trace(
                    "sleep",
                    include_str!("../../../tests/timer_parity/sleep.txt"),
                    &trace,
                )?;

                let count = Rc::new(Cell::new(0));
                let sink = count.clone();
                let timer =
                    root.interval(Duration::from_millis(10), move || sink.set(sink.get() + 1))?;
                let mut trace = vec![format!("scheduled count={}", count.get())];
                tokio::task::yield_now().await;
                advance(10).await;
                trace.push(format!("advance 10 count={}", count.get()));
                advance(10).await;
                trace.push(format!("advance 20 count={}", count.get()));
                timer.dispose()?.await?;
                trace.push("stopped".into());
                advance(20).await;
                trace.push(format!("advance 40 count={}", count.get()));
                assert_trace(
                    "interval",
                    include_str!("../../../tests/timer_parity/interval.txt"),
                    &trace,
                )?;

                assert!(root.drain()?.await?.failures().is_empty());
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok(())
    })
}
