# Loader L0–L5 / structural groups G0–G1 acceptance

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Status: **implementation corrections and final acceptance in progress**.

## Scope and baseline

- Integration: `feat/cordis-completion`, baseline
  `cefb4c2eb6e78e121cc3a6ca4a594e8202b8a5c5`.
- Reference: TypeScript `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
- Continue the user's approved [loader](../loader.md) and [group](../group.md)
  scope after the prior root chat failed with HTTP 413. Its unfinished code
  and final worker additions are preserved, then corrected in an isolated
  worktree before integration.
- Reuse core M0–M6, timer and console acceptance and accepted B0 evidence.
  No new core API, filesystem/include implementation, HMR engine, JS evaluator
  or binary hot loading is selected.

## Review corrections

Independent review identified dependency-loss/self-disposal attribution,
independent group failure handling, source adoption and owner provenance,
Pending/Failed group evaluation, detached cleanup errors, callback/destructor
reentry, and source error locations. Focused failing regressions precede fixes.
The final review result and executed counts will be recorded below.

Runtime whole-group preflight checks entry-option structure. It is not a
transaction over arbitrary child plugin validation; valid siblings continue
when one child fails. A loaded validation rejection retains that child's
accepted source; apply failure after acceptance retains new source + Failed.
This distinction follows pinned group/validate behavior without extending core.

Cleanup callback failures preserve core diagnostic reporting through
`Runtime::take_failures`. Loader joins all retired barriers and retains the
first actual retirement-task error, rather than storing unlimited failures or
changing core cleanup reports.

## Compatibility and limits

See [declaration map](loader-source-map.md) for all 140 declarations in eleven
pinned suites. It separates tested Rust representations, deliberate adaptations,
unsupported mechanisms, deferred integrations and missing focused evidence.
The map is not proof of 140 executed equivalents.

Two new paired scenarios exercise dependency failure/recovery and structural
group disabling; see [runner](../../../../tests/loader_parity/README.md).
They complement the unchanged six core scenarios / 73 observations.
Native callbacks/static typed injection, global loader EntryIds, rebuild-based
moves, extra outer owners, decoder-owned expression-shaped data and explicit
settlement are documented in [decisions](../../../decision.md#loader-and-structural-groups)
and [limitations](../../../limitation.md#loader-and-structural-groups).

## Final verification

The initial frozen implementation passed the full explicit stable/1.85.0 matrix,
including workspace/M0/Clippy/Rustdoc, four applications, 27 bilingual page pairs,
28 loader exports, seven checker regressions, eleven runnable snippets and the
external quick start. Six core traces/73 observations and two loader traces/nine
observations matched; 113 executable input hashes remained fixed.

Final independent review then reproduced eight untested defects; see
[the checkpoint review](loader-review.md). L0–L5/G0–G1 are not accepted yet.
The user requested a durable local checkpoint before the remaining fixes.
Fresh final checks will follow that correction/re-review pass.

## Correction checkpoint CP064

F1/F2/F3/F5/F6/F7/F8 are implemented and await scoped independent re-review.
The current loader suite passes 66 integration, three unit and two Rustdoc
checks, with strict Clippy and formatting. F4 overlapping direct-update work
remains open; its pending regressions are retained separately. This is a
green checkpoint, not final acceptance. See [implementation history](loader-implementation.md).
