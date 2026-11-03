# HMR implementation checkpoints

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Baseline: 47042d9 on feat/cordis-completion. H0 contract: 5065adc.
H1–H3 final independent review and shared acceptance remain pending.

## Loader batch seam checkpoint

The H2 prerequisite adds opaque FactoryReplacement values, optional hosted pure
preflight with a same-loader admission guard, and heterogeneous replace_factories.
All candidates are checked before catalog publication or reload teardown.
The singleton seam delegates to the batch. Per-entry scheduling observes current
identity/options and continues independent siblings after an observer error,
retaining that original failure in the returned Settlement.

Two public baseline probes failed on explicit stable and 1.85.0: sequential
singleton composition tore down A before rejecting B, and a commit callback
removing a sibling left an obsolete entry in the replacement pass. A third
in-tree RED test showed an observer error skipping the later sibling; the batch
now attempts it and retains the original cause. No core change was required.
See [replacement contract and probes](hmr-reload-contract.md).

Frozen source SHA-256:

- loader src/lib.rs: f771050c5c4f864bc47ad0ae46b01f6029f363471bc9055e0b8ec2b72456a50a
- loader tests/replacement_batch.rs: 76484c26e7e9a90f8e8a043daf0fe3bd1cda060b2d041fab10b710eb8d4f1059

Worker verification on explicit stable 1.97.1 and 1.85.0 passed 84 loader checks
(79 integration, three unit and two Rustdoc), strict all-target package Clippy
and warnings-denied no-deps Rustdoc. Parent independently reran all eight new
replacement_batch tests on both toolchains and matched both frozen fingerprints.
Cargo, rustc, rustdoc, cargo-clippy and clippy-driver were resolved through rustup;
RUSTUP_TOOLCHAIN, RUSTC, RUSTDOC and matching PATH were explicit.
Logs: target/hmr-h2/loader-{test,clippy,doc}-{stable,1.85.0}.log and
parent target/hmr-20261103/loader-checkpoint-{stable,1.85.0}.log.

Paired loader API/index and changelog edits accompany this checkpoint. Local
links/fences and seven documentation checker regressions pass. Full guide/API
compilation includes the concurrently developing HMR crate and remains the H3
gate; this checkpoint is not final H2 or whole-HMR acceptance.

## Paused H1/H2 implementation checkpoint

The parent independently verified 45 HMR checks, nine loader batch checks and
two Include linked-replacement checks on explicit stable 1.97.1 and 1.85.0.
Both documentation runs execute 13 snippets and the external quick start,
checking 31 bilingual page pairs and 19 HMR exports. H1 and H2 worker reports
record their full package/strict Clippy/Rustdoc checks. The code is a durable
implementation checkpoint, not acceptance: independent review confirms
[H2-R1](hmr-reload-review.md), which loses one distinct observer error when an
entry also has an apply failure. That correction and its scoped re-review remain.

The user requested a pause before leaving the office. All workers stopped.
The [Vietnamese resume handoff](hmr-resume/README.md) identifies exact remaining
work and preserves the H3 tests/example, failing H2 probe and matrix runner.
The optional watch_include adapter, offline example execution, final source-map
joins, independent watcher/adapter review and full shared matrix remain pending.
No push, publication, automation, create or R0/R1 work was started.
