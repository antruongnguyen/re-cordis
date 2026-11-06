# Documentation channels

[Tiếng Việt](../../vi/guides/documentation.md) | [English home](../README.md)

Use the guides for application contracts and Rustdoc for exact signatures,
generic bounds, trait implementations and public fields. Both must describe
the same source revision and feature configuration.

All seven experimental `0.1.0-alpha.1` packages are published on crates.io;
exact-version docs.rs Rustdoc is verified. Release source and Create's compatible
checkout are `84fa9bdf0eb4e631fce792bf163494b0a61aeba8`.
The development documentation site is public on GitHub Pages,
with [English](https://antruongnguyen.github.io/re-cordis/en/docs/) and
[Vietnamese](https://antruongnguyen.github.io/re-cordis/vi/docs/) guides and
same-revision Rustdoc. Its deployed build revision can differ from frozen
release source. Use the registry instructions in [getting started](../getting-started.md)
or the installed explicit-checkout CLI in [Create](create.md).

## Development guides

`docs/en/` and `docs/vi/` are the canonical Markdown sources. The website builds
these files directly, preserving matching pages and executable examples.
Internal engineering plans and design records stay in the checkout. Public
pages, search and Markdown downloads omit links to those records; crate packages
and Rustdoc do not import them. The website uses a full-width documentation frame
with a fixed sidebar and desktop table of contents.

Development guides may change without a package-version change. The public site
follows its deployed revision, recorded in
[build provenance](https://antruongnguyen.github.io/re-cordis/build-provenance.json).
Check that revision before comparing a saved guide with newer checkout code.

The static project-site layout uses `/re-cordis/en/docs/` and
`/re-cordis/vi/docs/`. Each rendered page provides its source and Markdown links.
The exported `/re-cordis/llms.txt` index and `/re-cordis/markdown/` files use site
paths. The separate [checkout AI index](../../llms.txt) uses paths relative to
`docs/` and needs the checkout to resolve them.

See the [website instructions](../../../website/README.md) for building,
previewing and refreshing GitHub Pages. Rust and documentation CI check pull
requests and pushes to main; the initial manual Pages deployment succeeded.
Preparing or previewing an export does not deploy it. Later documentation
refreshes require a separate manual deployment from the reviewed main revision.

## Exact Rustdoc from the checkout

From the repository root, resolve the stable Rust tools through rustup and
generate documentation for all seven packages and their dependencies, including
HMR's optional Include adapter:

```sh
CORDIS_DOC_TOOLCHAIN=stable
CORDIS_DOC_CARGO="$(rustup which --toolchain "$CORDIS_DOC_TOOLCHAIN" cargo)"
CORDIS_DOC_RUSTC="$(rustup which --toolchain "$CORDIS_DOC_TOOLCHAIN" rustc)"
CORDIS_DOC_RUSTDOC="$(rustup which --toolchain "$CORDIS_DOC_TOOLCHAIN" rustdoc)"
CARGO_TARGET_DIR=target/site-rustdoc \
RUSTUP_TOOLCHAIN="$CORDIS_DOC_TOOLCHAIN" \
RUSTC="$CORDIS_DOC_RUSTC" RUSTDOC="$CORDIS_DOC_RUSTDOC" \
PATH="$(dirname "$CORDIS_DOC_CARGO"):$PATH" \
"$CORDIS_DOC_CARGO" doc --workspace --all-features --locked
```

Open `target/site-rustdoc/doc/re_cordis_core/index.html`; the other crate
directories use the same underscore naming convention. Use `1.85.0` instead of
`stable` to generate documentation with the minimum supported compiler.
Dependencies must be available; add `--offline` only after restoring the locked
dependencies locally. `cargo doc` generates files and publishes nothing.

Include dependency documentation in this site bundle so Rustdoc's links to
dependency traits and signatures have local targets. This documentation build
does not add runtime acceptance evidence for those dependencies or platforms.

The website build takes this Rustdoc directory through `CORDIS_RUSTDOC_DIR`.
Its [API pages](../api/README.md) link to the matching crate under
`/re-cordis/rustdoc/{build_revision}/{crate}/index.html`, where `build_revision`
is the recorded full Git revision and `crate` is an underscore directory name
such as `re_cordis_hmr`. Build guides and Rustdoc from the same committed
checkout; do not reuse Rustdoc output from a different revision.

The full documentation build includes HMR's `include` feature. Applications
still need to enable that feature explicitly for `watch_include` and
`replace_include`; HMR's default features remain empty. See the
[HMR API contract](../api/hmr.md) for the feature-dependent methods.

## Published release documentation

[docs.rs builds](https://docs.rs/about/builds) generate Rustdoc from packages
released on crates.io. All seven exact `0.1.0-alpha.1` pages are verified;
use the [package links](../api/README.md#published-rustdoc) for released signatures.
HMR's [docs.rs metadata](https://docs.rs/about/metadata) exposes `watch_include`
and `replace_include` without changing its empty application defaults.
Hosted documentation does not expand tested Apple Silicon macOS runtime scope.

The [frozen source snapshot](https://github.com/antruongnguyen/re-cordis/tree/84fa9bdf0eb4e631fce792bf163494b0a61aeba8)
identifies the published package set and compatible Create checkout. Its guides
and package READMEs retain candidate-preparation statements from before upload;
those statements describe historical state. Current main guides add registry
installation and current availability after that frozen source. They do not
alter published packages. Use exact docs.rs versions rather than `latest` when
looking up an older application's API.

The Fumadocs site is a separate artifact, with guides and bundled Rustdoc from
its recorded build revision. A documentation commit, website build or registry
upload does not deploy Pages. Read public build provenance to identify the live
snapshot; subsequent refreshes retain the manual deployment gate.
