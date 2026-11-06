# D4 experimental alpha candidate

Date: 2026-11-06 (Asia/Ho_Chi_Minh).
Status: **D4 prepared and independently reviewed; awaiting owner publication approval**.

## Frozen release source

- Candidate: `84fa9bdf0eb4e631fce792bf163494b0a61aeba8`.
- Branch: `feat/alpha-release`; [review PR #5](https://github.com/antruongnguyen/re-cordis/pull/5).
- Version: `0.1.0-alpha.1` for all seven packages.
- Intended publisher: [antruongnguyen](https://crates.io/users/antruongnguyen), public user ID `447261`.
- Compatible complete checkout for offline Create: the same candidate SHA.
- Proposed tag: `v0.1.0-alpha.1`, targeting the frozen release source after approval.
- [Approved preparation contract](release-decision.md) and [draft release notes](release-notes-draft.md).

The source SHA above fixes the package inputs, tested archives and candidate
website. Later plan/roadmap/evidence commits record the results without changing
those inputs. Do not substitute a later branch HEAD for this SHA when approving
or publishing the candidate; reverify any changed source explicitly.

The candidate is committed and available for review. It is not a crates.io
release, a hosted docs.rs release, or a replacement of the live Pages snapshot.
Publication credentials and actual name ownership must be checked in D5.

## Prepared scope

All seven manifests inherit the coordinated version and restrict publishing to
`["crates-io"]`. All ten internal normal/optional/dev path edges also require
exact `=0.1.0-alpha.1`, including HMR's optional Include and dev-only logger.
The lockfile changes only seven workspace package versions; all 67 registry
package records, checksums and dependency resolution remain unchanged.

Rust 2024/resolver 3, MSRV 1.85, runtime/API source, feature defaults and all
applicable notices remain intact. HMR defaults remain empty. The entire basic
Create template remains byte-identical, including its application's separate
`0.0.0`/`publish=false` policy and explicit local paths.
No facade, template variant, runtime redesign or platform expansion is selected.

The website derives its version from Cargo.toml at build time through native
Bun TOML support and existing generated settings. It retains an explicit
unpublished candidate notice. Current guides and package READMEs distinguish
planned release references from verified availability; historical evidence
keeps its original versions and results.

## Shared acceptance and fixture correction

The existing 40-stage matrix passed at
`b6a7eacd0df7ab23ff0dfbbc74a79757f2ac3567`, on explicit stable 1.97.1 and
Rust 1.85.0 Apple Silicon Darwin tools. It recorded 180 unchanged inputs and
the clean TypeScript pin `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.

Results include 517 workspace checks per toolchain, M0, strict Clippy, formatting,
Rustdoc, runnable examples/real watching, nine docs regressions, real generated
basic/R0 applications, six core parity scenarios/73 observations and two loader
scenarios/nine observations per toolchain.

The later [package-fixture correction](core-archive-fixtures.md) adds twelve
byte-identical package-local traces and changes three consumers' include paths.
It preserves all root fixture paths and data. Focused core/loader/timer checks
on both toolchains emitted the same twelve expected traces; actual core archive
tests passed. Independent review approved carrying the unchanged-behavior matrix
with that explicitly covered path/input delta. This is not a claim that the
entire 40-stage matrix ran at `84fa9bd`.

## Clean integrated package and consumer checks

The controller ran the following at clean candidate `84fa9bd`:

| Check | Result |
|---|---|
| Locked offline Cargo metadata, both toolchains | Seven versions/publish policies and ten exact internal edges verified |
| Normal/build feature trees and package file lists | All seven inspected on each toolchain |
| Minimal library builds | Seven default-minimal graphs plus optional HMR Include pass on both toolchains (16 checks) |
| Core package construction and normal verification | Pass on stable and 1.85.0; 55 files |
| Extracted core archive suite | 187 checks pass per toolchain; extracted bytes unchanged |
| Create package construction and normal verification | Pass on stable and 1.85.0; 14 files |
| Extracted Create archive suite | 22 tests pass per toolchain using a separate complete candidate checkout |
| Actual archive-built Create CLI | Produces the expected five files with MIT notice |
| Generated app | Build, five tests/100 lifecycle cycles, run and strict Clippy pass per toolchain |
| Network-enabled core/Create publish dry runs | All four pass; no upload |
| Five dependent package archives | Stop because the exact unpublished core prerequisite is absent from crates.io; D5 gate |

Both toolchains' normalized core/Create archives identify the candidate SHA and
a clean tree. LICENSE, README, original manifests and the core upstream notice
match the intended inputs. Archive/generated source bytes remain unchanged after
tests. There is no `--allow-dirty` or `--no-verify` bypass in this integrated audit.

Initial offline publish dry runs stopped because Cargo needed HTTP access.
Repeating only those dry runs with network access passed; the archive checksums
remained unchanged. The source and lockfile were not modified.

| Package | Listed files | Actual archive availability |
|---|---:|---|
| core | 55 | Built and tested |
| timer | 27 | Registry prerequisite pending |
| logger | 12 | Registry prerequisite pending |
| loader | 29 | Registry prerequisite pending |
| include | 25 | Registry prerequisite pending |
| hmr | 20 | Registry prerequisite pending |
| create | 14 | Built and tested |

## Verified archive SHA-256

| Toolchain | Package | SHA-256 |
|---|---|---|
| stable 1.97.1 | core | `a43d1315315ec4c1833b8ca8e646838ccff834dcfde88f6aae8f9d86bf569cde` |
| stable 1.97.1 | create | `8c93e42f97460f76c48e3f9cd05ac41fcfca04081b7a95ae1910e117defe3889` |
| 1.85.0 | core | `553707353f62957c4bd85925a1ac4c96606ae5153c9ef6a7c8066f3a46ea78d4` |
| 1.85.0 | create | `96bcde6e97b4685a1527807025e720ddf90e9d6789367212bca828b88b2d5813` |

Cargo versions can produce different normalized artifacts. Recheck the selected
toolchain, source and checksum before any future upload.

## Exact candidate website and hosted checks

Fresh dependency-inclusive Rustdoc, site build, typecheck, lint and export checks
passed on the clean candidate. The export records version `0.1.0-alpha.1`,
revision `84fa9bd...`, status `development-unpublished` and `dirty=false`.
Eight site tests, 70 routes/Markdown pages, 471179 link/anchor references and
15875 asset references pass. The existing strict 112-reference/97-entry local
compiler boundary and zero framework/site exceptions remain intact.

A local exported preview at port 4187 was checked with Chrome DevTools:
1440×1000 desktop and 390×844 mobile show the candidate badge and unpublished
notice without document overflow or console errors. English-to-Vietnamese
navigation preserves the candidate version and translated status. Screenshots
were inspected inline. No candidate Pages deployment occurred; the accepted live
snapshot remains `638ae03`.

At the frozen candidate, [hosted Rust](https://github.com/antruongnguyen/re-cordis/actions/runs/42224982077)
and [hosted Documentation](https://github.com/antruongnguyen/re-cordis/actions/runs/42224982047)
both pass as PR-only runs. These complement the explicit local evidence.

## Remaining D5 gates

- Owner approval to publish this exact candidate; authenticate the designated account
  and recheck normalized name ownership/state. Earlier seven-name HTTP 404 results
  are observations, not reservations.
- Publish in dependency order: core, timer, logger, loader, include, HMR, Create.
- For each dependent package, verify its actual archive/tests/dry-run against the
  already-published prerequisites before uploading it. Do not substitute local
  path or patch graphs for real registry proof.
- After publication, test the actual registry consumer and installed Create CLI
  with the compatible checkout, then confirm exact-version docs.rs builds.
- Tag/release documentation and post-release status updates follow the approved
  plan. Stop on failure and report partial publication; do not automatically yank.

## Evidence retention

Ignored records live under `target/alpha-candidate-20261106/`:
`matrix/`, `packages/`, `minimal-graphs.json`,
`final-site-commands.json`, `final-site-provenance.json`,
`preview-checks.json`, `registry-observation.json`, `focused/` and
`fixture-fix/`. The original failing archive/logs are preserved in
`packages-red-26d54d2/`.

An evidence-copy mistake included build caches and exhausted local disk space.
Only those task-created duplicate copies were removed; original source,
successful command records, trace outputs and archives were preserved.
The integrated audit completed successfully and its archive checksums were
rechecked after cleanup and online dry runs. No source or test failure was
suppressed by that cleanup.

## Final D4 review

Independent review accepted frozen candidate `84fa9bdf0eb4e631fce792bf163494b0a61aeba8`
with no material findings. It verified archive hashes/bytes and test counts,
minimal builds, non-uploading dry runs, exact-candidate site provenance and
hosted success. The twelve package-local fixture correction and the status
wording correction are resolved. D5 registry validation and publication remain
separate, explicitly pending gates.
