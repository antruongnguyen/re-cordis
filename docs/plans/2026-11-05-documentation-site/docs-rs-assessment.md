# docs.rs preparation assessment — 2026-11-05

## Scope and baseline

Assessed checkout: branch `feat/cordis-completion`, starting revision
`94907cbea555e05cacaaf35a4d49a84ba9a503a4` (actual HEAD at dispatch).
The documentation-site delivery preserves
all seven crate versions at `0.0.0`, `publish=false`, production APIs, path
dependencies, default features, Cargo lockfiles and the five-file Create template.
No crate publication, release tag, registry dependency conversion, docs.rs upload
or GitHub Pages deployment is part of this assessment.

The canonical [English guide](../../en/guides/documentation.md) and
[Vietnamese guide](../../vi/guides/documentation.md) explain the documentation
channels. Engineering evidence remains separate from reader navigation.

## Official guidance consulted

Fetched the current official pages on 2026-11-05:

- [Builds](https://docs.rs/about/builds): docs.rs automatically documents crates
  released on crates.io, runs sandboxed nightly builds with blocked network
  access, discovers package README files by Cargo rules, and makes `docsrs` cfg
  available only to the final crate's Rustdoc invocation. The page reported
  `rustc 1.101.0-nightly (708c94ef0 2026-11-03)` at inspection time; this is a
  changing hosted environment, not a locally installed or tested toolchain.
- [Metadata](https://docs.rs/about/metadata): `features` adds selected Cargo
  features, while `all-features`, `no-default-features`, compiler arguments and
  target overrides are independent options. The default landing target is
  Linux; documentation target selection is not runtime acceptance evidence.

No nightly-only `doc_cfg`, `docsrs` conditionals, build-script bypass, custom
Rustdoc argument or target override is needed for the current source.

## Package review and minimal change

CodeGraph was consulted before source exploration. Crate-level comments not
returned by its symbol excerpts were read separately. HMR's feature-gated
`FileWatcher::watch_include` and `ReloadPlan::replace_include` were traced to
their actual source and callers; the existing bilingual HMR API contract
describes the same optional feature surface.

| Package | Existing onboarding and requirements | docs.rs preparation |
|---|---|---|
| `re-cordis-core` | Hosted local Runtime example; explicit typed APIs, cleanup/errors, local non-Send handles; README supplies checkout and Rustdoc instructions | Keep defaults |
| `re-cordis-timer` | Owner/host requirements in crate docs; TimerExt example and README state enabled Tokio time driver and joined owner cleanup | Keep defaults |
| `re-cordis-logger` | Crate docs identify synchronous scoped output; README explains owned writer errors, UTC/color policy and console example | Keep defaults |
| `re-cordis-loader` | Hosted Catalog/Loader crate example; README and comments explain retained source identity, immediate mutation and joined disposal | Keep defaults |
| `re-cordis-include` | Hosted refresh/cleanup crate example; README states bounded synchronous I/O, YAML limits and macOS-only runtime evidence | Keep defaults |
| `re-cordis-hmr` | Crate docs and README identify `include` opt-in, empty defaults, host/teardown requirements and external restart; reload/system examples already require `include` | Select only `include` for docs.rs |
| `re-cordis-create` | Crate docs describe offline five-file atomic-visibility scope; README states checkout requirements, archive-test fixture, exit codes and limits | Keep defaults |

Every crate has a package-root `README.md` and `LICENSE`, so Cargo/docs.rs README
auto-discovery already applies. Core also carries its adapted-material
`UPSTREAM-NOTICES.md`; existing crate and generated-starter notices are retained.
Repository guides are deliberately described as checkout files, not bundled
crate-archive contents. No README or Rust source changes were necessary.
No crate has a package-root `build.rs`; no docs.rs-specific workaround is added.

The sole Cargo change is:

```toml
[package.metadata.docs.rs]
features = ["include"]
```

This gives future HMR Rustdoc visibility to `watch_include` and
`replace_include`. Explicit selection avoids silently enabling future features
through `all-features = true`. Application defaults remain empty. Other packages
have no current optional documentation surface requiring an override. The
metadata does not make the unpublished path-dependent package set releasable;
that remains a separate coordinated release decision.

## Local verification gates

Cargo/full documentation execution belongs to the integration owner and is
serialized with other Rust commands. This subtask performed source/configuration
inspection and environment probes only. The integration delivery evidence must
record actual command results before claiming local build success.

Focused stdlib assertions passed: parsing the old/new HMR manifest yields equal
configuration after removing the sole new docs.rs metadata table; EN/VI code
fences are identical; owned files have no trailing whitespace. Scoped
`git diff --check` passed. These checks do not substitute for Cargo/full guide
execution. Environment command output is saved in
`target/documentation-site/docs-rs-environment.log`.

For each of `stable` and `1.85.0`, resolve `cargo`, `rustc`, `rustdoc`,
`cargo-clippy` and `clippy-driver` through rustup. Set matching
`RUSTUP_TOOLCHAIN`, absolute `RUSTC`/`RUSTDOC`, and lead `PATH` with the resolved
toolchain's `bin` directory. With dependencies deliberately restored, use the
resolved Cargo for these focused gates:

```sh
cargo doc --workspace --all-features --no-deps --locked --offline
cargo test -p re-cordis-hmr --doc --features include --locked --offline
cargo test -p re-cordis-hmr --doc --no-default-features --locked --offline
```

Set `RUSTDOCFLAGS="-D warnings"` for these gates. Use distinct target directories
for stable/MSRV verification. The `--no-deps` gate supplies local package
documentation compilation evidence; it is not the selected website artifact.
For the website's same-revision bundle, generate stable all-feature workspace
Rustdoc with dependency documentation into `target/site-rustdoc/doc`:

```sh
cargo doc --workspace --all-features --locked --offline
```

Pass `CORDIS_RUSTDOC_DIR=../target/site-rustdoc/doc` from `website/` to its build.
During integration, the owner observed source-generated unresolved dependency
prose hrefs such as `futures_core::future::TryFuture`, `TryFuture::Error` and
`TryFuture::Ok` in `LifecycleTask` when using `--no-deps`. An experimental
resolved-stable build with dependencies in
`target/site-rustdoc-with-deps/doc` removed those raw unresolved hrefs. The
chosen site bundle therefore includes dependency Rustdoc so generated trait
and signature links have local targets. The strict site checker continues to
validate those targets; no broad unresolved-link exemption is introduced.
Including dependency documentation does not expand runtime acceptance.

Inspect exported HMR HTML for `watch_include` and `replace_include`, and verify
links to the exact revision directories under `/re-cordis/rustdoc/`.
The package's docs.rs metadata itself is not consumed by ordinary `cargo doc`;
the explicit local feature flag exercises the selected documentation surface.

The unchanged production behavior permits reuse of accepted runtime evidence.
The full shared Rust/parity matrix is not required merely for metadata/prose.
The bilingual guide still requires the existing documentation regression and
full guide checks on stable and 1.85.0, executed by the integration owner.

## Environment boundary

Read-only probes on the local workstation found:

- Host: Darwin 25.6.0, `arm64` (`aarch64-apple-darwin`).
- Installed toolchains include stable and `1.85.0`; both have only the native
  `aarch64-apple-darwin` target installed. No nightly toolchain is installed.
- No `cargo-docs-rs`/`cargo-docsrs`, Docker or Colima executable was found.
- Podman 5.8.1 is present. `podman machine list --format json` reports its Linux
  machine stopped. `podman info` exits 125: connection refused to its local
  socket. No VM was started and no container/image/toolchain was installed.

Thus no docs.rs sandbox, Linux or nightly documentation build was available to
execute within the inspected active environment. Local macOS stable/MSRV
Rustdoc is the scoped available gate. Linux/nightly sandbox compatibility,
packaged registry dependency resolution and an actual hosted docs.rs build
remain outstanding release evidence. A Linux website build alone supplies no
runtime filesystem/watcher acceptance.

## Version and channel relationship

Development guides compile from `docs/en` and `docs/vi`; exact Rustdoc is
bundled from the same complete committed checkout under
`/re-cordis/rustdoc/{full_git_revision}/{crate}/`. The exported site records the
revision; rebuild after committing new documentation so pinned source links
exist at that revision. Do not combine stale Rustdoc with current guides.

Before any future package release, freeze a guide snapshot at the same revision
as the coordinated package set, record its feature/MSRV/platform boundaries,
and link it to each exact docs.rs package version only after the hosted build
succeeds. Keep development and release channels visibly distinct. No invented
release URL or `latest` link is introduced while publication is disabled.
