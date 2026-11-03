# HMR H2 implementation and narrow Include seam evidence

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Status: focused implementation verified; independent HMR review/final program
acceptance remain parent-owned gates.

## Inputs and checkpoints

- Initial code baseline: 47042d9e805fa558e69416c8ffb1030333b0c38b.
- H0 contract checkpoint: 5065adc.
- Verified initial loader batch checkpoint: bbef48b.
- TypeScript source pin: 35fdc969e4b6c48c48ed44717c7a018324e4dd88.
- Reused loader/core hosting, provider retention and latest-intent evidence;
  [exact H2/H3 contract](hmr-reload-contract.md) records the uncovered delta.
- No core production/API change, Node module loading, Rust recompilation,
  native ABI swapping or inferred Rust dependency graph was introduced.

## Executed failing evidence

Ignored external package target/hmr-reload-probe, stable and 1.85.0:

```text
second_refused=true first_catalog_changed=true old_instances_released=1
replacement_error=Some("unknown entry: b") remaining_entries=1
0 passed; 2 failed
```

The new loader and HMR tests first refused compilation because their new APIs
did not exist. A mistaken SourceDocument constructor and test cleanup Context
move were corrected separately; those are not behavioral regression evidence.

An additional synchronous commit-observer failure reproduced the initial
batch's early return after catalog publication. The correction schedules
unrelated current entries and carries original causes into Settlement.

After bbef48b, the required dropped-waiter probe inspected both diagnostics:

```text
later_settlement_has_original=false runtime_has_original=false
test dropped_batch_observer_error_remains_observable ... FAILED
```

Core sync emit returns its callback error directly; it does not invoke the
runtime diagnostic path (events.rs:749). The correction therefore reuses the
existing first-unobserved global loader slot, with no new state/queue. Both
live-entry and observer-removed-entry variants now retain the original cause
in later settlement, and a delivered settlement acknowledges it.

## Working behavior and bounds

FactoryReplacement type-erases compiled Plugin declarations. All candidate
names/contracts/config types and optional raw-source callbacks pass before
catalog publication or reload-owned teardown. The synchronous preflight
guard refuses same-loader mutation and disposal, resets on error/panic, and
releases before candidate capture destruction. Catalog records publish
together; declaration destructors and observers run outside framework borrows.
Current specs and allocation identity are rechecked while scheduling.

ReloadPlan has explicit factory-bearing/dependency-only nodes, reverse
dependency closure, supported cycles, exact replacement coverage and stable
declaration-order output. Bounds: 256 nodes, 1024 edges, 256 changed inputs and
256 candidates; names contain 1..=256 UTF-8 bytes. Existing loader bounds hold.

Gated tests prove exact old providers remain readable and alive through
consumer cleanup, fast unrelated replacement progresses during a slow gate,
and replacement resource acquisition follows old release. Config changes,
disable/remove, parent replacement, repeated re-enable init and owner disposal
supersede old intent. Nested core plugins rebuild once through an explicitly
declared host candidate. Failed post-unload apply retains source; later
candidate replacement recovers. Optional preflight executes before missing
services become ready, while the actual Plugin validator stays deferred.

Include forwarding awaits the exact loader batch and preserves source bytes,
data, provenance, journal, origin, transport and decoders. A later refresh uses
the replacement catalog. No file flush is requested by replacement.
The optional HMR forwarding method shares exact candidate validation.

## Focused verification

Explicit rustup tools are selected for each stable 1.97.1 and 1.85.0 run:
Cargo, rustc, rustdoc, cargo-clippy and clippy-driver; RUSTUP_TOOLCHAIN,
RUSTC, RUSTDOC and PATH point at that toolchain. Logs are local ignored
artifacts under target/hmr-h2.

- Loader: 9 new batch regressions; 85 total checks including its existing suite
  and Rustdoc checks after the observer-drop correction.
- HMR reload/nested: 17 integration tests with Include feature enabled; these
  totals exclude independent H1 watcher/path and H3 integration suites.
- Include: 2 new linked replacement regressions; 79 total checks including
  its existing suite and Rustdoc checks.
- One deterministic test performs 100 replacement cycles, verifies one live
  resource per step, restores Registry/effect baselines and releases captures.
- Strict all-target Clippy and warnings-denied Rustdoc cover loader, Include
  and HMR with Include enabled; unrelated H1 findings are handled by its owner.
- Parent owns shared guide/export checks, runnable H3 application, whole-workspace
  acceptance, independent review and milestone commits. This report does not
  claim those gates from focused tests.

Reproduction, from repository root after selecting explicit toolchains:

```sh
cargo test -p re-cordis-loader --locked
cargo test -p re-cordis-hmr --features include --test reload --test nested --locked
cargo test -p re-cordis-include --locked
cargo clippy -p re-cordis-loader -p re-cordis-include -p re-cordis-hmr \
  --all-targets --features re-cordis-hmr/include --locked -- -D warnings
cargo doc -p re-cordis-loader -p re-cordis-include -p re-cordis-hmr \
  --no-deps --features re-cordis-hmr/include --locked
```

Initial tests/validation logs are preserved separately from final runs;
observer-red.log, observer-drop-red.log and observer-drop-green.log record the
specific correction evidence.
