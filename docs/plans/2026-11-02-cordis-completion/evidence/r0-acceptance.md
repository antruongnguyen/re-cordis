# R0 whole-program integration acceptance

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
**Verdict: delivered and independently accepted within the offline R0 scope.**
Final executable revision: `35556d5108eaf13104a5fb287c83b28ffb5d1576`.
Generated-project checker revision: `4742efe44a55dd1464eb851f9d9eb11d66fde82b`.
The closing commit containing this report records final documentation/acceptance
on `feat/cordis-completion`; it adds no executable change.

## Delivered application and reproduction

```sh
cargo run -p re-cordis-hmr --example system_lifecycle --features include --locked --offline
cargo run -p re-cordis-hmr --example system_lifecycle --features include --locked --offline -- --watch
cargo test -p re-cordis-hmr --test system_lifecycle --features include --locked --offline
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
```

Run from the checkout. The [entry point](../../../../crates/re-cordis-hmr/examples/system_lifecycle.rs)
and [integration tests](../../../../crates/re-cordis-hmr/tests/system_lifecycle.rs)
import one [application module](../../../../crates/re-cordis-hmr/examples/system_lifecycle/app.rs).
Current-thread Tokio drives local Runtime; the application composes a scoped
console writer, typed provider/injecting consumer, timeout/interval/stream,
isolated structural groups, Include configuration/persistence and HMR.
Default execution uses deterministic synthetic ingress; --watch selects a
bounded one-shot real polling producer. Both modes perform actual updates,
linked replacement, controlled failure/recovery and joined shutdown.
See the [English](../../../en/guides/system-lifecycle.md) and
[Vietnamese](../../../vi/guides/system-lifecycle.md) guides.

The provider moves through Include from the left isolation group to the right.
A channel gate holds its old injecting consumer cleanup; before and after the
gate, the consumer calls the pointer-identical old provider while an unrelated
sibling and owner timer progress. The provider rebuilds with the same EntryId
and a different Fiber. The old consumer is Pending until moved, then binds the
new provider. Persisted bytes confirm the moves. Linked replacement and updates
assert concrete responses `v2:1`, `v3:1`, then `v3:5`.

Orderly application shutdown joins admitted watcher refresh while Include
remains live, then joins Include and the application owner. Separate forced
ancestor-cancellation tests inspect the retained canceled-refresh cause, final
flush and original cleanup errors. Successful waiter completion alone is not
used as a release/error oracle.

## Checkpoints and independent review

Clean start: Create acceptance `e8b5eb5`; no newer or uncommitted work was
discarded. Carry Create executable/checker `8373fa3`, HMR acceptance
`4c8f423` and executable `55a55d1`, Include `0a2df1e`, loader `101f37b`,
accepted timer/logger/core and B0. Production APIs and the Create basic template
remain unchanged.

| Milestone | Revision / evidence |
|---|---|
| Narrow loader correction | `f7a8e58`; [RED/GREEN and native regression](r0-loader-correction.md) |
| Shared application, seven initial integration tests and both example modes | `2023e56`; [implementation](r0-implementation.md) |
| Concrete replacement/config return assertions | `b35a979` |
| Actual generated R0 fixture/checker, paired docs and source reconciliation | `4742efe` |
| Fresh independent review | [R0 review](r0-review.md): one P2 R0-R1 |
| Verified orderly shutdown correction and eighth regression | `35556d5`; [correction](r0-corrections.md) |
| Fresh independent precise-delta re-review | [R0-R1 ADDRESSED, specification and quality approved](r0-rereview.md) |
| Full shared matrix | 40/40 stages at `35556d5`, recorded below |

The only production change is one loader owner-cleanup line clearing the
canceled convergence worker's running flag. The original integrated and reduced
native regressions showed Unloading after joined release with absent core
allocations on both toolchains. The fix preserves resource/error/hosting
contracts; a separate regression asserts actual capture/Registry/effect release
and final Disposed state. It does not redesign core or reopen B0.

Independent R0-R1 review caught a real MSRV smoke race missed by earlier passing
runs, then reproduced it deterministically on both toolchains using the exact
application. The application had canceled Include before watcher join, producing
an avoidable original `loader worker stopped` error. The correction keeps
Include live through watcher join and still attempts/joins every cleanup stage
before propagating errors. The unchanged independent probe and all eight
application tests pass on both toolchains in the fresh re-review.
Original RED logs/probe inputs remain intact; later passing repeats do not erase
the failure. No actionable finding remains in the reviewed correction.

## Executed final shared matrix

The tracked [runner](r0-run-matrix.py.txt) was copied without alteration to
`target/r0-20261104/run_matrix.py` and executed as:

```sh
python3 -B target/r0-20261104/run_matrix.py
```

For a fresh reproduction, run the tracked file directly from the repository
root with Python, or copy it to the shown target path first. It extends the
accepted 34-stage Create runner with the R0 default/watch runs and offline
GitHub digest, retaining all existing checks. Commands are process argument
arrays. `matrix.json` records each command, exit, duration and log;
`tools.json` records paths/versions; `final-inputs.json` and
`final-summary.json` record the input freeze and results in the same target folder.

**40/40 stages passed: 22 stable and 18 MSRV. All 166 executable/fixture
fingerprints matched before and after the run.**
The pinned TypeScript checkout remained tracked-clean at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`.

| Executed check | stable 1.97.1 | Rust 1.85.0 |
|---|---:|---:|
| Workspace all-feature/all-target check | Pass | Pass |
| Workspace all-feature tests and Rustdoc checks | 517 passed | 517 passed |
| R0 suite included above | 8 passed, including 100 complete cycles | Same |
| New loader snapshot regression included above | 1 passed | Same |
| Preserved Create subset | 22 passed | Same |
| Preserved GitHub consumers | 3 collector + 3 digest + 6 webhook passed | Same |
| Strict workspace all-feature/all-target Clippy | Pass | Pass |
| Warnings-denied workspace no-deps Rustdoc | Pass | Pass |
| HMR no-default-feature separate stage | 45 checks | 45 checks |
| Isolated M0 tests/Rustdoc and strict Clippy | 21 checks; pass | Same |
| Runnable offline applications | Eight programs; nine invocations | Same |
| Paired documentation/export/link checks | 34 EN/VI page pairs | Same |
| Existing executable docs and external quick start | 13 snippets + app | Same |
| Actual Create CLI/file set/metadata/basic external app | 5 tests, build/run/Clippy pass | Same |
| Extended actual generated R0 app | 8 tests, build/run/both modes/Clippy pass | Same |
| Core paired traces | 6 scenarios / 73 observations | Stable-only stage |
| Loader paired traces | 2 scenarios / 9 observations | Same |
| Documentation checker regressions | 8 passed | Toolchain-independent |
| Workspace/M0/generated-app formatting | Pass, explicit stable tools | Same source; no separate MSRV formatter gate |

The programs are lifecycle, timer_lifecycle, console, boot_reconfigure,
config_file, reload, system_lifecycle (default and --watch), and github_digest.
No authenticated collector or live webhook request was made. Existing bounded
consumer tests remain in the workspace matrix. The 517 total is the accepted
508 entry baseline plus eight R0 tests and one loader regression. Subsets,
external-project copies and repeated executions are not additional unique tests.

Every compilation/test/doc/lint stage resolves Cargo, rustc, rustdoc,
cargo-clippy and clippy-driver through rustup, verifies their common toolchain
bin and sets matching RUSTUP_TOOLCHAIN, RUSTC, RUSTDOC and leading PATH.
Stable Cargo is 1.97.1 (`c980f4866`), rustc/rustdoc 1.97.1 (`8bab26f4f`),
Clippy 0.1.97; MSRV Cargo is 1.85.0 (`d73d2caf9`), rustc/rustdoc 1.85.0
(`4d91de4e4`), Clippy 0.1.85. Stable cargo-fmt/rustfmt resolve explicitly to
rustfmt 1.9.0-stable. No toolchain component was installed and Homebrew Rust
was not used. Host: arm64 macOS 26.7.1, build 25G241; Python 3.15.2, Git 2.56.0.

## Actual generated external application

The checker invokes the delivered Create CLI with an explicit checkout path
and a destination containing spaces/Vietnamese characters. It first validates
the exact five-file basic output, independent resolver-3 workspace, version
0.0.0/publish=false/Rust 2024/MSRV 1.85 and canonical core/logger/loader paths;
it builds/tests/runs that unchanged basic application.

Only its temporary output then gains explicit local timer, Include and HMR
dependencies plus existing utility/Tokio features. Byte-for-byte copies of the
reference entry, shared application and integration suite run in that external
Cargo project. Its enabled include feature ensures all eight tests execute.
Cargo metadata verifies all six local Cordis dependencies; build resolves an
offline lockfile and test/run/Clippy use it. Both normal and real-watch binaries
execute. The production basic template is unchanged; no template variant,
new package, downloader or stubbed generator was introduced.

## Source dispositions and limits

[The composition source map](r0-source-map.md) carries accepted ledgers and
separates current native witnesses, Rust representations, exclusions and gaps.
R0 adds **eight native application oracles and one native loader regression**,
with **zero new whole-program Rust/TypeScript paired scenarios**. The six core
and two loader comparisons above were freshly rerun; timer's four/16 comparison
remains explicitly carried prerequisite evidence, not a new R0 execution.
B0's 147 evidence gaps and the loader ledger's 65 primary G marks are not silently
reclassified. Native provider movement adds relevant evidence without claiming
every upstream isolation variant. These units are not combined into a completion
percentage; excluded/unexecuted cases are not counted as passing.

- All crates remain unpublished 0.0.0, publish=false, Rust 2024/resolver 3/MSRV 1.85.
- Filesystem and watcher execution covers **macOS only**; Windows/Linux runtime
  evidence is missing. Synthetic/paused-time gates supply deterministic causal
  evidence; real polling is bounded supplemental evidence.
- Owned tasks, unread stream, writer, watch producer/callbacks, children,
  services, listeners and weak captures are checked at documented join/drop
  barriers. Include's catalog remains owned by retained Include handles; a
  failed journal remains observable until final handle drop.
- Synchronous console/file I/O may block. Per-file replacement/flush is not
  fsync, crash durability, a multi-file transaction, arbitrary concurrent-writer
  coordination or hostile-filesystem confinement. Blocked OS reads can delay join.
- Polling can miss changes between samples; bounded history can evict errors.
  Explicitly registered paths and already-linked candidates remain required.
- No native Rust binary hot swap, inferred module graph, JS/Node execution,
  process supervisor, remote Create, additional template, live integration,
  CI/platform expansion, standalone B1 or R1 release work was added.

Recurring entry/correctness/toolchain/documentation gates remain in
[shared acceptance](../acceptance.md); their generic unchecked items are future
requirements, separate from the completed executions recorded here.

## Final documentation and evidence closure

After the final guide/evidence links, both documentation toolchain runs passed
again: 34 page pairs, 13 snippets, external quick start, five unchanged basic
Create tests, eight generated R0 tests and both run modes. All eight checker
regressions and link/fence checks over 23 plan/R0/instruction files pass.
All 166 executable/fixture hashes still match the matrix; the runner copy is
byte-identical and the TypeScript checkout remains tracked-clean.

Logs are `docs-final-{stable,1.85.0}.log`, `docs-final-regression.log`,
`closure.json` and `closure-verification.json` under `target/r0-20261104/`.
The independent reviewer also checked the final matrix, counts, source hashes,
tool/host versions, generated results and source/platform boundaries; the
[final evidence-closure assessment](r0-rereview.md) approves acceptance with no
factual mismatch. Only final status/checklist/report prose follows that check.
The closing local commit records R0 as done. Nothing is pushed or published,
and no R1/standalone B1 work or background continuation is started.
