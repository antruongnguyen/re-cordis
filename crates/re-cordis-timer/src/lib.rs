//! Owner-scoped local timers for Cordis.
//!
//! Import TimerExt on Context. The owning Cordis host must remain driven;
//! cancellation requests abort; awaited owner disposal/drain joins capture release.

use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context as TaskContext, Poll},
    time::Duration,
};

use futures_util::{future::poll_fn, task::AtomicWaker};
use re_cordis_core::{AdmissionError, CallbackError, CleanupFailure, Context, Effect, TaskControl};

mod sealed {
    pub trait Sealed {}
    impl Sealed for re_cordis_core::Context {}
}

/// Timer admission, clock environment, callback, and cancellation errors.
#[derive(Debug)]
pub enum TimerError {
    /// The owner cancelled this operation.
    Disposed,
    /// A repeat period is zero or the monotonic deadline is unrepresentable.
    InvalidDelay,
    /// The current Tokio executor has no enabled time driver.
    TimeDriverUnavailable,
    /// The context could not admit this operation.
    Admission(AdmissionError),
    /// A synchronous scheduled callback panicked; the original diagnostic is retained.
    Callback(CleanupFailure),
}

impl std::fmt::Display for TimerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disposed => f.write_str("Context has been disposed"),
            Self::InvalidDelay => f.write_str("invalid timer delay"),
            Self::TimeDriverUnavailable => {
                f.write_str("enable the Tokio time driver before constructing timers")
            }
            Self::Admission(error) => error.fmt(f),
            Self::Callback(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for TimerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Admission(error) => Some(error),
            Self::Callback(error) => Some(error),
            _ => None,
        }
    }
}
impl From<AdmissionError> for TimerError {
    fn from(error: AdmissionError) -> Self {
        Self::Admission(error)
    }
}
impl From<CallbackError> for TimerError {
    fn from(error: CallbackError) -> Self {
        match error {
            CallbackError::Admission(error) => Self::Admission(error),
            CallbackError::Callback(error) => Self::Callback(error),
        }
    }
}

fn checked_sleep(delay: Duration) -> Result<Pin<Box<tokio::time::Sleep>>, TimerError> {
    let deadline = tokio::time::Instant::now()
        .checked_add(delay)
        .ok_or(TimerError::InvalidDelay)?;
    // Tokio has no public time-driver presence query. Contain its synchronous
    // construction panic; the normal panic hook still runs, with no detached task.
    std::panic::catch_unwind(|| Box::pin(tokio::time::sleep_until(deadline)))
        .map_err(|_| TimerError::TimeDriverUnavailable)
}

fn prepare_sleep(
    ctx: &Context,
    delay: Duration,
) -> Result<Pin<Box<tokio::time::Sleep>>, TimerError> {
    ctx.invoke_callback(|| checked_sleep(delay))?
}

fn prepare_interval(ctx: &Context, period: Duration) -> Result<tokio::time::Interval, TimerError> {
    ctx.invoke_callback(|| {
        if period.is_zero() {
            return Err(TimerError::InvalidDelay);
        }
        let deadline = checked_sleep(period)?.deadline();
        let mut interval = tokio::time::interval_at(deadline, period);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        Ok(interval)
    })?
}

/// Owner-scoped timer operations, implemented for [`Context`].
///
/// This trait is sealed: it is implemented only for [`Context`] and cannot be
/// implemented downstream. Every operation schedules work on the runtime's
/// local task set through core's supervised task bridge, so awaited owner
/// disposal cancels and joins the operation; no callback begins after its barrier.
/// Callback captures must be owned and 'static:
///
/// ```compile_fail
/// use re_cordis_core::Runtime;
/// use re_cordis_timer::TimerExt;
/// use std::time::Duration;
/// let runtime = Runtime::new();
/// let text = String::from("borrowed");
/// let _ = runtime.context().timeout(Duration::ZERO, || println!("{text}"));
/// ```
///
/// Awaited timers remain local:
///
/// ```compile_fail
/// use re_cordis_timer::Sleep;
/// fn move_to_executor(sleep: Sleep) { tokio::spawn(sleep); }
/// ```
///
/// Downstream types cannot implement this sealed trait:
///
/// ```compile_fail
/// use re_cordis_core::Effect;
/// use re_cordis_timer::{IntervalStream, Scheduled, Sleep, TimerError, TimerExt};
/// use std::time::Duration;
/// struct Foreign;
/// impl TimerExt for Foreign {
///     fn timeout<F: FnOnce() + 'static>(&self, _: Duration, _: F) -> Result<Effect, TimerError> { unimplemented!() }
///     fn sleep(&self, _: Duration) -> Sleep { unimplemented!() }
///     fn interval<F: FnMut() + 'static>(&self, _: Duration, _: F) -> Result<Effect, TimerError> { unimplemented!() }
///     fn interval_stream(&self, _: Duration) -> Result<IntervalStream, TimerError> { unimplemented!() }
///     fn throttle<F: FnMut() + 'static>(&self, _: Duration, _: bool, _: F) -> Result<Scheduled, TimerError> { unimplemented!() }
///     fn debounce<F: FnMut() + 'static>(&self, _: Duration, _: F) -> Result<Scheduled, TimerError> { unimplemented!() }
/// }
/// ```
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
    /// `Err(TimerError::Disposed)` if delivered owner cancellation wins first.
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

    /// Rate-limit `callback`: the first call in a window fires immediately
    /// (leading edge). Within the window, calls are suppressed; unless
    /// `no_trailing` is set, the most recent suppressed call fires once the
    /// window elapses (trailing edge). The window is monotonic.
    ///
    /// Returns a [`Scheduled`] handle: call it with [`Scheduled::call`], and
    /// cancel it with [`Scheduled::dispose`]. Owner disposal cancels any
    /// pending trailing call and makes further calls inert.
    fn throttle<F>(
        &self,
        delay: Duration,
        no_trailing: bool,
        callback: F,
    ) -> Result<Scheduled, TimerError>
    where
        F: FnMut() + 'static;

    /// Coalesce bursts of calls to a single trailing fire: each call restarts
    /// the `delay` window, and `callback` runs once the calls go quiet for the
    /// full delay. The window is monotonic.
    ///
    /// Returns a [`Scheduled`] handle: call it with [`Scheduled::call`], and
    /// cancel it with [`Scheduled::dispose`]. Owner disposal cancels any
    /// pending fire and makes further calls inert.
    fn debounce<F>(&self, delay: Duration, callback: F) -> Result<Scheduled, TimerError>
    where
        F: FnMut() + 'static;
}

impl TimerExt for Context {
    fn timeout<F>(&self, delay: Duration, callback: F) -> Result<Effect, TimerError>
    where
        F: FnOnce() + 'static,
    {
        let deadline = prepare_sleep(self, delay)?;
        let (effect, _) = self.task(async move {
            deadline.await;
            callback
        })?;
        Ok(effect)
    }

    fn sleep(&self, delay: Duration) -> Sleep {
        let registration = prepare_sleep(self, delay).and_then(|deadline| {
            let (sender, receiver) = tokio::sync::oneshot::channel();
            let (_, control) = self.task(async move {
                deadline.await;
                move || {
                    let _ = sender.send(());
                }
            })?;
            Ok((receiver, control))
        });
        Sleep {
            registration: Some(registration),
            terminated: false,
        }
    }

    fn interval<F>(&self, delay: Duration, callback: F) -> Result<Effect, TimerError>
    where
        F: FnMut() + 'static,
    {
        let ticker = prepare_interval(self, delay)?;
        let (effect, _) = self.task(repeat_callback(ticker, callback))?;
        Ok(effect)
    }

    fn interval_stream(&self, delay: Duration) -> Result<IntervalStream, TimerError> {
        let mut ticker = prepare_interval(self, delay)?;
        let gate = Rc::new(StreamGate {
            tick: Cell::new(false),
            terminal: RefCell::new(None),
            waker: AtomicWaker::new(),
            control: RefCell::new(None),
        });
        let guard = StreamGuard(gate.clone());
        let task_gate = gate.clone();
        let (_, control) = self.task(async move {
            let _guard = guard;
            while task_gate.terminal.borrow().is_none() {
                if let Err(error) = coalesced_tick(&mut ticker).await {
                    task_gate.close(Some(error));
                    break;
                }
                task_gate.tick.set(true);
                task_gate.waker.wake();
            }
            (|| {}) as fn()
        })?;
        *gate.control.borrow_mut() = Some(control);
        Ok(IntervalStream {
            control: IntervalControl { gate },
        })
    }

    fn throttle<F>(
        &self,
        delay: Duration,
        no_trailing: bool,
        callback: F,
    ) -> Result<Scheduled, TimerError>
    where
        F: FnMut() + 'static,
    {
        Scheduled::new(
            self,
            delay,
            ScheduleKind::Throttle { no_trailing },
            callback,
        )
    }

    fn debounce<F>(&self, delay: Duration, callback: F) -> Result<Scheduled, TimerError>
    where
        F: FnMut() + 'static,
    {
        Scheduled::new(self, delay, ScheduleKind::Debounce, callback)
    }
}

/// Future returned by TimerExt::sleep, admitted eagerly at construction.
///
/// Completion retires its registration before delivery. Owner cancellation
/// yields Disposed. Drop requests cancellation; retirement needs the original
/// host to be driven or its owner barrier awaited. Repoll after completion panics.
pub struct Sleep {
    registration: Option<Result<(tokio::sync::oneshot::Receiver<()>, TaskControl), TimerError>>,
    terminated: bool,
}

impl Future for Sleep {
    type Output = Result<(), TimerError>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        assert!(!self.terminated, "polled Sleep after completion");
        let result = match self.registration.as_mut().expect("live sleep registration") {
            Err(_) => {
                let Err(error) = self.registration.take().unwrap() else {
                    unreachable!()
                };
                Err(error)
            }
            Ok((receiver, _)) => match Pin::new(receiver).poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Ok(())) => Ok(()),
                Poll::Ready(Err(_)) => Err(TimerError::Disposed),
            },
        };
        self.terminated = true;
        self.registration.take();
        Poll::Ready(result)
    }
}

impl Drop for Sleep {
    fn drop(&mut self) {
        if let Some(Ok((_, control))) = self.registration.as_ref() {
            control.cancel();
        }
    }
}

struct StreamGate {
    tick: Cell<bool>,
    // None: open. Some(None): clean/consumed terminal. Some(Some(error)): final error.
    terminal: RefCell<Option<Option<TimerError>>>,
    waker: AtomicWaker,
    control: RefCell<Option<TaskControl>>,
}

impl StreamGate {
    fn close(&self, reason: Option<TimerError>) {
        let mut terminal = self.terminal.borrow_mut();
        if terminal.is_some() {
            return;
        }
        *terminal = Some(reason);
        drop(terminal);
        self.tick.set(false);
        if let Some(control) = self.control.borrow().as_ref() {
            control.cancel();
        }
        self.waker.wake();
    }
}

struct StreamGuard(Rc<StreamGate>);
impl Drop for StreamGuard {
    fn drop(&mut self) {
        self.0.close(Some(TimerError::Disposed));
    }
}

/// Cloneable authority to terminate one IntervalStream.
///
/// First committed terminal gate wins and discards a pending tick. Cancellation requests
/// are safe outside the host; producer destruction/retirement need host progress.
#[derive(Clone)]
pub struct IntervalControl {
    gate: Rc<StreamGate>,
}

impl IntervalControl {
    /// End cleanly, discarding any queued tick.
    pub fn stop(&self) {
        self.gate.close(None);
    }
    /// Deliver this error once, then stable None; discards any queued tick.
    pub fn fail(&self, reason: TimerError) {
        self.gate.close(Some(reason));
    }
}

/// Single-consumer stream with at most one coalesced pending tick.
///
/// Committed terminal state takes priority over tick delivery. A lifecycle request
/// precedes delivered owner cancellation; the first committed gate wins. Drop
/// requests cancellation.
pub struct IntervalStream {
    control: IntervalControl,
}

impl IntervalStream {
    /// Obtain cloneable terminal authority for this stream.
    pub fn control(&self) -> IntervalControl {
        self.control.clone()
    }
}

impl futures_util::Stream for IntervalStream {
    type Item = Result<(), TimerError>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        let gate = &self.control.gate;

        let mut terminal = gate.terminal.borrow_mut();
        if let Some(error) = terminal.as_mut() {
            return Poll::Ready(error.take().map(Err));
        }
        drop(terminal);
        if gate.tick.replace(false) {
            Poll::Ready(Some(Ok(())))
        } else {
            gate.waker.register(cx.waker());
            Poll::Pending
        }
    }
}

impl Drop for IntervalStream {
    fn drop(&mut self) {
        self.control.stop();
    }
}

#[derive(Clone, Copy)]
enum ScheduleKind {
    Throttle { no_trailing: bool },
    Debounce,
}

struct ScheduleState {
    callback: Option<Box<dyn FnMut()>>,
    pending: Option<Pin<Box<tokio::time::Sleep>>>,
    last_call: Option<tokio::time::Instant>,
    running: bool,
    disposed: bool,
}

struct ScheduleShared {
    state: RefCell<ScheduleState>,
    waker: AtomicWaker,
}

impl ScheduleShared {
    fn cancel(&self) {
        let (callback, pending) = {
            let mut state = self.state.borrow_mut();
            state.disposed = true;
            (state.callback.take(), state.pending.take())
        };
        drop((callback, pending));
        self.waker.wake();
    }

    fn execute(&self, ctx: &Context) -> Result<(), TimerError> {
        let mut callback = {
            let mut state = self.state.borrow_mut();
            if state.disposed {
                return Ok(());
            }
            let Some(callback) = state.callback.take() else {
                return Ok(());
            };
            state.running = true;
            state.last_call = Some(tokio::time::Instant::now());
            Some(callback)
        };
        let result = ctx
            .invoke_callback(|| (callback.as_mut().unwrap())())
            .map_err(TimerError::from);
        {
            let mut state = self.state.borrow_mut();
            state.running = false;
            if result.is_err() {
                state.disposed = true;
            }
            if !state.disposed {
                state.callback = callback.take();
            }
        }
        drop(callback);
        if result.is_err() {
            self.cancel();
        }
        result
    }
}

struct ScheduleGuard(Rc<ScheduleShared>);
impl Drop for ScheduleGuard {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

/// Owner-scoped throttle/debounce callback and its cancellation capability.
///
/// Callback state is never mutably borrowed during user code. Recursive positive
/// delay calls coalesce through the ordinary trailing window. Recursive zero
/// delay calls are ignored, bounding synchronous recursion and zero-delay loops.
/// Drop requests cancellation and releases idle captures synchronously; joined
/// task destruction/retirement needs host progress or an owner barrier.
pub struct Scheduled {
    ctx: Context,
    delay: Duration,
    kind: ScheduleKind,
    shared: Rc<ScheduleShared>,
    control: TaskControl,
}

impl Scheduled {
    fn new<F>(
        ctx: &Context,
        delay: Duration,
        kind: ScheduleKind,
        callback: F,
    ) -> Result<Self, TimerError>
    where
        F: FnMut() + 'static,
    {
        drop(prepare_sleep(ctx, delay)?);
        let shared = Rc::new(ScheduleShared {
            state: RefCell::new(ScheduleState {
                callback: Some(Box::new(callback)),
                pending: None,
                last_call: None,
                running: false,
                disposed: false,
            }),
            waker: AtomicWaker::new(),
        });
        let guard = ScheduleGuard(shared.clone());
        let task_shared = shared.clone();
        let task_ctx = ctx.clone();
        let (_, control) = ctx.task(async move {
            let _guard = guard;
            loop {
                let fire = poll_fn(|cx| {
                    task_shared.waker.register(cx.waker());
                    let mut state = task_shared.state.borrow_mut();
                    if state.disposed {
                        return Poll::Ready(false);
                    }
                    if let Some(timer) = state.pending.as_mut() {
                        if timer.as_mut().poll(cx).is_ready() {
                            state.pending.take();
                            return Poll::Ready(true);
                        }
                    }
                    Poll::Pending
                })
                .await;
                if !fire || task_shared.execute(&task_ctx).is_err() {
                    break;
                }
            }
            (|| {}) as fn()
        })?;
        Ok(Self {
            ctx: ctx.clone(),
            delay,
            kind,
            shared,
            control,
        })
    }

    /// Invoke this schedule under its original admitted host.
    ///
    /// Disposed calls are inert. Active calls return admission/range/driver
    /// errors before changing scheduling. Leading callback panic is contained,
    /// stops this schedule, and returns the original reported callback failure.
    pub fn call(&self) -> Result<(), TimerError> {
        if self.shared.state.borrow().disposed {
            return Ok(());
        }
        let mut timer = prepare_sleep(&self.ctx, self.delay)?;
        let execute = {
            let mut state = self.shared.state.borrow_mut();
            if state.running && self.delay.is_zero() {
                return Ok(());
            }
            state.pending.take();
            match self.kind {
                ScheduleKind::Throttle { no_trailing } => {
                    let deadline = state
                        .last_call
                        .and_then(|last| last.checked_add(self.delay));
                    let remaining = deadline
                        .map(|at| at.saturating_duration_since(tokio::time::Instant::now()))
                        .unwrap_or_default();
                    if remaining.is_zero() && !state.running {
                        true
                    } else {
                        if !no_trailing {
                            timer
                                .as_mut()
                                .reset(deadline.unwrap_or_else(tokio::time::Instant::now));
                            state.pending = Some(timer);
                        }
                        false
                    }
                }
                ScheduleKind::Debounce => {
                    state.pending = Some(timer);
                    false
                }
            }
        };
        self.shared.waker.wake();
        if execute {
            let result = self.shared.execute(&self.ctx);
            if result.is_err() {
                self.control.cancel();
            }
            result
        } else {
            Ok(())
        }
    }

    /// Request cancellation and make later calls inert, safely outside the host.
    ///
    /// Idle captures release now; an executing callback releases when it returns.
    /// The original owner barrier joins task destruction and effect retirement.
    pub fn dispose(&self) -> Result<(), TimerError> {
        self.shared.cancel();
        self.control.cancel();
        Ok(())
    }
}

impl Drop for Scheduled {
    fn drop(&mut self) {
        self.shared.cancel();
        self.control.cancel();
    }
}

async fn repeat_callback<F: FnMut()>(mut ticker: tokio::time::Interval, mut callback: F) -> fn() {
    while coalesced_tick(&mut ticker).await.is_ok() {
        callback();
    }
    (|| {}) as fn()
}

async fn coalesced_tick(ticker: &mut tokio::time::Interval) -> Result<(), TimerError> {
    let phase = ticker.tick().await;
    let now = tokio::time::Instant::now();
    let period = ticker.period();
    let remainder = now.saturating_duration_since(phase).as_nanos() % period.as_nanos();
    let remainder = Duration::new(
        (remainder / 1_000_000_000) as u64,
        (remainder % 1_000_000_000) as u32,
    );
    let next = now
        .checked_add(period - remainder)
        .ok_or(TimerError::InvalidDelay)?;
    // Tokio Skip still bursts when less than 5ms late. Reset explicitly to the
    // next original phase, also bounding admitted sub-millisecond periods.
    ticker.reset_at(next);
    Ok(())
}
