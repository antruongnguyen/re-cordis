#![allow(missing_docs)]

use re_cordis_core::{BoxError, Plugin};
use re_cordis_timer::TimerExt;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

include!("common/mod.rs");

struct Probe(Rc<Cell<usize>>);
impl Drop for Probe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn owner_barrier_releases_all_callback_captures() -> Result<(), BoxError> {
    block(|root| async move {
        let drops = Rc::new(Cell::new(0));
        let source = drops.clone();
        let holders = Rc::new(RefCell::new(Vec::new()));
        let output = holders.clone();
        let child = root.plugin(
            &Plugin::new("timer-captures", move |ctx: re_cordis_core::Context, ()| {
                let drops = source.clone();
                let output = output.clone();
                async move {
                    let a = Probe(drops.clone());
                    ctx.timeout(Duration::from_secs(60), move || {
                        let _capture = &a;
                    })?;
                    let b = Probe(drops.clone());
                    ctx.interval(Duration::from_secs(60), move || {
                        let _capture = &b;
                    })?;
                    let c = Probe(drops.clone());
                    let throttle = ctx.throttle(Duration::from_secs(60), false, move || {
                        let _capture = &c;
                    })?;
                    throttle.call()?;
                    throttle.call()?;
                    let d = Probe(drops.clone());
                    let debounce = ctx.debounce(Duration::from_secs(60), move || {
                        let _capture = &d;
                    })?;
                    debounce.call()?;
                    // Keep callable wrappers alive across owner disposal.
                    output.borrow_mut().extend([throttle, debounce]);
                    Ok::<_, BoxError>(())
                }
            }),
            (),
        )?;
        child.ready()?.await?;
        tokio::task::yield_now().await;
        assert_eq!(drops.get(), 0);
        child.dispose()?.await?;
        assert_eq!(
            drops.get(),
            4,
            "joined disposal must release captures held by both tasks and surviving handles"
        );
        assert!(root.get_effects()?.is_empty());
        assert!(root.registry().is_empty()?);
        Ok(())
    })
}

#[test]
fn one_hundred_completed_timer_cycles_restore_effect_baseline() -> Result<(), BoxError> {
    block(|root| async move {
        let baseline = root.get_effects()?.len();
        for _ in 0..100 {
            root.timeout(Duration::from_millis(1), || {})?;
            let sleep = root.sleep(Duration::from_millis(1));
            tokio::task::yield_now().await;
            tokio::time::advance(Duration::from_millis(1)).await;
            tokio::task::yield_now().await;
            sleep.await?;
            assert_eq!(root.get_effects()?.len(), baseline);
        }
        Ok(())
    })
}

#[test]
fn dropped_manual_completion_still_joins_cancelled_capture() -> Result<(), BoxError> {
    block(|root| async move {
        let drops = Rc::new(Cell::new(0));
        let probe = Probe(drops.clone());
        let effect = root.timeout(Duration::from_secs(60), move || {
            let _capture = &probe;
        })?;
        drop(effect.dispose()?);
        tokio::task::yield_now().await;
        tokio::task::yield_now().await;
        assert_eq!(drops.get(), 1);
        assert!(root.get_effects()?.is_empty());
        Ok(())
    })
}

#[test]
fn wrapper_drop_outside_host_is_cancelled_and_joined_by_original_owner() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()?;
    executor.block_on(async {
        let runtime = re_cordis_core::Runtime::new();
        let drops = Rc::new(Cell::new(0));
        let probe = Probe(drops.clone());
        let wrappers = runtime
            .run(async {
                let ctx = runtime.context();
                Ok::<_, BoxError>((
                    ctx.sleep(Duration::from_secs(60)),
                    ctx.interval_stream(Duration::from_secs(60))?,
                    ctx.debounce(Duration::from_secs(60), move || {
                        let _capture = &probe;
                    })?,
                ))
            })
            .await??;
        drop(wrappers);
        assert_eq!(
            drops.get(),
            1,
            "idle callback drops synchronously without host authority"
        );
        runtime
            .run(async {
                runtime.context().drain()?.await?;
                assert!(runtime.context().get_effects()?.is_empty());
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok(())
    })
}
