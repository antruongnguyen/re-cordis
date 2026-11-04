# Create application scaffolding implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Generate a small, runnable Cargo application that uses the accepted
Cordis APIs and shuts down correctly.
**Architecture:** A CLI writes a reviewed bundled/local template into a new
destination. Optional template transport is isolated from rendering and writing.
**Tech stack:** Rust 2024, std filesystem/process APIs; select a CLI/transport
dependency only if the reviewed interface requires it.
**Spec:** [Master design](README.md), pinned packages/create/src/{index,bin}.ts
and tests/scaffold.spec.ts.

## Global constraints

Rust edition 2024; resolver 3; minimum Rust 1.85, tested with 1.85.0. MIT;
version 0.0.0 and publish=false. Generated dependencies must resolve while
unpublished; do not emit fictional crates.io versions.

Upstream downloads registry templates, handles malformed responses and
transport/archive failures, edits package metadata, optionally initializes Git
and installs/runs an application, and stages Yarn binaries. Rust needs Cargo
application generation; npm metadata/Yarn staging are explicit non-applicable
mechanisms. Transport failures remain relevant if remote templates are offered.

## Selected offline S0–S2 execution (2026-11-04)

The user selected one bundled basic Cargo application with explicit local checkout
dependencies, no remote templates and no automatic Git/install/run behavior.
The [concrete contract and execution plan](evidence/create-contract.md) governs
this slice; the [14-declaration source map](evidence/create-source-map.md) keeps
unselected transport/Yarn behavior separate from execution evidence.
Implementation, independent review and shared acceptance remain in progress.

## S0 — CLI and template gate

**Entry:** C3 and L5 provide the stable-enough application shape for this scope.
**Files:** this plan, docs/decision.md, a reviewed generated-file manifest.

- [ ] Choose the crate/binary name and whether this is a standalone command or
  Cargo subcommand; document invocation before implementation.
- [ ] Specify project name/path validation, destination-exists behavior,
  noninteractive operation, template selection and exit/error output. Recommend
  refusing nonempty destinations instead of a destructive force option.
- [ ] Choose a bundled/local template first. Define explicit checkout/path
  dependency input while crates are unpublished; avoid machine-specific paths.
- [ ] Select the minimal generated app: explicit local host, logger, a typed
  plugin/service, accepted loader configuration and joined shutdown. Additional
  include/HMR template variants depend on I4/H3 and are separate optional content.
- [ ] Treat Git init, dependency installation and launching as explicit options.
  No shell interpolation of project names/paths and no automatic push.
- [ ] Decide whether remote templates are necessary. If selected, define origin,
  version/integrity, timeout/byte bounds, cache policy and archive path rules
  before adding a downloader.

## S1 — Offline scaffolding and generated-app verification

**Files to create after name decision:** crates/re-cordis-create/Cargo.toml,
src/main.rs, src/lib.rs, templates/basic/, tests/scaffold.rs. The directory
name is proposed; S0 may choose a Cargo-subcommand-compatible binary name.

- [ ] Test invalid names, spaces/Unicode in paths, missing parent, nonempty
  destination, write failure, repeat invocation and no partial final destination.
- [ ] Render into a staging directory, verify its manifest/file set and move
  into the final destination using the reviewed collision policy. Leave unrelated
  user files unchanged and clean up only owned staging artifacts.
- [ ] Generate actual local dependency manifests; test exact files and readable
  instructions. Use process argument arrays for optional Git/Cargo commands.
- [ ] Build/test/run the generated application as an external Cargo project
  on stable and 1.85.0. Trigger an update and shutdown and inspect cleanup errors.
- [ ] Document the CLI in both languages and commit the accepted offline slice.

## S2 — Template distribution decision and satellite acceptance

**Files:** tests/transport.rs and a transport module only if remote scope is
selected; otherwise source-case ledger, CLI docs and examples.

- [ ] Classify every scaffold.spec.ts declaration: portable failure behavior,
  Cargo equivalent, remote-transport conditional case, or npm/Yarn-specific exclusion.
- [ ] If remote templates are selected, test with a local fixture server:
  missing version, malformed metadata, failed/no-body response, header/body
  timeout, dropped connection, invalid archive and missing/invalid manifest.
- [ ] Enforce archive traversal/symlink policy, integrity verification, total
  bytes/files limits and atomic cache/destination commit; failed downloads
  must not overwrite an existing template or project.
- [ ] If remote templates are not selected, close those cases with explicit
  reviewed exclusions; do not ship a stub downloader or claim those tests passed.
- [ ] Run shared acceptance and generated-app checks; record supported
  template versions/capabilities, document and commit.

Publication of the CLI and a crates.io dependency template are R1 decisions.
Local scaffolding can be completed and used before publication.
