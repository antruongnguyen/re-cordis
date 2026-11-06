// Shared test harness: a current-thread, time-paused runtime that drives the
// re-cordis `Runtime` host. Included by each test file via `include!`.
//
// Deadlines are armed at admission. Advance may precede the task's first poll;
// yield after advance to drive delivery, and after cancel to drive retirement.

// Fully-qualified to avoid clashing with per-test `use` imports of these
// names (each test file is a separate crate that `include!`s this file).
#[allow(dead_code)]
fn block<F, Fut>(body: F) -> Result<(), re_cordis_core::BoxError>
where
    F: FnOnce(re_cordis_core::Context) -> Fut,
    Fut: std::future::Future<Output = Result<(), re_cordis_core::BoxError>>,
{
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()?;
    executor.block_on(async {
        let runtime = re_cordis_core::Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                body(root).await
            })
            .await??;
        Ok(())
    })
}
