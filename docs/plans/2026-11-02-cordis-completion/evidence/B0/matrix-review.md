# B0 command matrix independent review

Date: 2026-11-02. **Verdict: accepted for the command-matrix portion.**
All 22 required acceptance/example/documentation/parity invocations have
attributable zero-exit logs. The two evidence-assembly observations below
remain to be recorded by the integration owner. This verdict does not accept
B0 as a whole: the incomplete core declaration/type ledger is outside this
review and remains an independent acceptance gate.

## Scope and revisions

- Reviewed committed evidence at
  `7d1552a7612a6a7be18421c6a73779624ba41f28` on `feat/cordis-completion`,
  including the matrix artifacts introduced in `c3fbcb0` and the retained logs.
- Execution began at `578c066b8618bec9b1b2afc0a9757bb9b0f3a87e`
  ([before snapshot](matrix/before.json), 08:34:01 UTC). It ended at
  `e23dbac846454d4573eee2ba3eebbc5e33085d17`
  ([after snapshot](matrix/after.json), 08:36:28 UTC); the
  [supplement snapshot](matrix/after-supplement.json) is at 08:36:43 UTC.
- The Git diff between the execution revisions contains only `progress.md`.
  All 71 recorded executable-input SHA-256 values are identical in the three
  snapshots and independently match Git blobs at both execution revisions and
  the reviewed revision. The executable-path diff from baseline `808c9f5` to
  the reviewed revision is empty. The revision change does not invalidate the
  command results. The review workspace was at
  `fd0862982cdf2cc0ce3ea72ed6eddc702e1493d9`; its committed delta from the
  reviewed revision also contains only `progress.md`.
- Read the repository instructions, [master plan](../../README.md),
  [core B0 requirements](../../core.md), [shared acceptance](../../acceptance.md),
  [B0 execution plan](plan.md), and [review handoff](review-handoff.md).
  Reviewed existing artifacts and runner/checker source; no tests, examples,
  parity runs, or evidence-generation scripts were rerun.

## Commands and compiler attribution

The [JSON command index](matrix/commands.json) and
[TSV command index](matrix/commands.tsv) agree in all fields for all 53 rows.
Every indexed log has matching command, working directory, start/end UTC and
exit-code metadata. All 53 exits are zero. The 53 rows include 22 required
checks and 31 tool/environment/diff checks; they exclude the two summary
attempt logs.

Both toolchains ran workspace check with all targets, workspace tests,
workspace Clippy with warnings denied, workspace documentation with
`RUSTDOCFLAGS=-D warnings`, M0 tests and Clippy, and the actual `lifecycle` and
`timer_lifecycle` applications. Stable additionally ran workspace and M0
format checks. The three Python documentation commands and existing paired
runner ran serially after the Rust matrix. No required invocation is missing.

| Recorded tool | Stable | Minimum supported toolchain |
|---|---|---|
| rustc / rustdoc | 1.97.1, `8bab26f4f` | 1.85.0, `4d91de4e4` |
| Cargo | 1.97.1, `c980f4866` | 1.85.0, `d73d2caf9` |
| cargo-clippy / clippy-driver | 0.1.97, `8bab26f4f6` | 0.1.85, `4d91de4e48` |
| rustfmt | 1.9.0-stable, `8bab26f4f6` | Not required by the matrix |

The [stable environment](matrix/environment-stable.json) and
[1.85.0 environment](matrix/environment-1.85.0.json) name absolute binaries
under their respective `~/.rustup/toolchains/*-aarch64-apple-darwin/bin`
directories, prepend that directory to PATH, and explicitly set RUSTC and
RUSTDOC. Every applicable command row agrees with its recorded environment.
The separate build targets are `target/acceptance-stable` and
`target/acceptance-1.85.0`. Locator/version logs include both Clippy binaries
and the stable formatting binaries. Snapshots record no inherited Rust/Cargo
overrides or applicable Cargo configuration files. The unchanged docs checker
and paired runner also select explicit rustup tools; their own logs print the
expected compiler versions.

The recorded host is macOS 26.7.1 arm64. Python is 3.14.7; Node 22.23.3;
Corepack 0.36.0; Yarn 4.14.1; TypeScript 5.9.3; ESLint 8.57.1; Vitest 4.1.12;
cosmokit 1.8.1; Yakumo 3.2.1. The initial package inventory omitted the `tsx`
alias, but the retained
[supplemental package log](matrix/logs/environment-tsx-package.log) resolves it
to `@cordiverse/tsx` 4.19.3-fix.3. Existing dependencies sufficed; no restoration
or toolchain substitution is recorded. Unix fixture prerequisites are present.

## Independently reconciled outcomes

Reparsed the preserved test output without running `summarize.py`. All 440
individual Rust outcome records and all 66 Rust suite summaries match
[results.json](matrix/results.json), including crate/suite attribution and
compile-fail classification. The 440 records include both toolchains and the
six repeated Rust parity tests; they are not 440 distinct compatibility cases.

| Executed evidence | Stable | 1.85.0 |
|---|---:|---:|
| Core integration tests, 16 suites | 164 | 164 |
| Of those, GitHub application tests | 12 | 12 |
| Timer integration tests, 6 suites | 15 | 15 |
| Core Rustdoc runnable / compile-fail checks | 4 / 13 | 4 / 13 |
| Core and timer unit tests / timer Rustdoc checks | 0 / 0 | 0 / 0 |
| M0 integration tests, 4 suites | 20 | 20 |
| M0 compile-fail Rustdoc checks | 1 | 1 |
| Lifecycle and timer applications actually executed | 2 | 2 |
| Unique runnable guide/API snippets | 8 | 8 |
| Actual external quick-start application | 1 | 1 |

The Rust suite summaries report zero failed, ignored, measured, or filtered
tests. Stable splits core Rustdoc output into four runnable and thirteen
compile-fail results; 1.85.0 reports their combined seventeen-result suite.
This accounts for the different suite-summary totals.

The [Python regression log](matrix/logs/docs-regressions.log) reports five
passing tests. Both [stable](matrix/logs/docs-stable.log) and
[1.85.0](matrix/logs/docs-1.85.0.log) documentation logs report 21 page pairs,
56 core exports, 6 timer exports, 11 hook payloads and 10 hooks, followed by
eight unique executed snippets and the external quick start. These checks do
not establish that every new planning/evidence link was visited.

## Paired scenarios and input safeguards

The [paired-run log](matrix/logs/parity.log) records exit zero, six TypeScript
tests and six Rust tests, and every required exact comparison:

| Scenario | Observations |
|---|---:|
| combined | 11 |
| coalesced | 14 |
| removed | 12 |
| superseded-failure | 10 |
| latest-failure | 13 |
| explicit-recovery | 13 |
| Total | 73 |

All snapshots record TS HEAD
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`, no tracked TS changes, and absence
of the temporary upstream fixture. The 71 unchanged input hashes include
both Cargo locks, the runner, both language fixtures and all six expected
trace files. The existing ignored/untracked upstream Yarn lock is recorded
separately, with unchanged SHA-256
`fd25ff5b5f38a0e92f5d8ec33369da70c86ceee6bfcc5118d069bfe95dcd6ad3`.

Source inspection confirms the unchanged runner retains exact pin checking,
tracked-diff refusal, exclusive fixture creation, finally cleanup, explicit
stable tools, upstream ESLint and TypeScript checks, Corepack/Yarn test routing,
bounded subprocesses, and unsorted exact trace comparison. Its temporary
traces are deliberately deleted; the successful comparison log plus unchanged
expected-file hashes meet the B0 plan's evidence rule. The paired result is
limited to these six scenarios. No negative-guard rerun, repeated-run evidence,
full upstream core suite, or separate `events.types.ts` execution is claimed.

## Findings and remaining work

1. **B0-MATRIX-01 — summary attempt metadata is incomplete.**
   [Attempt 1](matrix/logs/summarize-attempt-1.log) records an AssertionError
   from requiring identical before/after HEADs. The independently verified
   progress-only revision change and unchanged input hashes explain this
   evidence-assembly failure; it is not a failed acceptance command.
   [Attempt 2](matrix/logs/summarize-attempt-2.log) contains the validation
   message and both final Rustdoc inventories, with no traceback. It must not
   be described as a second recorded failure. Neither attempt has command,
   UTC, environment or exit-status metadata, and neither appears in the
   command index. Do not infer or retroactively invent an exit code for
   attempt 2. Owner: matrix executor/integration owner. Record this distinction
   in the final evidence report; if a separately attributable summary-command
   result is wanted, preserve a new logged attempt without replacing either
   existing log. This does not block the independently reconciled matrix.

2. **B0-MATRIX-02 — the planned matrix report is missing.**
   `matrix/report.md`, explicitly required by the B0 plan, is absent from the
   reviewed commit. `results.json` contains Rust outcomes and comparisons but
   does not replace the required readable account of documentation/examples,
   revision changes, failed assembly attempts and evidence limits. Owner:
   matrix executor/integration owner. Add the report, or explicitly adopt
   this review's account as the report with an updated evidence index, before
   declaring the B0 evidence package complete. No acceptance rerun is required
   solely to assemble that document.

The accepted command baseline applies to the recorded macOS/arm64 environment
and preserved executable inputs. It does not close timer contract repair,
prove unexecuted platform coverage, freeze all transitive JS dependencies, or
complete the 144-declaration/156-variant and six-type-obligation ledger. The
integration owner must obtain the separate ledger review before accepting B0
or using B0 completion as a scheduling gate.
