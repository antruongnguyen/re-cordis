#![allow(missing_docs)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{AdmissionError, BoxError, Context, Plugin, Runtime};
use re_cordis_timer::{TimerError, TimerExt};

include!("common/mod.rs");

struct Probe(Rc<Cell<usize>>);
impl Drop for Probe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn no_host_returns_admission_error_without_panicking() {
    let runtime = Runtime::new();
    let ctx = runtime.context();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ctx.timeout(Duration::from_millis(1), || {})
    }));
    assert!(matches!(
        result,
        Ok(Err(TimerError::Admission(AdmissionError::HostNotRunning)))
    ));
}

#[test]
fn refused_callback_construction_drops_every_capture_immediately() -> Result<(), BoxError> {
    block(|_| async move {
        let runtime = Runtime::new();
        let ctx = runtime.context();
        let drops = Rc::new(Cell::new(0));
        let a = Probe(drops.clone());
        assert!(
            ctx.timeout(Duration::from_millis(1), move || {
                let _capture = &a;
            })
            .is_err()
        );
        assert_eq!(drops.get(), 1, "refused timeout retains no callback");
        let b = Probe(drops.clone());
        assert!(
            ctx.interval(Duration::from_millis(1), move || {
                let _capture = &b;
            })
            .is_err()
        );
        assert_eq!(drops.get(), 2, "refused interval retains no callback");
        let c = Probe(drops.clone());
        assert!(
            ctx.throttle(Duration::from_millis(1), false, move || {
                let _capture = &c;
            })
            .is_err()
        );
        assert_eq!(drops.get(), 3);
        let d = Probe(drops.clone());
        assert!(
            ctx.debounce(Duration::from_millis(1), move || {
                let _capture = &d;
            })
            .is_err()
        );
        assert_eq!(drops.get(), 4);
        Ok(())
    })
}

#[test]
fn stale_owner_refuses_all_timer_operations() -> Result<(), BoxError> {
    block(|root| async move {
        let slot = Rc::new(RefCell::new(None::<Context>));
        let output = slot.clone();
        let child = root.plugin(
            &Plugin::new("timer-owner", move |ctx, ()| {
                *output.borrow_mut() = Some(ctx);
                async { Ok::<_, BoxError>(()) }
            }),
            (),
        )?;
        child.ready()?.await?;
        child.dispose()?.await?;
        let ctx = slot.borrow_mut().take().unwrap();
        let is_stale = |result: Result<_, TimerError>| {
            matches!(
                result,
                Err(TimerError::Admission(AdmissionError::InactiveContext))
            )
        };
        assert!(is_stale(ctx.timeout(Duration::from_millis(1), || {})));
        assert!(matches!(
            futures_util::FutureExt::now_or_never(ctx.sleep(Duration::from_millis(1))),
            Some(Err(TimerError::Admission(AdmissionError::InactiveContext)))
        ));
        assert!(matches!(
            ctx.interval(Duration::from_millis(1), || {}),
            Err(TimerError::Admission(AdmissionError::InactiveContext))
        ));
        assert!(matches!(
            ctx.interval_stream(Duration::from_millis(1)),
            Err(TimerError::Admission(AdmissionError::InactiveContext))
        ));
        assert!(matches!(
            ctx.throttle(Duration::from_millis(1), false, || {}),
            Err(TimerError::Admission(AdmissionError::InactiveContext))
        ));
        assert!(matches!(
            ctx.debounce(Duration::from_millis(1), || {}),
            Err(TimerError::Admission(AdmissionError::InactiveContext))
        ));
        Ok(())
    })
}
