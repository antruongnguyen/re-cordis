#![allow(missing_docs)]

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::time::Duration;

use futures_util::{FutureExt, StreamExt};
use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::{Scheduled, TimerError, TimerExt};

include!("common/mod.rs");

#[test]
fn completed_timeout_releases_registration_before_callback() -> Result<(), BoxError> {
    block(|root| async move {
        let baseline = root.get_effects()?.len();
        let observed = Rc::new(Cell::new(usize::MAX));
        let sink = observed.clone();
        let ctx = root.clone();
        root.timeout(Duration::from_millis(10), move || {
            sink.set(ctx.get_effects().unwrap().len());
        })?;
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert_eq!(observed.get(), baseline, "self-cleanup precedes callback");
        assert_eq!(root.get_effects()?.len(), baseline);
        Ok(())
    })
}

#[test]
fn sleep_deadline_is_pinned_at_admission_and_completion_unregisters() -> Result<(), BoxError> {
    block(|root| async move {
        let baseline = root.get_effects()?.len();
        let sleep = root.sleep(Duration::from_millis(10));
        assert_eq!(root.get_effects()?.len(), baseline + 1);
        tokio::time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert!(
            matches!(sleep.now_or_never(), Some(Ok(()))),
            "late first poll must not restart delay"
        );
        assert_eq!(root.get_effects()?.len(), baseline);
        Ok(())
    })
}

#[test]
fn timeout_callback_can_register_nested_context_work() -> Result<(), BoxError> {
    block(|root| async move {
        let admitted = Rc::new(Cell::new(false));
        let sink = admitted.clone();
        let ctx = root.clone();
        root.timeout(Duration::from_millis(10), move || {
            sink.set(ctx.on_dispose_sync(|| Ok::<_, BoxError>(())).is_ok());
        })?;
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert!(
            admitted.get(),
            "timer task must carry its Cordis HOST authority"
        );
        Ok(())
    })
}

#[test]
fn wrong_host_refusal_does_not_leave_callback_running() -> Result<(), BoxError> {
    block(|_| async move {
        let other = Runtime::new();
        let fired = Rc::new(Cell::new(false));
        let sink = fired.clone();
        assert!(matches!(
            other
                .context()
                .timeout(Duration::from_millis(10), move || sink.set(true)),
            Err(TimerError::Admission(_))
        ));
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert!(
            !fired.get(),
            "refused construction must own no running task"
        );
        Ok(())
    })
}

#[test]
fn stream_retains_only_one_coalesced_tick() -> Result<(), BoxError> {
    block(|root| async move {
        let mut stream = root.interval_stream(Duration::from_millis(1))?;
        tokio::task::yield_now().await;
        for _ in 0..20 {
            tokio::time::advance(Duration::from_millis(1)).await;
            tokio::task::yield_now().await;
        }
        assert!(matches!(stream.next().now_or_never(), Some(Some(Ok(())))));
        assert!(
            stream.next().now_or_never().is_none(),
            "no unread tick history"
        );
        stream.control().stop();
        Ok(())
    })
}

#[test]
fn stream_terminal_wins_over_queued_tick_and_unregisters() -> Result<(), BoxError> {
    block(|root| async move {
        let baseline = root.get_effects()?.len();
        let mut stream = root.interval_stream(Duration::from_millis(1))?;
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(1)).await;
        tokio::task::yield_now().await;
        stream.control().fail(TimerError::InvalidDelay);
        tokio::task::yield_now().await;
        tokio::task::yield_now().await;
        assert!(matches!(
            stream.next().now_or_never(),
            Some(Some(Err(TimerError::InvalidDelay)))
        ));
        assert!(matches!(stream.next().now_or_never(), Some(None)));
        assert_eq!(root.get_effects()?.len(), baseline);
        Ok(())
    })
}

#[test]
fn scheduled_self_dispose_releases_capture_without_borrow_panic() -> Result<(), BoxError> {
    block(|root| async move {
        struct Probe(Rc<Cell<usize>>);
        impl Drop for Probe {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let dropped = Rc::new(Cell::new(0));
        let probe = Probe(dropped.clone());
        let slot: Rc<RefCell<Weak<Scheduled>>> = Rc::new(RefCell::new(Weak::new()));
        let callback_slot = slot.clone();
        let scheduled = Rc::new(root.throttle(Duration::from_millis(1), false, move || {
            let _keep_capture = &probe;
            callback_slot.borrow().upgrade().unwrap().dispose().unwrap();
        })?);
        *slot.borrow_mut() = Rc::downgrade(&scheduled);
        scheduled.call()?;
        assert_eq!(
            dropped.get(),
            1,
            "dispose releases the callback even while its handle survives"
        );
        Ok(())
    })
}

#[test]
fn dropping_operation_wrappers_unregisters() -> Result<(), BoxError> {
    block(|root| async move {
        let baseline = root.get_effects()?.len();
        let sleep = root.sleep(Duration::from_secs(10));
        let stream = root.interval_stream(Duration::from_secs(10))?;
        let scheduled = root.debounce(Duration::from_secs(10), || {})?;
        drop((sleep, stream, scheduled));
        tokio::task::yield_now().await;
        tokio::task::yield_now().await;
        assert_eq!(root.get_effects()?.len(), baseline);
        Ok(())
    })
}
