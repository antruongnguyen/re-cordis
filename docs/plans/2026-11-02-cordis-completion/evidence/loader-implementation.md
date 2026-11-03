# Loader implementation checkpoint history

Chronological record; later checkpoints supersede earlier frozen hashes, counts
and toolchain attribution. Final acceptance uses explicit rustup tool paths.

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


## Final-review correction wave — verified checkpoint 2

F3/F7/F8 and anonymous positional ownership now pass their targeted regressions.
F4 is the only remaining open finding; both overlap/teardown regressions and
their shared error helpers remain preserved in final-wave-pending.rs.

- F3 prepares shallow source nodes and claims IDs incrementally across the full
  source tree. Later duplicate children report their own original source path;
  the first valid x and independent y remain, including typed and JSON sources
  and a second reconciliation against the existing group.
- F7 source records are pruned to currently owned root entries; empty desired
  sources are removed instead of inserted. Named realms are weakly interned;
  immutable Scope definitions and EntrySnapshots retain tokens. The same label
  shares its identity while those tokens are live; after they release, the label
  can allocate a fresh identity. Old Context views preserve the old selected
  IsolationId. A maximum of MAX_ENTRIES (1024) simultaneously live named realms
  is enforced before inserting a new label; released scopes restore capacity.
  2048 empty-source and 2048 rejected-decode cycles both leave the corresponding
  metadata map empty after admission/pruning.
- F8 malformed nested group config errors are Located at the actual
  [root,"config",child,"config"] path with the supplied SourceId and span.
- Anonymous positional reuse requires current source ownership and root
  membership. A's next anonymous snapshot allocates a new entry after B has
  adopted A's prior identity; B's origin/config remain intact.

Green gates: cargo test -p re-cordis-loader PASS (66 integration tests, 3 unit
tests, 2 Rustdoc), Clippy --all-targets -D warnings PASS, crate fmt --check PASS.
No core API or production dependency additions.

SHA-256 at checkpoint 2:
- src/lib.rs: a16347a12023c05996ed5c2e0d595e07cf985541949c7cb894a7e3401d7aee3e
- src/scope.rs: 719989c379f002bfcf4d313a5388bfcac8369c3114fb8b7b6c4c226517aa0666
- tests/final_wave.rs: 88684ac9a657faceecb5d307ab210f188c4152d041b1e5459d152ca6bbdb644d


## Final-review correction wave — F4 progress (production implemented)

All 12 current final_wave tests pass, including the restored F4 overlap case
(success and older error after a dropped settlement waiter), scoped entry/loader
teardown gates, and a 1024 outstanding-update admission/capacity test.
Clippy --all-targets -D warnings passes after the queue changes. No design blocker.

Every direct update is recorded before commit in a bounded global pending queue.
Observation markers distinguish explicitly awaited operation results from dropped
waiters. A guarded reservation also counts reentrant synchronous admission
prefixes against the 1024 limit. One unobserved older failure is retained until
a completed settlement delivers it; dropped settlement futures do not acknowledge
it. Entry removal joins only the removed subtree's outstanding updates; loader
disposal and owner shutdown join the whole queue. Completed operations release
queue capacity on admission/pruning.

The final full crate suite and final frozen source fingerprints are still pending
at this progress boundary; no acceptance claim is made until those gates finish.


## Final-review correction wave — F4 finish and scoped F5 reentry repair

Continuation completed 2026-11-03 in loader-completion worktree. Preserved
all inherited F1/F2/F3/F5/F6/F7/F8 source, scope, tests and documentation.
Only src/lib.rs, tests/final_wave.rs and this appended report were changed
by this continuation. No core/dependency changes, commits or pushes.

F4 admission/teardown:
- Every direct update remains in the global 1024-operation bounded queue;
  reentrant synchronous prefixes reserve a slot before calling core update.
  Pruning completed operations releases capacity while retaining one global
  unobserved failure, rather than an unlimited historical error list.
- Settlement joins every queued update; subtree removal joins only that
  subtree's captured updates; loader disposal and external parent drain join
  the whole queue. Older work survives newer out-of-order completion.
- A completed original update waiter observes its own operation. A completed
  subtree removal observes its captured updates and matching retained failure.
  Completed settlement consumes the retained failure and observes completed
  queued operations. Dropped waiters never acknowledge. Loader.dispose and
  owner cleanup report/return retained unobserved failures without marking
  their observation token.
- Latest per-entry work failures continue to exist independently of bounded
  historical retention; explicit recovery replaces that latest work.
- Concrete additional red: after an older error was retained and a settlement
  waiter dropped, a newer gated error caused settlement's or_insert to return
  only the newer error and consume the older one. The dropped-settlement test
  failed contains_older(report.failures[0].error). Delivery now gives the first
  retained error precedence for its entry; the exact case is green.
- Original F4 overlap and teardown reds on b98eae9 remain documented in the
  inherited independent review/checkpoint reports. The resumed run verified
  their current greens and added direct parent-drain coverage.

Scoped F5 follow-up requested by parent/scoped reviewer:
- Concrete red 1: parent Group->Plugin(2) commit callback updates the same
  parent to Plugin(3); snapshot.children remained nonempty despite parent
  source=3/Active. Every accepted non-Group replacement now removes actual
  logical children it still owns, preserving revision/liveness guards.
- Concrete red 2: the gated variant returned Ready(Ok(())) for the original
  update before child cleanup released. Shared rebuild work is admitted
  before the synchronous commit callback, preserving the original retirement
  handoff through newer coalesced updates. drive additionally joins the
  snapshotted detached logical descendant retirements from existing retiring
  state before spawning the latest generation. No global unrelated-cleanup
  gate was added.
- Green oracle asserts original update remains Pending and apply values stay
  [1] before release; after release apply values are [1,3], accepted source=3,
  parent Active, no children, no child snapshot or address, no failures.

Final fresh gates (PATH Homebrew cargo/rustc 1.99.1, --locked):
- cargo test -p re-cordis-loader --locked: PASS — 71 integration tests,
  3 unit tests, 2 Rustdoc checks (14 final_wave tests).
- cargo clippy -p re-cordis-loader --all-targets --locked -- -D warnings: PASS.
- cargo fmt -p re-cordis-loader -- --check: PASS.
- git diff --check: PASS.
No broad workspace or stable/MSRV documentation matrix run by this worker;
parent owns that acceptance and independent scoped review.

New final_wave names:
- reentrant_plugin_replacement_removes_actual_obsolete_children
- dropped_settlement_after_older_error_retention_keeps_failure_visible
Extended:
- teardown_joins_an_older_direct_update_after_a_newer_update_completes
  now includes external root.drain for both successful and failing older work.
Inherited overlap, capacity and other final-wave tests preserved.

Source/tests frozen for parent integration:
- crates/re-cordis-loader/src/lib.rs:
  5a68375b64723f35dcb61eda49ed01458fd653687ec150e7c8d4e5450a91beb2
- crates/re-cordis-loader/tests/final_wave.rs:
  851ef8c009cd47072e4e19fd5bb74584922e46744a154c8abd6b72894fcb7d62

Ready for review. No source/test edits after the freeze notification unless
parent requests a further scoped repair.

Toolchain attribution checked explicitly after verification: /opt/homebrew/bin/cargo
and /opt/homebrew/bin/rustc were used. rustup active reports stable, but explicit
rustup stable is 1.97.1. These local gates therefore establish PATH 1.99.1 only;
parent owns explicit stable/1.85.0 verification. Source/tests hashes unchanged.
