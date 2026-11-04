# Create an offline Rust application

[Tiếng Việt](../../vi/guides/create.md) | [English home](../README.md)

`re-cordis-create` generates one small application using the local unpublished
core, console logger and loader. Offline S0–S2 is independently accepted on
stable/1.85.0; see the [acceptance report](../../plans/2026-11-02-cordis-completion/evidence/create-acceptance.md).

## Generate and run

Use Rust 1.85 or newer and a checkout of this repository. Run the following
from its root; the parent directory of the destination must already exist,
and `../my app` must be absent or an empty ordinary directory.

```sh
cargo run -p re-cordis-create --offline --locked -- \
  my-app --path '../my app' --cordis-path .
cargo test --manifest-path '../my app/Cargo.toml' --offline
cargo run --manifest-path '../my app/Cargo.toml' --offline --locked
```

The generator itself uses only the standard library and starts no subprocess.
These explicit Cargo commands need their dependencies in the local cache.
If a dependency is missing, offline Cargo reports the failure; the generator
does not download it. The first application test command creates its lockfile.

For command help:

```sh
cargo run -p re-cordis-create --offline --locked -- --help
```

| Argument | Meaning |
|---|---|
| `NAME` | Cargo package name; starts with an ASCII lowercase letter, then lowercase letters, digits, `_` or `-`, at most 64 bytes. Rust/Cargo reserved names, Windows device names, and `re-cordis-core`, `re-cordis-logger`, `re-cordis-loader` are refused. |
| `--path PATH` | Destination; defaults to `NAME` relative to the working directory. Native paths can contain spaces and Unicode. |
| `--cordis-path PATH` | Required local repository checkout supplying core, logger and loader. |
| `-h`, `--help` | Print usage and exit successfully without generating files. |

Exit status is `0` for success/help, `2` for usage or validation errors, and
`1` for I/O errors. Errors identify the failing validation or file operation.
See the [Create API](../api/create.md) for library calls.

## What the application contains

The fixed file set is `Cargo.toml`, `src/main.rs`, `src/app.rs`, `README.md`
and `.gitignore`. Read the generated README and `src/app.rs` to customize it.
The [generator source](../../../crates/re-cordis-create/src/lib.rs) renders the
[bundled application](../../../crates/re-cordis-create/templates/basic/app.rs).

The manifest has its own `[workspace]` with resolver 3 so the application is independent of
an enclosing workspace. It uses Rust 2024/MSRV 1.85, is unpublished, and records
canonical absolute path dependencies to the three local Cordis crates.
It references no fictional crates.io release of `re-cordis`.
The checkout must retain this repository's manifest/layout spelling; validation
is not a general Cargo TOML parser or source audit. Canonical dependency paths
must be Unicode to fit Cargo TOML. Moving the checkout requires updating them.

The application drives a local `Runtime::run` from current-thread Tokio,
registers an owned scoped console writer, publishes and reads a typed service,
boots a programmatic loader entry, updates configuration and explicitly joins
shutdown. Operation errors, cleanup reports and runtime failures are all checked.
Its tests exercise resource release and error reporting. Extend that working
program through the [loader](loader.md), [services](services-and-scoping.md)
and [cleanup](effects-and-cleanup.md) guides.

## Distribution and template notice

The generated README retains the original MIT notice for the starter code;
the output still has exactly five files. Preserve that notice when redistributing
the template or substantial portions. Your additions can carry their own notices.
Cargo.lock does not pin the contents of the absolute path dependencies. Use a
reviewed checkout revision; changing it can change the next application build.
The [release proposal](../../release-readiness.md) would distribute only this
explicit-checkout tool initially. A registry-backed generator mode remains a
separate future decision, and no alpha package is available from R1.

## Destination and scope

The generator refuses an existing file, symlink or nonempty directory, and a
destination ending in `.` or `..`.
It renders all five files in a sibling staging directory before one rename,
and checks the destination again before committing. It reports failed staging
cleanup. Rendering failure places no template files at the final destination;
an empty-directory rename can still fail on a particular platform/filesystem.

The generator initializes no Git repository, installs no dependencies and
launches no application. It offers no remote template, downloader, archive,
cache, Include/HMR variant or package-manager option. Portable filesystem
operations do not guarantee confinement against hostile concurrent parent
replacement or durability through power loss. Runtime filesystem evidence is
limited to the tested macOS host; see the
[limitations](../../limitation.md#create-offline-scaffolding).
