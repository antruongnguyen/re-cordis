# HMR and reload implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Provide scoped change detection and safe reload coordination, with an
explicit boundary between configuration reload and replacement of executable code.
**Architecture:** A local coordinator consumes filesystem events and explicit
factory revisions, delegates ownership to loader/core and uses accepted timer
debounce. Include registers file-refresh callbacks through an optional seam.
**Tech stack:** Rust 2024, core/loader/timer; a reviewed watcher crate such as
notify only after its MSRV/platform behavior is verified.
**Spec:** [Master design](README.md), pinned packages/hmr/src/{index,error}.ts,
tests/index.spec.ts and tests/path.spec.ts.

## Global constraints

Rust edition 2024; resolver 3; minimum Rust 1.85, tested with 1.85.0. MIT;
version 0.0.0 and publish=false. No Node module cache exists in this Rust port.
Calling Fiber restart with the same compiled plugin is not source-code reload.

## Required distinctions

Upstream resolves module dependency graphs, backs up ESM/CJS caches, imports
and validates all replacements before unloading, then rebuilds affected fibers.
It preserves per-instance configs, skips stale nested entries, waits for each
old instance's teardown while unrelated plugins progress, and has a fallback
that still watches config when Node internals are unavailable.

The Rust scope is proposed in three layers:

1. **File/config reload:** watch registered paths and invoke include refresh.
2. **Linked factory replacement:** the application explicitly supplies a new
   factory revision already compiled into the process; rebuild affected entries.
3. **Changed Rust source:** emit restart-required unless an external build/process
   supervisor supplies a supported replacement. Native binary hot swapping is
   a research decision, not a promised implementation in this plan.

All three have explicit dispositions; layers 1–2 do not establish Node HMR parity.

## H0 — Reload model and watcher gate

Completed contract: [Rust API and plan](evidence/hmr-contract.md),
[watcher dependency gate](evidence/hmr-watch-contract.md),
[batch seam probes](evidence/hmr-reload-contract.md) and
[independent review](evidence/hmr-h0-review.md).

**Entry:** T5 and L5 accepted; I0's callback/source identity contract is known.
**Files:** this plan, docs/decision.md, docs/limitation.md.

- [x] Specify public change/reload/restart-required events, owned watch token,
  path identity, debounce window and queue/backpressure bounds.
- [x] Choose watcher semantics for rename-based writes, remove/recreate,
  ignored directories, explicit watches overriding ignore, symlinks and
  Windows case/path behavior. Prove selected crate compatibility on 1.85.0.
- [x] Specify an explicit affected-entry/factory dependency model for layer 2.
  Do not infer a Rust module graph from filenames or Cargo dependencies.
- [x] Define replacement validation before unload, per-entry config retention,
  failure after unload, rollback limits and disposal superseding queued reloads.
  External effects cannot generally be rolled back transactionally.
- [x] Choose whether layer 3 ends with a restart-required event or receives a
  separately scoped process supervisor. A supervisor needs build failure,
  process ownership, readiness, state transfer and shutdown contracts.
- [x] Record a separate feasibility proposal for native dynamic loading only
  if requested: ABI compatibility, code/resource lifetimes, unwind safety,
  memory ownership, state migration and OS unloading. No implicit promise.

**Exit:** reviewed capability table and exact APIs for supported layers.

## H1 — Owner-scoped file watching and config callbacks

**Files to create:** crates/re-cordis-hmr/Cargo.toml, src/lib.rs, src/watch.rs,
tests/watch.rs, tests/paths.rs. Optional include adapter after I1.

- [ ] Begin with a deterministic synthetic event source: duplicate changes,
  burst coalescing, multiple callbacks on one path, ignored paths, explicit
  override, callback failure, dropped token and owner cleanup.
- [ ] Bridge watcher-thread events into the local host using bounded messages;
  never move non-Send Context/Fiber handles onto the watcher thread.
- [ ] Implement exact watch removal and joined producer shutdown using the
  reviewed authority/cancellation path. Cancellation of timer debounce must
  prevent a stale queued refresh after disposal.
- [ ] Add bounded real filesystem smoke tests for atomic rename/remove/recreate
  and platform path behavior. A smoke test supplements deterministic tests.
- [ ] Connect include.refresh without making include depend on HMR; parse
  failures are visible and leave later valid refreshes usable. Commit.

## H2 — Explicit factory replacement coordination

**Files:** create src/reload.rs, tests/reload.rs, tests/nested.rs.

- [x] Test two instances of one factory with different configs, two unrelated
  factories, a changed dependency, unchanged shared dependencies and nested hosts.
- [x] Validate all proposed replacement factories before scheduling teardown;
  invalid candidates keep old instances running.
- [x] For each affected owner, retain its exact providers through cleanup and
  start its replacement only after old resource release. Let unrelated entries
  progress while a slow sibling is gated.
- [x] During unload, change config, disable/remove entries, replace parent
  hosts and re-enable entries. Recheck current identity/intent before rebuilding
  so no child is instantiated twice or resurrected after removal.
- [x] Preserve failure diagnostics and define recovery after replacement apply
  fails. Do not claim pre-import rollback can reverse already-completed teardown.
- [x] Run repeated replacement/teardown baseline tests and commit.

## H3 — Integration, fallback and acceptance

**Files:** examples/reload.rs, tests/include_integration.rs, paired guides/hmr.md
and api/hmr.md, indexes and source-case evidence.

- [ ] Demonstrate config reload with optional include integration after I4,
  linked factory replacement, and a source change that reports restart-required.
- [ ] Verify the fallback still refreshes configuration when code replacement
  is unavailable; surface the capability once without warning on every event.
- [ ] Prove burst updates, rename-based self-writes, parse recovery, resource
  release and shutdown across timer/loader/include boundaries.
- [ ] Account for every HMR declaration, explicitly separating Node module
  import/cache/dependency behavior from tested Rust equivalents.
- [ ] Run shared acceptance and supported-platform watcher tests, document
  supported layers under the HMR name, and commit.
