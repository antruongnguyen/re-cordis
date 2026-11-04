# R0 reference application implementation

Date: 2026-11-04. Branch: `feat/cordis-completion`.
Baseline: `e8b5eb52b4c810591b24a7a51061da053e84b473`; newer controller work is
preserved. This report covers Task 1 of [r0-plan.md](r0-plan.md), not R0 final
acceptance. Independent review, generated external application, documentation,
paired-source reconciliation and the full shared matrix are controller-owned.

## Entry and copy contract

The runnable entry is
`crates/re-cordis-hmr/examples/system_lifecycle.rs`, using
`#[path = "system_lifecycle/app.rs"] mod app;`.
Its integration suite is `crates/re-cordis-hmr/tests/system_lifecycle.rs`, using
`#[path = "../examples/system_lifecycle/app.rs"] mod app;`.
The controller copies these exact files into the generated external project:
there is one application implementation and no rewritten fixture application.

The shared module exposes:

- `Application::boot(root: &Context, directory: &Path, writer: impl Write +
  'static, real_watch: bool) -> Result<Application, BoxError>`: a dedicated
  application owner, typed provider/injecting consumer, console writer,
  structural isolated loader groups through Include, timeout/interval/stream,
  and synthetic or real HMR ingress.
- Public read/operation handles for the test oracle; `entry`,
  `hold_consumer`, `emit_config`, `fire` and `replace` support the same
  application behavior exercised by the example.
- `Application::shutdown(self)`: admit subsystem/ancestor cancellation and
  join all results before observing pending journal, final Include error,
  watcher history and resource counters.
- `demonstration(real_watch: bool)`: creates temporary files and a local
  Runtime, performs the one-shot composition walkthrough, and joins root
  cleanup even when setup/work fails. CleanupReport and Runtime diagnostics
  are read before a work error is propagated.

Main builds current-thread Tokio with its drivers enabled. No arguments uses
synthetic ingress; `--watch` enables the same bounded one-shot application
with a real polling producer. Unknown arguments fail with the explicit usage
string. This is not a watcher daemon or supervisor.

Dependencies are existing local core, loader, timer, logger, Include and HMR
(`include` feature), plus existing serde_json, futures-util and Tokio
rt/time/macros/sync/test-util. HMR adds logger only as a dev dependency and
declares the example's required `include` feature. Cargo.lock adds that
existing local dependency; no new package is introduced. The external fixture
defines/enables its local `include` feature so the copied test gate runs.

## Executed oracles

| Test | What it establishes |
|---|---|
| `provider_move_keeps_exact_old_binding_and_sibling_progress` | Actually moves the provider from left to right isolation via Include, preserving logical EntryId and rebuilding Fiber; holds old injecting consumer cleanup; proves pointer-exact old provider remains callable/alive until release; unrelated sibling config and owned timer progress before release; left consumer becomes Pending, then a supported consumer move obtains the new provider; source move persists. |
| `linked_replacement_and_distinct_failure_channels_recover` | Linked ReloadPlan candidates preserve EntryId/file bytes while FactoryId changes; apply failure 201 reaches Report and Runtime with the same original error pointer; update middleware failure 202 reaches its operation and Runtime with the same pointer; distinct failures each appear once; explicit recovery restores Active; malformed bytes reach watcher history with the original serde_json error and recover without a parser Runtime diagnostic. |
| `shutdown_joins_pending_work_and_persists_journal` | With admitted watched refresh, owned timers/unread stream, a held consumer cleanup, a queued follow-up and pending journal, cancellation differs from join; ancestor flush persists the admitted entry before child release; the canceled refresh is retained once with its Include/loader cause and source location. |
| `shutdown_preserves_distinct_io_and_cleanup_failures` | The same pending shutdown preserves malformed source bytes and final_error/pending journal instead of overwriting them; original cleanup error 301 and original parser write error each reach Runtime once; nested root CleanupReport remains empty as accepted core channels specify. |
| `hundred_composed_lifecycles_restore_baselines_and_captures` | 100 deterministic full boots/config edits/linked replacements/flushes/disposals restore root effect/Registry and service baselines; post-shutdown event dispatch invokes no old consumer listener; provider, writer, catalog, watcher callback and timer captures release at their documented barriers. |
| `bounded_real_watcher_updates_the_shared_application` | A five-second-bounded macOS polling supplement uses an atomic rename to update the same application and joins producer shutdown. |
| `runnable_example_uses_the_same_application` | Calls the exact example walkthrough implementation under paused current-thread Tokio. |

Deterministic dispatch uses paused Tokio time and channel gates. The fixed
clock/yield stimulation is not a release oracle: actual joined handles, absent
core allocations, terminal entry snapshots, Registry/effect baselines, inert
post-shutdown listeners/writers, stream terminal delivery and weak capture
release are asserted separately.

The pending-shutdown tests independently check all watcher work counters are
zero, producer stopped, all Include entries Disposed with absent Fiber/outer
IDs, no live providers/consumers and all old service values released. Writer,
timer and watch closure captures release at owner/watcher join. Include's
catalog remains explicitly retained by its public handle after shutdown and
releases after that handle drops. A failed final journal stays observable until
the Include handle is dropped; it is not counted as pending executable work.
The unread stream reports disposal once and then stable termination.

## Corrections and carried contracts

Accepted T5/C3/L5/G1/I4/H3/S2 contracts were read and reused. CodeGraph preceded
production API exploration. Original application compile/failed test logs are
retained as local development evidence, not production failures:
`red.log`, `first.log`, `second.log`, `third.log`, `fourth.log`.
The corrections established that Include disposal waiters must be admitted
while their parent owner is live, and an accepted operational update whose
middleware returns error can retain the new source (99) without rolling it
back. This is distinct from a validation rejection.

The concrete additional production defect was isolated and repaired separately:
[f7a8e58 loader correction](r0-loader-correction.md). A canceled group
convergence worker skipped its running-flag tail, leaving a stale Unloading
snapshot after actual resources joined. The one-line loader owner-cleanup fix
and focused regression preserve every accepted production API, core host,
resource join and provider-retention behavior. No Create production template
changed.

The real watcher may observe the same controlled load failure during explicit
replacement/recovery. The walkthrough joins watcher work and permits only a
pointer-identical original cause from the exact failure Report; any unrelated
watcher failure is returned. This is a repeated observation of one failure,
not an extra distinct failure. Shutdown/error observations are not discarded.

## Focused verification

All **8/8 application stages pass**: seven integration tests, default runnable
walkthrough, real-watcher walkthrough and strict targeted Clippy, each on
stable 1.97.1 and Rust 1.85.0. There are seven unique tests; running them on
two toolchains does not produce fourteen unique tests. The 100-cycle scenario
and real polling smoke are included in those seven.

All tools were resolved through `rustup which --toolchain`; Cargo, rustc,
rustdoc, cargo-clippy and clippy-driver share the selected toolchain bin.
Matching RUSTUP_TOOLCHAIN, RUSTC, RUSTDOC and leading PATH are explicit.
`target/r0-implementation/tools.json` retains exact paths/versions,
`app-stages.json` retains argument arrays/exits/logs. Commands:

```sh
cargo test -p re-cordis-hmr --features include --test system_lifecycle --locked --offline
cargo run -p re-cordis-hmr --features include --example system_lifecycle --locked --offline
cargo run -p re-cordis-hmr --features include --example system_lifecycle --locked --offline -- --watch
cargo clippy -p re-cordis-hmr --features include --test system_lifecycle --example system_lifecycle --locked --offline -- -D warnings
```

Stable-resolved rustfmt checks all three application sources with edition 2024.
The separate loader correction passes 91 Rust tests and two Rustdoc checks and
strict all-target Clippy on both toolchains; see its report. Long logs/builds
are under `target/r0-implementation/`, not committed. Full shared
workspace/M0/docs/parity/external-app acceptance remains the controller's work.

## Boundaries

Runtime filesystem and real watcher evidence is macOS only. Synchronous file
I/O and per-file staging do not imply crash durability, an atomic multi-file
transaction, hostile-filesystem confinement or arbitrary concurrent-writer
safety. Polling can miss between-sample changes. Named isolated groups rebuild
core allocations after joined cleanup; they do not emulate mutable JS
prototype reparenting. The source-case accounting and prior paired traces are
preserved separately by the controller; these tests are Rust integration
oracles, not invented Rust/TypeScript equivalent traces.

The module's observation vectors are bounded by each fixed application/test
scenario; they are instrumentation, not framework work queues. Production
Include/HMR/timer bounds are reused. All changes are local; nothing is pushed
or published. R1 and broader B1/platform/source-equivalence work remain outside
this implementation.
