# Create application scaffolding implementation plan

**Status: offline S0–S2 implemented, independently reviewed and accepted (2026-11-04).**
Final executable revision: `8373fa3`; see [acceptance](evidence/create-acceptance.md).

**Goal:** Generate a small, runnable Cargo application using accepted Cordis APIs
and explicit joined shutdown.
**Architecture:** stdlib-only library and standalone CLI, one bundled basic
template, exclusive sibling staging and single-rename destination commit.
**Spec:** [Master design](README.md), [selected contract](evidence/create-contract.md),
pinned TypeScript Create src/index.ts, src/bin.ts and tests/scaffold.spec.ts.

## Global constraints

Rust 2024, resolver 3, minimum Rust 1.85 tested with 1.85.0. MIT,
version 0.0.0 and publish=false. Explicit checkout path dependencies remain
usable while unpublished. No core contract change, push or publication.

## S0 — CLI and template gate: complete

- [x] Choose crate/binary `re-cordis-create` and public `create`/`CreateError`.
  Invocation: `cargo run -p re-cordis-create --offline --locked -- NAME --path DEST --cordis-path CHECKOUT`.
- [x] Define strict Cargo names, native destination paths, existing parent,
  absent/empty destination, noninteractive operation and exit codes 0/1/2.
  Refuse files/symlinks/nonempty destinations and the three bundled dependency
  package names; no destructive force option.
- [x] Bundle one basic template and validate the explicit local checkout.
  Escape canonical core/logger/loader path dependencies; no machine path or
  fictional crates.io release is embedded.
- [x] Generate local Runtime, owned console logger, typed plugin/service,
  programmatic loader config, actual update and explicit joined shutdown.
- [x] Omit Git initialization, dependency installation and launch options.
  The generator starts no subprocess; tests/checkers use process argument arrays.
- [x] Exclude remote/local arbitrary templates, transport/archive/cache and
  Include/HMR variants. Source exclusions are explicit, not passing tests.

## S1 — Offline generation and actual applications: complete

Files: `crates/re-cordis-create/{Cargo.toml,README.md,src,templates/basic,tests}`.
Generated set: Cargo.toml, src/main.rs, src/app.rs, README.md and .gitignore.

- [x] Regress invalid/reserved names, spaces/Unicode and escaped/token-shaped paths,
  missing parents, checkout layout, files/symlinks/nonempty/repeated destinations,
  real write/rename failure and failed owned-stage cleanup.
- [x] Prepare complete sibling staging, check destination again and rename once.
  Correct CR2 before-create exact/case-folded stage aliases; test final invisibility
  during staging and preserve unrelated contents.
- [x] Validate exact files, readable usage and real Cargo manifest/workspace/path
  metadata. Correct CR1's three conflicting local package identities before writes.
- [x] Build/test/run external Cargo applications on stable and 1.85.0.
  Five app tests include 100 lifecycle cycles, weak resource/capture release,
  real update, joined cleanup and distinct original error channels.
- [x] Commit verified implementation `d44620c` and correction `8373fa3` with docs.
  [Implementation evidence](evidence/create-implementation.md);
  [correction evidence](evidence/create-corrections.md).

## S2 — Offline distribution decision and acceptance: complete

- [x] Classify all 14 source declarations in the [source map](evidence/create-source-map.md).
- [x] Remote transport remains unselected. HTTP, metadata, timeout, archive and
  cache tests/modules are not applicable to this delivered slice and were not run.
  No stub downloader or fictitious transport coverage is shipped.
- [x] Obtain independent full implementation/application [review](evidence/create-review.md).
  Resolve CR1/CR2 in a verified correction commit; fresh scoped
  [re-review](evidence/create-rereview.md) accepts both with no new important defect.
- [x] Update current front-door/status/decision/limitation docs, matched EN/VI
  guide/API/index/glossary routes and llms.txt. Extend the existing checker for
  Create exports, live CLI help and actual generated external applications.
- [x] Pass all 34 shared stages on the final source: 508 workspace checks per
  toolchain, strict lint/docs, M0, six examples, existing paired traces and
  generated applications. Formatting uses explicitly resolved stable tools.
- [x] Record commands, revisions, versions, counts, source hashes, limits and
  final acceptance. Commit the closing documentation/status record.

Publication and crates.io template versions remain R1 decisions; R0/R1, standalone
B1 and CI/platform expansion were not started. Runtime filesystem evidence is
macOS only. See [acceptance](evidence/create-acceptance.md) for exact boundaries.
