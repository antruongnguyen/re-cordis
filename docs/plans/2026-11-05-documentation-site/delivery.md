# Documentation site delivery — 2026-11-05

## Scope and starting point

Branch: feat/cordis-completion. Actual clean starting HEAD:
94907cbea555e05cacaaf35a4d49a84ba9a503a4. This includes the newer llms revision
and session prompt after accepted B1; no reset to the B1 checkpoint occurred.
Spec: [approved session](../2026-11-05-documentation-site-session.md).
Execution: [ownership and gates](plan.md).

The implementation is locally verified; the immutable implementation commit
and independent final verdict are recorded in the closing section after review.
No push, settings change, deployment, publication or release tag is performed.

## Delivered artifact and channels

Canonical content remains docs/en and docs/vi: 35 paired pages, including the
new [documentation-channel guide](../../en/guides/documentation.md).
The standalone website uses Fumadocs 16.17.10, Next.js 16.4.5, React 19.3.1,
TypeScript 6.0.3 and Bun 1.4.4 with a committed frozen lockfile. Node is
22.23.3 locally. Ordinary Markdown is compiled with remark/GFM and the Fumadocs
Shiki renderer; no hand-copied content tree or bulk MDX conversion is maintained.
Current official references are listed in the [website instructions](../../../website/README.md).

The actual origin is antruongnguyen/re-cordis, so the project path is /re-cordis.
Static locale routes, translated Fumadocs UI/navigation, browser search, Markdown
files and the public llms index share canonical route data. Engineering ledgers
stay out of reader navigation/search; explicit source/evidence links are pinned
to the build revision. The reviewed local docs/llms.txt is byte-identical
(7947 bytes) and remains checkout-relative.

| Channel | Exported path under /re-cordis |
|---|---|
| EN / VI guides | /en/docs/ and /vi/docs/; matching guide/API subpaths |
| Exact API | /rustdoc/{full-build-revision}/{crate}/index.html |
| Page Markdown | /markdown/{language}/{page}.md |
| Public retrieval index | /llms.txt |
| Browser search data | /search-index/en.json and /search-index/vi.json |

Rustdoc includes dependencies so inherited trait links in re-cordis pages
resolve. It is generated from the same checkout and copied without changing any
compiler-emitted byte. A package landing and four empty external-implementor
registration scripts complete references absent from the compiler output.
The exact script paths/owners/references and file hashes are recorded in public
build provenance. Public file-manifest keys hash relative paths to avoid copying
machine-specific generated names into website-owned provenance.

There is a deliberate checked dependency-documentation boundary: 112 unresolved
compiler-generated prose/method references, 76 targets in 32 dependency-owned
HTML pages (97 exact file/target/count tuples). The versioned list is
website/scripts/rustdoc-prose-links.json. Every exception is limited to an
anchor href on its exact dependency file. New/missing/count-changed entries fail.
No re-cordis page or website page is exempted; assets and real local paths/anchors
remain strictly checked. Rustdoc's emitted alias redirects and source line-range
anchors are validated according to their actual browser behavior.

Local Rustdoc source pages retain compiler-owned build-location metadata and one
generated filename contains build directories. Stable 1.97.1 accepts
remap-path-prefix but did not remove this source metadata; a separate tiny probe
of remap-path-scope=documentation failed because it requires unstable options.
No nightly/bootstrap or HTML rewriting is used. Website-owned public metadata
and client output exclude the workstation HOME. Pages will rebuild Rustdoc on
its hosted macOS runner, rather than upload this local export. This is not a
claim that every byte of the local compiler output is anonymous.

## CI and docs.rs preparation

The original Rust CI is unchanged. Documentation CI explicitly restores Cargo
dependencies and Bun's lockfile. macOS runs stable/1.85.0 full guide checks and
focused Rustdoc gates. The stable macOS job builds/uploads exact dependency
Rustdoc; the Linux job downloads that artifact and builds/checks only the site.
This transfer follows an early independent-review concern that a local macOS
compiler-output exception list does not establish Linux Rustdoc compatibility.

Pages uses official configure-pages@v5, upload-pages-artifact@v4 and
deploy-pages@v4. Normal pushes/PRs only check/upload a preview. Deployment is a
separate manual workflow requiring PAGES_DEPLOY_ENABLED=true, deploy=true,
successful documentation jobs and the github-pages environment. Only deployment
gets pages:write/id-token:write; build/configuration is read-only. Owners still
must push the reviewed commits, make the workflow available on the default
branch, enable the GitHub Actions Pages source, configure environment protection,
set the variable and dispatch the reviewed ref. These are prepared files,
not evidence of a hosted workflow or deployed site.

The [docs.rs assessment](docs-rs-assessment.md) reviews all seven crates.
Only HMR gains package.metadata.docs.rs selecting include. Versions remain
0.0.0/publish=false; dependencies/defaults/templates/notices/API source are
unchanged. docs.rs builds after actual crates.io publication; Fumadocs output
and cargo doc do not publish there. Exact-version release guides/docs.rs links,
registry dependency verification and publisher/release authority remain gates.
No active Linux container or nightly toolchain was available; no Linux/nightly
or hosted docs.rs success is claimed.

## Executed local verification

All Cargo operations were serialized by the controller. Each selected toolchain
resolved cargo, rustc, rustdoc, cargo-clippy and clippy-driver with rustup, with
matching RUSTUP_TOOLCHAIN, absolute RUSTC/RUSTDOC and leading toolchain bin PATH.
Stable cargo-fmt/rustfmt were resolved separately for generated-app formatting.
No Homebrew Rust fallback or component installation occurred.

| Check | Result |
|---|---|
| python3 -B tests/docs/test_check.py | 9/9 pass |
| python3 -B tests/docs/check.py --toolchain stable | 35 pairs, 13 Rust snippets, external quick start, real Create and generated R0 pass |
| Same full checker --toolchain 1.85.0 | Same scope passes |
| Actual generated basic / R0 apps in each checker | 5 / 8 tests, including 100 lifecycle cycles; build, run, strict Clippy and real-watch execution pass |
| cargo doc --workspace --all-features --no-deps --locked --offline | Warnings-denied pass on both toolchains |
| HMR --doc with include / no-default-features | One doctest each, both configurations on both toolchains pass |
| Optional HMR HTML | watch_include and replace_include anchors present on stable and 1.85.0 |
| Dependency-inclusive site Rustdoc | Stable warnings-denied build passes; fixes raw Rust-path links in own APIs |
| Empty-cache dependency restoration | Stable followed by both full guide runs pass; separately empty 1.85.0 cache also passes |
| bun install --frozen-lockfile | Pass |
| Website build/typecheck/lint/check | Pass; 5 unit regressions, 22 assertions |
| Provisional static export crawl | 70 HTML/Markdown routes; 470915 valid link/anchor references; 15875 asset references; known dependency boundary reported separately |
| Public retrieval/search | Code blocks preserved; public index covers routes; EN cleanup and VI dọn dẹp each return 10 correctly scoped results |
| Workflow validation | actionlint 1.7.7 and Bash/Python syntax checks pass |
| Preservation | 176 controller baseline inputs checked; only HMR Cargo metadata changed; local llms and historical acceptance files unchanged |

Stable tools are Cargo/rustc/rustdoc 1.97.1, Clippy 0.1.97, rustfmt 1.9.0-stable.
MSRV tools are 1.85.0 with Clippy 0.1.85. The existing full shared runtime/parity
matrix is reused for unchanged executable/API/feature/lock inputs; it was not
relabelled as freshly executed. This documentation delivery adds no TS parity
or broader runtime-platform claim.

Long logs and exact argument arrays/environments/hashes are retained under
target/documentation-site: tools-*.json, rustdoc-commands.json,
guides-commands.json, preservation.json, baseline-inputs.json, cold-msrv-*.log,
website-precommit-*.log/json, actionlint-final.log and browser-checks.json.
The local verify.py harness records each selected command/exit/time/log hash.
The provisional artifact is explicitly based on 94907cb plus dirty=true;
it is rebuilt after the implementation commit before final review.

## Browser evidence and corrections

Chrome DevTools MCP exercised the actual exported Bun preview at
http://127.0.0.1:4173/re-cordis/, not a Next.js dev server. Desktop 1440x1000
and mobile/touch 390x844 render readable content without document overflow.
Screenshots are retained as desktop-vi.jpg and mobile-vi.jpg in the log directory.

Fresh interactions cover EN cleanup and VI dọn dẹp search with keyboard
selection, locale-preserving language changes, translated empty results,
Escape dismissal, direct reload, mobile sidebar open/navigation/close,
skip-link focus, exact HMR Rustdoc navigation, public text/Markdown/search files
and HTTP-404 recovery to both-language entry links. Corrected Fumadocs pages
have no relevant console warnings/errors or framework overlay. Native Rustdoc
logs non-blocking font-preload warnings from unchanged compiler HTML.
Clipboard-read verification was unavailable (browser permission timeout);
no claim about clipboard contents is made.

Demonstrated failures and corrections:

- Workspace-only Cargo fetch missed futures-macro needed by the generated R0
  app's default futures-util features. The unmodified full checker failed from
  an empty cache; the new disposable dependency-superset restore step fixes it.
  Original failure output is retained as docs-stable-cold-failure.log.
- Search initially doubled /re-cordis because Next added basePath to published
  URLs. A single framework router/link boundary fixes it; route regressions and
  both browser languages verify the correction.
- SSR language, explicit subpath favicon and focusable skip target were added
  after browser inspection. Mobile/desktop and keyboard checks were repeated.
- No-deps Rustdoc inherited links failed; dependency-inclusive output resolves
  own API references. Compiler alias redirects/line anchors are followed by the
  checker. Missing empty-implementor files and finite dependency prose issues
  are explicitly accounted for, without changing generated API text.
- The checker rejected a stale source fingerprint and a concurrently changing
  Rustdoc input. Execution ownership was corrected and a stable snapshot used.
- TypeScript 7 was incompatible with the selected ESLint integration; pinning
  TypeScript 6.0.3 restores actual type/lint checks without suppressing them.
- actionlint caught accidental YAML indentation from a tool string; exact
  content was rewritten, then actionlint passed. Failed Kotlin edit/probe
  compilations wrote no source files.

## Review and closing record

Independent gpt-6-astra/high review is in progress in a fresh context. The
reviewer has inspected settled code, preservation and recorded gate hashes.
Final immutable snapshot, verdict and any scoped re-review are appended after
the implementation commit. This report does not yet claim independent acceptance.
