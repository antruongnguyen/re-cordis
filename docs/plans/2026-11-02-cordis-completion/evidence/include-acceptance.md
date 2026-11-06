# Include I0–I4 acceptance

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Status: **accepted — I0–I4 complete within the documented Rust scope**.
Final production/test revision: `0a2df1e`.

## Scope and revisions

The selected scope is the unpublished `re-cordis-include` satellite only.
Accepted core/loader `101f37b` and its scoped review are reused without production
changes. Integration is `feat/cordis-completion`, starting at `8ecbd6c`.

| Checkpoint | Delivered work |
|---|---|
| `261df14` | I0 format/ownership/durability contracts and five YAML capability fixtures |
| `e90c791` | I1 bounded reads, locations, nested sources and explicit refresh/recovery |
| `61b7f97` | I2 ordered patches, stable anonymous IDs and per-key provenance |
| `15670ea` | I3 bounded journals, conflict reconciliation and recoverable writes |
| `e46e4c8` | I4 lifecycle/example, original parser causes and first IR1–IR5 corrections |
| `e2c480c` | Recovered verification checkpoint and independent scoped review |
| `0a2df1e` | Residual subtree-deletion and anonymous-target discovery fixes; final code |

Closing commits update acceptance/status evidence only; the verified executable
inputs remain unchanged.

## Delivered contract and evidence

- **Formats and paths:** strict JSON and restricted value-only YAML; actual source
  locations, canonical native file identity, missing-only initial content,
  nested paths/cycle rejection and explicit refresh with last-valid recovery.
  See [contract](include-contract.md) and [boundary tests](include-boundary-tests.md).
- **Patches and ownership:** ordered name guards/overrides/inserts, replacement
  groups and per-entry/per-key ownership. `unset` represents effective absence
  without exposing hidden base values or conflating absence with JSON null.
  Ordinary plugin JSON stays opaque to source discovery and anonymous IDs.
- **Persistence:** bounded coalescing journal, previous/current/runtime merge,
  reported file-winning conflicts, sibling staging, bounded retries and pending
  recovery after partial writes. Invalid current files are never overwritten.
- **Lifecycle:** host/owner admission and reentry checks; disposal flushes before
  child cleanup, shares its retained join, preserves final error causes and
  stops further admission. Tests assert terminal states, Registry/capture release,
  slow/dropped cleanup and 100 deterministic resource-baseline cycles.
- **Usability:** the offline `config_file` example reads config, changes it,
  checks flush output and joins shutdown. Matching EN/VI guides/API, all include
  exports and executable snippets are checked by the documentation runner.

The [seven-suite disposition map](include-source-map.md) covers the 89 pinned
TypeScript declarations by selected Rust behavior and deliberate boundaries;
it does not claim 89 executed equivalents. Existing six core scenarios/73
observations and two loader scenarios/nine observations remain separate paired
evidence. There is no new paired TypeScript include trace.

## Review and verification

The completed [broad review](include-review.md) identified IR1–IR5.
[Correction evidence](include-corrections.md) records their fixes and public
regressions. The [scoped re-review](include-rereview.md) confirmed the original
six probes and 15 regressions while identifying IR2-R1 and IR4-R1. Acceptance
is complete after the [residual corrections](include-residual-corrections.md)
and [final independent re-check](include-final-rereview.md). Both residuals
are addressed; no Critical/Important regression was found in the correction
diff. Exact public reproductions and 21 correction regressions pass on both
toolchains.

The parent checkpoint at `e46e4c8` passed all 38 command stages of the
[shared matrix](../acceptance.md): 406 workspace checks per explicit stable
1.97.1/Rust 1.85.0, M0, strict Clippy/Rustdoc, formatting, five examples,
29 paired pages, 21 include exports, 12 unique executable snippets and external
quick start, seven checker regressions and both existing paired-trace families.
Those pre-residual logs are preserved under
`target/include-resume-20261103/before-residual-fixes/`; they are historical
checkpoint evidence, not verification of the later correction delta.

## Final verification of 0a2df1e

All **38 command stages passed** on the final code. Tools were resolved by
`rustup which --toolchain stable` and `rustup which --toolchain 1.85.0` for
Cargo, rustc, rustdoc, cargo-clippy and clippy-driver. The selected toolchain's
bin leads PATH; RUSTUP_TOOLCHAIN, RUSTC and RUSTDOC are explicit. Rustdoc and
Clippy deny warnings. No Homebrew compiler is used for this evidence.

| Check | stable 1.97.1 | Rust 1.85.0 |
|---|---|---|
| Workspace check, all targets, locked | pass | pass |
| Workspace tests and Rustdoc | 412 passed | 412 passed |
| Include subset | 75 Rust tests + 2 Rustdoc | 75 Rust tests + 2 Rustdoc |
| Workspace strict all-target Clippy and no-deps Rustdoc | pass | pass |
| M0 tests/Rustdoc and strict Clippy | 21 checks pass | 21 checks pass |
| Five offline examples | pass | pass |
| Documentation | 29 paired pages, 21 include exports, 12 unique snippets, external quick start pass | same |
| Loader paired traces | 2 scenarios / 9 observations match | same |

Stable workspace/M0 formatting checks, seven documentation-checker regressions
and six core paired scenarios/73 observations also pass. The five examples are
`lifecycle`, `timer_lifecycle`, `console`, `boot_reconfigure` and `config_file`.
Tool versions: rustc/rustdoc stable 1.97.1 (8bab26f4f, 2026-07-14), Cargo 1.97.1
(c980f4866, 2026-06-30); rustc/rustdoc 1.85.0 (4d91de4e4, 2025-02-17), Cargo
1.85.0 (d73d2caf9, 2024-12-31). Executed on aarch64 macOS.

Reproduction uses the [shared acceptance commands](../acceptance.md),
`cargo run --example NAME --locked --offline` in each explicit toolchain
environment, and `python3 -B tests/loader_parity/run.py --toolchain TOOLCHAIN`.
The resumed orchestrator script, exact logs, 38-stage `matrix.json` and
`final-inputs.json` are retained under ignored `target/include-resume-20261103/`.
All 139 source/manifest/test/fixture/executable-snippet fingerprints remained
unchanged across final verification. Source-specific fingerprints are also
recorded in the committed correction and independent review reports.

Closing documentation/status edits passed both toolchain documentation checks,
all seven checker regressions and `git diff --check`. Logs:
`closing-docs-{stable,1.85.0}.log` and `closing-docs-regression.log` in the same
resumed log directory. All 139 fingerprints still match and the pinned TypeScript
checkout is clean; no further production/test change is part of closure.

## Deliberate limits

YAML writes canonical pretty JSON values; comments, formatting and quoting style
are not preserved. Anchors, aliases and explicit tags are rejected. Filesystem I/O
is bounded and synchronous on the local runtime. Native paths are interpreted by the host OS;
only the executed macOS/Unix behavior is verified here.

Replacement is per file, without an atomic multi-file transaction, fsync crash
consistency or an arbitrary concurrent-writer guarantee. A final check-to-rename
race remains; ownership/ACL/xattr/hardlink metadata preservation is not promised.
Only admitted Include JSON mutations are journaled; arbitrary native config
updates are not implicitly serializable. Watchers, HMR, the create satellite,
JS evaluation and native module hot loading are outside this delivery.

Remaining unrelated B0 evidence gaps stay tracked. No push, publication or
other satellite is authorized by this acceptance record.
