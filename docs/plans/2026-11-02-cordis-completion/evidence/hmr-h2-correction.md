# HMR H2-R1 correction evidence

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Status: **focused correction verified and frozen; independent delta review pending**.
This is not H0–H3 acceptance.

## Scope and inputs

Branch: `feat/cordis-completion`.
Clean starting checkpoint: `56de5f400aba39feef29b25470bec08551b19246`.

Read AGENTS.md, [resume instructions](hmr-resume/README.md),
[confirmed review](hmr-reload-review.md), [reload contract](hmr-reload-contract.md)
and [implementation evidence](hmr-reload-implementation.md).
Reuse accepted loader/Include evidence; no broad prerequisite or core audit.
The only owned tracked changes are loader `src/lib.rs`, loader
`tests/replacement_batch.rs` and this report. Parent owns shared docs,
milestone commits and independent review. No core, public API, dependencies,
watcher, Include implementation, Git index or commits changed by this worker.

CodeGraph supplied current source and call paths before edits. RustRover VFS
performed tracked edits. Cargo verification used explicitly resolved rustup
tools; native Rust tests were run through Cargo. The advertised IDE
`mcp-steroid://ide/verify-after-edit` resource was unavailable, so compiler,
test and Clippy results provide verification.

## Reproduced before production edits

Restored [saved public source](hmr-resume/reload-review-probe.rs.txt) and
[manifest](hmr-resume/reload-review-probe.toml.txt) to ignored
`target/hmr-h2-correction/probe/`. Selected only
`every_original_batch_observer_error_survives_concurrent_apply_failure`.

Both stable and 1.85.0 compiled and failed behaviorally: 0 passed, 1 failed,
exit 101. Exact batch had `a:observer-a`, `b:apply-2`; later settlement and
runtime diagnostics had only the two apply causes. `observer-b` disappeared
from every observation channel.

The new integrated exact-batch regression also failed on both toolchains,
printing the same two-row report instead of the hand-written four-row
expectation. Its helper's Rust borrow-lifetime mistakes were corrected before
these behavioral RED runs; compilation failures are not counted as evidence.

Logs: `probe-red-{stable,1.85.0}.log` and
`regression-red-{stable,1.85.0}.log` under
`target/hmr-h2-correction/`.
The initial integrated RED run also contained an overly broad dropped-waiter
assertion; its final expectation was narrowed to the accepted policy below.

## Root cause and surgical correction

There were two loss points in exact batch delivery:

1. Ordinary settlement inserts the bounded first-unobserved error into a map
   keyed by EntryId. Thus the scheduling observer error for a could replace
   the lifecycle apply error for a in the exact batch's intermediate report.
2. Batch aggregation skipped every captured scheduling error when any existing
   report row had that EntryId. Thus b's distinct observer cause was discarded
   because the report already contained b's apply cause.

Entry identity does not identify an error occurrence. Error messages and
original source pointers are also not used as occurrence identity.

A private `settle_with_failures` collector shares the existing settlement
barrier. Each captured batch scheduling failure carries a private
`Rc<Cell<bool>>` occurrence marker. If retained in the existing global slot,
that slot uses the exact same marker. The collector skips inserting only the
slot occurrence already owned by this batch, identified by `Rc::ptr_eq`.
It then converts ordinary per-entry lifecycle failures to rows and appends
every captured batch scheduling failure. Original error objects/source chains
remain intact.

The loader still owns one first-unobserved global slot. A dropped exact waiter
does not add an error history or retain every observer failure globally.
Ordinary `settle()` passes an empty captured batch and preserves its accepted
map/first-unobserved behavior. In this two-entry abandoned-waiter case, its
first report remains `a:observer-a`, `b:apply-2`; a subsequent settlement
returns the two apply errors after the retained observer is acknowledged.

An intermediate broad append implementation changed ordinary settlement:
`final_wave::dropped_settlement_after_older_error_retention_keeps_failure_visible`
then returned two rows instead of retaining the older first-unobserved cause
as one row. Full loader verification caught this. The final collector keeps
that existing regression unchanged and passing. All-four cause retention
belongs to the exact returned batch; it is not a new ordinary-settlement
history promise.

Production delta: **21 added, 24 removed lines**. Test delta: **146 added
lines**, containing one shared original-I/O-error downcast helper and two tests.
Only added tests were formatted; existing formatting remains unchanged.

## Final checks on both explicit toolchains

stable: rustc 1.97.1 (8bab26f4f 2026-07-14).
MSRV: rustc 1.85.0 (4d91de4e4 2025-02-17).

For each toolchain, all five tools were resolved independently with
`rustup which --toolchain stable|1.85.0`: cargo, rustc, rustdoc,
cargo-clippy and clippy-driver. `RUSTUP_TOOLCHAIN`, `RUSTC`, `RUSTDOC`
and the leading PATH bin directory matched that toolchain.
`tools.json` records every path/version under the ignored evidence directory.
Each run used a separate `CARGO_TARGET_DIR`, `--locked --offline` and
warnings-denied Rustdoc.

| Verification | Each toolchain |
|---|---:|
| Full loader, including direct-update/acknowledgement regressions | 85 Rust tests + 2 Rustdoc = 87 |
| Focused replacement_batch rerun | 11 passed |
| Saved H2-R1 public probe | 1 passed |
| HMR reload + nested with Include feature | 17 passed |
| Include linked_replacement | 2 passed |
| Loader/Include/HMR all-target strict Clippy | exit 0 |
| Loader/Include/HMR no-deps strict Rustdoc | exit 0 |

The 11 batch tests are included in the full loader total, not additional
unique tests. The exact-batch regression downcasts all four original sources
to `io::Error` and checks the exact entry/cause pairs; the four-row count
also rejects duplicate delivery of the retained slot occurrence. It checks
later acknowledgement and original apply causes in Runtime diagnostics.
The dropped-batch regression verifies the bounded first-observer policy and
later acknowledgement. Existing live/removed-entry abandoned-waiter and
ordinary direct-update cases remain green.

All final logs are under `target/hmr-h2-correction/`:
`loader-`, `regression-green-`, `probe-green-`, `hmr-reload-`,
`include-linked-`, `clippy-`, `doc-` with `stable.log` or `1.85.0.log`.
No verbose logs are committed. Commands are recorded at the top of each log.

Reproduction after selecting the explicit tools/environment above:

```sh
cargo test -p re-cordis-loader --locked --offline
cargo test -p re-cordis-loader --test replacement_batch --locked --offline -- --nocapture
cargo test --manifest-path target/hmr-h2-correction/probe/Cargo.toml \
  --locked --offline every_original_batch_observer_error_survives_concurrent_apply_failure -- --nocapture
cargo test -p re-cordis-hmr --features include --test reload --test nested --locked --offline
cargo test -p re-cordis-include --test linked_replacement --locked --offline
cargo clippy -p re-cordis-loader -p re-cordis-include -p re-cordis-hmr \
  --all-targets --features re-cordis-hmr/include --locked --offline -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p re-cordis-loader -p re-cordis-include -p re-cordis-hmr \
  --no-deps --features re-cordis-hmr/include --locked --offline
```

## Frozen correction inputs

SHA-256 of final owned source/test files:

- `crates/re-cordis-loader/src/lib.rs`:
  `e575b5256309539d0f95f9314def30c3cc5b371b8a7a8c784c230b25893264d6`
- `crates/re-cordis-loader/tests/replacement_batch.rs`:
  `8e4d94b3f417dd0d342e242b2b70340f9bcb1fb1d8b93e5fb81f1430bd60000e`

Source/test delta is ready for independent review against the starting
checkpoint. Whole HMR acceptance, H3 integration and shared documentation
checks remain parent-owned.
