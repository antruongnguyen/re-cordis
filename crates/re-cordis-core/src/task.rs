use std::{
    cell::{Cell, RefCell},
    error::Error,
    fmt,
    future::Future,
    rc::Rc,
};

use tokio::{sync::watch, task::AbortHandle};

use crate::{AdmissionError, CleanupFailure, Context, Effect};

/// Refused admission or a contained synchronous callback panic.
#[derive(Clone, Debug)]
pub enum CallbackError {
    /// The selected Context could not admit this call.
    Admission(AdmissionError),
    /// The callback panicked; its original diagnostic is retained and reported once.
    Callback(CleanupFailure),
}

impl fmt::Display for CallbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission(error) => error.fmt(f),
            Self::Callback(error) => error.fmt(f),
        }
    }
}
impl Error for CallbackError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Admission(error) => Some(error),
            Self::Callback(error) => Some(error),
        }
    }
}
impl From<AdmissionError> for CallbackError {
    fn from(error: AdmissionError) -> Self {
        Self::Admission(error)
    }
}

struct TaskState {
    cancelled: Cell<bool>,
    finished: Cell<bool>,
    abort: RefCell<Option<AbortHandle>>,
    completion: watch::Receiver<bool>,
}

/// Cancellation authority for one admitted task, without authority to invoke work.
///
/// Cancellation is safe outside Runtime::run. It marks the operation and requests
/// abort; task destruction and registration retirement finish when its original
/// host is driven. Await the owner's disposal/drain to join cancelled captures.
/// Dropping this capability alone is inert.
#[derive(Clone)]
pub struct TaskControl(Rc<TaskState>);

impl TaskControl {
    /// Request cancellation once; a completed task is unaffected.
    pub fn cancel(&self) {
        if !self.0.finished.get() {
            self.0.cancelled.set(true);
            if let Some(abort) = self.0.abort.borrow().as_ref() {
                abort.abort();
            }
        }
    }

    async fn wait(&self) {
        let mut receiver = self.0.completion.clone();
        while !*receiver.borrow_and_update() {
            if receiver.changed().await.is_err() {
                break;
            }
        }
    }
}

impl Context {
    /// Invoke a synchronous callback under the current admitted Context.
    ///
    /// This does not create host authority: raw Tokio tasks and other hosts
    /// remain refused. Panics are contained, retain their source, and reach the
    /// normal core diagnostic path once. Self/ancestor lifecycle waits are
    /// refused while the callback runs.
    pub fn invoke_callback<F, T>(&self, callback: F) -> Result<T, CallbackError>
    where
        F: FnOnce() -> T,
    {
        let state = self
            .runtime
            .upgrade()
            .ok_or(AdmissionError::RuntimeDropped)?;
        state.admit()?;
        let owner = self.owner()?;
        owner.admit()?;
        owner
            .setup_scope(|| crate::effect::invoke(|| Ok(callback())))
            .map_err(|failure| {
                state.report_for(&owner, failure.clone());
                CallbackError::Callback(failure)
            })
    }

    /// Admit local work whose returned synchronous callback is its terminal action.
    ///
    /// The future is polled under this host and owner, with panic containment.
    /// Successful work retires its exact effect before invoking the returned
    /// callback, with no suspension between retirement and invocation. Future
    /// and callback panics are reported once. Owner cleanup requests abort in
    /// its synchronous prefix and joins destruction before completing.
    ///
    /// The returned Effect keeps core's inert Drop and supports Disposer transfer.
    /// TaskControl cancels without admitting or executing callbacks. Pending tasks
    /// require their original host to remain driven. Self/ancestor lifecycle
    /// waits from the task are refused to prevent joining its own cancellation.
    pub fn task<Fut, F>(&self, future: Fut) -> Result<(Effect, TaskControl), AdmissionError>
    where
        Fut: Future<Output = F> + 'static,
        F: FnOnce() + 'static,
    {
        let state = self
            .runtime
            .upgrade()
            .ok_or(AdmissionError::RuntimeDropped)?;
        let host = state.admit()?;
        let owner = self.owner()?;
        owner.admit()?;
        let (sender, receiver) = watch::channel(false);
        let control = TaskControl(Rc::new(TaskState {
            cancelled: Cell::new(false),
            finished: Cell::new(false),
            abort: RefCell::new(None),
            completion: receiver,
        }));
        let cleanup_control = control.clone();
        let effect = self.on_dispose(move || {
            cleanup_control.cancel();
            async move {
                cleanup_control.wait().await;
                Ok::<_, std::convert::Infallible>(())
            }
        })?;
        let id = effect.id;
        let task_control = control.clone();
        let task_owner = owner.clone();
        let weak_state = Rc::downgrade(&state);
        let worker = state.spawn(&host, async move {
            let result = task_owner
                .cleanup_scope(crate::effect::invoke_async(async move { Ok(future.await) }))
                .await;
            // Missing journal entry can mean transfer into a composed disposer.
            // Cancellation authority lives in TaskState, never journal presence.
            let retired = task_owner.cleanups.borrow_mut().remove(&id);
            drop(retired);
            if let Some(state) = weak_state.upgrade() {
                match result {
                    Ok(callback) if !task_control.0.cancelled.get() => {
                        if let Err(failure) = task_owner.setup_scope(|| {
                            crate::effect::invoke(|| {
                                callback();
                                Ok(())
                            })
                        }) {
                            state.report_for(&task_owner, failure);
                        }
                    }
                    Ok(_) => {}
                    Err(failure) => state.report_for(&task_owner, failure),
                }
            }
        });
        *control.0.abort.borrow_mut() = Some(worker.abort_handle());
        let supervisor_control = control.clone();
        let weak_state = Rc::downgrade(&state);
        state.spawn(&host, async move {
            let outcome = worker.await;
            let retired = owner.cleanups.borrow_mut().remove(&id);
            drop(retired);
            if let Err(error) = outcome {
                if error.is_panic() {
                    // Framework failure outside the contained user future/callback.
                    if let Some(state) = weak_state.upgrade() {
                        state
                            .report_for(&owner, CleanupFailure::panic(Box::new(error.to_string())));
                    }
                }
            }
            supervisor_control.0.finished.set(true);
            supervisor_control.0.abort.borrow_mut().take();
            sender.send_replace(true);
        });
        Ok((effect, control))
    }
}
