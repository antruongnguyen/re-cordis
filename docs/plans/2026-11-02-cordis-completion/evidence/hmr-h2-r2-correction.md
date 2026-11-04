# HMR H2-R2 correction

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Baseline: `8b2252bafcb85b4ae6c38965ce89c26e7e03fbe7`, `feat/cordis-completion`.
Scope: loader collector, two regression variants, this evidence.
Parent owns shared documentation, commit and independent re-review.
See [independent finding](hmr-h2-rereview.md#h2-r2--an-older-slot-occurrence-overwrites-a-current-exact-batch-apply-failure).

## Trigger and correction

An abandoned older replacement leaves `a:observer-1-a` in the bounded
first-unobserved slot. A newly awaited replacement fails both applies.
The previous exact collector inserted that unrelated slot into its EntryId map,
overwriting `a:apply-1`. This also occurred when current observers succeeded
and the captured scheduling vector was empty.

The private collector now receives `None` for ordinary `settle()` and
`Some(captured)` for an exact replacement batch, including an empty vector.
Exact batches append the unrelated slot to their local rows after lifecycle
collection. `Rc::ptr_eq` still suppresses only the same captured slot occurrence;
EntryId, error text and source-object equality are not occurrence identity.

Production delta: **15 added, 10 removed lines**. Tests: **106 added lines**.
No core, public API, dependency, global history or worker changes.
Ordinary settlement retains its EntryId map and first-unobserved policy;
abandoned waiters retain only the existing bounded slot. A delivered exact
report consumes that slot and acknowledges pending updates as before.

The new regressions check exact entry/cause pairs and counts, downcast every
original cause to `io::Error`, and check later acknowledgement/runtime causes:

| Current observers | Exact report | Later settlement / runtime |
|---|---|---|
| Fail | 5: apply-1, observer-1-a, observer-2-a, apply-2, observer-2-b | Both apply causes only |
| Succeed | 3: apply-1, observer-1-a, apply-2 | Both apply causes only |

The fresh-slot R1 test still requires exactly four causes and rejects duplicate
delivery of its retained scheduling occurrence. Existing `final_wave` source
and dropped-batch/ordinary policy tests remain unchanged.

## Executed verification

Both saved independent occupied-slot probes and both new in-tree variants were
behaviorally RED on stable and 1.85.0 before the production edit: 0 passed,
2 failed, exit 101; all failures identified missing `a:apply-1`.

All final commands below passed on both toolchains. All five Rust tools
(cargo, rustc, rustdoc, cargo-clippy, clippy-driver) were resolved independently
with `rustup which --toolchain stable|1.85.0`. Matching `RUSTUP_TOOLCHAIN`,
`RUSTC`, `RUSTDOC` and leading PATH were set; separate build directories and
`RUSTDOCFLAGS=-D warnings` were used. Versions: stable rustc 1.97.1;
MSRV rustc 1.85.0. Native Rust verification ran through Cargo.
CodeGraph preceded indexed reads; RustRover VFS performed tracked edits.
The advertised IDE verify-after-edit resource was unavailable.

| Final check | Each toolchain |
|---|---:|
| Full loader, including unchanged final_wave 14 and batch 13 | 87 Rust + 2 Rustdoc = 89 |
| Focused replacement_batch | 13 passed, included above |
| HMR reload/nested with Include | 17 passed |
| Include linked_replacement | 2 passed |
| Original saved H2-R1 probe | 1 passed |
| Independent H2-R2 probe, including fresh-slot control | 3 passed |
| Loader/Include/HMR all-target strict Clippy | exit 0 |
| Loader/Include/HMR no-deps strict Rustdoc | exit 0 |

After selecting the explicit tools/environment above:

```sh
cargo test -p re-cordis-loader --locked --offline
cargo test -p re-cordis-loader --test replacement_batch --locked --offline -- --nocapture
cargo test -p re-cordis-hmr --features include --test reload --test nested --locked --offline
cargo test -p re-cordis-include --test linked_replacement --locked --offline
cargo test --manifest-path target/hmr-h2-correction/probe/Cargo.toml --locked --offline every_original_batch_observer_error_survives_concurrent_apply_failure -- --nocapture
cargo test --manifest-path target/hmr-h2-rereview/probe/Cargo.toml --locked --offline -- --nocapture
cargo clippy -p re-cordis-loader -p re-cordis-include -p re-cordis-hmr --all-targets --features re-cordis-hmr/include --locked --offline -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p re-cordis-loader -p re-cordis-include -p re-cordis-hmr --no-deps --features re-cordis-hmr/include --locked --offline
```

Ignored logs under `target/hmr-h2-r2-correction/`:
`probe-red-`, `regression-red-`, `regression-green-`, `loader-`,
`hmr-reload-`, `include-linked-`, `probe-r1-green-`, `probe-r2-green-`,
`clippy-`, `doc-`, each ending `stable.log` or `1.85.0.log`.
Every log contains its complete command/environment and exit code;
`tools.json` and `frozen-inputs.json` record tool paths and final checks.
Only the added test tail was formatted; the edited settlement region matches
stable Rustfmt and `git diff --check` passes.

## Frozen source and test SHA-256

- `crates/re-cordis-loader/src/lib.rs`:
  `30beb0cd13a1307db9500b87cd68e096ac84f2131742ce254844f54289d11b43`
- `crates/re-cordis-loader/tests/replacement_batch.rs`:
  `fa37f8c4618a017f51f7aa7f55e12775a3cac10b285cb098234750c32958a672`

Ready for narrow aggregation/acknowledgement re-review. H2 acceptance,
H1/H3 and the final shared matrix remain separate parent-owned gates.
