# HMR H0–H3 acceptance

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
**Verdict: delivered and independently accepted within the documented Rust scope.**
Final executable revision: `55a55d1` on `feat/cordis-completion`.
The commit containing this report closes documentation and status only.

## Scope and checkpoints

Resumed the user's clean pause checkpoint `56de5f4`; no newer work was discarded.
Reuse accepted core, timer, logger, loader/group and Include evidence. No core,
timer, logger or Include production file changed after that pause. HMR adds
explicit file watching, linked compiled-factory replacement and optional Include
composition; source changes report RestartRequired.

| Milestone | Commit / evidence |
|---|---|
| Approved H0 design and watcher gate | `5065adc`; [H0 review](hmr-h0-review.md) |
| H2 exact-error corrections | `8b2252b`, `61510b4`, `0e8d306`; [R1](hmr-h2-correction.md), [R2](hmr-h2-r2-correction.md), [R3](hmr-h2-r3-correction.md) |
| Final H2 acceptance | code `0e8d306`, record `e44754e`; [independent review](hmr-h2-accepted-review.md) |
| Retained Include adapter and offline example | `77b50b5`; [implementation](hmr-include-implementation.md) |
| Paired guides and 53 source dispositions | `3f5e562`; [source map](hmr-source-map.md) |
| Read-error ingress correction | `55a55d1`; [correction](hmr-overflow-correction.md) |
| Final independent H1/H3 and documentation review | [accepted review](hmr-watch-adapter-review.md) at `55a55d1` |

H2-R1/R2 preserve distinct exact-batch scheduling/lifecycle causes, including an
older retained slot. H2-R3 deduplicates the same direct-update work occurrence,
including completed/pruned work, without changing ordinary first-unobserved
settlement policy. H1-R4 retries a read failure lost at ingress and coalesces it
only after queue admission. All findings are closed; original RED evidence and
unchanged independent public probes are retained in their linked reports.

## Delivered behavior and oracles

- Watch only explicit regular-file paths, with local hosted callbacks, bounded
  ingress/history/debounce, exact tokens, original errors and joined producer
  shutdown. Native rename/remove/recreate and first-registration Rescan are tested.
- `watch_include` retains the already-admitted IncludeTask outside callback
  cancellation. Stop suppresses follow-up; watcher/ancestor disposal joins that
  same work without a new Include call. Active plus retiring registrations count
  against quota until completion is observed and callback captures are destroyed.
- Returned Include errors and every Report failure are retained once in bounded
  WatchEvent history. Ordinary core cleanup failures remain in Runtime diagnostics.
  The saved draft's mistaken channel assertion was corrected using a direct
  Include reproducer; the original cleanup cause and actual resource release
  remain asserted. The delivered H3 suite has ten tests: eight saved cases,
  successful held/coalesced refresh, and the direct channel control.
- Explicit ReloadPlan graphs select already-linked candidates. Whole-batch
  validation precedes unload; config/provider retention, independent siblings,
  latest intent, nested hosts and recovery use accepted loader lifecycle workers.
- The offline [reload example](../../../../crates/re-cordis-hmr/examples/reload.rs)
  refreshes config 1→2, replaces compiled-v1 with compiled-v2 on the same EntryId,
  refreshes config 3, reports RestartRequired, joins shutdown and restores Registry.
- Independent review exercises missing/wrong/inactive admission, async refused
  self/ancestor joins, FileWatcher Drop, abandoned ancestor waiters and overflow
  recovery. Repeated 100-cycle resource/capture tests and bounded real polling
  supplement deterministic gates.

The H1 filesystem oracle was corrected to await an actual Removed event together
with missing bytes; an older queued callback alone could observe deletion.
Ten repeats per toolchain passed, followed by normal suites. The only loader
change after its accepted H2 source is one rustfmt newline; token sequence is
unchanged. No broader prerequisite review was reopened.

## Executed shared matrix

The parent ran the saved [matrix runner](hmr-resume/run_matrix.py.txt) at
`target/hmr-20261104/run_matrix.py` after independently resolving all five tools
with `rustup which --toolchain stable|1.85.0`: Cargo, rustc, rustdoc, cargo-clippy
and clippy-driver. Matching RUSTUP_TOOLCHAIN, RUSTC, RUSTDOC and leading toolchain
PATH were set; no Homebrew Rust supplied evidence. Stable rustc/Cargo are 1.97.1;
MSRV rustc/Cargo are 1.85.0. Full commands, exits and log paths are recorded in
`target/hmr-20261104/matrix.json`; tool paths/versions are in `tools.json`.

**34/34 stages passed**: 19 stable stages and 15 MSRV stages. Formatting and
core parity are stable-only as specified by the shared plan; other listed
checks run on both toolchains.

| Executed check | stable 1.97.1 | 1.85.0 |
|---|---:|---:|
| Workspace all-feature/all-target check | Pass | Pass |
| Workspace all-feature tests and Rustdoc tests | 486 passed | 486 passed |
| Strict workspace all-feature/all-target Clippy | Pass | Pass |
| Warnings-denied workspace no-deps Rustdoc | Pass | Pass |
| HMR all-feature subset of workspace | 55 Rust + 1 Rustdoc | 55 Rust + 1 Rustdoc |
| HMR no-default-feature separate stage | 44 Rust + 1 Rustdoc | 44 Rust + 1 Rustdoc |
| Isolated M0 tests/Rustdoc | 21 passed | 21 passed |
| Strict M0 all-target Clippy | Pass | Pass |
| Six runnable offline examples | 6 passed | 6 passed |
| Paired documentation checker | 31 page pairs, 19 HMR exports | Same |
| Executable documentation and external quick start | 13 snippets + app passed | Same |
| Loader paired traces | 2 scenarios / 9 observations | Same |
| Core paired traces | 6 scenarios / 73 observations | Stable-only stage |
| Workspace/M0 formatting | Pass | Same source |
| Documentation checker regressions | 7 passed | Toolchain-independent |

The six examples are lifecycle, timer_lifecycle, console, boot_reconfigure,
config_file and reload. Workspace totals include HMR/loader/Include tests;
subset counts and repeated independent checks must not be added as new unique
tests. Source dispositions account for all 49 index.spec.ts and four path.spec.ts
declarations; they do not claim 53 equivalent Rust/TypeScript HMR traces.

All **152 executable/fixture inputs** match `final-inputs.json` before and after
the matrix. The pinned TypeScript checkout remains clean at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`; temporary parity fixtures were removed.
`final-summary.json` records counts and the no-drift result. Build output is in
`target/hmr-acceptance-{stable,1.85.0}`; long logs remain under target, not Git.

Reproduction uses [shared acceptance commands](../acceptance.md#toolchain-matrix)
and the saved runner with freshly resolved `tools.json` paths. Include
`--all-features`, the no-default HMR stage, all six examples and both loader
trace runs as recorded in matrix.json. Final status/link edits are followed by
both documentation checker runs, the seven regression tests, plan/evidence link
checks and a final source fingerprint check before the closing commit.

## Final closure verification

After the status edits, both documentation checker runs passed again: 31 page
pairs, 19 HMR exports, 13 executable snippets and the external quick start.
All seven checker regressions and 21 HMR plan/evidence link/fence checks pass.
All 152 executable fingerprints still match, and the TypeScript checkout is
clean. Logs: `docs-final-{stable,1.85.0}.log` and `docs-final-regression.log`
under `target/hmr-20261104/`. The closing diff contains documentation only.

## Boundaries and remaining work

Runtime watcher evidence is aarch64 macOS only; Windows/Linux runtime acceptance
is still missing. Polling may miss between-sample changes or digest collisions;
symlink checks are ordinary filesystem behavior, not hostile-filesystem
confinement. A blocked OS read can delay joined shutdown. Bounded event history
can evict errors; counters expose eviction.

No Node import/cache/JS execution, inferred Rust module graph, native binary
swapping, process supervisor or unchanged-host arbitrary nested-code discovery
is provided. Compiled candidates and explicit dependency graphs are deliberate
Rust adaptations. Cleanup/external effects cannot be rolled back transactionally.

Create S0–S2 and whole-program R0/R1 were not started. B1 remains planned; accepted
prerequisite evidence and its tracked gaps are unchanged. All changes are local
commits on the requested integration branch; nothing was pushed or published.
