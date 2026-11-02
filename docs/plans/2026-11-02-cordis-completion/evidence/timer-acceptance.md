# Timer repair acceptance

Date: 2026-11-03 (Asia/Ho_Chi_Minh). Status: **T0–T5 and the narrow B2 timer bridge accepted**.

## Scope and baseline

- Integration: `feat/cordis-completion`, clean starting commit
  `d869cf159573f62f469d4bc7f1ab97bdf9d9650a` (accepted logger C0–C3).
- Fixed TypeScript specification:
  `35fdc969e4b6c48c48ed44717c7a018324e4dd88`, clean before and after execution.
- Scope: [timer repair T0–T5](../timer.md) and the reproduced
  [B2 core bridge](../core.md#b2--conditional-core-bridge-or-targeted-regression-fix).
  Loader and unrelated work remain unstarted.
- Existing B0 artifacts, logger implementation and dependency/feature manifests
  are unchanged. Existing core hosting, cleanup, composition, source identity
  and lifecycle regressions remain the baseline.

## Regression-first repair

Before production edits, 19 new timer tests ran: 18 exposed expected failures,
while stale-owner rejection already passed. Observed failures included work
surviving refused admission, retained completed registrations/callback captures,
deadlines starting on first poll, stale queued ticks preceding termination,
missing HOST authority, reentry borrow panic and missing panic diagnostics.
The [timer plan](../timer.md) records commands, names, the T0 operation contract,
22 upstream declaration dispositions, and TF1–TF10 evidence.

Independent `gpt-6-astra/high` contract review confirmed that existing cleanup
prefixes and barriers were reusable, but public APIs could not supply owned
HOST execution or safe retirement before a timeout callback. B2 adds
`Context::task`, cancellation-only `TaskControl`, and admitted
`Context::invoke_callback` with `CallbackError`. Core Effect Drop, composition
and the existing manual-completion semantics are unchanged.

The future returns its synchronous terminal callback; core retires its exact
registration immediately before invoking that callback. Cancellation outside
the host requests abort without running callbacks or creating HOST authority.
The existing supervisor joins release before retiring cancelled work, keeping
owner cleanup able to await it.

## Final explicit toolchain matrix

All commands passed against frozen executable and bilingual-document inputs.

| Toolchain | Verified versions | Workspace results |
|---|---|---|
| stable | rustc/rustdoc 1.97.1, Cargo 1.97.1, Clippy 0.1.97 | 259 passed, zero failures |
| 1.85.0 | rustc/rustdoc/Cargo 1.85.0, Clippy 0.1.85 | 259 passed, zero failures |

Per toolchain the workspace count is 170 core integration tests (including
6 new bridge tests), 48 timer integration tests, 21 logger tests, 17 core
Rustdoc checks and 3 timer Rustdoc checks. M0 separately passed 20 integration
tests and 1 Rustdoc check.

The [shared command matrix](../acceptance.md#toolchain-matrix) ran:
workspace all-target check/test/strict Clippy/Rustdoc, M0 test/strict Clippy,
and stable workspace/M0 formatting. Every invocation used rustup-resolved
compiler/Cargo/rustdoc/Clippy binaries, their bin directory first on PATH,
`target/acceptance-<toolchain>`, and warnings-denied Rustdoc.
No Cargo or Tokio feature changes were needed.

Both toolchains additionally executed:

```sh
cargo run -p re-cordis-timer --example timer_lifecycle --locked
cargo run -p re-cordis-core --example lifecycle --locked
cargo run -p re-cordis-logger --example console --locked
```

The timer application exercises all six operations across owner restart and
joined disposal, with assertions and matching output on both toolchains.
The existing core and logger applications also completed successfully.

## Documentation and paired traces

All passed:

```sh
python3 -B tests/docs/test_check.py
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
python3 tests/parity/run.py
python3 -B tests/timer_parity/run.py --toolchain stable
python3 -B tests/timer_parity/run.py --toolchain 1.85.0
```

The documentation regressions passed 6 tests. Each toolchain checked 23 page
pairs, 58 core exports, 6 timer exports, 8 logger exports, 11 hook payloads and
10 hooks; compiled/executed 9 unique examples shared by EN/VI; and passed the
actual external quick-start dependency manifest.

The original lifecycle comparison preserves all 6 scenarios / 73 observations.
The new [bounded timer fixture](../../../../tests/timer_parity/README.md)
passes 4 TypeScript cases / 16 byte-identical observations per toolchain:
timeout once, timeout cancellation, awaited sleep and interval stop.
Node 22.23.3 and Yarn 4.14.1 ran the pinned TS ESLint/tsc/Vitest commands.
The integrated runs retained traces under `target/timer-acceptance/traces-<toolchain>`.
The fixture worker also checked wrong-pin refusal, exclusive creation,
subprocess-failure cleanup and byte/newline mismatch rejection.
This is bounded comparison evidence, not parity for the Rust adaptations below.

## Independent implementation review

A fresh `gpt-6-astra/high` reviewer independently assessed the fresh
`gpt-6.1-sol/high` implementation, including task ownership/admission,
composition, panic containment, lifetime probes, examples, EN/VI contracts
and the paired fixture. Final verdict: **approve; no blocking findings remain**.

- TR-1: Ready stream polling retained consumer wakers. A failing Drop-probe
  regression reproduced it; wakers now register only for Pending. The final
  test retains a control handle after the stream is dropped. Re-review passed.
- TR-2: module-summary wording overstated cancellation as joined release.
  Rustdoc and paired contracts now distinguish request from awaited completion.
- Self-review reproduced Tokio's short-lag Skip catch-up burst. The repair
  resets to the next original phase strictly after now. Late-poll tests with
  1 ms, 1 µs and 1 ns periods pass; phase arithmetic was independently reviewed.
- A checked test claim was narrowed to the actual self-dispose, nested and
  recursive-call coverage. Cross-scheduled disposal has no separate targeted
  fixture; no code defect was inferred and no implementation-mirroring test
  was added solely to fill a checklist.

The reviewer independently ran 6 core task tests and 18 timer tests on explicit
stable, then rechecked strengthened edge cases. Those focused results complement
the full final matrix above.

## Rust contract and remaining boundaries

See [decisions](../../../decision.md#timer-satellite) and
[limitations](../../../limitation.md#timer-satellite) for reasons and user impact.

- Deadlines are checked and eager. Repeat periods must be positive; admitted
  sub-millisecond periods remain subject to Tokio's millisecond resolution.
  Zero one-shots/debounce are asynchronous; leading throttle may run immediately.
- The stream stores at most one pending tick; the first committed terminal
  state wins over stale ticks. It is a single-consumer Rust Stream, not JS's
  multi-reader iterator or return-value/throw protocol.
- `Scheduled::call()` now returns `Result`; synchronous local callbacks use
  captured Rust state. Recursive calls are bounded and callback panics stop
  the affected operation while preserving one core diagnostic.
- Wrapper Drop/control cancellation request stopping; resumed host progress
  or an awaited owner barrier completes resource release. Existing manually
  claimed Effect completion remains an independent waiter.
- Owner disposal requests follow existing core scheduling. No callback starts
  after the awaited barrier; raw request order is not a promise about which
  stream terminal state was committed first.
- A missing Tokio time driver is an explicit construction error. Rust's panic
  hook can still print when the constructor panic is caught. Panic-abort and
  hostile destructor recovery remain outside the framework contract.
- Tests use controlled clocks and local hosts on macOS. No real-time latency,
  throughput, multithreaded hosting or Windows execution claim is made.

## Integration

The final revision is the local integration commit containing this report:
`git log -1 --format=%H -- docs/plans/2026-11-02-cordis-completion/evidence/timer-acceptance.md`.
Fixture commit `fb009c3` was imported without committing overlapping work;
its independent worktree was subsequently archived recoverably.

Detailed local logs and frozen-input hashes are in `target/timer-acceptance/`
(ignored build output). Hash checks confirmed no executable or EN/VI document
changed during the final matrix. Closing status/evidence edits receive
structural/link/whitespace validation without redundant Rust reruns.
No push, publication or deployment was performed.
