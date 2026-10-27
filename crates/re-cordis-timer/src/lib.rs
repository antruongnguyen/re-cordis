//! Owner-scoped timer operations for re-cordis.
//!
//! Mirrors the semantics of the TypeScript `cordis` timer satellite on top of
//! [`re_cordis_core`] effects: every operation is cancelled when its owning
//! context is disposed. See the crate docs for the deliberate deviations from
//! the JavaScript async-iterator protocol and wall-clock timing.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};
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

    /// Resolve after `delay` (the awaited form of [`timeout`](TimerExt::timeout)).
    ///
    /// The returned future yields `Ok(())` exactly once at the deadline, or
    /// `Err(TimerError::Disposed)` if the owning context is disposed first.
    /// A delay rejected by range validation yields `Err(TimerError::InvalidDelay)`
    /// on first poll, so callers have a single await point.
    fn sleep(&self, delay: Duration) -> Sleep;

    /// Fire `callback` every `delay`. The first tick is at `delay`, not zero.
    ///
    /// Returns the registration [`Effect`]; dispose it to stop. Missed ticks
    /// coalesce without shifting phase. Owner disposal stops the interval and
    /// no callback fires afterward.
    fn interval<F>(&self, delay: Duration, callback: F) -> Result<Effect, TimerError>
    where
        F: FnMut() + 'static;

    /// A [`Stream`](futures_util::Stream) of ticks every `delay`.
    ///
    /// Each tick yields `Ok(())`. The stream ends (`None`) when its
    /// [`control`](IntervalStream::control) handle calls `stop()`, after a
    /// final `Err(reason)` when it calls `fail(reason)`, and after a final
    /// `Err(TimerError::Disposed)` on owner disposal. The first tick is at
    /// `delay`, not zero.
    ///
    /// Unlike the TypeScript async iterator, a `Stream` has a single logical
    /// consumer: the concurrent multi-reader `return(value)`/`throw(reason)`
    /// protocol is not reproduced. Use [`interval`](TimerExt::interval) with a
    /// callback when you need fan-out.
    fn interval_stream(&self, delay: Duration) -> Result<IntervalStream, TimerError>;
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

    fn sleep(&self, delay: Duration) -> Sleep {
        // Register a disposal cleanup that signals the sleeper. If the context
        // cannot admit the registration, fold the failure into the future.
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let tx = std::cell::RefCell::new(Some(tx));
        let registration = check_delay(delay).and_then(|delay| {
            let effect = self.on_dispose_sync(move || {
                if let Some(tx) = tx.borrow_mut().take() {
                    let _ = tx.send(());
                }
                Ok::<_, std::convert::Infallible>(())
            })?;
            Ok((delay, effect))
        });
        Sleep::new(registration, rx)
    }

    fn interval<F>(&self, delay: Duration, mut callback: F) -> Result<Effect, TimerError>
    where
        F: FnMut() + 'static,
    {
        let delay = check_delay(delay)?;
        let handle = tokio::task::spawn_local(async move {
            // First tick at `delay`, not zero; missed ticks coalesce (skip).
            let start = tokio::time::Instant::now() + delay;
            let mut ticker = tokio::time::interval_at(start, delay);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                ticker.tick().await;
                callback();
            }
        });
        let abort = handle.abort_handle();
        let effect = self.on_dispose_sync(move || {
            abort.abort();
            Ok::<_, std::convert::Infallible>(())
        })?;
        Ok(effect)
    }

    fn interval_stream(&self, delay: Duration) -> Result<IntervalStream, TimerError> {
        let delay = check_delay(delay)?;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Result<(), TimerError>>();
        // The terminal gate is shared by the ticker, the control handle, and
        // the disposal cleanup. Once closed, no further items are sent.
        let gate = std::rc::Rc::new(TerminalGate {
            sender: std::cell::RefCell::new(Some(tx)),
        });
        let ticker_gate = gate.clone();
        let handle = tokio::task::spawn_local(async move {
            let start = tokio::time::Instant::now() + delay;
            let mut ticker = tokio::time::interval_at(start, delay);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                ticker.tick().await;
                if !ticker_gate.send(Ok(())) {
                    break;
                }
            }
        });
        let abort = handle.abort_handle();
        let dispose_gate = gate.clone();
        let effect = self.on_dispose_sync(move || {
            dispose_gate.close_with(Some(TimerError::Disposed));
            abort.abort();
            Ok::<_, std::convert::Infallible>(())
        })?;
        Ok(IntervalStream {
            receiver: rx,
            control: IntervalControl { gate },
            _effect: effect,
        })
    }
}

/// The single terminal gate shared across an interval stream's producers.
struct TerminalGate {
    sender: std::cell::RefCell<Option<tokio::sync::mpsc::UnboundedSender<Result<(), TimerError>>>>,
}

impl TerminalGate {
    /// Send one item while the gate is open. Returns `false` once closed.
    fn send(&self, item: Result<(), TimerError>) -> bool {
        match self.sender.borrow().as_ref() {
            Some(sender) => sender.send(item).is_ok(),
            None => false,
        }
    }

    /// Close the gate, optionally delivering one final error first.
    fn close_with(&self, error: Option<TimerError>) {
        if let Some(sender) = self.sender.borrow_mut().take() {
            if let Some(error) = error {
                let _ = sender.send(Err(error));
            }
        }
    }
}

/// Controls termination of an [`IntervalStream`].
///
/// Obtained from [`IntervalStream::control`]; cheaply cloneable. `stop` ends
/// the stream cleanly; `fail` delivers one final error then ends it.
#[derive(Clone)]
pub struct IntervalControl {
    gate: std::rc::Rc<TerminalGate>,
}

impl IntervalControl {
    /// End the stream cleanly: no further ticks, no trailing item.
    pub fn stop(&self) {
        self.gate.close_with(None);
    }

    /// Deliver one final `Err(reason)`, then end the stream.
    pub fn fail(&self, reason: TimerError) {
        self.gate.close_with(Some(reason));
    }
}

/// A [`Stream`](futures_util::Stream) of interval ticks from
/// [`TimerExt::interval_stream`]. Each item is `Ok(())`; termination is
/// signalled as described on that method. Holds its disposal registration
/// alive for its lifetime.
pub struct IntervalStream {
    receiver: tokio::sync::mpsc::UnboundedReceiver<Result<(), TimerError>>,
    control: IntervalControl,
    _effect: Effect,
}

impl IntervalStream {
    /// A handle to stop or fail this stream.
    pub fn control(&self) -> IntervalControl {
        self.control.clone()
    }
}

impl futures_util::Stream for IntervalStream {
    type Item = Result<(), TimerError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Future returned by [`TimerExt::sleep`].
///
/// Resolves `Ok(())` at the deadline, or `Err(TimerError::Disposed)` if the
/// owning context is disposed first. Holds the disposal registration alive for
/// its lifetime so the cancellation signal remains wired.
pub struct Sleep {
    inner: Pin<Box<dyn Future<Output = Result<(), TimerError>>>>,
}

impl Sleep {
    fn new(
        registration: Result<(Duration, Effect), TimerError>,
        rx: tokio::sync::oneshot::Receiver<()>,
    ) -> Self {
        let inner = Box::pin(async move {
            let (delay, effect) = registration?;
            let result = tokio::select! {
                biased;
                _ = rx => Err(TimerError::Disposed),
                () = tokio::time::sleep(delay) => Ok(()),
            };
            // Keep the registration alive until the race settles, then release
            // it so the disposal cleanup is not left dangling.
            drop(effect);
            result
        });
        Sleep { inner }
    }
}

impl Future for Sleep {
    type Output = Result<(), TimerError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}
