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

- [ ] Map the thirteen declarations in group.spec.ts and the structural-group
  evaluation cases to L3 fixtures. Include enabled/disabled parent/child
  combinations, initialization, updates and nested settlement.
- [ ] Specify child ordering, ownership after moves, retained group identity,
  structural presence while descendants are disabled, and source commit origin.
- [ ] Recommend a loader Group export and documentation under the group name.
  If a separate re-cordis-group is selected, restrict it to a documented facade
  over that export, without a second registry, service or lifecycle.

**Exit:** decision enables L3; no empty crate is needed to close the inventory.

## G1 — Group behavior and public entry point acceptance

**Entry:** L3 implementation is available.
**Files:** crates/re-cordis-loader/src/group.rs, tests/groups.rs, tests/moves.rs;
paired docs/en/api/groups.md and docs/vi/api/groups.md and guides/groups.md.
A facade Cargo.toml/src/lib.rs is conditional on G0's packaging decision.

- [ ] Verify group updates settle included child work under the reviewed
  Pending/readiness semantics; do not imply all children are Active.
- [ ] Test child removal, disabled inheritance, nested groups, parent unload,
  file-driven adoption, reordering and a moved child retained by its new group.
- [ ] Assert repeated group reconfiguration/disposal restores Registry,
  effect, listener and publication baselines.
- [ ] Demonstrate grouping through the supported public Rust import path.
  If a facade exists, compile its downstream import without duplicated types.
- [ ] Run loader group/move tests and shared acceptance for the delivered
  package(s); document the upstream package correspondence and commit.

**Completion:** source behavior is covered regardless of whether there is a
separate Cargo package. This work cannot be replaced by merely showing that
core parent-owned child cleanup already works.
