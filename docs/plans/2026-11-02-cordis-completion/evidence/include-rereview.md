# Include scoped independent re-review

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Reviewed checkpoint: `e46e4c8c9cafe12eb38447c77fefadcdf1b03265`.
Verdict: **changes required — two Important residuals block I0–I4 acceptance**.

All six original public probes now pass, and all 15 correction regressions pass
on explicit stable and 1.85.0 toolchains. Two directly affected transitions still
fail through public APIs on both toolchains. They are detailed below. No Critical
finding was observed in this scope.

## Scope and inputs

This is the requested final scoped re-review of IR1–IR5. It reuses the completed
broad [include review](include-review.md) and accepted loader/core `101f37b`
evidence; it does not reopen architecture, completed lifecycle review or upstream
assertion accounting. Production was frozen during these reads and executions.
The parent checkpointed the already-reviewed bytes while this review ran.

Read repository AGENTS.md, [contract](include-contract.md),
[implementation evidence](include-implementation.md),
[correction evidence](include-corrections.md), the correction delta, complete
`source.rs`, `patch.rs`, `journal.rs`, relevant public mutation methods in
`lib.rs`, and `tests/review_repairs.rs`. CodeGraph supplied current source;
RustRover VFS supplied documents and the ignored public probes. No production,
test, shared index, branch or HEAD changes were made by this reviewer.

## IR dispositions

| Finding | Scoped disposition |
|---|---|
| IR1 — pending created parent dependencies | Resolved in the reviewed cases. Created children and existing-entry moves survive pending parents, coalesced edits, transitive parent chains and failed-flush retry. Real file parent removal still wins with conflicts. |
| IR2 — deleting moved subtrees | Original moved-baseline-entry reproduction passes, for both existing and newly created destination groups. **Residual IR2-R1 below:** canceling a subtree carried inside an ancestor creation snapshot resurrects it on flush. |
| IR3 — opaque plugin config | Original corruption resolved. Ordinary source, patch and runtime-created plugin arrays retain their values; include-looking objects neither load files nor reserve IDs. The discovery/ID-order interaction with IR4 remains below. |
| IR4 — replacement-patch nested includes and owners | Explicit-ID replacements load relative nested sources and preserve child-file owners; redundant groups and conversion of an earlier config pass; cycle/duplicate mounting rejection remains. **Residual IR4-R1 below:** replacement patches addressing stable anonymous targets cannot discover a newly referenced source. |
| IR5 — patch-owned effective absence | Resolved in the reviewed cases. `unset` preserves hidden base values and effective absence, the last effective patch retains ownership, setting clears its tombstone, config absence differs from JSON null, direct embedded removal edits its raw key, and invalid controls/name guards skip appropriately. |

## Important residuals

### IR2-R1 — P1: Remove canceled descendants from carried ancestor creation snapshots

Location: [lib.rs:813](../../../../crates/re-cordis-include/src/lib.rs#L813)
through line 818; the pending snapshot is later routed by
[journal.rs:521](../../../../crates/re-cordis-include/src/journal.rs#L521).

Start with `[]`, then await these operations:

```rust
include.create(json!({
    "id":"g", "group":true, "config":[{
        "id":"child", "group":true, "config":[{"id":"leaf","name":"worker"}]
    }]
}), None, 0);
include.remove("child");
include.flush();
```

The subtree and its leaf came from one admitted group creation. Neither is in
the accepted file baseline or has its own journal record. The new removal branch
therefore skips both IDs, but the surviving `g` record still contains them in its
creation snapshot/config. Flush succeeds without a conflict, restores the removed
subtree and leaf, writes them to disk and clears the journal.

```text
before=[{"config":[],"group":true,"id":"g"}] pending=1
after=[{"config":[{"config":[{"id":"leaf","name":"worker"}],"group":true,"id":"child"}],"group":true,"id":"g"}] pending=0 conflicts=[]
```

Canceling runtime-only descendants must also remove their representation from
surviving ancestor creation records, while preserving the corrected deletion
intent for baseline entries moved into a removed subtree. Add this one-create,
nested-subtree case to the focused regressions; its successful flush must retain
`g` with empty config and no child/leaf in effective or persisted data.

Public reproduction: `target/include-rereview/src/bin/transitions.rs`, case
`created_snapshot_subtree`; both toolchains exit 101 at the behavior assertion.
Logs: `target/include-rereview/created_snapshot_subtree-{stable,1.85.0}.log`.

### IR4-R1 — P2: Discover applied replacement sources for stable anonymous targets

Location: [source.rs:211](../../../../crates/re-cordis-include/src/source.rs#L211)
through line 246; patch target lookup skips absent IDs at
[patch.rs:141](../../../../crates/re-cordis-include/src/patch.rs#L141).

Read `{"entries":[{"group":true,"config":[]}]}` and retain the observed ID
`include-anon-1`. Add a replacement patch without changing the existing entry's
raw address `/entries/0`:

```json
{
  "entries": [{"group":true,"config":[]}],
  "patches": [{"id":"include-anon-1","config":[
    {"id":"nested","name":"include","config":{"path":"child.json"}}
  ]}]
}
```

`child.json` exists and contains a valid `worker` entry. Refresh fails with
`missing included source` (no location). The first discovery expansion has not
restored/generated the anonymous target ID, so it skips this patch and does not
load the child. The later expansion assigns the retained ID, applies the patch,
and encounters the unloaded child. The explicit-ID version passes.

```text
generated_id=include-anon-1
refresh_error=Some(IncludeError { message: "missing included source", location: None, conflicts: [], cause: None })
```

Source discovery must account for patches that become applicable to retained
anonymous IDs, while retaining explicit-ID collision avoidance and opacity of
ordinary/skipped plugin config. The regression must preserve the same file and
raw position across refresh; moving a bare array entry into an entries wrapper
would legitimately change its positional identity and is not this finding.

Public reproduction: the same `transitions.rs`, case `anonymous_patch_include`;
both toolchains exit 101 after the actual refresh error. Logs:
`target/include-rereview/anonymous_patch_include-{stable,1.85.0}.log`.

## Actual independent verification

Tool executables were resolved with `rustup which --toolchain stable` and
`rustup which --toolchain 1.85.0`. Every Cargo invocation used its full resolved
path, matching explicit `RUSTC`, `RUSTDOC` and `RUSTUP_TOOLCHAIN`, and PATH
containing only that toolchain's bin followed by
`/usr/bin:/bin:/usr/sbin:/sbin`. `cargo-clippy` and `clippy-driver` were also
resolved for each toolchain, but Clippy execution belongs to the parent's full
matrix and is not claimed as independent evidence here.

```text
stable: rustc/rustdoc 1.97.1 (8bab26f4f 2026-07-14)
        cargo 1.97.1 (c980f4866 2026-06-30)
1.85.0: rustc/rustdoc 1.85.0 (4d91de4e4 2025-02-17)
        cargo 1.85.0 (d73d2caf9 2024-12-31)
```

All artifacts/logs are isolated under ignored `target/include-rereview/`:

- `cargo build --manifest-path target/include-rereview/Cargo.toml --offline`:
  exit 0 on both; `build-{stable,1.85.0}.log`.
- Exact copies of the six original probe cases (`created_parent`, `move_parent`,
  `remove_moved`, `config_array`, `patch_include`, `remove_patch_field`):
  each exits 0 on both; `CASE-{stable,1.85.0}.log`.
- `cargo test -p re-cordis-include --test review_repairs --locked`:
  15 passed, 0 failed on each; `review-repairs-{stable,1.85.0}.log`.
- The two additional transition cases described above: each exits 101 on both
  from failed behavior assertions after successful build and public operations.
  These are not compilation, catalog-decoder, host or setup failures.
- SHA-256 verification of all five frozen inputs below: all match after tests.
  `git diff --check`: exit 0.

Stable scratch build target: `target/include-rereview/target`.
MSRV scratch build target: `target/include-rereview/target-msrv`.
The source/manifest are retained in that scratch package for exact reruns.
The parent owns workspace, docs, examples, Rustdoc and full Clippy/MSRV acceptance.
The broad review's accepted durability/lifecycle/platform boundaries are reused;
this report makes no stronger compatibility, concurrency or filesystem claims.

## Reviewed source fingerprints

SHA-256, unchanged throughout the review and matching `e46e4c8`:

| Include-relative path | SHA-256 |
|---|---|
| src/lib.rs | `09cd2d03cf302b868a48b9ee489b084550996b036fd142a318c009a37cebbf63` |
| src/journal.rs | `8de4d9d1ad2f4c9c1d285fa7580928bae09f680ff0a6ff2463799e0accf5e630` |
| src/patch.rs | `2093eee2dc2b5aedbff65eaa5c782a3a298ad4a3b6d51a6634bb2501b80a4532` |
| src/source.rs | `8c35b15f1ae5f2cb12c5fa5b1b2a8a5692ff90a73990e514a5600655792b5dfc` |
| tests/review_repairs.rs | `22d83dae91c206789a3f74609c20f7f0598cfd12ba71aaf31456f973ba07acfe` |

Further independent review should be limited to these two residual corrections
and their directly affected regressions. No broad audit restart is required.
