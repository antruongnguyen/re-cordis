# Documentation channels

[Tiếng Việt](../../vi/guides/documentation.md) | [English home](../README.md)

Use the guides for application contracts and Rustdoc for exact signatures,
generic bounds, trait implementations and public fields. Both must describe
the same source revision and feature configuration.

All seven crates currently remain unpublished, unstable `0.0.0` with
`publish=false`. This checkout prepares a development documentation site;
it does not establish a deployed site or a released docs.rs build. Continue
using the local-checkout instructions in [getting started](../getting-started.md)
and [Create](create.md).

## Development guides

`docs/en/` and `docs/vi/` are the canonical Markdown sources. The website builds
these files directly, preserving matching pages and executable examples.
Internal engineering plans and design records stay in the checkout. Public
pages, search and Markdown downloads omit links to those records; crate packages
and Rustdoc do not import them. The website uses a full-width documentation frame
with a fixed sidebar and desktop table of contents.

Development guides follow the checkout and may change without a package-version
change. Check the build revision before comparing a saved guide with newer code.

The static project-site layout uses `/re-cordis/en/docs/` and
`/re-cordis/vi/docs/`. Each rendered page provides its source and Markdown links.
The exported `/re-cordis/llms.txt` index and `/re-cordis/markdown/` files use site
paths. The separate [checkout AI index](../../llms.txt) uses paths relative to
`docs/` and needs the checkout to resolve them.

See the [website instructions](../../../website/README.md) for building,
previewing and activating GitHub Pages. Preparing or previewing an export does
not deploy it; owner-side activation and deployment remain separate steps.

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

## Future released documentation

[docs.rs builds](https://docs.rs/about/builds) automatically generate Rustdoc
from packages released on crates.io. The Fumadocs site is a separate artifact;
it is not uploaded to docs.rs. Current package publication is disabled and the
[coordinated alpha release](../../release-readiness.md) remains a proposal.
There is no current re-cordis docs.rs release to link to.

Before a release, prepare an immutable guide snapshot from the same source
revision as its package set. Link that snapshot to each package's exact
released version on docs.rs only after its hosted build succeeds. Retain an
explicit route back to development documentation, with both channels labelled.
A moving development page or docs.rs `latest` URL cannot identify the API of
an older application reliably.

HMR's prepared [docs.rs metadata](https://docs.rs/about/metadata) selects
`include` for documentation visibility, without changing application defaults.
Local stable/MSRV Rustdoc checks establish local documentation compatibility.
They do not establish a hosted docs.rs build or Linux/nightly sandbox success.
Likewise, a website build or cross-platform documentation compilation does not
expand the existing macOS-only runtime filesystem/watcher evidence.
