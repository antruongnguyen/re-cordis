# B0 scoped events/effects correction re-review

Date: 2026-11-02. **Spec verdict: accepted with tracked gaps. Quality verdict:
accepted.** EET-1–EET-9 are closed as evidence corrections. No new correction
finding remains in this scope. The 38 explicit evidence gaps remain open;
this verdict neither closes those gaps nor accepts the final B0 ledger.

## Reviewed revision and scope

- Review checkout: `1ac2986e05a4a2dbffc883d221cd32fbab92407b`, isolated at
  `/Users/I756434/.codex/worktrees/b0-events-effects-rereview/cordis-rust`.
- Correction: `6d94a1776e76bba157ec916df6df33fed7984463`, based on
  `ff414b545a2ca81405a5d4f04fb5a5b2105ba0e0`, integrated as `1ac2986`.
  The two corrected JSON files and B0-root report are byte-identical between
  the correction commit and this review checkout.
- Inputs: [initial review](events-effects-types-review.md), corrected
  [remaining events](ledger-inputs/events-rest.json),
  [dispose/effects](ledger-inputs/dispose.json), and
  [correction report](report-events-effects-types.md).
- Specification: integration's clean `local/cordis` checkout at
  `35fdc969e4b6c48c48ed44717c7a018324e4dd88`. CodeGraph was used against
  integration source first; omitted relevant assertion ranges were read
  directly.

Read repository instructions, the master's
[reuse rule](../../README.md#prior-decisions-and-evidence-reuse),
[resume instructions](../../progress.md#resume-next), B0
[Task 2](plan.md#task-2-complete-declaration-and-event-type-disposition) and
[Task 3](plan.md#task-3-independent-review-and-integration), and applicable
historical [decisions](../../../../decision.md),
[limitations](../../../../limitation.md) and
[implementation plan](../../../../implementation-plan.md). The exact delta is
the nine initial findings, their replacement witnesses, narrowed original
claims, sibling gaps and corrected report. Unchanged accepted mappings and
the six accepted [type representations](ledger-inputs/event-types.json) were
carried forward without another semantic audit.

Typed keys, ControlFlow, explicit eager factories, separate sync/async callback
roles, owned one-use continuations, effect ownership, local hosting and
explicit cleanup remain selected. The prior matrix/report, FR-1, recovered
validation and first-two-event acceptances remain closed. The six paired
scenarios/73 observations remain unchanged. This review owns only this file.

## Scoped finding disposition

Abbreviations retain the initial review's `events:L/aN` and `dispose:L/aN`
notation. Rust line references below are in `crates/re-cordis-core/` at the
reviewed revision. These are observations from source inspection joined to
the preserved passing matrix, not new executions.

| Finding | Verified correction and acceptance boundary |
|---|---|
| EET-1 | `events:65/a72` now owns the missing partial-removal observation; final bucket release remains separate. `events:77/a84` cites `removal_after_selection_does_not_cancel_in_flight_snapshots`: `tests/events.rs` 330–340 asserts `[first,selected]`, clears the trace, and asserts `[first]` after a second emit. Both emits therefore reach the surviving first listener once. |
| EET-2 | Filtered mode rows join the actual `emit`/`parallel`/`serial`/`bail` branches and `[global,visible]` at `tests/events.rs` 237–248. Unfiltered emit is 250–254; unfiltered bail/serial/parallel use distinct values and `[1,0,2,3]` at 55–70. Serial's three immediate rows additionally join the asynchronous first-factory prefix at 421–450, asserted before await. The report correctly keeps timing separate from the awaited selection traces; it does not attribute unfiltered non-emit calls to the filter loop. |
| EET-3 | `events:103/a128` and `/a129` now claim only the asserted `Symbol(parallel)` label, three failures and panic flag at `tests/events.rs` 397–402. Their `-named-label` and `-callback-messages` siblings preserve the missing source details. Emit `/a161-message` separately owns original message content. Dispose `/a188-message` and `/a200-message` preserve factory/iterator error-content obligations without weakening panic/Result/rollback representations. `tests/effects.rs` 231–247 confirms the cumulative failure count is two, including the earlier iterator failure. No input error string is treated as an output assertion. |
| EET-4 | `/a239` retains static Next double-move rejection. Corrected `events:265/a270` explicitly says the AsyncNext snippet at `src/events.rs` 263–265 does not await or execute either call. `events:230/a240` instead joins the runnable sync waterfall branch at `tests/events.rs` 688–725 and its exact selected-callback count at 811. Its narrowed claim is a valid one-use substitute, not execution of TS's invalid duplicate-next program. `/a241`, `events:265/a274` and `/a275` now own the missing terminal counts and single-layer async result 3. |
| EET-5 | The initial two-forwarder result 4 and `[before,after]` remain at `tests/events.rs` 579–584. `events:202/a211` now uses the measured inner forwarding callback in the valid two-frame snapshot substitute, with the different arithmetic/topology boundary explicit. `/a219` honestly retains the simpler outer-plus-veto result 10 at 585–598. `a219-topology` preserves the source's two forwarders, third veto, fourth suppressed listener and result 3; `/a220–a223` preserve all four missing counts. Nested async result 4 remains at 599–638; `events:278/a291–a292` now own its unmeasured listener and terminal counts. |
| EET-6 | `events:296/a310`, `330/a349` and `/a350` retain only scoped local/global count-five evidence at `tests/updates.rs` 424–471. The three `-payload-sequence` siblings separately preserve exact resolved values/order across reloads. The candidate at 219–258 asserts `[resolved 2]` under a vetoing local hook; it does not close those reload sequences. The report does not infer received payloads from update inputs. |
| EET-7 | Both `dispose:123/a138` and `/a140` now point to one before-first-yield regression: immediate empty trace, final `[1,2]`, no later acquisition. The latter's new gap expressly distinguishes `tests/effects.rs` 164–175, which waits for the second in-flight step and ends `[1,3,4,2]`. The accepted line-143 second-in-flight mapping remains separate. |
| EET-8 | `dispose:37/a69` joins the supplied composed-effect witness: setup marker 0 at `tests/effects.rs` 37, immediate `[0]` at 65, and joined `[0,3,2,1]` at 71–72. The `resource` helper at 11–15 uses async `Disposer::new`; the record correctly distinguishes this from all-sync cleanup timing at `/a71`. `dispose:23/a30`, `76/a83`, `143/a156` and `222/a230` now own the absent immediate observations, while their later outcomes remain represented. |
| EET-9 | `dispose:23/a27` retains explicitly named `[resource]`/`[renamed]` snapshots at `tests/effect_metadata.rs` 50–54; `37/a53` retains the explicit `outer`/`nested` tree and transferred listener labels at 123–153. Their two `-default-label` siblings preserve the missing anonymous labels before naming. Neither renamed tree is counted as default-label evidence. |

The restrictions in [M2 event modes](../../../../m2.md#five-dispatch-modes-with-honest-callback-shapes)
and [M2 effects](../../../../m2.md#effects-and-resource-acquisition) support the
bounded substitutes. The narrower original rows do not erase the source
details: sibling gaps or reclassified original rows retain every affected
assertion span. The eleven siblings are all missing-evidence records, not
extra source declarations, variants or successful parity cases.

## Accepted accounting and open work

| Family | Declarations | Variants | Runtime/type records | Source-mapped test | Rust representation | Missing evidence |
|---|---:|---:|---:|---:|---:|---:|
| Remaining events | 17 | 17 | 67 runtime | 5 | 38 | 24 |
| Dispose/effects | 13 | 13 | 43 runtime | 0 | 29 | 14 |
| Types, carried unchanged | — | — | 6 type | 0 | 6 | 0 |
| **Total** | **30** | **30** | **116** | **5** | **73** | **38** |

The 95 original runtime IDs/spans, four previous empty-path IDs/spans and
eleven new split IDs give **110 runtime records**. Adding the six unchanged
accepted type records gives 116. All original declaration metadata, default
variants and source `expect` starts remain accounted for; no non-obligation
was added. Derived parent classes agree with child classes.

All 12 prior gap IDs are retained, with the first-yield follow-up extended;
26 new gap IDs give 38. Each has a concrete observation, the core evidence
maintainer as owner, and the existing conditional follow-up route. This is
unfinished evidence, not a count of missing features or production defects.
Related gaps can share one extension of an existing witness.

There are **28 distinct Rust test/Rustdoc identities and 56 passing
toolchain/item joins**. The four added runtime witness identities are the
repeated-emit, serial-prefix, snapshot-removal waterfall and immediate
composed-setup tests named above. The six type records remain byte-identical
to `ff414b5` and to the correction commit; their accepted runnable/compile-fail
distinctions are preserved. No compile-fail item supplies runtime counts or
results. There are zero executed-comparison or unsupported records in this
fragment. The separate TypeScript type file was not executed by the B0 matrix.

The corrected report accurately states these totals and boundaries. A passing
follow-up can close an evidence gap without changing production code. Only a
failing public-API reproducer plus an actual dependent gate admits B2; a gap
needed by a later slice blocks that slice until resolved. Final assembly,
coordinated validation and independent B0 acceptance remain the integration
owner's work.

## Actual bounded verification

No Cargo, guide/example, upstream-test or parity suite was run. The accepted
matrix was reused after executable-input applicability was checked. Its
execution span remains `578c066` through `e23dbac`; applicability control
`9a7633c` and this review HEAD are not claimed as execution HEADs.

The corrected report's first shell block was rerun verbatim from the review
checkout. It invokes only the shared obligation helper with the two preserved
workspace results/logs, not the complete-ledger validator:

```sh
python3 -B - <<'PY'
from pathlib import Path
import re, subprocess
p = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/report-events-effects-types.md')
block = re.search(r'```sh\n(.*?)\n```', p.read_text(), re.S).group(1)
r = subprocess.run(['zsh', '-c', block], text=True, capture_output=True)
print(r.stdout)
print(r.stderr)
print('scoped_report_validation_exit=' + str(r.returncode))
r.check_returncode()
PY
```

Exit **0**, with:

```text
events-rest 17 67 {'rust-representation': 38, 'missing-evidence': 24, 'source-mapped-test': 5}
dispose 13 43 {'missing-evidence': 14, 'rust-representation': 29}
types 6 {'rust-representation': 6}
declarations=30 variants=30 obligations=116 Rust_items=28 passing_toolchain_joins=56 gaps=38 errors=0
retained_original_runtime_obligations= 95
scoped_report_validation_exit=0
```

The output also lists all fifteen additions against the original inventory:
four previously reviewed empty paths plus eleven correction splits. An
additional independent standard-library check compared JSON IDs/spans and
Git blobs, rather than relying on the report's prose totals. It asserted:

- Exactly three correction paths changed from `ff414b5` to `6d94a17`, and
  every other B0 blob at that correction base remained unchanged. This
  comparison uses the correction commit, so unrelated later integrations are
  not confused with this worker's ownership.
- All three corrected working files equal both `6d94a17` and `1ac2986` blobs;
  all six type records equal `ff414b5`, `6d94a17` and `1ac2986` bytes.
- All 99 prior runtime IDs/spans survive; the eleven new records each retain
  their original sibling's assertion span and have missing-evidence/
  not-executed status. Original inventory runtime IDs total 95; empty paths
  total four; runtime records total 110; types total six.
- Classes total 5/73/38, the twelve prior gap IDs survive, all 38 gap records
  retain owners and conditional routing, and unique Rust identities total 28.

That check exited **0** and printed:

```text
original_runtime=95 prior_empty_paths=4 correction_splits=11 runtime=110 types=6 total=116
classes={'rust-representation': 73, 'missing-evidence': 38, 'source-mapped-test': 5}
retained_gap_ids=12 new_gap_ids=26 total_gap_ids=38; all owned and conditionally routed
correction_changed_files=3 all_other_B0_blobs_preserved=true integrated_input_bytes_identical=true six_types_unchanged=true
executable_applicability=passed TS_pin=35fdc969e4b6c48c48ed44717c7a018324e4dd88 TS_clean=true
```

The executable/pin checks inside it used checked subprocess exits and exact
expected output:

```sh
git diff --quiet 9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc 1ac2986 -- crates Cargo.toml Cargo.lock rust-toolchain.toml tests .github
git -C /Users/I756434/Projects/private/cordis-rust/local/cordis rev-parse HEAD
git -C /Users/I756434/Projects/private/cordis-rust/local/cordis status --porcelain
```

All exited **0**; the first and third were empty, and the second returned the
fixed pin. The correction report's links/anchors and all three corrected
files' direct whitespace checks also exited **0**. Structural checks establish
accounting/references; the scoped source assessment above supplies semantic
acceptance.

Final review-file checks:

```sh
PYTHONPATH=tests/docs python3 -B - <<'PY'
from pathlib import Path
import check
p = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/events-effects-types-rereview.md').resolve()
check.links(p)
lines = p.read_text().splitlines()
assert all(line == line.rstrip() for line in lines)
assert not any('\t' in line for line in lines)
print('review links/anchors and direct whitespace: passed')
PY
git diff --cached --check
```

Both exited **0**. The staged file list contained only this review report.
