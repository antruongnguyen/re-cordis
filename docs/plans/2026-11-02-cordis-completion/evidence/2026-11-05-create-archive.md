# B1 Create archive fixture correction

Date: 2026-11-05 (Asia/Ho_Chi_Minh).
Branch: `feat/cordis-completion`.
Clean reproduction baseline and compatible runtime checkout:
`03d255029db6a57ccc17c0b1e26facdb491c058b`.

**Focused result: the actual corrected Create archive passes all 22 tests on
stable and Rust 1.85.0 when supplied the documented compatible checkout.**
Repository tests also pass without the override. This closes the demonstrated
fixture-location failure in the bounded selected work; it does not establish
registry readiness or approve publication.

## Contract and correction

[The scaffold test helper](../../../../crates/re-cordis-create/tests/scaffold.rs)
now reads `CORDIS_CREATE_TEST_CHECKOUT` with `std::env::var_os`, converts it to a
native `PathBuf`, and otherwise retains the existing two-parent repository
fallback. The environment is supplied by the caller before running Cargo; tests
do not mutate process environment. Child CLI/test processes inherit that value.
Invalid explicit checkouts still reach the existing production validation.
No tests were removed or skipped.

[The package README](../../../../crates/re-cordis-create/README.md) records the
variable, outside-workspace extraction and the exact compatible checkout pin.
A clean local clone at that pin was used for both archive-test and generated-app
verification. The fixture remains externally checkout-dependent; no runtime
crates are bundled into Create.

Only `tests/scaffold.rs` and the package README change inside the crate. Its
production Rust source, manifests, embedded template and LICENSE retain baseline
bytes. There is no production flag/API/dependency/template/version/feature
change. The package stays `0.0.0`/`publish=false`.

## Actual archive reproduction

Before editing, stable Cargo constructed and verified the clean baseline archive:

```sh
cargo package -p re-cordis-create --locked --offline
```

The command used the absolute rustup-resolved Cargo path and an external
`CARGO_TARGET_DIR` under ignored `target/b1-create/package-baseline`.
Its clean VCS metadata names the baseline revision. The archive was extracted to:

```text
/private/tmp/cordis-b1-create-hove55h5/baseline/re-cordis-create-0.0.0
```

That location and its ancestors contain no source workspace `Cargo.toml`.
Both toolchains directly executed the unchanged extraction with
`cargo test --manifest-path ABSOLUTE_EXTRACTED_MANIFEST --locked --offline`,
without an override. Each run compiled tests and exited 101: five unit tests
passed, 11 of 17 scaffold tests passed, and the following six failed:

- `complete_file_set_and_repeat_refusal`;
- `empty_directory_and_native_unicode_spaces_destination`;
- `cli_success_uses_argument_arrays_and_explicit_checkout`;
- `native_nonunicode_destination_obeys_host_filesystem_and_cleans_failure`;
- `stage_shaped_and_long_destinations_generate_complete_projects`;
- `other_cordis_crate_names_remain_available_for_projects`.

The helper selects `/private/tmp/cordis-b1-create-hove55h5`, which is missing
`Cargo.toml`, instead of a Cordis checkout. Consequently the non-Unicode macOS
case observes validation exit 2 before its expected filesystem exit 1. The
stage-shaped test also reports its single failed child test; this is part of the
same one failed scaffold test, not an additional unique test.

This reproduces the fixture assumption reported by [R1 review](r1-review.md)
and [R1 re-review](r1-rereview.md). Their nested parent-workspace discovery
attempts were separate setup failures. No such setup failure occurred here:
these are executed assertion failures from an admitted, actual archive suite.
The historical R1 evidence remains unchanged.

## Corrected archive and application checks

Stable Cargo constructed and verified the corrected archive using the authorized
uncommitted correction:

```sh
cargo package -p re-cordis-create --locked --offline --allow-dirty
```

Its VCS metadata records baseline revision plus `dirty=true`; it is not claimed
as a clean committed artifact. The exact Create input hashes are retained in
`target/b1-create/archives.json`. Relevant SHA-256 values are:

| Input/artifact | SHA-256 |
|---|---|
| Clean baseline .crate | `8cdd87af70135e388e43efc8e80bdb1b17101bd7a62af57cdc12d750c279171c` |
| Corrected .crate | `c0dbebd76d57697bbedbe0a9fe23d7ede4e753ec51bfb3db8f5b1d6ed9fc8cca` |
| Corrected tests/scaffold.rs | `37f882a5b79f5abaeb161ad76cfcb0c67659ead88b19e180f740fcfcef5ba93b` |
| Corrected package README | `6daffd0d6fbc09f1c796828855b2f3aa0f384e80b01be645df12cabc757242bc` |

The corrected archive was separately extracted outside source workspaces at
`/private/tmp/cordis-b1-create-hove55h5/corrected/re-cordis-create-0.0.0`.
For each toolchain, the archive tests received
`CORDIS_CREATE_TEST_CHECKOUT=/Users/I756434/Projects/private/cordis-rust/target/b1-create/checkout`;
that clean detached local clone is pinned to the exact baseline revision above.
Ordinary repository test runs explicitly removed the variable.

| Fresh stage | Stable | Rust 1.85.0 |
|---|---|---|
| Unchanged baseline archive suite without override | 5 unit + 11 scaffold pass; six scaffold failures; exit 101 | Same counts/failures; exit 101 |
| Corrected actual archive suite with pinned checkout | 5 unit + 17 scaffold pass; exit 0 | Same 22 passes; exit 0 |
| Repository Create suite without override | 22 pass; exit 0 | 22 pass; exit 0 |
| CLI built from corrected archive | Build and generation pass | Build and generation pass |
| Initial generated file set and MIT notice | Exact five files; root notice present | Exact five files; root notice present |
| Actual generated app build/test/run | All pass; five tests include 100 complete lifecycle cycles | Same checks pass |
| Generated app strict all-target Clippy | Pass | Pass |

The archive-built CLI received explicit argument-array `--cordis-path` and
generated each app outside source workspaces, including a native space/Unicode
destination. Before Cargo touched either app, its exact file set was
`Cargo.toml`, `src/main.rs`, `src/app.rs`, `README.md`, `.gitignore`.
The full repository MIT notice occurs in README. Initial offline app build
resolved its lockfile; subsequent test/run/Clippy commands used
`--locked --offline`. Clippy used `--all-targets -- -D warnings`.
The five generated source/document bytes remained unchanged after those commands.

Both baseline and corrected extracted archives contain exactly 14 files.
Every extracted file was compared byte-for-byte to its respective .crate before
and after all verification, with no extra files admitted. Only README,
tests/scaffold.rs and Cargo's VCS dirty metadata differ between the archives.
The baseline extraction was never edited; the corrected extraction was also
never edited. Cargo output stayed in separate target directories.

## Tool provenance and retained logs

For each of `stable` and `1.85.0`, Cargo, rustc, rustdoc, cargo-clippy and
clippy-driver were independently resolved with
`rustup which --toolchain TOOLCHAIN TOOL`. All resolved paths were absolute,
existing files in that toolchain's common bin directory. Matching
`RUSTUP_TOOLCHAIN`, absolute `RUSTC`/`RUSTDOC`, and that bin directory first in
`PATH` were supplied to each command. Stable Cargo/rustc/rustdoc are 1.97.1;
MSRV tools are 1.85.0. Clippy versions are 0.1.97 and 0.1.85 respectively.

Stable cargo-fmt/rustfmt were likewise resolved explicitly and verified to use
the same bin; rustfmt is 1.9.0-stable. The explicit stable
`cargo-fmt fmt --all -- --check` passes. No component/toolchain installation
or Homebrew compiler was used.

Supporting local artifacts live under ignored `target/b1-create/`:

- `commands.json`: 53 exact argument arrays, working directories, selected
  environments, expected/actual exits and stdout/stderr names; 51 exit-zero
  commands and the two expected baseline-suite exit-101 commands.
- `tools.json`: all resolved paths, version output and environments.
- `archives.json`: revision, clean fixture pin, exact input/archive hashes,
  archive VCS metadata, 14-file inventories and before/after byte checks.
- `generated.json`: exact initial generated file inventories/hashes and notice
  and unchanged-byte assertions for both toolchains.
- `summary.json`: parsed test output counts, command totals and byte-check result.
- `verify.py` and `corrected.py`: the local argument-array reproduction and
  verification harnesses, with per-stage `.stdout`/`.stderr` output.

The source/helper/package README were frozen after these focused checks.
Cargo/full-documentation/parity execution ownership was explicitly returned to
the controller for the complete shared matrix. This evidence adds no fresh
whole-workspace, full documentation checker, parity, other-platform or registry
result. Current [release readiness](../../../release-readiness.md) remains
local/Git checkout consumption only. No commit, push, tag or publication was
performed by this worker.
