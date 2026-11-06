# re-cordis documentation website

Public development documentation using standard Fumadocs UI, Next.js
static export and Bun. `../docs/en` and `../docs/vi` are the sole user content
sources. Markdown is parsed at build time with remark/GFM and rendered with
Fumadocs' Shiki highlighter. The highlighted tree renders through native
CodeBlock/Pre components: block typography is isolated from inline code, with
localized copy buttons that preserve the complete code text. No copied content tree, MDX conversion,
request-time filesystem, middleware, server search or AI backend is needed.

The [live site](https://antruongnguyen.github.io/re-cordis/) provides
[English](https://antruongnguyen.github.io/re-cordis/en/docs/) and
[Vietnamese](https://antruongnguyen.github.io/re-cordis/vi/docs/) guides,
[API contracts](https://antruongnguyen.github.io/re-cordis/en/docs/api/),
same-revision Rustdoc and a [public AI index](https://antruongnguyen.github.io/re-cordis/llms.txt).
The initial manual deployment on 2026-11-06 used revision
`3718217aac6f8f11f1f11c221bf2730cbe169bd6`; its
[build provenance](https://antruongnguyen.github.io/re-cordis/build-provenance.json)
records that revision. Later documentation changes require a new manual deployment.
The seven `0.1.0-alpha.1` candidate crates remain unpublished and experimental.
The existing live deployment is a separate snapshot; local candidate changes do
not refresh it. Website labels, Markdown, llms.txt and public provenance derive
the crate version from `../Cargo.toml` through generated settings. The private
website package's own `0.0.0` version is independent.

## Build and preview

Use Bun 1.4.4 and Node.js 22. From the repository root, produce exact Rustdoc:

```sh
source .github/scripts/rust-tools.sh stable
"$CORDIS_CARGO" fetch --locked
CARGO_TARGET_DIR="$PWD/target/site-rustdoc" "$CORDIS_CARGO" doc --workspace --all-features --locked --offline
```

This uses matching absolute rustup tools. Linux compilation does not establish
Linux runtime acceptance. Keep Cargo and website operations serialized.

From `website/`:

```sh
bun install --frozen-lockfile
CORDIS_RUSTDOC_DIR=../target/site-rustdoc/doc bun run build
bun run typecheck
bun run lint
bun run check
bun run preview
```

Preview serves the exported `out/` files at
`http://127.0.0.1:4173/re-cordis/`, including deep links and a real 404 response.
`PORT` can change the port. Stop with Ctrl-C; rebuild after content changes.

The build requires an explicit complete Rustdoc input and verifies all seven
crate indexes plus HMR's optional `watch_include` API. Generate it freshly from
the same checkout before each accepted build. The full Rustdoc directory,
including dependencies and their crosslinks, sources/search/assets, is copied to `out/rustdoc/<full-HEAD>/`.
`build-provenance.json` records source hashes, Rustdoc hashes, revision and dirty
status. Optional `CORDIS_REVISION` must exactly equal the current full HEAD.
Local changes are allowed for review; working public source links require the
final committed build revision to be available on GitHub.


Dependency documentation is included to resolve Rustdoc's inherited trait/prose
crosslinks; package-only `--no-deps` remains an optional local API view, not the
site bundle. Every compiler-emitted file stays byte-identical. The assembly adds
one package landing because workspace Rustdoc omits its root `index.html`, and
exactly four missing empty external-implementor registrations using Rustdoc's
own protocol (ArcWake, EventReceiver, ServiceInvoke, VariantAccess). Their exact
paths/references/reasons are recorded separately in provenance; new missing
assets fail the checker. Compiler-generated Rustdoc font preload warnings are
retained, while the Fumadocs pages have no browser warnings/errors.


A precise dependency-only boundary remains: the accepted Rustdoc 1.97.1 artifact
has 112 compiler-owned references in 97 file/target/count entries (76 targets,
32 dependency files); the hosted Rustdoc 1.99.1 artifact has 110 references in
96 entries (75 targets, 31 files). The difference is exactly two hidden
`syn::parse::Peek::Token` uses in `ParseBuffer::parse_terminated`: 1.97.1 emits
links to an absent anchor; 1.99.1 renders both uses as plain `P::Token` text.
`scripts/rustdoc-prose-links.json` retains the original 97 entries. The checker
selects the 96-entry expectation only when that exact method signature contains
both plain token uses and the old target is absent. Every other omitted/changed
entry still fails. Exceptions apply only to anchor elements on those
dependency files. There are zero framework/website exceptions; all assets remain
strict. `.generated/static-check.json` records each entry's build hash, and the
checker visibly reports the boundary. Valid Rustdoc re-export redirects preserve
queries/anchors and are followed before checking target IDs. These retained
dependency prose limitations are not fully clean Rustdoc hyperlinks or extra
runtime support.

Public provenance omits private input paths and keys Rustdoc manifests by
SHA-256 of relative paths (recorded as `sha256-relative-path`). Exact compiler
source HTML/filenames can still retain local paths: stable Rustdoc's supported
path-remap flags did not remove these source metadata paths, and documentation
remap scope requires nightly unstable options. Do not upload this workstation
artifact. The Pages workflow rebuilds Rustdoc on its hosted macOS runner;
website-owned public data/bundles are checked for workstation paths. No
anonymous-source or hosted-release claim is made for this local preview.

## URL and content organization

The Pages path is inferred from the GitHub origin, currently
`https://github.com/antruongnguyen/re-cordis.git`, giving `/re-cordis`.
Trailing-slash pages are enumerated for both languages:

- `/re-cordis/en/docs/` and `/re-cordis/vi/docs/`: README indexes.
- `/re-cordis/<language>/docs/guides/<page>/`: practical/package guides.
- `/re-cordis/<language>/docs/api/<page>/`: API contracts with exact Rustdoc.
- `/re-cordis/rustdoc/<full-HEAD>/<crate>/index.html`: exact Rust API.
- `/re-cordis/markdown/<language>/<page>.md`: actual static Markdown files.
- `/re-cordis/llms.txt`: bilingual page index and retained essential contracts.
- `/re-cordis/search-index/<language>.json`: browser-side search indexes.

Page/navigation/search/LLM routes derive from the same canonical data.
Task navigation and Fumadocs UI are translated. Language switching preserves
the page. Relative canonical Markdown/README links become website links;
public source links use GitHub blob URLs pinned to the build revision.
Links, images and Markdown reference definitions targeting docs/plans or
docs/superpowers are removed before HTML, search and public Markdown generation.
Their descriptive link text is retained where useful; neither tree is exported
or listed in public provenance. Images for public sources use pinned raw URLs.
The repository's local canonical Markdown and engineering records remain intact.
Compatibility/releases/changelog retain explicit pinned source pages, with
translated sidebar labels. Historical evidence remains unchanged.

The local `../docs/llms.txt` remains checkout-relative. The public index reuses its
package/essential contracts and page routes; per-page Markdown preserves code
and adapts links. No `llms-full.txt` is generated: retrieve relevant pages from
the bounded bilingual index.

## Layout

The documentation frame fills the viewport, with a 64 px desktop header, a
fixed left sidebar and a right-hand TOC. The reading column grows up to 72rem
while the outer frame remains full-width. Mobile retains the Fumadocs drawer,
search and TOC popover. The layout follows the structure of
[shadcn docs](https://ui.shadcn.com/docs) and
[AI SDK docs](https://ai-sdk.dev/docs/introduction) using existing Fumadocs slots.

## Verification and deployment

`bun run check` runs route-adapter regressions, then inspects actual exported
locale/page routes, HTML links/anchors/assets, pinned source paths/revisions,
seven Rustdoc channels/assets, Markdown code preservation, native code-block
wrappers/copy controls and exact rendered code text, public LLM coverage,
source/Rustdoc fingerprints, private-document exclusion and English/Vietnamese
static search results. Crate package lists and Rustdoc inputs exclude both
internal trees; docs.rs builds crate documentation, not the website or root docs.
Browser checks additionally cover search, language switching, keyboard/mobile
navigation and deep-link reloads against the exported preview.

Workflow actions use major-version tags; macOS guide checks run on supported
macos-26. The Rustdoc artifact is transferred to the Linux site build. Rust and
documentation checks run on pull requests and pushes to main. Initial hosted
guide jobs passed while site checks exposed the compiler rendering distinction
above. Fresh main checks passed with the corrected checker, including the
deployed revision's [Rust CI](https://github.com/antruongnguyen/re-cordis/actions/runs/42204299211)
and [documentation CI](https://github.com/antruongnguyen/re-cordis/actions/runs/42204299299).
The [initial Pages workflow](https://github.com/antruongnguyen/re-cordis/actions/runs/42204868321)
succeeded, and the site, both locale indexes, public llms.txt and provenance
returned HTTP 200. This establishes deployment availability; live browser QA
acceptance remains separate.

After separate authorization to push reviewed commits, ensure the workflow is
available on the default branch. Set Settings → Pages → Source to GitHub Actions;
protect the `github-pages` environment with allowed branches/reviewers; set the
repository Actions variable `PAGES_DEPLOY_ENABLED=true`; dispatch `pages.yml` on
the reviewed main revision with `deploy=true`. A push alone does not deploy.
The initial deployment succeeded; retain the same manual gates for
subsequent refreshes. docs.rs builds from crates published to crates.io; no hosted docs.rs
release is established by preparing these unpublished candidate crates.

## Official references consulted

- https://www.fumadocs.dev/llms.txt
- https://www.fumadocs.dev/docs/deploying/static
- https://www.fumadocs.dev/docs/mdx/collections
- https://www.fumadocs.dev/docs/internationalization/next
- https://www.fumadocs.dev/docs/headless/search/orama
- https://www.fumadocs.dev/docs/integrations/llms
- https://www.fumadocs.dev/docs/manual-installation/next
- https://www.fumadocs.dev/docs/ui/translations
- https://nextjs.org/docs/app/guides/static-exports
- https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages
- https://docs.rs/about/builds
- https://docs.rs/about/metadata

Current Fumadocs built-in search uses ZBSearch despite the legacy `orama` URL
and compatibility module names. This site uses its supported static export
and browser client, with language-scoped JSON output.
