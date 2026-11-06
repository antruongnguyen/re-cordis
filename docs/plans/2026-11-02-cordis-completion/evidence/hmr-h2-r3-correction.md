# HMR H2-R3 correction evidence

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Baseline: `61510b4b20035f0f5c72d74f5ff1e12555688ff5`, branch `feat/cordis-completion`.
Status: **focused correction verified and source/test frozen; independent review pending**.
This is not H2 or H0–H3 acceptance.

## Scope and reproduced failure

Read repository instructions, [H2-R1 correction](hmr-h2-correction.md),
[H2 re-review](hmr-h2-rereview.md), [H2-R2 correction](hmr-h2-r2-correction.md),
[final H2-R2 re-review](hmr-h2-final-rereview.md) and
[linked replacement contract](hmr-reload-contract.md).
Reuse accepted loader/Include and the carried H2 assessment.
Own only loader `src/lib.rs`, `tests/replacement_batch.rs` and this report.
Parent owns shared documentation, Git/index/commits and independent review.
H3 remained read-only; no core, public API, dependency or global history change.

Before production edits, ran the unchanged independent public probe at
`target/hmr-h2-r2-review/probe-duplicate/Cargo.toml` on both explicit toolchains.
Each produced **1 passed, 2 failed, exit 101**. The update callback ran once.
Ordinary settlement returned one original `a:one-direct-update` cause;
empty exact replacement and unrelated live-factory replacement each returned
that occurrence twice. All roots downcast to `io::Error`.

Added in-tree regressions before production edits. Both exact variants also
failed behaviorally on both toolchains: **0 passed, 2 failed, exit 101**.
Their first subcase waits for callback completion and admits an unpolled
settlement, pruning completed pending work before the exact batch. The
one-row assertion still found two rows with callback count one.
The equal-message distinct-cause control already passed before the fix
(**1 passed** per toolchain); it is compatibility evidence, not a RED claim.

Logs: `probe-red-<toolchain>.log`, `regression-red-<toolchain>.log` and
`distinct-control-red-<toolchain>.log` under `target/hmr-h2-r3-correction/`.

## Root cause and correction

The exact collector had only scheduling-observer markers when it decided
whether to append the first-unobserved slot. A direct update also has an
entry-owned `Shared` work handle; joining pending updates retains the same
failure in the slot, while the lifecycle loop reports it from `entry.work`.
The slot marker was absent from captured scheduling failures, so it appended
the same direct occurrence again.

A private `UpdateFailure` replaces the slot's two-field tuple. It keeps the
existing original failure and acknowledgement marker, plus an optional
shared `Work` identity. Direct failures store their actual work handle;
scheduling failures store `None`. That identity remains available when
`prune()` removes completed pending updates. The slot is still bounded to one
first-unobserved occurrence.

Each lifecycle-map row now carries optional work identity together with its
original error. A failed work row retains an unpolled handle and awaits a clone:
`Shared::ptr_eq` returns false for a consumed handle. Other lifecycle causes
carry `None`. Convergence overwrites/removes the error and identity together;
there is no ever-seen marker set that could suppress an older slot whose row
has since disappeared.

An exact batch suppresses slot append only when the existing captured observer
marker identifies that same occurrence, or the currently retained row's actual
`Shared` work identifies it. Distinct older direct failures, current apply
failures and current scheduling-observer failures remain separate. EntryId,
message and original-source pointer are not occurrence identity. Ordinary
settlement still uses its accepted first-unobserved map overwrite policy.
Removal acknowledgement uses the same private slot fields; teardown and
acknowledgement policy are unchanged.

Production delta: **57 added, 27 removed lines**. Tests: **197 added lines**.
The four earlier R1/R2 tests and all earlier test source are retained byte-for-byte;
`final_wave.rs` is unchanged. Reconstructed baseline source/test hashes match
the reviewed baseline; ignored `delta-stats.json` and `delta-*.patch` record
that verification without this worker accessing Git.

## Regression coverage and final verification

New tests:

- `empty_exact_batch_deduplicates_direct_update_occurrence`
- `unrelated_exact_batch_deduplicates_direct_update_occurrence`

Each runs both pending and callback-completed/pruned work. It checks one
callback, one exact original cause, subsequent one-row settlement and joined
disposal.

- `exact_batch_keeps_distinct_direct_apply_and_observer_occurrences_with_equal_messages`

This prunes an older direct error, then awaits a current failing replacement
and observer. All three roots have `same-message`, but their original
`io::ErrorKind` values are BrokenPipe, PermissionDenied and NotFound.
Exactly three rows survive; later settlement contains only the current
PermissionDenied apply failure. This rejects text/entry-based deduplication.

All final commands passed on **stable and 1.85.0**:

| Check | Each toolchain |
|---|---:|
| Full loader | 90 Rust tests + 2 Rustdoc checks = 92 |
| Focused replacement_batch + unchanged final_wave | 16 + 14 passed |
| HMR reload + nested with Include feature | 17 passed |
| Include linked_replacement | 2 passed |
| Original H2-R1 public probe | 1 passed |
| Original H2-R2 public probe, including fresh-slot control | 3 passed |
| Independent H2-R3 duplicate probe, including ordinary control | 3 passed |
| Loader/Include/HMR all-target strict Clippy | exit 0 |
| Loader/Include/HMR no-deps strict Rustdoc | exit 0 |

Focused loader checks are included in the full-loader total.
The four earlier R1/R2 exact-cause/older-slot/dropped-waiter expectations remain
green, together with unchanged final-wave first-unobserved tests.

All five tools—cargo, rustc, rustdoc, cargo-clippy and clippy-driver—were
independently resolved with `rustup which --toolchain stable|1.85.0`.
Each toolchain has one matching bin directory; `RUSTUP_TOOLCHAIN`, `RUSTC`,
`RUSTDOC` and leading PATH match it. Stable rustc is 1.97.1; MSRV rustc
is 1.85.0. Separate `CARGO_TARGET_DIR` directories and
`RUSTDOCFLAGS=-D warnings` were used. `tools.json` records paths/versions.
CodeGraph preceded indexed reads and source edits. RustRover VFS performed
tracked edits and formatted only the added test tail. The advertised IDE
`mcp-steroid://ide/verify-after-edit` resource was unavailable; native Cargo
compiler/tests, Clippy and Rustdoc provide the verification.

After selecting the explicit tools/environment above:

```sh
cargo test -p re-cordis-loader --locked --offline
cargo test -p re-cordis-loader --test replacement_batch --test final_wave --locked --offline -- --nocapture
cargo test -p re-cordis-hmr --features include --test reload --test nested --locked --offline
cargo test -p re-cordis-include --test linked_replacement --locked --offline
cargo test --manifest-path target/hmr-h2-correction/probe/Cargo.toml --locked --offline every_original_batch_observer_error_survives_concurrent_apply_failure -- --nocapture
cargo test --manifest-path target/hmr-h2-rereview/probe/Cargo.toml --locked --offline -- --nocapture
cargo test --manifest-path target/hmr-h2-r2-review/probe-duplicate/Cargo.toml --locked --offline -- --nocapture
cargo clippy -p re-cordis-loader -p re-cordis-include -p re-cordis-hmr --all-targets --features re-cordis-hmr/include --locked --offline -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p re-cordis-loader -p re-cordis-include -p re-cordis-hmr --no-deps --features re-cordis-hmr/include --locked --offline
```

Ignored final logs under `target/hmr-h2-r3-correction/`:
`loader-`, `regression-green-`, `hmr-reload-`, `include-linked-`,
`probe-r1-green-`, `probe-r2-green-`, `probe-r3-green-`, `clippy-`,
`doc-`, each ending `stable.log` or `1.85.0.log`.
Every log records the complete command/environment and exit code;
`frozen-inputs.json` records final source/test and matrix-log fingerprints.

## Frozen fingerprints

Source/test freeze was signalled to the parent before writing this report.

| File | SHA-256 |
|---|---|
| loader `src/lib.rs` | `4eddb5e1c81873b9f8fdde1185ea825b1d59d6fe4a6fcaa1ba7ea138d4a808f8` |
| loader `tests/replacement_batch.rs` | `ed690dd2b689435c7f90a52c06c13c7dbfa7342f4866f3684211b2e4418d59fc` |
| unchanged loader `tests/final_wave.rs` | `851ef8c009cd47072e4e19fd5bb74584922e46744a154c8abd6b72894fcb7d62` |

Ready for narrow occurrence-accounting, slot-lifetime and acknowledgement
re-review. H2 acceptance and all H1/H3/shared-matrix gates remain parent-owned.
