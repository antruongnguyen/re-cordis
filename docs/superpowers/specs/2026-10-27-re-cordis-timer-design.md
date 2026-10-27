# re-cordis-timer — Design

Date: 2026-10-27
Status: Approved for implementation (bounded design gate passed)
Scope: First satellite crate. Timer only. Not logger, not loader, not M7.

## 1. Purpose

Port the TypeScript `cordis` **timer** satellite
(`packages/timer/src/index.ts`, pinned at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`) to an unpublished Rust crate
`crates/re-cordis-timer`, mirroring its **semantics** on top of existing
`re-cordis-core` effects and lifecycle.

The TypeScript project is the specification. The independent
`local/cordis-rs` port is **reference only** — its contracts are not our spec,
though we use it as evidence of what is workable in Rust.

## 2. Source review (what the spec actually requires)

`TimerService extends Service` mixes six methods onto `Context`. Every
operation wraps a JS `setTimeout`/`setInterval` inside
`ctx.effect(() => { …; return () => clearTimeout(timer) })`, so **owner
disposal cancels the timer**. The 20 test cases
(`packages/timer/tests/index.spec.ts`) are the behavioral source of truth.

| TS API | Behavior established by tests |
|---|---|
| `timeout(cb, delay)` | fire once at deadline, then self-dispose; returns `dispose()`; not called after manual dispose (#22, #32). |
| `timeout(delay)` → `Promise` | resolves at deadline; on context dispose before firing, **rejects** `"Context has been disposed"`; resolves exactly once (#41). |
| `interval(cb, delay)` | fire every `delay`; returns `dispose()`; stops after dispose (#59). |
| `interval(delay)` → async iterator | `next/return/throw` + `Symbol.asyncIterator`; concurrent `next()` reads resolve FIFO; `return(v)` → all pending `{done:true,value:v}`; `throw(r)`/context-dispose → all pending reject; no ticks after done (#72–#253). |
| `throttle(cb, delay, noTrailing?)` | leading call fires immediately; `Date.now()`-based remaining window; optional trailing call; `.dispose()` on the returned wrapper; owner dispose stops it (#257–#341). |
| `debounce(cb, delay)` | coalesce to trailing edge; each call resets; `.dispose()`; owner dispose stops it (#345–#386). |
| `setTimeout` / `setInterval` | **deprecated** aliases → `timeout`/`interval` callback forms. |

## 3. Core facts this builds on (verified in `re-cordis-core`)

- `Context::on_dispose_sync(FnOnce -> Result) -> Result<Effect, AdmissionError>`
  is the direct analogue of `ctx.effect(() => cleanup)`. `Effect::dispose()`
  runs the cleanup; owner/fiber disposal runs it too. This is exactly the TS
  `ctx.effect` disposal semantics the timer relies on.
- The runtime host is a `tokio::task::LocalSet` (single-thread, `!Send`,
  `Rc`/`LocalFuture`). Timer operations schedule on that same local runtime.
- Core's `tokio` features are `["rt","sync"]` only — **no `time`**. The timer
  crate adds `tokio` with the `time` feature itself. No core change.
- There is no JS `mixin`/Proxy/callable-object. Methods become explicit Rust
  methods exposed via a sealed extension trait (an M5-documented deviation
  class).

## 4. Public Rust API

New unpublished crate `crates/re-cordis-timer` (Rust 2024, MSRV 1.85,
`version = "0.0.0"`, `publish = false`). A `TimerService` is published under a
`ServiceKey` by a `Plugin`; consumers reach operations through a **sealed**
extension trait on `Context`:

```rust
pub trait TimerExt: sealed::Sealed {
    /// Fire `callback` once after `delay`, then self-dispose.
    /// Returns an Effect; dispose to cancel early.
    fn timeout<F>(&self, delay: Duration, callback: F)
        -> Result<Effect, TimerError>
    where F: FnOnce() + 'static;

    /// Resolve after `delay`. The returned future yields
    /// Err(TimerError::Disposed) if the owner tears down before the deadline,
    /// Err(TimerError::InvalidDelay) if the delay was rejected at construction,
    /// and Ok exactly once otherwise. Registration failure is folded into the
    /// future's first poll so callers have a single await point (unlike the
    /// callback forms, which fail synchronously at registration).
    fn sleep(&self, delay: Duration) -> Sleep; // Sleep: Future<Output = Result<(), TimerError>>

    /// Fire `callback` every `delay`. Returns an Effect; dispose to stop.
    fn interval<F>(&self, delay: Duration, callback: F)
        -> Result<Effect, TimerError>
    where F: FnMut() + 'static;

    /// A Stream of ticks every `delay`. Ends (yields None) on owner disposal
    /// or when the handle is dropped. Also exposes stop()/fail() terminal
    /// controls (the return/throw analogues).
    fn interval_stream(&self, delay: Duration)
        -> Result<IntervalStream, TimerError>;

    /// Leading-edge rate limit; optional trailing call. The returned handle
    /// is callable (`.call()`) and `.dispose()`-able; owner dispose stops it.
    fn throttle<F>(&self, delay: Duration, no_trailing: bool, callback: F)
        -> Result<Scheduled, TimerError>
    where F: FnMut() + 'static;

    /// Trailing-edge coalescing; each call resets the window.
    fn debounce<F>(&self, delay: Duration, callback: F)
        -> Result<Scheduled, TimerError>
    where F: FnMut() + 'static;
}
```

- `IntervalStream` implements `futures_util::Stream<Item = Result<(), TimerError>>`.
  Normal ticks yield `Ok(())`. `stop()` ends it cleanly (next poll → `None`);
  `fail(reason)` yields one final `Err(reason)` then `None`; owner disposal
  yields one final `Err(TimerError::Disposed)` then `None`. Concurrent
  multi-reader `next()` semantics from the JS iterator are **not** reproduced
  (see Limitations).
- `Scheduled` has `call()` (invoke) and `dispose()`. The TS returned object is
  a callable with a `.dispose` property; Rust uses an explicit `.call()`.
- `sealed::Sealed` is implemented only for `re_cordis_core::Context`, so the
  trait is not an open extension point.

## 5. Ownership and cancellation

Every operation registers an `on_dispose_sync` cleanup that aborts its Tokio
timer task. Therefore:

- **Owner/context disposal** cancels every outstanding timer (matches TS
  `ctx.effect` teardown). No callback fires after joined disposal.
- **Manual cancel** = `Effect::dispose()` / `Scheduled::dispose()` /
  `IntervalStream::stop()`.
- **Awaited `sleep`** returns `Err(TimerError::Disposed)` if the owner tears
  down before the deadline (mirrors the TS promise rejection), else `Ok(())`
  exactly once.

## 6. Clock decision (the one real semantic fork)

TS throttle/debounce use **wall-clock `Date.now()`**; tests use fake timers.
We use **`tokio::time`** (monotonic `Instant`), the clock the local runtime
drives. Tests are deterministic via `tokio::time::pause()` + `advance()` (the
Tokio analogue of `vi.useFakeTimers` / `advanceTimersByTimeAsync`) — **no
wall-clock sleeps**. Throttle/debounce windows are therefore monotonic, not
wall-clock. Documented in `limitation.md` as a deliberate, more-correct fork.

### Behavior definitions (resolve before coding)
- **Zero delay:** admitted; fires on the next runtime turn (does not reject).
- **Overflow:** delays beyond `Duration`/Tokio range are rejected at
  construction with `TimerError::InvalidDelay`, not silently clamped.
- **First vs subsequent interval tick:** first tick at `delay` (not at 0),
  matching TS (#59 asserts 0 calls before the first `delay`).
- **Missed ticks:** coalesce without shifting phase (Tokio
  `MissedTickBehavior::Skip`), consistent with a single JS `setInterval`.
- **Callback panic:** isolated to the timer task; surfaced via the core logger
  path, does not poison the runtime. Documented.
- **Once-only `timeout`:** self-dispose on fire guarantees exactly one call.

## 7. Scope / exclusions (YAGNI)

**Included:** `timeout` (callback + awaited), `interval` (callback + stream),
`throttle`, `debounce`, owner-scoped cancellation, deterministic clock tests, a
runnable example, bilingual docs.

**Excluded, documented as limitations:**
- Deprecated `setTimeout` / `setInterval` aliases — intentionally omitted.
- The JS async-iterator `return(value)`-to-all-concurrent-readers and
  `throw(reason)`-to-all protocol — a Stream has one logical consumer; mapped
  to stream end / terminal-error and documented.
- Any `Context` auto-mixin magic — replaced by the explicit sealed trait.

No core change unless a gap is proven; any such gap is a separately justified,
targeted fix.

## 8. Testing (exit evidence)

Deterministic tests over `tokio::time::pause/advance`, mirroring the TS cases
minus the excluded alias/iterator-exact ones (~15–18 tests):

- timeout: just-before/at-deadline fire-once; manual dispose; owner dispose
  mid-flight; awaited sleep resolves once; awaited sleep rejects on owner
  dispose.
- interval: tick counts at successive deadlines; stop after dispose; owner
  dispose stops ticks; `interval_stream` yields `Ok(())` ticks then a final
  `Err(Disposed)`+`None` on owner dispose; `stop()` → clean `None`; `fail()` →
  final `Err(reason)`+`None`.
- throttle: leading fire; trailing within window; `no_trailing`; `.dispose()`;
  owner dispose.
- debounce: coalesce to trailing; reset on each call; `.dispose()`; owner
  dispose.
- no callback after joined disposal (the cross-cutting core guarantee).

## 9. Acceptance checklist (shared, from docs/todo.md)

- `cargo test -p re-cordis-timer` green; `cargo test --workspace` still green;
  the six paired lifecycle scenarios and existing observations preserved.
- Strict Clippy, `cargo fmt`, warnings-denied Rustdoc — on **stable and
  1.85.0** with explicit toolchain paths and isolated artifacts.
- Runnable `crates/re-cordis-timer/examples/timer_lifecycle.rs`.
- English + Vietnamese guide/API pages; `docs/llms.txt`; extend
  `tests/docs/check.py` for the new crate's exports/snippets.
- Update `README.md` status/layout/star matrices, `CHANGELOG.md`,
  `docs/todo.md`, `docs/decision.md`, `docs/limitation.md`, `AGENTS.md`.
- Add only `crates/re-cordis-timer` to the workspace; keep MSRV 1.85 and
  `publish = false`.
- One commit on `main`; do not push.

## 10. Decisions locked at the gate

1. interval(delay) → **Stream + callback form**; JS-iterator-exact cases
   documented as limitations.
2. Consumer ergonomics → **sealed `TimerExt` trait** on `Context`.
3. Deprecated aliases → **omitted + documented**.
4. Clock → **`tokio::time` monotonic**, deterministic tests via pause/advance.
