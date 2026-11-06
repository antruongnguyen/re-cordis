# R0 whole-program integration implementation plan

**Goal:** Accept one useful offline application composed from the already accepted core and satellites.
**Architecture:** Place the runnable entry at crates/re-cordis-hmr/examples/system_lifecycle.rs,
shared application implementation under examples/system_lifecycle/app.rs, and integration oracles
at tests/system_lifecycle.rs. Both reference and generated external application use that same
implementation. Keep the Create production basic template unchanged.
**Spec:** acceptance.md#r0--whole-program-integration-acceptance and the user's 2026-11-04 R0 request.
**Tech stack:** Existing Rust 2024 workspace, current-thread Tokio, local Runtime; no new dependencies.

## Global constraints

- Integration branch feat/cordis-completion; clean start e8b5eb52b4c810591b24a7a51061da053e84b473.
- Carry Create executable/checker 8373fa3, HMR executable 55a55d1 and acceptance 4c8f423;
  loader 101f37b, Include 0a2df1e, timer/logger and B0 accepted evidence.
- Rust 2024, resolver 3, MSRV 1.85; version 0.0.0, publish=false.
- No production change without a concrete failing integration reproducer. No B0 reopening/core redesign.
- No R1, standalone B1, CI/platform expansion, remote transport/template variants, live service or supervisor.
- Runtime filesystem/watcher evidence is macOS only. Ordinary synchronous I/O is not crash durability or hostile-filesystem confinement.
- Use rustup-resolved cargo/rustc/rustdoc/cargo-clippy/clippy-driver and matching RUSTUP_TOOLCHAIN/RUSTC/RUSTDOC/leading PATH.
  Formatting uses explicitly resolved stable cargo-fmt/rustfmt only; install no components.
- Coordinate all Cargo/parity runs through the parent. No pushes/publication/Co-authored-by.
- Fresh implementation agent gpt-6.1-sol/high and review agent gpt-6-astra/high, fork_turns=none.
- The controller owns shared status docs, checker/external fixture and final matrix; the implementation worker
  owns only the HMR application/example/tests and necessary dev-dependency manifests/lockfile.
- No worker spawns another agent. Commit only owned verified changes.

## Task 1: Reference application and deterministic integration oracles

Read accepted contracts through decision.md, limitation.md and relevant EN API guides and acceptance reports.
Use CodeGraph before code exploration. Reuse existing Include/HMR integration and generated basic app patterns.

- [x] Compose local Runtime/current-thread Tokio, scoped console writer, typed provider and injecting consumer,
  owner timer, structural loader groups, Include files/persistence and optional synthetic/real watcher.
- [x] Expose a small application module usable by the example, tests, and copied external fixture.
  Define its entry contract in r0-implementation.md for the checker author.
- [x] Test supported Include edits/moves across isolation scopes. Gate dependent cleanup with a oneshot;
  prove exact old provider identity remains callable until release, replacement readiness afterward,
  and unrelated sibling progress before gate release.
- [x] Replace a linked factory through ReloadPlan; exercise controlled load/reconfiguration failure and
  explicit recovery. Check original causes in the correct operation/report/history/Runtime channels,
  distinguishing repeated observations of the same failure from distinct failures.
- [x] Shut down with admitted timer/watch/Include work and pending journal. Prove request differs from join.
  Inspect Include final_error, cleanup reports and Runtime diagnostics. Assert persistence outcome,
  no live tasks/watches/writers/children/services and weak/capture release at documented barriers.
- [x] Repeat at least 100 deterministic lifecycles with effect/listener/Registry/publication baselines
  and weak release. Keep bounded real filesystem watcher smoke supplemental.
- [x] Run focused tests/example, strict targeted Clippy and stable formatting; record command/log results.
- [x] Commit application/tests and an implementation report. Production APIs/template remain unchanged
  unless a concrete failure is escalated and justified.

## Task 2: Real generated external application and user documentation

- [x] Invoke delivered Create CLI with an explicit checkout path using process argument arrays.
  Extend only test-owned external fixture with local timer/Include/HMR dependencies, copy the shared
  app and its tests and reuse the entry point; build/test/run on stable and 1.85.0 offline.
- [x] Extend existing documentation checker narrowly; preserve basic-template five tests and other checks.
- [x] Add paired EN/VI integration guide, connect relevant API indexes/guide navigation and llms.txt.
  Update README, CHANGELOG, AGENTS, todo, decision, limitation, docs README and execution progress.
- [x] Record composition-relevant source dispositions by carrying existing accepted ledgers; preserve
  six core paired scenarios/73 observations and accepted loader paired traces. No invented equivalence.

## Task 3: Independent review, correction and acceptance

- [x] Provide exact implementation diff, report, commands and evidence to a fresh independent reviewer.
- [x] Correct actionable findings with focused regressions, commit verified deltas, and obtain scoped re-review.
- [x] Run complete current shared stable/1.85.0 matrix, all relevant runnable examples, generated apps,
  documentation regressions and paired traces serially. Save long logs and tool/input hashes under target/.
- [x] Save durable r0-acceptance.md with revisions, tool versions, results, review closure, source
  dispositions and remaining limits; distinguish recurring gates from completed R0 evidence.
- [x] Commit final acceptance and verify no task-owned changes remain. Do not start another package.

## Reproduced integration correction

Pending-work shutdown tests proved Registry/effects, services, writer, timer and
watch captures released, but a retained structural group snapshot still reported
Unloading with no Fiber handles. Cancellation skipped the loader worker's normal
running=false tail. The implementation worker also owns the single owner-cleanup
running-flag correction in crates/re-cordis-loader/src/lib.rs, a focused public
regression in tests/shutdown_snapshot.rs in that crate, and r0-loader-correction.md.
Require RED/GREEN on stable and 1.85.0, all loader tests, a separate verified
correction commit, and independent review. This does not reopen core/B0 or change
hosting semantics; the R0 snapshot assertion remains in place.
