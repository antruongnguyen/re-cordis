#![allow(missing_docs)]

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_timer::TimerExt;

include!("common/mod.rs");

#[test]
fn timeout_fires_once_at_deadline() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        root.timeout(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        assert_eq!(count.get(), 0);
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1);
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1, "fires exactly once");
        Ok(())
    })
}

#[test]
fn timeout_manual_dispose_prevents_fire() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let effect = root.timeout(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        let _completion = effect.dispose()?;
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 0);
        Ok(())
    })
}
