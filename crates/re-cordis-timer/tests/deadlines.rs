#![allow(missing_docs)]

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use futures_util::{FutureExt, StreamExt};
use re_cordis_core::BoxError;
use re_cordis_timer::{TimerError, TimerExt};

include!("common/mod.rs");

#[test]
fn unrepresentable_deadlines_are_rejected_for_all_six_methods() -> Result<(), BoxError> {
    block(|root| async move {
        assert!(matches!(
            root.timeout(Duration::MAX, || {}),
            Err(TimerError::InvalidDelay)
        ));
        assert!(matches!(
            root.sleep(Duration::MAX).now_or_never(),
            Some(Err(TimerError::InvalidDelay))
        ));
        assert!(matches!(
            root.interval(Duration::MAX, || {}),
            Err(TimerError::InvalidDelay)
        ));
        assert!(matches!(
            root.interval_stream(Duration::MAX),
            Err(TimerError::InvalidDelay)
        ));
        assert!(matches!(
            root.throttle(Duration::MAX, false, || {}),
            Err(TimerError::InvalidDelay)
        ));
        assert!(matches!(
            root.debounce(Duration::MAX, || {}),
            Err(TimerError::InvalidDelay)
        ));
        assert!(root.get_effects()?.is_empty());
        Ok(())
    })
}

#[test]
fn zero_repeat_periods_are_rejected_without_spawning() -> Result<(), BoxError> {
    block(|root| async move {
        assert!(matches!(
            root.interval(Duration::ZERO, || {}),
            Err(TimerError::InvalidDelay)
        ));
        assert!(matches!(
            root.interval_stream(Duration::ZERO),
            Err(TimerError::InvalidDelay)
        ));
        assert!(root.get_effects()?.is_empty());
        Ok(())
    })
}

#[test]
fn admitted_timeout_and_interval_do_not_restart_on_late_task_poll() -> Result<(), BoxError> {
    block(|root| async move {
        let timeout_count = Rc::new(Cell::new(0));
        let interval_count = Rc::new(Cell::new(0));
        let t = timeout_count.clone();
        let i = interval_count.clone();
        root.timeout(Duration::from_millis(10), move || t.set(t.get() + 1))?;
        let interval = root.interval(Duration::from_millis(10), move || i.set(i.get() + 1))?;
        tokio::time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        tokio::task::yield_now().await;
        assert_eq!(timeout_count.get(), 1, "timeout anchored at admission");
        assert_eq!(interval_count.get(), 1, "interval anchored at admission");
        interval.dispose()?.await?;
        Ok(())
    })
}

#[test]
fn sleep_is_pending_just_before_deadline_and_ready_at_it() -> Result<(), BoxError> {
    block(|root| async move {
        let mut sleep = Box::pin(root.sleep(Duration::from_millis(10)));
        tokio::time::advance(Duration::from_millis(9)).await;
        assert!(sleep.as_mut().now_or_never().is_none());
        tokio::time::advance(Duration::from_millis(1)).await;
        tokio::task::yield_now().await;
        assert!(matches!(sleep.as_mut().now_or_never(), Some(Ok(()))));
        Ok(())
    })
}

#[test]
fn late_stream_poll_coalesces_missed_ticks_without_shifting_phase() -> Result<(), BoxError> {
    block(|root| async move {
        let mut stream = root.interval_stream(Duration::from_millis(10))?;
        tokio::time::advance(Duration::from_millis(35)).await;
        tokio::task::yield_now().await;
        assert!(matches!(stream.next().now_or_never(), Some(Some(Ok(())))));
        assert!(stream.next().now_or_never().is_none());
        tokio::time::advance(Duration::from_millis(5)).await;
        tokio::task::yield_now().await;
        assert!(matches!(stream.next().now_or_never(), Some(Some(Ok(())))));
        stream.control().stop();
        Ok(())
    })
}
