# R0 application shutdown correction

Date: 2026-11-04. Branch: `feat/cordis-completion`.
Correction base: `4742efe44a55dd1464eb851f9d9eb11d66fde82b`.
Finding: [r0-review.md](r0-review.md), **R0-R1**.

## Contract and correction

Orderly `Application::shutdown` must stop/join the watcher while its
already-admitted Include refresh can still run. It then disposes/joins Include
and finally disposes/joins the application owner. This uses the accepted
watcher contract; the application example changes no framework/public API.

The previous application requested all three disposals before joining any.
That canceled the loader worker awaited by an admitted valid refresh and
created a retained `WatchFailure -> IncludeError -> "loader worker stopped"`.
The independent review reproduced this on both supported toolchains.

Each shutdown stage now captures its admission/join result without returning
from the outer function. Include and owner teardown still run after an earlier
stage fails. All stage results, Include `final_error` and retained watcher
history are inspected before errors are propagated, with the existing error
priority and successful-shutdown resource assertions preserved.

## Deterministic regression

`application_shutdown_joins_admitted_refresh_before_disposing_include` uses
the real shared application under paused current-thread Tokio. A oneshot gate
holds injecting-consumer cleanup after a watched provider configuration
refresh is admitted. Polling `Application::shutdown` remains pending at that
barrier, and the exact old provider remains callable as `v1:1`.

After release, shutdown must succeed. The test also proves that the admitted
replacement finished and the consumer rebound before teardown; exact retained
binding/call results are preserved. It checks terminal Include snapshots,
empty journal, released provider values, writer/timer/watch captures, restored
Registry/effects and empty Runtime diagnostics. An explicitly retained Include
handle owns the catalog until that final handle is dropped.

The new regression was run against unchanged application source first:
**stable and 1.85.0 both exited 101 at the clean-shutdown assertion with the
reported loader-worker cause**. After the ordering correction, both pass.
The existing forced ancestor-cancellation tests, history/cause assertions,
concrete service returns, joined barriers and 100 complete cycles are unchanged.

## Commands, tools and results

All Cargo executions were serialized under the controller's exclusive lease.
`target/r0-correction/run.py` resolves cargo, rustc, rustdoc, cargo-clippy and
clippy-driver using `rustup which --toolchain`, sets matching
`RUSTUP_TOOLCHAIN`, explicit `RUSTC`/`RUSTDOC`, selected toolchain bin first
in PATH and separate `target/r0-correction/build-{toolchain}` targets.
Stable cargo/rustc/rustdoc are **1.97.1**, Clippy **0.1.97**; MSRV tools are
**1.85.0**, Clippy **0.1.85**. No component was installed.

Exact command arrays, paths, versions, exits, timings and logs are recorded in
`target/r0-correction/results.jsonl` and each log's first JSON line.
The command bodies below are invoked with the resolved Cargo:

| Command | stable | 1.85.0 |
|---|---|---|
| `test -p re-cordis-hmr --features include --test system_lifecycle --locked --offline application_shutdown_joins_admitted_refresh_before_disposing_include -- --exact --nocapture` before correction | Expected RED, exit 101 | Expected RED, exit 101 |
| Same regression after correction | 1 passed | 1 passed |
| `test -p re-cordis-hmr --features include --test system_lifecycle --locked --offline` | 8 passed, including 100 cycles | 8 passed, including 100 cycles |
| `run -p re-cordis-hmr --features include --example system_lifecycle --locked --offline` | Passed | Passed |
| Same example with `-- --watch` | Passed | Passed |
| `clippy -p re-cordis-hmr --features include --test system_lifecycle --example system_lifecycle --locked --offline -- -D warnings` | Passed | Passed |
| `run --manifest-path target/r0-review/shutdown-probe/Cargo.toml --offline` | Passed, `Application shutdown result: Ok(())` | Passed, `Application shutdown result: Ok(())` |
| `fmt -p re-cordis-hmr -- --check` | Passed | Stable-only formatter policy |

Formatting used only rustup-resolved stable cargo-fmt/rustfmt. Direct rustfmt
first formatted the two owned Rust files with `--edition 2024 --config
skip_children=true`; the final package formatting check passed.
`git diff --check` passed. Verification source hashes are recorded in
`target/r0-correction/verified-source-hashes.json`.

Logs are named `regression-{red,green}-{toolchain}.log`,
`integration-{toolchain}.log`, `example-{default,watch}-{toolchain}.log`,
`clippy-{toolchain}.log` and
`deterministic-shutdown-green-{toolchain}.log`. A separate
`regression-before-edit-stable.log` preserves an additional unchanged-source
RED repetition following a rejected edit anchor; it is not a corrected-source
verification result.

## Independent artifact preservation and boundary

The reviewer's probe source/manifest and
`target/r0-review/deterministic-shutdown-{stable,1.85.0}.log` were not edited.
Their before/after SHA-256 values are identical in
`target/r0-correction/independent-artifact-hashes-{before,after}.json`.
The original probe ran unchanged against corrected application source; its
GREEN logs are separate from the preserved independent RED evidence.

This correction contains only the shared example application, its new
regression and this report. Shared docs, generated external fixture, paired
traces, final matrix, independent re-review and R0 acceptance remain
controller-owned. This report does not mark R0 accepted. Runtime filesystem and
real-watcher observations remain bounded macOS evidence.

**Cargo lease released after all focused correction commands completed.**
