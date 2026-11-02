# Core and evidence implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Preserve the delivered core and fix only demonstrated defects or
missing contracts needed by approved satellites.
**Architecture:** Existing Context, Runtime, Fiber, Effect, service and exporter
ownership remain authoritative. Evidence precedes any new core API.
**Tech stack:** Existing Rust 2024, Tokio local hosting and futures-util.
**Spec:** [Master design and global constraints](README.md).

## Global constraints

Rust edition 2024; resolver 3; minimum Rust 1.85, tested with 1.85.0. MIT;
version 0.0.0 and publish=false. All master constraints and the shared
[acceptance protocol](acceptance.md) apply. Core is delivered, not scheduled
for a rewrite.

## Evidence classes

- **Observed source gap:** current source directly contradicts a written
  contract. It still needs a failing executable regression before a fix.
- **Investigation:** a plausible failure path, not a confirmed runtime defect.
- **Representation:** an intentional, documented Rust replacement for JS.
- **Missing evidence:** a contract has no identified executable coverage; it
  is not automatically a production bug.

Timer observations are catalogued in [timer.md](timer.md). Do not inflate them
into a claim that the core lifecycle implementation is broken.

## B0 — Reproducible baseline and complete case disposition

**Files:** read docs/m0-cases.md, docs/m6.md, docs/lifecycle-parity.md,
docs/github-activity.md, .github/workflows/ci.yml; use
crates/re-cordis-core/tests/, tests/parity/ and tests/docs/.
Create docs/plans/2026-11-02-cordis-completion/evidence.md during execution.

**Consumes:** current public core APIs and pinned source/test declarations.
**Produces:** revision/toolchain-tagged results and a case ledger separating
executed comparisons, source-mapped Rust tests, representations and open gaps.

- [x] Run the shared baseline matrix on stable and 1.85.0 with explicit tools
  and separate artifact directories. Record actual test totals rather than
  copying historical counts.
- [x] Run six paired scenarios with the existing runner; verify 73 observations
  and preserve its source-pin/dirty-tree safeguards.
- [x] Reconcile the 144-declaration core ledger and events.types.ts inventory
  against current test names. Parameterized declarations are expanded before
  claiming each variant is covered.
- [x] Audit each semantic family: Context/metadata; plugin identity; effects
  and cancellation; five event modes; publication/retention; isolation and
  traced calls; update/recovery; logging/error identity; explicit injection.
  Map gaps to a concrete regression or an existing documented restriction.
- [x] Commit the evidence with no production change if no core defect is found.

**Acceptance (2026-11-03):** [independent B0 review](evidence/B0/review.md) accepts
the baseline and complete accounting with 147 tracked evidence gaps.

**Exit:** every existing core case has an explicit evidence class; unexpected
failures have a reproducer and owner work package. No new suite-wide parity claim.

## B1 — Make status and acceptance automation accurate

**Files:** docs/todo.md, docs/README.md, docs/implementation-plan.md,
docs/decision.md, docs/limitation.md, docs/llms.txt, README.md, AGENTS.md,
CHANGELOG.md; tests/docs/check.py and tests/docs/test_check.py only for actual
checker changes; .github/workflows/ci.yml for a reviewed CI addition.

- [ ] Replace current-status claims that no satellites exist or the checker
  is core-only. Preserve dated M0–M6 evidence as historical.
- [ ] Mark timer contract repair as open; do not erase its delivered code or
  successful historical tests. Correct the old timer spec's 20-case claim: the
  pinned timer test file has 22 syntactic declarations, not an executed count.
- [ ] Separate recurring acceptance requirements from per-slice completed
  checkboxes. Each delivery records its own commands and revision.
- [ ] Keep link checking over the new planning tree, including a regression
  for a deliberately broken plan link if permanent checker coverage is added.
- [ ] Decide whether to add Python doc regressions/guide execution to CI;
  current Rust CI does not run them. Add them only with explicit dependency
  restoration and per-toolchain results, never by relying on a warm cache.

**Exit:** public status, roadmap and machine-readable reading paths agree;
both language docs remain correct for actually available APIs.

## B2 — Conditional core bridge or targeted regression fix

**Timer scope accepted (2026-11-03):** reproduced HOST/retirement failures
justified the small owned-task and admitted-callback bridge. The final
[acceptance report](evidence/timer-acceptance.md) records independent review,
all core/timer regressions and explicit stable/1.85.0 verification. Future
features still need their own reproducer before further core changes.


**Entry:** T1/T2 or a satellite gate supplies a failing public-API reproducer.
Candidate files are crates/re-cordis-core/src/runtime.rs, context.rs, effect.rs,
error.rs and tests/foundations.rs or effect_metadata.rs. Service/loader issues
instead target service.rs/plugin.rs and their existing concern tests.

**Concrete timer question:** State::spawn installs HOST.scope; timer currently
uses raw tokio::task::spawn_local. Runtime::run explicitly documents that raw
spawned tasks lack admission authority. A callback that registers another
timer/effect, and completion that disposes its own Effect, exercise this gap.

- [x] Prove the failure under a real Runtime::run and compare intended callback
  behavior with pinned TS. Include wrong-host and disposed-owner refusals.
- [x] Try existing public effect/setup APIs first. If they cannot carry owned
  timer work with host attribution and joined cancellation, specify the smallest
  scoped bridge: admission, owner attribution, start/return boundary, callback
  panic reporting, terminal ownership and cancellation acknowledgment.
- [x] Review exact signatures and a regression before changing public exports.
  Do not expose private State or an unbounded general-purpose scheduler merely
  to bypass task-local checks.
- [x] Implement only the proven gap. Keep core's Tokio time/macros features
  unchanged unless independently required by core behavior.
- [x] Verify nested callbacks, wrong runtime, non-running host, late owner
  disposal, host destruction, dropped waiters and exact diagnostics.
- [x] Run the complete core compatibility and type-check matrix; document and
  commit any new public contract with both language references.

**Exit:** the reproducer passes without weakening admission or ownership. If
no core change is needed, record that result and close B2 for this slice. B2
is conditional work, not an automatic public task API commitment.
