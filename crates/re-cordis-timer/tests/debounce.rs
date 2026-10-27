#![allow(missing_docs)]

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{BoxError, Plugin};
use re_cordis_timer::TimerExt;

include!("common/mod.rs");

#[test]
fn debounce_coalesces_to_trailing_edge() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let debounced = root.debounce(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        debounced.call();
        assert_eq!(count.get(), 0);
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(400)).await;
        tokio::task::yield_now().await;
        debounced.call();
        assert_eq!(count.get(), 0, "reset within window");
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(400)).await;
        tokio::task::yield_now().await;
        debounced.call();
        assert_eq!(count.get(), 0, "reset again");
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1, "fires once after the calls go quiet");
        Ok(())
    })
}

#[test]
fn debounce_manual_dispose_prevents_fire() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let debounced = root.debounce(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        debounced.dispose()?;
        debounced.call();
        assert_eq!(count.get(), 0);
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 0);
        Ok(())
    })
}

#[test]
fn debounce_owner_dispose_prevents_fire() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let holder: Rc<RefCell<Option<re_cordis_timer::Scheduled>>> = Rc::new(RefCell::new(None));
        let c = count.clone();
        let slot = holder.clone();
        let child = root.plugin(
            &Plugin::new("debouncer", move |ctx: re_cordis_core::Context, (): ()| {
                let c = c.clone();
                let slot = slot.clone();
                async move {
                    let debounced =
                        ctx.debounce(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
                    *slot.borrow_mut() = Some(debounced);
                    Ok::<_, BoxError>(())
                }
            }),
            (),
        )?;
        child.ready()?.await?;
        let debounced = holder.borrow_mut().take().unwrap();
        debounced.call();
        tokio::task::yield_now().await;
        child.dispose()?.await?;
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 0, "owner dispose cancels the pending fire");
        debounced.call();
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 0, "calls after owner dispose are inert");
        Ok(())
    })
}
