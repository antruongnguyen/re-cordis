# Loader L0–L5 / structural groups G0–G1 acceptance

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Status: **accepted within the declared Rust scope**.

## Scope and delivered revision

- Integration branch: `feat/cordis-completion`.
- Slice baseline: `cefb4c2eb6e78e121cc3a6ca4a594e8202b8a5c5`.
- Implementation checkpoints: `b98eae9`, `f70a695`, `81289ae`, and
  final corrected code `101f37b`. The commit containing this completed report
  closes documentation/status only; executable inputs remain identical.
- TypeScript reference: `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
- Delivered: typed factory catalog, owned entries, updates/rebuilds, structural
  groups and moves, typed scopes/evaluation, bounded JSON, source reconciliation,
  snapshots/commits, factory replacement seam, example and bilingual APIs.
- Reused accepted M0–M6, timer, console and B0 contracts/evidence. No new core
  API, include persistence, filesystem watching, HMR engine, JS evaluator or
  native binary loading is selected.

The interrupted sessions' unfinished work was preserved and integrated.
[Implementation history](loader-implementation.md) retains the recovery,
failing regressions, correction checkpoints and preliminary toolchain caveat.

## Independent review

The [broad review](loader-review.md) identified F1–F8 after the first full
matrix. The [final scoped re-review](loader-rereview.md) approves all eight
findings on `101f37b`, with no new Critical/Important defect or out-of-scope
finding. The additional F5 callback case was reproduced and corrected before
approval: old children disappear from the logical tree, the original update
waits for cleanup, and only the latest replacement applies afterward.

F4 keeps each admitted direct-update barrier even when newer updates finish
first. A global limit of 1024 includes reentrant admission reservations.
Dropped waiters do not acknowledge errors; a completed original/removal waiter
or successful settlement delivery does. One globally retained unobserved error
bounds history; disposal may report that same retained error again. Subtree
removal joins its updates, while settlement and owner disposal join all.

Runtime whole-group preflight checks structural entry options; it is not a
transaction over arbitrary plugin validation. Valid siblings continue after
another child fails. Loaded validation rejection retains accepted source;
post-acceptance apply failure retains new source and Failed state.

Core cleanup callback failures remain visible through `Runtime::take_failures`.
Loader joins retired barriers and retains the first actual retirement-task
error without changing core cleanup reports.

## Final executed verification

The same 119 executable/fixture inputs remained byte-identical throughout
final verification and after integration.

| Check | Explicit stable 1.97.1 | Explicit MSRV 1.85.0 |
|---|---|---|
| Workspace all-target check, locked | Pass | Pass |
| Workspace tests and Rustdoc checks | 335 passed | 335 passed |
| Loader subset of that total | 71 integration + 3 unit + 2 Rustdoc | 71 integration + 3 unit + 2 Rustdoc |
| Workspace strict all-target Clippy | Pass | Pass |
| Workspace Rustdoc, warnings denied | Pass | Pass |
| Isolated M0 tests/Rustdoc and strict Clippy | 21 checks passed | 21 checks passed |
| Core/timer/logger/loader runnable applications | All four passed | All four passed |
| Documentation checker | 27 EN/VI page pairs; 28 loader exports | Same |
| Executable documentation | 11 unique snippets + external quick start | Same |
| Paired loader scenarios | 2 scenarios / 9 matching observations | Same |

Stable workspace/M0 formatting checks pass. The Python documentation checker's
seven regression tests pass. The unchanged six core paired scenarios match
TypeScript in all 73 observations using the runner's explicit stable toolchain.
The pinned TypeScript checkout remains clean and both temporary fixtures are
removed. No live GitHub/network application request was made.

The preliminary implementer checks used PATH Homebrew Rust 1.99.1. They are
separate from this final matrix, which explicitly selected Cargo, rustc,
rustdoc, cargo-clippy and clippy-driver via `rustup which --toolchain`.

Reproduce the [shared toolchain matrix](../acceptance.md#toolchain-matrix) and
[documentation checks](../../../README.md#maintaining-these-documents), then run
the four examples (`lifecycle`, `timer_lifecycle`, `console`,
`boot_reconfigure`) with the same selected toolchain environment.
Paired checks:

```sh
python3 -B tests/parity/run.py
python3 -B tests/loader_parity/run.py --toolchain stable
python3 -B tests/loader_parity/run.py --toolchain 1.85.0
```

Local command logs and the 119-input hash manifest are retained under
`target/loader-final-20261103/`. They are local artifacts; the commands above
and tracked tests reproduce the evidence.

Frozen source fingerprints (SHA-256):

- `crates/re-cordis-loader/src/lib.rs`:
  `5a68375b64723f35dcb61eda49ed01458fd653687ec150e7c8d4e5450a91beb2`
- `crates/re-cordis-loader/tests/final_wave.rs`:
  `851ef8c009cd47072e4e19fd5bb74584922e46744a154c8abd6b72894fcb7d62`

## Compatibility and remaining scope

The [declaration map](loader-source-map.md) covers 140 declarations in eleven
pinned suites and preserves explicit Rust adaptations, unsupported mechanisms,
deferred integrations and narrower missing evidence. It is not proof of 140
executed equivalents; the two paired loader scenarios have the limits stated in
[their runner documentation](../../../../tests/loader_parity/README.md).

Global EntryIds, rebuild-based moves, typed callbacks/injection, extra outer
owners, decoder-owned data, explicit settlement and bounded error history are
documented in [decisions](../../../decision.md#loader-and-structural-groups) and
[limitations](../../../limitation.md#loader-and-structural-groups).
Named realm sharing follows live Scope definitions; anonymous positional source
matching requires current source ownership.

L0–L5 and G0–G1 are complete. Include, HMR, create and whole-program
release/distribution packages remain planned. All crates remain unpublished
with `publish = false`; no push or publication was performed.
