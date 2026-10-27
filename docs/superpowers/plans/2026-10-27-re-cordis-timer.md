# re-cordis-timer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Deliver an unpublished `crates/re-cordis-timer` satellite that mirrors the TypeScript `cordis` timer semantics on top of existing `re-cordis-core` effects, with deterministic tests, a runnable example, and bilingual docs.

**Architecture:** Consumers reach operations through a sealed `TimerExt` trait implemented for `Context` — no service registration is required (settled at the gate). Every operation schedules a `tokio::time` task on core's `LocalSet` host via `spawn_local` and registers a `Context::on_dispose_sync` cleanup that aborts the task, so owner disposal cancels all timers. `interval` offers a callback form and a `Stream` form; `sleep` is the awaited `timeout`.

**Tech Stack:** Rust 2024, `re-cordis-core`, `tokio` (`time`, `rt`), `futures-util` (`Stream`). Tests use `tokio` `test-util`/`macros` with `pause()`/`advance()`.

**Spec:** `docs/superpowers/specs/2026-10-27-re-cordis-timer-design.md`

## Global Constraints

- Rust edition 2024, MSRV `rust-version = "1.85"`, `version = "0.0.0"`, `publish = false`.
- Add only `crates/re-cordis-timer` to the workspace `members`. No core source change unless a gap is proven (then it is a separate, justified task).
- `unsafe_code = "forbid"`, `missing_docs = "warn"` (inherit `[lints] workspace = true`).
- `tokio` `time` feature belongs to the timer crate, not core. No hidden runtime.
- Deterministic tests only — `tokio::time::pause()`/`advance()`, no wall-clock `sleep`.
- Monotonic clock (`tokio::time::Instant`), not wall-clock.
- Crate name is `re-cordis-timer`; library crate name `re_cordis_timer`.
- Bilingual docs: every English page has a Vietnamese counterpart.
- One commit on `main` at the end of the slice. Do not push. No `Co-authored-by` trailer.
- Verify on BOTH `stable` and `1.85.0` toolchains before the final commit.

---

### Task 1: Crate scaffold + TimerError + sealed trait skeleton

**Files:**
- Create: `crates/re-cordis-timer/Cargo.toml`
- Create: `crates/re-cordis-timer/src/lib.rs`
- Modify: `Cargo.toml` (workspace `members`)
- Modify: `Cargo.toml` (workspace `[workspace.dependencies]`: add `futures-util` is already present; ensure `tokio` time usable)

**Interfaces:**
- Produces: crate `re_cordis_timer`; `pub enum TimerError { Disposed, InvalidDelay, Admission(AdmissionError) }`; `pub trait TimerExt: sealed::Sealed {}` (empty for now); `mod sealed { pub trait Sealed {} impl Sealed for re_cordis_core::Context {} }`.

- [ ] **Step 1: Add the crate to the workspace members**

Modify root `Cargo.toml`:

```toml
[workspace]
members = ["crates/re-cordis-core", "crates/re-cordis-timer"]
exclude = ["spikes/m0"]
resolver = "3"
```

- [ ] **Step 2: Write the crate manifest**

Create `crates/re-cordis-timer/Cargo.toml`:

```toml
[package]
name = "re-cordis-timer"
description = "Owner-scoped timers (timeout, interval, throttle, debounce) for re-cordis"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
publish = false

[dependencies]
re-cordis-core = { path = "../re-cordis-core" }
futures-util.workspace = true
tokio = { workspace = true, features = ["rt", "time"] }

[dev-dependencies]
tokio = { workspace = true, features = ["rt", "time", "macros", "test-util"] }

[lints]
workspace = true
```

- [ ] **Step 3: Write the lib skeleton with TimerError and the sealed trait**

Create `crates/re-cordis-timer/src/lib.rs`:

```rust
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
    /// The requested delay is zero-rejected or exceeds the supported range.
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
pub trait TimerExt: sealed::Sealed {
    // operations added in later tasks
    #[doc(hidden)]
    fn __timer_marker(&self) {}
}

impl TimerExt for Context {}

// re-export for doctests / examples
pub use re_cordis_core;
```

Note: `Duration` and `Effect` imports are used by later tasks; keep them if the compiler warns, otherwise add them when the first operation lands. Prefer adding imports in the task that first uses them to avoid unused-import warnings under `-D warnings`.

- [ ] **Step 4: Build the crate**

Run: `cargo build -p re-cordis-timer`
Expected: compiles clean (remove any unused `Duration`/`Effect` import if it warns; they return in Task 2).

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/re-cordis-timer/
git commit -m "feat(timer): scaffold re-cordis-timer crate with sealed TimerExt and TimerError"
```

---

### Task 2: `timeout(delay, callback)` — fire-once + cancel

**Files:**
- Modify: `crates/re-cordis-timer/src/lib.rs`
- Create: `crates/re-cordis-timer/tests/timeout.rs`

**Interfaces:**
- Consumes: `TimerError`, `TimerExt`, `Context::on_dispose_sync`, `Effect::dispose`.
- Produces: `fn timeout<F: FnOnce() + 'static>(&self, delay: Duration, callback: F) -> Result<Effect, TimerError>` on `TimerExt`.

Design notes for the implementer:
- Core runs on a `tokio::task::LocalSet`; spawn the timer with `tokio::task::spawn_local`. The future is `!Send`, which is correct here.
- The spawned task sleeps `delay` then, if not cancelled, runs the callback once. Guard a shared `Cell<bool>`/`Rc<Cell<...>>` so the dispose cleanup aborts it and the callback never runs after cancel.
- Register `ctx.on_dispose_sync(move || { abort_handle.abort(); Ok(()) })` and return that `Effect`. Disposing the `Effect` (or owner disposal) aborts the task.
- Self-dispose after firing: the TS `dispose()` is called inside the callback path. In Rust, once the task fires it is finished; the registered cleanup aborting an already-finished task is a no-op, which is fine.

- [ ] **Step 1: Write the failing tests**

Create `crates/re-cordis-timer/tests/timeout.rs`:

```rust
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;

// Runtime is NOT Clone and its root context must be used inside `run`.
// Build the runtime inside the harness and hand the async body the root Context.
fn block<F, Fut>(body: F) -> Result<(), BoxError>
where
    F: FnOnce(re_cordis_core::Context) -> Fut,
    Fut: std::future::Future<Output = Result<(), BoxError>>,
{
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                body(root).await
            })
            .await??;
        Ok(())
    })
}

#[test]
fn timeout_fires_once_at_deadline() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        root.timeout(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        assert_eq!(count.get(), 0);
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1);
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1, "fires exactly once");
        Ok(())
    })
}

#[test]
fn timeout_manual_dispose_prevents_fire() -> Result<(), BoxError> {
    block(|root| async move {
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let effect = root.timeout(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        effect.dispose()?;
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 0);
        Ok(())
    })
}
```

VERIFIED core API (read from `crates/re-cordis-core`):
- `Runtime` has only `new()`, `context()`, `run()`, `take_failures()` — it is **not** `Clone`. Build it inside the harness; do not clone it.
- `Context` **is** `#[derive(Clone)]`.
- `Context::plugin(&Plugin<C,R>, config) -> Result<FiberHandle<C>, LifecycleError>`; `FiberHandle::dispose() -> Result<LifecycleTask, _>` is awaited to dispose a child fiber. This is how owner-disposal tests (sleep/stream/throttle created inside a child) are driven: create the op inside `root.plugin(&Plugin::new("probe", |ctx, ()| async move { /* op on ctx */ }), ())`, then `handle.dispose()?.await?`.
- `start_paused(true)` requires the `test-util` dev-dep (set in Task 1).

Service-presence note: SETTLED — **no service**. `TimerExt` methods work on any live `Context` (`on_dispose_sync` + `spawn_local`); there is no `TimerService`/`TimerPlugin` and no `install()` step. Tests call timer methods directly on `root` (or on a child `ctx`). The TS deviation (TS registers a `timer` service) is documented in decision.md/limitation.md.

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p re-cordis-timer --test timeout`
Expected: FAIL to compile — `timeout` not found on `Context`.

- [ ] **Step 3: Implement `timeout`**

In `crates/re-cordis-timer/src/lib.rs`, add the first operation to the `TimerExt` trait and its `impl TimerExt for Context`. No service/plugin registration — the methods work on any live `Context`. Replace the placeholder `__timer_marker` in the trait with the real `timeout` signature.

```rust
use std::future::Future;

fn check_delay(delay: Duration) -> Result<Duration, TimerError> {
    // zero is admitted (fires next turn); reject only unrepresentable ranges
    Ok(delay)
}

impl TimerExt for Context {
    fn timeout<F: FnOnce() + 'static>(
        &self,
        delay: Duration,
        callback: F,
    ) -> Result<Effect, TimerError> {
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
        let guard = fired.clone();
        let effect = self.on_dispose_sync(move || {
            guard.set(true);
            abort.abort();
            Ok::<_, std::convert::Infallible>(())
        })?;
        Ok(effect)
    }
    // ... other methods added later
}
```

The implementer MUST replace the `TimerExt` trait definition's placeholder `__timer_marker` with the real `timeout` signature and add later signatures as those tasks land. Keep the trait and impl in sync.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p re-cordis-timer --test timeout`
Expected: PASS (both tests).

- [ ] **Step 5: Commit**

```bash
git add crates/re-cordis-timer/
git commit -m "feat(timer): add timeout(delay, callback) with owner-scoped cancellation"
```

---

### Task 3: `sleep(delay)` — awaited timeout with disposal rejection

**Files:**
- Modify: `crates/re-cordis-timer/src/lib.rs`
- Create: `crates/re-cordis-timer/tests/sleep.rs`

**Interfaces:**
- Consumes: Task 2 infrastructure.
- Produces: `fn sleep(&self, delay: Duration) -> Sleep` on `TimerExt`; `pub struct Sleep` implementing `Future<Output = Result<(), TimerError>>`.

- [ ] **Step 1: Write the failing tests**

Create `crates/re-cordis-timer/tests/sleep.rs`:

```rust
use std::time::Duration;

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;

// Reuse the same `block` harness shape as tests/timeout.rs.
// (Copy the helper; tests are compiled per-file.)
include!("harness.rs");

#[test]
fn sleep_resolves_once_at_deadline() -> Result<(), BoxError> {
    block(|runtime| async move {
        let root = runtime.context();
        let fut = root.sleep(Duration::from_millis(1000));
        let task = tokio::task::spawn_local(fut);
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
    block(|runtime| async move {
        let root = runtime.context();
        // install the sleep inside a child plugin whose fiber we dispose
        // (mirror the TS 'promise' test disposing ctx.fiber).
        // Verify the future resolves to Err(TimerError::Disposed).
        // Exact child-fiber disposal uses the Plugin/FiberHandle API from
        // lifecycle.rs; assert the Err arm.
        Ok(())
    })
}
```

Create `crates/re-cordis-timer/tests/harness.rs` with the `block` helper extracted from Task 2 (so both test files share it via `include!`). Move the helper there and have `tests/timeout.rs` also `include!("harness.rs")` instead of defining its own, in this task's Step 3.

- [ ] **Step 2: Run to verify failure**

Run: `cargo test -p re-cordis-timer --test sleep`
Expected: FAIL — `sleep`, `Sleep` not found.

- [ ] **Step 3: Implement `Sleep`**

Add to `lib.rs`:

```rust
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};

/// Future returned by [`TimerExt::sleep`].
pub struct Sleep {
    inner: Pin<Box<dyn Future<Output = Result<(), TimerError>>>>,
}

impl Future for Sleep {
    type Output = Result<(), TimerError>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}
```

In the `TimerExt` impl, `sleep` registers an `on_dispose_sync` that flips a shared disposed-flag + wakes the sleeper, and returns `Sleep` whose inner future awaits either the deadline (`Ok(())`) or the disposed-flag (`Err(TimerError::Disposed)`). Fold `check_delay` failure into the first poll (`Err(InvalidDelay)`), per the spec. Extract the shared harness and update `tests/timeout.rs` to `include!("harness.rs")`.

- [ ] **Step 4: Run to verify pass**

Run: `cargo test -p re-cordis-timer`
Expected: PASS (timeout + sleep).

- [ ] **Step 5: Commit**

```bash
git add crates/re-cordis-timer/
git commit -m "feat(timer): add awaited sleep(delay) rejecting on owner disposal"
```

---

### Task 4: `interval(delay, callback)` — repeating + cancel

**Files:**
- Modify: `crates/re-cordis-timer/src/lib.rs`
- Create: `crates/re-cordis-timer/tests/interval_callback.rs`

**Interfaces:**
- Produces: `fn interval<F: FnMut() + 'static>(&self, delay: Duration, callback: F) -> Result<Effect, TimerError>`.

- [ ] **Step 1: Write the failing test**

Create `crates/re-cordis-timer/tests/interval_callback.rs`:

```rust
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;

include!("harness.rs");

#[test]
fn interval_fires_each_period_and_stops_on_dispose() -> Result<(), BoxError> {
    block(|runtime| async move {
        let root = runtime.context();
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let effect = root.interval(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        assert_eq!(count.get(), 0);
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1);
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 2);
        effect.dispose()?;
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 2);
        Ok(())
    })
}
```

- [ ] **Step 2: Run to verify failure**

Run: `cargo test -p re-cordis-timer --test interval_callback`
Expected: FAIL — `interval` not found.

- [ ] **Step 3: Implement `interval` callback form**

Spawn a `tokio::time::interval(delay)` loop via `spawn_local`; first tick at `delay` (call `interval.tick()` once to consume the immediate tick before entering the loop, OR set `MissedTickBehavior::Skip` and skip the leading tick so the first callback is at `delay`, matching the `count==0` before first advance assertion). Register `on_dispose_sync` that aborts the loop. Return the `Effect`.

- [ ] **Step 4: Run to verify pass**

Run: `cargo test -p re-cordis-timer`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/re-cordis-timer/
git commit -m "feat(timer): add interval(delay, callback) with owner-scoped stop"
```

---

### Task 5: `interval_stream(delay)` — Stream form with stop/fail

**Files:**
- Modify: `crates/re-cordis-timer/src/lib.rs`
- Create: `crates/re-cordis-timer/tests/interval_stream.rs`

**Interfaces:**
- Produces: `fn interval_stream(&self, delay: Duration) -> Result<IntervalStream, TimerError>`; `pub struct IntervalStream` implementing `futures_util::Stream<Item = Result<(), TimerError>>` with `fn stop(&self)` and `fn fail(&self, reason: TimerError)`.

- [ ] **Step 1: Write the failing tests**

Create `crates/re-cordis-timer/tests/interval_stream.rs`:

```rust
use std::time::Duration;

use futures_util::StreamExt;
use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::{TimerError, TimerExt};

include!("harness.rs");

#[test]
fn stream_yields_ticks_then_ends_on_stop() -> Result<(), BoxError> {
    block(|runtime| async move {
        let root = runtime.context();
        let mut stream = root.interval_stream(Duration::from_millis(1000))?;
        let handle = stream.clone_control(); // if stop/fail live on a control handle
        let reader = tokio::task::spawn_local(async move {
            let mut seen = 0u32;
            while let Some(item) = stream.next().await {
                match item {
                    Ok(()) => seen += 1,
                    Err(_) => break,
                }
                if seen == 2 { handle.stop(); }
            }
            seen
        });
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(reader.await.unwrap(), 2);
        Ok(())
    })
}

#[test]
fn stream_ends_with_disposed_on_owner_dispose() -> Result<(), BoxError> {
    block(|runtime| async move {
        let root = runtime.context();
        // install the stream in a child plugin, dispose its fiber, assert the
        // final item is Err(TimerError::Disposed) then None. Uses the
        // Plugin/FiberHandle API from lifecycle.rs.
        Ok(())
    })
}
```

NOTE: the `clone_control()` detail is a design choice for the implementer: `stop`/`fail` may be methods on `IntervalStream` directly (if the test drives them between polls) or on a cloneable control handle (if driven concurrently). Pick whichever keeps `IntervalStream: Stream` clean and document it. Keep the public method names `stop`/`fail`.

- [ ] **Step 2: Run to verify failure**

Run: `cargo test -p re-cordis-timer --test interval_stream`
Expected: FAIL — `interval_stream` not found.

- [ ] **Step 3: Implement `IntervalStream`**

Back the stream with a `tokio::sync::mpsc` (or a `tokio::time::Interval` polled directly in `poll_next`). Ticks yield `Ok(())`. A shared terminal state (`None` after `stop`; `Some(Err(reason))` once then `None` after `fail`; `Some(Err(Disposed))` once then `None` on owner dispose). Register `on_dispose_sync` to set the disposed terminal state and wake the stream. Implement `futures_util::Stream`.

- [ ] **Step 4: Run to verify pass**

Run: `cargo test -p re-cordis-timer`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/re-cordis-timer/
git commit -m "feat(timer): add interval_stream(delay) Stream with stop/fail/dispose termination"
```

---

### Task 6: `throttle` — leading edge + optional trailing

**Files:**
- Modify: `crates/re-cordis-timer/src/lib.rs`
- Create: `crates/re-cordis-timer/tests/throttle.rs`

**Interfaces:**
- Produces: `fn throttle<F: FnMut() + 'static>(&self, delay: Duration, no_trailing: bool, callback: F) -> Result<Scheduled, TimerError>`; `pub struct Scheduled` with `fn call(&self)` and `fn dispose(self) -> Result<(), TimerError>` (dispose consumes or uses `&self` + internal flag — pick one and keep consistent with debounce).

- [ ] **Step 1: Write the failing tests**

Create `crates/re-cordis-timer/tests/throttle.rs` mirroring TS cases #257 (basic: leading fires, within-window suppressed, trailing fires), #272 (trailing mode), #286 (no-trailing), #300 (owner dispose stops), #320 (manual dispose). Use monotonic `advance`. Example of the basic case:

```rust
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;

include!("harness.rs");

#[test]
fn throttle_leading_and_trailing() -> Result<(), BoxError> {
    block(|runtime| async move {
        let root = runtime.context();
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let throttled = root.throttle(Duration::from_millis(1000), false, move || c.set(c.get() + 1))?;
        throttled.call();
        assert_eq!(count.get(), 1);               // leading
        tokio::time::advance(Duration::from_millis(600)).await;
        throttled.call();
        assert_eq!(count.get(), 1);               // within window, suppressed
        tokio::time::advance(Duration::from_millis(600)).await;
        tokio::task::yield_now().await;
        throttled.call();
        assert_eq!(count.get(), 2);
        tokio::time::advance(Duration::from_millis(2000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 3);               // trailing from the 3rd call
        Ok(())
    })
}
```

Add the remaining four cases (trailing-mode, no-trailing true, owner dispose via child fiber, manual `dispose()` before any call → never fires).

- [ ] **Step 2: Run to verify failure**

Run: `cargo test -p re-cordis-timer --test throttle`
Expected: FAIL — `throttle`/`Scheduled` not found.

- [ ] **Step 3: Implement `Scheduled` + `throttle`**

Port the TS `_schedule` + `throttle` logic using monotonic `tokio::time::Instant::now()` for `last_call`, and `tokio::time::sleep` (via `spawn_local`) for the trailing timer. `Scheduled` holds an `Rc` of mutable state (`last_call`, current trailing task abort handle, `disposed` flag) and the `on_dispose_sync` `Effect`. `call()` replicates: if `remaining <= 0` execute immediately and set `last_call`; else if `!no_trailing` (re)arm a trailing `sleep(remaining)` that executes and sets `last_call`. `dispose()` aborts the trailing task and sets `disposed`; owner dispose does the same via the registered cleanup.

- [ ] **Step 4: Run to verify pass**

Run: `cargo test -p re-cordis-timer`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/re-cordis-timer/
git commit -m "feat(timer): add throttle(delay, no_trailing, cb) with monotonic window"
```

---

### Task 7: `debounce` — trailing edge coalescing

**Files:**
- Modify: `crates/re-cordis-timer/src/lib.rs`
- Create: `crates/re-cordis-timer/tests/debounce.rs`

**Interfaces:**
- Produces: `fn debounce<F: FnMut() + 'static>(&self, delay: Duration, callback: F) -> Result<Scheduled, TimerError>` (reuses `Scheduled`).

- [ ] **Step 1: Write the failing tests**

Create `crates/re-cordis-timer/tests/debounce.rs` mirroring TS #345 (coalesce: repeated calls within window → one fire after final `delay`), #360 (manual dispose → never fires), #370 (owner dispose → never fires, and calls after dispose still never fire).

```rust
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;

include!("harness.rs");

#[test]
fn debounce_coalesces_to_trailing_edge() -> Result<(), BoxError> {
    block(|runtime| async move {
        let root = runtime.context();
        let count = Rc::new(Cell::new(0u32));
        let c = count.clone();
        let debounced = root.debounce(Duration::from_millis(1000), move || c.set(c.get() + 1))?;
        debounced.call();
        assert_eq!(count.get(), 0);
        tokio::time::advance(Duration::from_millis(400)).await;
        debounced.call();
        assert_eq!(count.get(), 0);
        tokio::time::advance(Duration::from_millis(400)).await;
        debounced.call();
        assert_eq!(count.get(), 0);
        tokio::time::advance(Duration::from_millis(1000)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.get(), 1);
        Ok(())
    })
}
```

Add the two dispose cases.

- [ ] **Step 2: Run to verify failure**

Run: `cargo test -p re-cordis-timer --test debounce`
Expected: FAIL — `debounce` not found.

- [ ] **Step 3: Implement `debounce`**

Reuse `Scheduled`. `call()` aborts any pending trailing task and arms a fresh `sleep(delay)` that fires the callback. `dispose()`/owner dispose aborts the pending task and sets `disposed` so later `call()`s are inert.

- [ ] **Step 4: Run to verify pass**

Run: `cargo test -p re-cordis-timer`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/re-cordis-timer/
git commit -m "feat(timer): add debounce(delay, cb) trailing-edge coalescing"
```

---

### Task 8: Runnable example

**Files:**
- Create: `crates/re-cordis-timer/examples/timer_lifecycle.rs`

**Interfaces:**
- Consumes: the full `TimerExt` surface.

- [ ] **Step 1: Write the example**

Create `crates/re-cordis-timer/examples/timer_lifecycle.rs` following the structure of `crates/re-cordis-core/examples/lifecycle.rs` (a `run()` returning `Result<(), BoxError>` and a `main` calling it). It must: build a current-thread runtime, schedule a `timeout`, an `interval` (take 3 ticks via `interval_stream`), a `throttle` and a `debounce`, then dispose the owner and show no further callbacks fire. Use real (short) durations here (this is an example, not a test) OR drive it deterministically — prefer deterministic with `start_paused` + `advance` so it is reproducible and fast.

- [ ] **Step 2: Run the example**

Run: `cargo run -p re-cordis-timer --example timer_lifecycle`
Expected: exits 0, prints the lifecycle trace.

- [ ] **Step 3: Commit**

```bash
git add crates/re-cordis-timer/examples/
git commit -m "docs(timer): add runnable timer_lifecycle example"
```

---

### Task 9: Rustdoc on every public item + doc build

**Files:**
- Modify: `crates/re-cordis-timer/src/lib.rs`

- [ ] **Step 1: Ensure every public item has a doc comment**

`missing_docs = "warn"` is inherited. Add `///` docs to `TimerError` (and variants), `TimerExt` (and every method), `Sleep`, `IntervalStream` (+ `stop`/`fail`), `Scheduled` (+ `call`/`dispose`). Each method doc states: what fires, when, ownership/cancellation, and error conditions.

- [ ] **Step 2: Build docs warnings-denied**

Run: `RUSTDOCFLAGS="-D warnings" cargo doc -p re-cordis-timer --no-deps`
Expected: clean, no missing-docs warnings.

- [ ] **Step 3: Commit**

```bash
git add crates/re-cordis-timer/src/lib.rs
git commit -m "docs(timer): document all public timer items"
```

---

### Task 10: English docs (API + guide)

**Files:**
- Create: `docs/en/api/timer.md`
- Modify: `docs/en/api/README.md`
- Create: `docs/en/guides/timers.md`
- Modify: `docs/en/README.md` (link the new guide/api pages if it lists them)

**Interfaces:**
- The doc-checker (Task 12) requires: each timer public export appears in `docs/en/api/README.md` (indexed) AND in some `docs/en/api/*.md` chapter (covered). Any ` ```rust ` block must be a complete runnable program.

- [ ] **Step 1: Write `docs/en/api/timer.md`**

Document `TimerExt` (`timeout`, `sleep`, `interval`, `interval_stream`, `throttle`, `debounce`), `TimerError`, `Sleep`, `IntervalStream`, `Scheduled`. Include at least one complete runnable `rust` snippet (a full `fn main()` program that uses `timeout` under a paused runtime, asserting a fire). The snippet will be compiled+run by the checker against `re_cordis_timer` + `tokio`.

- [ ] **Step 2: Index the page in `docs/en/api/README.md`**

Add a row/link for the timer chapter and ensure every timer export name appears as `` `Name` `` in the index text.

- [ ] **Step 3: Write `docs/en/guides/timers.md`**

A task-oriented guide: owner-scoped timing, cancellation, the Stream vs callback choice, and the monotonic-clock/deterministic-testing note. Prose may reference APIs; any rust block must be complete + runnable.

- [ ] **Step 4: Commit**

```bash
git add docs/en/
git commit -m "docs(timer): add English timer API reference and guide"
```

---

### Task 11: Vietnamese docs (mirror of Task 10)

**Files:**
- Create: `docs/vi/api/timer.md`
- Modify: `docs/vi/api/README.md`
- Create: `docs/vi/guides/timers.md`
- Modify: `docs/vi/README.md`

- [ ] **Step 1: Mirror each English page in Vietnamese**

Translate the prose; keep code snippets identical to the English ones (the checker shares snippets across both languages — see `check.py` "both languages"). Ensure the same export names appear as `` `Name` `` in `docs/vi/api/README.md` and a `docs/vi/api/*.md` chapter.

- [ ] **Step 2: Commit**

```bash
git add docs/vi/
git commit -m "docs(timer): add Vietnamese timer API reference and guide"
```

---

### Task 12: Extend the doc-checker for the timer crate

**Files:**
- Modify: `tests/docs/check.py`

Design notes: currently `check_structure()` reads exports only from `re-cordis-core/src/lib.rs` (line 94) and `run_examples()` builds only `re-cordis-core` and asserts libraries `== {re_cordis_core, tokio}` (line 160), linking snippets with those two externs. The timer slice needs: (a) timer exports indexed+covered in the timer api pages, and (b) timer snippets compiled against `re_cordis_timer` too.

- [ ] **Step 1: Read exports from both crates**

In `check_structure()`, after building `exports` from core, also parse `crates/re-cordis-timer/src/lib.rs` for `pub use`, `pub trait`, `pub struct`, `pub enum`, `pub fn` public names, into a `timer_exports` set. Require each timer export is indexed in `docs/<lang>/api/README.md` and covered in some `docs/<lang>/api/*.md` chapter — reuse the existing index/chapter loop, scoped so core exports may live in core chapters and timer exports in the timer chapter (simplest: require each timer export appears somewhere in the combined chapters text, same as core).

- [ ] **Step 2: Build and link the timer crate for snippets**

In `run_examples()`, add a second `cargo build -p re-cordis-timer --lib ... --message-format=json` pass, collect its `re_cordis_timer` rlib (and any new transitive rlibs it needs: `futures_util`), and extend the extern/`-L` lists so snippets can `use re_cordis_timer::...`. Relax the `require(set(libraries) == {...})` assertion to a superset check that includes `re_cordis_timer` when timer snippets are present. Keep core-only behavior working.

- [ ] **Step 3: Run the checker on stable**

Run: `python3 -B tests/docs/check.py --toolchain stable`
Expected: passes, reports timer exports checked and timer snippets executed.

- [ ] **Step 4: Run the structural self-test**

Run: `python3 -B tests/docs/test_check.py`
Expected: passes (add/adjust a regression if `test_check.py` asserts the crate list; keep its existing assertions green).

- [ ] **Step 5: Commit**

```bash
git add tests/docs/
git commit -m "test(docs): extend checker to cover re-cordis-timer exports and snippets"
```

---

### Task 13: llms.txt + project docs (status/changelog/decision/limitation/AGENTS/todo)

**Files:**
- Modify: `docs/llms.txt`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `docs/todo.md`
- Modify: `docs/decision.md`
- Modify: `docs/limitation.md`
- Modify: `AGENTS.md`

- [ ] **Step 1: Update `docs/llms.txt`**

Add a timer reading-path entry under the appropriate H2 section (format must match the checker: `- [label](path): description`, keep file under 8 KiB, exactly one H1).

- [ ] **Step 2: Update `README.md`**

Add `re-cordis-timer` to the status/layout matrices as the first delivered satellite (with its row-specific accuracy/limits, no unmeasured performance claims). Add the crate to the planned/actual layout.

- [ ] **Step 3: Update `CHANGELOG.md`**

Add an entry (Keep a Changelog style) for the timer satellite.

- [ ] **Step 4: Update `docs/todo.md`**

Check off the re-cordis-timer boxes under "1. re-cordis-timer" that are now done; keep logger/loader unchecked. Note timer as delivered in the satellite section; do not claim logger/loader.

- [ ] **Step 5: Update `docs/decision.md` with the four locked decisions AND their rationale**

Record, each with the *why* (per the user's explicit request to capture explanations, not just choices):
1. interval(delay) → Stream + callback form. Why: Stream is the idiomatic Rust async-sequence type and matches what the independent cordis-rs port shipped; the JS async-iterator return-to-all/throw-to-all protocol is an iterator-protocol artifact better served by owner disposal + the callback form. Dropped cases documented in limitation.md.
2. Ergonomics → sealed `TimerExt` trait on `Context`. Why: closest to the TS `ctx.timeout()` mixin intent and the one existing Rust precedent (cordis-rs sealed TimerExt); sealed so it is not an open extension point; each method returns `Result` for the not-installed case. (Revised from an initial explicit-handle lean after reviewing cordis-rs.)
3. Deprecated setTimeout/setInterval aliases → omitted. Why: deprecated upstream; porting deprecated surface adds no value. Documented in limitation.md.
4. Clock → monotonic `tokio::time` with deterministic pause/advance tests. Why: it is the clock the LocalSet host drives; monotonic is more correct than wall-clock Date.now() for throttle/debounce windows; enables deterministic tests with no wall-clock sleeps.

- [ ] **Step 6: Update `docs/limitation.md`**

Document: omitted deprecated aliases; the JS async-iterator concurrent-read return-to-all/throw-to-all protocol not reproduced (Stream has one logical consumer); monotonic vs wall-clock timing; no `Context` auto-mixin (explicit sealed trait).

- [ ] **Step 7: Update `AGENTS.md`**

Update the status paragraph: timer satellite delivered (unpublished `crates/re-cordis-timer`, Tokio `time` feature in the timer crate); next targets logger then loader. Keep prior milestone evidence as historical.

- [ ] **Step 8: Commit**

```bash
git add docs/ README.md CHANGELOG.md AGENTS.md
git commit -m "docs(timer): record timer delivery, decisions, limitations across project docs"
```

---

### Task 14: Final full verification (BOTH toolchains) + squash-free review

**Files:** none (verification only)

- [ ] **Step 1: Format check**

Run: `cargo fmt --all --check`
Expected: no diffs. (If it reports diffs, run `cargo fmt --all` and amend the relevant commit.)

- [ ] **Step 2: Strict Clippy, all targets, warnings denied**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: clean.

- [ ] **Step 3: Workspace tests**

Run: `cargo test --workspace`
Expected: all green — the new timer tests PLUS all pre-existing core/application tests (the six paired lifecycle scenarios and existing observations preserved).

- [ ] **Step 4: Warnings-denied Rustdoc, whole workspace**

Run: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
Expected: clean.

- [ ] **Step 5: Doc checker on BOTH toolchains**

Run:
```bash
python3 -B tests/docs/test_check.py
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
```
Expected: all pass. (Requires `rustup` toolchains `stable` and `1.85.0` installed; if `1.85.0` is missing, run `rustup toolchain install 1.85.0` first.)

- [ ] **Step 6: MSRV build of the timer crate**

Run: `cargo +1.85.0 build -p re-cordis-timer`
Expected: compiles on the MSRV.

- [ ] **Step 7: Review the full diff for surgical scope**

Run: `git log --oneline origin/main..HEAD && git diff --stat origin/main..HEAD`
Confirm: only `crates/re-cordis-timer/**`, workspace `Cargo.toml`, `tests/docs/check.py` (+ maybe `test_check.py`), and the doc files were touched. No unrelated core source change. If any orphan/unused item was introduced by these changes, remove it; do not touch pre-existing unrelated code.

- [ ] **Step 8: Report**

Summarize: tests passing (count), both toolchains green, files changed, decisions recorded. Do NOT push.

---

## Self-Review

**Spec coverage:** Every spec section maps to a task — API (T2–T7), clock/determinism (all test steps; T13 decision), ownership/cancellation (every op's dispose step), exclusions (T13 limitation.md), example (T8), rustdoc (T9), bilingual docs (T10/T11), checker extension (T12), acceptance checklist (T13/T14). Covered.

**Placeholder scan:** The `todo!()` in Task 2 Step 3 and the "adapt to the real core API" notes are deliberate — they flag the ONE unknown (exact `Plugin`/`provide`/`Runtime` spelling) that the implementer must read from `lifecycle.rs` rather than me guessing a signature that may not exist. Every test body is concrete. The `clone_control()` and child-fiber-dispose notes name the real API to consult. These are guided decisions, not vague placeholders.

**Type consistency:** `Scheduled` (T6) is reused by T7. `TimerError`, `TimerExt`, `Sleep`, `IntervalStream`, `Scheduled` names are consistent across tasks. The shared test `harness.rs` is introduced in T3 and reused via `include!` by T3–T7 (T2 defines its own `block`, then T3 Step 3 extracts it — noted explicitly).

**Verified core API (no guesses remain):** `Runtime` is not `Clone` (build it inside the harness); `Context` is `Clone`; `Context::plugin(&Plugin, config) -> FiberHandle`, `FiberHandle::dispose()?.await?` drives child-fiber disposal for owner-dispose tests; `on_dispose_sync` returns the `Effect`. No service registration is used. All confirmed against `crates/re-cordis-core` source and `examples/lifecycle.rs`.
