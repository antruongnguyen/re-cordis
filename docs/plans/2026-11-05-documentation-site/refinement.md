# Documentation refinement — 2026-11-05

Baseline: abfa0f3092fd8b06765d3c004fbf130793b69fe5 on feat/cordis-completion.
The user requested current GitHub Actions, exclusion of internal planning/design
documents from public docs, up-to-date Markdown and a full-width reference layout.

## Delivered changes

All 15 remote action uses across three workflows now use full commit pins for
current official releases (nine versioned actions and the maintained
rust-toolchain master snapshot). Exact releases, SHAs, official URLs and migration
checks are in [the action audit](actions-refresh.md). The retired macos-14 guide
runner is replaced by supported macos-26. Commands, stable/1.85.0 selection,
artifact transfer, permissions and manual Pages gates are preserved.

Canonical docs/en and docs/vi remain the only reader-page sources. A build-time
Markdown projection removes links, images and reference definitions targeting
docs/plans or docs/superpowers, including relative/encoded and pinned GitHub/raw
URLs. Descriptive link text remains prose; local Markdown retains its engineering
links. The same filtered tree generates HTML, TOC, search and public Markdown.
No hidden targets enter the public source manifest. The output checker rejects
internal paths in filenames and text payloads, including HTML, RSC/JavaScript,
JSON search/provenance, Markdown, llms.txt and bundled Rustdoc. This controls
documentation publication; it does not make tracked Git files confidential.

Both internal trees are structurally outside all seven nested crate roots.
Every crate uses its own README and source documentation, with no private-tree
references, symlinks or doc-attribute imports. Explicit package exclusions would
be redundant. [Package visibility evidence](package-docs-visibility.md) records
the audit and actual verification commands. No crate manifest, API, feature,
Cargo lock or five-file Create template is changed.

The layout follows the inspected [shadcn docs](https://ui.shadcn.com/docs) and
[AI SDK docs](https://ai-sdk.dev/docs/introduction): a viewport-wide frame, 64px
desktop header, fixed left sidebar and right TOC. The article grows to 72rem;
mobile retains Fumadocs search, drawer and TOC popover. Existing Fumadocs slots
provide search/language/theme controls without new dependencies. The development
notice is compact and remains visible. Headings have a sticky-header-aware scroll
margin; an observed hidden-heading regression was corrected.

The root README now leads with concise current status, package capabilities and
documentation entry points rather than historical test-count paragraphs.
AGENTS, CHANGELOG, maintenance/roadmap/release/decision/limitations, matching
EN/VI overviews/channel guides and the local LLM map reflect the new behavior.
The LLM map is 8035 bytes, retaining its <8KiB rule and unchanged essential
framework contracts. Historical acceptance reports remain unchanged.

## Verification

- Latest official actionlint 1.7.12 validates all three workflows. The worker
  independently verified action releases/SHAs and normalized the workflow delta
  to show unchanged commands, gates, permissions and artifact paths.
- 14 actual cargo package --list commands pass (seven crates on stable/1.85.0).
  Every list includes its README/LICENSE and no private tree or parent traversal.
  Counts per toolchain: core49, timer23, logger12, loader27, include25, hmr20,
  create14. Existing exact Rustdoc: 3422 text files pass the private-path scan,
  and both optional HMR method anchors remain present.
- Nine existing documentation regressions and complete stable/1.85.0 guide
  checks pass: 35 pairs, 13 runnable Rust snippets, actual quick start,
  five-test Create and eight-test generated R0 applications, with 100 cycles.
  All Rust tools resolve through rustup using matching explicit environments.
- Website typecheck/lint/build and seven Bun regressions (36 assertions) pass.
  The privacy regression first failed with the missing implementation; after
  filtering it verifies private relative/encoded/pinned URLs, images and reference
  links while preserving public API links and executable code.
- Complete export checking passes 70 canonical HTML/Markdown routes,
  471159 link/anchor and 15875 asset references. Both static search queries return
  ten results with correct locale and project path. No private path is exported.
  The previously accepted 112 dependency-generated Rustdoc link limitations remain
  explicitly counted; no site/framework link exception is added.
- Actual exported-site browser checks pass at /re-cordis: desktop1440 and1920,
  tablet900, mobile390; search/keyboard selection, same-page EN/VI switching,
  reload, desktop theme controls, mobile drawer navigation, and both hidden-tree
  URLs returning404. Corrected heading target appears at80px below the64px header.
  No relevant browser console warning/error occurred before intentional404 probes.

Logs are under target/documentation-refinement, including visibility-red/green,
typecheck/lint/build/check, package-visibility, per-toolchain docs logs and exact
guides-commands.json. desktop-wide.jpg and mobile.jpg capture the layout.
The repeated final guide run is in final-guides-console.log. Site build attempts
during development use baseline HEAD plus dirty=true; rebuild after committing
to pin the public source links to the completed revision.

## Review and remaining gates

Independent [scoped review](refinement-review.md) accepts the final source and
artifact with no actionable findings. It independently checks all ten unique
action implementations, workflow preservation, package boundaries and 3937
exported text files, and records source/artifact fingerprints. This follow-up
does not rerun the accepted 40-stage runtime/parity matrix: production Rust,
existing Python checker behavior, feature/dependency resolution and templates
are unchanged. The closing commit is the latest commit touching this report;
after committing, rebuild the export to replace provisional baseline/dirty
provenance with the clean completed revision.

The result remains local preparation. No GitHub Actions run, Pages deployment,
registry publication or docs.rs hosted/Linux/nightly build is claimed. The future
Pages workflow builds fresh Rustdoc on its hosted macOS runner; the local
compiler-owned metadata/path and dependency-link boundaries remain documented.
