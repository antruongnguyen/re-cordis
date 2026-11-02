# Loader implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Deliver explicit Rust plugin registration/resolution and a source-aware
entry tree with predictable update, failure, isolation and teardown behavior.
**Architecture:** A loader owns entries and factory adapters over core plugins.
It delegates lifecycle and service retention to core; include owns file persistence.
**Tech stack:** Rust 2024, re-cordis-core, standard collections; serde/serde_json
only after L0 selects a serialized boundary.
**Implementation:** Delivered L0–L5/G0–G1 in the current working tree; final
independent review, explicit stable/1.85.0 matrix and commit are tracked by
the parent acceptance record. Checkboxes below record implementation, not
unrestricted execution of all 140 upstream declarations.

**Spec:** [Master design](README.md), pinned packages/loader/src/ and its eleven
spec files listed in [the inventory](source-inventory.md).

## Global constraints

Rust edition 2024; resolver 3; minimum Rust 1.85, tested with 1.85.0. MIT;
version 0.0.0 and publish=false. No timer/logger dependency by default.
No native shared-library loading, JS evaluation engine or filesystem watcher
is implied by the name loader. Each departure gets a source-case disposition.

## Source facts that must survive the translation

- EntryTree identifies nested trees with composite IDs, creates/removes/moves
  entries, emits commit records and waits for currently tracked tasks.
- EntryGroup updates entries independently and catches per-entry failures.
  Valid siblings survive. Runtime mutation validation rejects before commit;
  source-driven invalid entries are removed independently.
- Entry separates an outer injection owner from the inner plugin when required.
  Entry-level injection/evaluation and plugin injection have different authority.
- Entry updates can replace the plugin name or injection shape while an import,
  patch, load or disposal is pending. Latest intent and removal must win.
- Loader config hooks preserve source versus evaluated/resolved values; child
  plugin activity must not be mistaken for the entry's own update/disposal.
- Groups remain structural even when their verdict disables descendants.
- Isolation moves retain old exact providers until consumers finish cleanup.
- Node resolution/cache internals and JavaScript expressions are mechanisms
  requiring Rust replacements, not convenient names for already supported APIs.

## L0 — Entry model, factory catalog and compatibility gate

**Files:** this plan, docs/decision.md, docs/limitation.md; inspect core
plugin.rs/service.rs/hooks.rs and all source suites in the inventory.

- [ ] Define exact types/signatures for loader, entry ID, entry specification,
  immutable entry snapshot, factory catalog, operation result and commit record.
  Distinguish entry identity, factory identity and core Fiber identity.
- [ ] Specify heterogeneous factories through a private erased adapter around
  typed Plugin<C, R>; retain structured decode/validation failures and original
  errors. Reuse one Plugin identity for repeated entries of that factory.
- [ ] Separate factory configuration decoding from core validation timing.
  Required services may be Pending; do not eagerly run a plugin's validator
  before the core's documented readiness boundary.
- [ ] Choose explicit names/IDs, duplicate catalog/entry handling, anonymous ID
  allocation, root/nested separators, unknown factories and entry limits.
- [ ] Define programmatic mutation versus source reconciliation policies:
  reject invalid runtime mutations before commit; preserve independent valid
  siblings during source reconciliation. Roll back only resources acquired by
  the failed attempt. Global all-or-nothing startup is optional additional scope.
- [ ] Define typed service-name adapters for serialized inject/isolate/intercept
  fields; unknown service contracts produce a clear error, not unchecked Any casts.
- [ ] Specify admission versus settlement versus readiness. Missing dependencies
  are Pending; Failed recovery requires an explicit update.
- [ ] Choose programmatic typed input first and a bounded JSON transport at L4.
  Define typed evaluation callbacks and which JS expression cases cannot be
  represented. Reject unsupported expression nodes explicitly.

**Exit:** reviewed public signatures, operation/state table, source-case map,
dependency decision and exact limits. Generate the short coding plan for L1.

## L1 — Registered factories and in-memory entries

**Files to create:** crates/re-cordis-loader/Cargo.toml, src/lib.rs,
src/lib.rs, src/scope.rs, src/source.rs, tests/catalog.rs, tests/entries.rs.

- [ ] Add failing tests for known/unknown/duplicate factory names, stable
  identity across two entries, duplicate explicit IDs, generated IDs and
  heterogeneous typed configs including a decode failure.
- [ ] Implement catalog resolution and entry creation/removal with parent-owned
  core plugins. Expose snapshots, not mutable core internals.
- [ ] Prove dropped handles do not orphan entries, failed admission leaves no
  entry, and removed entries release effects and listeners.
- [ ] Run `cargo test -p re-cordis-loader --test catalog --test entries --locked`,
  add a small programmatic boot/shutdown example, document and commit.

**Exit:** useful standalone in-memory loader; no files or expressions required.

## L2 — Updates, readiness, failure and rebuilds

**Files:** src/lib.rs, tests/updates.rs, tests/rebuild.rs,
tests/failures.rs. Reuse core update/completion APIs.

- [ ] Port source-backed cases for Pending activation, explicit Failed recovery,
  invalid loaded update retaining the old config, and source versus resolved values.
- [ ] Gate cleanup while changing factory name or injection twice; assert only
  the latest entry starts and removal/disable during cleanup prevents resurrection.
- [ ] Preserve entry-level versus plugin-level injection authority. Typed
  entry evaluation waits for the former before creating the inner plugin.
- [ ] Distinguish entry self-disposal from a nested child's disposal and parent
  dependency loss. Preserve entry options without spuriously persisting disabled.
- [ ] Record create/update/remove commit payloads with old/new snapshots, exact
  entry IDs and origin. Read-only reconciliation must not emit write-back commits.
- [ ] Test failure in one entry with a healthy sibling, failed partial setup,
  abandoned waiters, removal while starting and repeated baseline restoration.
- [ ] Run update/rebuild/failure tests and selected core lifecycle suites; commit.

## L3 — Structural groups, moves and isolation

**Depends on:** L2 and G0's agreed group contract.
**Files:** src/lib.rs, src/scope.rs, tests/groups.rs,
tests/moves.rs, tests/isolation.rs. See [group plan](group.md).

- [ ] Implement nested groups, composite IDs, ordered children and create/
  remove/move operations without duplicating core's cleanup machinery.
- [ ] Test moving an entry within/between groups and source trees: update the
  owner relation once, preserve identity, emit correct old/target group records
  and prevent the old group from disposing a newly adopted entry.
- [ ] Match structural group disabled behavior and child settlement after
  configuration changes; do not flatten group state to an ordinary plugin flag.
- [ ] Map fresh/shared isolation labels and intercept changes onto existing
  core typed APIs. Prove old committed views survive gated provider/consumer
  cleanup through repeated realm moves, including self-providing plugins.
- [ ] Add a narrowly scoped B2 issue only if these public-API fixtures expose
  an actual missing core contract. Never reach into private core stores.
- [ ] Run groups/moves/isolation tests plus G1 acceptance; document and commit.

## L4 — Bounded configuration and typed evaluation

**Files:** src/lib.rs, src/source.rs, tests/config.rs,
tests/evaluation.rs, tests/fixtures/; update Cargo.toml/lock for accepted serde
dependencies. File reading/writing belongs to include.

- [ ] Implement the reviewed JSON-to-entry boundary with explicit maximum
  input bytes, depth, entries, IDs and values. Limits are named/tested at L0;
  no arbitrary magic limits are silently inherited from the GitHub examples.
- [ ] Preserve absent/null/deleted fields and option merge semantics. Validate
  runtime group input as a whole; keep source entry failures independent.
- [ ] Evaluate typed conditions/intercepts/config at the correct parent, entry
  injection or plugin injection scope; cache verdicts until an invalidating
  update/dependency transition and prohibit out-of-scope service access.
- [ ] Test invalid expression markers, decoder failures, unknown fields under
  the selected policy, no evaluation while disabled, changing parent/isolation,
  failing intercept preventing activation and explicit recovery.
- [ ] Provide a source-location hook carrying structured paths and optional
  spans; runtime-originated data must not invent a filename or source span.
- [ ] Run config/evaluation tests, document the JS eval and module resolution
  exclusions, and commit.

## L5 — Loader acceptance and future integration seam

**Files:** examples/boot_reconfigure.rs, public Rustdoc, paired guides/loader.md
and api/loader.md, indexes, tests/docs/, status/evidence documents.

- [ ] Demonstrate boot, provider arrival, reconfigure, failure, explicit
  recovery and joined shutdown through the public loader API.
- [ ] Expose only the reviewed source reconciliation/commit/snapshot seam
  required by include and factory-replacement seam required by HMR. Test both
  with in-memory fixtures before publishing their signatures.
- [ ] Dispose a tree with gated children and assert all registrations,
  publications and retained providers return to baseline.
- [ ] Complete the declaration-by-declaration map for all eleven loader spec
  files, including Node-specific resolve/import/internal dispositions.
- [ ] Run shared acceptance and selected paired loader scenarios with causal
  observations; keep the six existing core traces unchanged; commit.

**Completion boundary:** L1 is minimal loading; L1–L5 plus G1 are the approved
loader/group scope. Arbitrary JS imports, eval, mutable prototypes, V8 stack
rewriting and native hot loading require explicit unsupported entries or a
separate approved project; they are never marked complete by Rust API resemblance.

## Delivered Rust contract and targeted checks

The public API is `Catalog`, `Config`, `EntrySpec/EntryKind`,
`EntryId/FactoryId`, `Group`, `Scope`, `Loader`, immutable
`EntrySnapshot`, `Settlement/EntryFailure`, `LoaderTask`,
`Origin/CommitRecord`, `SourceId/SourceDocument/SourceLocation/SourceSpan`
and `EntryPatch/FieldPatch`. Exact signatures live in Rustdoc and
[the paired API](../../en/api/loader.md).

- Catalog retains original typed Plugin identity; decoding is separate from
  dependency-ready validation. Native Clone sources need no Serialize/Send.
- Stable global EntryId differs from catalog FactoryId and rebuilt core FiberId.
  Explicit IDs are nonempty, at most 128 bytes and exclude ':'; factory names
  are nonempty and at most 256 bytes; generated IDs use checked entry-N counters.
- Programmatic whole-group input rejects before mutation. Source reconciliation
  isolates entry failures and preserves healthy siblings, including nested
  children. Runtime events carry old/new/group/origin; source reads do not write back.
- Outer entry injection and inner plugin injection have separate authority.
  Conditions/intercepts evaluate after outer commitment; mapping evaluates
  in the exact inner Context before its plugin validator. Resolved R is
  plugin-owned; snapshots expose source, original transport and evaluated C.
- One coalesced rebuild intent joins old generations; every manual retirement
  is retained independently of task cancellation. Removed-but-retiring records
  count toward 1024 admission capacity. Owner disposal breaks Entry/Fiber
  cycles and joins detached cleanup after Loader/waiter Drop.
- Structural groups retain ordered logical children when disabled.
  Cross-group/scope moves rebuild core allocation identity after cleanup;
  same-group reorder is inert. Group addresses adapt navigation, while source
  IDs are globally unique instead of upstream nested-source-local namespaces.
- JSON is 1 MiB, 32 levels, 65536 nodes/string bytes; group depth is 32.
  Decoders see absent/null distinctly. FieldPatch explicitly keeps/sets/clears.
  Registered typed service adapters support JSON injection, fresh/shared
  isolation and intercept configs. Unsupported expression markers fail.
- SourceDocument/reconcile/commits and replace_factory are tested in-memory
  integration seams; include persistence and HMR remain deferred.

Targeted command: `cargo test -p re-cordis-loader --locked`.
Fixtures are catalog.rs, entries.rs, updates.rs, groups.rs, source.rs,
regressions.rs, admission.rs, lifecycle.rs, reentrant.rs and sources_moves.rs.
They include gated rename/removal, source adoption/order, deferred validator
timing, loaded rejection, explicit failure recovery, exact child/self-disposal,
100 group cycles, source sibling failures, UTF-8/input bounds and parent-joined
retirement after all Loader handles drop. The runnable
[boot_reconfigure example](../../../crates/re-cordis-loader/examples/boot_reconfigure.rs)
covers Pending provider arrival, update, apply failure, recovery and shutdown.
Strict loader all-target Clippy and targeted tests have passed during development;
final stable/MSRV, docs and paired scope results belong to final acceptance.

## Source-case dispositions

All eleven pinned suites are reviewed under their declaration groups below.
The inventory retains every original line/name. These are semantic dispositions,
not an assertion that a Rust fixture executed each original JavaScript test.

| Pinned suite / declarations | Rust representation and executable fixture | Mechanism/evidence boundary |
|---|---|---|
| commit.spec.ts / 10 | CommitRecord old/new/target/origin, read-only reads, adoption, exact inner hooks: source.rs, lifecycle.rs, sources_moves.rs | Core events replace JS tree override callbacks; original transport is retained separately. |
| evaluate.spec.ts / 37 | Typed conditions/intercepts/config mapping and separate entry/plugin injection: groups.rs, regressions.rs, reentrant.rs | JS expression parsing, interpolation trace offsets and JS truthiness are explicitly unsupported; typed callbacks express useful intent. |
| group.spec.ts / 13 | Structural disabled inheritance, ordered updates, adoption/removal and repeated cleanup: groups.rs, regressions.rs, sources_moves.rs | Loader-owned Group export; no separate facade/registry. Native source IDs are loader-wide. |
| import.spec.ts / 2 | Explicit Catalog names and typed declarations: catalog.rs, entries.rs | JS import/export wrapping and arbitrary module execution are unsupported. |
| index.spec.ts / 19 | Create/update/remove, Pending, failure/recovery and joined teardown: entries.rs, updates.rs, admission.rs, reentrant.rs | Explicit Loader handle replaces implicit JS loader service/property integration; invalid schema values keep core validation timing. |
| internal.spec.ts / 1 | Typed replacement seam exercised by source.rs | Node ModuleJob/cache monkey-patching is unsupported, not implemented by the replacement API. |
| isolate-provide.spec.ts / 7 | Typed Scope and committed-provider retention: groups.rs, lifecycle.rs | Public core isolation/publication ownership reused; JS prototype reflection is not reproduced. |
| isolate.spec.ts / 21 | Fresh/shared typed realms, scope moves and old exact consumer views: groups.rs, lifecycle.rs, regressions.rs | Move preserves EntryId while rebuilding Fiber IDs; nested source namespaces are deliberately adapted. |
| rebuild.spec.ts / 12 | One latest revision during gated cleanup, removal wins, factory replacement: updates.rs, source.rs | Synchronous registered catalog replaces JS asynchronous import windows; lifecycle cleanup/load windows remain tested. |
| resolve.spec.ts / 12 | Known/unknown/duplicate registered names: catalog.rs, entries.rs | Node project helper/package exports/self references/drive paths/rename retries are unsupported; use explicit native registration. |
| validate.spec.ts / 6 | Typed payload checks, bounded JSON, whole runtime groups, independent source failures: source.rs, admission.rs, regressions.rs | No JS StandardSchema discovery or expression schema; original Rust decoder/core error objects retained. |

Node-specific import/resolve/internal declarations are exclusions of unsuitable
JS machinery, not missing native loading features. File durability/watchers are
deferred satellite scope, not a loader promise. No unrestricted 140-test parity
percentage is reported. See [decision](../../decision.md#loader-and-structural-groups)
and [limitations](../../limitation.md#loader-and-structural-groups).
