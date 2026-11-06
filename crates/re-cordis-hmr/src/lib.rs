//! Bounded owner-scoped registered-file watching and explicit reload coordination.
//!
//! Registered source changes require an external restart; local handles retain
//! the original Cordis host. Only SyntheticEvents crosses producer threads.
//!
//! With the `include` feature, `FileWatcher::watch_include` refreshes one explicit
//! source and retains admitted Include work through callback cancellation.
//! Watcher or ancestor disposal joins that work without disposing the Include.
//!
//! ```compile_fail
//! use re_cordis_hmr::{FileWatcher, WatchToken};
//! fn send<T: Send>() {}
//! send::<FileWatcher>();
//! send::<WatchToken>();
//! ```
mod paths;
mod reload;
mod watch;
pub use re_cordis_loader::FactoryReplacement;
pub use reload::{ReloadNode, ReloadPlan};
pub use watch::{
    Change, ChangeKind, FileWatcher, SyntheticDelivery, SyntheticError, SyntheticEvents,
    WatchBackend, WatchCapabilities, WatchError, WatchEvent, WatchFailure, WatchFailureStage,
    WatchId, WatchOptions, WatchStats, WatchToken,
};
