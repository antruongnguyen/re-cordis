# Cordis completion execution evidence

Execution began 2026-11-02 on local branch feat/cordis-completion in
/Users/I756434/Projects/private/cordis-rust. See [canonical progress](progress.md)
for all 35 package states, ownership, decisions, blockers and exact resume steps.

## B0 baseline (partial package; resumed for acceptance)

- [Accepted execution plan](evidence/B0/plan.md).
- [Independent matrix review](evidence/B0/matrix-review.md): command portion accepted at 7d1552a. Executed inputs at 578c066 through e23dbac have identical hashes and no executable delta from implementation baseline 808c9f5.
- [Command index](evidence/B0/matrix/commands.tsv), [actual results](evidence/B0/matrix/results.json), and full committed logs under evidence/B0/matrix/logs/.
- [Matrix report](evidence/B0/matrix/report.md) at 25dcde8 records all results, failed evidence-assembly attempts, successful validation and known metadata limits. [Scoped re-review](evidence/B0/matrix-rereview.md), integrated at b87c1b9, closed B0-MATRIX-01/02 and verified original artifact preservation.
- Stable 1.97.1 and 1.85.0 each passed 164 core integration tests (including 12 Unix application tests), 15 timer tests, 17 core Rustdoc checks, 20 M0 tests and one M0 Rustdoc check. Both examples and guide checks (8 snippets plus external quick start per toolchain) passed; five Python regressions passed.
- The fixed TypeScript specification is 35fdc969e4b6c48c48ed44717c7a018324e4dd88. Exactly six paired scenarios and 73 observations passed. These counts do not establish full upstream parity or non-macOS platform acceptance.
- [Core inventory](evidence/B0/ledger/cases.json): the original 144-declaration / 156-variant inventory remains unmerged. Completed family drafts now cover all declarations and six type obligations, but this is not an accepted final case ledger.
- [Decorator/reflection fragment](evidence/B0/ledger-inputs/report-decorator-reflect.md): 5 declarations / 23 obligations mapped to source tests, deliberate Rust representations, existing exclusions or named evidence gaps. FR-1 was corrected in 59be2db and [independently accepted](evidence/B0/fragment-rereview.md) in b28a608. All seven evidence gaps remain open; their regressions and dependency triage precede conditional B2. No other semantic classification changed.
- [Interrupted events inventory](evidence/B0/ledger-inputs/events.json): 19 declarations / 61 initial obligations; audit did not finish. Initial missing-evidence fields are unreviewed placeholders, not accepted dispositions.
- [First two events](evidence/B0/ledger-inputs/report-events-first-two.md) now have five tested Rust-representation obligations and zero gaps or paired claims. [Independent review](evidence/B0/events-first-two-review.md) accepted the fragment; implementation/review integrated at `0c934ed` / `11d84b1`. Together with decorator/reflect, seven declarations / 28 obligations have accepted dispositions.
- [Worker infrastructure failures](evidence/B0/infrastructure.md) recurred in native and CLI gpt-6.1-sol attempts. The user subsequently authorized gpt-5.6-sol/high, which completed the remaining source drafts. This is an authorized fallback, not proof that the original model transport is repaired.
- The earlier integrated recovery validation failed HTTP 400 before saving a report. A fresh executor at `f2ff4c3` has now saved [integrated validation](evidence/B0/resume-validation.md), including passing bounded checks and an actual raw-byte failure caused only by commands.tsv working-copy line endings; all 65 committed original artifacts are unchanged. That worker then disconnected before its final handoff. The completed [scoped independent review](evidence/B0/resume-validation-review.md), at review HEAD `fb0c194`, accepts the integrated-check gate with this preservation boundary. The original failing result remains recorded; no rerun or artifact normalization is required. This does not establish reliable source-audit transport or supersede accepted matrix/FR-1 reviews.

- [Scoped/logging drafts](evidence/B0/ledger-inputs/report-scoped-logging.md): 40 declarations / 136 raw obligations, integrated through b453ac6. [Independent review](evidence/B0/scoped-logging-review.md), integrated at 4407379, requires SL-1–SL-6 corrections. Submitted counts are not accepted semantic pass totals.
- [Lifecycle/update drafts](evidence/B0/ledger-inputs/report-lifecycle-update.md): 42 declarations / 44 variants / 188 raw records, integrated through 4ca5d39. [Independent review](evidence/B0/lifecycle-update-review.md), integrated at 64b7c54, requires LR1-LR5 corrections, including explicit non-obligation metadata for three inventory artifacts.
- [Events/effects/types drafts](evidence/B0/report-events-effects-types.md): 30 declarations / 30 variants, 99 runtime records and six type records, integrated through bdc73a4. The exact Rustdoc-metadata correction resolved 20 mechanical join failures; all 105 records pass shared partial schema/reference checks. Initial semantic review remains pending.
- [Services drafts](evidence/B0/ledger-inputs/report-services.md): 25 declarations / 35 variants / 81 records, integrated at 7edb4f9. Initial semantic review must also assess four proposed guarded-variant omissions.

The user resumed B0 after the committed pause checkpoint at 7fd7cde. Fresh
gpt-6.1-sol/high correction workers and gpt-6-astra/high independent reviewers
are authorized; the failed CLI path remains prohibited. All draft
fragments are preserved: 144 declarations / 156 variants, 532 raw runtime
records and six types. Transfer checks confirmed exact worker-commit bytes;
all 538 records pass partial schema/reference/log checks. This does not resolve
semantic review findings, the non-obligation accounting or the complete-ledger
gate. Only the prior seven declarations / 28 obligations are independently
accepted.

Resume with [review corrections](evidence/B0/handoffs/review-corrections.md),
the two pending initial reviews, then coordinated inventory/control repair,
ledger assembly/validation and final independent review. No final-ledger
passing result or B0-wide acceptance exists yet.
No production code, runtime dependencies, public APIs, source pins or existing
compatibility contracts changed. T0 and all later packages remain unstarted;
no release-readiness proposal exists yet. R1 remains a local proposal-only gate.

