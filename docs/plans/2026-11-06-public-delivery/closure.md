# Experimental alpha delivery closure — 2026-11-06

Status: **D1–D6 complete within the approved experimental scope**.

## Published release

- [GitHub prerelease v0.1.0-alpha.1](https://github.com/antruongnguyen/re-cordis/releases/tag/v0.1.0-alpha.1)
- [Current English documentation](https://antruongnguyen.github.io/re-cordis/en/docs/)
  and [Vietnamese documentation](https://antruongnguyen.github.io/re-cordis/vi/docs/)
- Seven crates.io packages and their exact-version docs.rs pages are verified in
  [publication evidence](publication.md).

Frozen package source and compatible Create checkout:
`84fa9bdf0eb4e631fce792bf163494b0a61aeba8`.
The annotated release tag points to that exact source.
The GitHub entry is public and marked prerelease; its publication timestamp is
2026-11-06 15:17:31 UTC (22:17:31 Asia/Ho_Chi_Minh).

No package bytes were changed after approval or publication. Current documentation
and evidence updates are later commits, distinct from the immutable release source.

## D6 documentation and website

[PR #6](https://github.com/antruongnguyen/re-cordis/pull/6) integrated the reviewed
current-release wording, registry quick start, installed Create guidance and
published-alpha site status. Its candidate `42afb3fa` passed
[Rust](https://github.com/antruongnguyen/re-cordis/actions/runs/42237050645)
and [Documentation](https://github.com/antruongnguyen/re-cordis/actions/runs/42237050677).

Merged documentation revision:
`0c9eb372e41bc8e63080eb15f23905e632b1dc15`.
Both [main Rust](https://github.com/antruongnguyen/re-cordis/actions/runs/42237834247)
and [main Documentation](https://github.com/antruongnguyen/re-cordis/actions/runs/42237834661)
passed. The macOS jobs waited for runner allocation, then completed successfully;
no check or runner gate was bypassed.

The [manual Pages deployment](https://github.com/antruongnguyen/re-cordis/actions/runs/42240164944)
passed all verification and deployment jobs at that exact documentation revision.
Live provenance reports:

- version `0.1.0-alpha.1`;
- status `experimental-alpha-published`;
- documentation revision `0c9eb372e41bc8e63080eb15f23905e632b1dc15`;
- release revision `84fa9bdf0eb4e631fce792bf163494b0a61aeba8`.

Chrome desktop 1440×1000 and mobile/touch 390×844 checks show the published-alpha
notice and version without document overflow. The English quick start contains
the exact registry pin. The Vietnamese Create guide shows the installed CLI
command, frozen checkout and explicit `--cordis-path` requirement.
The observed application pages have no console warnings/errors and no links to
internal planning trees. Screenshots were inspected inline.

Eight tag-pinned EN/VI index, installation, Create and API-contract links returned
HTTP 200. The tagged source preserves preparation-time notes; the published
release entry, exact-version docs.rs and current guides describe actual availability.
The live GitHub release URL also returned HTTP 200.

This closure changes repository planning/evidence only. It does not alter the
published guides, website code or Rustdoc inputs, so no further site deployment
is required for these completion checkboxes.

## Preserved contract

The API remains experimental, with exact alpha pins recommended. Intentional
breaking changes require release notes and a new coordinated alpha version.
Rust 1.85/edition 2024, local/non-Send hosting, explicit joined cleanup, the tested
Apple Silicon macOS runtime scope, and existing notices remain preserved.

Create is the published offline tool requiring a separate compatible checkout;
its generated application retains local paths and its own `0.0.0`/
`publish=false`. No registry-backed generation mode, new runtime feature,
broader platform promise or full upstream parity claim was introduced.

## Next decisions

No implementation work was automatically selected after this release.
The actual registry R0 and installed Create consumers passed without a remaining
runtime defect. Two concrete follow-up decisions are available:

1. Evaluate whether clone-free scaffolding is worth selecting next. The installed
   Create exercise required a separate pinned checkout and emitted canonical local
   paths; a registry-backed mode would address that observed onboarding constraint.
2. Select an additional runtime platform only when an adopter needs it. Current
   full runtime acceptance is Apple Silicon macOS; hosted Linux docs and CI are not
   a substitute for a separately scoped platform acceptance project.

The 147 tracked B0 evidence gaps retain their original meaning and are not a
new list of demonstrated defects or automatically approved implementation work.

## Evidence retention

Local records are under `target/publication-20261106/`, including registry
owner/checksum records, progressive package/consumer logs, docs.rs observations,
tag-link checks, `github-release.json`, `live-release-provenance.json`,
`live-release-browser.json`, release-doc implementation logs and Pages run metadata.

Independent reviews accepted the D5 evidence, D6 current-docs change and release
notes, with only a checksum-table formatting correction and minor release-note
copy edits. Those corrections are applied.
