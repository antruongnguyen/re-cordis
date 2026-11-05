# Independent documentation delivery review

Date: 2026-11-05. Scope: the approved documentation-site session and execution
plan, including the completed static website, CI preparation and docs.rs metadata.

## Snapshot and verdict

- Baseline: `94907cbea555e05cacaaf35a4d49a84ba9a503a4`.
- Reviewed implementation: `f7f503fb629652fe9d6d66edca122ab01b7297d8` on
  `feat/cordis-completion`.
- Final export provenance identifies that full revision and `dirty=false`.
  Its SHA-256 is
  `0066499db29c329db47abb82a684edf9fe000883cc4f0238ea5eeae89d7090f8`.
- The checkout was clean before this review report was written. The reviewer
  made no source/index/HEAD changes and ran no Cargo/full-guide/parity commands.

**Accepted within local documentation delivery and deployment-preparation scope.**
No remaining actionable blocking or non-blocking implementation findings were
identified in this snapshot. Acceptance includes the explicitly bounded
compiler-owned Rustdoc limitations below; it does not mean every dependency
hyperlink is functional or every local compiler-output byte is anonymous.

## Independent checks performed

CodeGraph was used before exploring indexed website code. The review covered
the entire website architecture and final source delta, both workflows and
restoration scripts, the paired channel guides, current-status documentation,
the package assessment and minimal HMR metadata.

Fresh reviewer execution and artifact inspection established:

| Check | Result |
|---|---|
| Bun unit regressions | 5 passed, 0 failed; 22 assertions |
| Bash syntax and baseline-to-implementation `git diff --check` | Pass |
| Canonical rendered documentation | All 35 EN/VI pairs present; independent crawl checked 4,822 site-local references and HTML anchors, document language and focus destination |
| Export/source provenance | Recorded source hashes match the committed source blobs |
| Exact Rustdoc copy | All 3,443 compiler-emitted files compared byte-for-byte with the fresh input; 3,448 final Rustdoc files match the hashed-path manifest |
| Generated additions | One package landing and four finite empty implementor registrations accounted for separately |
| Known dependency references | All 97 exact tuples and their 112 occurrence counts checked against actual dependency `<a>` elements; no framework entries |
| Public website data | No workstation HOME path found outside compiler-owned Rustdoc; public provenance omits the private input path |
| Static HTTP | EN/VI pages, llms.txt, Markdown, both search indexes and HMR FileWatcher return 200 with appropriate content types; missing route returns 404 and bilingual recovery text |
| Browser interaction | English and Vietnamese search, keyboard result selection, same-page language switching and deep-link reload pass under `/re-cordis` |
| Browser rendering | Correct title/language/revision, meaningful content and no framework overlay; no relevant console warnings/errors after reload; desktop/mobile widths stay within 1440/390-pixel viewports |

The final browser flow used Chrome DevTools MCP against the exported Bun preview,
with the controller relinquishing page 4. EN `cleanup` and VI `dọn dẹp` each
returned ten language-appropriate options. The Vietnamese selection navigated
to the cleanup guide with one project prefix and survived a hard reload.
The reviewer inspected the final mobile screenshot inline and the controller's
saved desktop/mobile screenshots. A screenshot file-save attempt was refused
by the browser tool's workspace routing; inline capture worked. This was a tool
restriction, not an application failure.

Audit records are under `target/documentation-site-review/`:
`fresh-checks.json`, `artifact-audit.json`, `http-checks.json`,
`browser-checks.json`, `preservation-and-evidence.json` and
`committed-log-hashes.json`. These are local audit outputs, not hosted evidence.

## Implementation and preservation assessment

The website compiles canonical Markdown at build time, with explicit locale
routes, standard Fumadocs UI, static browser search and actual Markdown/LLM
files. Navigation covers every canonical page in both languages. Engineering
ledgers remain outside reader navigation/search; useful source/evidence links
pin the build revision. The real Git remote determines `/re-cordis`. Static
export needs no request-time Next.js server, middleware or search service.

The independent preservation audit covered 182 Rust/checker/CI/LLM inputs.
Only HMR's docs.rs metadata differs; parsing and removing that table yields
the original manifest. APIs, dependencies, feature defaults, Cargo locks,
Create template/notices and existing Rust CI are preserved. The local llms.txt
remains exactly 7,947 bytes. All 233 historical completion files have no Git
content changes; one pre-existing TSV has normal CRLF checkout normalization.

The early review concern about building Linux Rustdoc against a macOS-derived
compiler exception set is resolved: stable macOS produces the exact Rustdoc
artifact, including hidden files, and Linux downloads it for static site work.
Both macOS guide jobs must succeed first. Normal CI is read-only. Pages
deployment requires manual input, the repository opt-in variable, successful
verification and the `github-pages` environment; only the deploy job receives
Pages write/OIDC permissions. Environment protection and activation remain
documented owner actions.

HMR's selected `include` docs.rs feature exposes the optional API without
changing application defaults. The package/channel documentation accurately
separates local Rustdoc, Fumadocs, future exact-version releases and actual
docs.rs builds following crates.io publication. No invented released package,
registry installation instruction, target acceptance or hosted success is added.

## Evidence reviewed, not rerun by this reviewer

The reviewer read the controller's exact command/environment records, verified
their recorded successful log hashes, and inspected the final committed-build
logs. They record nine Python regressions, full stable and 1.85.0 guide runs
(35 pairs, 13 Rust snippets, quick start, real Create and generated R0 checks),
five/eight application tests including 100 lifecycle cycles, the cold-cache
restoration correction and independent cold MSRV run, and the six focused
workspace/HMR Rustdoc configuration checks.

Final committed-source logs record a fresh dependency-inclusive stable Rustdoc
build, frozen Bun install, static build, typecheck, lint and complete export
check. The complete checker reports 470,915 link/anchor references and 15,875
asset references, with the known dependency boundary reported separately.
These broader results are reviewed controller evidence; the table above
identifies the reviewer's own executions. The accepted shared runtime/parity
matrix is reused for unchanged inputs and is not claimed as freshly rerun.

## Accepted boundaries

The finite dependency-only set contains 112 unresolved compiler-generated
references: 54 Rust-path, 35 inherited-anchor and 23 inactive/incorrect targets,
across 32 dependency pages and 76 targets. Exact tuple/count checks fail on drift;
they apply only to those dependency anchor elements. There is no framework/site
exemption. The four added empty registrations follow the immediate/deferred
Rustdoc protocol, are tested, and do not replace any compiler-emitted file.

The independent full-byte scan found workstation HOME text in 508 compiler-owned
Rustdoc files and one compiler-owned filename. This is explicitly documented.
Website-owned public data is clean, while emitted API/source HTML remains exact.
The prepared Pages workflow rebuilds on hosted macOS rather than uploading this
local export. This boundary is acceptable for the authorized local-only scope;
the local compiler output must not be described as anonymized.

No hosted Actions, Pages activation/deployment, pushed-source availability,
crates.io/docs.rs publication, Linux/nightly docs.rs execution, broader runtime
platform support or clipboard-content verification is established. Those gates
remain outstanding as stated in the delivery report and owner instructions.
