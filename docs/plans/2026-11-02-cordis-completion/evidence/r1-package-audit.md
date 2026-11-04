# R1 API/package distribution audit

Date: 2026-11-05 (Asia/Ho_Chi_Minh). Entry revision:
`e3f0f656aabfecb0088c5d63e1306231bfbe0188`, branch `feat/cordis-completion`.
**Verdict: usable through a complete local checkout; registry distribution is
not ready or authorized.** This audit supports an experimental release proposal,
not publication, stable API compatibility, remote Git installation or new
platform support. All seven crates remain `0.0.0`, `publish=false`, Rust 2024,
resolver 3, MSRV 1.85, MIT. No production API, Rust source, manifest, dependency,
feature, lockfile or generated application Rust source changed.

Raw evidence is under ignored `target/r1-audit/`: `commands.jsonl` records 107
argument-array commands, working directories, exits and durations; each command
has separate `.stdout`/`.stderr` files. `tools.json`, full metadata outputs,
`workspace-summary.json`, CodeGraph export snapshots, initial/final archives,
registry HTTP records and `final-summary.json` preserve the inspected inputs.
All command exits match the expected success or stated packaging guard.

Every Cargo stage resolves `cargo`, `rustc`, `rustdoc`, `cargo-clippy` and
`clippy-driver` with `rustup which --toolchain stable|1.85.0 TOOL`, confirms
their common toolchain `bin`, sets matching `RUSTUP_TOOLCHAIN`, absolute
`RUSTC`/`RUSTDOC` and leading `PATH`, and uses a toolchain-specific audit target
directory. Stable-only `cargo-fmt`/`rustfmt` are explicitly resolved in the same
way. No toolchain/component was installed and Homebrew Rust was not used.

| Tool | stable | 1.85.0 |
|---|---|---|
| Cargo | 1.97.1 (`c980f4866`) | 1.85.0 (`d73d2caf9`) |
| rustc / rustdoc | 1.97.1 (`8bab26f4f`) | 1.85.0 (`4d91de4e4`) |
| cargo-clippy / clippy-driver | 0.1.97 | 0.1.85 |
| cargo-fmt / rustfmt | rustfmt 1.9.0-stable | Not used |

Tool resolution/version inspection is not a fresh workspace Clippy or formatting
run; those execution gates are carried from R0 or recorded in parent R1 checks.

## Manifests, exports and dependency resolution

All seven package manifests inherit workspace version, edition, MSRV, license
and repository `https://github.com/antruongnguyen/re-cordis`; each explicitly
disables publication. That repository matches the configured Git origin.
There is no facade package. Every crate supplies a library; Create additionally
supplies the `re-cordis-create` binary. HMR's `reload` and `system_lifecycle`
examples require its `include` feature. Package lists retain source, tests,
examples and their referenced fixture/application modules; Create retains its
five bundled template inputs. Repository-wide guides are outside the archives.

CodeGraph was used first to inspect the public roots. The exported package
boundaries remain the accepted ones:

| Crate | Public surface inspected |
|---|---|
| core | `Runtime`, `Context`, `Plugin`/`Registry`/Fiber lifecycle, effects/completions, errors, typed events/listeners, identity, logging, accessors, services/dependencies, `Disposer`/`SetupError`, `CallbackError`/`TaskControl`, public typed `hooks` module |
| timer | Sealed `TimerExt` on `Context`; `TimerError`, `Sleep`, `IntervalControl`, `IntervalStream`, `Scheduled` |
| logger | `ColorMode`, `Alignment`, `LabelStyle`, `Timestamp`, `ConsoleOptions`; `render`, `register`, `register_stdout` |
| loader | `Config` aliases core `hooks::Value`; `Scope`, source/patch/location types and bounds, entry/factory IDs, catalog/replacement, groups/specs/snapshots, settlements/tasks and `Loader` |
| include | Parsing/document API, bounded file/entry/patch constants, locations/errors, provenance/ownership, options/reports/tasks/conflicts/JSON edits and `Include` |
| HMR | Loader `FactoryReplacement` re-export; reload plan/node and explicit watch/event/error/options/stats/token types |
| Create | `create(name, destination, cordis_path) -> Result<PathBuf, CreateError>`; validation/I/O/cleanup error variants and exit-code mapping |

This is an inventory against current contracts, not a semver compatibility
comparison against a prior published release. Local non-Send hosting, explicit
joined shutdown and documented satellite boundaries remain release constraints.

Actual `cargo metadata --format-version 1 --locked --offline --all-features`
on both toolchains resolves the same 74 packages and dependency/feature graph.
Stable default metadata and feature trees were also inspected. The resolved
graph contains 161 normal, one build and 15 dev dependency-kind edges, including
23 target-conditioned transitive edges. The sole build edge is
`generic-array -> version_check`. None of the workspace manifests declares a
build dependency or target-specific dependency. Transitive conditions include
Unix, Apple/aarch64, Android/Linux, Windows, WASI, wasm-family exclusions and
`tokio_unstable`; listing those edges is not execution on those targets.
No workspace package has a Cargo `custom-build` target or packaged `build.rs`;
the absence of workspace build scripts does not extend to registry dependencies.

| Crate | Normal local dependencies | Normal registry requirements/features |
|---|---|---|
| core | None | `futures-util ^0.3.31`, defaults off, `std,async-await`; `tokio ^1.48`, defaults off, `rt,sync` |
| timer | core | Same futures-util; Tokio defaults off, `rt,sync,time,macros` |
| logger | core | `time =0.3.36`, defaults off, `std,formatting` |
| loader | core | Same futures-util and core Tokio; `serde_json ^1` |
| include | core, loader | Same futures-util; `serde_json ^1`; `yaml-rust2 ^0.10`, defaults off |
| HMR | core, timer, loader; optional include | Same futures-util; Tokio defaults off, `rt,sync,macros` |
| Create | None | None; stdlib only |

All listed local requirements are currently path-only: metadata reports `*`
with no registry source. HMR alone declares crate features:
`default=[]`, `include=["dep:re-cordis-include"]`. Its isolated normal feature
tree excludes Include by default and includes it when enabled. Full workspace
metadata sees Include as its own workspace member even without the HMR feature;
that does not make the HMR edge active. The core-only normal feature tree proves
Tokio has only `rt,sync`; timer/application features seen in a workspace-wide
unified graph do not become production core requirements.

Dev edges were inspected separately. Core's GitHub examples use versioned
`axum`, `hmac`, `hyper`, `hyper-util`, `serde`/derive, `serde_json`, `sha2` and
Tokio `fs,io-util,macros,net,process,signal,test-util,time` in addition to its
inherited `rt,sync`. Timer adds Tokio `test-util`; logger adds `macros`; loader
adds `time,macros`; Include adds `macros`; HMR adds path-only logger,
`serde_json` and Tokio `time,macros,test-util`. Create has no dev dependencies.

Locked direct registry versions are futures-util 0.3.34, Tokio 1.54.1,
time 0.3.36, serde_json 1.0.154, yaml-rust2 0.10.4, axum 0.8.9,
hmac 0.12.1, hyper 1.12.0, hyper-util 0.1.21, serde 1.0.229 and sha2 0.10.9.
No resolved package declares an MSRV above 1.85; absent MSRV metadata is not
a compatibility guarantee. The locked host builds below and carried R0 matrix
provide execution evidence. This is not an unlocked future-version audit.
Third-party metadata includes MIT/Apache alternatives and other notices
(including BSD, Unicode, LLVM exception and Zlib); those dependency licenses
are not replaced by this repository's MIT declaration or bundled into its
source archives. A future binary/vendor distribution needs its own notice audit.

## Executed Cargo stages and package corrections

These distinct commands were exercised with matching explicit rustup tools:

```sh
cargo metadata --format-version 1 --locked --offline --all-features
cargo package -p PACKAGE --list --locked --offline --allow-dirty
cargo package -p PACKAGE --no-verify --locked --offline --allow-dirty
cargo package -p PACKAGE --locked --offline --allow-dirty
cargo publish -p PACKAGE --dry-run --locked --offline --allow-dirty
```

Actual argument arrays contain absolute Cargo paths. `--allow-dirty` permits
the authorized uncommitted documentation additions during this audit; it does
not bypass dependency or publication guards. Release packages must be recreated
from the final clean commit without that flag. Initial archives/logs were retained
before final package/documentation reruns.

| Package | Baseline listed files | Final listed files, both toolchains | Archive/verification result |
|---|---:|---:|---|
| core | 46 | 48 | Initial stable `--no-verify`: 0; final verified package build: 0 on both |
| timer | 21 | 23 | `--no-verify`: 101 on both final toolchains |
| logger | 10 | 12 | `--no-verify`: 101 on both final toolchains |
| loader | 25 | 27 | `--no-verify`: 101 on both final toolchains |
| include | 23 | 25 | `--no-verify`: 101 on both final toolchains |
| HMR | 18 | 20 | `--no-verify`: 101 on both final toolchains |
| Create | 13 | 14 | Initial stable `--no-verify`: 0; final verified package build: 0 on both |

Every package `--list` exits 0. That lists planned files; it does not construct
an archive or validate registry resolution. The five satellite `--no-verify`
attempts fail before archive construction, despite disabling the compilation
verification step. The exact error is:

```text
all dependencies must have a version requirement specified when packaging.
dependency `re-cordis-core` does not specify a version
Note: The packaged dependency will use the version from crates.io,
the `path` specification will be removed from the dependency declaration.
```

Every initial/final `cargo publish --dry-run` exits 101 at the preserved guard:

```text
`PACKAGE` cannot be published.
`package.publish` must be set to `true` or a non-empty list in Cargo.toml to publish.
```

No upload occurs and those dry-runs do not reach archive verification or
registry dependency checks. Only core and Create verified packages were actually
constructed and built from extracted archives, on stable and 1.85.0. Verification
builds are not all-target tests or evidence that a registry accepts publication.

Three demonstrated documentation/notice faults were corrected by the parent:

1. All seven initial lists and the constructed core/Create archives omitted
   the root MIT permission/copyright notice. Seven package-local `LICENSE`
   copies now match the root bytes exactly (SHA-256
   `d74d38d5593aca56b840d0010cfe6728e7bb5e7f915bc6e3edf481747e5ee5ef`).
2. Six packages had no README. Six short crate-local READMEs now explain the
   checkout-only distribution, relevant public contract, local Rustdoc command
   and where the unbundled guides live. Create's README gains the same distribution
   status. Cargo metadata automatically detects all seven `README.md` files;
   every final list includes both README and LICENSE. Local LICENSE links resolve.
3. Actual generated five-file applications declared MIT but omitted its notice.
   The exact root MIT text is now appended to the existing template README,
   preserving the file count, manifests, Rust source and dependency choices. Final
   Create archives on both toolchains contain that exact appendix and package
   LICENSE; downstream generated-file checks are recorded by the parent R1
   documentation verification, not claimed as a new execution here.

The baseline root license reads `Copyright (c) 2026 re-cordis contributors`.
The locally inspected TS pin
`35fdc969e4b6c48c48ed44717c7a018324e4dd88` carries MIT with
`Copyright (c) 2021-present Shigma`. This audit does not prove literal copying
or provide a legal conclusion. Before publication, refresh provenance review
for any literal derived code/docs/templates and preserve the relevant upstream
copyright/permission text where that review identifies it. Root MIT copies
alone do not establish upstream provenance closure.

Repository URLs match Git metadata but were not fetched; no claim is made
that these new local pages exist online. Crates have no manifest
`documentation`/`homepage` fields and no published docs.rs release was tested.
The checkout-oriented READMEs explicitly distinguish local guides from archive
contents, avoiding links to unbundled files as if they were present.

## Actual generated application and carried execution evidence

The delivered CLI generated separate actual applications for stable and 1.85.0
under `target/r1-audit/{toolchain} generated ứng dụng`, using an explicit canonical
checkout. Each original output had exactly `Cargo.toml`, `src/main.rs`,
`src/app.rs`, `README.md`, `.gitignore`; its recorded manifest has:

```toml
[package]
name = "r1-package-app"
version = "0.0.0"
edition = "2024"
rust-version = "1.85"
publish = false
license = "MIT"
[workspace]
resolver = "3"
[dependencies]
re-cordis-core = { path = "/Users/I756434/Projects/private/cordis-rust/crates/re-cordis-core" }
re-cordis-logger = { path = "/Users/I756434/Projects/private/cordis-rust/crates/re-cordis-logger" }
re-cordis-loader = { path = "/Users/I756434/Projects/private/cordis-rust/crates/re-cordis-loader" }
tokio = { version = "1.48", default-features = false, features = ["rt"] }
```

Both `cargo build --offline`, full locked metadata, `cargo test --locked --offline`
and `cargo run --locked --offline` exit 0; five tests each include 100 complete
lifecycle cycles. Generated-app package lists also exit 0, without constituting
archive or registry evidence. These runs preceded the MIT README appendix and
remain initial generated evidence; final generated notice/doc checks are separate.
No mocked generator, alternative template, downloaded dependency or rewritten
application source was used.

Paths make the checkout part of the application input. `Cargo.lock` does not
freeze those path dependency source bytes: edits affect later builds and moving
the checkout requires path updates. Reproduction/adoption therefore needs a
pinned full checkout plus the recorded lockfile/cache. No direct remote Cargo
Git dependency, `cargo install --git`, registry installation or published
Create invocation was tested. "Git consumption" here means obtaining a pinned
complete checkout and using its local paths.

The final R0 fingerprint comparison checks all 166 accepted inputs: only the
existing Create README and template README differ. The 13 new package
READMEs/licenses are additions outside that initial fingerprint set. All Rust source,
manifest, lockfile, executable fixtures and feature/dependency graph inputs
remain unchanged. Carry [R0 acceptance](r0-acceptance.md)'s 40-stage,
517-workspace-check-per-toolchain result within those same runtime boundaries;
it is not a fresh R1 full matrix or a count of the packaging probes above.

## Publication order and remaining decision gates

The normal/optional graph is acyclic:

```text
core -> timer, logger, loader
core + loader -> include
core + timer + loader + include(optional) -> HMR
Create: no Cargo dependency on the runtime packages
```

One practical order is core, then timer/logger/loader, then Include, then HMR.
Optional Include still needs a valid published version for an HMR registry
manifest; disabled default features do not make a path-only optional dependency
publishable. Create can be built independently, but its application template
still requires a compatible full checkout and core/logger/loader at use time.
Publishing it would not create a registry-backed template.

Core's versioned **dev-only** GitHub/Tokio dependencies do not add satellite
predecessors or consumer runtime requirements; this is why standalone core
archive verification succeeds while satellites fail. HMR's logger edge is
also dev-only. A tiny isolated stdlib Cargo probe in `target/r1-audit/dev-path-probe`
confirms Cargo permits and removes a versionless path-only dev dependency from
the normalized package manifest (`--no-verify` exits 0). That exception avoids
a mandatory runtime publication edge, but dropping logger would leave the
bundled HMR logger-using examples/tests without their declared dependency.
Use a tested, published versioned dev requirement before claiming independent
HMR all-target package verification; publishing logger before HMR makes that
verification practical. The probe is separate scratch evidence, not a rewritten
production manifest or a fabricated satellite package success.

The proposed `0.1.0-alpha.1` for the six runtime crates and stdlib offline Create
tool is coherent with this scope, subject to a separate user release decision.
No alpha version or publication flag is applied here. Before that release:

- Confirm names/ownership and API/MSRV/platform/semver policy. Exact-name
  read-only `https://crates.io/api/v1/crates/NAME` requests returned HTTP 404 for
  all seven names at 2026-11-04 17:08:00–17:08:23 UTC (2026-11-05 local), using
  an identifying user agent. This is a snapshot of absent API records, not a
  reservation, permission, ownership check or guarantee that names are available.
- Select tested internal published version constraints while retaining local
  paths for development; revalidate each normalized archive and dependency graph
  in publication order against real registry versions. Current versionless
  paths are intentional local distribution, not defects to hide with fake versions.
- Recreate clean release archives; verify their actual contents, notices,
  library/binary/examples/tests, default and optional HMR configurations, and
  MSRV consumption. The current satellite guards leave that evidence missing.
- Complete upstream provenance/notice review and any distribution-specific
  third-party notice work. Preserve the generated README's MIT notice.
- Keep Create's explicit-checkout contract, or separately implement and test
  versioned template dependencies only after compatible packages are published.
  Revalidate the offline template against each declared compatible checkout;
  its layout/name validation is not a version, authenticity or API audit.

No publishing, pushing, tags, API redesign, facade, platform expansion or remote
Create work occurred. Final guide/generated-project verification belongs to
the parent R1 evidence; Cargo ownership was explicitly returned before those runs.
