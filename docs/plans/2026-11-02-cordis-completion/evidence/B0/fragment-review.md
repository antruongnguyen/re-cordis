# Independent decorator/reflect fragment review

Date: 2026-11-02. Verdict: **changes-required**, limited to the scheduling
metadata finding below. The five declarations and 23 semantic dispositions
are otherwise accepted as bounded evidence. No additional tests or production
changes are required to correct this fragment.

Reviewed the [JSON fragment](ledger-inputs/decorator-reflect.json) and
[fragment report](ledger-inputs/report-decorator-reflect.md) at stable commit
`ac72512`. Both files were unchanged at review checkout
`fb162cd5e6851936cef569a25fc51ce35baf0305`. TypeScript was at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88` with no tracked changes.
The implementation remains `808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f`;
the ledger inventory identifies Rust revision
`9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc`.

This is the narrow review requested by the [handoff](handoffs/fragment-review.md),
not whole-package acceptance. B0 remains incomplete and blocked by the recorded
worker API failures. The accepted matrix at `7d1552a` was not rerun.

## Required correction

**FR-1 / P2 — Keep evidence follow-up separate from conditional B2 entry.**
All seven gaps set `work_package` to
`B2 (targeted evidence regression before dependent satellite gate)`
(JSON lines 80, 137, 161, 638, 658, 678 and 698); the report repeats this
unqualified routing at line 23. Each gap is correctly classified as missing
evidence, and the report explicitly says these are not confirmed production
failures. However, [core B2](../../core.md#b2--conditional-core-bridge-or-targeted-regression-fix)
requires a failing public-API reproducer supplied by T1/T2 or a satellite gate
before entry. The fragment identifies neither such a failure nor a particular
dependent gate. Assigning the assertion additions directly to B2 leaves their
next action behind a prerequisite that these entries do not establish.

Correction owner: ledger fragment owner / core evidence maintainer. Preserve
the seven gap IDs and concrete assertion actions, but route them to core
evidence follow-up and dependency triage. State explicitly that the relevant
satellite gate must first establish that it depends on a gap and run the
targeted regression; only a failing public-API reproducer opens B2. A passing
regression closes the evidence gap without opening a production-fix package.
Update the report to match. This is a metadata/report correction; it does not
require adding tests in B0, broadening B2, or choosing a hypothetical satellite
dependency now.

Scheduling consequence: keep B2 conditional and dependent implementation
closed while B0 is incomplete. Once B0 accounting is complete, these gaps
block only a slice shown to depend on them. Re-review the corrected routing
before treating this fragment as accepted for ledger integration.

## Semantic review

Every assertion in the two pinned TS files was inspected, together with the
relevant `Inject` method branch, `Context.is`/extension implementation, and
`ReflectService` read/set/provide/mixin paths. CodeGraph MCP and its `node` CLI
provided the current Rust test bodies and relevant production branches.

| Source obligation | Review result |
|---|---|
| Decorator lines 31, 34, 38 | The explicit `inject_method` child is the documented lifecycle substitute. `acceptance.rs:269–276` asserts zero callbacks before arrival, one callback after arrival, and one cleanup after first withdrawal. `Method::run` registers cleanup explicitly instead of returning the TS disposer. |
| Decorator lines 32, 35, 37 | Correctly missing evidence: no intermediate zero-cleanup assertions and no callback-count assertion immediately after first withdrawal. The final aggregate equality does not close those boundaries. |
| Reflect line 9 | Correctly unsupported under the existing M6 subclass-brand restriction. `acceptance.rs:703–705` tests concrete `Any` detection only; its passing result is not attributed to the subclass assertion. |
| Reflect lines 16, 17 | Correctly unsupported under the existing transparent-reflection restriction. TS special `prototype` access and inherited `constructor` access bypass the injection path at `reflect.ts:95–100`. |
| Reflect lines 18, 19, 23–26 | The typed checked-access, missing-provision, provision-admission, duplicate-slot and owner-set mappings are supported by the cited services tests. Typed error variants and setup differences are disclosed rather than treated as exact JS exception or trace equivalence. |
| Reflect lines 39, 41 | Published root lookup and accessor/service-namespace separation are asserted at `acceptance.rs:542–554`. `Accessor::new` is used in this particular fixture; the report correctly describes it as an explicit accessor substitute, not transparent mixin property access. |
| Reflect lines 43, 47, 51, 52 | Correctly missing evidence: metadata versus service namespace, the reset warning baseline, metadata in a plugin created from an extended injecting Context, and warning silence after that read. Generic metadata layers and the separate bound-service metadata fixture do not cover this combined child path. |
| Reflect lines 62, 64 | Active required access and retained-context refusal after joined disposal are directly supported; the live provider remains readable through the explicit direct path. |

The setup differences in the access mappings are acceptable for the stated
source-mapped class. The TS unknown-service read and Rust sibling-publication
read both fail for absent declaring ancestry. The missing-publication set
test is at Rust root; `set_value` checks publication existence before owner
identity (`service.rs:598–615`). The duplicate test uses another Context;
both TS `reflect.ts:214–215` and Rust `service.rs:466–476` reject an occupied
slot without an exception for the same owner. These are bounded mappings of
those checks, not execution of the exact TS plugin bodies or messages.

The exclusions are grounded in existing
[M6 restrictions](../../../../m6.md#remaining-restrictions-and-safety-deviations)
and its [source-map audit](../../../../m6.md#source-map-audit). The method and
accessor substitutes agree with
[M6 method injection](../../../../m6.md#initialization-and-method-injection)
and [M5 reflection](../../../../m5.md#explicit-reflection). No new unsupported
behavior is introduced or counted as parity.

## Accounting and execution-reference checks

| Count | Verified result |
|---|---:|
| Declarations / default variants | 5 / 5 |
| Unique assertion obligations | 23 |
| Source-mapped / Rust representation | 9 / 4 |
| Unsupported / missing evidence | 3 / 7 |
| Executed comparisons | 0 |
| Distinct cited Rust tests | 5 |

Four tests substantiate the mapped behavior and substitutes. The fifth is
the related concrete-Any test attached to the excluded subclass assertion.
All five were independently joined by command ID, crate, suite and test name
to successful rows in [matrix results](matrix/results.json), and to the actual
`test ... ok` lines in the [stable](matrix/logs/stable-workspace-test.log) and
[1.85.0](matrix/logs/1.85.0-workspace-test.log) logs. Both recorded workspace
commands exited zero with explicit matching Cargo/RUSTC/RUSTDOC paths and
separate target directories. These remain the saved runs, not new executions
at the fragment revision.

Fresh read-only checks also confirmed:

- The fragment's five declaration identities, titles, suites, line bounds,
  historical-checklist fields, default variants and 23 assertion IDs/lines
  exactly match the five records in the inventory at `ac72512`.
- Derived classes and all required evidence/restriction/gap fields are valid;
  cited Rust symbols and line bounds exist, and restriction anchors resolve.
  All paired-evidence lists are empty.
- All 71 executable-input hashes in matrix before/after/supplement snapshots
  match one another and the current files; executable changes since
  `808c9f5` are absent. Snapshot TS pins agree with the clean current checkout.
- The report's relative links and headings resolve using the existing
  `tests/docs/check.py` link helper; the same helper checks this review file.

The review ran source/accounting/hash/log/link checks only. It did not run
Cargo, the documentation examples, the parity runner, or the incomplete full
ledger validator. Only this review file was written; no implementation,
fragment, progress, manifest, other evidence or commit was changed.
