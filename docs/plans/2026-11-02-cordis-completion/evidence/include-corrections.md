# Include final-review corrections (2026-11-03)

Scope: IR1–IR5 from [the independent review](include-review.md), against I3
`15670ea` plus the frozen I4 implementation. Accepted core/loader `101f37b`
is reused unchanged. No existing test, example, core or loader file was edited
by this correction worker. Parent-owned paired documentation and final shared
acceptance remain with the parent.

## Corrections

- **IR1:** Reconciliation includes the surviving pending placement graph, with
  children carried by creation snapshots and transitive parent dependencies.
  A genuinely removed file parent still wins and reports a conflict. Separate
  child creation, moves and coalesced edits survive a new parent and retries.
- **IR2:** Subtree removal journals deletions for descendants present in the
  accepted baseline, including entries moved into the subtree. Runtime-only
  descendants cancel their pending records. Removing a runtime-created group
  also deletes baseline entries that were moved into it.
- **IR3:** ID reservation/assignment and source discovery follow actual entry
  addresses collected through structural expansion. Arbitrary plugin arrays
  with `id`, `name`, `group` or include-looking objects remain value-identical
  in source reads, ordinary plugin patches and runtime creation. No file read
  or identity reservation occurs through those opaque values or skipped patch
  configs. Explicit IDs in loaded sources and structural patches are reserved
  before anonymous IDs are generated.
- **IR4:** Nested files load during structural expansion, including applied
  replacement-group configs. Relative paths use the actual config owner's
  file; ordinary config supplied by an earlier patch expands if a later patch
  converts its plugin to a group. Inserted plugin configs later replaced are
  opaque until their structural context applies. Redundant group patches keep
  already expanded children and their original file provenance. Initial source
  loading retains byte/depth/file bounds and canonical cycle/duplicate-mount
  checks; already loaded journal models retain the prior ability to expand
  temporary repeated descriptors while a move is routed.
- **IR5:** Ordered override patches accept `unset:["key", ...]`, applied after
  ordinary overrides. The array must contain strings other than the controls
  `id`, `name`, `insert`, `unset`; invalid input skips the entire patch with
  a located warning. Unset keys retain their patch owner despite being absent.
  Removing a patch-owned effective field writes that tombstone to its actual
  owner and preserves the hidden base. A later set clears its tombstone.
  Config absence and explicit `config:null` remain distinct. Direct file and
  embedded-entry removals still remove their actual raw key directly. Name
  guards and skipped patches retain their existing ownership semantics.

The source preparation uses the existing structural expansion twice: first
collect actual raw entry addresses/load the graph without generated IDs, then
reserve explicit IDs and normalize only those entries before final expansion.
It introduces no second patch interpreter and no filesystem I/O into journal
routing. Unset config/group keys are removed before structural expansion, so
values erased by that patch do not load files or acquire anonymous IDs.

## Reproduction and regressions

The six independent scratch probes all compiled and failed their expected
behavior assertions before production edits; the six new initial regressions
likewise failed (not setup or compile errors). All six original probes now
pass. Logs are under ignored `target/include-20261103/corrections/`:
`probe-red-CASE.log`, `probe-green-CASE.log`, `regressions-red.log`.

`crates/re-cordis-include/tests/review_repairs.rs` adds 15 public API tests:
creation retry; both edit/move orders into a pending parent; baseline versus
runtime subtree deletion; opaque source/patch/runtime config; nested relative
replacement include ownership; effective removal/null/set distinction;
earlier-patch group conversion and explicit-ID collision avoidance;
skipped/opaque/unset configs; inserted-plugin replacement; unset control/name
guards; last-patch/direct-embedded removal; redundant group provenance;
transitive pending groups after retry; actual file parent removal; and
replacement-patch cycle/duplicate-mount rejection with last-valid recovery.

Additional expected red transition logs:
`transitions-first.log` (earlier patch config later converted to group),
`redundant-group-red.log` (redundant group patch re-expanded normalized children).
Both are green in the final matrix. The existing descriptor-move lifecycle test
also remained unchanged and caught overly strict checks on intermediate journal
models; its final behavior remains green.

## Verification and freeze

Explicit rustup toolchains stable (Rust 1.97.1) and 1.85.0:

- `cargo test -p re-cordis-include --locked`: 69 Rust tests and 2 Rustdoc
  checks, all pass on both (71 total; original 56 retained plus 15 new tests).
- `cargo clippy -p re-cordis-include --all-targets --locked -- -D warnings`:
  passes on both. MSRV dispatch uses the resolved 1.85.0 `cargo-clippy` and
  `clippy-driver` explicitly with `RUSTUP_TOOLCHAIN=1.85.0`; an initial implicit
  Clippy dispatch selected stable against MSRV artifacts and was rerun after
  correcting tool selection. No dependency/core repair or target cleanup.
- Unchanged `cargo run -p re-cordis-include --example config_file --locked`:
  passes on both and reports persisted config 2 and joined Registry restoration.
- Scoped rustfmt check and `git diff --check`: pass.

Final logs: `test-{stable,1.85.0}.log`, `clippy-{stable,1.85.0}.log`,
`example-{stable,1.85.0}.log`. All final commands use resolved Cargo, RUSTC,
RUSTDOC and Clippy executables with a selected-toolchain/system-only PATH.

Correction production/tests are frozen for the parent's scoped independent
rereview and final workspace/documentation acceptance. No commit or push.

Frozen SHA-256 (include-relative paths):

| Path | SHA-256 |
|---|---|
| src/journal.rs | `8de4d9d1ad2f4c9c1d285fa7580928bae09f680ff0a6ff2463799e0accf5e630` |
| src/lib.rs | `09cd2d03cf302b868a48b9ee489b084550996b036fd142a318c009a37cebbf63` |
| src/patch.rs | `2093eee2dc2b5aedbff65eaa5c782a3a298ad4a3b6d51a6634bb2501b80a4532` |
| src/source.rs | `8c35b15f1ae5f2cb12c5fa5b1b2a8a5692ff90a73990e514a5600655792b5dfc` |
| tests/review_repairs.rs | `22d83dae91c206789a3f74609c20f7f0598cfd12ba71aaf31456f973ba07acfe` |
| tests/lifecycle.rs (unchanged) | `d8041fa90ba00a43fd6bc7bf6edf5789badc392bc62776f147424a236f440b77` |
| examples/config_file.rs (unchanged) | `71c5722a0de0ee12513e24f3521734062078d28eaf0ccdb342b5927d73f31960` |
