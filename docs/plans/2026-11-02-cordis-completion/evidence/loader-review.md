# Final independent loader review — changes requested

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Worktree: /Users/I756434/.codex/worktrees/loader-completion/cordis-rust.
Baseline: cefb4c2; frozen full package in final-review.diff.
Verdict: **changes requested before L0–L5/G0–G1 acceptance**.

The documented idiomatic Rust scope is appropriate. This review does not require
unrestricted TypeScript parity, a core rewrite, filesystem persistence or HMR.
It found eight concrete behavior defects in the frozen implementation. The
original correction report is accurate about its selected passing scenarios,
but several adjacent branches still violate the delivered Rust contracts.

## Findings

### F1 — P1: Reentrant group reconciliation overwrites newer accepted membership

Location: crates/re-cordis-loader/src/lib.rs:1216–1256, especially the unconditional
children assignment at 1256 after callbacks at 1231–1232.

Reproduction: start group with child; while a runtime update commits child,
a synchronous commit observer reconciles the same group to contain new instead.
The inner reconciliation succeeds and the accepted options contain new. The
older reconcile_group then resumes and overwrites children with child.

Observed:
- options desire [new], but snapshot.children is [child];
- entries() contains only group, and resolve("group:new") fails;
- settlement failures and runtime diagnostics are both empty.

The new child remains in the entries map/core ownership tree but is hidden from
public tree traversal; the removed old ID is retained in the parent list. This is
silent tree corruption under a callback pattern explicitly supported by
review_repairs.rs:547. That regression changes only config on the same child and
therefore cannot expose the stale-membership overwrite. Track/recheck the parent
revision/liveness across callback-bearing child mutations and abandon a stale
reconciliation before further updates/removals or the final membership write.
Verify membership replacement/removal, not only config replacement.

### F2 — P1: An Active group's failed reevaluation leaves the old group and children Active

Location: crates/re-cordis-loader/src/lib.rs:1501–1514; start propagates the
condition/intercept error at 1017–1021 / 1033–1038 without a lifecycle transition.

Reproduction: create a group whose captured condition initially returns true;
after it and its child are Active, make the same closure return an error and
update with a clone of the same spec. The structural fast path calls start
directly rather than through an outer lifecycle apply.

Observed: update returns Err, but group and child remain Active; the subsequent
settlement has no failures. The prior true verdict and active resources survive.

This contradicts the paired API/decision contract that typed condition/intercept
failure produces Failed and requires explicit recovery. The original Pending and
already-Failed tests pass because they take the rebuild path, not this fast path.
Route fallible reevaluation through a lifecycle transition or explicitly provide
the same failure/cleanup/recovery behavior; verify Active→failure→explicit
recovery for both condition and intercept.

### F3 — P1: Duplicate source child IDs discard the entire group and healthy siblings

Location: crates/re-cordis-loader/src/lib.rs:2144–2163, especially whole-group
preflight at 2162 after individually preparing children.

Reproduction: reconcile a typed source containing group children x, x, y, all
otherwise valid. Each child passes its independent preflight, then the group's
preflight detects the duplicate and rejects the parent.

Observed: zero entries; both group:x and group:y are absent; one DuplicateEntry(x)
failure is attributed to the parent group at source path [0]. For a previously
accepted group, the invalid-parent removal branch can remove its healthy children
as well.

The source contract explicitly preserves valid siblings. The root duplicate-ID
fix at 2239–2244 does not extend to siblings inside a group (and decoded JSON
groups reach the same prepare_source_spec path). Filter/check source IDs
incrementally at each relevant tree scope, preserving the first accepted identity
and independent valid siblings, with the colliding child's original location.
Runtime whole-input duplicate rejection should remain atomic.

### F4 — P1: Settlement loses an earlier admitted direct update when updates overlap

Location: crates/re-cordis-loader/src/lib.rs:1516–1525, especially work.replace at
1523; settle only awaits the one saved work at 1681–1689.

Reproduction: one loaded plugin has update middleware. Gate its first update;
admit a second update that completes without calling next; drop the first waiter.
Call settle while the first middleware is still blocked.

Observed: settlement_pending=false and first_update_done=false.

The second direct update overwrites the sole saved barrier. core Fiber::ready
tracks a lifecycle pass, not every admitted update middleware future, so the
later ready calls cannot restore the missing barrier. The original acceptance
fixture gates one update only. Preserve/join all outstanding direct update work
with a bounded admission/coalescing policy; a second update must not let settle
or teardown lose the first admitted operation. Verify out-of-order completion
and an error in the older update as well as the gated success case.

### F5 — P2: Replacing a group with a plugin retains disposed logical descendants

Location: crates/re-cordis-loader/src/lib.rs:1484 and 1046–1052; group→plugin
replacement never reconciles/removes the former children.

Reproduction: create parent as Group([child]), settle, then update parent to a
normal catalog plugin and settle.

Observed: entries are [(parent, Active), (parent:child, Disposed)],
parent.children remains [child], and resolve("parent:child") succeeds.

The old core outer is disposed, so the old child stops running, but its logical
Entry/config remains resident and consumes loader capacity until parent removal.
EntrySpec is documented as a complete replacement and does not forbid changing
EntryKind. Remove/join obsolete descendants during this transition (or reject
the transition before mutation if deliberately excluded). Verify child identity,
capture release and capacity restoration, including gated child teardown.

### F6 — P2: Existing-tree cycle checks happen after runtime source commit

Location: crates/re-cordis-loader/src/lib.rs:1411–1417 / 1493–1500, with the
effective rejection deferred to move_origin at 1550–1555.

Reproduction: create outer containing inner, then update inner to contain the
existing outer ID. The candidate contains no repeated ID within its own spec,
so the current preflight accepts it; reconcile_group later attempts the cyclic
move and rejects.

Observed: update returns Err, but one commit has already been emitted and inner's
accepted desired options now contain outer.

This is an entry-identity/tree-structure error, not arbitrary plugin validation,
so it remains inside the corrected whole-runtime structural preflight contract.
Validate the prospective current-tree ancestry before replacing source or
emitting commit. Verify no source, parent/child list or commit changes on refusal.

### F7 — P2: Source and realm side state accumulates outside the entry bound

Locations:
- crates/re-cordis-loader/src/lib.rs:2331 retains every SourceId, including empty
  desired lists; State.sources is at 686.
- crates/re-cordis-loader/src/lib.rs:1805–1817 interns every label permanently in
  State.realms (687), including labels reached before a later decode failure.

Reproduction: reconcile 2048 distinct empty SourceDocuments, then perform 2048
rejected JSON decodes, each with a distinct valid isolate label followed by an
unknown intercept service. Every source reconciliation succeeds, all decodes
fail only on the later field, and resident entries remains zero.

The inserts and absence of removal/cap mean both maps retain all those keys for
the lifetime of a long-lived Loader; MAX_ENTRIES does not apply. The label length
bound alone cannot bound cumulative state, and failed read-only decoding should
not permanently consume an unlimited realm cache. Release obsolete source
metadata, avoid/roll back realm interning on rejected candidates, and bound or
reclaim inactive named realms while preserving the declared sharing semantics.
Document any new explicit limits and verify repeated empty/rejected cycles.

### F8 — P2: Malformed nested JSON groups bypass source locations entirely

Location: crates/re-cordis-loader/src/lib.rs:1825–1829 and 2204–2214.

Reproduction: reconcile a SourceDocument with a named outer group containing
{"id":"broken","group":true,"config":42}, with a locator returning line 7,
column 9.

Observed failure: EntryFailure { id: broken, error: Invalid("group config must be
array") }. It has no SourceId, structured path or span.

decode_source_node returns this bare Invalid before decode_node can attach its
field path, and the nested-failure conversion only decorates errors already
Located. Add the actual child/config path and preserve the source/span through
all nested decode failures. The fixed initial/loaded plugin validation tests do
not cover this early structural decoder branch.

## Disposition of the seven original findings

| Original finding | Review result |
| --- | --- |
| 1 Dependency teardown mistaken for self-disposal | Fixed for the selected contract: owners_active checks loader and the whole outer chain before exact-inner disposal attribution; explicit self-disposal remains separate. |
| 2 Group atomicity/siblings | Correctly narrowed: arbitrary plugin validators are not a cross-plugin transaction; loaded child rejection retains accepted source and valid siblings proceed. Remaining structural defects are F3 and F6. |
| 3 Source adoption/provenance | The requested explicit nested→root adoption, old-source removal guards and runtime-edit owner retention are present and covered. See the nonblocking policy observation below for anonymous positional candidates. |
| 4 Pending/Failed outer gates | Pending/already-Failed/Unloading now use rebuild; initial disabled/blocked evaluation is skipped. Active reevaluation still has F2. |
| 5 Detached cleanup error visibility | The implementation retains the first lifecycle-task error, joins every snapshotted retirement, and uses core diagnostics for ordinary user cleanup failures. The unit fixture is accurately described as synthetic driver-error coverage; no unsupported real-driver-panic claim is made. |
| 6 Callback/destructor borrows | The reported origin/config borrow hazards are fixed: origin is snapshotted and old evaluated Config is dropped outside the mutable borrow. The stronger reentrant tree invariant still fails in F1. |
| 7 Source error location | Initial/loaded plugin validation, original causes and uncompacted JSON child paths are repaired for tested cases; F8 remains in early structural decode. |

## Nonblocking policy observation

Anonymous source roots are matched by the previous positional SourceId list at
2277–2282 without checking current origin. In a probe, A's anonymous entry was
explicitly adopted by B; A's next anonymous reconciliation reclaimed that same
entry and changed its origin/config back to A. This is not counted as a blocker:
cross-source explicit adoption is supported and anonymous matching is positional,
so the intended conflict policy is not unambiguous. Document it or require
ownership when reusing anonymous positional identities before include relies on
this behavior.

## Documentation, checker and paired fixtures

- English/Vietnamese loader/group guides and APIs agree on the core contract and
  contain matching runnable examples. Their broad claims about complete source
  replacement, admitted-work settlement, Failed callbacks and independent
  source siblings are currently contradicted by F1–F6; fixing the behavior is
  preferable to weakening those agreed contracts.
- The checker addition recognizes the current explicit root reexports/Config
  alias and includes loader when compiling examples. No blocker found in this
  focused checker diff.
- The two paired Rust/TypeScript scenarios derive observations from real state,
  membership, apply values and cleanup counts. Their fixture paths, pin, exclusive
  temporary TS file, bounded subprocess/scenario execution and byte comparison
  are clear. No unconditional success observations or unrestricted parity claim
  found. The root owns their final rerun.
- The declaration map explicitly separates 140 declarations into evidence,
  adaptations, unsupported/deferred mechanisms and gaps. Its missing-evidence
  statements remain useful; they are not claims of executed parity.
- The evidence/progress documents intentionally remain pending. That status is
  not a finding. No source/code changes or additional production dependencies
  are requested merely for style.

## Executed review evidence

Only targeted public-API probes were run, in an external standalone scratch crate:
- /tmp/loader-final-review-3sitw303/Cargo.toml
- /tmp/loader-final-review-3sitw303/src/main.rs

It points its path dependencies at the frozen core/loader crates, uses the
explicit rustup-selected stable cargo/rustc/rustdoc, builds offline in its own
target directory, and has a ten-second overall behavior timeout. The final
run exited 0 and printed:

```text
overlap: settlement_pending=false first_update_done=false
kind_switch: entries=[("parent", Active), ("parent:child", Disposed)] children=[EntryId("child")] old_child_resolves=true
nested_duplicates: entries=0 x=false y=false failures=[(EntryId("group"), "duplicate entry: x at [Index(0)]")]
anonymous_identity: entries=1 old_id_origin=Source(SourceId("A")) old_id_config=3
metadata: empty_source_labels_accepted=2048 rejected_decodes_with_fresh_realm=2048 resident_entries=0
reentrant_group: desired=[Some("new")] children=[EntryId("child")] visible=["group"] new_resolves=false failures=[] diagnostics=[]
nested_location: failures=[EntryFailure { id: EntryId("broken"), error: Invalid("group config must be array") }]
active_group_failure: update_refused=true entries=[("group", Active), ("group:child", Active)] settlement_failures=[]
cycle_preflight: refused=true commits=1 retained_desired=[Some("outer")]
```

These probes report observations without intentional assertion failure so all
independent cases run; the reported values are the failing contract oracles.
The accepted crate tests/fmt/Clippy/example and full stable/MSRV/doc matrices were
not redundantly rerun. The parent was running final matrices on the same frozen
code concurrently; a green matrix does not cover these newly reproduced cases.

No production code, test, documentation or Cargo files were edited by this
review. This report is the only retained worktree review artifact created here.
