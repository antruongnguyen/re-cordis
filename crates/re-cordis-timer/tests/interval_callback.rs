#![allow(missing_docs)]

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::BoxError;
use re_cordis_timer::TimerExt;

include!("common/mod.rs");

#[test]
fn interval_fires_each_period_and_stops_on_dispose() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let effect = root.interval(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        assert_eq!(count.get(), 0);
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1);
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 2);
        let _completion = effect.dispose()?;
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 2, "no ticks after dispose");
        Ok(())
    })
}
