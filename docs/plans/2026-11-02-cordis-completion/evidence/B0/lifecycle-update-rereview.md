# Independent lifecycle/update correction re-review

Reviewed snapshot: **1ac2986e05a4a2dbffc883d221cd32fbab92407b**.
Correction: **165df37**, based on **1c38165**, integrated as **f438233**.
The four corrected files are byte-identical at the correction commit, its
integration commit and this review snapshot. Specification: Cordis
**35fdc969e4b6c48c48ed44717c7a018324e4dd88**, clean in the integration reference
checkout. Review worktree:
`/Users/I756434/.codex/worktrees/b0-lifecycle-update-rereview/cordis-rust`.

**Spec verdict: accepted with tracked gaps for LR2–LR5. Quality verdict:
accepted. No new findings.** Correction acceptance is conditional only on the
separately owned LR1 inventory integration described below. This scoped verdict
does not accept the complete B0 ledger or close the remaining evidence gaps.

Inputs are the [initial review](lifecycle-update-review.md), corrected
[plugin](ledger-inputs/plugin.json), [fiber](ledger-inputs/fiber.json),
[update](ledger-inputs/update.json), and [correction report](ledger-inputs/report-lifecycle-update.md).
The [master reuse rule](../../README.md#prior-decisions-and-evidence-reuse),
[current progress](../../progress.md) and [B0 Tasks 2–3](plan.md#task-2-complete-declaration-and-event-type-disposition)
define the scope. Existing M3–M6 typed adapters, identity, lifecycle tasks,
validation, cleanup, streams and Failed-recovery decisions remain carried
contracts. Accepted matrix/report, FR-1, first-two-events, event-type and
integrated-validation reviews are not repeated.

## LR2 — Callback inputs and counts addressed

- `plugin :8/default/a14, a15` and `fiber :292/default/a298, a299` now cite
  `typed_object_method_adapter_keeps_config_and_cleanup_with_its_instance`.
  In `plugins.rs`, the ordinary `Plugin::new` callback forwards its input to
  `Behavior::apply` at 1161–1162; 1144 records that input. Admissions at
  1166–1167 use distinct `first`/`second` values; after both ready barriers,
  1170–1172 compare the complete sorted vector to exactly those two values.
  This establishes one invocation with each distinct admitted input. The
  functional-plugin records retain the explicit-adapter representation; the
  initial fiber callback records retain their source mapping.
- `fiber :292/default/a303, a304, a308, a309` are precise sequential-input/count
  gaps. `acceptance.rs:165` ignores the callback input; handle getters and the
  alias identity at 172–174/189 cannot supply that journal. The shared follow-up
  requires exact initial, first-update and second-update vectors/counts while
  preserving the separately tested alias representation.

## LR3 — Every cleanup and checkpoint correction addressed

| Obligations | Verified correction |
|---|---|
| `plugin :165/default/a178, a179, a181, a182` | All four are evidence gaps with initializer/returned-disposer counter follow-up. `acceptance.rs:319–325` registers the cleanup that changes `closed`; the actual initializer return at 335–337 is a no-op, as is the repeated-initialization candidate at 771. The availability barrier remains carried evidence, not a disposer-count assertion. |
| `plugin :36/default/a47` | Gap now distinguishes admission rejection from zero callback invocation. `plugins.rs:419` passes the empty late-child callback; no callback counter exists. The separate admission-error mapping at a41 is preserved. |
| `plugin :86/default/a103` | Gap records missing zero-count observation after readiness and before dispatch. The first counter assertion is four at `plugins.rs:380`. |
| `plugin :86/default/a118, a120` | Gaps now require Registry and root-only-dispatch observations after the second completed disposal and before root drain. The empty Registry at 383 and count five at 389 precede disposal 390. |
| `plugin :86/default/a104, a106, a111, a113` | The shared rationale now describes its actual first-disposal scope. Three Registry groups at 373, first count four at 380, empty Registry at 383 and root-only increment at 389 remain valid mappings. |
| `plugin :145/default/a150` | Correctly rejoins the opaque-identity substitute at `plugins.rs:43–53`, including explicit child/sibling and child/root inequality at 49–50. |
| `plugin :145/default/a152` | Correctly remains a cardinality gap. `plugins.rs:103` reads the first entry's child-disposal label but never asserts journal length exactly one; the root-drain candidate never reads its journal. The action requires exactly one registration before root drain. |
| `plugin :145/default/a157, a162` | Gaps identify missing root-journal snapshots after completed drains. Journal observations after Registry removal or explicit child disposal do not establish these root-drain checkpoints. |
| `plugin :145/default/a159, a160, a161` | Gaps distinguish overlapping drains at `plugins.rs:835–841` from a second drain started after first completion. Identity/state at 842–843 precede the later drain 846; no disposer count is asserted. The action adds the completed-second-drain identity, state, count and empty-journal checks before fresh admission. |
| `fiber :185/default/a197, a200` | Gaps now require the throwing cleanup's own pre/post-disposal counter. `plugins.rs:942` measures only the successful sibling registered at 926–929; the throwing callback at 930 has no counter. Failure/panic observations at 943–946 remain diagnostic evidence at a201. |

These dispositions preserve the selected scoped cleanup/FnOnce substitutes
without treating them as observations of absent counts or operation boundaries.

## LR4 — Failed-to-Failed recovery boundary addressed

`fiber :144/default/a152` is now missing evidence, and a153's existing action
is corrected. `updates.rs:888` is Active before the retained failing -2 reload
at 889. The actual subsequent failing recovery -3 at 891 drops its update
waiter; 892 awaits `ready()`. Neither observes the source's retained recovery
update returning its own retry error from an already-Failed fiber. Both gap
actions now require that starting state, the retained update's own callback
error and final Failed state, preserving the separate dropped-waiter test.

## LR5 — Final-state rule applied consistently

`update :54/default/a65`, `update :188/default/a208`, and
`fiber :396/default/a409` are explicit evidence gaps. Their candidates at
`updates.rs:172–177`, 752–757 and 239–259 respectively observe successful joins,
apply/stream traces or validation rejection, but no Active comparison at the
claimed boundary. The stream representation at update a207 is preserved.

`fiber :326/default/a364` now uses the accepted **combined** comparison.
`lifecycle_parity.rs:233–273` selects Active for the combined consumer, joins
readiness and compares both provider and consumer states; the wrapper at
292–293 selects `Scenario::Combined`. The TS fixture at
`tests/parity/lifecycle.spec.ts:132–137` compares both states after readiness.
The complete `settled provider=Active consumer=Active` line is present in
`tests/parity/combined.txt`, with the existing successful comparison in the
parity log. The unrelated middleware-error state at `acceptance.rs:185` and
the wrapper restriction are removed from this behavioral checkpoint.

This is one additional obligation attached to existing evidence: **24** paired
obligations across four reused scenarios, with no new scenario or observation.
The accepted corpus remains six scenarios / 73 observations.

## Accounting and sole integration condition

All **42 declarations / 44 variants / 188 raw records** preserve their original
IDs, titles, suites, source spans, assertion-line arrays and exact variant
parameters. The parameter sets remain inject=false/true and mode=restart/config
ABA. Family counts remain plugin 10/10/46, fiber 22/22/88 and update 10/12/54.

The correction changes 35 obligation records: 21 plugin, 12 fiber and two
update. Twenty unsubstantiated witness claims become gaps; five existing gap
records receive more precise actions; nine mapping/representation records
receive corrected witnesses or rationale; one representation becomes the
combined executed comparison. No record is added, removed or renumbered.

| Meaningful evidence class | Count |
|---|---:|
| Executed comparison | 24 |
| Source-mapped test | 84 |
| Rust representation | 40 |
| Missing evidence | 37 |
| **Meaningful obligations** | **185** |

Raw classes still include 40 missing-evidence records because these **three
LR1 records are deliberately unchanged**:

- `core/plugin.spec.ts:77/default/a84` — closing token, no observation.
- `core/fiber.spec.ts:71/default/a86` — commented-out expectation.
- `core/update.spec.ts:7/false/a21` — inactive inject guard; true variant stays real.

The exclusive ledger owner must preserve those exact IDs/source/variant
provenance in validated inventory-non-obligation metadata with
`execution_status=not-applicable`, outside behavioral/class/gap totals. This
is the sole condition on accepting this correction; no further LR2–LR5 repair
is requested. The original 144-declaration/156-variant controls remain intact.

All 37 genuine gaps have concrete regression actions, `core evidence
maintainer` ownership, and dependent-gate triage. Conditional B2 admission
requires a failing public-API reproducer and a dependent satellite need.
No production failure or B2 entry was established. B0 accounting cannot close
these gaps or admit a dependent slice whose needed evidence is still absent.

## Actual checks and limits

CodeGraph MCP/CLI supplied the corrected Rust witnesses, relevant pinned TS
assertions and the combined helper/fixture checkpoints. I checked all LR2–LR5
findings and the initial review's rationale subtables against the changed JSON
and report, rather than repeating the whole-family audit.

The correction report's recorded final check was rerun with only its ownership
comparison adapted to this later review snapshot: compare changed paths from
`1c38165` to `165df37`, then require all four owned files to equal their
`165df37` and `f438233` blobs. All other source/variant/schema/log assertions
were retained. It exited **0** with:

~~~text
PASS shared obligation schema/source/variant provenance: 42 declarations, 44 variants, 188 raw records
PASS raw classes: comparisons=24 source-mapped=84 representations=40 missing=40
PASS meaningful obligations=185 missing=37; three LR1 records unchanged for integration
PASS 148 Rust references, 296 both-toolchain exact result/log joins, 31 identities; Rustdoc item/line/kind checked
PASS 24 paired obligations across four existing scenarios; combined final-state join exact; accepted corpus six/73 unchanged
PASS clean TS pin; exactly four owned paths; executable inputs, shared ledger/validator and matrix unchanged
PASS correction 165df37 and integrated f438233 owned bytes match review HEAD 1ac2986
~~~

Thus every one of 148 Rust evidence references has an exact passing result and
original log join on both toolchains, including Rustdoc item/line/kind. The
31 distinct identities and 296 joins agree with the corrected report. The
report preserves its earlier refinement failure that expected 32 identities;
removing the label-only journal witness explains the derived count of 31.
The failed refinement is not claimed as a successful run.

A separate standard-library comparison derived the 35 changed records and
class transitions above, and asserted all 37 genuine gaps' populated actions,
owner and dependent-gate/failing-reproducer routing; it exited **0**.
The scoped reference checks validate restriction anchors and complete paired
trace lines. Local links/anchors and whitespace in this review and the
correction report were checked before committing; only this review was staged.

The first review-file formatting check exited 1 after detecting indentation
inserted into the IDE-written multiline literal and a missing final newline.
Only this review's formatting was normalized; the retry passed.

No Cargo, Rustdoc, documentation-example, TypeScript or parity suite was
rerun. No production, test, fixture, matrix, shared ledger/validator, tracking
or other family file was changed. No subagents, push or publication were used.
This is evidence acceptance for the LR2–LR5 correction, not whole-B0 acceptance.
