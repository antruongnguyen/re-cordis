# Create S0–S2 offline acceptance

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
**Verdict: delivered and independently accepted within the minimal offline scope.**
Final executable/checker revision: `8373fa33ea165cf7d02ba7e515ce9ac085248ecf`
on `feat/cordis-completion`. The commit containing this report closes
documentation and acceptance status.

## Delivered contract and checkpoints

The unpublished stdlib-only `re-cordis-create` crate supplies a synchronous
`create` API and a standalone CLI with one bundled basic Cargo application:

```sh
cargo run -p re-cordis-create --offline --locked -- \
  my-app --path '../my app' --cordis-path .
cargo test --manifest-path '../my app/Cargo.toml' --offline
cargo run --manifest-path '../my app/Cargo.toml' --offline --locked
```

Run from the checkout; the destination parent must exist. The generator creates
exactly Cargo.toml, src/main.rs, src/app.rs, README.md and .gitignore.
It accepts an explicit checkout and writes escaped canonical local dependencies
for core, logger and loader. It starts no subprocess, installs nothing, performs
no network operation and refuses files, symlinks and nonempty destinations.
A complete exclusive sibling stage is committed by one rename. Errors preserve
operation/path/cause, including failed owned-stage cleanup. See the
[contract](create-contract.md), [guide](../../../en/guides/create.md) and
[API](../../../en/api/create.md).

| Milestone | Revision / evidence |
|---|---|
| Clean start and carried project instructions | `8124fed` |
| Accepted HMR prerequisite | acceptance `4c8f423`, executable `55a55d1` |
| Verified Create S0/S1 implementation and bilingual docs | `d44620c`; [implementation](create-implementation.md) |
| Initial independent implementation/application review | [review](create-review.md), two P2 findings |
| Verified CR1/CR2 correction plus checker precision | `8373fa3`; [corrections](create-corrections.md) |
| Fresh independent scoped re-review | [approved](create-rereview.md): CR1 and CR2 ADDRESSED |
| S2 closure | this report's containing commit |

Core M0–M6, timer, logger, loader/groups, Include and HMR remain accepted and
unchanged in production source. B0 is not reopened. The user selected only
offline Create S0–S2; B1, CI/platform expansion, R0/R1, push and publication
were not undertaken.

## Working application and corrected boundaries

The generated app uses current-thread Tokio and local Runtime, an owned console
writer, typed Plugin/ServiceKey, programmatic loader configuration, an actual
greeting update and explicit joined loader/root shutdown. Partial setup/update
errors also drain admitted resources. Main inspects Runtime failures as well as
returned errors; root CleanupReport and nested cleanup diagnostics stay distinct.

Its five tests check weak old/new service and writer/capture release, final
Registry/effect baselines, updated log output, original cleanup error causes and
failed writer setup. One repeats **100 actual lifecycle cycles**. Independent
review additionally failed logging during update and the final service read/log;
both paths returned visible errors and joined resource release. These are real
application executions, not compilation or a success-message-only oracle.

CR1 rejects the three fixed Cordis dependency package names before staging:
their separate path packages also have version 0.0.0 and collide in Cargo's lockfile.
Library/CLI tests preserve absent and empty destinations, with exit 2.
CR2 chooses opposite leading-dot status for staging before create_dir, keeping
exact and case-folded final paths separate. Tests observe that final paths remain
absent during staged writes, then contain exactly five files after commit.
Independent exact/uppercase probes and long/alternate-name cases pass.

## Final shared matrix

The saved [runner](create-run-matrix.py.txt) was executed from the repository root
as `python3 -B target/create-20261104/run_matrix.py` after copying that saved file.
It records commands, exits, timings and log paths in
`target/create-20261104/matrix.json`; tool paths/versions are in `tools.json`.

**34/34 stages passed** at `8373fa3`: 19 stable stages and 15 MSRV stages.
All **162 executable/fixture inputs** matched before and after the matrix.
`final-inputs.json` and `final-summary.json` retain the hashes and result.
The TypeScript checkout remained clean at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`.

| Executed check | stable 1.97.1 | Rust 1.85.0 |
|---|---:|---:|
| Workspace all-feature/all-target check | Pass | Pass |
| Workspace all-feature tests and Rustdoc checks | 508 passed | 508 passed |
| Create subset included above | 22 Rust tests | 22 Rust tests |
| Strict workspace all-feature/all-target Clippy | Pass | Pass |
| Warnings-denied workspace no-deps Rustdoc | Pass | Pass |
| HMR no-default-feature separate stage | 45 checks | 45 checks |
| Isolated M0 tests/Rustdoc and strict Clippy | 21 checks; pass | 21 checks; pass |
| Six offline examples | 6 passed | 6 passed |
| Paired documentation/export/link checks | 33 pairs; 2 Create exports | Same |
| Existing executable docs and external quick start | 13 snippets + app | Same |
| Actual Create CLI/help, file set and Cargo metadata | Pass | Pass |
| Generated external app build/test/run and strict Clippy | 5 tests; pass | 5 tests; pass |
| Core paired traces | 6 scenarios / 73 observations | Stable-only stage |
| Loader paired traces | 2 scenarios / 9 observations | Same |
| Documentation checker regressions | 8 passed | Toolchain-independent |
| Workspace/M0/generated-app formatting | Pass, explicit stable tools | Same source; no separate MSRV formatting gate |

The six examples are lifecycle, timer_lifecycle, console, boot_reconfigure,
config_file and reload. Create's 22 tests are 5 unit + 17 integration.
Generated app tests are outside the workspace count; subsets and repeated
worker/reviewer/parent runs must not be added as new unique tests.
The docs stage runs the new external Cargo app with spaces/Vietnamese Unicode
in its path, validates its own resolver-3 workspace, version 0.0.0, publish=false,
Rust 2024/MSRV 1.85 and canonical dependency paths, then builds/tests/runs it.
First resolution is offline from cached dependencies; later commands use its lockfile.

Every compilation/test/doc/lint run resolves cargo, rustc, rustdoc, cargo-clippy
and clippy-driver with rustup and matching RUSTUP_TOOLCHAIN, RUSTC, RUSTDOC and
leading toolchain PATH. Stable rustc/Cargo are 1.97.1; MSRV is 1.85.0.
Stable cargo-fmt/rustfmt are also resolved explicitly; rustfmt is 1.9.0-stable.
No Homebrew compiler supplied final evidence. The correction report records the
earlier optional rustfmt 1.85.0 component installation; it is not an MSRV gate.

Build artifacts use `target/create-acceptance-{stable,1.85.0}`; long logs remain
under target. Independent reports point to their separate probe logs.
The complete earlier `d44620c` matrix is preserved in `before-review/`
(503 workspace checks per toolchain), and is not substituted for final evidence.

## Source dispositions and remaining limits

The [source map](create-source-map.md), independently reviewed, accounts for all
14 pinned scaffold.spec.ts declarations: one Cargo-equivalent purpose,
one portable-failure/Cargo representation, ten unselected remote-transport
declarations and two npm/Yarn exclusions. No excluded or unexecuted declaration
is counted as a passing test; no downloader stub or new Create Rust/TS trace exists.

- One basic bundled template only; no arbitrary local/remote templates,
  registry/download/archive/cache, Git/install/launch options or Include/HMR variants.
- Crates and generated package remain unpublished 0.0.0, Rust 2024/MSRV 1.85.
  Checkout layout validation is deliberately narrow, not a general TOML parser
  or source audit. Canonical dependency paths need Unicode and must be updated
  if the checkout moves. Offline Cargo requires cached third-party dependencies.
- Runtime filesystem evidence is **arm64 macOS 26.7.1 only**. Existing HMR
  watcher evidence remains macOS-only too; no Windows/Linux runtime acceptance.
  Host filesystems may reject native byte sequences or empty-directory rename.
- Single rename is ordinary filesystem visibility, not power-loss durability
  or confinement against hostile ancestor replacement. No force overwrite exists;
  cleanup failure reports the owned staging path for manual recovery.
- No publication, stable API guarantee, R0 integration or R1 release decision.

## Final documentation closure

After the closing status/guide edits, both documentation runs passed again:
33 page pairs, 2 Create exports, 13 executable snippets, external quick start,
and actual generated application checks with all five tests. Eight checker
regressions and 12 plan/evidence link/fence checks pass. The final llms.txt
status was independently rechecked for structure, links and its 8 KiB bound.
All 162 executable hashes still match the accepted matrix; only documentation
and the independent review report changed after `8373fa3`.

Logs: `docs-final-{stable,1.85.0}.log` and `docs-final-regression.log`
under `target/create-20261104/`. The closing local commit carries this acceptance
record. Nothing is pushed or published; B1 and R0/R1 remain unselected.
