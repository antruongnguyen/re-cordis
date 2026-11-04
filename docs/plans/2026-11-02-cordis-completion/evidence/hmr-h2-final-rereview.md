# HMR H2 final scoped re-review: H2-R2

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Frozen revision: `61510b4b20035f0f5c72d74f5ff1e12555688ff5`.
Branch: `feat/cordis-completion`; clean at review entry.
Reviewed correction: `8b2252b..61510b4`.

**Final H2 specification and quality verdict: request changes for H2-R3.**
H2-R2 is addressed, including successful current observers; H2-R1 remains
addressed. The correction introduces duplicate delivery of a direct-update
occurrence through the same collector. H3 remains gated.

## Scope and carried assessment

Read repository instructions, [previous full H2 assessment](hmr-h2-rereview.md),
[R2 correction evidence](hmr-h2-r2-correction.md) and the
[exact-batch contract](hmr-reload-contract.md). Reuse the completed H2
graph/preflight/replacement/Include assessment in the previous review; do not
reopen accepted prerequisites, H1, H3, examples or the final shared matrix.

This review inspects only the mode-dependent aggregation/acknowledgement delta
and its two regressions. The collector uses `None` for ordinary settlement and
`Some(captured)` for exact batches, including an empty capture. An older
unrelated scheduling slot now accompanies current lifecycle rows. Fresh
reproduction confirms R2 returns all five causes with failing current observers
and all three with successful observers. The fresh-slot R1 control still passes.
Ordinary collection and bounded slot ownership are unchanged.

CodeGraph supplied indexed source/callers; RustRover VFS read evidence and
created only the assigned report/scratch probes. No tracked production/test file,
Git index, branch or commit was changed by this reviewer. The earlier complete
assessment carries forward, with R3 below the sole remaining actionable finding
identified by this scoped review.

## H2-R3: exact reports duplicate a direct-update occurrence

**Severity: Important (P2); request changes.**
Location: `crates/re-cordis-loader/src/lib.rs:2039–2044`, especially the new
`scheduled.push((failure, observed))`.

The global slot also retains failed `PendingUpdate` occurrences. For an
unobserved direct update, `join_updates` records its marker in that slot while
the lifecycle loop collects the same failure from `entry.work` at lines
1970–1977. The new exact branch only searches captured scheduling markers.
It therefore appends the already-collected direct failure as another row.
This is one occurrence, not two distinct failures with equal text.

Public reproduction uses two live factories, worker entry `a` and other entry
`b`. The worker registers one update listener that counts invocations and
returns an original `io::Error("one-direct-update")`. Drop the direct-update
waiter, then await the selected operation:

| Operation | Callback count | Returned cause rows |
|---|---:|---|
| Ordinary `settle()` control | 1 | `a:one-direct-update` |
| Exact `replace_factories(vec![])` | 1 | the same row twice |
| Exact replacement of the unrelated live `other` factory | 1 | the same row twice |

Both explicit stable and 1.85.0 produce these results. Every row's root cause
downcasts to `io::Error`. The two exact variants fail a one-row assertion; the
ordinary control passes. Against the previous loader source, the identical
three-case probe passes on both toolchains. Its scratch `lib.rs` is byte-for-byte
equal to `git show 8b2252b:crates/re-cordis-loader/src/lib.rs`, verified by bytes
and SHA-256. Other loader source/core code is unchanged in the correction delta.
The scratch manifest preserves the original dependency features.

This violates the contract's deduplication of the same error supplied by the slot
and changes failure counts for an otherwise unrelated successful replacement.
Preserve distinct R1/R2 scheduling/lifecycle occurrences while deduplicating a
slot occurrence already represented in lifecycle output. Do not collapse by
EntryId, message or original-source pointer alone. Retain ordinary settlement,
acknowledgement and bounded abandoned-waiter behavior.

Minimal regression names in the saved public probe:

- `empty_exact_deduplicates_direct_occurrence`
- `unrelated_exact_deduplicates_direct_occurrence`
- Control: `ordinary_deduplicates_direct_occurrence`

## Fresh checks and fingerprints

All five tools (Cargo, rustc, rustdoc, cargo-clippy and clippy-driver) were
independently resolved through `rustup which --toolchain stable|1.85.0`, with
one matching bin directory per toolchain. Matching `RUSTUP_TOOLCHAIN`, `RUSTC`,
`RUSTDOC`, leading PATH and `RUSTDOCFLAGS=-D warnings` were set. Native Cargo
used separate `target/hmr-h2-r2-review/build-<toolchain>` directories.
Stable rustc is 1.97.1; MSRV rustc is 1.85.0.

| Fresh check | Each toolchain |
|---|---|
| Saved unchanged H2-R1/R2 public probe | 3 passed; exit 0 |
| Loader `replacement_batch` + `final_wave` | 13 + 14 passed; exit 0 |
| New duplicate probe against frozen current source | 1 passed, 2 failed; exit 101 |
| Identical duplicate probe against previous loader source | 3 passed; exit 0 |

Commands use `cargo test --locked --offline -- --nocapture` with either the
probe manifest or `-p re-cordis-loader --test replacement_batch --test final_wave`.
The first stable duplicate-probe run used `--offline` to create its new lockfile;
the paired MSRV run and prior-source controls used `--locked --offline`.
An initial scratch dependency-path setup error was corrected before compilation;
it is not counted as behavioral evidence. No full matrix, Clippy, Rustdoc or
documentation checker was rerun here; those earlier runs remain attributed to
the correction worker/parent. `git diff --check` passes.

Ignored artifacts under `target/hmr-h2-r2-review/`:

- `probe-duplicate/{Cargo.toml,Cargo.lock,src/lib.rs}`: current-source reproducer.
- `probe-baseline/`, `loader-baseline/`: identical probe and previous-source control.
- `probe-<toolchain>.log`, `regressions-<toolchain>.log`,
  `duplicate-<toolchain>.log`, `baseline-<toolchain>.log`: commands, environment,
  complete output and exit codes.
- `tools-<toolchain>.json`, `frozen-inputs.json`: paths/versions, HEAD, results,
  all reviewed source/probe hashes and all fresh log hashes.

| Frozen file | SHA-256 |
|---|---|
| loader `src/lib.rs` | `30beb0cd13a1307db9500b87cd68e096ac84f2131742ce254844f54289d11b43` |
| loader `tests/replacement_batch.rs` | `fa37f8c4618a017f51f7aa7f55e12775a3cac10b285cb098234750c32958a672` |
| loader `tests/final_wave.rs` | `851ef8c009cd47072e4e19fd5bb74584922e46744a154c8abd6b72894fcb7d62` |
| New probe source (current and prior controls identical) | `897ed0bf1f8d07d2296d3601faa0a7181418baf23ee8af421d8e54901d3b68c0` |
| Prior loader control `src/lib.rs` | `e575b5256309539d0f95f9314def30c3cc5b371b8a7a8c784c230b25893264d6` |

Next gate: correct H2-R3 and narrowly re-review occurrence accounting, retaining
R1/R2 and ordinary-settlement regressions. H2 acceptance is not granted at this
revision; the completed earlier H2 assessment otherwise remains applicable.
