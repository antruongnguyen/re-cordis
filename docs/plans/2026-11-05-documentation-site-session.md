# New-session prompt: publishable documentation for re-cordis

Implement and verify a bilingual Fumadocs documentation site for re-cordis,
prepare GitHub Pages workflows, and prepare the crates for docs.rs documentation.
Execute the work, update documentation, obtain independent review and commit
verified milestones locally. Do not stop at a plan.

Communicate with me in Vietnamese. Write plans, evidence and handoff prompts
in English.

Repository: /Users/I756434/Projects/private/cordis-rust
Integration branch: feat/cordis-completion
Accepted B1 checkpoint: 7112582cb2b52573980f9796030dda5496667d3e
Use the actual latest checkout, including the subsequent llms.txt revision and
this prompt. Start by reading AGENTS.md and checking branch, HEAD and dirty
state. Preserve all newer/unrelated work; do not reset to the checkpoint.

## Current baseline

Core, timer, logger, loader/groups, Include, HMR, offline Create, R0, R1 and
bounded B1 are independently accepted within their documented Rust scopes.
All 35 completion work packages are done; this is a new documentation delivery,
not a reopening of those packages or proof of full TypeScript equivalence.

Seven crates remain 0.0.0/publish=false, Rust 2024/resolver 3/MSRV 1.85.
Preserve production APIs, path dependencies, feature defaults, lockfiles and
the five-file Create template with its MIT notice. The seven-package
0.1.0-alpha.1 remains a proposal requiring a separate release decision.
Create still requires a separate compatible checkout. CORDIS_CREATE_TEST_CHECKOUT
is a test-only archive fixture, not a user-facing generator option.
Runtime filesystem/watcher evidence remains macOS-only.

Current docs have 34 EN/VI page pairs, core/all-satellite API contract chapters,
13 unique runnable Rust snippets, and actual quick-start/Create/R0 application
checks. B1 passed the complete 40-stage stable/1.85.0 matrix. Reuse applicable
accepted evidence; do not convert B0's 147 evidence gaps into a new backlog.
The revised docs/llms.txt is a compact local reading map under 8 KiB.

Read:
- docs/llms.txt
- docs/README.md
- docs/en/README.md and docs/vi/README.md
- docs/en/getting-started.md and its Vietnamese counterpart
- both API indexes and the package guides relevant to your changes
- docs/release-readiness.md, docs/decision.md, docs/limitation.md, docs/todo.md
- docs/plans/2026-11-02-cordis-completion/acceptance.md
- docs/plans/2026-11-02-cordis-completion/evidence/2026-11-05-b1-closure.md
- the linked B1 archive/provenance/review/re-review evidence

Consult current official documentation before choosing implementation APIs:
- https://www.fumadocs.dev/llms.txt
- https://www.fumadocs.dev/docs/deploying/static
- https://www.fumadocs.dev/docs/mdx/collections
- https://www.fumadocs.dev/docs/internationalization/next
- https://www.fumadocs.dev/docs/headless/search/orama
- https://www.fumadocs.dev/docs/integrations/llms
- https://nextjs.org/docs/app/guides/static-exports
- https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages
- https://docs.rs/about/builds
- https://docs.rs/about/metadata

## Selected implementation

1. Keep a single source of user documentation.
   - Preserve docs/en and docs/vi as the canonical Markdown sources.
   - Add a separate website/ application using Fumadocs + Next.js + TypeScript,
     built with Bun and a committed lockfile. Keep website dependencies outside
     the Cargo graph. Use the standard Fumadocs documentation layout/theme;
     no separate branding redesign or image-generation project.
   - Compile existing Markdown at build time. Do not maintain hand-copied
     content trees or convert every page to MDX unnecessarily.
   - Add only necessary title/navigation metadata and small route/link adapters.
     Preserve executable Rust/TOML/shell examples and existing checker paths.

2. Organize the public site around reader tasks.
   - Getting started: overview, package selection, requirements, first app.
   - Core: Runtime/Context, plugins, services/injection, events, effects/cleanup,
     logging and errors.
   - Packages: timer, console logger, loader/groups, Include, HMR and Create.
   - Practical guides: composition, reconfiguration, recovery, shutdown,
     examples and troubleshooting.
   - API contracts: retain existing chapters and connect them to exact Rustdoc.
   - Compatibility/releases: known limits, features, MSRV/platform boundaries,
     changelog and current distribution status.
   - Provide complete EN/VI navigation, language switching and translated UI.
     Keep engineering plans/ledgers/reviews out of default reader navigation
     and search, while preserving explicit evidence/source links where useful.
   - Label the current site development/unpublished/local-checkout documentation.
     Do not replace working local install instructions with fictional registry
     commands or present proposed alpha packages as available.

3. Make the built artifact work on GitHub Pages.
   - Use static export, enumerate all locale/page routes, and configure trailing
     slashes/basePath deliberately. Infer the project-site path from the actual
     Git remote; do not confuse the local cordis-rust folder with the repo name.
   - Avoid request-time middleware, locale negotiation, server search, server
     actions and runtime filesystem access. Pre-rendered GET outputs are fine.
   - Export a static search index and use the browser-side search client.
     Verify English and Vietnamese queries and correct language/basePath URLs.
   - Map README.md, relative .md links, anchors, images and source/evidence links
     to their intended website or pinned GitHub destinations. Do not assume
     the existing filesystem-link checker validates generated website routes.
   - Provide a useful 404 page, working deep-link reloads, accessible keyboard
     navigation and readable desktop/mobile layouts.
   - Generate public llms.txt and per-page Markdown as actual static files with
     correct published paths. Preserve the reviewed local docs/llms.txt and its
     framework constraints; build the public index from the same content/route
     data instead of blindly copying checkout-relative URLs. Do not require
     Accept-header rewrites, an MCP server or an AI backend on GitHub Pages.
     Add llms-full.txt only if it has a clear useful bounded scope.

4. Integrate meaningful verification and prepare deployment workflows.
   - Preserve existing Rust CI. Add website build/static checks and the existing
     documentation regressions/full guide checks with explicit dependency
     restoration rather than relying on the local workstation's warm cache.
   - Keep runtime documentation checks on an appropriate macOS runner until
     wider runtime support is independently established. A Linux site build or
     docs.rs compilation does not establish Linux runtime acceptance.
   - Use official GitHub Pages configure/upload/deploy actions with minimum
     permissions and a separate gated/manual deployment path.
   - Provide reproducible local build/preview commands. Test the exported files
     under the intended project subpath, not only a running Next.js dev server.
   - Do not push, change repository settings or trigger deployment in this
     session. Finish the concrete artifact/workflows first and state the final
     owner-side activation/deployment steps.

5. Prepare docs.rs as a separate API channel.
   - docs.rs builds Rustdoc automatically from crates published to crates.io;
     Fumadocs output is not uploaded there. cargo doc alone does not publish.
   - Review crate-level onboarding, links, feature requirements, examples and
     package README/notices. Preserve exact source-owned API documentation.
   - Add minimal package.metadata.docs.rs only where justified, especially
     inclusion of the optional HMR Include API. Do not alter runtime defaults
     or advertise extra platform support through documentation target metadata.
   - Validate the chosen docs build configuration in an appropriate environment
     if available. Report missing docs.rs/Linux/nightly evidence honestly.
     Never claim a hosted docs.rs success before an actual released build.
   - Plan consistent links between development docs, future versioned guide
     snapshots and exact-version docs.rs pages; do not silently link release
     guides to incompatible development APIs.
   - Keep crate versions, publish flags and registry dependency changes for the
     separate release decision. No fake versions, dependency patches or tags.

## Verification and delivery

State a short execution plan with measurable gates before editing.
Use CodeGraph before exploring indexed code. Keep shared-file writes serialized
and one owner for Cargo/full docs/parity operations.

For Rust commands, resolve cargo, rustc, rustdoc, cargo-clippy and clippy-driver
through rustup for stable and 1.85.0. Set matching RUSTUP_TOOLCHAIN, absolute
RUSTC/RUSTDOC and leading PATH. Formatting uses explicitly resolved stable
cargo-fmt/rustfmt only; do not install redundant components or fall through
to Homebrew Rust.

Run:
- python3 -B tests/docs/test_check.py
- python3 -B tests/docs/check.py --toolchain stable
- python3 -B tests/docs/check.py --toolchain 1.85.0
- locked website install/build, appropriate type/lint checks and static-output
  link/anchor/asset checks
- browser checks of the exported site: both languages, search, language
  switching, deep-link reloads, API/source links, mobile and keyboard behavior
- public llms/Markdown checks against actual exported routes and current claims
- relevant package/Rustdoc checks for docs.rs metadata changes

Do not weaken existing checks to accommodate the site. If Rust production
code, existing executable test/checker behavior, dependency resolution or
feature behavior changes, run the current complete shared stable/1.85.0 matrix
after focused regressions. Reuse accepted runtime evidence for unchanged inputs;
do not repeat the matrix merely for prose or website-only changes.

Update relevant README/CHANGELOG, docs maintenance/roadmap and matching EN/VI
documentation. Keep historical acceptance reports intact; add dated delivery
evidence under docs/plans. Save long logs under target/. Record exact commands,
tool versions, revisions, failures/corrections and evidence boundaries.

Obtain independent review of the completed site/workflows/docs.rs preparation,
then fresh scoped re-review for substantive corrections. If using subagents,
use fresh fork_turns=none contexts: gpt-6.1-sol/high for implementation/evidence
and gpt-6-astra/high for review, with explicit file ownership. Do not resume
workers from earlier sessions.

Commit each verified milestone locally on feat/cordis-completion, without
Co-authored-by trailers. No push, Pages deployment, crate publication, release
tags, remote Create, new templates or framework feature expansion.

Finish with working preview/build instructions, the source/URL organization,
actual checks and review results, evidence paths and commits, clean task tree,
and a precise separation of:
- locally implemented and verified documentation delivery;
- GitHub Pages activation/deployment still requiring an owner action;
- docs.rs/public registry release gates that remain outstanding.
