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

**Status:** T0–T5 and the required B2 bridge are independently accepted on
2026-11-03; final stable/1.85.0, documentation and paired checks passed.
See [acceptance evidence](evidence/timer-acceptance.md).

## Resolved T0 contract

Implemented on 2026-11-03 under the useful, idiomatic Rust direction.
The six method names, sealed trait, local non-Send hosting, inert core Effect
Drop and ordinary transfer/composition behavior are preserved.

Admission precedence is actual host/runtime, live owner, checked clock environment
and deadline/period, then exact registration commit. No task or callback survives
refusal. Sleep admits eagerly and stores refusal for first-poll delivery.
One-shot deadlines are pinned at successful construction; scheduled deadlines at
successful call. Positive sub-millisecond repeat periods are admitted with Tokio
millisecond precision. Zero repeat periods are InvalidDelay; zero one-shots and
debounce are async, zero throttle leads immediately. Missing time driver is
TimeDriverUnavailable after containing Tokio's synchronous construction panic;
the standard panic hook remains visible and no detached diagnostic is created.

| Operation | Fire/completion | Manual cancellation | Owner cancellation | Wrapper Drop | Panic |
|---|---|---|---|---|---|
| timeout | Retire exact effect before one callback | Effect.dispose starts cancel and returns its own join | Abort and join captures; no callback after barrier | Effect Drop stays inert | One diagnostic; operation already retired |
| sleep | Eager deadline; retire before Ok delivery | Drop waiter requests cancellation | Pending result becomes Disposed; join owned task | Request abort; retirement needs original host progress | No user callback |
| interval | First period, then fixed-phase coalescing | Effect.dispose starts cancel and returns its own join | Abort and join captures | Effect Drop stays inert | Stop operation; one diagnostic |
| interval_stream | At most one coalesced pending tick | First committed stop gives None; fail gives one original Err | Delivered cancellation commits Disposed if gate open | Commit stop and request abort | No user callback |
| throttle | Leading immediate; optional fixed-window trailing | dispose is capability cancellation, safe outside host | Cancel pending work and release captures by barrier | Cancel; idle captures release synchronously | Stop; leading call returns original failure, async path diagnoses once |
| debounce | Replace deadline per successful call | Same Scheduled capability contract | Same Scheduled owner contract | Same Scheduled Drop contract | Stop; diagnose once |

Terminal state outranks queued ticks. The first **committed** gate state wins;
a Fiber.dispose request and delivered owner cleanup are distinct moments.
After final stream Err, every subsequent poll is None; ready polls retain no
consumer waker. Control cancellation does not wait for another period.
Core TaskControl cancellation marks+requests abort only; the supervisor joins
before retiring. Owner cleanup retains its existing ordering/dependency rules.
Separately claimed manual Effect completions remain independent waiters.

Callbacks run outside mutable framework borrows. Self-disposal and positive-delay
recursive calls are supported; recursive zero calls are ignored, bounding both
synchronous recursion and zero-delay loops. Self/ancestor lifecycle waits remain
refused. Hosted callbacks may register nested Context work. If checked arithmetic
cannot represent a subsequent tick phase, callback repetition stops and streams
terminate with InvalidDelay instead of spawning a detached panic.

### Pinned 22-declaration disposition

This is a compact declaration map, not an exhaustive assertion ledger or claim
of unrestricted TypeScript parity. Upstream remains 35fdc969e4b6c48c48ed44717c7a018324e4dd88.

| Declaration lines in timer/index.spec.ts | Disposition |
|---|---|
| 22, 32, 41 (timeout callback, cancel, awaited timeout) | Rust timeout/sleep equivalents, eager ownership and cancellation regressions |
| 59 (interval callback) | Positive-delay callback equivalent; monotonic fixed-phase adaptation |
| 72, 140, 163, 187, 211 (iterator return/throw/break/owner disposal) | Single-consumer Stream representation; stop/fail/Drop/Disposed regressions |
| 95, 117, 130, 240 (concurrent next/return/throw/disposal readers) | Reviewed JS multi-reader exclusions; no Rust parity claim |
| 257, 272, 286, 300, 320, 332 (six throttle cases) | Leading/trailing/no-trailing/manual+owner cancellation equivalents; monotonic windows |
| 345, 360, 370 (three debounce cases) | Coalescing/manual+owner cancellation equivalents; monotonic windows |

The old design's 20 count is corrected to 22 declarations. Deprecated aliases,
TimerService installation, wall-clock adjustment and repeated JS iterator
rejection/return values remain explicit adaptations/exclusions in decision.md
and limitation.md. No accepted B0 artifact or six-scenario/73-observation runner
was changed.

## Executed repair evidence

Baseline: clean feat/cordis-completion at d869cf1 plus newly added tests, before
production changes. Each command was run individually because Cargo stops
after a failing test target.

| Command target (cargo test -p re-cordis-timer ... --locked) | Original result | Concrete failure |
|---|---|---|
| --test regressions | 0 passed, 8 failed | retained timeout effect; delayed Sleep; raw HOST refusal; refused callback still fired; unbounded tick history; stale tick before fail; RefCell self-dispose panic/capture retention; three effects after wrapper Drop |
| --test admission | 1 passed, 2 failed | no-host spawn_local panic and refused callback capture retention; stale-owner rejection already passed |
| --test deadlines | 0 passed, 5 failed | unchecked MAX/zero periods; late task deadlines; at-deadline Sleep; late stream phase |
| --test ownership | 0 passed, 2 failed | first completed cycle retained two effects; owner barrier released two captures instead of four |
| --test callbacks | 0 passed, 1 failed | timeout panic produced zero retained core diagnostics |

The first 16 tests gave 15 red/one green; extended baseline gave 18 red/one green.
The wrapper Drop regression was aligned with T0's explicit host-progress/join
contract; it never asserts synchronous effect retirement.

Independent review TR-1 added ready_stream_polls_do_not_retain_consumer_wakers:
ArcWake Drop probe remained 0 instead of 1 on Ready tick/error/end while stream
and control survived. It failed before the Pending-only waker registration fix.
Self-review added small_late_interval_poll_coalesces_instead_of_bursting:
a 1ms period first polled at 4ms fired four callbacks instead of one.
It failed before the explicit checked phase reset, since native Tokio Skip has
a 5ms lateness threshold. Both passed with the same targeted edge/deadline suite.

Current scope: 48 deterministic timer integration tests, three compile-fail
Rustdoc checks (sealed trait, static captures, local Sleep), six focused core
task-bridge tests, and the all-six-operation timer_lifecycle example.
Core effects/effect_metadata regressions also pass. B2 is justified by the
public HOST and self-retirement reproducers, with no change to generic Effect Drop.
The paired timer fixture is imported separately: four scenarios/16 equivalent
positive-delay observations; it does not compare Rust-specific adaptations.
Final shared toolchain/docs/parity review is recorded by the parent in
[evidence/timer-acceptance.md](evidence/timer-acceptance.md).

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

- [x] Classify each declaration in [the inventory](source-inventory.md):
  callback timeout/interval, awaited timeout, nine iterator cases, six throttle
  cases and three debounce cases. Preserve documented omitted aliases and
  multi-reader iterator differences.
- [x] Resolve zero for each operation; recommend an explicit minimum positive
  repeat period and asynchronous one-shot zero. Decide exact minimum, overflow
  boundary and rejection precedence before writing tests.
- [x] Pin start time at successful registration; distinguish an admitted Sleep
  whose polling is delayed from a future that has never been admitted.
- [x] Define Stream backpressure: recommend no unbounded tick history and at
  most one coalesced pending tick; document the difference from TS, which only
  serves pending next() calls. Terminal state takes precedence over stale ticks.
- [x] Resolve Drop separately for Effect handles, Sleep, IntervalStream, and
  Scheduled. Keep core's inert Drop; operation wrappers may cancel their own
  wait/producer only under an explicitly documented policy.
- [x] Define a valid Cordis host whose Tokio time driver is disabled: choose
  an explicit environment error or a documented construction precondition,
  and test it without hiding a detached-task panic.
- [x] Define panic/reentry and whether manual synchronous disposal merely
  initiates cancellation while owner completion joins captured-resource release.

**Exit:** accepted truth table for every operation × admission/fire/manual
cancel/owner cancel/drop/panic, with exact errors and terminal outcomes.

## T1 — Admission, range and clock boundaries

**Files:** src/lib.rs; create tests/admission.rs and tests/deadlines.rs in
crates/re-cordis-timer; reuse tests/common/mod.rs.
**Consumes:** T0 truth table and Context admission. **Produces:** no orphan
work on refusal, validated deadline/period, admission-time clock anchors.

- [x] Add failing tests for Duration::MAX, zero, just-before/at deadline, late
  first polling, missed intervals, wrong/non-running host, disabled time
  driver and stale owners.
- [x] Add a callback that registers on its Context to identify the HOST bridge
  requirement; route that failure to B2/T2 instead of weakening admission.
- [x] Run `cargo test -p re-cordis-timer --test admission --test deadlines --locked`
  and retain the expected failures.
- [x] Validate and admit before spawning. Use checked monotonic deadline
  arithmetic; rollback any acquired work if registration cannot commit.
- [x] Make these targeted tests pass, document policies, for parent final review/commit.

## T2 — Hosted ownership, one-shot cleanup and joined cancellation

**Files:** src/lib.rs; tests/timeout.rs, tests/sleep.rs; create
tests/ownership.rs. Core changes only through B2.

- [x] Write regressions that count live effects before/after repeated completed
  timeouts and sleeps, cancelled/dropped waits and plugin restart.
- [x] Use captured Drop probes plus a cleanup gate to distinguish abort request
  from actual task/resource release. Await owner disposal and assert releases
  occurred, no new callbacks execute and the Registry/effect baseline is restored.
- [x] Prove callbacks can register nested timers/effects under the correct
  owner; wrong-host work remains rejected. Preserve explicit cancellation of
  a timeout before its callback is invoked.
- [x] Repair hosted task ownership and exact terminal unregistration; use
  B2's reviewed bridge if necessary. Never use drop(Effect) as unregister.
- [x] Verify manual effect transfer/composition still works and does not
  double-dispose a self-completed timer. Test dropped completion waiters.
- [x] Run ownership/timeout/sleep tests and affected core tests, for parent final review/commit.

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

- [x] Add tests with no reader across many ticks, delayed readers, stop before
  first tick, fail/stop/dispose in both orders, repeated terminal calls and drop.
- [x] Assert T0's exact queued-tick/terminal policy, bounded pending state,
  producer cancellation, capture release and restored live effects.
- [x] Replace unbounded history with the chosen minimal stream state. Join
  cancellation through T2's ownership path rather than waiting for the next tick.
- [x] Verify one final Disposed/error or clean None as specified, followed by
  stable None; owner disposal and control calls never duplicate termination.
- [x] Run interval tests, update both language contracts for parent final review/commit.

## T4 — Scheduled callback reentry, failure and release

**Files:** src/lib.rs, tests/throttle.rs, tests/debounce.rs; create tests/callbacks.rs.

- [x] Add self-dispose, nested timer, and supported recursive
  call cases for leading/trailing callbacks. Decide and test recursive call
  coalescing explicitly rather than holding a RefCell borrow across user code.
- [x] Cover panics in timeout/interval/leading throttle/trailing throttle/
  debounce, with one diagnostic per failure and a usable host afterward.
- [x] Release the callback outside framework mutable borrows; restore state
  only if the operation is still live. Ensure cancellation drops captures
  promptly without user destructors running under the shared-state borrow.
- [x] Verify pending-task replacement, boundary-time calls, repeated disposal
  and callback capture Drop probes. Run targeted tests for parent final review/commit.

## T5 — Timer acceptance and evidence correction

**Files:** examples/timer_lifecycle.rs, timer Rustdoc, docs/en/api/timer.md,
docs/vi/api/timer.md and paired guides; tests/docs/; status documents.

- [x] Add compile-fail checks for the sealed trait and required callback
  lifetime/local-host restrictions using Rustdoc where sufficient.
- [x] Exercise each operation through plugin update/restart/owner disposal and
  a service-bound callback. Assert resource baselines and original errors.
- [x] Build a bounded paired timer fixture only for equivalent TS observations;
  keep monotonic clock, single-reader and zero/range deviations separate.
- [x] Run full shared acceptance, the runnable timer example and both guide
  toolchains. Correct the earlier design/roadmap claims using actual results.
- [x] Commit the verified slice. Timer is accepted only when TF1–TF10 each
  have a passing regression or an explicitly reviewed non-defect/deviation.
