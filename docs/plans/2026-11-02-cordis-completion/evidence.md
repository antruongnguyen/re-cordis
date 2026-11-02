# Cordis completion execution evidence

Execution began 2026-11-02 on local branch feat/cordis-completion in
/Users/I756434/Projects/private/cordis-rust. See [canonical progress](progress.md)
for all 35 package states, ownership, decisions, blockers and exact resume steps.

## B0 baseline (partial package; blocked)

- [Accepted execution plan](evidence/B0/plan.md).
- [Independent matrix review](evidence/B0/matrix-review.md): command portion accepted at 7d1552a. Executed inputs at 578c066 through e23dbac have identical hashes and no executable delta from implementation baseline 808c9f5.
- [Command index](evidence/B0/matrix/commands.tsv), [actual results](evidence/B0/matrix/results.json), and full committed logs under evidence/B0/matrix/logs/.
- Stable 1.97.1 and 1.85.0 each passed 164 core integration tests (including 12 Unix application tests), 15 timer tests, 17 core Rustdoc checks, 20 M0 tests and one M0 Rustdoc check. Both examples and guide checks (8 snippets plus external quick start per toolchain) passed; five Python regressions passed.
- The fixed TypeScript specification is 35fdc969e4b6c48c48ed44717c7a018324e4dd88. Exactly six paired scenarios and 73 observations passed. These counts do not establish full upstream parity or non-macOS platform acceptance.
- [Core inventory](evidence/B0/ledger/cases.json): 144 declarations / 156 expanded cases; mapping incomplete. It is not an accepted case ledger.
- [Decorator/reflection fragment](evidence/B0/ledger-inputs/report-decorator-reflect.md): 5 declarations / 23 obligations mapped to source tests, deliberate Rust representations, existing exclusions or named evidence gaps. Independent review is a separate gate.
- [Interrupted events inventory](evidence/B0/ledger-inputs/events.json): 19 declarations / 61 initial obligations; audit did not finish. Initial missing-evidence fields are unreviewed placeholders, not accepted dispositions.
- [Worker infrastructure failures](evidence/B0/infrastructure.md) prevent continued reliable execution with the requested worker settings. No model substitution was made.

The matrix acceptance is one part of B0. The remaining core/type mappings,
case-ledger validation and independent review must finish before B0 can be done.
No production code, runtime dependencies, public APIs, source pins or existing
compatibility contracts changed. T0 and all later packages remain unstarted;
no release-readiness proposal exists yet. R1 remains a local proposal-only gate.

