# Independent lifecycle/update B0 review

Reviewed commit: **ad3a799** (family commits 755a1a7, 9cc0a64 and 1626c54).
This is an isolated managed checkout of that commit. Specification: Cordis
**35fdc969e4b6c48c48ed44717c7a018324e4dd88**.

**Spec verdict: changes requested. Quality verdict: changes requested.**
The declarations, variants, preserved provenance and execution joins reconcile,
and the four bounded paired-scenario mappings are sound. Several ordinary
passing-test dispositions describe assertions absent from their cited tests.
These are evidence defects; this review demonstrates no production failure and
authorizes no B2 change. This is a family review, not full-B0 acceptance.

Inputs: [plugin](ledger-inputs/plugin.json), [fiber](ledger-inputs/fiber.json),
[update](ledger-inputs/update.json), and the
[family report](ledger-inputs/report-lifecycle-update.md). The accepted matrix,
recovered validation and earlier decorator/reflect/events decisions were reused.
M3-M6 typed declarations, stable identities, initialization, source/resolved
validation, lifecycle tasks, status hooks and effect streams remain selected
contracts. No unchanged design decision is reopened.

## LR1 — Separate three non-obligations from behavioral gaps (P2)

All three flagged records are genuine inventory non-obligations. None is a
source-offset error or an uncovered framework behavior:

| Preserved obligation ID | Exact pinned source | Required disposition |
|---|---|---|
| core/plugin.spec.ts:77/default/a84 | plugin.spec.ts:84 is the closing token; the four registry operations are at 80-83 and already have records. | Parser false positive; no executable observation. |
| core/fiber.spec.ts:71/default/a86 | fiber.spec.ts:86 is a commented-out expectation of two apply calls. Live assertions are at 84, 85, 87 and 90. | Commented expectation; no asserted count at this location. |
| core/update.spec.ts:7/false/a21 | update.spec.ts:21 guards the dependency-value assertion with if (inject). | Guard is false in this variant. The true-variant obligation remains real and separately classified. |

Their missing-evidence/observed-source-gap representation preserves provenance
but includes them in behavioral gap totals. At final integration, retain their
original IDs, source file/line arrays, declaration and variant parameters in
explicit inventory-non-obligation metadata, with the reasons above and
execution_status **not-applicable**. Exclude them from behavioral obligation,
class and gap counts. Extend the integrated validator to check this provenance
and branch disposition. Do not silently delete or renumber the original
control records, count these as passing tests, or call them unsupported behavior.

The original 144 declaration / 156 registered-variant controls stay unchanged.
This family's raw 188 inventory records contain **185 meaningful obligations
and three non-obligations**, before corrective splitting/remapping. Removing
these three alone leaves 17 currently declared behavioral gaps; LR2-LR5 mean
final class totals must be recomputed afterward. No source/test fix is required
for any of the three artifacts.

## LR2 — Configuration and invocation claims exceed their witnesses (P2)

In plugins.rs, cloned_declarations_group_distinct_instances_and_empty_groups_are_pruned
records (FiberId, config) at 36, but 56-58 assert the two handle configs and only
the vector length. They do not compare recorded callback configs or establish
one invocation per distinct instance. The explanation that recorded inputs for
both instances are asserted is false for plugin :8/default/a15 and
fiber :292/default/a299. The per-instance count claims at plugin :8/default/a14
and fiber :292/default/a298 also need a more precise witness.

A concrete existing candidate is plugins.rs,
typed_object_method_adapter_keeps_config_and_cleanup_with_its_instance: the
ordinary Plugin callback forwards config to Behavior::apply at 1161-1162,
records it at 1144, and asserts the complete two-input vector at 1170-1172.
Reuse that witness with its explicit adapter/constructor relationship and
already-passing stable/MSRV identity; do not invent a vector comparison in the
original witness.

More substantially, acceptance.rs:163-189 creates a shared-handle plugin whose
callback **ignores its u32 config** (165). It observes handle configs 5 and 7
and shared identity, but records neither apply inputs nor apply counts. It
cannot substantiate these source-mapped obligations:

| Obligation | Missing observation |
|---|---|
| fiber :292/default/a303 | Exactly two callback invocations after the first update. |
| fiber :292/default/a304 | The callback receives the first updated input. |
| fiber :292/default/a309 | The callback receives the second updated input. |

The existing a308 gap correctly identifies the third sequential count. Extend
its next action to these callback-input/count boundaries if no exact existing
witness is found. A handle getter is not an apply-input assertion. Keep the
alias test for the documented shared-handle representation.

## LR3 — Cleanup witnesses and repeated-disposal scope need correction (P2)

Plugin :165/default/a182 claims that the initializer's returned disposer ran
exactly once, citing acceptance.rs:293-373. The closed flag is set by a
**separately registered** on_dispose_sync at 319-325. The disposer returned by
the initializer at 335-337 is a no-op. The assertion at 359 does not observe
that returned disposer at all, much less its count. The additional M6 candidate
repeated_initialized_services_restore_registry_effect_and_listener_baselines
(745-803) also returns a no-op disposer (771). Reclassify the exact boundary as
missing evidence unless another returned-disposer witness is joined; extend
the initialization-count action to observe the returned disposer. Preserve
the tested availability barrier and existing initialization representation.

In plugins.rs, parent_cleanup_cascades_to_nested_plugins_and_preserves_root_listeners,
the final listener count is **5 at 389**, before the second disposal at 390.
The next operation drains the root at 391; there is no post-second-disposal
emit/count or Registry-empty assertion. Plugin :86/default/a118 and a120 cannot
say that this test asserts an empty Registry and root-only dispatch after
repeated disposal. Rejoin an exact existing witness or retain these boundaries
as named gaps. The first-disposal assertions at 382-389 remain valid.

These narrower explanations also need correction:

| Obligation(s) | Cited evidence limitation / correction |
|---|---|
| plugin :36/default/a47 | The rejected late child at plugins.rs:419 has an empty callback. An empty runtime-failure list (466) does not prove that callback never ran. Keep admission rejection at a41; rejoin or gap the separate zero-invocation claim. |
| plugin :86/default/a103 | The counter is initialized at 286 but not asserted after load and before dispatch. Its first observation is 4 at 380. Describe the actual witness or retain the unobserved pre-dispatch boundary. |
| plugin :145/default/a150 | The root-drain test contains no child/root identity comparison. Existing exact comparison is at plugins.rs:49-50; rejoin it for the typed identity substitute. |
| plugin :145/default/a152, a157, a162 | The root-drain witness (812-851) never reads the journal. Later successful admission/drain does not prove an empty journal. Existing journal observations include plugins.rs:103, 113 and 1019-1020; reconcile their exact operation scope or mark the remaining root-drain boundary missing. |
| plugin :145/default/a159, a160, a161 | The second drain overlaps the first (835-841). Identity/state checks are at 842-843, before the later drain at 846. This is not the claimed post-completed-second-disposal checkpoint or a disposer counter assertion. Preserve the overlapping-drain evidence without claiming the different repeated-completed-disposal phase. |
| fiber :185/default/a197 | plugins.rs:917-953 has no pre-disposal zero-counter check; the counter is asserted only after disposal at 942. |
| fiber :185/default/a200 | The cleaned counter observes the successful sibling disposer (926-929), not the throwing disposer (930). Failure entries and panic/error checks at 943-946 support diagnostics for the failing callbacks; rewrite the invocation/count explanation accurately or retain the exact invocation-count boundary as missing. |

This does not remove Rust's selected FnOnce/scoped-effect representations.
Identify the tested substitute and actual scope without inventing a checkpoint,
counter or journal read.

## LR4 — Failed recovery is confused with a failed active reload (P2)

Fiber :144/default/a152 maps to updates.rs,
asynchronous_recovery_is_awaited_and_dropped_failed_recovery_is_still_diagnosed.
That test is **Active at 888** when it awaits failing update -2 at 889: a failed
reload from Active. The actual failed-to-failed recovery is update -3 at 891,
whose update waiter is dropped; line 892 awaits ready(). Neither exercises the
source's retained recovery-update waiter returning the retry failure from an
already-Failed Fiber.

Rejoin an existing retained failed-recovery update assertion or mark a152 as a
named gap. Correct a153's action: adding a state check after the -2 reload alone
does not test failed-to-failed recovery. The minimal follow-up observes the
recovery update's own error and Failed state from the correct starting state.
Initial-error and dropped-waiter coverage cannot close this distinct boundary.

## LR5 — Apply one rule to final-state evidence (P2)

The fragment correctly leaves final Active checkpoints open for unloading,
ABA, pending dependencies and accepted-during-loading updates when the test
only awaits and unwraps a task. That rule is inconsistently waived here:

| Obligation | Claimed checkpoint absent from cited witness |
|---|---|
| update :54/default/a65 | updates.rs:172-177 joins two tasks and checks the sole applied value 3; it does not compare the returned state or first.state(). |
| update :188/default/a208 | updates.rs:752-758 joins the update, checks the stream trace and no failures; it does not compare Active. A stream representation does not supply the lifecycle assertion. |
| fiber :396/default/a409 | updates.rs:239-259 has no Active-state assertion after invalid loaded validation. |
| fiber :326/default/a364 | acceptance.rs:185 checks Active after an update-middleware type error on a different plugin, not the concurrently updated provider/consumer checkpoint. |

Rejoin exact existing state evidence where available; otherwise record the
uncovered checkpoint using the same rule as the existing state gaps. For a364,
the accepted combined paired scenario supplies an exact witness:
tests/parity/lifecycle.spec.ts:132-137 and tests/parity/combined.txt observe
settled provider=Active consumer=Active. Attach its current counterpart/result
join instead of the unrelated middleware-error state. If classified as another
executed comparison, recompute the total rather than preserving 23 by fiat.

## Accepted scope and evidence boundaries

All 42 declaration identities, titles, suites, spans and original assertion-line
provenance agree with the inventory and pinned source. The 44 variants preserve
inject=false/true and restart/config ABA exactly, without an unintended product.
All assigned source bodies and meaningful helper/branch paths were compared;
the three non-obligations were checked against actual tokens and guards.

File counts are plugin 10 declarations / 10 variants / 46 records, fiber
22 / 22 / 88, and update 10 / 12 / 54. Submitted classes are 23 executed
comparisons, 96 source-mapped tests, 49 Rust representations and 20 missing
evidence records. These are **verified accounting totals, not accepted semantic
pass totals** pending LR1-LR5.

All 168 Rust evidence references join one exact passing result and original
log line on each toolchain: **336 joins, 31 distinct Rust test/Rustdoc identities**.
The malformed-plugin representation correctly joins the plugin::Plugin
compile-fail Rustdoc at line 85 on both toolchains. No claimed test was missing
from either result set or log. This confirms execution, not the semantic
implications challenged above.

The current 23 paired obligations are bounded appropriately across four
scenarios: explicit-recovery (Failed refresh, second attempt and gated recovery
waiter), latest-failure (retained/abandoned waiters, rollback, state and original
error), superseded-failure (rollback, latest success, diagnostic and cleanup),
and combined (old/old and new/new application pairs only). I compared the TS
fixture's branches/assertions and the Rust helper bodies, including original
error-identity helpers, with the retained observations. Trace strings are
complete lines in the fixed expected files and all four comparison summaries
appear in the parity log. The accepted corpus remains six scenarios / 73
observations; this review adds no scenarios and does not execute the original
plugin/fiber/update suites.

The 17 non-inventory gaps identify concrete boundaries in candidate tests;
owners and conditional B2 routing are populated. LR4 corrects a gap action;
LR2/LR3/LR5 add or rejoin unsubstantiated claims. Evidence follow-up comes first.
Production B2 entry still requires a failing public-API reproducer plus an
actual dependent gate; neither was established here.

## Actual review checks

Used CodeGraph MCP first against integration, then qualified codegraph node
file/range queries for trimmed sections. Inspected the assigned pinned TS
specs/utils, current plugins/updates/acceptance/hooks tests, relevant parity
helpers and TS fixture, and carried maps/decisions. Additional candidate reads
included foundations and the initialized-service baseline test.

Independent standard-library checks executed in this review session:

- Parsed fragments and control inventory; compared identities, exact variant
  parameters, original obligation IDs/line arrays, derived classes, source
  title/line bounds and Rust reference bounds. Result: 42/44/188 and submitted
  class totals, with no identity/provenance mismatch.
- Parsed matrix commands/results and original logs; checked exact suite,
  symbol, outcome, command membership/exit zero and Rustdoc item/line/kind for
  every entry. Result: 168 references, 336 passing joins, 31 unique identities.
  Each paired observation matched a complete expected-trace line and its
  scenario's exact comparison summary in the parity log.
- Called the documentation checker's anchors() helper without running examples.
  Result: PASS restrictions=52 gap-routes=20 report-links=11.
- Ran git diff --exit-code 808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f ad3a799
  with pathspecs crates, tests, Cargo.toml, Cargo.lock and spikes/m0: exit 0,
  no output. Reused matrix executable inputs are unchanged.
- Upstream git rev-parse HEAD returned the exact fixed pin. Upstream git status
  --porcelain exited 0 without output.
- Review-file link validation and git diff --cached --check passed before
  commit; only this review file was staged.

The report's mechanical validation is reproducible within its accounting and
reference limits. Those checks cannot prove callback input, final state or
disposer identity merely because lines are in bounds and a cited test passes.
No Cargo, docs examples, parity or baseline reruns, input edits, production
changes or subagents were used. Corrections and integration remain with the parent.
