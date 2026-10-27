#![allow(missing_docs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use futures_util::StreamExt;
use re_cordis_core::{BoxError, Plugin};
use re_cordis_timer::{TimerError, TimerExt};

include!("common/mod.rs");

#[test]
fn stream_yields_ticks_then_ends_on_stop() -> Result<(), BoxError> {
    block(|root| async move {
        let stream = root.interval_stream(Duration::from_millis(1000))?;
        let control = stream.control();
        let seen = Rc::new(RefCell::new(Vec::<Result<(), TimerError>>::new()));
        let sink = seen.clone();
        tokio::task::spawn_local(async move {
            futures_util::pin_mut!(stream);
            while let Some(item) = stream.next().await {
                let is_err = item.is_err();
                sink.borrow_mut().push(item);
                if is_err {
                    break;
                }
            }
        });
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        control.stop();
        tokio::task::yield_now().await;
        let got = seen.borrow();
        assert_eq!(
            got.len(),
            2,
            "two Ok ticks, then clean end (no trailing item)"
        );
        assert!(got.iter().all(|item| item.is_ok()));
        Ok(())
    })
}

#[test]
fn stream_fail_delivers_terminal_error() -> Result<(), BoxError> {
    block(|root| async move {
        let stream = root.interval_stream(Duration::from_millis(1000))?;
        let control = stream.control();
        let last = Rc::new(RefCell::new(None::<Result<(), TimerError>>));
        let sink = last.clone();
        tokio::task::spawn_local(async move {
            futures_util::pin_mut!(stream);
            while let Some(item) = stream.next().await {
                let stop = item.is_err();
                *sink.borrow_mut() = Some(item);
                if stop {
                    break;
                }
            }
        });
        tokio::task::yield_now().await;
        control.fail(TimerError::InvalidDelay);
        tokio::task::yield_now().await;
        assert!(
            matches!(*last.borrow(), Some(Err(TimerError::InvalidDelay))),
            "fail delivers one terminal Err"
        );
        Ok(())
    })
}

#[test]
fn stream_ends_with_disposed_on_owner_dispose() -> Result<(), BoxError> {
    block(|root| async move {
        let last = Rc::new(RefCell::new(None::<Result<(), TimerError>>));
        let sink = last.clone();
        let child = root.plugin(
            &Plugin::new("streamer", move |ctx: re_cordis_core::Context, (): ()| {
                let sink = sink.clone();
                async move {
                    let stream = ctx.interval_stream(Duration::from_millis(1000))?;
                    tokio::task::spawn_local(async move {
                        futures_util::pin_mut!(stream);
                        while let Some(item) = stream.next().await {
                            let stop = item.is_err();
                            *sink.borrow_mut() = Some(item);
                            if stop {
                                break;
                            }
                        }
                    });
                    Ok::<_, BoxError>(())
                }
            }),
            (),
        )?;
        child.ready()?.await?;
        tokio::task::yield_now().await;
        child.dispose()?.await?;
        tokio::task::yield_now().await;
        assert!(
            matches!(*last.borrow(), Some(Err(TimerError::Disposed))),
            "owner dispose ends the stream with Disposed, got {:?}",
            *last.borrow()
        );
        Ok(())
    })
}
