# Loader implementation checkpoint history

Chronological record; later checkpoints supersede earlier frozen hashes and counts.

# Loader correction slice — frozen for independent review

Worktree: /Users/I756434/.codex/worktrees/loader-completion/cordis-rust.
Base: cefb4c2 plus the inherited, uncommitted loader implementation.
Scope: loader crate, its dependency lock entry, and this report. No core, timer,
logger, guide, checker, or original-checkout source changes; no commits or subagents.
The separate paired-fixture tests/loader_parity.rs was preserved.

## Final verification

Explicit toolchain executables were used, with the toolchain bin directory first
on PATH for Cargo subcommands. Rust and Cargo report 1.97.1.

- cargo test -p re-cordis-loader: PASS — **57 integration tests, 1 unit test, 2 Rustdoc checks**.
- cargo clippy -p re-cordis-loader --all-targets -- -D warnings: PASS.
- cargo fmt -p re-cordis-loader -- --check: PASS.
- cargo run -p re-cordis-loader --example boot_reconfigure: PASS, exit 0.
  The example shows Pending activation, successful reconfiguration, synchronous
  invalid-loaded-config rejection, accepted apply failure, explicit recovery,
  and joined shutdown. Its deliberate apply failure emits the expected diagnostic.
- New adversarial/acceptance integration scenarios have a five-second timeout.
- Full-workspace/MSRV/documentation matrices remain assigned to the parent agent
  after independent final review and integration.

No outstanding known failure among the selected requirements. Final review is
still required; these checks establish the stated scenarios rather than exhaustive
upstream assertion accounting.

## Reviewed findings and corrections

1. **Dependency-driven cleanup was persisted as self-disposal.**
   The inner plugin-removal observer now requires the loader owner and the entry's
   entire outer-owner chain to remain Active before interpreting exact inner
   removal as deliberate entry disposal. Entry dependency withdrawal/return now
   returns to Active without a disabled source flag or commit. Existing exact
   manual inner disposal still produces the disabled commit.
   Red evidence: entry_dependency_withdrawal_recovers_without_disabled_commit
   failed at the disabled-source assertion before the fix.

2. **Group failure policy was corrected to the reference behavior.**
   Per the parent/reviewer ruling, runtime whole-group atomicity covers typed
   entry/options/name/identity/capacity preflight; arbitrary plugin validators
   are not a cross-plugin transaction. A loaded child validator rejection is
   retained against that child and does not abort later siblings. The child's
   prior accepted source remains, while the parent group retains its desired
   child options. Settlement reports the original rejected child's cause.
   Successful later mutation clears the retained rejection. Apply failures after
   source acceptance keep the accepted source and normal Failed lifecycle state.
   Typed source groups now filter invalid children independently, matching the
   JSON source-group sibling policy.
   Red evidence: runtime_group_options_preflight_is_atomic_but_loaded_validation_is_independent
   aborted update with the middle validator error; typed_source_group_keeps_valid_siblings_and_locates_contract_failure
   lost the healthy first sibling before the fixes.
   No core validation seam or transaction machinery was added.

3. **Source provenance was separated from mutation origin and adoption/order repaired.**
   Runtime source edits emit Runtime commits without replacing the entry's
   Source owner. Source reconciliation can explicitly adopt that ownership.
   Adopting a nested entry at document root moves it to root; root entries are
   reordered by source position. Old source cleanup checks current ownership and
   current parent, including the invalid-candidate removal branch, so stale source
   input cannot remove another source's adopted entry. Existing roots selected as
   desired descendants are protected until deferred group adoption moves them.
   Red evidence: source_adoption_moves_nested_identity_to_root_and_preserves_runtime_edit_owner
   could not resolve the root address; original-worker sources_moves root ordering
   remained [a,b] rather than [b,a]; a gated root-to-group adoption failed with
   DuplicateEntry(child); invalid_old_source_cannot_remove_identity_adopted_by_another_source
   removed the adopted entry. Each now passes.

4. **Structural updates now respect the actual outer lifecycle.**
   In-place structural evaluation requires an Active outer and no running
   rebuild. Pending, Failed, and Unloading outers use joined lifecycle rebuild.
   Explicit same-spec Failed-group update recovers the outer instead of running
   start against the still-Failed allocation. Disabled/ancestor-blocked structural
   groups retain children without evaluating conditions or intercepts.
   Red evidence: pending_group_update_defers_condition_until_entry_dependencies_commit
   observed one condition call before dependency publication; same_failed_group_update_explicitly_recovers_outer
   stayed Failed after update. Both now pass.

5. **Detached task completion errors remain bounded and visible.**
   Retirement pruning retains the first actual LifecycleError separately from
   finished task/fiber captures. join_retired awaits every retained retirement
   before returning that error; settle, disposal, and owner cleanup use it.
   Removing completed task/fiber captures keeps retained state bounded by the
   resident/retiring entry limit plus one completion error.
   The focused unit supplies an already-failed LifecycleTask-shaped shared
   retirement and a second gated retirement to exercise the otherwise difficult
   host/driver-error branch: pre-fix pruning discarded the error; the fixed owner
   cleanup joins the second task and retains the original error in diagnostics.
   This is a branch-level completion fixture, not a claim to reproduce host
   destruction or a real core driver panic.
   Separately, the real gated user cleanup error test drops the removal waiter,
   admits another entry/prunes, drops loader handles, and shuts down the parent;
   its original typed error survives in runtime.take_failures().
   Accepted core semantics are unchanged: ordinary callback errors can accompany
   successful Disposed lifecycle completion and must be inspected through core
   diagnostics. Nested callback errors are not promised in the root CleanupReport.
   No drain-before-dispose workaround or core API change was added.

6. **Callbacks/destructors no longer run under the reviewed entry borrows.**
   Group reconciliation snapshots parent origin before child mutation callbacks.
   Config mapping replaces evaluated state then drops the old user value outside
   the mutable borrow.
   Both adversarial probes were verified against temporarily restored original
   expressions: the child-commit observer reproduced "RefCell already borrowed";
   the mapped-config destructor reproduced "RefCell already mutably borrowed".
   The original expressions were immediately restored to their fixed form, and
   both probes pass with snapshot reading/parent reconciliation and no diagnostics.

7. **Source locations now cover lifecycle/admission failures as well as decoding.**
   Private source locations follow admitted specs into deferred plugin validation
   and nested lifecycle settlement. Loaded rejections use the new document's
   source/path/span while preserving original error chains/downcasts.
   JSON source filtering preserves original child indices and original transport,
   so an async failure after an earlier rejected sibling is not assigned a
   compacted child index.
   Red evidence: source_initial_and_loaded_validation_failures_keep_original_cause_and_location
   originally returned an unlocated Lifecycle callback error. Initial and loaded
   failures now retain source identity, actual supplied span, path, and a
   downcastable ValidationFailure through Error::source.

## Additional integrity corrections requested during the slice

- SourceDocument::json applies document-wide byte/depth/node/string bounds, but
  malformed entry IDs are decoded/preflighted per entry. Reserved/129-byte IDs
  produce located [index,"id"] failures while valid siblings survive.
  Runtime create_json retains whole-input ID rejection.
  The older source limit test was updated to reflect deferred entry validation.
  The new source-ID scenario was observed failing at document parsing before this
  correction.
- replace_factory rejects a changed Rust source-config contract even with no live
  entries, preserving compatibility with the retained explicit JSON decoder.
  The zero-live-entry replacement regression failed before this guard.
- Runtime group update preflight includes an omitted effective root ID. A child
  cannot reuse that ID and make the cycle guard reject after the parent commit.
  The regression originally observed a commit despite update refusal.
- Generated parent IDs skip all explicit requested descendant IDs. The regression
  originally allocated entry-1 for both generated group and requested child.
- Source duplicate root IDs fail independently: first accepted x remains,
  later colliding x reports DuplicateEntry, and healthy y remains. Repeated
  reconciliation does not duplicate order/desired identities or remove the first
  accepted x. The regression originally reported no failure and processed x twice.

## Requested acceptance evidence

New tests/acceptance.rs covers:

- Dropped direct-update waiter plus a real middleware gate: settlement stays
  Pending until release, including a three-entry nested loader grandchild tree.
- Condition verdict invalidation on dependency republication and explicit
  disable/enable, with exact evaluation/apply counts.
- Six realm generations of a self-provider: every move changes Fiber identity,
  preserves logical entry identity, joins old cleanup, and reads its old own
  publication through the original Context/isolation view.
- Exact create/update/move/remove commit source payloads, parent fields, Runtime
  origin, and source-reconciliation commit suppression.
- 100 create/remove cycles with exact event count, registry/effect baseline
  restoration on each cycle, and weak native capture release.

Original-worker tests/catalog.rs, tests/nested.rs, and tests/sources_moves.rs were
imported. Their useful coverage includes heterogeneous native configs, name
limits, all nested disabled combinations, gated latest entry-injection intent,
failing intercept/recovery, and read-only root source moves/order.
Existing tests cover retained-provider gated old-view cleanup, gated detached
parent shutdown, partial apply failure, capacity bounds, and 100 group
reconfiguration cycles; these were preserved rather than duplicated.

## Original-worker final delta reconciliation

Read .superpowers/sdd/loader/old-worker-final.diff before finalization.

- Preserved original crate executable Rustdoc and Send compile-fail Rustdoc.
- Preserved private original JSON transport on specs and entry creation, with a
  local transport clone before moving the spec instead of cloning the whole spec.
- Preserved/superseded original root adoption/reordering with ownership and gated
  descendant protection fixes.
- Preserved the original shared direct-update work and iterative settle bridge.
- No original requested implementation was overwritten wholesale.
- Removed unused direct serde dependency; serde_json remains. Cargo.lock now
  records loader dependencies without direct serde.

## Frozen SHA-256 fingerprints

| File | SHA-256 |
| --- | --- |
| crates/re-cordis-loader/src/lib.rs | 1d3d08ed65b53e3c7b9a0c929485d46a6018c0df5d7448bf6c21dec327276b33 |
| crates/re-cordis-loader/src/scope.rs | 18f876b7d0c0305689dc6b8dd49089691be1a78d64efee689c5ebf9cb74737c9 |
| crates/re-cordis-loader/src/source.rs | 09b607ce8c0ad82971bb3588f9729c44d0f968651d79ad970628a63904705283 |
| crates/re-cordis-loader/Cargo.toml | 085ba38084a52f9a47728b1ad40913234cb39c3243c31c719a03847a447c885e |
| crates/re-cordis-loader/examples/boot_reconfigure.rs | a249e35ce5ad251111196f7f51f9748478eee4d29bb29a9876bdcb51a59471d8 |
| crates/re-cordis-loader/tests/review_repairs.rs | 8f7b3184478a1bdaa7d7e0e2c1686cf2018623b4445ff7c050bf9f88bd3ce8eb |
| crates/re-cordis-loader/tests/acceptance.rs | 6ea86a4201f3e4398761d68d589516a1deefa2ac958a985b81ba97b95f2f1377 |
| crates/re-cordis-loader/tests/loader_parity.rs (separate agent, preserved) | 5ebde46941e5d6a3e74a16ad8600e8dfeb2dba444cd80a4adcba479bb276bfe3 |


## Final-review correction wave — verified checkpoint 1

Base checkpoint: b98eae9. The final independent review reproduced F1–F8;
the previous freeze above is superseded by this correction wave.

Added eight bounded public regression scenarios in tests/final_wave.rs.
All eight were observed failing on b98eae9 for the review's specific oracles
(F7 metadata gets separate private-state/limit coverage in the next subset).

Verified first subset: F1/F2/F5/F6.

- F1 checks entry liveness/revision across callback-bearing child mutations and
  before membership assignment; a reentrant membership replacement now leaves
  group:new visible and removes group:child.
- F2 callback-bearing group reevaluation runs through a rebuilt outer lifecycle.
  Same-spec condition and intercept failures now clean up to Failed, and explicit
  recovery restores Active. Pure structural groups retain the fast path.
  Logical EntryIds remain stable; group/descendant FiberIds may change during
  callback reevaluation.
- F5 group-to-plugin replacement removes its obsolete logical descendants and
  joins the gated core cleanup before success; child config captures release.
- F6 prospective ancestry IDs are included in structural update preflight,
  preserving source, membership and commit stream on cyclic refusal.

The four new affected regressions pass. Existing review_repairs, groups, nested,
updates and acceptance suites were rerun for this subset (32 integration tests);
their exact result is recorded in the agent checkpoint message.
Known open findings at this checkpoint: F3/F4/F7/F8 and anonymous positional-owner
policy. Final-wave tests for those branches remain intentionally failing.
No core APIs or commits were added by this worker; root serializes checkpoints.


Checkpoint 1 final gate: PASS — cargo test -p re-cordis-loader (61 integration
tests, 1 unit, 2 Rustdoc), Clippy --all-targets -D warnings, and crate fmt --check.
The four unresolved public regressions, shared helpers and complete original
final-wave content are preserved in .superpowers/sdd/loader/final-wave-pending.rs;
normal integration discovery now contains only the four fixed final-wave tests.
