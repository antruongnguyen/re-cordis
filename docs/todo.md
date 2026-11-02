# Roadmap / TODO

A living plan for porting [Cordis](https://github.com/cordiverse/cordis) to
Rust. The source of truth is the TypeScript `cordis` core
(`local/cordis/packages/core/src/`). This list will be reordered and refined as
we learn; it is not a contract. Detailed source evidence, design gates,
milestone prerequisites, and exit criteria live in the
[implementation plan](implementation-plan.md).

## Completion planning (2026-11-02)

Execution follows the [2026-11-03 Rust implementation direction](decision.md#2026-11-03--deliver-cordis-features-through-idiomatic-rust-contracts):
deliver usable feature slices with idiomatic Rust contracts, tests and examples,
then review the implementation. Track adaptations and limitations explicitly;
B0's evidence gaps are not a blanket prerequisite for satellite delivery.

- [x] Save a [source-backed completion plan](plans/2026-11-02-cordis-completion/README.md)
  for core evidence, timer repair and all seven pinned upstream satellites.
  Separate observed source gaps, investigation, Rust representations and
  missing evidence; index all 323 satellite test declaration lines.
- [x] Complete B0’s baseline and case ledger with [independent acceptance](plans/2026-11-02-cordis-completion/evidence/B0/review.md) and 147 tracked evidence gaps.
- [x] Implement T0–T5 timer repair: checked eager deadlines, hosted task authority,
  self-retirement, bounded streams, reentry and joined owner capture release.
  Shared review/toolchain acceptance is recorded in the
  [timer report](plans/2026-11-02-cordis-completion/evidence/timer-acceptance.md).
- [ ] Review and schedule the [work packages](plans/2026-11-02-cordis-completion/schedule.md):
  loader/group, include, HMR, create, then integrated acceptance.
  API/dependency choices and calendar dates in this program are not frozen.

The older milestones below retain their historical delivery evidence. A
checked historical box does not override an open repair in the completion plan.

Current execution: **B0 is complete and independently accepted with tracked
gaps on 2026-11-03**. All correction, scoped review, non-obligation accounting,
ledger validation and final acceptance gates passed. The ledger preserves
144 declarations / 156 variants, 540 runtime obligations, six type obligations
and seven separate non-obligations. It records 147 owned evidence gaps; these
are not demonstrated core defects or implemented follow-ups.

[Final acceptance](plans/2026-11-02-cordis-completion/evidence/B0/review.md)
confirms normal validation, 36 negative checks, exact reviewed assembly and
preserved matrix/report, FR-1, first-two-events and recovered-validation evidence.
The six paired scenarios / 73 observations remain unchanged. The subsequent
logger C0–C3 delivery is complete with independent approval and stable/1.85.0
acceptance; see [logger evidence](plans/2026-11-02-cordis-completion/evidence/logger-acceptance.md).
Timer T0–T5 and its required minimal B2 task bridge are independently accepted,
with 48 timer tests/three Rustdoc checks and six focused core tests.
The full stable/1.85.0 matrix and both paired fixtures passed.
All remaining 23 packages stay planned. No background
continuation is scheduled. See
[detailed progress](plans/2026-11-02-cordis-completion/progress.md#resume-next)
and [B0 evidence](plans/2026-11-02-cordis-completion/evidence.md).

## Planning and project setup

- [x] Initialize documentation (`AGENTS.md`, `README.md`, `docs/*`).
- [x] Review and pin both references, including independent read-only agent
  reviews; write and source-check the initial core implementation plan.
- [x] Select Rust edition 2024 as the initial baseline (no older edition).
- [x] M0: Inventory all pinned core test declarations and exercise lifecycle,
  typed access/tracing, and event/config API candidates in `spikes/m0`.
  Record local-first execution, cancellation/dispatch restrictions, and the
  production handoff in [m0.md](m0.md); parity cases remain pending M2-M6.
- [x] Choose MIT licensing (see `LICENSE`).
- [x] M1: Add only `re-cordis-core` to the root Cargo workspace (edition 2024,
  resolver 3, Rust 1.85 minimum), explicit local hosting, Context/metadata/root
  cleanup foundations, integrated ownership/error checks, and stable/MSRV CI.
  Supported APIs and scope are documented in [m1.md](m1.md).

## Core delivery (`re-cordis-core`)

M2-M6 depend on their predecessor; see the implementation plan for the
additional design gates and exact tests. Each slice lands with documentation
and behavioral tests, not just public type declarations.

- [x] M2 (root scope): Effect setup/rollback/composition, synchronous disposal,
  labels/tree introspection, five typed dispatch modes, listener snapshot
  filtering/cleanup, and core diagnostics. See [m2.md](m2.md) for contracts and
  explicit Rust equivalents. M3 now integrates plugin-owned disposal; the
  special internal-update listener protocol is implemented under M5. These are not
  claims of complete plugin/event/service suite parity.
- [x] M3 (base lifecycle): Registry identity/grouping, observable Fiber states,
  typed source/resolved validation, supervised restart/disposal, parent-owned
  child/listener/exporter cleanup, and reusable-root disposal; see [m3.md](m3.md).
  M4 adds dependency-driven Pending; M5 adds config updates/recovery and internal
  status/update hook protocols. Complete upstream suite parity is not claimed.
- [x] M4 (global slots): Reflect/Service typed publication and lookup, injection resolution,
  Pending/Active convergence, committed snapshots, and exact-provider retention
  through consumer cleanup (including consumers removed from the Registry).
  See [m4.md](m4.md). Isolation-dependent service cases and complete reflection/
  internal hook protocols remain assigned to M5/M6, not claimed as suite parity.
- [x] M5 (explicit Rust adapters): Isolation boundaries, intercepts, definition/use-site tracing,
  explicit accessor/mixin/associated/callable equivalents, internal hooks,
  update middleware, concurrent latest-config convergence, and the remaining
  isolation-dependent service-disposal cases. See [m5.md](m5.md) for contracts,
  safety deviations, and explicit replacements for JavaScript reflection.
  Remaining case-level evidence is reviewed under M6, not inferred from API names.
- [x] M6 (explicit Rust acceptance): Core Logger behavior, decorator-injection equivalents,
  compile-time API checks, an end-to-end example, and a parity/deviation review.
  See [m6.md](m6.md). Checked cases include documented Rust representations,
  not unrestricted JavaScript object/property/reflection compatibility.

## Next implementation targets: satellites

The **2026-10-27** roadmap selected timer → console logger → loader.
The user's **2026-11-03** execution direction selected console before timer
repair. **`re-cordis-timer` and `re-cordis-logger` are implemented** (see below);
console C0–C3 and timer repair T0–T5 are independently accepted. Loader and other
remaining entries are future targets, not available APIs.

The TypeScript specification remains pinned to
`35fdc969e4b6c48c48ed44717c7a018324e4dd88` in `local/cordis`.
The independent `local/cordis-rs` at
`1754062cef67cfc119fc7a2084c0321a231b9092` is an engineering reference only;
its redesigned timer/loader contracts and older TS parity pin are not our spec.

| Target | Actual prerequisites | Initial focus | Exit evidence |
|---|---|---|---|
| `re-cordis-timer` | Existing core effects, explicit local hosting and lifecycle | Owner-scoped time operations with explicit cancellation and joined cleanup | Source-case map, deterministic clock/gate tests, runnable example, bilingual guide/API |
| `re-cordis-logger` | Existing core Logger/exporter APIs | Console/export adapter, not a second logging core | Exact captured output/routing/error tests and scoped exporter teardown |
| `re-cordis-loader` | Existing core plugins, validation, injection and cleanup | Explicit Rust plugin registration/resolution and bounded configuration loading | Fixture-backed load/update/rollback/teardown tests with documented resolution limits |

This is a preferred sequence, not a false dependency chain: logger does not
require timer, and loader should not depend on either unless an approved
contract demonstrates the need.

### 1. re-cordis-timer — implemented with repaired contracts

Initially delivered on **2026-10-27** as an unpublished `crates/re-cordis-timer`.
The [2026-11-02 repair plan](plans/2026-11-02-cordis-completion/timer.md) records
the resolved T0 contracts and executed TF1–TF10 regression evidence.
The [acceptance report](plans/2026-11-02-cordis-completion/evidence/timer-acceptance.md)
records independent review and shared verification. Historical design and plan: [spec](superpowers/specs/2026-10-27-re-cordis-timer-design.md),
[plan](superpowers/plans/2026-10-27-re-cordis-timer.md). Decisions and
deviations are in [decision.md](decision.md#timer-satellite) and
[limitation.md](limitation.md#timer-satellite).

- [x] Review `packages/timer/src/index.ts` and `packages/timer/tests/index.spec.ts`
  at the TS pin. Inventory callback timeout/interval, awaited timeout, interval
  iterator next/return/throw/disposal, throttle, debounce and deprecated aliases.
  Map every source case to a Rust equivalent or an explicit limitation; do not
  silently replace the TS API with the independent port's sleep/timeout design.
- [x] Show a concise context map, proposed public Rust API, ownership/cancellation
  rules, dependencies/features, exclusions and measurable exit criteria.
  Review the bounded scope before implementation; do not scaffold other crates.
- [x] Implement the agreed slice in an unpublished `crates/re-cordis-timer`, keeping
  Rust 2024/MSRV 1.85 and reusing core effects. Timer-required Tokio features
  belong to the timer crate; do not add a hidden runtime or expand core's
  production dependencies speculatively.
- [x] Reconcile and verify duration admission, zero/overflow behavior,
  first/subsequent tick behavior, callback errors/panics, once-only delivery, cancellation, dropped
  waiters and owner disposal. Resolve TS wall-clock versus Tokio clock behavior
  explicitly, especially throttle/debounce. Document omitted overloads/aliases.
- [x] Complete coverage of just-before/at-deadline behavior, manual/owner
  cancellation, restart, acquired-resource cleanup, no callbacks after joined disposal, concurrent
  iterator exclusions, and source-backed throttle/debounce edges for included APIs.
  Use controlled time and channels, not wall-clock sleeps.
- [x] Add a runnable lifecycle example, public Rustdoc, and English/Vietnamese
  timer documentation.
- [x] Complete timer compile-time API checks (sealed trait, static captures, local Sleep).
- [x] Record final shared acceptance in the timer report after independent review
  and stable/1.85.0 verification.

### 2. re-cordis-logger — implemented and independently accepted

The [C0–C3 work package](plans/2026-11-02-cordis-completion/logger.md) records
owned writer/renderer delivery, 21 targeted tests, the three-line runnable
example and completed independent/integrated acceptance. Console was selected
before timer repair under the 2026-11-03 useful-Rust direction.

- [x] Review TS `packages/logger-console/src/{index,shared,browser}.ts` and
  `packages/logger-console/tests/index.spec.ts`. Record why the Rust crate is
  named `re-cordis-logger` while the upstream satellite is `logger-console`.
- [x] Agree on writer ownership, formatting/level routing, color and timestamp
  policy, terminal detection, synchronous I/O/backpressure behavior, write-error
  visibility and scoped removal before selecting dependencies.
- [x] Implement only the agreed adapter using `Logger`, `LogRecord`,
  `ExporterOptions` and existing formatting/color helpers. Do not duplicate the
  default buffer, error tree, service tracing or lifecycle core.
- [x] Verify deterministic captured output, Unicode/line limits, plain/color
  modes, independent exporters, failed writes, and no output after exact removal
  or owner cleanup. Document Node/browser-specific behavior not reproduced.
- [x] Add a console example and paired language guide/API pages; preserve core
  logging contracts and extend executable documentation/export checks.
- [x] Complete independent review, explicit stable/MSRV/shared acceptance and
  commit the console slice.

### 3. re-cordis-loader — explicit Rust loading, not native hot reload

- [ ] Review TS `packages/loader/src/` and `packages/loader/tests/`, especially
  resolve/import, entry/tree/group, validation/commit, evaluation and isolation.
  Separate core loading contracts from include/HMR/file-persistence features.
- [ ] Choose a bounded first slice: explicitly registered Rust plugin factories
  and typed configuration/resolution contracts. Agree on input format and
  serialization dependencies before adding them to a new loader crate.
- [ ] Define missing/duplicate/unknown entries, validation timing, admission
  versus readiness, load ordering, exact dependency retention, partial-load
  rollback, explicit updates and parent-owned teardown. Missing requirements
  remain Pending; Failed recovery remains an explicit update.
- [ ] Map JS module imports, runtime expressions, schema discovery and mutable
  entry/group behavior to supported Rust equivalents or clear exclusions.
  Do not imply arbitrary JavaScript execution, native shared-library reload,
  filesystem watching or transparent JS compatibility.
- [ ] Test fixtures and controlled failure paths through the public core APIs,
  including loading/unloading gates and repeated baseline restoration.
  Any demonstrated core gap needs a separately justified, targeted fix.
- [ ] Add runnable boot/reconfigure/shutdown examples and paired language
  guide/API pages; complete the shared checklist.

### Other satellites — planned after the first three targets

- [ ] `group`: implement structural group behavior with loader; decide whether
  a thin facade crate is useful. Upstream group re-exports loader Group. See
  [G0–G1](plans/2026-11-02-cordis-completion/group.md).
- [ ] `include`: evaluate configuration includes, patches, provenance and
  failure-safe persistence after loader format/ownership contracts exist.
- [ ] `hmr`: define what Rust reconfiguration/reload actually means after loader
  acceptance. File watching and native hot loading require separate decisions.
- [ ] `create`: consider scaffolding only once public crate names, dependency
  setup and application layout are sufficiently settled.

Do not treat this list as approval to implement all satellites together.
Publication, API stabilization, public ingress and new external-service
integrations remain separate scope decisions.

### Shared implementation and documentation acceptance

- [ ] Before each slice, identify affected files, TS source cases, public
  contracts, dependencies and measurable thresholds. Reuse existing helpers.
- [ ] Add only the selected crate to the workspace; keep Rust 1.85 support,
  unpublished status and existing core behavior unless deliberately revised.
- [ ] Run targeted tests first, then applicable workspace/M0 tests, all-target
  checks, strict Clippy, formatting and warnings-denied Rustdoc with explicit
  MSRV/stable tool paths and isolated artifacts. Preserve the six paired
  lifecycle scenarios and 73 observations; do not equate satellite tests with
  expanded differential parity.
- [ ] Update matching `docs/en/` and `docs/vi/` guides/API indexes, examples,
  glossary terms as needed, and `docs/llms.txt` in the same slice. Extend
  `tests/docs/check.py` for each delivered crate's exports/snippet dependencies;
  its current automated export coverage includes core, timer and logger.
- [ ] Keep `README.md` status/layout/star matrices, `CHANGELOG.md`,
  `docs/todo.md`, `docs/decision.md`, `docs/limitation.md` and `AGENTS.md` accurate.
  Preserve old milestone evidence as historical, not claims of new delivery.
- [ ] Run documentation regressions and both toolchain documentation checks.
  Commit only the approved, verified slice under repository conventions;
  never push automatically.

## Post-M6 core hardening

- [x] Approved focused lifecycle-parity slice: execute three equivalent
  Rust/TypeScript scenarios for concurrent provider/config updates, replacement
  coalescing during gated cleanup, and Registry-removed consumer disposal.
  Shared traces and readiness assertions are documented in
  [lifecycle-parity.md](lifecycle-parity.md). No production fix was required.
  This does not select a satellite or define a broader M7.

- [x] Approved bounded failure/recovery evidence using the existing lifecycle-parity
  fixtures: superseded failure rolls back without poisoning the latest target;
  latest failure reaches all waiters while abandoned waits do not hide
  diagnostics or cancel rollback; provider refresh does not retry Failed consumers,
  but explicit update recovers through gated asynchronous setup.
  Three new paired scenarios add 36 observations (six scenarios / 73 total).
  No production fix, new dependency, or public API change was required.

These are bounded executed compositions, not complete upstream failure/recovery
protocol parity. Both slices are complete; they did not select satellites.
The later satellite roadmap above is a separate workstream, not another parity
slice or a broader M7. Retain acceptance checks and require evidence before
changing production behavior.

## Approved first consumers: GitHub activity

The user selected repository activity reporting, collection, and webhook
journaling. Keep them thin application examples, with no production core change
unless an approved requirement demonstrates a gap. See
[github-activity.md](github-activity.md) for the bounded scope and limitations.

- [x] Snapshot-to-Markdown digest: validated bounded input, deterministic output,
  failure-safe replacement, destination updates and explicit processor recovery.
- [x] `gh` collector: bounded complete snapshots, timeouts, one-shot/periodic
  operation, reconfiguration and joined shutdown.
- [x] Loopback webhook journal: raw-body signature verification, repository/event
  validation, limits, append-before-ack and graceful in-flight shutdown.
- [x] Final consumer acceptance: explicitly selected Rust 1.85.0/stable matrices,
  the six paired scenarios, and offline executable checks.
- [x] Authorized one-shot live collector/digest pilots on 2026-10-27:
  `antruongnguyen/cordis-rust` (the repository's former name, since renamed to
  `antruongnguyen/re-cordis`) returned no issues/PRs; `cordiverse/cordis`
  returned 52 issues and 100 PRs (PR cap reached). Point-in-time application
  observations only; no new parity or live webhook claim, no committed live data.

Public ingress, real GitHub webhook configuration/delivery, mutations,
notifications to other people and durable/exactly-once delivery are excluded.
These consumers are not a generic harness or a new satellite/M7 commitment.

## User documentation

- [x] Add matching English/Vietnamese user guides and grouped public API reference.
- [x] Add language navigation, a shared terminology glossary, and `docs/llms.txt`.
- [x] Add repeatable offline link/coverage checks and execute complete guide
  examples with explicitly selected Rust 1.85.0 and stable.
- [x] Rereview both language sets against public source; correct config-stage,
  initialization/injection and snapshot/filter wording; add checker regressions.
- [x] Add source-pinned README comparison matrices with row-specific star ratings
  and explicit limits; no unmeasured performance or universal-winner claim.

See [the documentation map](README.md). These pages document the existing
unpublished core, timer and logger; the remaining satellite roadmap is future work,
not an API-stability promise.
Maintain both languages and the agent index when public contracts change.

## Cross-cutting

- [x] Initial TS-to-Rust mapping and source-backed testing strategy documented.
- [x] Maintain case-level parity evidence and explicit limitations through M6.
- [x] Prove error reporting, dropped-waiter behavior, and resource retention
  without strong-reference leaks; do not equate Rust Drop with async cleanup.
- [x] Run targeted tests per slice, then full workspace/doc tests, lint, and
  declared-MSRV/current-stable checks before core acceptance.

Future work must preserve these checks. Arbitrary user-created Rc cycles,
executor destruction, and the explicit compatibility restrictions in M6 are
not promises of automatic cleanup or full upstream syntax parity.
