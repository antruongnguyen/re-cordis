# re-cordis

A Rust framework for applications whose plugins, services, and configuration
change at runtime, based on the lifecycle model of
[Cordis](https://github.com/cordiverse/cordis).

re-cordis combines typed plugin registration, dependency injection, scoped
resources, and explicit asynchronous cleanup. It is designed for long-lived
applications that need to reconfigure components while preserving ownership
and resource lifetimes.

[English documentation](https://antruongnguyen.github.io/re-cordis/en/docs/) ·
[Tài liệu tiếng Việt](https://antruongnguyen.github.io/re-cordis/vi/docs/) ·
[API reference](https://antruongnguyen.github.io/re-cordis/en/docs/api/) ·
[Changelog](CHANGELOG.md)

## Status

The core and six companion crates are implemented and tested together.
The workspace requires **Rust 1.85 or newer** and uses edition **2024**.

The seven packages are published on [crates.io](https://crates.io/users/antruongnguyen)
at **`0.1.0-alpha.1`**, with [exact-version Rustdoc](https://docs.rs/re-cordis-core/0.1.0-alpha.1/re_cordis_core/).
Use exact version pins. The API is experimental: intentional breaking changes
require release notes and a new coordinated alpha version.
See [release readiness](docs/release-readiness.md) for the approved contract and
current distribution requirements.

## Packages

| Package | Purpose |
|---|---|
| [`re-cordis-core`](crates/re-cordis-core) | Runtime and context, typed plugins and services, dependency injection, events, effects, and diagnostics |
| [`re-cordis-timer`](crates/re-cordis-timer) | Scoped timeouts, sleep, intervals, streams, throttle, and debounce |
| [`re-cordis-logger`](crates/re-cordis-logger) | Console output with owned writers, color policy, UTC timestamps, and visible I/O errors |
| [`re-cordis-loader`](crates/re-cordis-loader) | Typed catalogs, configuration loading, structural groups, and source reconciliation |
| [`re-cordis-include`](crates/re-cordis-include) | Bounded configuration files, patches, source locations, and persistence |
| [`re-cordis-hmr`](crates/re-cordis-hmr) | File watching and replacement of linked factories, with optional Include integration |
| [`re-cordis-create`](crates/re-cordis-create) | Offline application scaffolding from a compatible local checkout |

Structural groups are part of the loader. Applications can select the
companion crates they need; HMR's Include adapter is an opt-in feature.

## Getting started

Add `re-cordis-core = "=0.1.0-alpha.1"` to your application's dependencies and
follow the [registry quick start](docs/en/getting-started.md) for a complete app.

From a complete compatible checkout, run the integrated lifecycle example:

```sh
cargo run -p re-cordis-hmr --example system_lifecycle --features include --locked
```

The example composes all six runtime libraries and exercises configuration
changes, failure recovery, and joined shutdown. The
[system lifecycle guide](docs/en/guides/system-lifecycle.md) explains the
application and its file-watching mode.

For application setup, start with the [quick start](docs/en/getting-started.md)
or use [Create](docs/en/guides/create.md) to generate a small application.
Install the CLI with `cargo install re-cordis-create --version '=0.1.0-alpha.1' --locked`.
Create requires an explicit `--cordis-path` checkout at release source
`84fa9bdf0eb4e631fce792bf163494b0a61aeba8` and does not download, install, or
launch application dependencies. Its generated application remains
`0.0.0` with `publish=false`.

## Design

- **Typed contracts.** Plugins, configuration, service keys, and callbacks
  express their requirements through Rust types.
- **Scoped ownership.** Plugins own their child instances, effects, listeners,
  and other registered resources.
- **Explicit hosting.** The framework runs on a local Tokio host and supports
  non-`Send` resources and futures.
- **Joined cleanup.** Awaited disposal reports completion of asynchronous
  teardown; dropping a handle is not a substitute.
- **Runtime reconfiguration.** Dependency changes and configuration updates
  use defined lifecycle states, retained provider bindings, and explicit
  failure recovery.

TypeScript Cordis is the behavioral specification. Rust APIs use ownership,
traits, typed keys, and explicit adapters to represent those concepts.
Transparent JavaScript Proxy/decorator compatibility and full upstream
behavioral equivalence are not promised.

Filesystem and watcher behavior has been verified on Apple Silicon macOS.
Broader platform acceptance, native binary hot loading, and registry-backed
Create templates are outside the current scope. See
[limitations](docs/limitation.md) and [design decisions](docs/decision.md).

## Documentation

The [public documentation site](https://antruongnguyen.github.io/re-cordis/)
provides English and Vietnamese guides, API contracts, and Rustdoc from the
same build revision. Check its build provenance: the site changes only after a
manual deployment. Published package source is frozen at
`84fa9bdf0eb4e631fce792bf163494b0a61aeba8`; current guides include subsequent
documentation updates. Candidate-preparation statements in that source snapshot
are historical. The canonical [English](docs/en/README.md) and
[Vietnamese](docs/vi/README.md) sources cover
setup, concepts, examples, troubleshooting, and the public API. The
[documentation map](docs/README.md) also links contributor and engineering
references.

Generate a local API reference:

```sh
cargo doc --workspace --all-features --no-deps --locked
```

See [website development](website/README.md) for the bilingual documentation
site and its local preview. For AI-assisted development, use the
[public reading index](https://antruongnguyen.github.io/re-cordis/llms.txt)
or the [checkout reading index](docs/llms.txt) with this checkout.

## Development

Run checks from the repository root:

```sh
cargo test --workspace --all-features --locked
cargo fmt --all --check
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
python3 -B tests/docs/test_check.py
```

Maintain compatibility with both stable Rust and Rust 1.85.0. Contributor
instructions are in [AGENTS.md](AGENTS.md); the
[roadmap](docs/todo.md) tracks delivery work. Documentation changes follow
the [documentation checks](docs/README.md#maintaining-these-documents).
CI runs on pull requests and pushes to `main`; GitHub Pages deployment remains
manual.

The isolated M0 research package and Rust/TypeScript comparison fixtures have
their own reproduction instructions in the
[engineering documentation](docs/README.md#engineering-references).

## License

[MIT](LICENSE). Notices for adapted Cordis/cosmokit material and quoted upstream
evidence are retained in [UPSTREAM-NOTICES.md](UPSTREAM-NOTICES.md).
