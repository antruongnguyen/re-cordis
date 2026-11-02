#![allow(missing_docs)]

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;
use std::time::Duration;

#[test]
fn every_timer_callback_panic_is_reported_once_and_host_remains_usable() -> Result<(), BoxError> {
    for kind in 0..5 {
        let executor = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()?;
        executor.block_on(async {
            let runtime = Runtime::new();
            runtime
                .run(async {
                    let root = runtime.context();
                    let mut keep_scheduled = None;
                    match kind {
                        0 => {
                            root.timeout(Duration::from_millis(10), || panic!("timeout failure"))?;
                        }
                        1 => {
                            root.interval(Duration::from_millis(10), || {
                                panic!("interval failure")
                            })?;
                        }
                        2 => {
                            let scheduled =
                                root.throttle(Duration::from_millis(10), false, || {
                                    panic!("leading failure")
                                })?;
                            let result =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    scheduled.call()
                                }));
                            assert!(result.is_ok(), "leading callback panic is contained");
                            keep_scheduled = Some(scheduled);
                        }
                        3 => {
                            let mut calls = 0;
                            let scheduled =
                                root.throttle(Duration::from_millis(10), false, move || {
                                    calls += 1;
                                    if calls == 2 {
                                        panic!("trailing failure");
                                    }
                                })?;
                            scheduled.call()?;
                            scheduled.call()?;
                            keep_scheduled = Some(scheduled);
                        }
                        _ => {
                            let scheduled = root.debounce(Duration::from_millis(10), || {
                                panic!("debounce failure")
                            })?;
                            scheduled.call()?;
                            keep_scheduled = Some(scheduled);
                        }
                    }
                    tokio::task::yield_now().await;
                    tokio::time::advance(Duration::from_millis(10)).await;
                    tokio::task::yield_now().await;
                    tokio::task::yield_now().await;
                    let failures = runtime.take_failures();
                    assert_eq!(failures.len(), 1, "one diagnostic for callback kind {kind}");
                    assert!(failures[0].is_panic());
                    assert!(failures[0].to_string().contains("failure"));
                    tokio::time::advance(Duration::from_millis(100)).await;
                    tokio::task::yield_now().await;
                    assert!(runtime.take_failures().is_empty());
                    assert!(
                        root.get_effects()?.is_empty(),
                        "failed operation unregisters"
                    );
                    root.on_dispose_sync(|| Ok::<_, BoxError>(()))?
                        .dispose()?
                        .await?;
                    drop(keep_scheduled);
                    Ok::<_, BoxError>(())
                })
                .await??;
            Ok::<_, BoxError>(())
        })?;
    }
    Ok(())
}
