#![allow(missing_docs)]

use re_cordis_core::{AdmissionError, BoxError, CallbackError, Disposer, Runtime};
use std::{cell::Cell, rc::Rc};

fn block<F, Fut>(body: F) -> Result<(), BoxError>
where
    F: FnOnce(Runtime) -> Fut,
    Fut: std::future::Future<Output = Result<(), BoxError>>,
{
    tokio::runtime::Builder::new_current_thread()
        .build()?
        .block_on(body(Runtime::new()))
}

#[test]
fn task_retires_before_terminal_callback_and_inert_effect_drop() -> Result<(), BoxError> {
    block(|runtime| async move {
        runtime
            .run(async {
                let ctx = runtime.context();
                let count = Rc::new(Cell::new(0));
                let output = count.clone();
                let check = ctx.clone();
                let (effect, _) = ctx.task(async move {
                    move || {
                        assert!(check.get_effects().unwrap().is_empty());
                        check.on_dispose_sync(|| Ok::<_, BoxError>(())).unwrap();
                        output.set(1);
                    }
                })?;
                drop(effect);
                tokio::task::yield_now().await;
                assert_eq!(count.get(), 1);
                ctx.drain()?.await?;
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok(())
    })
}

#[test]
fn transferred_completed_task_cleanup_is_safe_and_pending_transfer_cancels() -> Result<(), BoxError>
{
    block(|runtime| async move {
        runtime
            .run(async {
                let ctx = runtime.context();
                let (sender, receiver) = tokio::sync::oneshot::channel::<()>();
                let fired = Rc::new(Cell::new(false));
                let output = fired.clone();
                let (effect, _) = ctx.task(async move {
                    let _ = receiver.await;
                    move || output.set(true)
                })?;
                let composed = ctx.effect(|| {
                    Ok::<_, BoxError>(Disposer::compose(vec![Disposer::from_effect(effect)?]))
                })?;
                sender.send(()).unwrap();
                tokio::task::yield_now().await;
                assert!(fired.get(), "journal transfer is not cancellation");
                composed.dispose()?.await?;

                let probe = fired.clone();
                let (effect, _) = ctx.task(async move {
                    std::future::pending::<()>().await;
                    move || probe.set(false)
                })?;
                let composed = ctx.effect(|| Ok::<_, BoxError>(Disposer::from_effect(effect)?))?;
                composed.dispose()?.await?;
                assert!(fired.get(), "cancelled pending transfer never executes");
                assert!(ctx.get_effects()?.is_empty());
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok(())
    })
}

#[test]
fn task_capability_cancels_outside_host_and_owner_barrier_joins_captures() -> Result<(), BoxError> {
    block(|runtime| async move {
        struct Probe(Rc<Cell<bool>>);
        impl Drop for Probe {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }
        let dropped = Rc::new(Cell::new(false));
        let probe = Probe(dropped.clone());
        let control = runtime
            .run(async {
                let ctx = runtime.context();
                let (_, control) = ctx.task(async move {
                    let _capture = probe;
                    std::future::pending::<()>().await;
                    || {}
                })?;
                Ok::<_, BoxError>(control)
            })
            .await??;
        control.cancel();
        assert!(
            !dropped.get(),
            "abort request does not promise joined destruction"
        );
        runtime
            .run(async {
                runtime.context().drain()?.await?;
                assert!(dropped.get());
                assert!(runtime.context().get_effects()?.is_empty());
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok(())
    })
}

#[test]
fn callback_and_task_scopes_reject_self_drain_without_fabricating_host() -> Result<(), BoxError> {
    block(|runtime| async move {
        let ctx = runtime.context();
        assert!(matches!(
            ctx.invoke_callback(|| ()),
            Err(CallbackError::Admission(AdmissionError::HostNotRunning))
        ));
        runtime
            .run(async {
                assert!(matches!(
                    ctx.invoke_callback(|| ctx.drain()),
                    Ok(Err(AdmissionError::ReentrantDrain))
                ));
                let check = ctx.clone();
                let (effect, _) = ctx.task(async move {
                    assert!(matches!(check.drain(), Err(AdmissionError::ReentrantDrain)));
                    move || assert!(matches!(check.drain(), Err(AdmissionError::ReentrantDrain)))
                })?;
                tokio::task::yield_now().await;
                assert_eq!(effect.dispose()?.await?.attempted(), 0);
                let raw_ctx = ctx.clone();
                let raw = tokio::task::spawn_local(async move { raw_ctx.invoke_callback(|| ()) });
                assert!(matches!(
                    raw.await?,
                    Err(CallbackError::Admission(AdmissionError::HostNotRunning))
                ));
                let failure = ctx
                    .invoke_callback(|| panic!("contained callback"))
                    .unwrap_err();
                assert!(matches!(failure, CallbackError::Callback(_)));
                assert_eq!(runtime.take_failures().len(), 1);
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok(())
    })
}

#[test]
fn first_poll_future_panic_releases_capture_and_reports_once() -> Result<(), BoxError> {
    block(|runtime| async move {
        struct Probe(Rc<Cell<bool>>);
        impl Drop for Probe {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }
        let dropped = Rc::new(Cell::new(false));
        let probe = Probe(dropped.clone());
        runtime
            .run(async {
                let ctx = runtime.context();
                ctx.task(async move {
                    let _capture = probe;
                    panic!("future first-poll failure");
                    #[allow(unreachable_code)]
                    {
                        (|| {}) as fn()
                    }
                })?;
                tokio::task::yield_now().await;
                assert!(dropped.get());
                let failures = runtime.take_failures();
                assert_eq!(failures.len(), 1);
                assert!(failures[0].is_panic());
                assert!(ctx.get_effects()?.is_empty());
                assert!(runtime.take_failures().is_empty());
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok(())
    })
}

#[test]
fn host_destruction_releases_unpolled_task_capture_with_surviving_control() -> Result<(), BoxError>
{
    block(|runtime| async move {
        struct Probe(Rc<Cell<bool>>);
        impl Drop for Probe {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }
        let dropped = Rc::new(Cell::new(false));
        let probe = Probe(dropped.clone());
        let control = runtime
            .run(async {
                let (_, control) = runtime.context().task(async move {
                    let _capture = probe;
                    std::future::pending::<()>().await;
                    || {}
                })?;
                Ok::<_, BoxError>(control)
            })
            .await??;
        assert!(!dropped.get());
        drop(runtime);
        assert!(dropped.get());
        control.cancel();
        Ok(())
    })
}
