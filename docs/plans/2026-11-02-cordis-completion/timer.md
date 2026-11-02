# Timer repair implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Make the existing timer meet its reviewed admission, timing, resource
ownership and diagnostic contracts before relying on it in later satellites.
**Architecture:** Preserve sealed TimerExt and owner-scoped effects. Repair
operation ownership within the timer, using B2 only for a proven core bridge.
**Tech stack:** Rust 2024, Tokio time, existing core and futures-util.
**Spec:** [Master design](README.md), the historical
[timer design](../../superpowers/specs/2026-10-27-re-cordis-timer-design.md),
and the decisions produced by T0. T0 resolves historical contradictions.

## Global constraints

Rust edition 2024; resolver 3; minimum Rust 1.85, tested with 1.85.0. MIT;
version 0.0.0 and publish=false. Preserve existing method names unless a
reviewed contract change requires otherwise. Keep time features in timer.

## Source observations and investigation register

Locations below refer to crates/re-cordis-timer/src/lib.rs at `808c9f5`. These
are source findings, not claims that new reproductions have been executed.

| ID | Evidence | Required outcome / reproduction |
|---|---|---|
| TF1 | check_delay, lines 49–53, always returns Ok; interval paths use unchecked Instant addition and interval_at | Unrepresentable deadlines return InvalidDelay; zero never becomes a detached-task panic; test all six operations |
| TF2 | timeout/interval/interval_stream spawn before on_dispose_sync, lines 139/178/205 | Failed admission leaves no running callback/task/resource; prove outside host, wrong host and stale child Context |
| TF3 | timeout, lines 132–154, never unregisters after firing | Repeated completed timeouts restore effect baseline; self-dispose occurs before user callback as upstream does |
| TF4 | Sleep::new, lines 477–486, uses drop(effect); core Effect Drop is inert | Completion and dropped waiter policies release exactly the intended registration; owner cancellation remains observable |
| TF5 | IntervalControl stop/fail only close sender; stream is an unbounded channel with no explicit Drop cleanup | Decide queued-tick policy; terminal/drop cleanup stops and joins producer promptly and bounds memory |
| TF6 | Raw spawn_local tasks lack core HOST scope; discarded JoinHandles hide task outcomes | Timer callbacks can perform the reviewed Context operations and failures reach the selected core diagnostic path exactly once |
| TF7 | Deadlines are constructed inside spawned/polled futures | Decide admission-time deadline versus first-poll delay; delayed polling must not silently extend the declared timer |
| TF8 | Scheduled executes user callbacks while holding RefCell mutable state; cleanup borrows the same state | Self-disposal and supported reentry do not panic with BorrowMutError or corrupt pending scheduling |
| TF9 | Cleanup aborts tasks but does not await task completion | Prove captured resources are released before the promised joined owner barrier; no callback begins afterward |
| TF10 | Scheduled state retains boxed callback after dispose; stream terminal state and owner cleanup can race | Assert capture release and once-only terminal behavior with Drop probes, not just callback counts |

TF1–TF8 have direct source evidence; the exact runtime symptoms and TF9/TF10
resource timing require controlled tests. The 15 existing happy-path/cancel
tests do not settle these questions.

## T0 — Resolve contracts and map all 22 upstream declarations

**Files:** this plan, docs/decision.md, docs/limitation.md; read upstream
packages/timer/src/index.ts and tests/index.spec.ts and the existing six Rust
timer test files.

- [ ] Classify each declaration in [the inventory](source-inventory.md):
  callback timeout/interval, awaited timeout, nine iterator cases, six throttle
  cases and three debounce cases. Preserve documented omitted aliases and
  multi-reader iterator differences.
- [ ] Resolve zero for each operation; recommend an explicit minimum positive
  repeat period and asynchronous one-shot zero. Decide exact minimum, overflow
  boundary and rejection precedence before writing tests.
- [ ] Pin start time at successful registration; distinguish an admitted Sleep
  whose polling is delayed from a future that has never been admitted.
- [ ] Define Stream backpressure: recommend no unbounded tick history and at
  most one coalesced pending tick; document the difference from TS, which only
  serves pending next() calls. Terminal state takes precedence over stale ticks.
- [ ] Resolve Drop separately for Effect handles, Sleep, IntervalStream, and
  Scheduled. Keep core's inert Drop; operation wrappers may cancel their own
  wait/producer only under an explicitly documented policy.
- [ ] Define a valid Cordis host whose Tokio time driver is disabled: choose
  an explicit environment error or a documented construction precondition,
  and test it without hiding a detached-task panic.
- [ ] Define panic/reentry and whether manual synchronous disposal merely
  initiates cancellation while owner completion joins captured-resource release.

**Exit:** accepted truth table for every operation × admission/fire/manual
cancel/owner cancel/drop/panic, with exact errors and terminal outcomes.

## T1 — Admission, range and clock boundaries

**Files:** src/lib.rs; create tests/admission.rs and tests/deadlines.rs in
crates/re-cordis-timer; reuse tests/common/mod.rs.
**Consumes:** T0 truth table and Context admission. **Produces:** no orphan
work on refusal, validated deadline/period, admission-time clock anchors.

- [ ] Add failing tests for Duration::MAX, zero, just-before/at deadline, late
  first polling, missed intervals, wrong/non-running host, disabled time
  driver and stale owners.
- [ ] Add a callback that registers on its Context to identify the HOST bridge
  requirement; route that failure to B2/T2 instead of weakening admission.
- [ ] Run `cargo test -p re-cordis-timer --test admission --test deadlines --locked`
  and retain the expected failures.
- [ ] Validate and admit before spawning. Use checked monotonic deadline
  arithmetic; rollback any acquired work if registration cannot commit.
- [ ] Make these targeted tests pass, document policies, and commit.

## T2 — Hosted ownership, one-shot cleanup and joined cancellation

**Files:** src/lib.rs; tests/timeout.rs, tests/sleep.rs; create
tests/ownership.rs. Core changes only through B2.

- [ ] Write regressions that count live effects before/after repeated completed
  timeouts and sleeps, cancelled/dropped waits and plugin restart.
- [ ] Use captured Drop probes plus a cleanup gate to distinguish abort request
  from actual task/resource release. Await owner disposal and assert releases
  occurred, no new callbacks execute and the Registry/effect baseline is restored.
- [ ] Prove callbacks can register nested timers/effects under the correct
  owner; wrong-host work remains rejected. Preserve explicit cancellation of
  a timeout before its callback is invoked.
- [ ] Repair hosted task ownership and exact terminal unregistration; use
  B2's reviewed bridge if necessary. Never use drop(Effect) as unregister.
- [ ] Verify manual effect transfer/composition still works and does not
  double-dispose a self-completed timer. Test dropped completion waiters.
- [ ] Run ownership/timeout/sleep tests and affected core tests, then commit.

Concrete first regression, to append to the existing timeout.rs (its imports
and block harness already supply every symbol):

```rust
#[test]
fn completed_timeout_releases_registration() -> Result<(), BoxError> {
    block(|root| async move {
        let baseline = root.get_effects()?.len();
        root.timeout(Duration::from_millis(10), || {})?;
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;
        assert_eq!(root.get_effects()?.len(), baseline);
        Ok(())
    })
}
```

Run `cargo test -p re-cordis-timer --test timeout completed_timeout_releases_registration --locked`.
At the inspected revision the registration is retained; verify that expected
failure before repair. The snippet is a planned regression, not a reported run.

## T3 — Interval stream terminal behavior and bounded retention

**Files:** src/lib.rs, tests/interval_callback.rs, tests/interval_stream.rs.

- [ ] Add tests with no reader across many ticks, delayed readers, stop before
  first tick, fail/stop/dispose in both orders, repeated terminal calls and drop.
- [ ] Assert T0's exact queued-tick/terminal policy, bounded pending state,
  producer cancellation, capture release and restored live effects.
- [ ] Replace unbounded history with the chosen minimal stream state. Join
  cancellation through T2's ownership path rather than waiting for the next tick.
- [ ] Verify one final Disposed/error or clean None as specified, followed by
  stable None; owner disposal and control calls never duplicate termination.
- [ ] Run interval tests, update both language contracts and commit.

## T4 — Scheduled callback reentry, failure and release

**Files:** src/lib.rs, tests/throttle.rs, tests/debounce.rs; create tests/callbacks.rs.

- [ ] Add self-dispose, dispose-another, nested timer, and supported recursive
  call cases for leading/trailing callbacks. Decide and test recursive call
  coalescing explicitly rather than holding a RefCell borrow across user code.
- [ ] Cover panics in timeout/interval/leading throttle/trailing throttle/
  debounce, with one diagnostic per failure and a usable host afterward.
- [ ] Release the callback outside framework mutable borrows; restore state
  only if the operation is still live. Ensure cancellation drops captures
  promptly without user destructors running under the shared-state borrow.
- [ ] Verify pending-task replacement, boundary-time calls, repeated disposal
  and callback capture Drop probes. Run targeted tests and commit.

## T5 — Timer acceptance and evidence correction

**Files:** examples/timer_lifecycle.rs, timer Rustdoc, docs/en/api/timer.md,
docs/vi/api/timer.md and paired guides; tests/docs/; status documents.

- [ ] Add compile-fail checks for the sealed trait and required callback
  lifetime/local-host restrictions using Rustdoc where sufficient.
- [ ] Exercise each operation through plugin update/restart/owner disposal and
  a service-bound callback. Assert resource baselines and original errors.
- [ ] Build a bounded paired timer fixture only for equivalent TS observations;
  keep monotonic clock, single-reader and zero/range deviations separate.
- [ ] Run full shared acceptance, the runnable timer example and both guide
  toolchains. Correct the earlier design/roadmap claims using actual results.
- [ ] Commit the verified slice. Timer is accepted only when TF1–TF10 each
  have a passing regression or an explicitly reviewed non-defect/deviation.
