//! Owner-scoped timer operations for re-cordis.
//!
//! Mirrors the semantics of the TypeScript `cordis` timer satellite on top of
//! [`re_cordis_core`] effects: every operation is cancelled when its owning
//! context is disposed. See the crate docs for the deliberate deviations from
//! the JavaScript async-iterator protocol and wall-clock timing.

use std::time::Duration;

use re_cordis_core::{AdmissionError, Context, Effect};

mod sealed {
    pub trait Sealed {}
    impl Sealed for re_cordis_core::Context {}
}

/// Errors from timer construction and awaited operations.
#[derive(Debug)]
pub enum TimerError {
    /// The owning context was disposed before the operation completed.
    Disposed,
    /// The requested delay exceeds the supported range.
    InvalidDelay,
    /// The context could not admit the registration.
    Admission(AdmissionError),
}

impl std::fmt::Display for TimerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerError::Disposed => write!(f, "Context has been disposed"),
            TimerError::InvalidDelay => write!(f, "invalid timer delay"),
            TimerError::Admission(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for TimerError {}

impl From<AdmissionError> for TimerError {
    fn from(error: AdmissionError) -> Self {
        TimerError::Admission(error)
    }
}

/// Validate a requested delay. Zero is admitted (the operation fires on the
/// next runtime turn); only unrepresentable ranges are rejected.
fn check_delay(delay: Duration) -> Result<Duration, TimerError> {
    Ok(delay)
}

/// Owner-scoped timer operations, implemented for [`Context`].
///
/// This trait is sealed: it is implemented only for [`Context`] and cannot be
/// implemented downstream. Every operation schedules work on the runtime's
/// local task set and registers a disposal cleanup, so disposing the owning
/// context cancels the operation and no callback fires afterward.
pub trait TimerExt: sealed::Sealed {
    /// Fire `callback` once after `delay`, then self-dispose.
    ///
    /// Returns the registration [`Effect`]; dispose it to cancel before the
    /// deadline. The callback runs at most once: a fired timeout does not run
    /// again, and a disposed timeout never runs. Owner disposal cancels it.
    fn timeout<F>(&self, delay: Duration, callback: F) -> Result<Effect, TimerError>
    where
        F: FnOnce() + 'static;
}

impl TimerExt for Context {
    fn timeout<F>(&self, delay: Duration, callback: F) -> Result<Effect, TimerError>
    where
        F: FnOnce() + 'static,
    {
        let delay = check_delay(delay)?;
        let fired = std::rc::Rc::new(std::cell::Cell::new(false));
        let armed = fired.clone();
        let handle = tokio::task::spawn_local(async move {
            tokio::time::sleep(delay).await;
            if !armed.get() {
                armed.set(true);
                callback();
            }
        });
        let abort = handle.abort_handle();
        let guard = fired;
        let effect = self.on_dispose_sync(move || {
            guard.set(true);
            abort.abort();
            Ok::<_, std::convert::Infallible>(())
        })?;
        Ok(effect)
    }
}
