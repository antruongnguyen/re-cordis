# HMR H2 independent acceptance after H2-R3

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Frozen revision: `0e8d306b25b54600f15aac79514d45abf5431497`.
Branch: `feat/cordis-completion`; clean at review entry and after fresh tests.
Reviewed correction: `61510b4..0e8d306`.

**Final H2 specification and quality verdict: accepted.**
H2-R3 is corrected, and H2-R1/R2 remain corrected. No new actionable finding
was identified in this narrow review. This closes the H2 review gate for H3;
H1/H3 acceptance and the final shared matrix remain separate.

## Scope and carried assessment

Read AGENTS.md, the [completed H2 assessment](hmr-h2-rereview.md),
[H2-R2 final re-review](hmr-h2-final-rereview.md),
[R3 correction evidence](hmr-h2-r3-correction.md) and
[linked replacement contract](hmr-reload-contract.md).
The completed H2 graph, preflight, replacement/race, cleanup/resource and narrow
Include forwarding assessment carries forward. Those areas and accepted
loader/Include prerequisites were not reopened.

Fresh review covers occurrence accounting, slot/work lifetime and
acknowledgement, the three added regressions, and matching English/Vietnamese
API wording in the correction. CodeGraph supplied current source/call paths;
RustRover VFS read saved public probes and wrote only this report and assigned
scratch. Native Cargo provides the explicitly selected toolchain checks below.
The reviewer made no production/test, Git index, branch or commit changes.

## R3 correction and quality

The slot now retains `UpdateFailure { failure, observed, work }`.
Direct failures store the actual shared work; synchronous scheduling failures
store no work. Both `join_updates` and `prune` keep an unconsumed cloned handle
after observing completion. The single first-unobserved slot remains bounded;
it does not retain a new pending task or introduce failure history.

Every lifecycle map row carries its error and optional work together.
The collector awaits a clone, leaving the identity handle available for
`Shared::ptr_eq`. Work failure insertion records that handle; outer/instance
readiness and rejected errors replace it with `None`; successful convergence
removes the whole row. Thus deduplication tests the occurrence in the current
output row, not an ever-seen entry or a stale separate identity set.

Exact mode remains `Some(scheduled)`, including an empty vector. A slot is
suppressed only by its existing observer marker or by the same shared work in
the lifecycle row. Different work, observer and apply occurrences survive even
with the same EntryId and displayed message. The saved public R3 probe now
returns one original I/O cause for one callback in all three modes:
ordinary settlement, empty exact replacement, and unrelated live-factory
replacement.

Ordinary settlement retains its existing map overwrite policy. Delivery still
takes the slot and acknowledges completed pending updates; direct waiters retain
their existing observation marker. Pruning drops observed slots, and removal
uses the same marker/error fields as before. The added Work is held only after
completion and adds no independently running work. Existing overlapping-update,
dropped-settlement, teardown and capacity checks pass.

The two new same-occurrence tests cover both pending and callback-completed,
pruned updates, then check later settlement and joined disposal. The third
retains three original `io::Error` causes with identical text but different
kinds: BrokenPipe (older direct), PermissionDenied (current apply), and NotFound
(current observer). Later settlement contains only the current apply cause.
This is meaningful protection against both duplicate delivery and overbroad
deduplication.

## Fresh independent checks

All five tools—cargo, rustc, rustdoc, cargo-clippy and clippy-driver—were
independently resolved using `rustup which --toolchain stable|1.85.0`.
Each toolchain's paths share one bin directory. Commands set matching
`RUSTUP_TOOLCHAIN`, `RUSTC`, `RUSTDOC`, leading PATH,
`RUSTDOCFLAGS=-D warnings` and separate `CARGO_TARGET_DIR`.
Stable rustc is 1.97.1; MSRV rustc is 1.85.0.

| Fresh command | stable | 1.85.0 |
|---|---:|---:|
| Saved R3 public duplicate probe | 3 passed; exit 0 | 3 passed; exit 0 |
| Saved R1/R2 public probe | 3 passed; exit 0 | 3 passed; exit 0 |
| Loader replacement_batch + final_wave | 16 + 14 passed; exit 0 | 16 + 14 passed; exit 0 |

The R1/R2 probe returns four fresh-slot causes, five occupied-slot causes with
failing current observers, and three occupied-slot causes with successful
current observers. Both current apply causes remain in every exact report;
later settlement and runtime diagnostics retain the apply causes.

Commands after selecting the environment above:

```sh
cargo test --manifest-path target/hmr-h2-r2-review/probe-duplicate/Cargo.toml --locked --offline -- --nocapture
cargo test --manifest-path target/hmr-h2-rereview/probe/Cargo.toml --locked --offline -- --nocapture
cargo test -p re-cordis-loader --test replacement_batch --test final_wave --locked --offline -- --nocapture
```

Full outputs and exit codes were inspected. The deliberate contained preflight
panic is followed by its passing test; no test failed. `git diff --check
61510b4..0e8d306` also passes. No full workspace, HMR/Include matrix, Clippy,
Rustdoc, example or documentation-checker rerun is claimed here. Those checks
remain attributed to the correction worker/parent and the carried H2 evidence.

Ignored artifacts under `target/hmr-h2-r3-review/`:

- `run_review.py`: exact paired runner.
- `tools-{stable,1.85.0}.json`: all five resolved paths and versions.
- `r3-probe-`, `r1-r2-probe-`, `regressions-` logs for each toolchain:
  commands, selected environment, full output and exit code.
- `frozen-inputs-{stable,1.85.0}.json`: unchanged HEAD, reviewed source/probe
  hashes, results and log hashes.

## Frozen fingerprints

HEAD and all recorded inputs matched before/after both fresh runs.
The unchanged public R3 probe hash matches its prior failing review.

| File | SHA-256 |
|---|---|
| loader `src/lib.rs` | `4eddb5e1c81873b9f8fdde1185ea825b1d59d6fe4a6fcaa1ba7ea138d4a808f8` |
| loader `tests/replacement_batch.rs` | `ed690dd2b689435c7f90a52c06c13c7dbfa7342f4866f3684211b2e4418d59fc` |
| unchanged loader `tests/final_wave.rs` | `851ef8c009cd47072e4e19fd5bb74584922e46744a154c8abd6b72894fcb7d62` |
| saved R3 public probe `src/lib.rs` | `897ed0bf1f8d07d2296d3601faa0a7181418baf23ee8af421d8e54901d3b68c0` |
| unchanged HMR `src/reload.rs` | `64ed969a10b166a192feb1f34b63deafb4577abb974133651410016df8dc6b4e` |
| unchanged Include `src/lib.rs` | `7381718f2a5c05c7c30f36f726b75dedc58fef919bcd0629597e832fd104e345` |

H2's linked, already-compiled factory scope and documented Rust boundaries
remain unchanged. This acceptance makes no claim of runtime source compilation,
native code swapping, automatic nested declaration discovery, or full H0–H3
delivery.
