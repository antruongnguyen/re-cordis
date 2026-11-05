# Fresh scoped correction verification

Date: 2026-11-05. Verdict: **all four selected corrections are ADDRESSED;
no new actionable breakage found within this scope.**

This is fresh verification of corrections made during preparation and initial
review, already present in implementation commit
`f7f503fb629652fe9d6d66edca122ab01b7297d8`. It is not another full implementation
review and does not describe a post-review code delta. The completed
[independent full review](review.md) remains the full-delivery assessment.

## Exact inputs and method

- Approved [session](../2026-11-05-documentation-site-session.md), repository
  AGENTS.md and [execution plan](plan.md).
- Initial delivery baseline: `94907cbea555e05cacaaf35a4d49a84ba9a503a4`.
- Reviewed HEAD: `f7f503fb629652fe9d6d66edca122ab01b7297d8`, branch
  `feat/cordis-completion`. The only non-ignored change before this pass was the
  authorized, untracked first review report; implementation files were unchanged.
- Frozen `website/out/build-provenance.json`: revision above, `dirty=false`,
  SHA-256 `0066499db29c329db47abb82a684edf9fe000883cc4f0238ea5eeae89d7090f8`.
- First review report as read: SHA-256
  `f67b8da86d00f93bda54dd7b2ac9678ba0b94bf3b7cb7661fbbb50ead303d03f`.
- [Delivery evidence](delivery.md) as read, before the controller's closing
  update: SHA-256
  `edcc8a607cc674133ddec0277e779851957b7871c64107a1db3e6e3ae7e196b9`.

CodeGraph supplied current relevant source for the router/provider, search,
locale/page layouts, preparation/finalization, Rustdoc additions and export
checker. Workflow YAML, the finite dependency list, focused regression source,
saved browser records and committed-build logs were also inspected.

A fresh read-only Python audit inspected the existing export, compared compiler
input/output bytes, parsed actual anchor elements and validated saved log hashes.
Its results are in `target/documentation-site-rereview/correction-audit.json`
(SHA-256 `09c367398081e55ed6d007e7b5bdeaa3945c912fd970b55e43e1b60fa2940656`).
No build, Cargo, full-guide, browser lifecycle or deployment command was run by
this reviewer. No source, export, Git index or HEAD was modified.

## Correction findings

### 1. Search routing, SSR language and keyboard target — ADDRESSED

`website/components/provider.tsx` places both Fumadocs links and imperative
router navigation through `routerUrl`. The helper strips the existing project
prefix at the Next.js boundary, while the search client fetches the published
locale index under `/re-cordis/search-index/{locale}.json`. The locale root
layout emits `html lang` during static rendering; page content supplies
`id="main-content"` with `tabIndex={-1}` for the translated skip link.

The fresh artifact audit confirms correct language and a unique, focusable skip
destination with a matching skip link on all 70 locale pages. Existing focused
route regression output records the single-prefix behavior. The immutable-f7
browser review records successful EN `cleanup` and VI `dọn dẹp` searches,
keyboard selection, same-page language switching and a hard reload with one
project prefix. Actual skip-link focus is controller browser evidence from the
corrected provisional build; this pass independently verifies the same final
source and emitted focus attributes, without claiming a new browser execution.

### 2. macOS Rustdoc handoff and manual deployment gates — ADDRESSED

`documentation.yml` generates dependency-inclusive, all-feature Rustdoc only
in the stable macOS guide job and uploads the complete `doc` tree with
`include-hidden-files: true`. The Linux site job depends on the guide matrix,
downloads that named artifact and points `CORDIS_RUSTDOC_DIR` at it. It does not
regenerate Rustdoc on Linux. Thus the prepared workflow uses the compiler-output
platform against which the finite dependency boundary was inspected.

`pages.yml` is dispatch-only. Its verification job requires both `deploy=true`
and `PAGES_DEPLOY_ENABLED=true`; deployment needs successful verification and
uses the `github-pages` environment. Only the deploy job has `pages: write` and
`id-token: write`; site/configuration jobs have read permissions. Enabling Pages
and configuring environment protection remain owner actions. This is a source
and recorded-validation assessment, not evidence of hosted execution.

### 3. Public provenance and compiler-owned paths — ADDRESSED

Finalization removes `rustdocInput` from public provenance and hashes relative
Rustdoc paths as manifest keys. The frontend imports public settings rather
than private build provenance. Fresh scanning finds no workstation HOME bytes
in website-owned output. Every one of the 3,448 final Rustdoc files matches its
hashed-path manifest entry; all 3,443 compiler-emitted files are byte-identical
to the recorded compiler input, including original filenames.

This correction deliberately does not anonymize compiler output. The fresh
scan confirms HOME bytes remain in 508 compiler-owned files and one emitted
filename, matching the first review and explicit delivery limitation. Hashing
manifest keys avoids repeating these names in website-owned metadata; it does
not remove source metadata from exact Rustdoc. No broader anonymity claim is
accepted or made.

### 4. Exact dependency exceptions and empty registrations — ADDRESSED

The fresh audit verifies all 97 file/target/count tuples against actual
dependency `<a href>` elements, totaling 112 occurrences. None belongs to a
`re_cordis_` page. The checker permits this finite set only at the exact
revision-scoped dependency source and target, only for anchor elements, then
requires the full tuple set and every expected occurrence count. Site/framework
links and asset references receive no such exception. The committed checker
log explicitly reports zero framework/site exceptions.

The four generated implementor scripts use Rustdoc's immediate
`register_implementors` or deferred `pending_implementors` protocol. Finalization
checks their known source references, refuses alternate emitted registries and
does not overwrite an emitted file. The focused VM regression covers both
protocol branches; its saved passing output was inspected and hash-verified.
The fresh byte/set comparison accounts for exactly those four scripts plus one
package landing as the only additions to compiler output. The framework-owned
ServiceInvoke empty registration is a generated asset, not a framework link
exception or a replacement for compiler-emitted API bytes.

## Evidence boundaries

Eight committed-build log hashes and three initial-review execution log hashes
were freshly verified. Their successful build, unit/type/lint/export checks are
reviewed prior execution evidence, not reruns by this reviewer. Existing browser
interaction evidence is likewise attributed above. No uncertainty within these
four corrections required repeating the already-passing full suites.

The 112 finite dependency links remain unresolved as documented. This verdict
does not establish hosted Actions/Pages operation, pushed-source availability,
crates.io/docs.rs publication, Linux/nightly docs.rs execution, new runtime
platform support or clipboard contents. Those gates remain separate. The
controller owns the final closure commit and subsequent clean-HEAD build.
