# Logger C0–C3 acceptance

Date: 2026-11-03 (Asia/Ho_Chi_Minh). Status: **C0–C3 accepted** within the declared native console scope.

## Scope and inputs

- Integration branch: `feat/cordis-completion`; clean starting checkpoint
  `a59edfcaa0710b47f9e8e841602d7641629d3867`.
- TypeScript intent: `local/cordis` at
  `35fdc969e4b6c48c48ed44717c7a018324e4dd88`, verified clean.
- Scope: [logger C0–C3](../logger.md), including the unpublished crate,
  owned writer, renderer, runnable application and matching English/Vietnamese
  documentation. Timer repair and loader remain outside this change.
- Reuse core M6 logging, current lifecycle contracts and accepted B0 evidence.
  No B0 artifact or ledger is changed; its 147 gaps are not this delivery's queue.

## Implementation verification

The initial implementation run passed the [shared acceptance commands](../acceptance.md#toolchain-matrix)
for both toolchains: workspace all-target check, workspace tests, strict Clippy,
Rustdoc with warnings denied, M0 tests/Clippy, and the console application.
Stable workspace/M0 formatting also passed. An ANSI16 timestamp capability
fix overlapped the initial run. After all code/tests were frozen, both explicit
toolchains reran workspace check/test/Clippy/Rustdoc and the console application;
stable workspace formatting also reran. Every final check passed and input hashes
confirmed that source files did not change during final verification.
No core, timer, M0 or parity executable input changed from the starting checkpoint.

| Toolchain | Versions | Executed tests |
|---|---|---|
| stable | rustc/rustdoc 1.97.1, Cargo 1.97.1, Clippy 0.1.97 | 164 core integration + 15 timer + 21 logger + 17 core Rustdoc; M0 20 integration + 1 Rustdoc |
| 1.85.0 | rustc/rustdoc/Cargo 1.85.0, Clippy 0.1.85 | Same test counts |

Each Cargo run selected rustup's explicit compiler/Cargo/rustdoc/Clippy binaries,
prepended their bin directory to PATH and used `target/acceptance-<toolchain>`.
The package remains Rust 2024, MSRV 1.85, version 0.0.0, unpublished.

Both executions of `cargo run -p re-cordis-logger --example console --locked`
produced exactly:

```text
[I] app plugin ready
[W] app owned output
[I] app manual output
```

`python3 tests/parity/run.py` passed with Node 22.23.3 and Yarn 4.14.1:
six TypeScript and six Rust cases, preserving all 73 observations
(11 combined, 14 coalesced, 12 removed, 10 superseded-failure,
13 latest-failure, 13 explicit-recovery). This is a regression check, not a new
B0 audit or claim of console byte parity.

## Documentation acceptance

All passed on the final implementation:

- `python3 -B tests/docs/test_check.py`: 6 checker regressions.
- `python3 -B tests/docs/check.py --toolchain stable`.
- `python3 -B tests/docs/check.py --toolchain 1.85.0`.

Each toolchain checked 23 page pairs, 56 core exports, 6 timer exports,
8 logger exports, 11 hook payloads and 10 hooks; compiled and executed all
10 unique examples shared by both languages; and passed the actual external
quick-start dependency manifest. Final status/report-only edits received
structural/link/diff checks without redundant Rust execution.

## Independent review

A fresh `gpt-6-astra/high` agent reviewed the frozen implementation independently
of the fresh `gpt-6.1-sol/high` implementer. Verdict: **approve C0–C3; no
actionable findings**. It reviewed production code, all 21 logger tests,
the application example, workspace/dependencies, pinned TypeScript source/tests,
paired documentation and checker changes. It independently passed the structural
documentation check and inspected the recorded console output and matrix results.

Confirmed boundaries include core filtering/ownership/error delegation;
bounded recursive export errors; success-only elapsed-state updates;
inert registration-token Drop; preserved user-supplied ANSI; capture release,
refused admission, restart/disposal, in-flight snapshot removal, and 100
resource-baseline cycles. No review correction was required.

## Supported scope and limitations

The owned synchronous writer uses `write_all` and per-record `flush`, with
visible I/O errors and no retry of an entire failed record. Partial bytes cannot
be rolled back; flush is not an fsync/durability guarantee. The core token and
owner control removal; an already-selected callback snapshot may finish.

Rendering supports severity/channel labels, multiline indentation, UTF-16 limits
and padding without splitting scalars, plain/ANSI16/ANSI256/Auto, optional fixed
UTC record timestamps and elapsed milliseconds. Rust Display/Debug and explicit
error trees replace JS arguments/inspection. See [decisions](../../../decision.md#console-logger-satellite)
and [limitations](../../../limitation.md#console-logger-satellite) for reasons
and user impact.

No truecolor, custom local calendar templates, terminal-cell/grapheme alignment,
browser console dispatch or asynchronous output queue is delivered.
Actual terminal emulators and Windows behavior were not exercised; the
deterministic color-policy tests and captured-output macOS runs are the evidence.
No new console cross-language trace is claimed.

## Integration

The local integration commit containing this report is the final revision for
this slice; resolve it with
`git log -1 --format=%H -- docs/plans/2026-11-02-cordis-completion/evidence/logger-acceptance.md`.
The source began at the checkpoint above. Final diff inspection, source-input
hash checks and whitespace checks passed. No B0 evidence, core or timer code
changed. No push, publication or deployment was performed.

Local detailed logs live in `target/logger-acceptance/` (ignored build output).
This tracked report records commands and outcomes; logs are not release artifacts.
