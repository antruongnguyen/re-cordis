# B0 baseline and core case-ledger execution plan

> For agentic workers: use superpowers:executing-plans for the assigned task.
> The orchestrator owns dispatch and review. Do not spawn agents or commit
> while another writer is active in the shared checkout.

**Goal:** Establish a fresh executable baseline and an honest disposition of
every pinned core declaration without changing production behavior.

**Architecture:** A matrix worker owns execution and logs; a ledger worker
independently reads source and maps concrete obligations to current Rust tests
or restrictions. The ledger joins execution results before independent review.

**Tech stack:** Rust 2024, rustup stable/1.85.0, Python standard library,
and the existing Corepack/Yarn/Yakumo/Vitest runner.

**Spec:** [Core B0](../../core.md), [acceptance](../../acceptance.md),
[master constraints](../../README.md), [schedule](../../schedule.md),
[execution decisions](../../progress.md).

## Constraints and verified planning observations

- Preserve Rust 2024, resolver 3, MSRV 1.85.0, MIT, version 0.0.0,
  publish=false, local non-Send hosting, provider retention, Pending/readiness/
  Failed distinctions, Effect/Fiber Drop semantics, and six paired scenarios.
- B0 is evidence only: no production, existing tests/fixtures, manifests,
  lockfiles, CI, bilingual guides or historical checklist changes. B1 owns
  status corrections; demonstrated production defects get a separate task.
- No live GitHub, credentials, push, publication or public ingress. Preserve
  the existing fake-process and signed-loopback integration tests.
- Use CodeGraph first for indexed source. Qualify paths to distinguish
  TypeScript specifications from this project's Rust tests.
- Planning inspected source and inventories only; no baseline tests ran here.

Planning observations on 2026-11-02:

| Item | Observed value |
|---|---|
| Branch / HEAD | feat/cordis-completion / 6ddbae2e8505c8cae6217cbec36c798921f1edd9 |
| Starting plan commit | c5019cc71286273b994c29cdb03b3fd79ab63b24 |
| Baseline implementation | 808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f |
| Fixed TS HEAD | 35fdc969e4b6c48c48ed44717c7a018324e4dd88 |
| Initial status | Root and TS checkout clean before this plan |
| Delta since implementation | Documentation only; crates, tests, manifests and locks unchanged |
| Core inventory | 144 declaration sites in 15 files; eight it.each sites expand to 156 registered cases before in-body obligations |
| Event types | Six expression obligations at lines 13, 14, 17, 19, 21, 22; registered signature at line 9 |
| Expected trace lines | combined 11; coalesced 14; removed 12; superseded-failure 10; latest-failure 13; explicit-recovery 13; total 73 |
| CI | Both toolchains check/test/M0; lint and formatting stable only; no Python guides or paired runner |
| JS pin boundary | Yarn packageManager is 4.14.1; upstream does not track a transitive Yarn lockfile |

These are source observations, not passing-test claims. Capture the actual
execution HEAD and versions anew. A later documentation-only HEAD is acceptable
if evidence proves executable inputs unchanged; name the exact revision used.

## Ownership and concurrency

Paths below are relative to the completion-plan directory.

| Assignment | Mandatory model/effort | Exclusive write ownership |
|---|---|---|
| Planner | gpt-6-astra / high | evidence/B0/plan.md |
| Matrix executor | gpt-6.1-sol / high | evidence/B0/matrix/ |
| Ledger executor | gpt-6.1-sol / high | evidence/B0/ledger/ |
| Independent reviewer | gpt-6-astra / high | evidence/B0/review.md |
| Integration owner | orchestrator | evidence.md, progress.md, final local commit |

Matrix and ledger **can run independently in parallel** while executable
inputs are frozen. Only matrix invokes Cargo, docs/parity execution, dependency
restoration or temporary upstream fixture creation. Ledger reads source and
initially marks execution references pending, then joins matrix logs. No second
parity process may use the same upstream checkout. Matrix serializes docs and
parity because both internally use target/stable; acceptance builds use
target/acceptance-stable and target/acceptance-1.85.0. Workers do not edit shared
status docs or commit. Reviewer starts when both outputs settle.

## Task 1: Fresh matrix and preserved run evidence

**Owner:** Matrix executor. Read [M6](../../../../m6.md),
[lifecycle parity](../../../../lifecycle-parity.md),
[GitHub consumers](../../../../github-activity.md),
[documentation checks](../../../../README.md), and repository CI.

**Outputs:** matrix/report.md, commands.tsv, environment/source snapshots,
full per-command logs, and optional evidence-local execution script. Record
argv, cwd, explicit environment, start/end UTC, exit code, and stdout/stderr.
Preserve failed attempts instead of overwriting their logs on retry.

- [ ] Capture git HEAD/branch/status and delta from 808c9f5, OS/architecture,
  Python/Node/Corepack/Yarn versions, and installed upstream TypeScript,
  ESLint, Vitest, cosmokit, tsx and Yakumo versions.
- [ ] Require exact TS HEAD and clean tracked TS tree; require absence of
  local/cordis/packages/core/tests/re-cordis-lifecycle-parity.spec.ts.
  Never delete a pre-existing fixture to make the runner start.
- [ ] Hash Cargo.lock, spikes/m0/Cargo.lock, tests/parity/run.py,
  tests/parity/lifecycle.spec.ts, six tests/parity/*.txt files and
  crates/re-cordis-core/tests/lifecycle_parity.rs before/after. Record any
  untracked upstream yarn.lock hash separately.
- [ ] Check Unix fixture prerequisites /bin/sh, mkfifo, /bin/kill and head.
  Record compiler wrappers/flags or Cargo configuration overrides. Missing
  selected rustup components or offline dependencies are setup failures.
  Restore only missing components/dependencies, log setup and recheck the pin
  and tracked tree; never fall through to system tools or delete caches.
  Documented JS restoration is: (cd local/cordis && corepack yarn install).
- [ ] Execute the complete acceptance matrix below from repository root.
  Wrap individual commands in logging without masking their exit codes.
  Missing setup or a failed command is not a passed check. Continue independent
  checks explicitly if useful, listing anything not run.

~~~bash
set -euo pipefail
for cordis_tc in stable 1.85.0; do
  (
    set -euo pipefail
    cordis_rustc=$(rustup which --toolchain "$cordis_tc" rustc)
    cordis_cargo=$(rustup which --toolchain "$cordis_tc" cargo)
    cordis_rustdoc=$(rustup which --toolchain "$cordis_tc" rustdoc)
    cordis_clippy=$(rustup which --toolchain "$cordis_tc" cargo-clippy)
    cordis_driver=$(rustup which --toolchain "$cordis_tc" clippy-driver)
    cordis_bin=$(dirname "$cordis_rustc")
    test "$(dirname "$cordis_cargo")" = "$cordis_bin"
    test "$(dirname "$cordis_rustdoc")" = "$cordis_bin"
    test "$(dirname "$cordis_clippy")" = "$cordis_bin"
    test "$(dirname "$cordis_driver")" = "$cordis_bin"
    export PATH="$cordis_bin:$PATH"
    export RUSTC="$cordis_rustc" RUSTDOC="$cordis_rustdoc"
    export CARGO_TARGET_DIR="$PWD/target/acceptance-$cordis_tc"
    export RUSTDOCFLAGS='-D warnings'
    "$cordis_rustc" --version
    "$cordis_cargo" --version
    "$cordis_rustdoc" --version
    "$cordis_clippy" --version
    "$cordis_driver" --version
    "$cordis_cargo" check --workspace --all-targets --locked
    "$cordis_cargo" test --workspace --locked
    "$cordis_cargo" clippy --workspace --all-targets --locked -- -D warnings
    "$cordis_cargo" doc --workspace --no-deps --locked
    "$cordis_cargo" test --manifest-path spikes/m0/Cargo.toml --locked
    "$cordis_cargo" clippy --manifest-path spikes/m0/Cargo.toml --all-targets --locked -- -D warnings
    if test "$cordis_tc" = stable; then
      cordis_fmt=$(rustup which --toolchain stable cargo-fmt)
      cordis_rustfmt=$(rustup which --toolchain stable rustfmt)
      test "$(dirname "$cordis_fmt")" = "$cordis_bin"
      test "$(dirname "$cordis_rustfmt")" = "$cordis_bin"
      "$cordis_rustfmt" --version
      "$cordis_cargo" fmt --all --check
      "$cordis_cargo" fmt --manifest-path spikes/m0/Cargo.toml --check
    fi
    "$cordis_cargo" run -p re-cordis-core --example lifecycle --locked
    "$cordis_cargo" run -p re-cordis-timer --example timer_lifecycle --locked
  )
done
python3 -B tests/docs/test_check.py
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
python3 tests/parity/run.py
~~~

The existing workspace includes the bounded GitHub application tests; no live
collector or user-secret listener is needed. The extra lifecycle/timer example
commands above exercise their actual applications. Cargo check builds all
targets but does not itself execute examples.

- [ ] Derive actual counts from complete output separately for each toolchain:
  units, integration suites, Rustdoc pass/fail examples, M0, doc regressions,
  unique guide snippets and external quick start. Include ignored/filtered/
  failed counts. Historical hints: 164 core integration tests including 12
  Unix application tests; 15 timer tests; 17 core Rustdoc checks; M0 20 plus
  one Rustdoc; five Python regressions; eight guide snippets plus quick start.
  Do not substitute these hints for observed counts or count repeated parity
  runs as additional tests.
- [ ] Require paired-run exit 0, six TS tests, six Rust tests, all six named
  comparison summaries and exactly 73 observations with the per-scenario
  counts above. Preserve unsorted exact traces and in-language original-error
  identity/diagnostic assertions; do not infer their ordering from shared lines.
- [ ] Audit and retain the unmodified runner's pin check, tracked-diff refusal,
  exclusive fixture creation, finally cleanup, explicit stable tools,
  upstream ESLint/TypeScript commands, existing Corepack/Yakumo/Vitest route,
  timeouts and exact comparison. Do not substitute Bun/npm/direct Vitest or
  relax guards. Verify fixture absence, TS tracked cleanliness and hashes after
  success or failure. Runner logs plus unchanged expected-file hashes suffice;
  emitted temporary traces are intentionally deleted by the existing runner.
- [ ] One positive paired run satisfies B0. Historical repeated runs, negative
  expectation check and 61-test upstream subset remain historical unless
  actually rerun. A changed runner would require separate negative-guard
  validation; this task does not change it.

**Exit:** All required checks have attributable results with complete logs;
six/73 preserved; executable inputs unchanged; any failure or skipped platform
coverage explicitly recorded. No green baseline while required checks fail.

## Task 2: Complete declaration and event-type disposition

**Owner:** Ledger executor. Read [historical checklist](../../../../m0-cases.md),
M2–M6 source maps, all 15 pinned core spec bodies/helpers, events.types.ts,
relevant TS implementation, and current Rust tests/Rustdoc. Checkmarks and test
names are search leads, not proof. The detailed audit belongs to this executor.

**Resume clarification (2026-11-02 user request):** Apply the master plan's
[reuse rule](../../README.md#prior-decisions-and-evidence-reuse) first, including
[decisions](../../../../decision.md), [limitations](../../../../limitation.md)
and the [previous plan](../../../../implementation-plan.md). Start each family
from its existing M2–M6 mapping and accepted B0 fragments. The source list above
defines the audit's coverage, not a requirement to repeat a previously
substantiated semantic evaluation. Carry forward sufficient mappings with
their original document/section, exact case/variant, Rust assertion or trace,
revision and current matrix join, using existing evidence/rationale fields.
Read uncovered or mismatched bodies/helpers to supply missing provenance;
do not infer assertions from checkmarks. Preserve historical M6 delivery while
finishing the finer B0 ledger. Independent review verifies this reconciliation
and its new findings without reopening accepted unchanged design decisions.

**Outputs:** ledger/cases.json (144 declaration records), event-types.json
(six expression obligations), report.md (family findings/counts/gaps), and a
small standard-library validate.py for inventory/reference consistency.

- [ ] Reconcile unique source filename/line/title/enclosing suite against all
  historical rows. Use stable IDs such as core/service-dispose.spec.ts:95.
  Per-file declaration counts: associate 6; decorator 1; dispose 13; events 19;
  fiber 22; intercept 7; invoke 2; isolate 6; logger 11; plugin 10; reflect 4;
  service-dispose 17; service 8; shadow 8; update 10. Total 144.
- [ ] Expand these exact parameter sets before classifying obligations:

| Source site | Parameters |
|---|---|
| service-dispose.spec.ts:31 | action = consumer, provider, root |
| service-dispose.spec.ts:95 | action = provider, root, provider first, consumer first, restart |
| service-dispose.spec.ts:156 | fail = false, true |
| service-dispose.spec.ts:238 | ownerShutdown = false, true |
| service-dispose.spec.ts:323 | rootShutdown = false, true |
| service-dispose.spec.ts:386 | withdrawnFirst = false, true |
| update.spec.ts:7 | inject = false, true |
| update.spec.ts:96 | mode = restart, config ABA |

This yields 27 service-dispose and 12 update registered cases, 156 total.
It is source expansion, not a B0 TS execution count. Do not invent a Cartesian
product across unrelated declarations. Nonparameterized sites have one default
variant; in-body assertion paths are obligations within it.

- [ ] Read all in-body branches. For example events.spec.ts:18 covers symbol
  emit/bail/serial/parallel and disposal, not waterfall; line 46 covers absent
  toString, registered __proto__/toString/constructor, removal and bucket
  release. Each dispatch test's empty/filter/error/settlement paths need
  explicit evidence. Repeated assertion loops are not extra declarations.
- [ ] Link exact Rust file, test function or Rustdoc item, assertion lines and
  reason the assertion covers each parameter/behavior. Inspect helpers and
  Rust loops. Candidates already found include updates.rs function
  all_concurrent_waiters_follow_latest_config_with_and_without_dependencies
  and service_dispose.rs function
  provider_resources_survive_all_disposal_orders_and_restart. Confirm their
  assertions and fresh results; a matching name is insufficient.

### Record format and evidence classes

Each declaration record has id, source_file, source_line, title, suite,
variants and derived classes. Each variant has parameters and obligations.
Each obligation has id, concrete behavior, source_assertion_lines, class,
rust_evidence (file/symbol/lines/assertion/command IDs), paired_evidence
(scenario/observation/scope), restriction (document/anchor/reason or null),
gap (ID/kind/concrete next action/owner/work package or null), execution_status
and rationale. Use passed/failed/not-executed/not-applicable; pending-matrix
is allowed only before the execution join. Split mixed obligations instead of
letting one classification conceal another.

| Class | Required basis | Boundary |
|---|---|---|
| executed-comparison | Exact obligation exercised by a successful current paired fixture | Does not cover every related source variant |
| source-mapped-test | Reviewed TS-to-Rust assertion/variant mapping plus current Rust result | TS counterpart was not thereby executed |
| rust-representation | Documented deliberate Rust substitute and current substitute test/compile evidence | Not JS mechanism compatibility |
| unsupported | Exact existing restriction, rationale and review disposition | Never counted as parity or silently newly excluded |
| missing-evidence | Insufficient assertion, variant mapping or substitute evidence; named next action | Not a confirmed production defect without reproducer |

Parent classes are the set of child classes. Keep mixed declarations mixed.
Use additional gap kinds observed-source-gap, investigation or confirmed-failure
as appropriate; these do not invent execution. After matrix completes, every
claimed Rust test must appear in its suite's passing output on both toolchains.
Match Rustdoc item/file/line and compile-pass versus compile-fail outcomes.
Compilation of a file, disabled tests and mere function presence are not passes.

- [ ] Inventory event-types.ts separately: line 13 registered-symbol typed
  bail/result; 14 typed listener parameter; 17 rejected dispatch argument;
  19 rejected listener parameter; 21 dynamic heterogeneous emit; 22 dynamic
  listener. Link the registered signature at line 9. Current Event Rustdoc
  contains wrong-argument rejection and a runnable typed symbol/listener/result
  example; locate listener mismatch and Vec<hooks::Value> evidence separately.
  Dynamic Value lists are an intentional Rust representation. These six
  expressions are not runtime tests; the parity fixture's tsc invocation does
  not check this separate source file. Inventory does not require new TS runs.
- [ ] Audit all semantic families with a short finding and gap list:
  Context/metadata; plugin identity; effects/cancellation; all five event
  modes; publication/retention; isolation/traced calls/association; update/
  recovery; logging/error identity; initialized factories/explicit injection.
  Start in core tests foundations, plugins, effects, effect_metadata, events,
  hooks, services, service_dispose, scoping, updates, logger, acceptance and
  lifecycle_parity. Do not cite M0 models/reference-port/application tests as
  unmarked substitutes for current core-production coverage.
- [ ] Validate exact 144 unique declarations, per-file counts and checklist
  reconciliation; eight exact parameter sets/156 variants; at least one
  obligation per variant; six exact event-type expression lines; valid classes;
  class-required evidence/restriction/gap fields; existing files/anchors/line
  bounds; matching matrix test results; no pending execution links; derived
  summary totals. Run:

~~~sh
python3 -B docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/validate.py
git diff --check
~~~

Validation proves accounting/references, not semantic equivalence. Record its
command/output/status in ledger/report.md. Report declaration, registered-case,
obligation, type-obligation, actual-Rust-test and paired-scenario totals
separately; never report a combined parity percentage.

**Exit:** Every declaration/type expression has a current disposition, all
parameters and meaningful assertion paths are covered or explicitly missing,
and gaps have concrete regression/decision actions and owners. A historical
checkmark may become missing evidence without rewriting historical documents.

## Task 3: Independent review and integration

- [ ] Reviewer checks actual binary selection, full matrix/logs, suite counts,
  before/after hashes, pin/cleanup safeguards, and exact six/73 results.
- [ ] Review all eight parameter families, six event-type obligations, paired
  mappings, and every representation/unsupported/missing-evidence disposition.
  Inspect ordinary assertion mappings across every semantic family; expand
  sampling on any mismatch. Confirm duplicate titles are disambiguated.
- [ ] Confirm evidence-only diff and valid evidence Markdown links/anchors.
  The existing guide checker does not automatically prove all new planning
  links were visited; use its link helper directly without changing scope.
- [ ] Write review.md with accepted, accepted-with-tracked-gaps, or
  changes-required; findings identify source, evidence, owner, correction and
  scheduling consequence. Orchestrator creates evidence.md index, updates
  progress.md after review, and commits the verified B0 evidence locally.

**Accepted:** Required baseline passes, unchanged six/73 comparison, complete
honest inventory/type accounting, and no unowned defect.

**Accepted with tracked gaps:** Baseline passes and all cases are accounted
for, but bounded missing evidence or existing exclusions remain. Each has an
owner, concrete next action and work-package gate. A gap affecting a dependent
slice blocks that slice; B0 accounting is not implementation of the gap.

**Changes required / baseline incomplete:** Required failure or omission,
unverified toolchain, unexpected skips, wrong/dirty pin, changed fixtures,
missing scenarios/observations/logs, unexpanded variants, nonexistent test
claims, or exclusions counted as parity. Preserve the failed command as the
reproducer. Setup can be repaired/rerun in B0; production defects require a
separate reproducible task before claiming a green baseline.

Each executor returns owned paths, revisions, actual outcomes or source-only
work, changed-file list, counts with denominators, restriction/gap IDs and
owners, outstanding dependency on the other worker, and review readiness.
