# B0 command matrix report

Date: 2026-11-02. The preserved matrix has attributable exit-zero results
for all 22 required commands. The [independent matrix review](../matrix-review.md)
accepted this portion at `7d1552a7612a6a7be18421c6a73779624ba41f28`.
**B0 remains incomplete:** the separate 144-declaration / 156-variant /
six-event-type-obligation ledger and its acceptance review are outstanding.
This report closes the missing readable-report finding; it does not accept B0
or close timer contract repair.

## Execution revisions and unchanged inputs

The [before snapshot](before.json) records execution start at
`578c066b8618bec9b1b2afc0a9757bb9b0f3a87e`, 08:34:01 UTC. The
[after snapshot](after.json) records completion at
`e23dbac846454d4573eee2ba3eebbc5e33085d17`, 08:36:28 UTC; the
[supplement snapshot](after-supplement.json) uses the same revision at
08:36:43 UTC. These are the recorded matrix revisions on
`feat/cordis-completion`, rather than the later report-writing revision.
The review independently checked that the intervening Git diff contains only
`docs/plans/2026-11-02-cordis-completion/progress.md`.

All three snapshots contain the same 71 executable-input SHA-256 values.
These include both Cargo lockfiles, Rust source/tests/examples, documentation
checker, paired runner, both language fixtures, and all six expected traces.
The recorded executable-path delta from implementation baseline `808c9f5` is
empty. The independent review also matched these hashes to Git blobs at both
execution revisions and its reviewed revision.

The fixed upstream TS HEAD is
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`. Every snapshot records a clean
tracked TS checkout and absence of the temporary upstream parity fixture.
The existing untracked/ignored upstream Yarn lock is recorded separately and
unchanged, SHA-256
`fd25ff5b5f38a0e92f5d8ec33369da70c86ceee6bfcc5118d069bfe95dcd6ad3`.
This does not freeze all transitive JS dependencies.

## Actual tools and command index

The recorded host is macOS 26.7.1 arm64; Python 3.14.7; Node 22.23.3;
Corepack 0.36.0; Yarn 4.14.1; TypeScript 5.9.3; ESLint 8.57.1;
Vitest 4.1.12; cosmokit 1.8.1; Yakumo 3.2.1. The initial package log
omitted the `tsx` alias; the [supplemental package log](logs/environment-tsx-package.log)
resolves it to `@cordiverse/tsx` 4.19.3-fix.3. No dependency restoration or
toolchain substitution is recorded. `/bin/sh`, `/bin/kill`, `mkfifo` and
`head` prerequisites were present.

| Recorded tool | Stable | MSRV |
|---|---|---|
| rustc / rustdoc | 1.97.1 (`8bab26f4f`) | 1.85.0 (`4d91de4e4`) |
| Cargo | 1.97.1 (`c980f4866`) | 1.85.0 (`d73d2caf9`) |
| cargo-clippy / clippy-driver | 0.1.97 (`8bab26f4f6`) | 0.1.85 (`4d91de4e48`) |
| rustfmt | 1.9.0-stable (`8bab26f4f6`) | Not required / not run |

The [stable environment](environment-stable.json) and
[MSRV environment](environment-1.85.0.json) select absolute tools under their
respective rustup `*-aarch64-apple-darwin/bin` directories, prepend that
directory to PATH, and explicitly set RUSTC and RUSTDOC. Build targets are
`target/acceptance-stable` and `target/acceptance-1.85.0`;
`RUSTDOCFLAGS=-D warnings` applies. Snapshots record no inherited Rust/Cargo
overrides or applicable Cargo configuration. The documentation checker and
paired runner select their own explicit rustup tools and print versions.

The [JSON index](commands.json) and [TSV index](commands.tsv) retain exact
argv, cwd, environment, start/end UTC, exit status and log path for all
53 indexed commands: 22 required checks and 31 environment/locator/version/diff
checks. Every indexed exit is zero. The two original summary attempts are
excluded from this index; the new summary validation below is separately
attributed and does not change the 53-row matrix.

The 22 required invocations are accounted for below. Cargo entries use the
absolute binary in the corresponding environment; each cell links the full
command/output log.

| Command arguments | Stable log (exit 0) | 1.85.0 log (exit 0) |
|---|---|---|
| `check --workspace --all-targets --locked` | [check](logs/stable-workspace-check.log) | [check](logs/1.85.0-workspace-check.log) |
| `test --workspace --locked` | [test](logs/stable-workspace-test.log) | [test](logs/1.85.0-workspace-test.log) |
| `clippy --workspace --all-targets --locked -- -D warnings` | [Clippy](logs/stable-workspace-clippy.log) | [Clippy](logs/1.85.0-workspace-clippy.log) |
| `doc --workspace --no-deps --locked` | [docs](logs/stable-workspace-doc.log) | [docs](logs/1.85.0-workspace-doc.log) |
| `test --manifest-path spikes/m0/Cargo.toml --locked` | [M0 tests](logs/stable-m0-test.log) | [M0 tests](logs/1.85.0-m0-test.log) |
| `clippy --manifest-path spikes/m0/Cargo.toml --all-targets --locked -- -D warnings` | [M0 Clippy](logs/stable-m0-clippy.log) | [M0 Clippy](logs/1.85.0-m0-clippy.log) |
| `fmt --all --check` | [format](logs/stable-workspace-fmt.log) | Not required / not run |
| `fmt --manifest-path spikes/m0/Cargo.toml --check` | [M0 format](logs/stable-m0-fmt.log) | Not required / not run |
| `run -p re-cordis-core --example lifecycle --locked` | [application](logs/stable-example-lifecycle.log) | [application](logs/1.85.0-example-lifecycle.log) |
| `run -p re-cordis-timer --example timer_lifecycle --locked` | [application](logs/stable-example-timer_lifecycle.log) | [application](logs/1.85.0-example-timer_lifecycle.log) |

The remaining four required commands are
[`python3 -B tests/docs/test_check.py`](logs/docs-regressions.log),
[`python3 -B tests/docs/check.py --toolchain stable`](logs/docs-stable.log),
[`python3 -B tests/docs/check.py --toolchain 1.85.0`](logs/docs-1.85.0.log), and
[`python3 tests/parity/run.py`](logs/parity.log), each exit 0.

## Observed results and comparison limits

Counts come from preserved outputs and [results.json](results.json), reconciled
independently in the review and by the new summary validation.

| Executed evidence | Stable | 1.85.0 |
|---|---:|---:|
| Core integration tests, 16 suites | 164 | 164 |
| Included GitHub application tests | 12 | 12 |
| Timer integration tests, 6 suites | 15 | 15 |
| Core Rustdoc runnable / compile-fail checks | 4 / 13 | 4 / 13 |
| Core and timer unit tests / timer Rustdoc checks | 0 / 0 | 0 / 0 |
| M0 integration tests, 4 suites | 20 | 20 |
| M0 compile-fail Rustdoc checks | 1 | 1 |
| Lifecycle / timer applications executed | 2 | 2 |
| Unique runnable guide/API snippets | 8 | 8 |
| External quick-start application | 1 | 1 |

All Rust suite summaries report zero failed, ignored, measured and filtered
outcomes. Stable emits separate four-runnable and thirteen-compile-fail core
Rustdoc summaries; MSRV combines all seventeen. The inventory contains
66 suite summaries and 440 individual Rust outcome records: 217 per toolchain
plus six Rust tests repeated by the paired runner. These are not 440 distinct
compatibility cases. The Python regression command reports five passing tests.
Both guide checks report 21 page pairs, 56 core exports, 6 timer exports,
11 hook payloads and 10 hooks, eight executed snippets and the quick start.
Their normal page scope does not automatically validate new evidence links.

The paired runner records six TS tests, six Rust tests, and unsorted exact
Rust/TS comparisons for combined (11 observations), coalesced (14), removed
(12), superseded-failure (10), latest-failure (13), explicit-recovery (13):
**six scenarios / 73 observations**. The unchanged runner retains exact pin
and tracked-diff guards, exclusive temporary-fixture creation, finally cleanup,
explicit stable tools, upstream ESLint/TypeScript checks, Corepack/Yakumo/Vitest
routing, bounded subprocesses and exact trace comparison. Its temporary traces
are deliberately removed; the retained comparison log and unchanged expected
trace hashes satisfy the plan's evidence rule.

Coverage is bounded to the recorded macOS/arm64 host and these six paired
scenarios. Linux/Windows coverage, the full upstream core suite, separate
`events.types.ts` execution, negative-guard reruns and repeated-run evidence
were not established by this matrix. The GitHub tests use bounded fake-process
and signed-loopback fixtures; no live collector or user-secret listener ran.
Passing existing timer tests does not repair the open timer contract gaps.

## Summary attempts and report validation

[Original attempt 1](logs/summarize-attempt-1.log) contains an AssertionError
at `assert before['head']==after['head']`. This was an evidence-assembly
failure caused by requiring equal HEADs across tracking-only commits despite
unchanged executable inputs; it was not a failed required acceptance command.
[Original attempt 2](logs/summarize-attempt-2.log) prints the successful
53/66/440/six/73/71 validation message and both Rustdoc inventories with no
traceback. It is not a second recorded failure. Neither original log preserves
command, UTC, environment or exit metadata, and neither is indexed. Their
original exit codes remain unknown; this report does not infer attempt 2's
exit status or claim that every command ever attempted passed.

A new run of the unchanged summary script,
`/opt/homebrew/opt/python@3.14/bin/python3.14 -B docs/plans/2026-11-02-cordis-completion/evidence/B0/matrix/summarize.py`,
ran from the repository root at report-assembly HEAD
`3463486544d7b3c7a34c2f14f57321651a92f5ef`. It started at
2026-11-02T08:53:17.908594+00:00, ended at
2026-11-02T08:53:17.945149+00:00, and exited 0. The
[new log](logs/summarize-validation-2026-11-02.log) and
[metadata](summarize-validation-2026-11-02.json) preserve actual argv, cwd,
selected Python environment, timestamps and exit code. It validated the
53 indexed zero-exit logs, 66 suite summaries, 440 Rust outcomes, six/73
comparisons and 71 unchanged inputs. Hashes before/after this summary run
confirm `results.json`, the script and both original attempts remained
byte-for-byte unchanged. This run reparsed retained evidence only; no matrix
acceptance command or paired scenario was rerun.

The [report consistency log](logs/report-validation-2026-11-02.log) and
[metadata](report-validation-2026-11-02.json) record the final local link,
index/log metadata, preserved-input and whitespace checks. The command matrix
and original summary attempts remain intact.

The first report link check exited 1 because its own linked output log did
not yet exist at check time; that failed check and metadata are preserved
above. A [retry log](logs/report-validation-retry-2026-11-02.log) and
[retry metadata](report-validation-retry-2026-11-02.json) record the check
after creating its output paths. This report-assembly failure is separate
from the accepted matrix and the successful summary validation.
