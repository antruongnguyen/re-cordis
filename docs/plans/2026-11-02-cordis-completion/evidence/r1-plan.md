# R1 execution plan

Date: 2026-11-05 (Asia/Ho_Chi_Minh).
Baseline: e3f0f656aabfecb0088c5d63e1306231bfbe0188, clean
feat/cordis-completion. No existing R1 work or newer changes were discarded.

## Scope and carried evidence

Complete API/package inspection, distribution evidence, a concrete release
proposal, independent review, matching documentation and local acceptance.
Keep 0.0.0/publish=false, Rust 2024/resolver 3/MSRV 1.85. No publication, push,
tags, facade implementation, remote Create, new templates, platform/CI
expansion, standalone B1 or renewed B0 audit.

Carry accepted R0 executable 35556d5108eaf13104a5fb287c83b28ffb5d1576,
checker 4742efe44a55dd1464eb851f9d9eb11d66fde82b, Create 8373fa3/e8b5eb5,
HMR 55a55d1/4c8f423 and earlier accepted satellites/core/B0.
See [R0 acceptance](r0-acceptance.md), [source map](r0-source-map.md),
[decisions](../../../decision.md) and [limitations](../../../limitation.md).
R0's 40 stages, 517 workspace checks/toolchain and 166 input fingerprints
are carried evidence only where unchanged inputs establish applicability.
Filesystem/watcher execution covers macOS only.

## Execution and verification gates

1. Audit manifests, root exports, features/dependency resolution, MSRV,
   license notices, package contents, README metadata and generated manifests.
   Save exact commands, exits, tool paths/versions and reproducible findings.
2. Distinguish metadata and package file lists from archive construction,
   verification builds and publish dry-runs. Retain expected failures for
   unresolved unpublished path dependencies; never disguise them using patches.
3. Correct only demonstrated packaging/adoption problems, then write
   docs/release-readiness.md with scope, version/stability proposal, dependency
   order, platform evidence, facade decision, blockers and remaining decisions.
   Production dependency edges/features and the basic Create template stay
   unchanged unless a concrete defect requires correction.
4. Update status/decision/limitation and paired user documentation. Run
   python3 -B tests/docs/test_check.py and the documentation checker on explicit
   stable and 1.85.0. Compare current executable/fixture inputs to R0.
   Code/dependency/feature changes require focused checks and the full shared
   matrix; prose/package-document additions require package and docs checks.
5. Obtain independent astra/high review, correct actionable findings, and
   request fresh review of precise correction deltas. Commit each verified
   milestone/correction locally; finish with clean task-owned work.

## Ownership

Fresh gpt-6.1-sol/high r1_package_audit owns only r1-package-audit.md
and ignored target/r1-audit artifacts. It has the exclusive Cargo lease during
initial package evidence collection. The controller owns status/user docs and
focused package corrections. Fresh gpt-6-astra/high review owns only its report
and isolated ignored review artifacts. Shared writes and Cargo/docs/parity
execution are serialized; all workers start with fork_turns=none.
