# R1 scoped documentation re-review

Date: 2026-11-05 (Asia/Ho_Chi_Minh).
Audit/package-notice revision: `b45a11cc2e9b95d64c213f47c2961ce50d6f15ad`.
Reviewed correction: `82d8f70e939e7886db835fcfb55e8d630c745e87`.
Branch: `feat/cordis-completion`.

**Verdict: R1-R1 and R1-R2 are addressed. No new actionable finding in the
scoped correction. Final local R1 assessment acceptance may proceed.**
This approves closure of the assessment and proposal, not publication or the
future version/dependency changes.

## Finding dispositions

| Finding from the [original review](r1-review.md) | Disposition | Verified correction |
|---|---|---|
| R1-R1 — P2: CHANGELOG implies the proposal was approved | ADDRESSED | The entry now calls it a concrete proposal requiring a separate release decision. This agrees with the report's local/Git-checkout-only current recommendation and future seven-package 0.1.0-alpha.1 proposal. |
| R1-R2 — P2: demonstrated Create archive-test limitation omitted | ADDRESSED | The primary report records five passing library tests, 11 of 17 passing scaffold tests and six failures on stable, their repository-relative fixture cause, and the outstanding release test-scope/fixture decision. Acceptance preserves that result and states that the MSRV archive suite was not run. |

The [readiness report](../../../release-readiness.md) distinguishes successful
archive verification builds from the later failed archive tests. The retained
`stable-create-archive-tests-member` output and exit 101 confirm the counts and
missing `stable/crates/re-cordis-core/Cargo.toml` cause. The earlier nested-workspace
discovery errors are separate review setup failures; the ignored parent workspace
admits the unchanged extracted package and does not repair its checkout fixture.
Neither the setup failures nor the six test failures are converted into passes.

The separate successful workflow is accurately bounded: the archive-built CLI
receives the real checkout through `--cordis-path`. Retained commands and logs
show generation, application build, five tests including 100 lifecycle cycles,
run and strict all-target Clippy passing on stable and 1.85.0. The recorded output
is five files with the MIT notice in README; the four other generated files match
the initial output. Fresh byte comparisons confirm all 14 files in each extracted
Create package still match its archive. The stable archive SHA-256 remains
`5413dafe6cd29b2da8c6e137ccd8d982a182e0f221de9fbaf6345933ac0d0bd8`.
No MSRV archive-suite execution is inferred from these CLI/application results.

An explicit future choice between a checkout-aware archive fixture and documented
checkout-only tests with packaged-CLI distribution coverage closes the reporting
finding without pretending the existing archive suite passes. The report retains
that choice as a publication gate; it does not drop tests or broaden R1 into a
generator/test redesign.

## Fresh scoped checks and evidence applicability

The exact audit-to-correction diff contains five Markdown files: CHANGELOG,
readiness, acceptance, progress and the original review. There is no Rust source,
test/checker, package notice/template, manifest, lockfile or feature change in
this correction. All seven manifests retain version 0.0.0 and publish=false.

Fresh hashes against R0's retained 166-input set confirm 164 unchanged inputs;
only the previously documented Create crate and basic template READMEs differ.
The original R1 audit adds six package READMEs and seven LICENSE files outside
that fingerprint set. R0's 40-stage/517-workspace-check result per toolchain
therefore remains explicitly **carried evidence**, not a fresh re-review run.

The retained audit contains 107 commands: 71 successful and 36 expected
dependency/publication refusals. The inspected controller records still show
eight documentation regressions and successful corrected stable/1.85.0 checker
runs with 34 paired pages, 13 snippets, five-test basic applications and
eight-test generated R0 applications. These are prior R1 executions; the scoped
re-review does not repeat or add them to unique test totals.

Fresh assertion, manifest-policy, archive-byte, fingerprint, Markdown link/anchor
and fence checks pass. The link sweep covers the five changed pages, the root
README, todo, master plan, shared acceptance and this report. Exact revision
`git diff --check` and the report's whitespace check pass. Supporting local
artifacts are ignored `target/r1-rereview/verify.py` and `checks.json`.
Reproduce the scoped snapshot check at the reviewed correction using:

```sh
python3 -B target/r1-rereview/verify.py
git diff --check b45a11cc2e9b95d64c213f47c2961ce50d6f15ad 82d8f70e939e7886db835fcfb55e8d630c745e87
```

No fresh Cargo, full documentation checker, shared matrix or platform run is
claimed here. Existing executed evidence remains applicable to this prose-only
correction; no concrete mismatch calls for a broader rerun.

## Acceptance handoff

At reviewed revision 82d8f70, progress is `in_review`, todo leaves R1 acceptance
unchecked, and [R1 acceptance](r1-acceptance.md) awaits this scoped re-review.
Root/master wording likewise describes the proposal without premature acceptance.
These states are consistent with the review gate that was still open.

The controller may now mark R1 done/accepted, identify both exact revisions and
both review reports, and record final documentation/status verification in the
root, progress, todo, master/shared acceptance and R1 acceptance pages. That is
routine closure bookkeeping, not an additional finding. The current consumption
verdict stays local/Git checkout only; publication, version changes, ownership,
registry prerequisites and remaining distribution decisions stay separate.

This reviewer writes only this report and ignored `target/r1-rereview/` artifacts.
No code/package change, commit, push, tag, publication or subagent dispatch occurs.
