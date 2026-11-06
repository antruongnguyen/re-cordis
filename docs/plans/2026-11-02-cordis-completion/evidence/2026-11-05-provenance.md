# Focused source provenance and notice review

Date: 2026-11-05 (Asia/Ho_Chi_Minh). Reviewed baseline:
`03d255029db6a57ccc17c0b1e26facdb491c058b` on `feat/cordis-completion`, after
accepted R1. Baseline tree: `0065323bf74aaebe233f0f023f969900953a4ce7`.

**Finding: the identified upstream notice omissions are corrected by scoped
root/core notices carrying the exact Shigma MIT text.** Repository/package
MIT notices from R1 are intact; they carry the repository's own copyright,
while the added notices cover source-derived logger portions and literal
upstream source excerpts. This focused review identifies concrete
material, rather than attributing the entire Rust implementation to either
reference project. It is not legal certification, a dependency license audit,
or a publication decision. The seven packages remain local/Git-checkout-only
`0.0.0`/`publish=false` under [R1](../../../release-readiness.md).

## Inputs and scope

The review read repository instructions, release readiness, decisions,
limitations and [R1 package evidence](r1-package-audit.md). It inspected current
notices, source-derived logger candidates, the bundled Create starter and its
history, parity fixtures/runners, synthetic GitHub fixtures, source-review
ledgers and the single CI workflow. CodeGraph was used before inspecting
indexed source, including the local references. The installed cosmokit source
is outside its index and was read directly after that result.

All reference Git checkouts had clean `git status --short` when inspected.
No checkout was fetched or changed. The existing README's historical Rust
reference pin remains valid historical evidence; its present checkout HEAD
is recorded separately.

| Input | Exact Git revision | Git tree |
|---|---|---|
| TypeScript behavioral specification, `local/cordis`, origin `https://github.com/cordiverse/cordis.git` | `35fdc969e4b6c48c48ed44717c7a018324e4dd88` | `d6c2f0d9fba954a58692f8b8236ce375f8e7752a` |

License/source hashes below are SHA-256 of exact file bytes:

| Input | Copyright text / identity | SHA-256 |
|---|---|---|
| Root LICENSE and all seven package LICENSE copies | `Copyright (c) 2026 re-cordis contributors` | `d74d38d5593aca56b840d0010cfe6728e7bb5e7f915bc6e3edf481747e5ee5ef` |
| TypeScript LICENSE | `Copyright (c) 2021-present Shigma` | `034fb52b1d57360ecbae6cb1632a88f86fd7c3d3f5631a5f082710203dda0be7` |
| Installed cosmokit `package.json` | Declares version `1.8.1`, MIT, repository `https://github.com/shigma/cosmokit` | `0a0c4b1ad727fef5ff734b2f824d6476660a0c48ad9fa91a8d5b717b0b85024d` |
| Installed cosmokit `src/string.ts` | Local bytes used for tokenizer comparison | `ec9fce9f28bc9892736803b3080418ebc9b0724d409de04f086f1bfebe2bd9ec` |
| Installed cosmokit LICENSE | Exact same bytes as TypeScript LICENSE | `034fb52b1d57360ecbae6cb1632a88f86fd7c3d3f5631a5f082710203dda0be7` |

The installed package and Yarn state identify `cosmokit@npm:1.8.1`; this review
does not independently authenticate its registry tarball or assign a Git pin
to installed npm files. Recorded local bytes make that evidence boundary
explicit. No cosmokit Cargo dependency is introduced.

## Concrete source-derived material

### Core logger palettes and channel-color selection

At the baseline, `crates/re-cordis-core/src/logger.rs:164–175` retains both
Cordis palettes: six ANSI16 values and the exact ordered 75-value ANSI256
selection in the pinned `packages/core/src/logger.ts:147–154`. Compact JSON of
the 75-value sequence hashes to
`9637cf187a1f019855d96865bf45d950a0e0e6b1bb75a529b98529a36f91b871`.
The Rust UTF-16 fold is the explicit signed-wrapping form of the upstream
`Logger.code` hash at lines 70–77: multiply the prior hash by seven and add
the code unit plus 13 before selecting from the palette. The escape rendering
uses Rust formatting and explicit color-depth choices.

Commit `a145288250ea228638fd888a0b7a99d81f44a893` introduces the palette/hash
implementation at its former `crates/cordis-core/src/logger.rs` path. The
rename commit `7f37bbed0570a368b015478519df6515c62a9a8f` carries it unchanged
to the present path. [M6](../../../m6.md#logger-contract) explicitly records
the pinned palettes and UTF-16 hash. These concrete selections justify
preserving the upstream notice; the finding does not depend on a blanket
claim that all matching framework concepts are copied code.

### Core logger default-name tokenization

Baseline `logger.rs:185–213` follows cosmokit 1.8.1 `src/string.ts:19–64`
(`tokenize`, `paramCase` and the `hyphenate` alias). Both use the ASCII
upper/lower/delimiter state transitions, look ahead for an acronym boundary,
emit one normalized delimiter, and preserve other characters without changing
the letter state. Rust uses a peekable character iterator and two booleans;
cosmokit uses UTF-16 code units and a State enum. This is a recognizable
source-informed adaptation, with its origin explicitly recorded by the same
M6 commit and the [decision](../../../decision.md#2026-10-23--m6-completes-explicit-rust-logging-and-acceptance).
The local cosmokit notice has the same Shigma copyright and MIT permission
bytes as Cordis, so one scoped copy can identify both origins.

### Literal upstream source excerpts in documentation evidence

B0 `ledger/cases.json` contains literal assertion expressions tied to exact
source paths/lines. For example, `core/associate.spec.ts:5/default/a23` retains
`expect(root.foo).to.be.instanceof(Foo)` from pinned
`packages/core/tests/associate.spec.ts:23`. The initial checkpoint commit
`c3fbcb0a485603a55de97adb06a0f2e42c1fea7b` already stores that expression;
later B0 assembly preserves the record alongside Rust evidence.

A mechanical comparison of all 540 behavior records found 214 containing
their cited TS assertion text verbatim, including 136 equal to it outright.
Those 214 records contain 135 distinct cited texts totaling 6,744 characters.
The counts only describe an identifiable quotation subset: paraphrases,
parameter substitutions, declaration titles, other ledger fields and review
reports are not counted as if this were a copyright threshold or complete
copy inventory. Fragments such as `B0/ledger-inputs/logger.json` also retain
literal assertions, while source-case inventories and reports identify their
upstream origins. A root notice can cover those literal source excerpts
without rewriting any accepted historical report or ledger bytes.

No B0 behavioral classification, execution verdict or count is revised here.
Its 147 missing-evidence items remain accepted bounded evidence; they are
neither notice defects nor an implementation backlog created by this review.

## Package/template evidence and notice correction

Read-only inspection of the carried R1 stable and 1.85.0 core archives found
48 members each, including the source-derived `src/logger.rs` (SHA-256
`df4f078c418794c803dc1d1f445e0f9748c69b182949d50b436f8e2594cf4f5a`), the
repository LICENSE, and no upstream notice. The archives are historical inputs:
stable SHA-256 `3c666f2ad6d6312c2542c4529157a8388d5db1bddcd3dab23c1c3db2be9b909d`,
1.85.0 SHA-256 `9bc383a09151acf19c7ebd86982c92747eeda3e9d37b1d04b10c10019d9d6280`.
No Cargo command or new package build was run by this reviewer.

The parent applied the narrow correction: root
[UPSTREAM-NOTICES.md](../../../../UPSTREAM-NOTICES.md) identifies the
logger/Cordis/cosmokit portions and literal source excerpts; core-package
[UPSTREAM-NOTICES.md](../../../../crates/re-cordis-core/UPSTREAM-NOTICES.md)
identifies the two logger origins and carries the exact original MIT notice.
Both actual files match the proposed bytes below and end with the original
upstream LICENSE verbatim. The repository LICENSE, seven package copies,
template appendix and historical source-review documents remain intact. The
core README links its bundled notice. Root README, readiness, decisions and
limitations describe the precise scope without attributing unrelated Rust
code to reference authors. Those wording changes agree with this review's
findings and limits. Final new-archive and shared-matrix verification belongs
to the parent's follow-up evidence; the carried R1 archive observations above
are not new-archive results.

Exact proposed bytes are saved under ignored `target/b1-provenance/`:
`proposed-root-UPSTREAM-NOTICES.md` (SHA-256
`22ae31fa799b9dcb90fdeff423f911c0299aa51641482b83c6c5ebaa8228830e`) and
`proposed-core-UPSTREAM-NOTICES.md` (SHA-256
`f9a7afd6684a9ae0e2792b5f1616a76fffc88096c095eb5eda785483bad98297`).
Both reproduce `Copyright (c) 2021-present Shigma`, the full permission/condition
paragraphs and the original disclaimer; the applicability prefaces differ.

The Create basic README's final suffix still equals the repository LICENSE
exactly. The generator/template implementation originates at
`d44620c9c28acfa5fd2e4b93c761af08126580f4`; R1 commit
`b45a11cc2e9b95d64c213f47c2961ce50d6f15ad` adds the notice appendix. No upstream generator/template copy was identified in the inspected
material. Its five-file layout and offline checkout contract remain unchanged.
Refer to [R1 package evidence](r1-package-audit.md) for actual generated-app
execution, which this source review does not rerun or replace.

## Candidate search limits and other inspected material

The saved stdlib script reads baseline Git objects and compares three
consecutive nonblank, indentation-stripped lines with at least 180 total
characters. The repository/TypeScript inputs comprise 439 repository text
files and 150 pinned TypeScript files. Its matches beyond the
manually examined candidates consist of standard MIT wording. This is a
literal candidate finder: changed syntax, short snippets, translated prose
and algorithmic derivation can evade it, as the manually identified palette
and tokenizer demonstrate. File counts are scan inputs, not reviewed legal
obligations. Exact hashes and paths are in `inputs.json`; candidate records,
Git command argument arrays and exits are retained alongside the script.

The parity TS fixtures are repository-authored scenario/trace harnesses added
by the lifecycle, timer-repair and loader commits; they exercise pinned
upstream code in temporary fixtures. The loader fixture imports upstream
`MockLoader` from its original checkout rather than vendoring its class.
The inspected GitHub JSON fixtures are explicitly synthetic in their source
documentation. No tracked vendor directory, raster/vector media, binary
library or archive was found by the suffix/path inventory. This says nothing
about third-party files in ignored build/cache directories.

## CI observation and distribution-specific gates

The unchanged `.github/workflows/ci.yml` defines Ubuntu `stable` and `1.85.0`
jobs: stable-only workspace/M0 formatting and Clippy, workspace target checks,
workspace tests/Rustdoc, and separate M0 tests. It has no Python documentation
regression step or bilingual guide/snippet execution. Recommend a separately
authorized CI change running `python3 -B tests/docs/test_check.py` and the
documentation checker on each explicit toolchain after deliberate dependency
restoration for its offline Cargo applications. Current workstation cache
availability must not be assumed on a fresh hosted runner. The proposal
does not implement CI or convert Ubuntu configuration into executed platform
acceptance; [documentation maintenance](../../../README.md#maintaining-these-documents)
records the current local workflow.

For a later distribution decision, verify the actual final clean source and
core archives retain the upstream notice, including any intentionally trimmed
source distribution. Binary/vendor distributions additionally need a notice
inventory for the exact resolved third-party code they contain; registry
license metadata alone is insufficient evidence. Generated starter source
must retain its template notice; distributing its compiled dependencies adds
the relevant dependency notices. Publication still needs the R1 version,
internal dependency, package-content, ownership and compatibility gates.
This review supplies no registry readiness, online repository availability,
new-platform promise, whole-program parity or full provenance closure.
