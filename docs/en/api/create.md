# Create offline scaffolding

[Tiếng Việt](../../vi/api/create.md) | [API index](README.md)

Source: [lib.rs](../../../crates/re-cordis-create/src/lib.rs).
CLI: [main.rs](../../../crates/re-cordis-create/src/main.rs).
Guide: [create an application](../guides/create.md).

`re-cordis-create` (`0.0.0`, unpublished, Rust 2024, MSRV 1.85) supplies
one synchronous standard-library-only generator and a binary of the same name.
It requires no Context, Runtime or framework host. S0–S2 implementation and
acceptance are tracked in the
[work package](../../plans/2026-11-02-cordis-completion/create.md).
The [Rust contract](../../plans/2026-11-02-cordis-completion/evidence/create-contract.md)
records the selected boundaries.

## `create`

Signature: `create(name: &str, destination: &Path, cordis_path: &Path) -> Result<PathBuf, CreateError>`.

The name starts with an ASCII lowercase letter, contains only lowercase
letters/digits/`_`/`-`, and is at most 64 bytes. Rust/Cargo reserved names
and Windows device names are refused. The separate destination preserves native
OS path arguments, including spaces and Unicode; the filesystem may reject other
byte sequences with an I/O error. Its parent must already exist; the destination must be
absent or an empty ordinary directory. Files, symlinks and nonempty directories
are refused, as are final `.`/`..` destinations.

The checkout is validated for the core, logger and loader package manifests
and their `src/lib.rs` files. It requires the literal current `[workspace]`
and `[package]`/package-name spelling; this is a layout check, not a general
Cargo TOML parser or source audit. Dependency paths are canonical absolute
Unicode paths, escaped as TOML strings. The application has its own `[workspace]` with resolver 3, Rust
2024/MSRV 1.85 and unpublished status. The fixed template writes exactly
`Cargo.toml`, `src/main.rs`, `src/app.rs`, `README.md` and `.gitignore`.

Rendering completes in a sibling staging directory before a single rename.
Destination checks run before rendering and before commit; a nonempty
destination race is refused by standard rename. Failed staging cleanup is
reported. The returned absolute path uses the canonical parent and identifies the
completed application directory.
An existing empty directory may still cause a platform-specific rename failure.

## `CreateError`

| Variant | Fields / contract |
|---|---|
| `Validation` | `String` message for invalid names, destination/checkout layout and missing inputs. |
| `Io` | `operation: &'static str`, `path: PathBuf`, `source: std::io::Error`; preserves the failing filesystem operation. |
| `Cleanup` | `original: Box<CreateError>`, `staging: PathBuf`, `source: std::io::Error`; preserves the original failure and failed cleanup, naming staging for manual removal. |

`exit_code(&self) -> u8` returns `2` for validation and `1` for I/O/cleanup.
Display supplies the CLI diagnostic. `Error::source` is absent for validation,
the I/O source for `Io`, and the original CreateError for `Cleanup`; the cleanup
I/O error is available through its `source` field. Missing parent/checkout/manifest
and identity mismatch are validation errors; other read/canonicalization failures
retain their I/O source.
Generate exact Rustdoc locally:

```sh
cargo doc -p re-cordis-create --no-deps --offline --locked
```

Open `target/doc/re_cordis_create/index.html` (adjusted for `CARGO_TARGET_DIR`).

## CLI

`re-cordis-create NAME --cordis-path PATH [--path PATH]` uses the same generator.
`--path` defaults to `NAME`; `--cordis-path` is required.
`-h`/`--help` prints usage without generation and exits `0`.
Unknown options and missing/duplicate arguments exit `2`; successful generation
exits `0`, validation exits `2`, I/O or cleanup exits `1`.

The binary parses native path arguments without requiring Unicode conversion.
It runs no Git, install or launch subprocess and performs no network operation.
There are no remote templates, archive/cache options, Include/HMR variants
or automatic dependency resolution. The explicit Cargo application commands
are in the [guide](../guides/create.md).

Portable staging/rename is not hostile-parent-swap confinement or a power-loss
durability protocol. Filesystem runtime evidence is macOS only; see
[limitations](../../limitation.md#create-offline-scaffolding).
