# Group implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Cover the group satellite's structural configuration behavior with
one implementation owned by loader.
**Architecture:** Upstream packages/group/src/index.ts imports Group from
plugin-loader and exports it as default. The behavior resides in loader's
config/group.ts and config/entry.ts. Rust follows that ownership boundary.
**Tech stack:** re-cordis-loader and re-cordis-core.
**Spec:** [Master design](README.md), [loader plan](loader.md), pinned
packages/loader/tests/group.spec.ts, evaluate.spec.ts, commit.spec.ts and
isolate.spec.ts. There is no standalone upstream group test suite.

## Global constraints

Rust edition 2024; resolver 3; minimum Rust 1.85, tested with 1.85.0. MIT;
version 0.0.0 and publish=false. Existing parent-owned plugins supply teardown,
but are not by themselves the complete configurable group contract.

## G0 — Decide group semantics and packaging

**Entry:** L0 entry/injection model is reviewed.
**Files:** this plan, docs/decision.md, proposed loader group API.

- [x] Map the thirteen declarations in group.spec.ts and the structural-group
  evaluation cases to L3 fixtures. Include enabled/disabled parent/child
  combinations, initialization, updates and nested settlement.
- [x] Specify child ordering, ownership after moves, retained group identity,
  structural presence while descendants are disabled, and source commit origin.
- [x] Recommend a loader Group export and documentation under the group name.
  If a separate re-cordis-group is selected, restrict it to a documented facade
  over that export, without a second registry, service or lifecycle.

**Exit:** decision enables L3; no empty crate is needed to close the inventory.

## G1 — Group behavior and public entry point acceptance

**Entry:** L3 implementation is available.
**Delivered files:** crates/re-cordis-loader/src/lib.rs, tests/groups.rs, tests/sources_moves.rs;
paired docs/en/api/groups.md and docs/vi/api/groups.md and guides/groups.md.
A facade Cargo.toml/src/lib.rs is conditional on G0's packaging decision.

- [x] Verify group updates settle included child work under the reviewed
  Pending/readiness semantics; do not imply all children are Active.
- [x] Test child removal, disabled inheritance, nested groups, parent unload,
  in-memory source adoption, reordering and a moved child retained by its new group.
- [x] Assert repeated group reconfiguration/disposal restores Registry,
  effect, listener and publication baselines.
- [x] Demonstrate grouping through the supported public Rust import path.
  If a facade exists, compile its downstream import without duplicated types.
- [x] Run loader group/move tests and shared acceptance for the delivered
  package(s); document the upstream package correspondence and commit.

**Completion:** source behavior is covered regardless of whether there is a
separate Cargo package. This work cannot be replaced by merely showing that
core parent-owned child cleanup already works.

## Delivered G0/G1 contract and evidence boundary

Import `re_cordis_loader::Group`. Behavior lives in the loader's lib.rs; there
is no empty group crate, second registry or independent lifecycle engine.
Group owners remain structural while disabled verdicts suppress descendants.
Ordered children follow current logical ownership; old groups do not remove
adopted entries. Stable EntryId survives cross-group/source adoption; crossing
groups/scopes joins old cleanup and rebuilds core Fiber allocations.
Same-group reordering does not restart.

The thirteen group.spec.ts declarations are represented by:
initialization and normal groups; parent/child disabled combinations and nested
structural presence; child source replacement/removal; adoption/reorder;
parent/loader cleanup and readiness that may settle Pending. Executable fixtures
live in groups.rs, regressions.rs (100 cycles and source child failures) and
sources_moves.rs. Isolation/old committed views are in lifecycle.rs. This is
native Rust behavior, not JS mutable prototype or source-namespace emulation.

See [group API](../../en/api/groups.md) and
[loader dispositions](loader.md#source-case-dispositions).
Independent review and explicit stable/1.85.0 verification are complete at
code checkpoint `101f37b`; see [acceptance](evidence/loader-acceptance.md).
The source inventory's lack of standalone group tests remains true:
upstream group simply exports loader Group.
