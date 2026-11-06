# D4 package-local parity fixtures — 2026-11-06

Status: focused fixture correction verified; complete integrated package and
registry validation remain separate. No publication, tag or deployment.

## Reproduction and cause

The controller's clean `26d54d2` core archive passes Cargo's package verification,
which builds the library, but its extracted test suite fails to compile six
`include_str!` references outside the package root. Preserved controller red
evidence is under `target/alpha-candidate-20261106/packages-red-26d54d2/`,
including `stable-archive-tests-re-cordis-core.log`, the actual `.crate`,
`archive-sha256.json` and `commands.json`.

A fresh worker reproduction at `c1198119b4e2c6dff9df14f940a180fc8824e208`
also packages successfully and fails extracted stable tests with the same six
missing-file errors. Logs: `target/core-archive-fixtures/red-package-stable.log`
and `red-archive-tests-stable.log` in the alpha-candidate-edit worktree.

CodeGraph traced the core Rust test expectations and the existing paired runner.
The Rust test consumes an embedded expected trace and optionally writes its actual
trace. TypeScript independently asserts against repository-root traces; the
runner then compares both actual outputs. Thus package-local copies do not
weaken the existing differential comparison.

## All-seven-package scan and minimal change

| Affected consumer | External traces | New package-local location | Retained root location |
|---|---:|---|---|
| core `tests/lifecycle_parity.rs` | 6 | `tests/fixtures/lifecycle_parity/` | `tests/parity/` |
| timer `examples/timer_parity.rs` | 4 | `examples/fixtures/timer_parity/` | `tests/timer_parity/` |
| loader `tests/loader_parity.rs` | 2 | `tests/fixtures/loader_parity/` | `tests/loader_parity/` |

These three consumers account for all twelve package-external literal embedded
trace paths. The remaining core GitHub fixtures and Create's five template
inputs are already package-local; logger, Include and HMR have no literal
`include_str!`/`include_bytes!` inputs. Other `include!`/module paths are
package-local. Create retains its documented separate checkout fixture.

Copy all twelve trace files byte-for-byte and change only the twelve Rust include
paths, with one formatter-required match-arm wrap. Preserve all root trace paths
and expected bytes. No runtime/API, dependency, manifest, lockfile, template,
upstream pin or runner changes; no symlinks or build script.

## Focused checks and actual archive boundary

Use explicit tools from the repository root:

```sh
source .github/scripts/rust-tools.sh stable
"$CORDIS_CARGO" package -p re-cordis-core --locked --offline
# Extract the actual .crate outside any Cargo workspace.
CARGO_TARGET_DIR="$PWD/target/core-archive-fixtures/archive-tests-stable" \
  "$CORDIS_CARGO" test --manifest-path "$cordis_archive_manifest" --locked --offline
```

The red extraction reports exactly six missing expected-trace files. For the
uncommitted correction, review packaging adds `--allow-dirty` solely to include
the reviewed test inputs; normal Cargo archive verification remains enabled.
No `--no-verify`, registry overrides or publication commands are used.
The actual review archive SHA-256 is
`e3b82a372d2458ff8d8cce10a7b425735f29ae3745fde8ac36c9d4bb56fef3dc`.
It is review evidence, not the final integrated candidate archive/checksum.

- Actual extracted core `cargo test --locked --offline` passes on stable 1.97.1
  and 1.85.0: 170 unit/integration tests and 17 Rustdoc checks, 187 per toolchain.
  All 55 extracted files remain byte-identical to the original archive after
  both suites; Cargo targets are outside the extracted tree.
- Core's six parity tests and loader's two parity tests pass on both toolchains.
  The timer parity example runs all four scenarios on both toolchains.
  Emitted Rust output is byte-identical to all twelve retained root expectations.
- All twelve local copies match the original root files and their committed
  pre-change bytes. `fixture-integrity.log` records each byte count and SHA-256.
  No trace expectation was regenerated or relaxed.
- `cargo package --list` covers all seven packages and includes every literal
  embedded input: core 11, Create 5, loader 2, timer 4; zero for the other three.
  This proves file inclusion only, not satellite normalized archives or registry
  dependency resolution.
- The three changed Rust files pass explicit stable rustfmt. Documentation
  regressions and diff checks accompany the correction.

Worker logs and extracted-manifest pointers live under
`target/core-archive-fixtures/`; package lists are `file-list-<crate>.log`,
focused logs are `focused-<crate>-<toolchain>.log`, and actual green archive
logs are `green-archive-tests-<toolchain>.log`. `archive-integrity.log` records
all-package input inclusion, extracted byte preservation and emitted/root trace
equality. These local logs are separate from controller evidence.

## Remaining gates

The controller repeats packaging from the clean integrated candidate and owns
the complete audit. Satellite archive/dry-run validation still requires actual
registry prerequisites; no successful satellite registry preflight is claimed.
Existing core/loader/timer paired runners retain their root fixture paths and
will catch divergence between Rust and independently root-checked TypeScript
outputs. Fresh TypeScript runs and the full shared matrix are controller-owned.
