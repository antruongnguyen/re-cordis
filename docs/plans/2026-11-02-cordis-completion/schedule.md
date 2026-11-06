# Implementation scheduling worksheet

Status: proposed dependency order, **no calendar jobs or automations created**.
Read [the master plan](README.md) first. Schedule implementation after reviewing
the work packages and their design decisions.

## Recommended waves

| Wave | Outcome | Packages |
|---|---|---|
| 0 | Trustworthy baseline and accurate roadmap | B0, B1 |
| 1 | Timer contracts and resource safety | T0–T5, conditional B2 |
| 2 | Usable console adapter | C0–C3 |
| 3 | In-memory loader and update/failure correctness | L0–L2, G0 |
| 4 | Groups, isolation, configuration and loader acceptance | L3–L5, G1 |
| 5 | Include plus watcher/reload groundwork | I0–I4, H0–H2 |
| 6 | HMR integration and scaffolding | H3, S0–S2 |
| 7 | Integrated acceptance and distribution decision | R0, R1 |

These waves preserve the timer→logger→loader preference. Their order is a
recommendation; hard prerequisites below are the scheduling constraints.
S0–S1 can move earlier after C3/L5 for the basic template. HMR/include may
overlap only after their interfaces are reviewed.

## Work-package graph and review boundaries

Sizing is relative planning risk, **not a time estimate**: S is a narrow
contract/check; M is a cohesive behavioral slice; L needs several reviewed
test-first tasks. Split each L at its gate before assigning dates. All rows
begin Planned; completion requires linked evidence and a commit.

| ID | Deliverable | Hard prerequisites | Size | Exit/review point |
|---|---|---|---|---|
| B0 | Fresh baseline and core case ledger | None | M | Actual toolchain/test/trace evidence |
| B1 | Status and documentation accuracy | B0 | S | Current claims and checker/CI decisions agree |
| T0 | Timer decisions and source map | B0 | M | Truth table and 22 declarations classified |
| T1 | Admission/range/deadline behavior | T0 | M | Refusal/clock regressions; HOST issue reproduced |
| B2 | Conditional minimal core bridge/fix | Failing reproducer from T1 or later gate | M | Scoped authority tests pass, or no change justified |
| T2 | Hosted one-shot ownership and joins | T1; B2 if required | L | Effect/capture baselines and joined release |
| T3 | Stream terminal/bounded behavior | T2 | M | Stop/fail/drop/owner cancellation matrix |
| T4 | Reentry, panic and callback release | T2 | M | No borrow panic; exact diagnostics and release |
| T5 | Timer delivery evidence | T3, T4, B1 | M | Full acceptance, example and paired docs |
| C0 | Console configuration/API decisions | B0 | M | Writer/render/default/error policy accepted |
| C1 | Deterministic renderer | C0 | M | Exact byte fixtures and Unicode cases |
| C2 | Scoped writer/exporter | C1 | M | I/O failure and removal/cleanup fixtures |
| C3 | Console delivery | C2 | S | Example, source map and full acceptance |
| L0 | Loader API/entry/config decisions | B0 | L | Typed factory and lifecycle contract reviewed |
| L1 | Catalog and in-memory entries | L0 | M | Boot/remove and identity tests |
| L2 | Update/readiness/rebuild/failure | L1 | L | Gated races and per-entry failure isolation |
| G0 | Group contract/packaging | L0 | S | Loader-owned behavior and import decision |
| L3 | Groups, moves and isolation | L2, G0 | L | Retained provider and adoption tests |
| G1 | Group public behavior acceptance | L3 | S | Structural/disabled/group settlement evidence |
| L4 | Config decoding/evaluation/source seam | L3 | L | Validation timing and authority fixtures |
| L5 | Loader delivery/integration seam | L4, G1 | M | Example, eleven-suite disposition and acceptance |
| I0 | Include parser/conflict/durability gate | L5 | L | Parser capability fixtures and signatures |
| I1 | File read/refresh/location | I0 | M | Missing/invalid/nested source tests |
| I2 | Patches/provenance/anonymous IDs | I1 | L | Effective owner and ordered patch tests |
| I3 | Reconciliation and persistence | I2 | L | Conflict/retry/write-back/failed-write fixtures |
| I4 | Include delivery and joined flush | I3 | M | Seven-suite disposition and acceptance |
| H0 | HMR capabilities/watcher contract | T5, L5, I0 | L | Config vs linked-code vs source-restart contract |
| H1 | Scoped watcher/config callbacks | H0, I1 | M | Bounded queue, paths, rename and cleanup |
| H2 | Linked factory replacement | H0 | L | Nested identity, prevalidation, teardown races |
| H3 | HMR delivery/integration | H1, H2, I4 | M | Fallback and integrated acceptance |
| S0 | CLI/template/distribution contract | C3, L5 | S | Generated-file manifest and local dependency plan |
| S1 | Offline scaffold | S0 | M | Generated app builds/runs on both toolchains |
| S2 | Transport disposition/delivery | S1 | M or L with remote transport | Source cases and scaffold acceptance |
| R0 | Whole-program integration | T5, C3, L5, G1, I4, H3, S2 | L | Causal lifecycle/I/O/resource evidence |
| R1 | Release readiness proposal | R0 | M | Packaging/semver/platform decision document |

B2 is a conditional service to other packages, not a dependency loop. Attach
the exact reproducer and finish that fix before its caller resumes. G1 follows
L3 and precedes L5; L3 must not wait on G1.

## Scheduling rules

1. Start with B0, then T0. Schedule implementation only after its public
   contract and dependency choices are reviewed.
2. Default to one active implementation slice plus review. Parallel work is
   optional: C0/L0 source review can overlap timer repair; H1/H2 can overlap
   after H0; scaffolding can overlap late include/HMR after C3/L5.
3. Assign one owner per shared file set. Timer tasks share src/lib.rs; loader
   tree/group/config tasks share interfaces. Cargo manifests/lockfiles and
   status docs need a single integration owner.
4. Reserve review, test and documentation time in every delivery estimate.
5. Re-estimate after each wave from actual throughput and newly found defects.
   Do not turn L-sized discovery work into a fixed deadline.
6. Advance after the exit checklist and evidence commit. Test failures,
   unresolved contracts and parser limitations change the schedule; they do
   not disappear into a completed checkbox.

## Calendar assignment record

When choosing dates, record for each selected ID: executor/owner, prerequisite
commits, available engineering hours, start/end window, review owner/window,
check commands, checkpoint artifact and next gate. Obtain desired cadence and
time zone before creating a job. Planning time zone: Asia/Ho_Chi_Minh.

A future scheduled implementation prompt should identify one work package:

> Execute work package T1 from docs/plans/2026-11-02-cordis-completion/timer.md.
> Read the master constraints, accepted T0 decisions and acceptance checklist.
> Confirm prerequisite evidence, implement named regressions and the minimal
> fix, update paired docs, run required checks and commit the verified slice.
> Report changed behavior, evidence and unresolved gates. Do not start another
> satellite merely because time remains.

For design gates request a decision document, not production code. Scheduled
work must not invent approval for pending public APIs, serialization policy,
native reload mechanisms or releases.
