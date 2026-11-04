# HMR H2 independent final review and H2-R1 re-review

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Frozen revision: `8b2252bafcb85b4ae6c38965ce89c26e7e03fbe7`.
Branch: `feat/cordis-completion`; clean at review entry.
Correction delta: `56de5f4..8b2252b`.

**Final H2 specification and quality verdict: request changes for H2-R2 below.**
The original H2-R1 fresh-slot reproduction is corrected. A directly adjacent
occupied-slot variant still loses a current lifecycle cause from the exact
returned batch, so H2 is not accepted at this revision. The scoped review is
complete; no other actionable finding was identified in the reviewed H2 delta.
This is not H0–H3 or whole-program acceptance.

## Scope and reused evidence

Read AGENTS.md, [HMR plan](../hmr.md), [shared acceptance](../acceptance.md),
[resume instructions](hmr-resume/README.md), [H0 review](hmr-h0-review.md),
[replacement contract](hmr-reload-contract.md), [paused H2 review](hmr-reload-review.md),
[implementation evidence](hmr-reload-implementation.md) and
[H2-R1 correction evidence](hmr-h2-correction.md).

Reuse the earlier review of baseline `47042d9`, H0 checkpoint `5065adc`,
initial batch checkpoint `bbef48b` and accepted loader/Include prerequisites.
This review completes the paused H2 verdict, checks the correction and its
error acknowledgement/retention effects, and confirms the unchanged ReloadPlan
and narrow Include forwarding source. No broad core/timer/logger/loader/Include
audit was reopened. The previous H2 graph, preflight, nested/race and Include
findings are carried evidence, not newly claimed test runs.

CodeGraph supplied current source and callers; RustRover VFS read the saved
public probe and wrote only assigned scratch/report files. The fresh probes use
public core/loader APIs. No production/test file, Git index, branch or commit
was changed by this reviewer. H1 watcher, H3 watch_include/adapter, source-map
completeness, examples and final shared matrix remain parent-owned gates.

## H2-R1 original reproduction — addressed

The old EntryId-only deduplication is gone. Each synchronous scheduling failure
now has a distinct private `Rc<Cell<bool>>` occurrence marker; the existing
first-unobserved slot shares that marker when it retains that occurrence.
`Rc::ptr_eq` recognizes the same retained occurrence, without treating equal
EntryIds, equal messages or equal original-source pointers as error identity.
Batch-local scheduling failures are appended after ordinary lifecycle collection.
No public API, dependency or global failure history was added.

With a fresh global slot, the independent public probe and the integrated test
both return exactly these four original I/O causes:

```text
a:apply-1, a:observer-2-a, b:apply-2, b:observer-2-b
```

The exact probe downcasts every cause to `io::Error`; each current observer
occurrence appears once. The integrated regression also checks later settlement
and runtime causes. Original H2-R1 is therefore addressed for its recorded
trigger. The occupied-slot extension below prevents accepting the complete
error-aggregation contract yet.

## H2-R2 — An older slot occurrence overwrites a current exact-batch apply failure

**Severity: Important (P2); request changes before H2 acceptance.**

Location: `crates/re-cordis-loader/src/lib.rs:2036–2044`, especially
`failures.insert(failure.id, failure.error)` at line 2043.
`Loader::replace_factories` reaches this collector at line 2867.

If the global slot holds an unobserved occurrence from an older batch, its marker
is absent from the current batch's `scheduled` vector. The collector therefore
inserts that older error into the EntryId-keyed lifecycle map. This replaces,
rather than accompanies, a distinct current apply failure for the same entry.
The subsequent append preserves the current scheduling errors but cannot restore
the overwritten lifecycle cause.

Public reproduction in `target/hmr-h2-rereview/probe/src/lib.rs`:

1. Create live worker entries `a`/`b` with configs 1/2 and settle them.
2. Install a synchronous commit observer which returns tagged original I/O errors.
3. Admit an inert replacement and drop its waiter, retaining `observer-1-a`
   under the accepted bounded first-unobserved policy.
4. Admit and await a new replacement whose applies return `apply-1`/`apply-2`.
5. Inspect the exact returned report, then later settlement and runtime diagnostics.

Both explicit stable and 1.85.0 reproduce these results:

| Current commit observers | Exact new batch | Later settlement / runtime |
|---|---|---|
| Return `observer-2-a`/`observer-2-b` | `a:observer-1-a`, `a:observer-2-a`, `b:apply-2`, `b:observer-2-b` | both apply causes remain visible |
| Succeed | `a:observer-1-a`, `b:apply-2` | both apply causes remain visible |

Both probes fail the assertion that the **exact awaited new batch** contains
`a:apply-1`. The first older waiter is intentionally abandoned; the new waiter
is awaited. This is incomplete direct result delivery, not disappearance from
all diagnostic channels. It conflicts with the exact-batch guarantee in
`docs/en/api/loader.md:94` and the requirement to preserve lifecycle causes
alongside distinct scheduling causes. Callers receiving the returned Settlement
cannot fully diagnose the replacement without another global observation.

Required correction: in exact replacement reports, preserve current lifecycle
rows while delivering any retained older slot occurrence without overwriting
those rows. Keep ordinary `Loader::settle()` and abandoned-waiter policies
unchanged. Exact-batch mode must remain distinguishable even when the current
batch has **zero scheduling failures**, as the second reproduction demonstrates.
This needs no error history or core API change. Retain both regressions and the
existing first-unobserved/acknowledgement tests.

## Completed H2 specification/quality assessment

| Requirement | Review disposition and evidence |
|---|---|
| Whole-batch name/type/config checks before publication/teardown | Satisfied in inspected source and carried review; `every_candidate_is_checked_before_catalog_or_teardown` rerun passes. |
| Hosted pure preflight, same-loader mutation refusal, guard release before capture drop | Satisfied in inspected ordering and carried review; the three preflight/guard regressions rerun pass. Core dependency-ready validation remains separate. |
| Explicit bounded graph, reverse closure, cycles, unchanged shared dependency, exact coverage | Unchanged `ReloadPlan` source checked; graph has 256-node/1024-edge bounds and stable declaration-order output. Carried HMR graph/config tests cover selected contract. |
| Current entry identity/intent, per-instance config, sibling progress, exact old provider release | Carried gated replacement and nested tests establish the selected behavior; unchanged scheduling guards were inspected. Previous newer-candidate-during-cleanup public probe remains carried evidence. |
| Failure/recovery and original causes | Single failure recovery is covered; original H2-R1 passes; H2-R2 remains the sole blocking defect. |
| Repeated cleanup and bounded ownership | Carried 100-replacement resource/capture baseline test; correction adds only per-batch occurrence handles and retains one global slot. No new worker or global history. |
| Narrow Include forwarding | Unchanged code awaits the exact admitted loader task and maps its failures; no source/journal/provenance/flush change. Carried two linked tests and independent original-error/data-preservation probe remain applicable. H2-R2 is in the underlying loader collector. |
| Ordinary settlement compatibility | Fresh `final_wave` and batch dropped-waiter regressions pass on both toolchains; their first-unobserved policy is intentionally preserved. |

The immutable graph and reuse of existing loader lifecycle workers are suitably
small for the selected Rust feature. The remaining quality issue is specifically
the mode-dependent failure aggregation; no speculative refactor is requested.
The linked-factory scope does not imply Node cache/import parity, source
recompilation, native code swapping or arbitrary nested declaration discovery.

## Fresh verification and retained artifacts

Every Rust command independently resolved Cargo, rustc, rustdoc, cargo-clippy
and clippy-driver through `rustup which --toolchain stable|1.85.0`; all five
paths share the expected bin directory. Matching `RUSTUP_TOOLCHAIN`, `RUSTC`,
`RUSTDOC` and leading PATH were set. `RUSTDOCFLAGS=-D warnings` was set;
these focused commands are tests, not a fresh Rustdoc or Clippy claim.
Separate build directories are `target/hmr-h2-rereview/build-<toolchain>`.

| Fresh command | stable | 1.85.0 |
|---|---:|---:|
| Standalone three-case public probe, `--locked --offline -- --nocapture` | 1 passed, 2 failed; exit 101 | 1 passed, 2 failed; exit 101 |
| Loader `--test replacement_batch --test final_wave --locked --offline` | 25 passed; exit 0 | 25 passed; exit 0 |

The 25 tests comprise 11 replacement-batch and 14 existing final-wave tests.
No full workspace/M0/docs/examples/parity matrix was rerun by this reviewer.
The correction worker's recorded full loader/HMR/Include and strict checks,
and the parent's paired documentation checks, remain attributed carried evidence.
They do not invalidate the new public regression.

Ignored artifacts under `target/hmr-h2-rereview/`:

- `probe/{Cargo.toml,Cargo.lock,src/lib.rs}`: independent public reproductions.
- `probe-{stable,1.85.0}.log`: full commands, output and exit 101.
- `regressions-{stable,1.85.0}.log`: focused existing test outputs and exit 0.
- `tools.json`: resolved paths and versions for both toolchains.
- `frozen-inputs.json`: reviewed HEAD/source hashes and focused results.

## Frozen fingerprints

These matched the correction and earlier H2 review fingerprints after the fresh
runs, while source still matched the frozen revision. No conclusion here
covers subsequent edits.

| File | SHA-256 |
|---|---|
| loader `src/lib.rs` | `e575b5256309539d0f95f9314def30c3cc5b371b8a7a8c784c230b25893264d6` |
| loader `tests/replacement_batch.rs` | `8e4d94b3f417dd0d342e242b2b70340f9bcb1fb1d8b93e5fb81f1430bd60000e` |
| HMR `src/reload.rs` | `64ed969a10b166a192feb1f34b63deafb4577abb974133651410016df8dc6b4e` |
| HMR `tests/reload.rs` | `8828143415c049913151ea644a310e33a033ceff11ee4ce16f2f089945f8a937` |
| HMR `tests/nested.rs` | `3eab229df85ab52a45f9deea17da79ed566a59dd3954e2f6e741e8ebfbcc7ebc` |
| Include `src/lib.rs` | `7381718f2a5c05c7c30f36f726b75dedc58fef919bcd0629597e832fd104e345` |
| Include `tests/linked_replacement.rs` | `c2cff761c13b66f4d46eccdf8ab0ed807c0efeb3eeed87386a3e107426c55973` |

Next gate: correct H2-R2, freeze its narrow delta, and re-review only that
aggregation/acknowledgement change before accepting H2. H1/H3 and the final
shared matrix remain separate gates.
