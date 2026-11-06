# Include final independent review

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Verdict: **changes required; I0–I4 acceptance is blocked by IR1–IR5 below**.

This review assesses the declared idiomatic Rust include contract. It reuses
accepted loader/core implementation `101f37b`, `loader-acceptance.md` and
`loader-rereview.md`. It does not reopen loader/core or claim 89 executed
TypeScript equivalents. All five findings are reproducible through public
Include APIs and concern promised behavior, rather than the accepted format,
durability or native-API limitations.

## Reviewed inputs

- Branch `feat/cordis-completion`; baseline `8ecbd6c`.
- I0 `261df14`, I1 `e90c791`, I2 `61b7f97`, I3
  `15670ea1337245342a1e19740122f2aeadb1a035` (HEAD).
- Frozen uncommitted I4: parser-cause changes in `format.rs`/`lib.rs`,
  `tests/lifecycle.rs`, and the complete `examples/config_file.rs`, including
  the parent's edit-report and root-cleanup assertions.
- Production read: `src/lib.rs` public admission, refresh, mutation, writer
  and disposal paths; complete `source.rs`, `patch.rs`, `journal.rs`,
  `persist.rs` and `format.rs`. Relevant tests read:
  `tests/reconcile.rs` and `tests/lifecycle.rs`.
- Contract/evidence read: repository `AGENTS.md`, work-package `include.md`
  and `acceptance.md`, `include-contract.md`,
  `include-implementation.md`, `include-boundary-tests.md`,
  `include-source-map.md`, and accepted loader evidence above.
- Current paired English/Vietnamese include API and guide, plus include
  sections in `docs/decision.md` and `docs/limitation.md`.

The parent owns the final workspace, documentation, example and MSRV matrix.
This reviewer did not repeat it or treat the implementer's reported counts
as independent verification.

## Findings

### IR1 — P1: Preserve dependencies on runtime-created parents during reconciliation

Evidence: [journal.rs:134](../../../../crates/re-cordis-include/src/journal.rs#L134)
through line 143, and lines 163–173.

Start with `[]`; create group `newg`, then create child `child` under it,
awaiting both operations, and flush. Both operations are admitted and the
runtime contains the child. Reconciliation checks the parent only against
the current file tree, where the new group is naturally absent, and discards
the child's journal record as if the file had removed the parent. Flush
succeeds, writes an empty group, removes the child and clears the journal.
The same check discards a move of an existing entry into a pending new group.

Observed `created_parent` probe:

```text
before=[{"config":[{"config":7,"id":"child","name":"worker"}],"group":true,"id":"newg"}] pending=2
after=[{"config":[],"group":true,"id":"newg"}] pending=0
conflict: file parent removed; runtime creation discarded
```

No external file edit occurs. A pending parent is not a file-side removal.
Reconcile structural dependencies against the surviving admitted journal as
well as the file tree. Preserve both separate child creation and existing-child
moves into a newly created group; retain the real file-removal conflict case.

### IR2 — P1: Removing a moved subtree must retain deletion intent for its former members

Evidence: [lib.rs:802](../../../../crates/re-cordis-include/src/lib.rs#L802)
through line 805.

Start with root entry `a` and empty group `g`. Move `a` into `g`,
then remove `g`, then flush. The live tree is empty after removal.
`remove` erases every descendant's pending record, including the only record
that moved `a` away from its original source position. The remaining
group-removal record deletes only the original empty group. Flush silently
restores `a` at the root, starts its plugin again, and clears the journal.

Observed `remove_moved` probe:

```text
before=[] pending=1
after=[{"config":1,"id":"a","name":"worker"}] pending=0 conflicts=[]
```

Coalescing subtree removal must distinguish newly created descendants from
existing entries moved into that subtree. Preserve deletion intent for existing
descendants instead of dropping their only pending change.

### IR3 — P1: Anonymous-ID assignment must not traverse ordinary application configuration

Evidence: [source.rs:249](../../../../crates/re-cordis-include/src/source.rs#L249)
through line 263; entry classification at lines 228–230.

An ordinary plugin may accept an arbitrary JSON array as its config. Reading

```json
[{"id":"a","name":"worker","config":[{"name":"Alice"}]}]
```

changes the application config to
`[{"id":"include-anon-1","name":"Alice"}]` before the catalog decoder sees it.
`assign` recurses into every array-valued `config`, even when its parent is
an ordinary plugin rather than a structural group, and classifies objects
with `name` as entries. This corrupts caller data and can cause a valid
strict application decoder to reject the source. A later write can persist
the added field.

The `config_array` public probe confirms the unexpected key on initial refresh.
Restrict ID assignment/reservation to actual entry collections, including
schema-aware structural-group patch configs, while leaving arbitrary plugin
config values intact.

### IR4 — P2: Discover nested sources inside replacement group-config patches

Evidence: [source.rs:117](../../../../crates/re-cordis-include/src/source.rs#L117)
through line 123; expansion at
[patch.rs:194](../../../../crates/re-cordis-include/src/patch.rs#L194).

With a valid `child.json`, this documented combination fails:

```json
{
  "entries": [{"id":"g","group":true,"config":[]}],
  "patches": [{"id":"g","config":[
    {"id":"nested","name":"include","config":{"path":"child.json"}}
  ]}]
}
```

The discovery pass visits `config` only when the raw containing object has
`group:true`. A patch addressing an existing group need not repeat that
field, so its child source is never loaded. The later patch expansion attempts
to use it and returns `missing included source` with no location. Initial
refresh fails; an already running graph cannot accept the otherwise valid
replacement. The `patch_include` probe reproduces this directly.

Discover sources in effective structural-group replacement configs, keeping
paths relative to the patch's owning file and preserving bounds/cycle checks.

### IR5 — P1: Removing a patch-owned effective field must not reveal a hidden base value

Evidence: [journal.rs:454](../../../../crates/re-cordis-include/src/journal.rs#L454)
through line 459.

Start with base entry `{"id":"a","name":"worker","config":1,"disabled":true}`
and patch `{"id":"a","disabled":false}`. Admit
`edit("a", JsonEdit::new().remove("disabled"))` and flush.
The API initially applies the effective field removal. Routing then simply
deletes the member from its patch owner, exposing the underlying
`disabled:true`. Flush succeeds with no conflict, re-disables the plugin,
and clears the journal.

Observed `remove_patch_field` probe:

```text
before=[{"config":1,"id":"a","name":"worker"}]
after=[{"config":1,"disabled":true,"id":"a","name":"worker"}] conflicts=[]
```

The paired guide promises removal of the effective top-level field.
Write-back must preserve that effective absence through patch layering,
including later patches and the distinction between config removal and
explicit JSON null. Do not accept a runtime removal and silently replace its
meaning at flush.

## Independent reproduction

Only isolated scratch files were created under ignored `target/include-review/`.
The public probe registers a linked `worker` plugin with an
`Option<serde_json::Value>` decoder, constructs Include inside
`Runtime::run`, and awaits every admitted operation. It neither accesses
private members nor modifies workspace production/tests.

Tools were resolved with `rustup which --toolchain stable` for Cargo,
rustc and rustdoc. PATH contained the selected toolchain bin followed by
`/usr/bin:/bin:/usr/sbin:/sbin`.

```text
rustc 1.97.1 (8bab26f4f 2026-07-14)
cargo 1.97.1 (c980f4866 2026-06-30)
rustdoc 1.97.1 (8bab26f4f 2026-07-14)
```

Commands and exact results:

- Explicit Cargo `build --manifest-path target/include-review/Cargo.toml --offline`:
  exit 0; log `target/include-review/build.log`.
- Run `target/include-review/target/debug/include-final-review-probes CASE`,
  with CASE `created_parent`, `move_parent`, `remove_moved`,
  `config_array`, or `patch_include`: each exits 101 at its behavior assertion.
  Each output is retained at `target/include-review/CASE.log`.
- Explicit Cargo `run --manifest-path target/include-review/Cargo.toml --offline
  -- remove_patch_field`: exit 101 at the effective-field assertion;
  `target/include-review/remove_patch_field.log`.
- Probe source and standalone manifest:
  `target/include-review/src/main.rs` and `target/include-review/Cargo.toml`.

All six executions compiled successfully. Failures are the expected assertions
shown above, after actual public operations produced the wrong observable
result; they are not setup, host, decoder or compilation failures.

## Scope and quality assessment

The dedicated local owner, private loader, explicit JSON admission, bounded
synchronous staging writer and reused loader lifecycle contracts are suitable
for the requested Rust scope. The reviewed disposal path performs a synchronous
final flush before requesting child teardown and retains the joined barrier;
the added lifecycle tests explicitly inspect file content during gated cleanup,
terminal entry states, Registry release, and original parser causes.

The known canonical JSON/YAML rewrite, unsupported aliases/tags/JS, native-only
paths, lack of fsync durability, non-atomic multi-file visibility and final
check-to-rename race are honestly documented adaptations. They do not explain
the five failures above, all of which use ordinary supported local operations.
No watcher, HMR, create-satellite, JS/native loading or core redesign is needed
to fix them. No unrelated cleanup or broad refactor is requested.

The offline example and paired guide use the same complete flow and inspect
edit/flush reports and final core diagnostics. The seven-suite disposition map
appropriately distinguishes selected Rust evidence from excluded/deferred
mechanisms. Delivery cannot be approved until IR1–IR5 have focused regressions
and corrections. Any subsequent review should inspect only those corrections
and their directly affected transitions, rather than repeating this broad audit.

## Frozen fingerprints

SHA-256 of the reviewed inputs, captured before writing this report:

| File within crates/re-cordis-include | SHA-256 |
|---|---|
| src/lib.rs | `5d06325d6aa41e8c7caa8afe07fb9edfd7a408cf633c3295b649916dbb889bce` |
| src/source.rs | `08c402a5829c0e4ac031bfc6cb40db9576cf28c187ddd7142525da9c4b476f5b` |
| src/patch.rs | `d74a4ff84afaf635d125ff5ec07ece7cf8dbc2d579d92020c430e69e39cb99b1` |
| src/journal.rs | `1e734253810c6d9c14b57b5a3ca15fe0a70d103a2ea8dee5a7c7fb2a012d4bc4` |
| src/persist.rs | `e0d28594b26e868eb86414244cca582c101820114aada8f3ef6dce2a4f52bdfc` |
| src/format.rs | `213a9261bf105b279abbeaf3685075bc7c9d66953ad707ce219e780b38d21192` |
| tests/lifecycle.rs | `d8041fa90ba00a43fd6bc7bf6edf5789badc392bc62776f147424a236f440b77` |
| examples/config_file.rs | `71c5722a0de0ee12513e24f3521734062078d28eaf0ccdb342b5927d73f31960` |
