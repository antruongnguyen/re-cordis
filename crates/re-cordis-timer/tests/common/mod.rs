// Shared test harness: a current-thread, time-paused runtime that drives the
// re-cordis `Runtime` host. Included by each test file via `include!`.
//
// Timing discipline (see the timer design notes): a timer task must be polled
// once — via `tokio::task::yield_now().await` — AFTER it is created so its
// `sleep` registers on the paused timer driver, and only THEN may the test
// `advance` the clock. The pattern is: create op → yield → advance → yield →
// assert.

use re_cordis_core::{BoxError, Runtime};

#[allow(dead_code)]
fn block<F, Fut>(body: F) -> Result<(), BoxError>
where
    F: FnOnce(re_cordis_core::Context) -> Fut,
    Fut: std::future::Future<Output = Result<(), BoxError>>,
{
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                body(root).await
            })
            .await??;
        Ok(())
    })
}
