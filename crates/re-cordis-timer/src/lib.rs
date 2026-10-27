//! Owner-scoped timer operations for re-cordis.
//!
//! Mirrors the semantics of the TypeScript `cordis` timer satellite on top of
//! [`re_cordis_core`] effects: every operation is cancelled when its owning
//! context is disposed. See the crate docs for the deliberate deviations from
//! the JavaScript async-iterator protocol and wall-clock timing.

use re_cordis_core::{AdmissionError, Context};

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

/// Owner-scoped timer operations, implemented for [`Context`].
///
/// This trait is sealed: it is implemented only for [`Context`] and cannot be
/// implemented downstream. Operations are added as the crate grows.
pub trait TimerExt: sealed::Sealed {}

impl TimerExt for Context {}
