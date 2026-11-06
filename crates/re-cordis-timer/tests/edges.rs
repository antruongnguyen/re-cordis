#![allow(missing_docs)]

use futures_util::{FutureExt, StreamExt};
use re_cordis_core::{AdmissionError, BoxError, Runtime};
use re_cordis_timer::{Scheduled, TimerError, TimerExt};
use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
    time::Duration,
};

include!("common/mod.rs");

#[test]
fn disabled_time_driver_returns_construction_errors_without_detached_work() -> Result<(), BoxError>
{
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                assert!(matches!(
                    root.timeout(Duration::ZERO, || {}),
                    Err(TimerError::TimeDriverUnavailable)
                ));
                assert!(matches!(
                    root.sleep(Duration::ZERO).now_or_never(),
                    Some(Err(TimerError::TimeDriverUnavailable))
                ));
                assert!(matches!(
                    root.interval(Duration::from_millis(1), || {}),
                    Err(TimerError::TimeDriverUnavailable)
                ));
                assert!(matches!(
                    root.interval_stream(Duration::from_millis(1)),
                    Err(TimerError::TimeDriverUnavailable)
                ));
                assert!(matches!(
                    root.throttle(Duration::ZERO, false, || {}),
                    Err(TimerError::TimeDriverUnavailable)
                ));
                assert!(matches!(
                    root.debounce(Duration::ZERO, || {}),
                    Err(TimerError::TimeDriverUnavailable)
                ));
                assert!(root.get_effects()?.is_empty());
                tokio::task::yield_now().await;
                assert!(
                    runtime.take_failures().is_empty(),
                    "construction refusal is not a detached callback diagnostic"
                );
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok(())
    })
}

#[test]
fn zero_one_shots_are_async_and_zero_schedule_reentry_is_bounded() -> Result<(), BoxError> {
    block(|root| async move {
        let timeout_count = Rc::new(Cell::new(0));
        let output = timeout_count.clone();
        root.timeout(Duration::ZERO, move || output.set(1))?;
        let mut sleep = Box::pin(root.sleep(Duration::ZERO));
        assert!(sleep.as_mut().now_or_never().is_none());
        assert_eq!(timeout_count.get(), 0);

        for debounce in [false, true] {
            let count = Rc::new(Cell::new(0));
            let slot = Rc::new(RefCell::new(Weak::<Scheduled>::new()));
            let output = count.clone();
            let callback_slot = slot.clone();
            let callback = move || {
                output.set(output.get() + 1);
                callback_slot.borrow().upgrade().unwrap().call().unwrap();
            };
            let schedule = Rc::new(if debounce {
                root.debounce(Duration::ZERO, callback)?
            } else {
                root.throttle(Duration::ZERO, false, callback)?
            });
            *slot.borrow_mut() = Rc::downgrade(&schedule);
            schedule.call()?;
            assert_eq!(count.get(), if debounce { 0 } else { 1 });
            tokio::task::yield_now().await;
            tokio::task::yield_now().await;
            assert_eq!(count.get(), 1, "recursive zero call is ignored");
            schedule.dispose()?;
        }
        assert_eq!(timeout_count.get(), 1);
        assert!(matches!(sleep.as_mut().now_or_never(), Some(Ok(()))));
        Ok(())
    })
}

#[test]
fn positive_recursive_calls_coalesce_without_mutable_borrow_conflicts() -> Result<(), BoxError> {
    block(|root| async move {
        for debounce in [false, true] {
            let count = Rc::new(Cell::new(0));
            let slot = Rc::new(RefCell::new(Weak::<Scheduled>::new()));
            let output = count.clone();
            let callback_slot = slot.clone();
            let callback = move || {
                output.set(output.get() + 1);
                if output.get() == 1 {
                    callback_slot.borrow().upgrade().unwrap().call().unwrap();
                }
            };
            let schedule = Rc::new(if debounce {
                root.debounce(Duration::from_millis(10), callback)?
            } else {
                root.throttle(Duration::from_millis(10), false, callback)?
            });
            *slot.borrow_mut() = Rc::downgrade(&schedule);
            schedule.call()?;
            for _ in 0..2 {
                tokio::time::advance(Duration::from_millis(10)).await;
                tokio::task::yield_now().await;
            }
            assert_eq!(count.get(), 2);
            schedule.dispose()?;
        }
        Ok(())
    })
}

#[test]
fn scheduled_calls_from_raw_tasks_are_refused_before_firing() -> Result<(), BoxError> {
    block(|root| async move {
        let fired = Rc::new(Cell::new(false));
        let output = fired.clone();
        let scheduled = root.throttle(Duration::ZERO, false, move || output.set(true))?;
        let task = tokio::task::spawn_local(async move { scheduled.call() });
        assert!(matches!(
            task.await?,
            Err(TimerError::Admission(AdmissionError::HostNotRunning))
        ));
        assert!(!fired.get());
        Ok(())
    })
}

#[test]
fn stream_first_terminal_wins_and_drop_stops_before_next_period() -> Result<(), BoxError> {
    block(|root| async move {
        let baseline = root.get_effects()?.len();
        for fail_first in [false, true] {
            let mut stream = root.interval_stream(Duration::from_secs(60))?;
            let control = stream.control();
            if fail_first {
                control.fail(TimerError::InvalidDelay);
                control.stop();
            } else {
                control.stop();
                control.fail(TimerError::InvalidDelay);
            }
            if fail_first {
                assert!(matches!(
                    stream.next().now_or_never(),
                    Some(Some(Err(TimerError::InvalidDelay)))
                ));
            }
            assert!(matches!(stream.next().now_or_never(), Some(None)));
            assert!(matches!(stream.next().now_or_never(), Some(None)));
            drop(stream);
            tokio::task::yield_now().await;
            tokio::task::yield_now().await;
            assert_eq!(root.get_effects()?.len(), baseline);
        }
        let stream = root.interval_stream(Duration::from_secs(60))?;
        let control = stream.control();
        drop(stream);
        control.fail(TimerError::InvalidDelay);
        tokio::task::yield_now().await;
        tokio::task::yield_now().await;
        assert_eq!(
            root.get_effects()?.len(),
            baseline,
            "drop cancels without waiting for a tick"
        );
        Ok(())
    })
}

#[test]
fn stream_owner_and_control_terminal_order_is_once_only() -> Result<(), BoxError> {
    block(|root| async move {
        for control_first in [false, true] {
            let slot = Rc::new(RefCell::new(None));
            let output = slot.clone();
            let child = root.plugin(
                &re_cordis_core::Plugin::new("terminal-order", move |ctx, ()| {
                    let output = output.clone();
                    async move {
                        *output.borrow_mut() = Some(ctx.interval_stream(Duration::from_secs(60))?);
                        Ok::<_, BoxError>(())
                    }
                }),
                (),
            )?;
            child.ready()?.await?;
            let mut stream = slot.borrow_mut().take().unwrap();
            let control = stream.control();
            if control_first {
                control.fail(TimerError::InvalidDelay);
            }
            child.dispose()?.await?;
            control.fail(TimerError::InvalidDelay);
            let item = stream.next().now_or_never();
            if control_first {
                assert!(matches!(item, Some(Some(Err(TimerError::InvalidDelay)))));
            } else {
                assert!(matches!(item, Some(Some(Err(TimerError::Disposed)))));
            }
            assert!(matches!(stream.next().now_or_never(), Some(None)));
        }
        Ok(())
    })
}

#[test]
fn asynchronous_scheduled_callbacks_self_dispose_and_register_nested_work() -> Result<(), BoxError>
{
    block(|root| async move {
        for debounce in [false, true] {
            let count = Rc::new(Cell::new(0));
            let nested = Rc::new(Cell::new(0));
            let slot = Rc::new(RefCell::new(Weak::<Scheduled>::new()));
            let output = count.clone();
            let nested_output = nested.clone();
            let callback_slot = slot.clone();
            let ctx = root.clone();
            let callback = move || {
                output.set(output.get() + 1);
                if debounce || output.get() == 2 {
                    let sink = nested_output.clone();
                    ctx.timeout(Duration::ZERO, move || sink.set(sink.get() + 1))
                        .unwrap();
                    callback_slot.borrow().upgrade().unwrap().dispose().unwrap();
                }
            };
            let schedule = Rc::new(if debounce {
                root.debounce(Duration::from_millis(10), callback)?
            } else {
                root.throttle(Duration::from_millis(10), false, callback)?
            });
            *slot.borrow_mut() = Rc::downgrade(&schedule);
            schedule.call()?;
            if !debounce {
                schedule.call()?;
            }
            tokio::time::advance(Duration::from_millis(10)).await;
            for _ in 0..3 {
                tokio::task::yield_now().await;
            }
            assert_eq!(count.get(), if debounce { 1 } else { 2 });
            assert_eq!(nested.get(), 1);
            schedule.call()?;
            tokio::time::advance(Duration::from_millis(100)).await;
            tokio::task::yield_now().await;
            assert_eq!(nested.get(), 1);
        }
        Ok(())
    })
}

#[test]
fn ready_stream_polls_do_not_retain_consumer_wakers() -> Result<(), BoxError> {
    struct WakeProbe(std::sync::Arc<std::sync::atomic::AtomicUsize>);
    impl Drop for WakeProbe {
        fn drop(&mut self) {
            self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    impl futures_util::task::ArcWake for WakeProbe {
        fn wake_by_ref(_: &std::sync::Arc<Self>) {}
    }
    block(|root| async move {
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut stream = root.interval_stream(Duration::from_millis(1))?;
        let control = stream.control();
        tokio::time::advance(Duration::from_millis(1)).await;
        tokio::task::yield_now().await;
        for phase in 0..3 {
            if phase == 1 {
                control.fail(TimerError::InvalidDelay);
            }
            let probe = std::sync::Arc::new(WakeProbe(drops.clone()));
            let waker = futures_util::task::waker(probe);
            let mut context = std::task::Context::from_waker(&waker);
            let result =
                futures_util::Stream::poll_next(std::pin::Pin::new(&mut stream), &mut context);
            assert!(result.is_ready());
            drop(waker);
            assert_eq!(
                drops.load(std::sync::atomic::Ordering::SeqCst),
                phase + 1,
                "Ready tick/error/end must not retain caller waker"
            );
        }
        drop(stream);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 3);
        control.stop();
        Ok(())
    })
}

#[test]
fn small_late_interval_poll_coalesces_instead_of_bursting() -> Result<(), BoxError> {
    block(|root| async move {
        for period in [
            Duration::from_millis(1),
            Duration::from_micros(1),
            Duration::from_nanos(1),
        ] {
            let count = Rc::new(Cell::new(0));
            let output = count.clone();
            let interval = root.interval(period, move || output.set(output.get() + 1))?;
            tokio::time::advance(Duration::from_millis(4)).await;
            tokio::task::yield_now().await;
            assert_eq!(
                count.get(),
                1,
                "coalesce missed periods even below Tokio's 5ms threshold"
            );
            interval.dispose()?.await?;
        }
        Ok(())
    })
}
