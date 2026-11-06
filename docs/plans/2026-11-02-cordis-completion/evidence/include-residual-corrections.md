# Include residual corrections (2026-11-03)

Scope: the two Important residuals IR2-R1 and IR4-R1 from the
[scoped independent re-review](include-rereview.md), against frozen
`e46e4c8c9cafe12eb38447c77fefadcdf1b03265` on `feat/cordis-completion`.
Accepted loader/core `101f37b` is reused. No public API, dependency, format,
durability or lifecycle scope was added. The parent owns shared documentation,
the final independent re-check, commits and workspace/documentation acceptance.

## Root causes and repairs

- **IR2-R1:** A single created group carries its embedded descendants twice:
  in its fallback snapshot and its coalesced `config` delta. Canceling only the
  descendants' individual records leaves both ancestor copies unchanged.
  Removal now prunes every removed subtree ID from surviving structural ancestor
  snapshots and structural config deltas before canceling runtime-only records
  or recording baseline deletion. It reuses the existing structural `detach`
  walker; ordinary plugin config is opaque. Baseline entries moved into a
  removed subtree still receive their own deletion records.
- **IR4-R1:** Initial structural discovery has no generated IDs, so patches
  addressing retained anonymous IDs are skipped before their new sources load.
  After that first discovery reserves explicit IDs at actual entry addresses,
  one optional additional discovery restores noncolliding cached IDs only into
  effective entry clones. It uses the same expansion and patch implementation.
  Newly reached files and entry addresses then participate in the existing
  explicit-ID reservation and anonymous normalization before final expansion.
  Raw files remain unchanged during discovery.

Discovery remains bounded: at most two discovery expansions plus the existing
final expansion, rather than a fixed-point loop or a second patch interpreter.
Each discovery retains per-file byte limits, depth/file limits and canonical
cycle/duplicate-mount checks. The cache remains positional, pruned and bounded
at 1024; explicit IDs retain precedence. Ordinary application config and skipped
patch config are not traversed. Final journal models use an empty discovery
identity map and `load_missing=false`, preserving the lack of filesystem reads
in journal routing. The extra discovery reuses already parsed documents.

## Red and green evidence

Two focused public-API regressions were appended before production edits.
Explicit stable Cargo compiled and ran them with the original 15 tests:
**15 passed, 2 failed at the expected behavior assertions**.

- `removed_embedded_created_subtree_stays_removed_after_flush`: the accepted
  removal left `g.config=[]`, but flush restored `child` and `leaf`.
- `stable_anonymous_patch_discovers_relative_source_and_child_owner`: the same
  raw `/entries/0` retained `include-anon-1`; refresh returned
  `missing included source` for the replacement patch's existing child file.

Log: ignored `target/include-residual-fixes/regressions-red-stable.log`.
The subtree regression also compiled and failed at that assertion on 1.85.0:
`regression-red-1.85.0.log`. The initial shell wrappers attempted to assign
zsh's read-only `status` after Cargo finished; the captured Cargo logs contain
the actual failing test results. Subsequent commands use `task_exit`.
The independent reviewer separately reproduced both residuals on both
toolchains; its exact red probe logs remain under `target/include-rereview/`.

The deletion-only correction then passed the exact subtree regression:
`deletion-green-stable.log`. After the identity correction, all 17 tests
passed: `regressions-green-stable.log`.

Four additional directly affected transition tests bring `review_repairs`
to **21 public API tests**, retaining all original tests byte-for-byte:

- Ancestor edit/move in both orders around cancellation, a moved baseline
  child, embedded leaf edits, opaque config containing matching IDs, failed
  flush and retry.
- A leaf moved out of the canceled embedded subtree remains alive and persists
  its later edit.
- A newly explicit ID takes precedence over the cached anonymous ID; a skipped
  guarded patch does not read its include-looking config, and the explicit
  entry's raw owner remains correct.
- Replacement patches against the stable anonymous ID retain cycle and
  duplicate-mount rejection, last-valid state and valid recovery.

The source/ownership regression also edits the included child and verifies that
flush writes the child file while retaining the parent's relative descriptor.

## Final verification

All commands use full executables resolved through
`rustup which --toolchain stable` / `rustup which --toolchain 1.85.0`,
matching `RUSTUP_TOOLCHAIN`, `RUSTC`, `RUSTDOC` and a PATH containing the
selected toolchain bin plus system directories. Clippy uses the resolved
`cargo-clippy` and `clippy-driver` explicitly, with matching `CARGO` and
`RUSTC_WORKSPACE_WRAPPER`.

| Check | stable | 1.85.0 |
|---|---|---|
| `cargo test -p re-cordis-include --locked` | 75 Rust tests + 2 Rustdoc checks pass | 75 Rust tests + 2 Rustdoc checks pass |
| `cargo-clippy clippy -p re-cordis-include --all-targets --locked -- -D warnings` | exit 0 | exit 0 |
| `cargo run -p re-cordis-include --example config_file --locked` | exit 0 | exit 0 |

The unchanged example reports config 2 persisted and joined shutdown restored
the Registry on both toolchains. Existing partial multi-file persistence,
descriptor moves, identity/cache bounds and journal recovery tests pass in the
complete package runs. Stable is Rust 1.97.1; the minimum toolchain is Rust 1.85.0.

Final logs under ignored `target/include-residual-fixes/`:
`include-tests-{stable,1.85.0}.log`, `clippy-{stable,1.85.0}.log`,
`example-{stable,1.85.0}.log`. MSRV artifacts use the isolated `msrv/`
target directory. Scoped rustfmt and `git diff --check` pass. Original tests
were preserved while rustfmt output for new edits was applied through IDE VFS.
No broad review or final workspace/documentation matrix is claimed here.

## Frozen fingerprints

Production and tests are frozen for the parent's scoped re-check. No worker
commit or push. Baseline hashes are retained in
`target/include-residual-fixes/before-sha256.txt`; final hashes in
`after-sha256.txt` matched after verification.

| Include-relative path | Final SHA-256 |
|---|---|
| src/lib.rs | `046131aaf4b8a71714fa68029964968aaf3ee0e6cfb0be22dbbafe1b31479c8e` |
| src/journal.rs | `6bdbac5b444cf6d4b34450b93d9c3ce326895bbc1f76ee848f74be29beb7bce6` |
| src/source.rs | `b4ee38c15924427dc65ae6b80a612f1fe5c32c515fba65fb0b284192c0bea3e9` |
| tests/review_repairs.rs | `51ef47ce160e4bf24f82224bdb56220fa7ab1998c9fa7929688514aa263f78ec` |
| src/patch.rs (unchanged) | `2093eee2dc2b5aedbff65eaa5c782a3a298ad4a3b6d51a6634bb2501b80a4532` |
| tests/lifecycle.rs (unchanged) | `d8041fa90ba00a43fd6bc7bf6edf5789badc392bc62776f147424a236f440b77` |
| examples/config_file.rs (unchanged) | `71c5722a0de0ee12513e24f3521734062078d28eaf0ccdb342b5927d73f31960` |

`format.rs` and `persist.rs` also retain their baseline fingerprints.
