# Include final residual re-review

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Reviewed correction checkpoint: `0a2df1e081ec385440048fc98222daeb4972e694`.
Verdict: **IR2-R1 and IR4-R1 resolved; no Critical or Important regression found in this correction delta.**
These two residuals no longer block I0–I4 acceptance. Full workspace and
documentation acceptance remains the parent's separate gate.

## Scope

This independent re-check covers only the two residuals in
[include-rereview.md](include-rereview.md), their corrections described in
[include-residual-corrections.md](include-residual-corrections.md), and directly
affected regressions. The production baseline is `e46e4c8`; initial review HEAD
`e2c480c` added documentation only. The parent committed the frozen corrections
as `0a2df1e` while this review ran, without changing the reviewed bytes.

Read AGENTS.md, both residual reports, the complete frozen four-file correction
diff, the relevant source-discovery/patch flow, journal snapshot/merge/routing
flow and public mutation methods, all six added tests, and the two unchanged
public probes. CodeGraph supplied production source; RustRover VFS supplied the
ignored probes and verification logs. The original 15 focused tests were
independently confirmed byte-for-byte preserved.

The completed broad review, prior IR1–IR5 dispositions, lifecycle review and
accepted core/loader `101f37b` evidence are reused. This report does not restart
those reviews or make broader compatibility, durability, concurrency or platform
claims. The reviewer changed no production/test files, shared index, branch or
HEAD. Only this report and ignored scratch artifacts were written.

## Per-finding verdicts

| Finding | Verdict | Evidence |
|---|---|---|
| IR2-R1 — canceled subtree carried by an ancestor creation | **Resolved** | Removal now prunes each canceled ID from both structural ancestor snapshots and coalesced structural config deltas before canceling records or recording baseline deletion. The exact one-create public reproduction leaves `g.config=[]` before and after flush, writes that empty config, reports no conflicts and clears the journal on both toolchains. |
| IR4-R1 — replacement include patch addressing a retained anonymous ID | **Resolved** | An optional second discovery restores noncolliding cached positional IDs into effective clones before applying patches and discovering sources. The exact public reproduction retains `include-anon-1` at unchanged `/entries/0`, discovers `child.json`, returns no error/failures and exposes its leaf on both toolchains. |

Direct regression checks also pass: ancestor edits/moves in both orders around
cancellation; moved baseline children; leaf edits; opaque config containing
matching IDs; failed flush and retry; a leaf moved out before subtree
cancellation; newly explicit ID precedence and guarded skipped config; relative
child-source ownership and child-only persistence; cycle and duplicate-mount
rejection with last-valid state and recovery.

The pruning walker descends through groups only and reuses the existing
structural detach implementation. The discovery correction reuses the existing
patch/expansion path, adds at most one discovery pass, leaves raw input unchanged
during discovery, and preserves `load_missing=false` with an empty identity map
for journal models. No directly introduced regression was found in these paths.

## Verification actually executed

Both toolchains were resolved through `rustup which --toolchain <tc>`.
Cargo commands used the full resolved executable, matching explicit
`RUSTUP_TOOLCHAIN`, `RUSTC`, `RUSTDOC`, and PATH consisting of the selected
toolchain bin plus `/usr/bin:/bin:/usr/sbin:/sbin`.

| Independent check | stable | 1.85.0 |
|---|---|---|
| `cargo test -p re-cordis-include --test review_repairs --locked` | 21 passed, 0 failed | 21 passed, 0 failed |
| `cargo build --manifest-path target/include-final-rereview/Cargo.toml --bin transitions --locked --offline` | exit 0 | exit 0 |
| Exact public `created_snapshot_subtree` probe | exit 0 | exit 0 |
| Exact public `anonymous_patch_include` probe | exit 0 | exit 0 |

Stable: rustc/rustdoc 1.97.1 (`8bab26f4f`), cargo 1.97.1 (`c980f4866`).
MSRV: rustc/rustdoc 1.85.0 (`4d91de4e4`), cargo 1.85.0 (`d73d2caf9`).

The scratch manifest, lockfile and probe source are exact copies from
`target/include-rereview/`. Builds use separate
`target/include-final-rereview/build-{stable,1.85.0}/` target directories.
Complete logs are retained in `target/include-final-rereview/`:
`review-repairs-{stable,1.85.0}.log`, `build-{stable,1.85.0}.log`,
`created_snapshot_subtree-{stable,1.85.0}.log` and
`anonymous_patch_include-{stable,1.85.0}.log`.

The supplied `target/include-resume-20261103/residual-review.diff` exactly
matches the four-file diff from `e46e4c8` to `0a2df1e`.
`git diff --check e46e4c8 0a2df1e` passes. All five frozen SHA-256 values below
match before and after execution. Full package tests, Clippy, examples,
workspace tests and documentation checks recorded by the worker/parent are
separate evidence and are not claimed as independently rerun here.

## Final reviewed fingerprints

| Include-relative path | SHA-256 |
|---|---|
| src/lib.rs | `046131aaf4b8a71714fa68029964968aaf3ee0e6cfb0be22dbbafe1b31479c8e` |
| src/journal.rs | `6bdbac5b444cf6d4b34450b93d9c3ce326895bbc1f76ee848f74be29beb7bce6` |
| src/source.rs | `b4ee38c15924427dc65ae6b80a612f1fe5c32c515fba65fb0b284192c0bea3e9` |
| src/patch.rs (unchanged) | `2093eee2dc2b5aedbff65eaa5c782a3a298ad4a3b6d51a6634bb2501b80a4532` |
| tests/review_repairs.rs | `51ef47ce160e4bf24f82224bdb56220fa7ab1998c9fa7929688514aa263f78ec` |
