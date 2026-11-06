# B0 matrix report scoped re-review

Date: 2026-11-02. **Verdict: B0-MATRIX-01 addressed; B0-MATRIX-02
addressed. No new material report issue found.** This accepts the scoped
report repair, not B0 as a whole.

## Scope and preservation

Fresh independent review of commit `25dcde8`, limited to
[matrix/report.md](matrix/report.md) and its three new validation JSON/log
pairs, against the accepted matrix at
`7d1552a7612a6a7be18421c6a73779624ba41f28` and the two findings in
[matrix-review.md](matrix-review.md). The isolated review worktree started
at `25329ad995ba21b326e116e801f7f57816d4ed0b`; its matrix files have no
delta from `25dcde8`. The current dispatch supersedes the older handoff's
reviewer-routing and no-commit instructions.

Compared all 65 original matrix files byte-for-byte across the accepted
commit, report commit, review HEAD and working tree. All are identical,
including the before/after/supplement snapshots, command indexes, results,
scripts and original attempt logs. Independently hashed the 71 recorded
executable inputs in the review worktree; all match the preserved baseline.
The recorded execution-revision diff contains only `progress.md`.

This was a document/artifact review using Git comparisons, retained records
and read-only consistency checks. No Cargo command, test, example, parity
runner, summary script or original validation command was rerun. The source
ledger and other changes in the report commit were outside scope.

## Finding dispositions

**B0-MATRIX-01 — addressed.** The report accurately distinguishes the first
original summary log's HEAD-equality AssertionError from the second log's
validation message and Rustdoc inventories without a traceback. Neither log
has command, UTC, environment or exit metadata. The report leaves both
original exit codes unknown, does not call attempt 2 a second failure, and
does not claim every attempted command passed. The original bytes remain
unchanged; no metadata was retroactively inserted.

The new [summary metadata](matrix/summarize-validation-2026-11-02.json) and
[log](matrix/logs/summarize-validation-2026-11-02.log) agree on argv, cwd,
start/end UTC and exit 0. The report's separate attribution to assembly HEAD
`3463486544d7b3c7a34c2f14f57321651a92f5ef` matches that metadata. Its recorded
before/after hashes agree and independently match the current bytes of
`results.json`, `summarize.py` and both original attempts. This new result
does not supply an exit code for either historical attempt.

**B0-MATRIX-02 — addressed.** The readable report now covers the execution
revisions and progress-only change, actual tool versions and environments,
all 22 required invocations, test outcomes, executed applications, guide/API
checks, six paired scenarios and evidence limits. Its counts agree with the
accepted review and retained results: 53 indexed commands, 66 suite summaries,
440 Rust outcome records, six comparisons / 73 observations and 71 input
hashes. It explains that the 440 records include repeated tests and records
the different Rustdoc grouping between toolchains.

All 53 JSON command rows agree with the TSV and corresponding log metadata,
including zero indexed exit statuses. All 40 local report link targets exist.
The report retains the macOS/arm64 and six-scenario bounds, unexecuted
platform/type-test/negative-guard limits, transitive-JS dependency limitation,
and open timer contract repair. It explicitly keeps the declaration/variant/
type-obligation ledger and independent acceptance outside this verdict.

## Preserved report-validation failure and retry

The [first report validation](matrix/report-validation-2026-11-02.json) and
[its log](matrix/logs/report-validation-2026-11-02.log) agree on exit 1 and
show the missing self-linked output log at the link-check step. The report
acknowledges this assembly failure. The
[retry metadata](matrix/report-validation-retry-2026-11-02.json) and
[retry log](matrix/logs/report-validation-retry-2026-11-02.log) agree on
argv, cwd, timestamps and exit 0, retaining the failed attempt separately.
The committed report's SHA-256 matches the successful retry's
`9226c3da232c54bd0c61797788504ae842b371177c653360d46e46547f8f8e9a`.
The earlier hash belongs to the earlier attempt and was not overwritten.

The accepted command-matrix baseline remains bounded as previously reviewed.
This re-review closes only the two matrix-report findings and does not open
the B0 completion scheduling gate.
