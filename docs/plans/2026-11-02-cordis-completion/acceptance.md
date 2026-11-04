# Shared acceptance and integration plan

**Spec:** [Master design and global constraints](README.md). Applies to every
implementation work package; R0/R1 are final integration/distribution decisions.

## Per-slice entry gate

- [ ] Confirm selected work package, current revision, understood worktree
  and prerequisite acceptance. Do not discard unrelated changes.
- [ ] Apply the [prior-decision/evidence reuse rule](README.md#prior-decisions-and-evidence-reuse):
  cite relevant decision.md, limitation.md and implementation-plan.md sections,
  carried evidence and the uncovered delta in the handoff. Give a concrete
  reason before reopening a completed decision, review or implementation.
- [ ] Read pinned source and every parameter variant relevant to this slice.
  Check the accepted decision record and exact public signatures.
- [ ] Complete the package's design gate before implementation. Expand only
  this slice into a concrete test-first coding plan; do not guess future APIs
  from a conceptual name in the portfolio plan.
- [ ] Record files, expected failing regressions, dependencies, platform limits
  and measurable resource bounds. Make one independently reviewable slice.

## Correctness requirements

Every owned resource has a construction/admission point, cancellation request,
release point, and observation of completion. Test the distinctions explicitly.

| Concern | Required evidence |
|---|---|
| Admission | Wrong host, missing host and inactive owner do not leak tasks/registrations or invoke refused work |
| Cleanup | Manual and owner disposal are idempotent at the occurrence level; joined barriers include promised resource release |
| Lifecycle | Pending differs from failure/readiness; explicit update recovers Failed; latest intent and disposal win |
| Retention | Exact old providers remain usable through dependent cleanup, including Registry removal and moves |
| Failure | Original causes survive; abandoned waiters do not cancel committed cleanup or hide diagnostics |
| Reentry | No user callback/destructor runs under a framework mutable borrow that its supported operations can reenter |
| Bounded work | Stream queues, file input, journals, watcher queues and retries have explicit tested bounds where applicable |
| Long-lived use | Repeat at least 100 deterministic create/use/dispose cycles and restore effect/listener/Registry/publication baselines; test weak/capture release separately |
| Compatibility | Each included source variant has an oracle; reviewed deviations are never counted as equivalent traces |
| Files/processes | Failures leave owned staging cleanup and visible errors; document atomicity, concurrent-writer and durability limits separately |

Use paused time, channel gates, fixed records and temporary fixtures. Real OS
smoke tests are bounded supplements. Do not substitute sleeps, callback counts
or a successful cleanup waiter for actual resource-release/error assertions.

## Toolchain matrix

Run targeted tests first. Before marking a slice accepted, execute workspace,
M0 and documentation checks using the explicit tools below. Commands are
reproduction instructions, not evidence that future slices have passed.

```sh
set -eu
for cordis_tc in stable 1.85.0; do
  (
    set -eu
    cordis_rustc=$(rustup which --toolchain "$cordis_tc" rustc)
    cordis_cargo=$(rustup which --toolchain "$cordis_tc" cargo)
    cordis_rustdoc=$(rustup which --toolchain "$cordis_tc" rustdoc)
    cordis_clippy=$(rustup which --toolchain "$cordis_tc" cargo-clippy)
    cordis_driver=$(rustup which --toolchain "$cordis_tc" clippy-driver)
    cordis_bin=$(dirname "$cordis_rustc")
    test "$(dirname "$cordis_clippy")" = "$cordis_bin"
    test "$(dirname "$cordis_driver")" = "$cordis_bin"
    export PATH="$cordis_bin:$PATH"
    export RUSTUP_TOOLCHAIN="$cordis_tc"
    export RUSTC="$cordis_rustc" RUSTDOC="$cordis_rustdoc"
    export CARGO_TARGET_DIR="$PWD/target/acceptance-$cordis_tc"
    export RUSTDOCFLAGS='-D warnings'
    "$cordis_rustc" --version
    "$cordis_cargo" --version
    "$cordis_rustdoc" --version
    "$cordis_clippy" --version
    "$cordis_cargo" check --workspace --all-targets --locked
    "$cordis_cargo" test --workspace --locked
    "$cordis_cargo" clippy --workspace --all-targets --locked -- -D warnings
    "$cordis_cargo" doc --workspace --no-deps --locked
    "$cordis_cargo" test --manifest-path spikes/m0/Cargo.toml --locked
    "$cordis_cargo" clippy --manifest-path spikes/m0/Cargo.toml --all-targets --locked -- -D warnings
    if test "$cordis_tc" = stable; then
      "$cordis_cargo" fmt --all --check
      "$cordis_cargo" fmt --manifest-path spikes/m0/Cargo.toml --check
    fi
  ) || exit 1
done
python3 -B tests/docs/test_check.py
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
python3 tests/parity/run.py
```

Run from repository root. Missing toolchain components or offline dependencies
are explicit setup failures, never a reason to fall through to Homebrew/system
tools. Follow the pinned parity runner's existing setup; do not substitute a
different JS runner. Its source pin does not freeze every transitive JS
dependency. Preserve the six scenarios / 73 observations.

Additionally run each changed crate's runnable example and any generated
external application. Cargo test does not execute all examples as applications.
Use the same selected compiler environment for these commands. New suite
counts describe tests actually run, not an inferred one-to-one source mapping.

## Documentation and commit gate

- [ ] Update README.md, CHANGELOG.md, AGENTS.md, docs/todo.md, docs/decision.md,
  docs/limitation.md and docs/llms.txt where the slice changes current claims.
- [ ] Add matched English/Vietnamese guide/API pages, export indexes, examples
  and glossary terms for each shipped public contract.
- [ ] Extend tests/docs/check.py per delivered crate. It now covers core,
  timer, logger, loader, include, HMR and Create. Create additionally executes
  live CLI/help, exact generated files, Cargo metadata and external applications
  on both toolchains; future crate exports/snippets must be checked too.
- [ ] Keep proposed APIs in planning docs until implemented. Do not insert
  unavailable examples into current user guides merely to illustrate a plan.
- [ ] Record baseline and final revision, commands, tool versions, outcomes,
  counts, deviations and remaining work IDs in the slice's evidence.
- [ ] Inspect the diff; commit code and relevant docs together without
  Co-authored-by, and do not push automatically.

Planning-only edits use link/structure/status consistency checks; the full
Rust matrix is for implementation acceptance, not every prose edit.

## R0 — Whole-program integration acceptance

**Completed:** independently accepted on 2026-11-04; see [R0 evidence](evidence/r0-acceptance.md).

**Entry:** T5, C3, L5, G1, I4, H3 and S2 accepted within reviewed scope.
**Files:** crates/re-cordis-hmr/examples/system_lifecycle.rs and
crates/re-cordis-hmr/tests/system_lifecycle.rs, sharing
crates/re-cordis-hmr/examples/system_lifecycle/app.rs. Console is a dev-only
integration dependency; Include uses the existing optional feature.
The documentation checker extends actual Create-generated temporary output
with the same application/tests and explicit local dependencies.

- [x] Boot a generated application with logger, typed provider/consumer,
  timer, loader groups, include source and optional watcher.
- [x] Change config, move a provider between isolation groups, hold consumer
  cleanup, replace a linked factory, fail/recover a load and shut down while
  a file write/watch event is pending. Assert causal order and original errors.
- [x] Prove all owned tasks, streams, writers, watches, journals and children
  release at the documented barrier; unrelated siblings still progress.
- [x] Preserve the existing GitHub application tests and offline examples.
  Add no live external calls to the ordinary test suite.
- [x] Reconcile composition-relevant source dispositions using accepted ledgers. Keep separate totals for
  executed equivalents, Rust representations, exclusions and open defects;
  do not collapse these into a misleading completion percentage.
- [x] Run the complete matrix, review the integration diff and commit.

## R1 — API/package readiness and release decision

**Entry:** R0. This prepares a release proposal; it does not publish.
**Files:** docs/release-readiness.md, package manifests only after the
corresponding packaging decision, and generated-template dependency metadata.

- [ ] Review crate names, exports, dependencies/features, MSRV, licensing
  notices, package contents, README manifests and documentation links.
- [ ] Inspect distribution with Cargo metadata and packaging checks appropriate
  to unpublished path dependencies. Do not claim publish dry-run can validate
  unresolved unpublished dependencies.
- [ ] Define release order from actual Cargo dependencies. Replace local
  scaffold dependencies with tested versions only after publication.
- [ ] Decide supported OSes, semver/stability policy, version numbers and
  whether to add a facade crate. Avoid one solely for symmetry.
- [ ] Present the concrete release proposal for a separate user decision.
  Keep publish=false until that decision is made.
