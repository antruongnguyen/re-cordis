# Create S1 implementation checkpoint

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Baseline: clean `8124fed` on `feat/cordis-completion`.
The commit containing this report is the verified implementation checkpoint;
independent review and the full shared matrix are still required for S2.

## Delivered

The stdlib-only unpublished `re-cordis-create` library/binary renders one
bundled basic Cargo application. Its API, exact five files, destination policy,
name/path rules and error/exit contracts are recorded in
[the S0 contract](create-contract.md) and paired [guide](../../../en/guides/create.md).

The app has its own resolver-3 workspace with explicit local core/logger/loader
dependencies, current-thread Tokio, local Runtime, a typed greeting plugin/service,
programmatic loader configuration and a real update followed by joined shutdown.
Failure paths also join root cleanup. No prerequisite production source changed.

## Executed evidence

Fresh sol/high implementation owns the new crate; parent separately extended
and executed the existing documentation checker. The parent checked the frozen
source hashes against the worker's verified files before committing.

| Check | stable 1.97.1 | 1.85.0 |
|---|---:|---:|
| Generator package tests | 17 (4 unit + 13 CLI/filesystem) | 17 |
| Package all-target check, strict Clippy, warnings-denied Rustdoc, build and format | Pass | Pass |
| Actual external app build/test/run, strict all-target Clippy and format | Pass; 5 tests | Pass; 5 tests |
| Parent paired docs/export/link checks | 33 pairs, 2 Create exports | Same |
| Parent existing executable documentation | 13 snippets + quick-start app | Same |
| Parent generated app: actual CLI/help, five files, Cargo metadata, build/test/run, Clippy/format | Pass; 5 tests | Pass; 5 tests |
| Documentation checker regressions | 8 passed (toolchain-independent) | Same check |

The five generated tests are `hundred_cycles_join_update_and_release_service_writer_and_catalog`,
`run_uses_updated_config_and_releases_writer`,
`root_report_and_runtime_diagnostics_retain_original_cleanup_cause`,
`successful_loader_disposal_can_still_have_runtime_cleanup_errors`, and
`failed_log_setup_still_drains_partial_resources`. They verify weak service/writer
capture release, Registry/effect baselines, real source update and original error
causes. One test repeats 100 lifecycles; that is not 100 distinct test cases.
Root CleanupReport and Runtime diagnostics remain separate channels; successful
loader disposal alone can coexist with a reported original cleanup failure.

Both executed binaries emitted, after actual service assertions:

```text
[I] app hello from Cordis
[I] app updated greeting
[I] app service: updated greeting
shutdown complete
```

The parent generated an independent external application under a temporary path
with spaces and Vietnamese Unicode. Cargo metadata proved its own workspace root,
version 0.0.0, Rust 2024/MSRV 1.85, publish=false and exact canonical dependency
paths. No source modification was used to make the generated project compile.

## Regressions and corrections before freeze

Initial RED cases established missing scaffolding behavior. Focused inspection
then found and reproduced recursive token expansion when an explicit checkout
path contained `{{loader}}`, normalized trailing `/.` admission, checkout-as-file
error classification and Cargo-reserved `build`. Corrections use one-pass fixed
template expansion, raw final-component validation, directory validation and
the reserved-name guard. These are pre-review implementation corrections,
not independent acceptance findings.

Actual filesystem write/rename failures prove no partial final project and
preservation of a concurrent winning file. Cleanup failure preserves both causes
and names the owned leftover staging path. Native Unicode/space, quote/backslash,
control and template-token paths are covered. macOS rejects the non-Unicode path
probe with EILSEQ; the generator retains that I/O error and removes staging.
No Linux/Windows execution is claimed.

## Commands, tools and logs

Every Rust execution resolved cargo, rustc, rustdoc, cargo-clippy and clippy-driver
using `rustup which --toolchain stable|1.85.0`, with matching RUSTUP_TOOLCHAIN,
RUSTC, RUSTDOC and leading toolchain PATH. Stable rustc/Cargo are 1.97.1;
MSRV rustc/Cargo are 1.85.0. No Homebrew compiler supplied this evidence.

Targeted package commands: `cargo check -p re-cordis-create --all-targets`,
`cargo test -p re-cordis-create`, strict all-target Clippy, `cargo doc`
with `RUSTDOCFLAGS=-D warnings`, `cargo build` and `cargo fmt --all --check`;
Cargo resolution used `--locked --offline`.
External application commands use `--manifest-path` as a process argument,
`--offline`, and `--locked` after the first resolution.
The parent ran `python3 -B tests/docs/check.py --toolchain stable` and
`--toolchain 1.85.0`, plus `python3 -B tests/docs/test_check.py`.

Logs are under `target/create-20261104/`: `implementation/package-{stable,1.85.0}.log`,
`implementation/external-{stable,1.85.0}.log`, `implementation/generation.log`,
`docs-milestone-{stable,1.85.0}.log`, and `docs-{red,green}.log`.
The implementation directory also retains `scaffold-red.log`,
`input-edge-red.log`, `input-edge-green.log` and `reserved-name-red.log`.
Repeated parent/worker checks are repeated evidence, not additional unique tests.

## Acceptance boundary

S2 still needs fresh astra/high independent review, any verified correction/re-review
batches, [the shared matrix](../acceptance.md), and a final acceptance commit.
[All 14 source dispositions](create-source-map.md) distinguish Cargo/native
representations from ten unselected remote cases and two Yarn exclusions.
No downloader stub, publication, push, CI/platform expansion, B1, R0 or R1 is delivered.
