# Loader implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Deliver explicit Rust plugin registration/resolution and a source-aware
entry tree with predictable update, failure, isolation and teardown behavior.
**Architecture:** A loader owns entries and factory adapters over core plugins.
It delegates lifecycle and service retention to core; include owns file persistence.
**Tech stack:** Rust 2024, re-cordis-core, standard collections; serde/serde_json
only after L0 selects a serialized boundary.
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
src/catalog.rs, src/entry.rs, tests/catalog.rs, tests/entries.rs.

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

**Files:** src/entry.rs; create src/tree.rs, tests/updates.rs, tests/rebuild.rs,
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
**Files:** src/tree.rs; create src/group.rs, src/scope.rs, tests/groups.rs,
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

**Files:** create src/config.rs, src/source.rs, tests/config.rs,
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
