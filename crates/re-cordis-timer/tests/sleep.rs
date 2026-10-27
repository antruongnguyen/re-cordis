#![allow(missing_docs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{BoxError, Plugin};
use re_cordis_timer::{TimerError, TimerExt};

include!("common/mod.rs");

#[test]
fn sleep_resolves_once_at_deadline() -> Result<(), BoxError> {
    block(|root| async move {
        let fut = root.sleep(Duration::from_millis(1000));
        let task = tokio::task::spawn_local(fut);
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(500)).await;
        tokio::task::yield_now().await;
        assert!(!task.is_finished());
        tokio::time::advance(Duration::from_millis(500)).await;
        tokio::task::yield_now().await;
        assert!(matches!(task.await, Ok(Ok(()))));
        Ok(())
    })
}

#[test]
fn sleep_rejects_on_owner_dispose() -> Result<(), BoxError> {
    block(|root| async move {
        // Create the sleep inside a child plugin, then dispose that child's
        // fiber before the deadline. The awaited future must reject with
        // TimerError::Disposed, mirroring the TS `ctx.fiber.dispose()` case.
        let outcome = Rc::new(RefCell::new(None::<Result<(), TimerError>>));
        let sink = outcome.clone();
        let child = root.plugin(
            &Plugin::new("sleeper", move |ctx: re_cordis_core::Context, (): ()| {
                let sink = sink.clone();
                async move {
                    let fut = ctx.sleep(Duration::from_millis(1000));
                    tokio::task::spawn_local(async move {
                        *sink.borrow_mut() = Some(fut.await);
                    });
                    Ok::<_, BoxError>(())
                }
            }),
            (),
        )?;
        child.ready()?.await?;
        tokio::task::yield_now().await;
        // dispose before the deadline
        child.dispose()?.await?;
        tokio::task::yield_now().await;
        assert!(
            matches!(&*outcome.borrow(), Some(Err(TimerError::Disposed))),
            "sleep must reject with Disposed on owner dispose, got {:?}",
            *outcome.borrow()
        );
        Ok(())
    })
}
