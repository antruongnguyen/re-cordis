# Independent FR-1 correction re-review

Date: 2026-11-02. Reviewed integration revision:
`59be2db95333f6fa9440dec3108416fe8634ce89`, in a managed isolated worktree
created at that revision. Compared the two decorator/reflect fragment files
with baseline `25329ad995ba21b326e116e801f7f57816d4ed0b`; their corrected
contents also exactly match worker commit `91948f1`.

**Spec verdict: accepted. Quality verdict: accepted. FR-1: addressed.**
No new material breakage was found in this correction. There are no remaining
correction findings. This accepts the corrected fragment for ledger integration;
it does not close its seven evidence gaps or make a B0-wide acceptance claim.

## Correction assessment

The [original review](fragment-review.md) required evidence follow-up to be
separate from conditional B2 entry. All seven `gap.work_package` values now
route the work to core evidence follow-up and dependency triage. The core
evidence maintainer writes and runs each named regression. A passing result
closes the missing evidence without production change. The relevant satellite
gate must establish that its slice depends on the gap and supply a failing
public-API reproducer before B2 can open. This satisfies the
[B2 entry condition](../../core.md#b2--conditional-core-bridge-or-targeted-regression-fix)
for these records without assuming a satellite dependency.

The [corrected report](ledger-inputs/report-decorator-reflect.md) states the same
passing and failing paths. It keeps B0 accounting separate from implementing
the regressions, dependent implementation closed while B0 is incomplete, and
later blocking limited to slices shown to depend on a gap. No evidence action
is assigned unconditionally to B2.

The following gap IDs are unchanged, together with every original next action,
owner (`core evidence maintainer`), kind/class (`missing-evidence`), and
execution status (`not-executed`):

- `G-B0-decorator.spec.ts:7/default/a32`
- `G-B0-decorator.spec.ts:7/default/a35`
- `G-B0-decorator.spec.ts:7/default/a37`
- `G-B0-reflect.spec.ts:30/default/a43`
- `G-B0-reflect.spec.ts:30/default/a47`
- `G-B0-reflect.spec.ts:30/default/a51`
- `G-B0-reflect.spec.ts:30/default/a52`

Exactly seven JSON scalar values changed, each a `gap.work_package` value.
All other declaration, variant, obligation, evidence, restriction, source-map,
matrix-reference and rationale data are identical to the accepted baseline.
The report retains all previous content apart from replacing its routing
paragraph and appending the correction and validation record. This review
does not repeat the already accepted semantic mapping audit.

## Actual read-only checks

The final bounded checks below passed; no Cargo, documentation-example, parity
or baseline-matrix commands were run.

| Check | Result |
|---|---|
| `git diff 25329ad 59be2db --` the two fragment paths | Inspected all seven scheduling replacements and the complete report change. |
| `python3 -B -` recursive comparison of both JSON revisions | Exactly seven changed leaves, all `gap.work_package`; all other values identical. Verified the exact seven IDs, unchanged owner/class/kind/status, and identical conditional routing on every gap. Exit 0. |
| Same Python accounting check | Five declarations, five variants, 23 obligations: 9 source-mapped, 4 Rust representations, 3 unsupported, 7 missing evidence, 0 executed comparisons. The routing references only known work-package ID B2 against the 35 schedule IDs. |
| Same Python report comparison | Replacing the old routing paragraph and removing the appended correction section reconstructs the baseline report exactly, apart from trailing whitespace. |
| Git object and scope checks through Python | Both reviewed files were unchanged from `25329ad` to the integration correction's parent, both equal worker `91948f1`, and `59be2db` changes only those two files. The worker's parent is exactly `25329ad`. Exit 0. |
| `tests/docs/check.py::links`, imported by `python3 -B -` | Corrected report and this re-review have resolving local links and heading anchors. Exit 0. |
| `git diff --check 25329ad 59be2db` | Exit 0, no output. |
| Initial `git status --porcelain` in the review worktree | Empty before this review file was created. |

The integration history contains three unrelated documentation paths changed
between `25329ad` and `59be2db`. The correction commit itself contains only the
two reviewed files. The report's historical two-file validation was run on the
worker's direct `25329ad` lineage; it is not a claim about that wider integration
range. An initial reviewer guard against the wider range was narrowed to the
correction commit after confirming this history; an earlier stdin quoting
error was corrected before the successful structural run. Neither failure
indicated a fragment defect.

Only this re-review file was written. Production, fragments, progress, ledger,
matrix evidence and manifests were not changed; the accepted matrix remains
the saved execution evidence.
