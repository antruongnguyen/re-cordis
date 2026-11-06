# R0-R1 independent scoped re-review

Date: 2026-11-04. Branch: `feat/cordis-completion`.
Executable correction range:
`4742efe44a55dd1464eb851f9d9eb11d66fde82b..35556d5108eaf13104a5fb287c83b28ffb5d1576`.

**Verdict: R0-R1 ADDRESSED. The correction passes this scoped specification
and code-quality review. No new actionable finding was found in the correction
or the separately labeled controller prose clarification.** The original
review's remaining selected R0 conclusions carry forward. Final shared
matrix/generated-application evidence is still required for R0 acceptance;
this report does not itself accept the entire R0 milestone.

## Scope and method

Read AGENTS.md, [the original independent review](r0-review.md) and
[the correction report](r0-corrections.md) first. Reviewed the exact executable
range and `target/r0-review/correction-package.diff`, whose separately labeled
controller section changes CHANGELOG, the decision record, matching English/
Vietnamese guides and progress. The reviewed package SHA-256 is
`5bac489a5104b19135961c290bc9ee70cbcf179fe2660b924079b17b941b4292`.

CodeGraph supplied the current shutdown, watcher/Include ownership and error
paths, the new regression, forced-cancellation tests and demonstration caller.
The IDE supplied the unindexed original probe and correction hash records.
Git supplied the exact delta. The correction changes only the shared
application shutdown, one new regression and its report; the existing tests
and real-watcher readiness logic are unchanged.

This is a fresh, scoped re-review of R0-R1 closure and breakage introduced by
that delta. It does not reopen accepted B0/core/satellite work, the previous
loader correction, source accounting, generated-fixture design, or the
original review's other selected composition conclusions.

## Finding disposition

| Finding / required correction | Disposition and evidence |
|---|---|
| **R0-R1 — join retained watcher refresh before canceling Include** | **ADDRESSED.** `Application::shutdown` at `crates/re-cordis-hmr/examples/system_lifecycle/app.rs:330–373` awaits watcher disposal before calling Include disposal, then awaits Include before owner disposal. The original unchanged deterministic probe now returns `Ok(())` on both toolchains. |
| Continue teardown after an earlier error | **ADDRESSED.** Each stage uses a local async result block. Its `?` exits that block, not `Application::shutdown`. Every stage is attempted and each successfully admitted barrier is awaited before the function propagates any stored error. Include/owner work cannot be skipped by an earlier watcher result. |
| Preserve the existing error channels | **ADDRESSED.** All three stage results are captured before Include `final_error` and watcher history are read. The previous return priority remains owner, watcher admission/join, Include admission/join, final Include error, then first remaining watcher failure. Nested cleanup diagnostics remain in Runtime; the change does not convert them into successful operation results or reclassify them. |
| Add a deterministic application-level regression | **ADDRESSED.** The new paused-time test at `crates/re-cordis-hmr/tests/system_lifecycle.rs:371–465` gates injecting-consumer cleanup after admitted watched refresh, polls `app.shutdown()` while held, and requires success after release. It proves a second provider and rebound consumer actually completed, verifies the old exact callable binding, terminal Include snapshots, resource/capture release, Registry/effect baselines and empty Runtime diagnostics. |
| Preserve forced ancestor-cancellation expectations | **ADDRESSED.** The two existing tests and their common `pending_shutdown` helper are byte-unchanged by the correction. Fresh execution on both toolchains preserves the retained `loader worker stopped` watch failure, invalid final journal/parser error and distinct original cleanup cause 301 assertions. |
| Do not hide the race using delay, history filtering or smoke readiness | **ADDRESSED.** No delay, readiness change, error filter, history-assertion relaxation or framework modification appears in the correction. The unchanged independent gate reproducer is the primary closure evidence. |

The captured results are the existing single-error return policy, not a new
aggregate-error API. Code inspection confirms that error propagation occurs
after the admitted teardown barriers. The unchanged demonstration still
drains root cleanup and reads Runtime diagnostics after its work result;
the unchanged forced-cancellation tests explicitly distinguish nested Runtime
diagnostics from root CleanupReport and watcher history.

The new regression's catalog assertion also preserves the ownership boundary:
the retained Include handle intentionally keeps the catalog alive until that
last handle is dropped. Its terminal snapshot/resource assertions are not
substitutes for successful completion of the refresh: the test separately
requires both replacement and consumer rebound.

## Independent commands and results

All Cargo executions were serialized under the exclusive reviewer lease.
For each toolchain, cargo, rustc, rustdoc, cargo-clippy and clippy-driver were
resolved with `rustup which --toolchain`; their versions were queried.
Execution used matching `RUSTUP_TOOLCHAIN`, explicit `RUSTC`/`RUSTDOC`,
the selected toolchain bin directory first in PATH, and isolated
`target/r0-rereview/build-{toolchain}` targets.

Stable cargo/rustc/rustdoc: **1.97.1**; Clippy **0.1.97**.
MSRV cargo/rustc/rustdoc: **1.85.0**; Clippy **0.1.85**.
Platform: aarch64 macOS. No components were installed.

| Fresh independent command, using resolved Cargo | stable | 1.85.0 |
|---|---|---|
| `run --manifest-path target/r0-review/shutdown-probe/Cargo.toml --locked --offline` | Exit 0; `Application shutdown result: Ok(())` | Exit 0; `Application shutdown result: Ok(())` |
| `test -p re-cordis-hmr --features include --test system_lifecycle --locked --offline` | 8 passed; 0 failed/ignored/filtered | 8 passed; 0 failed/ignored/filtered |
| `git diff --check 4742efe44a55dd1464eb851f9d9eb11d66fde82b..35556d5108eaf13104a5fb287c83b28ffb5d1576` | Passed once for the shared correction | Same source |

The eight-test run includes the added regression, both forced-cancellation
oracles, the bounded real watcher, runnable shared demonstration and 100
complete composed lifecycle cycles. This review did not repeat a full shared
matrix, documentation checker or paired-trace run.

Exact command arrays, selected paths/environment, versions, exit statuses,
timings and log paths are in `target/r0-rereview/tools.json` and
`target/r0-rereview/results.json`. Logs are
`deterministic-shutdown-{stable,1.85.0}.log` and
`integration-{stable,1.85.0}.log` in that directory.

## Evidence integrity and prose

Before and after the independent Cargo runs, SHA-256 checks matched all four
preserved artifacts to the correction's pre-change hash record: original
probe source, its manifest and both original independent RED logs. The
original probe was executed in place without editing it. The checked values
are saved in `target/r0-rereview/preserved-artifact-hashes.json`.

Current application/test hashes match the correction worker's verified
source hashes and are recorded in
`target/r0-rereview/verified-source-hashes.json`. Inspected correction-worker
RED logs on both toolchains fail at the intended clean-shutdown assertion
with the original loader-worker cause. These are inspected worker RED
executions; the GREEN executions in the table above are fresh independent
runs.

The controller prose correctly distinguishes orderly watcher-first shutdown
from forced ancestor cancellation, and retains filesystem, blocking-I/O and
macOS evidence limits. It adds no new API or TypeScript-equivalence claim.
The in-review/progress text is appropriate while final acceptance remains
pending. No change to the accepted generated-fixture design was necessary.

**Cargo lease released immediately after the independent probes and focused
suite completed, before the controller began its final shared matrix.
No reviewer Cargo process remains.** The reviewer wrote only this report
and ignored metadata/logs under `target/r0-rereview/`; no application,
framework, test, checker, template or original review artifact was modified,
and no commit, subagent or push was made.

## Remaining acceptance gate

The scoped correction is approved. The controller must still finish the
required explicit stable/1.85.0 shared matrix and actual generated external
applications on the frozen executable correction, then provide the final
evidence/status delta for narrow closure review. Earlier passing smoke runs
are not used as a substitute for those final executions.

## Final evidence-closure review — 2026-11-04

**Evidence approved for the final R0 status update and closing commit.**
This narrow read-only review completes the pending matrix/documentation gate
above. R0-R1 remains ADDRESSED; no factual mismatch or new evidence blocker was
found. No additional code review, Cargo command or shared test was run by this
reviewer during closure.

Compared [r0-acceptance.md](r0-acceptance.md) with the controller's
`target/r0-20261104/{matrix.json,tools.json,environment.json,final-inputs.json,final-summary.json}`,
the corresponding stage logs, [source dispositions](r0-source-map.md), original
review and correction reports. Recomputed test totals from the saved logs and
compared the current bytes of all 166 frozen inputs with their recorded hashes.

- All 40 recorded matrix stages exit 0: 22 stable and 18 MSRV, at executable
  `35556d5108eaf13104a5fb287c83b28ffb5d1576`. All 166 inputs still match.
  The tracked runner and executed target copy are byte-identical.
- Both workspace logs total 517 passed checks with zero failed or ignored:
  508 carried baseline plus eight R0 application tests and one loader regression.
  The 22 Create tests are five unit plus 17 scaffold tests; the 12 GitHub tests
  are three collector, three digest and six webhook tests.
- Each toolchain executes eight distinct example programs in nine invocations.
  Both documentation runs confirm 34 page pairs, 13 snippets, actual quick start,
  five basic generated-app tests and eight generated R0 tests, with default and
  real-watch execution. These subsets/copies/repeats are not added to 517.
- The saved core comparison contains six scenarios/73 observations on stable;
  both loader comparisons contain two scenarios/nine observations. Timer's
  four/16 remains carried prerequisite evidence. The report retains zero new
  whole-program Rust/TypeScript pairs, 147 B0 gaps, 65 primary loader G marks
  and the macOS-only runtime boundary; exclusions are not treated as passes.
- Tool paths/versions and host metadata match the report. The recorded runner
  selects rustup tools and matching environment, uses isolated target paths,
  and sets warnings-denied Rustdoc. M0 21 and HMR no-default 45 checks per
  toolchain match their separate logs.

At final inspection, `closure.json` records exit 0 for
`docs-final-regression`, `docs-final-stable` and `docs-final-1.85.0`.
The final checker logs independently show the same 34 pairs, 13 snippets,
five basic and eight R0 generated tests, and completed real-watch runs.
The checker regression log reports eight passing tests. These are inspected
controller executions after the prose updates, separate from this reviewer's
earlier independent probes.

README, AGENTS, todo, progress and matching integration-guide changes point to
the acceptance report without expanding source/platform claims. Generic
recurring gates remain future requirements rather than being reclassified as
additional completed runs. The controller may now record R0 accepted and commit
the closing documentation; no executable or generated-template change is
required by this assessment.
