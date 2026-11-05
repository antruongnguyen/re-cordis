# Independent documentation refinement review

Reviewed on 2026-11-05 at 06:49 UTC against baseline
`abfa0f3092fd8b06765d3c004fbf130793b69fe5`, on `feat/cordis-completion`.

**Verdict: accepted within local preparation. No actionable correctness or
scope findings in the reviewed delta.**

The review covers the four requested refinements: current workflow actions,
exclusion of the two internal documentation trees from public documentation,
current Markdown/status, and the full-width reader layout. It does not reopen
the independently accepted runtime, package policy or Rustdoc dependency-link
boundaries.

## Source and workflow review

- Read the current AGENTS.md, complete relevant diff, new visibility transform
  and tests, its prepare/check integration, layout components and CSS, current
  documentation changes, and the action/package audit reports. The transform
  runs before HTML, TOC, search and Markdown generation. It removes private
  link/image/reference targets while retaining useful descriptive prose and
  canonical repository evidence.
- Independently queried GitHub's latest release API and resolved each tag to
  its commit. All nine versioned action pins match the latest stable releases;
  rust-toolchain matches its maintained master commit. Read all ten pinned
  action manifests: eight use Node 24, two are composite. The upload archive
  and Pages enablement defaults remain compatible with the callers.
- Independently reconstructed every workflow back to baseline by reversing
  only the action pins, explicit stable input and macOS runner substitution.
  All three files match byte-for-byte. There are 15 immutable remote uses and
  one unchanged local reusable workflow call; commands, permissions, matrices,
  artifact paths and manual deployment gates do not drift.
- Confirmed the official macOS 14 retirement issue specifies 2026-11-02 for
  GitHub Actions and names macos-26 as a supported migration target.
- Confirmed the virtual workspace and all seven nested package manifests keep
  `0.0.0`/`publish=false`, independent package READMEs, and no import/build
  configuration for repository-level private Markdown. All 14 recorded package
  lists contain README/LICENSE and exclude private paths and parent traversal.
  No Rust source, Cargo manifest/lockfile, website dependency/lockfile or Create
  template changes occur in this delta.
- The root front door and matching EN/VI channel guides reflect the completed
  local site and current release boundaries. Historical reports remain dated
  evidence. The layout reuses installed Fumadocs controls and layout variables;
  the 64px desktop header, viewport-wide frame, sidebar, right TOC, bounded
  article and mobile drawer match the requested reference structure. Inspected
  the controller's desktop-wide and mobile screenshots. The final CSS includes
  the observed heading-scroll correction.

## Verification inspected and independently checked

Independently verified zero exits and SHA-256 hashes in the final
`target/documentation-refinement/guides-commands.json`, then read both full
guide logs and the regression log: nine regressions; 35 language pairs and
13 executed snippets on explicit stable/1.85.0; actual quick-start/Create and
generated R0 applications, including their five/eight tests and lifecycle
cycles. Read the final build/check logs: 70 routes, seven Bun tests with
36 assertions, 471159 link/anchor checks and 15875 asset references. Typecheck,
lint and actionlint results are recorded by the controller/action audit.

The controller owns browser execution. Its final evidence records 1440/1920
desktop layouts, 900 tablet and 390 mobile without document overflow; keyboard
search, same-page language switching, theme changes, mobile navigation and
reload; both private-tree URLs returning 404; and the corrected heading landing
at about 80px below the 64px header. This review inspected the screenshots and
implementation rather than repeating those browser interactions.

After the controller froze source and marked the final artifact ready, this
review independently scanned all 3957 exported files and 3937 text payloads.
The scan also normalized JSON-escaped slashes, percent encoding and HTML
entities. No private directory path, route or reference was present. All
70 generated reader sources are in docs/en or docs/vi, in 35 matching pairs.
The public provenance source hashes match the current files; all seven Rustdoc
entry points and the optional Include watcher method remain present. The export
contains 3448 bundled Rustdoc files. The previously accepted 112 dependency
prose-link boundary remains unchanged, with no framework/site exception added.
`git diff --check` also passed.

## Reviewed fingerprints and limits

The source fingerprint covers the 27 changed/new implementation, current-doc
and audit files. It excludes this review and the controller's new refinement
summary, which describe the work rather than feed the public artifact. Manifest
hashes below use SHA-256 over the compact, key-sorted JSON mapping of relative
file paths to file SHA-256 values.

| Item | SHA-256 |
| --- | --- |
| Reviewed source manifest | `272a70011f830554b9415b2ec68123e5d31fc1c0214174f45fe37b75ae365787` |
| Complete exported-file manifest | `e650493cc88928fa61f625f61f238a5c1d8742b85def8865d559e6781b6d8461` |
| Exported build-provenance.json bytes | `997c2e30989f6c95ac074897b8b682b814727d5272b7175a8515f340ff87397b` |

Local review manifests and independent API results are retained under
`target/documentation-refinement-review/`. The inspected artifact truthfully
reports baseline HEAD and `dirty=true`: its pinned GitHub source URLs identify
the baseline commit, not yet the uncommitted refinements. Commit the accepted
source and rebuild/check from the clean resulting HEAD before using a final
revision-pinned artifact.

This acceptance establishes local documentation preparation only. It does not
claim hosted Actions execution, Pages deployment, registry publication, hosted
docs.rs/nightly/Linux builds, or additional runtime-platform support. Existing
compiler-owned Rustdoc metadata paths and dependency-link limitations retain
their accepted, documented scope.
