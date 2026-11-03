# HMR H2 linked replacement contract

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Status: H0 contract independently approved; H2 implementation pending.
Baseline: `47042d9e805fa558e69416c8ffb1030333b0c38b` on `feat/cordis-completion`.
TypeScript pin: `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.

## Reused evidence and uncovered delta

Reuse [loader acceptance](loader-acceptance.md), its [scoped re-review](loader-rereview.md),
[loader decisions](../../../decision.md#loader-and-structural-groups),
[loader limitations](../../../limitation.md#loader-and-structural-groups) and accepted
core provider retention, local hosting, started work and dependency-ready validation.
Loader already converges each entry to the latest source after joined old cleanup;
its independent hosted workers let siblings progress. No core change is selected.

The uncovered delta is whole-batch factory admission and callback reentry during
replacement. Current `TypedFactory::check` checks only `Config::get::<C>()`.
`replace_factory` changes one catalog record, then calls `update_origin` for an
old `Rc<Entry>` list. Sequentially composing it allows A teardown before B fails;
a commit callback may remove B before its old list entry is processed.

Pinned TS `packages/hmr/src/index.ts:405` imports all declarations and checks
export applicability before unload. This is not dependency-ready config
validation. It then unloads synchronously and rebuilds fibers concurrently,
filtering inactive ancestors and rechecking current entry/host intent after
cleanup (`:425`, `:455`, `:463`). Its nested, handoff, malformed-export,
multiple-entry and unload-race variants are in `tests/index.spec.ts:974–1610`.
Path tests select no extra executable-code replacement semantics.

## Exact proposed API

These signatures remain planning material until implemented:

```rust
// re-cordis-loader
pub struct FactoryReplacement { /* private erased declaration and preflight */ }
impl FactoryReplacement {
    pub fn new<C: Clone + 'static, R: 'static>(
        name: impl Into<String>, plugin: Plugin<C, R>,
    ) -> Self;
    pub fn name(&self) -> &str;
    pub fn with_preflight<F, E>(self, validate: F) -> Self
    where F: Fn(&Config) -> Result<(), E> + 'static,
          E: Into<BoxError> + 'static;
}
impl Loader {
    pub fn replace_factories(
        &self, replacements: Vec<FactoryReplacement>,
    ) -> Result<LoaderTask<Settlement>, LoaderError>;
    // Existing replace_factory delegates to the one-element batch.
}

// re-cordis-hmr; FactoryReplacement is re-exported, no second wrapper.
#[derive(Clone, Debug)]
pub struct ReloadNode {
    pub name: String,
    pub factory: Option<String>,
    pub dependencies: Vec<String>,
}
pub struct ReloadPlan { /* private checked graph */ }
impl ReloadPlan {
    pub fn new(nodes: Vec<ReloadNode>) -> Result<Self, LoaderError>;
    pub fn affected(&self, changed: &[String]) -> Result<Vec<String>, LoaderError>;
    pub fn replace(
        &self, loader: &Loader, changed: &[String],
        replacements: Vec<FactoryReplacement>,
    ) -> Result<LoaderTask<Settlement>, LoaderError>;
}
```

The application names code units and maps each factory-bearing node explicitly
to a catalog factory. Dependency-only nodes have `factory: None`; every edge
names a declared node. A reverse dependency closure selects affected factories;
unchanged shared dependencies never expand the set sideways. All affected
factories require exactly one compiled candidate, including affected dependents.
Candidates outside that set, duplicates, unknown nodes, missing edge targets,
duplicate factory mappings and unknown catalog factories reject before teardown.
Cycles are supported by a bounded fixed-point traversal. Stable output follows
input node order. Empty changes yield no affected factories; extra candidates
still reject. Names are nonempty and at most 256 UTF-8 bytes; graph bounds are
256 nodes, 1024 edges, 256 candidates. Loader retains its 1024 entry/work bound.

`Settlement` is the completed reload observation: current entries and per-entry
failures. The plan owns no resource or background worker. Existing loader owner
admission and disposal govern replacement; dropped waiters do not cancel
admitted cleanup. H1 watcher ownership stays separate.

## Validation and mutation ordering

1. Validate candidate names, uniqueness, graph coverage, catalog existence,
   unchanged source type C, all current matching source config types, and
   catalog identity capacity before any catalog change or retirement.
2. Optional candidate preflight runs synchronously for each matching retained
   raw `Config`, including disabled entries, under loader owner callback hosting.
   It gets no synthetic bound Context and must be pure. No framework borrow
   spans a user callback. A same-loader `State.preflighting: Cell<bool>` RAII
   admission guard covers optional callbacks: `admit()` refuses create/update/
   patch/remove/reconcile/replace and other admitted mutation paths; `dispose()`
   checks the guard explicitly because it currently bypasses `admit()`. Refusal
   occurs before any same-loader API can initiate teardown. Reset the guard
   before dropping user captures, including error and panic exits. Also capture
   private entry identity/revision and catalog epoch, and recheck owner liveness
   after callbacks, before catalog publication. This catches invalidation through
   other supported callbacks. Direct core lifecycle calls or external effects
   violate preflight purity and cannot be rolled back by the loader guard.
3. Publish all checked replacement catalog records before releasing displaced
   declarations or scheduling entry teardown. Release user captures outside
   mutable borrows. Preserve each existing decoder; factory identity advances.
4. Recheck live current entry allocation, factory and intent before refreshing.
   Skip removed/recreated allocations and entries already advanced by a
   callback; never write an old spec over a newer config/disable/parent change.
   Existing per-entry workers join exact old provider/resource cleanup, then
   read latest source and parent view. Siblings converge independently.

Compiled `Plugin<C,R>` guarantees callable declaration shape. Optional preflight
is an application-level raw-source check, not core validation exposure. Core
mapping/interception and Plugin validation still occur on the replacement's
actual dependency-ready Context after cleanup. Missing services remain Pending;
validation/apply failures after unload remain Failed with original diagnostics
and accepted new source. A later candidate or explicit update recovers them.
No rollback claim reverses completed teardown or external side effects.

## Explicit Rust boundary

A candidate is linked into the running process already. Neither filenames nor
Cargo metadata imply a Rust module graph. Reloading unchanged compiled code is
not recompiling changed Rust source; source-only changes need restart-required
handling in H1/H3. There is no Node import/cache protocol or native ABI swap.

Catalog entries are reload units. Nested core plugins created by an entry's
apply are owned by that entry and rebuild through its supplied host candidate.
Applications must declare that host's code dependency edge. This slice does
not discover and replace an arbitrary nested registry plugin declaration
inside an unchanged compiled host. Structural loader children use existing
logical identity/liveness and joined cleanup guards; a parent rebuild must not
apply a stale child twice or resurrect a removed child.

## Failing public-API evidence

An isolated package under ignored `target/hmr-reload-probe/` exercises the real
current core/loader. No tracked implementation was changed. Explicit rustup
stable and 1.85.0 Cargo/rustc/rustdoc plus RUSTUP_TOOLCHAIN/RUSTC/RUSTDOC/PATH
produced the same two behavioral failures:

```text
second_refused=true first_catalog_changed=true old_instances_released=1
replacement_error=Some("unknown entry: b") remaining_entries=1
0 passed; 2 failed
```

The first test asserts that rejecting a later candidate must leave all old
instances live, and fails because sequential singleton replacement already
released one. The second asserts that a sibling removed by the first commit
is skipped, and fails with UnknownEntry after catalog mutation. These motivate
the batch seam and stale-entry guard, not a change to core validation timing.
Local source/logs: `target/hmr-reload-probe/{Cargo.toml,src/lib.rs,stable.log,msrv.log}`.
The initial missing closure type annotation was corrected before the reported
behavioral failures; compilation errors are not counted as regression evidence.

## Test-first H2 plan

1. Add batch admission regressions for wrong C/unknown/duplicate last candidate
   and optional preflight rejection/panic/mutation. Assert old identities,
   resources, configs and catalog remain accepted; no reload teardown occurred.
2. Add removal/recreation/factory/config reentry cases during commit/destructor
   callbacks. Implement one minimal batch seam, delegate singleton replacement,
   and preserve loader origin/transport/decoder data.
3. Add explicit graph tests: changed dependency, unchanged shared dependency,
   dependency-only nodes, cycles, malformed graphs, exact candidate coverage and
   bounds. Implement the local immutable ReloadPlan over the batch seam.
4. Add gated lifecycle tests: two configs for one factory; independent fast
   replacement while slow cleanup waits; exact old provider use until cleanup;
   config change/disable/remove/parent replacement/re-enable during unload;
   failure after unload and subsequent recovery; 100-cycle resource baselines.
5. Run focused stable/1.85.0 tests with explicit tools, strict Clippy/Rustdoc,
   examples and parent-owned documentation checks; independent scoped review
   follows. Parent owns shared Cargo/docs wiring; this worker owns loader seam,
   its regressions, HMR reload module/tests and this contract evidence.
