# R0 independent specification and code review

Date: 2026-11-04. Branch: `feat/cordis-completion`.
Reviewed range: `e8b5eb52b4c810591b24a7a51061da053e84b473..4742efe44a55dd1464eb851f9d9eb11d66fde82b`.

**Verdict: request changes. R0-R1 below is a reproduced application shutdown defect.
R0 is not ready for acceptance at this revision.** The remaining selected
composition and generated-fixture implementation satisfy the reviewed design;
no additional actionable production/API defect was found. The final shared
matrix is intentionally pending correction and scoped re-review.

## Scope and method

Read AGENTS.md and [r0-plan.md](r0-plan.md) first, then
[shared R0 acceptance](../acceptance.md#r0--whole-program-integration-acceptance),
the implementation/correction/source-map reports, current paired guides and
decision/limitation changes. Reviewed all executable changes in the four
commits `f7a8e58`, `2023e56`, `b35a979`, and `4742efe`.
CodeGraph supplied current source and the ownership/call paths through loader,
Include and HMR; Git supplied the exact range/diff.

Carried accepted B0/core, timer, console, loader/groups, Include, HMR and Create
evidence, including the accepted final review/acceptance checkpoints cited by
[r0-source-map.md](r0-source-map.md). This review does not re-audit B0 or invent
new TypeScript comparisons. Initial controller/worker logs were inspected as
reported evidence, separately from the independent executions below.

The independent reviewer changed no production, application, test, checker or
template source. Only this report and ignored probes/logs under
`target/r0-review/` were written. No subagent was spawned and no commit or push
was made.

## Actionable finding

### R0-R1 — P2: Join retained watcher work before canceling Include ownership

**Location:** `crates/re-cordis-hmr/examples/system_lifecycle/app.rs:332–337`.
The exposed caller is
`crates/re-cordis-hmr/tests/system_lifecycle.rs:435–447`.

`Application::shutdown` requests watcher disposal, Include disposal and
ancestor disposal before awaiting any of them. Watcher disposal retains and
joins the already-admitted Include refresh; it does not dispose Include.
Immediately disposing Include/its ancestor cancels the loader workers that
refresh is still awaiting. The retained watcher outcome then becomes
`WatchFailure -> IncludeError -> LoaderError::Invalid("loader worker stopped")`,
and an otherwise clean shutdown returns an error.

This is observable in the delivered suite. The independent 1.85.0 run failed
`bounded_real_watcher_updates_the_shared_application` with exactly that cause
after the test observed sibling configuration 12. Seeing one sibling's updated
value does not mean every entry in the admitted refresh has settled.

A deterministic independent probe uses the exact shared application module:
boot under paused current-thread Tokio, gate the injecting consumer cleanup,
admit a watched provider-config refresh, wait for cleanup entry, poll
`Application::shutdown`, release cleanup, and assert clean shutdown succeeds.
It fails on **both stable and 1.85.0**, with the same retained loader-worker
error. The probe contains no application replacement or framework patch.

Artifacts:

- `target/r0-review/integration-1.85.0.log`: delivered suite, six passed / one failed.
- `target/r0-review/shutdown-probe/src/main.rs`: deterministic public application probe.
- `target/r0-review/shutdown-probe/Cargo.toml`: isolated probe manifest using current local crates.
- `target/r0-review/deterministic-shutdown-stable.log` and
  `deterministic-shutdown-1.85.0.log`: both exit 101 at the intended clean-shutdown assertion.

**Required correction:** orderly application shutdown must stop/join the
watcher while its admitted Include work can still run, then dispose/join
Include and the application owner. Preserve cleanup/error observation when any
stage fails; do not mask the failure, add a delay, or weaken history assertions.
Add a paused-time application-level regression that holds an admitted refresh
and exercises `Application::shutdown` itself. Preserve the existing forced
ancestor-cancellation tests and their retained cancellation/error expectations:
they exercise a different supported operation. No production API change is
needed to use the accepted watcher/Include contract correctly.

Ten isolated real-watcher repetitions per toolchain subsequently passed. Those
passes characterize a scheduling race; they do not erase the original failure
or the deterministic reproduction.

## Specification and quality assessment

| Requirement | Inspected evidence and assessment |
|---|---|
| One usable implementation | Example entry and integration tests import the same `examples/system_lifecycle/app.rs`; generated fixture copies these exact files. No second application implementation or facade is introduced. |
| Complete local composition | A current-thread Tokio entry drives local Runtime. Application owns console writer, typed provider/injecting consumer, interval/timeout/stream, structural isolated loader groups, Include and synthetic/real file watcher. Added logger dependency is dev-only; Cargo.lock adds no new package. |
| Actual provider move and retention | Test moves provider left to right via Include, keeps logical EntryId and changes FiberId. Cleanup reads the old provider through its Context before/after a channel gate and verifies exact pointer identity and callable resource state. Sibling config and owner timer progress before release. The old consumer becomes Pending; moving it binds the new provider. Old service weak pointer expires after caller release. File content confirms persisted movement. |
| Linked behavior and recovery | Tests assert concrete provider responses `v2:1`, recovered `v3:1`, updated `v3:5`, and consumer binding after settlement. Apply failure 201 and update failure 202 are downcast to original causes and pointer-compared across Report/operation and Runtime. Parser history is distinguished from Runtime lifecycle diagnostics and explicitly recovered. |
| Pending forced shutdown | The ancestor-disposal tests admit a watched refresh, unread stream/timers, follow-up event and pending Include journal. A gate proves disposal request is pending. Persistence succeeds before dependent cleanup, or invalid bytes/final_error/pending journal remain visible. Full watcher counters, core allocations, provider/consumer counts, Registry/effects, writer/callback/timer weak captures, terminal stream and Runtime causes are checked. These tests pass independently, but do not cover orderly `Application::shutdown` with admitted work; R0-R1 does. |
| 100 complete cycles | Each cycle boots the composition, edits provider config, replaces linked behavior, flushes, dispatches a listener and joins shutdown. Later dispatch is inert; provider weak values, writer/catalog/callback/timer captures, Registry and effects return to baseline. These are 100 application cycles, not a sum of prerequisite suites. |
| Loader correction | The only production line clears `entry.running` when loader owner cleanup closes entries. The canceled task cannot reach its ordinary flag-reset tail; stale `running` otherwise wins over non-live state in snapshots. The native regression separately proves held cleanup, joined capture release, empty Registry/effects/diagnostics and terminal snapshot. Independent GREEN passes on both toolchains; inspected worker RED logs contain the expected `Unloading != Disposed` assertion on both. |
| Error handling | Reviewed operation errors, per-entry Reports, retained watch history, Include final_error, root CleanupReport and Runtime channels. Runnable demonstration drains root on work/setup failure and reads diagnostics. Controlled watch observations are allowed only for pointer-identical original load causes. R0-R1 retains rather than hides an error but creates it through incorrect shutdown ordering. |
| Bounded evidence and scope | Fixed instrumentation vectors belong to bounded application scenarios. Real polling remains supplemental/macOS-only. Docs retain synchronous I/O, crash-durability, hostile-filesystem and platform limits, and separate linked Rust candidates from source compilation/Node behavior. B0/loader gaps and existing paired denominators are carried, not reclassified as new equivalence. |
| Scope restraint | Core, public APIs and basic Create production template are unchanged; no additional dependency package, live service, remote transport, supervisor, platform/CI expansion, R1 or standalone B1 work is included. |

## Actual Create-generated external verification

Inspected `tests/docs/check.py:237–340` and the new
`run_generated_r0` path at lines 185–234.

The checker actually executes the delivered Create CLI using an argument array
with `--cordis-path ROOT` and a destination containing spaces/Vietnamese
characters. It verifies the basic generated file set, independent workspace,
package policy and local dependency paths, then builds/tests/runs the basic
application and strict Clippy before extending it.

Only that temporary output gains timer/Include/HMR plus existing utility
dependencies. The exact example entry, shared app and integration tests are
copied with `read_bytes()/write_bytes()`; the fixture's enabled `include`
feature prevents the copied test gate from silently skipping the suite.
Cargo metadata checks all six local framework dependency paths. Build, locked
test/run, strict all-target Clippy, stable-only formatting and real-watch run
use the selected explicit toolchain environment.

Inspected controller `docs-initial-{stable,1.85.0}.log` under
`target/r0-20261104/`: both record 34 paired pages, 13 snippets, basic external
five-test pass, R0 external seven-test pass and both application run modes.
These are actual earlier executions; R0-R1 means they do not establish a
reliable final result for the shared application. The final corrected checker
execution remains part of the controller's shared acceptance matrix.

## Independent commands and results

All five tools were resolved by `rustup which --toolchain`, with matching
`RUSTUP_TOOLCHAIN`, explicit `RUSTC`/`RUSTDOC`, the selected bin directory
first in PATH, and isolated `target/r0-review/build-{toolchain}` targets.
Exact paths, versions, command arrays, exit statuses, timings and log paths are
in `target/r0-review/probe-tools.json` and `probe-results.json`.

Stable: cargo/rustc/rustdoc 1.97.1, Clippy 0.1.97.
MSRV: cargo/rustc/rustdoc 1.85.0, Clippy 0.1.85.
Runtime platform: aarch64 macOS. No components were installed.

| Independent execution | stable | 1.85.0 |
|---|---|---|
| `cargo test -p re-cordis-hmr --features include --test system_lifecycle --locked --offline` | 7 passed | 6 passed, 1 failed (R0-R1) |
| `cargo test -p re-cordis-loader --test shutdown_snapshot --locked --offline` | 1 passed | 1 passed |
| Real watcher test alone, exact filter, ten repetitions | 10 passed | 10 passed |
| `cargo run --manifest-path target/r0-review/shutdown-probe/Cargo.toml --offline` | Expected RED, exit 101 | Expected RED, exit 101 |
| `git diff --check e8b5eb52b4c810591b24a7a51061da053e84b473..4742efe44a55dd1464eb851f9d9eb11d66fde82b` | Passed once, shared diff | Same source |

The isolated repetition command appends
`bounded_real_watcher_updates_the_shared_application -- --exact --test-threads=1`
to the focused integration command. Repetitions are not unique test counts.
The probe's first offline build resolves its own local lockfile; subsequent
MSRV execution uses that fixture. No shared docs/parity/full-matrix run was
started by this reviewer.

## Closure gate

Correct R0-R1, run its deterministic regression on both toolchains, verify the
full focused application suite/both example modes, and request scoped re-review.
Then execute the unchanged required shared stable/1.85.0 matrix and actual
generated external applications. Do not mark R0 accepted from this report or
the earlier passing runs.

**Cargo lease released after the independent probes completed. No reviewer
Cargo process remains running.**
