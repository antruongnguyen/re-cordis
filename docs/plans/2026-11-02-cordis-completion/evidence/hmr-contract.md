# HMR H0–H3 Rust contract and execution plan

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Status: H0 independently approved; H1–H3 implementation acceptance pending.
Review: [H0 verdict and required oracles](hmr-h0-review.md).
Baseline: 47042d9; TypeScript: 35fdc969e4b6c48c48ed44717c7a018324e4dd88.

## Reused contracts and new work

Reuse include 0a2df1e and loader 101f37b, their final scoped reviews,
accepted timer T0–T5 and core task bridge. HMR adds local change coordination;
it does not reopen B0 evidence or recreate lifecycle drivers.
See [watch contract](hmr-watch-contract.md) and
[replacement contract](hmr-reload-contract.md) for exact APIs and probes.

| Capability | Rust contract |
|---|---|
| File/config watching | Explicit registered regular files; owned bounded polling producer, local callbacks, exact token removal and joined shutdown. No automatic recursive tree enumeration. |
| Include refresh | Optional HMR adapter invokes Include refresh for explicitly registered source paths. Include never depends on HMR. Invalid bytes preserve last-valid state; successful self-writes converge without a loop. |
| Linked factory replacement | Explicit factory dependency graph and heterogeneous compiled candidate batch. All declaration/type/preflight checks complete before any teardown. Loader owns per-entry lifecycle, config, providers and latest-intent checks. |
| Changed Rust source | Emit restart-required. Application decides how to rebuild/restart; no supervisor is provided. |
| Node/native loading | No inferred Rust module graph, Node cache, JS evaluation, dynamic ABI or binary hot swapping. |

Candidate preflight checks declaration/config compatibility and optional
application validation over retained source C. It does not move core's
dependency-ready Plugin validator earlier or promise transactional external
effects. Failed apply after teardown stays visible and needs an explicit
replacement/update to recover. No automatic rollback after unload.

## Implementation sequence and checks

- [x] H0: review exact signatures, bounds, path policy, preflight semantics
  and watcher dependency/MSRV probe. Record chosen tradeoffs in decision.md
  and limitation.md, then commit the contract.
- [ ] H1: add crate, synthetic events, bounded polling, debounce, registration
  tokens, diagnostics and joined cleanup. First prove duplicate/burst/overflow,
  ignored override, original errors and no work after disposal with deterministic
  tests; supplement with bounded atomic rename/remove/recreate smoke. Run focused
  tests and strict package Clippy on explicit stable and 1.85.0; commit.
- [ ] H2: implement the smallest proven loader batch seam and explicit HMR
  dependency plan. Test invalid whole-batch candidates, two configs, transitive
  dependencies, unchanged shared dependencies, slow independent siblings, exact
  retained providers, nested hosts, mutation during cleanup and explicit recovery.
  Run loader/HMR focused tests and strict Clippy on both toolchains; commit.
- [ ] H3: optional Include adapter, runnable offline reload example, paired
  guide/API pages and checker support. Record all pinned HMR declarations'
  dispositions, independent review findings and correction deltas. Run shared
  workspace/M0/docs/examples/parity acceptance on explicit stable and 1.85.0;
  commit verified corrections and final acceptance separately.

The parent owns shared manifests, documentation indexes/checker and progress.
Watch and replacement workers write only their assigned files. Reviewers write
reports and ignored probes; they do not change production files or Git state.
Logs and full command output live under target/hmr-*/. Every milestone and
correction batch receives a local commit after its focused checks pass.

## H3 same-instance Include composition

Include owns a private Loader, so the existing public API cannot replace linked
factories of the same instances whose configuration it refreshes. The selected
minimal seam is Include::replace_factories(Vec<FactoryReplacement>) returning
IncludeTask<Report>; it admits through Include and awaits the exact loader batch
Settlement, preserving scheduling failures. It changes no journal, provenance,
source data or file bytes and does not flush. Include still depends only on
loader, never HMR. Optional ReloadPlan::replace_include performs the same exact
candidate coverage check before forwarding. Tests must prove same-instance
configuration retention, unchanged pending edits/bytes, rejection before unload
and subsequent refresh using the new catalog. This is a concrete HMR usability
gap, not a reopening of accepted Include parsing/persistence contracts.
