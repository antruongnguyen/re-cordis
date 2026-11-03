# Include implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Load configuration sources, apply ordered patches, preserve provenance
and reconcile runtime/file changes without silently losing either side's data.
**Architecture:** re-cordis-include owns file snapshots, patch ownership and a
pending-change journal; loader owns entries and lifecycle. Explicit refresh
works without a watcher. HMR integration is optional and later.
**Tech stack:** Rust 2024, loader/core, selected JSON/YAML parser and filesystem
I/O. Async I/O and timer dependencies are added only where required.
**Spec:** [Master design](README.md); pinned packages/include/src/{index,patch,journal}.ts
and seven source test suites in [the inventory](source-inventory.md).

## Global constraints

Rust edition 2024; resolver 3; minimum Rust 1.85, tested with 1.85.0. MIT;
version 0.0.0 and publish=false. No persistence logic in core; no implicit
watcher dependency. A successfully loaded initial file is only I1 completion.

## Source behavior to preserve

The upstream supports JSON/YAML, source locations, nested includes, initial
content only for a missing file, ordered patches, anonymous IDs and a change
journal. Patch ownership is per entry and per key; updates to patch-owned
values must go back to their actual owner, not the underlying included file.
It coalesces changes, rereads before writing, preserves runtime changes after
write failure, gives the file precedence on conflicting edits with a report,
and flushes/retries pending work during disposal.

## I0 — File format, ownership and durability gate

**Entry:** L4/L5 source and commit contracts and G1 group behavior accepted.
**Files:** this plan, docs/decision.md, docs/limitation.md.

- [x] Specify Include configuration, source snapshot, structured source path/
  location, patch, change journal, refresh/flush and disposal result signatures.
- [x] Choose JSON first; compare YAML parsers for YAML 1.2/core scalar behavior,
  duplicate keys, aliases, tags, spans and comment/style retention. Do not
  claim lossless editing using a parser that only returns values.
- [x] Define support for YAML aliases/anchors and !js/expression nodes. Without
  a JS engine reject unsupported executable nodes explicitly; preserve source
  diagnostics. Value-only YAML support has a clearly labelled boundary.
- [x] Define base path/nested resolution, absolute and Windows paths, symlink
  identity, include cycle detection and limits on bytes/depth/entries/patches.
- [x] Specify file-vs-runtime conflict granularity using pinned journal.ts and
  sync.spec.ts; decide atomic replacement and crash-durability promises
  separately. An atomic rename is not a complete concurrent-writer protocol.
- [x] Choose serialized writer ownership, reread/reconcile before each retry,
  bounded transient retry, permanent error reporting and disposal flush failure.
  Document residual races with uncooperative external writers; do not promise
  arbitrary concurrent-writer safety without an enforceable protocol.

**Exit:** accepted schema and conflict/format policy, parser capability fixture
results, file-operation limits and platform claims.

## I1 — Read, locate and refresh configuration

**Files to create:** crates/re-cordis-include/Cargo.toml, src/lib.rs,
src/source.rs, tests/read.rs, tests/locations.rs, tests/fixtures/.

- [x] Test missing file with/without initial data, malformed existing file,
  non-array input, invalid individual entries, nested groups, path bases and
  source-originated versus runtime-originated validation errors.
- [x] Read bounded input, parse the selected format and reconcile through
  loader. Never overwrite an existing invalid file with initial configuration.
- [x] Preserve last valid running state for an unreadable/unparseable source;
  apply the reviewed per-entry invalidation policy for a structurally parsed source.
- [x] Implement explicit refresh and deduplicate identical content. Test
  invalid→valid recovery, renamed entry factories and nested include cycles.
- [x] Add tested filename/path/span mapping, including UTF-8 versus upstream
  UTF-16 column conventions. Do not invent precise locations for runtime data.
- [x] Run `cargo test -p re-cordis-include --test read --test locations --locked`,
  update paired docs and commit.

## I2 — Ordered patches and provenance

**Files:** create src/patch.rs, tests/patches.rs; extend tests/locations.rs.

- [x] Test overrides, disable, matching/mismatched names, missing IDs, insert
  at root/group, invalid non-group target and multiple ordered patches.
- [x] Preserve the case where a later patch addresses an entry inserted by an
  earlier patch. Replacing a group's config must remove stale child ownership.
- [x] Track the effective owner of each entry and key, including embedded
  group configs, nested includes and keys overridden by later patches.
- [x] Keep anonymous IDs stable without eagerly writing them; avoid collisions
  with explicit source IDs and patch insert IDs at every nesting level.
- [x] Test removing a patch restores source values and skipped patches own
  nothing. Locate errors in the owning file/patch. Run tests and commit.

## I3 — Journal reconciliation and persistence

**Files:** create src/journal.rs, src/persist.rs, tests/reconcile.rs, tests/write.rs.
**Consumes:** loader commit records with origin and I2 ownership.

- [x] Test coalesced updates, create-then-remove, moves plus updates, separate
  file/runtime edits, same-entry conflict, and both refresh/update orderings.
- [x] Implement the source-backed three-way merge of last snapshot, latest
  file and journal. Apply file precedence on the defined conflict and report it.
- [x] Route runtime changes to file-owned, overridden, inserted and embedded
  patch values; recompute ownership after moves/removals. File-originated
  reconciliation must not generate a write-back feedback loop.
- [x] Write through a sibling temporary file and accepted replacement policy.
  Test short writes, permission errors, interrupted replacement, transient
  rename retries and an external edit during the retry window.
- [x] Never overwrite an unparseable current file. Preserve pending runtime
  changes in memory after failure; retry on a later refresh/flush/disposal.
- [x] Test journal capacity limits and overflow behavior, so a persistent
  failure cannot create an unbounded backlog. Gate I/O with a test seam and
  use temporary directories; do not make chmod the sole failure oracle.
- [x] Run reconcile/write tests, document durability limits, and commit.

## I4 — Joined disposal, YAML completion and acceptance

**Files:** tests/lifecycle.rs, selected YAML capability tests,
examples/config_file.rs, paired guides/include.md and api/include.md, indexes.

- [x] Stop admitting new mutations, drain/flush the journal, then stop the
  loader subtree in the chosen order; prove no write/callback outlives joined
  disposal. A failed final write must remain visible in the cleanup result.
- [x] Test source replacement during apply, repeated refresh/reconfigure,
  slow child teardown, nested source removal and restoration of all baselines.
- [x] Finish approved YAML round-trip/location cases. If canonical rewriting
  or read-only YAML was selected, list affected source cases as deviations
  and do not mark lossless YAML accepted.
- [x] Complete all seven suite dispositions, run shared acceptance, demonstrate
  offline read/change/flush/shutdown, and commit.

Watch integration is H1/H3. Include remains usable through explicit refresh
even when no filesystem watcher is installed.

I0 decisions and executed parser fixtures: [contract](evidence/include-contract.md).
Current suite boundaries: [source map](evidence/include-source-map.md).

I0–I4 is accepted at final code `0a2df1e`; see [acceptance](evidence/include-acceptance.md)
and [final scoped re-check](evidence/include-final-rereview.md). HMR/create remain
outside the selected include delivery.
