# HMR H2 and narrow Include forwarding independent review

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Status: **paused at the user's request; H2-R1 is confirmed and blocks acceptance.**
This is an interim durable review record, not a final H0–H3 acceptance verdict.
No production/test source, Git index, branch or commit was changed by this reviewer.
Only this report and ignored scratch artifacts were written.

## Scope and reused evidence

Integration branch: `feat/cordis-completion`.
Initial baseline: `47042d9e805fa558e69416c8ffb1030333b0c38b`.
Approved H0 contract checkpoint: `5065adc`.
First loader batch checkpoint: `bbef48b`.

Reviewed the new loader batch/preflight delta versus the baseline, the immutable
HMR ReloadPlan graph and replacement forwarding, HMR reload/nested tests, and
the narrow Include::replace_factories forwarding delta plus its two tests.
Read AGENTS.md, [HMR plan](../hmr.md), [shared acceptance](../acceptance.md),
[H0 review](hmr-h0-review.md), [reload contract](hmr-reload-contract.md), and
[implementation evidence](hmr-reload-implementation.md).
Reused accepted loader `101f37b` and Include `0a2df1e`, especially
[loader re-review](loader-rereview.md) and
[Include final re-review](include-final-rereview.md). These are not broad audit
targets. CodeGraph supplied source/call paths; RustRover VFS supplied ranges
omitted by CodeGraph and wrote only the allowed report/scratch files.

H1 watcher implementation, H3 watch_include adapter, shared Cargo wiring,
current user-guide/export edits and final whole-program acceptance belong to
the parent/other reviewers. They are excluded except as compilation dependencies.
No core change is proposed or justified by this finding.

## H2-R1 — Distinct errors for one entry are incorrectly deduplicated

**Severity: Important; request changes before accepting H2.**

Location: `crates/re-cordis-loader/src/lib.rs`,
`Loader::replace_factories`, final async aggregation after `settled.await?`.
The loop skips each captured scheduling failure whenever the report already
contains any failure with the same EntryId:

```rust
if !report.failures.iter().any(|current| current.id == failure.id) {
    report.failures.push(failure);
}
```

This removes different original causes. The single first-unobserved global
slot is intentionally bounded and cannot retain every scheduling failure;
the exact batch task is therefore required to retain its own captured errors.
ID equality does not prove that the settlement row is that same error.

### Actual public-API reproduction

Ignored standalone package:
`target/hmr-reload-review/probe/{Cargo.toml,Cargo.lock,src/lib.rs}`.
Test:
`every_original_batch_observer_error_survives_concurrent_apply_failure`.

1. Create two live entries `a` and `b` for factory `worker` with configs 1 and 2.
2. Register a synchronous loader commit observer returning distinct original
   I/O errors `observer-a` and `observer-b` for those entries.
3. Replace worker with a plugin whose apply returns `apply-1` and `apply-2`.
4. Await the exact returned batch task, then inspect later loader settlement
   and Runtime::take_failures.

Both explicit toolchains produce:

```text
exact batch failures: a -> observer-a, b -> apply-2
later settlement:     a -> apply-1,    b -> apply-2
runtime diagnostics:  apply-1, apply-2
observer-b absent from every channel
0 passed; 1 failed; exit 101
```

This waiter is awaited; the behavior cannot be explained by the documented
first-unobserved policy for abandoned waiters. The captured `observer-b` is
available in the batch-local vector and is silently dropped by the final
ID-only check. Ordinary synchronous emit returns the observer error directly,
so the runtime does not independently preserve it.

Logs: `target/hmr-reload-review/probe-stable.log` and
`target/hmr-reload-review/probe-1.85.0.log`. They include the complete actual
Settlement and runtime diagnostic values. The assertion fails after successful
compilation on both toolchains; it is behavioral regression evidence.

The scratch source was subsequently extended with two passing probes and a
crate-level `cfg(test)` attribute. The failing test body remains intact.
Select it by name to rerun the exact behavior. Example after explicit toolchain
selection as below:

```sh
cargo test --manifest-path target/hmr-reload-review/probe/Cargo.toml \
  --locked --offline \
  every_original_batch_observer_error_survives_concurrent_apply_failure \
  -- --nocapture
```

Expected correction: preserve each distinct original batch scheduling cause
without losing an unrelated apply/validation cause for the same entry.
Deduplicate only the same retained error if necessary; do not add an unbounded
history or change core validation timing. Add a focused regression that checks
both observer causes and lifecycle causes. The parent will select and implement
the minimal bounded representation, then request a scoped frozen re-review.

## Evidence actually executed before pause

Every command resolved Cargo, rustc, rustdoc, cargo-clippy and clippy-driver
through `rustup which --toolchain stable` or `--toolchain 1.85.0` and checked
their common bin directory. RUSTUP_TOOLCHAIN, RUSTC, RUSTDOC and PATH select
that toolchain; PATH used its bin directory plus `/usr/bin:/bin:/usr/sbin:/sbin`.
Separate build directories are `target/hmr-reload-review/build-<toolchain>`.
Rustdoc uses `RUSTDOCFLAGS=-D warnings`.

| Independent command/check | stable | 1.85.0 |
|---|---|---|
| Loader `--test replacement_batch --locked` | 9 passed | 9 passed |
| HMR `--features include --test reload --test nested --locked` | 17 passed | 17 passed |
| Include `--test linked_replacement --locked` | 2 passed | 2 passed |
| H2-R1 standalone reproduction | 1 failed as described | 1 failed as described |
| Additional standalone probes, excluding H2-R1 | 2 passed | 2 passed |
| Loader + Include Clippy, all targets, warnings denied | exit 0 | exit 0 |
| HMR Clippy, Include feature, reload/nested targets, warnings denied | exit 0 | exit 0 |
| Loader + Include + HMR Rustdoc, Include feature, no deps, warnings denied | exit 0 | exit 0 |

The two additional probes independently confirm:

- A newer candidate admitted while old cleanup is gated supersedes the first
  candidate and uses the latest config exactly once; owner Registry clears.
- Include awaits its admitted batch's original apply failure, preserves the
  I/O error downcast, leaves data/origin/file bytes/pending journal unchanged,
  and returns `Report.changed == false`.

Logs: `target/hmr-reload-review/tests-{stable,1.85.0}.log` and
`checks-{stable,1.85.0}.log`. The checks process had already completed when the
pause arrived; no additional probe/review work was started after the pause.
The first scratch-creation tool call was refused because directory creation
was outside a VFS write action; the corrected call wrote the scratch package.
That tool setup failure is not counted as a product defect or test result.

## Positive observations within the reviewed delta

Whole candidate name/type/config checks precede catalog publication and
replacement-owned teardown. Optional raw-source preflight is hosted, same-loader
mutation/disposal is guarded, and the guard is released before rejected candidate
capture destruction. This does not expose or eagerly invoke core dependency-ready
validation; the executed Pending/dependency-ready test preserves that distinction.

All checked catalog records publish before displaced declarations drop. Current
entry identity/spec/factory checks, the reused loader workers and gated tests cover
later config, disable/remove, re-enable, parent replacement and old provider
retention while unrelated siblings progress. The immutable graph checks explicit
node/edge/factory bounds, reverse closure, cycles and exact candidate coverage.
Include adds a narrow exact-task forwarding method with no journal/file mutation.
These positive checks do not override the confirmed diagnostics defect.

## Frozen fingerprints checked at review entry

| File | SHA-256 |
|---|---|
| crates/re-cordis-loader/src/lib.rs | `89725d04b931fe7615bfc08da9a02d9dae500cb392058f0097d56ccb926d193c` |
| crates/re-cordis-loader/tests/replacement_batch.rs | `3844ef3bed91c485726ae663511f3e6522aa761fee53d02b3b05aa167a909d13` |
| crates/re-cordis-hmr/src/reload.rs | `64ed969a10b166a192feb1f34b63deafb4577abb974133651410016df8dc6b4e` |
| crates/re-cordis-hmr/tests/reload.rs | `8828143415c049913151ea644a310e33a033ceff11ee4ce16f2f089945f8a937` |
| crates/re-cordis-hmr/tests/nested.rs | `3eab229df85ab52a45f9deea17da79ed566a59dd3954e2f6e741e8ebfbcc7ebc` |
| crates/re-cordis-include/src/lib.rs | `7381718f2a5c05c7c30f36f726b75dedc58fef919bcd0629597e832fd104e345` |
| crates/re-cordis-include/tests/linked_replacement.rs | `c2cff761c13b66f4d46eccdf8ab0ed807c0efeb3eeed87386a3e107426c55973` |

No post-pause fingerprint pass was run. Resume from these frozen fingerprints or
record the parent correction checkpoint explicitly before reviewing its delta.

## Remaining work on resume

1. Correct H2-R1 and retain its original red logs; rerun the public reproduction
   and the affected targeted suites on both explicit toolchains.
2. Review only that correction and any directly affected error acknowledgement /
   bounded retention semantics, then record final specification/quality verdict.
3. Finish the parent-owned H1/H3 reviews and full workspace/M0/docs/examples/parity
   matrix. This review has not run those acceptance commands or established Linux
   or Windows watcher runtime support.

The review paused before issuing final specification and quality approval.
No additional confirmed defect was found in the inspected frozen delta.
