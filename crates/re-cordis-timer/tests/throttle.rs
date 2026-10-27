#![allow(missing_docs)]

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{BoxError, Plugin};
use re_cordis_timer::TimerExt;

include!("common/mod.rs");

#[test]
fn throttle_leading_and_trailing() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let throttled = root.throttle(Duration::from_millis(1000), false, move || {
            c.set(c.get() + 1)
        })?;
        throttled.call();
        assert_eq!(count.get(), 1, "leading edge fires immediately");
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(600)).await;
        tokio::task::yield_now().await;
        throttled.call();
        assert_eq!(count.get(), 1, "within window, suppressed (trailing armed)");
        tokio::task::yield_now().await; // register the armed trailing task
        tokio::time::advance(Duration::from_millis(600)).await;
        tokio::task::yield_now().await;
        throttled.call();
        assert_eq!(count.get(), 2);
        tokio::task::yield_now().await; // let the armed trailing task register
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 3, "trailing call fires");
        Ok(())
    })
}

#[test]
fn throttle_no_trailing_suppresses_trailing_call() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let throttled = root.throttle(Duration::from_millis(1000), true, move || {
            c.set(c.get() + 1)
        })?;
        throttled.call();
        assert_eq!(count.get(), 1);
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(500)).await;
        tokio::task::yield_now().await;
        throttled.call();
        assert_eq!(count.get(), 1);
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1, "no trailing fire");
        throttled.call();
        assert_eq!(count.get(), 2, "window elapsed, leading fires again");
        Ok(())
    })
}

#[test]
fn throttle_manual_dispose_prevents_all() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let throttled = root.throttle(Duration::from_millis(1000), false, move || {
            c.set(c.get() + 1)
        })?;
        throttled.dispose()?;
        throttled.call();
        assert_eq!(count.get(), 0);
        tokio::time::advance(Duration::from_millis(500)).await;
        tokio::task::yield_now().await;
        throttled.call();
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 0, "disposed throttle never fires");
        Ok(())
    })
}

#[test]
fn throttle_owner_dispose_stops() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let holder: Rc<RefCell<Option<re_cordis_timer::Scheduled>>> =
            Rc::new(RefCell::new(None));
        let c = count.clone();
        let slot = holder.clone();
        let child = root.plugin(
            &Plugin::new("throttler", move |ctx: re_cordis_core::Context, (): ()| {
                let c = c.clone();
                let slot = slot.clone();
                async move {
                    let throttled =
                        ctx.throttle(Duration::from_millis(1000), false, move || {
                            c.set(c.get() + 1)
                        })?;
                    *slot.borrow_mut() = Some(throttled);
                    Ok::<_, BoxError>(())
                }
            }),
            (),
        )?;
        child.ready()?.await?;
        let throttled = holder.borrow_mut().take().unwrap();
        throttled.call();
        assert_eq!(count.get(), 1);
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(500)).await;
        tokio::task::yield_now().await;
        throttled.call(); // arms a trailing call within the window
        child.dispose()?.await?;
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1, "owner dispose cancels the pending trailing call");
        throttled.call();
        assert_eq!(count.get(), 1, "calls after owner dispose are inert");
        Ok(())
    })
}
