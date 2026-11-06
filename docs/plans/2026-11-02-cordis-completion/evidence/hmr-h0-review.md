# HMR H0 independent design review

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Reviewer: independent gpt-6-astra/high.
Rust baseline: `47042d9e805fa558e69416c8ffb1030333b0c38b`.
TypeScript pin: `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.

**Verdict: the selected H0 design is approved with the concrete corrections
below. These corrections form part of the approved contract and must be
incorporated in the watch/replacement contracts before the H0 commit.**
This authorizes implementation within the reviewed Rust scope; it does not
accept H1/H2/H3 code, test counts or unsupported operating systems.

## Scope and evidence reused

Read AGENTS.md, H0–H3 in [hmr.md](../hmr.md), shared
[acceptance](../acceptance.md), the [combined contract](hmr-contract.md),
[watch contract](hmr-watch-contract.md) and
[replacement contract](hmr-reload-contract.md).
Reused accepted Include `0a2df1e` and loader `101f37b` evidence, especially
[Include final re-review](include-final-rereview.md) and
[loader re-review](loader-rereview.md), plus accepted timer/core task contracts.
No broad re-audit or core change was selected.

CodeGraph supplied the relevant current loader factory/rebuild path, core
task/callback/lifecycle APIs and Include refresh/settlement path. Reviewed
pinned TS HMR implementation and the relevant dependency, nested-host,
resource-handoff, malformed-export, multiple-config, unload-race, callback,
path and no-internals test variants. IDE reads filled only omitted code/test
ranges. Inspected existing dependency-probe and loader reproducer logs.
The reviewer did not rerun Rust tests for this design-only gate; the worker's
two failing loader reproductions and notify MSRV checks remain carried evidence.

## Corrections and decisions

| ID | Finding | Approved correction |
|---|---|---|
| HR0-1 | User preflight may reenter loader and invalidate a batch before catalog publication. | Same-loader RAII admission guard covers optional preflight, including explicit guard in dispose. Preserve error/panic cleanup, release borrows before callbacks, restore the guard before dropping captures, and recheck private identity/revision/catalog state before publication. Purity excludes direct external/core side effects; no rollback of those is claimed. |
| HR0-2 | A silent first polling baseline can swallow a write immediately after watch returns. | Real registrations receive an initial Rescan; a newly added duplicate callback also receives its own initial Rescan. Membership is armed synchronously, sampling remains on the producer. No quiet-start guarantee; synthetic delivery remains explicitly driven. |
| HR0-3 | Calling Include::settle for the first time during watcher cleanup can fail admission after ancestor disposal starts; aborting a callback alone does not join already-admitted Include work. | Retain the IncludeTask obtained by hosted refresh as shared local completion outside the cancellable callback. Stop suppresses follow-up but retains this completion. Watcher/ancestor cleanup joins it without new Include admission and reports its original error or Report.failures through bounded watcher history. |
| HR0-4 | Stopping tokens could release admission quota while slow Include barriers remain, allowing unbounded retired work. | Count active and retired registrations with unfinished retained completion against max_watches; reclaim quota only after that completion is observed. No extra public API is needed. |

The reviewed optional adapter signature is:

```rust
#[cfg(feature = "include")]
pub fn watch_include(
    &self,
    path: impl AsRef<Path>,
    include: re_cordis_include::Include,
) -> Result<WatchToken, WatchError>;
```

Include remains independent of HMR. Registrations are explicit paths; no
automatic Include graph discovery is promised. The adapter starts at most one
refresh per registration and coalesces a single follow-up. It does not dispose
the supplied Include. WatchError must preserve LifecycleError from dedicated
Fiber construction/disposal in addition to the documented admission, timer and
I/O errors.

Retain explicit watch/observe and source counterparts: their ignore behavior is
clear and needs no glob parser. A streaming noncryptographic fingerprint is
acceptable for the documented ordinary-file scope and bounded memory; collision
and between-sample limitations remain visible. Std polling with a retained
JoinHandle avoids adding a watcher dependency whose Drop cannot establish joined
shutdown. A blocking OS read can delay join; no hard shutdown deadline is claimed.

A WatchToken stop/Drop is cancellation only. The watcher/ancestor lifecycle
barrier observes destruction and producer completion. Self/ancestor lifecycle
waits initiated by a hosted watcher callback must be refused before watcher
state is stopped. All public APIs continue to follow the existing host rules.

The immutable ReloadPlan's reverse dependency closure, dependency-only nodes,
exact candidate coverage and bounded cycle handling are appropriate. Factory
batch validation is separate from dependency-ready Plugin validation.
Catalog-backed entries are the reload units; arbitrary nested core declarations
inside an unchanged host are outside scope. A changed compiled dependency
requires an explicitly supplied host candidate. Settlement supplies reload
completion and per-entry diagnostics; no source recompilation or transactional
post-unload rollback is claimed.

## Required implementation evidence

1. Batch rejection: invalid final candidate, unknown/duplicate name, wrong source
   type and optional preflight rejection/panic leave old instances and catalog
   unchanged. Preflight attempts create/update/remove/reconcile/replace/dispose
   on the same loader fail before mutation; guard resets after every exit.
2. Replacement reentry: publication/destructor/commit callbacks removing,
   recreating, disabling or reconfiguring later entries cannot replay stale
   specs. Test parent/child replacement, repeated re-enable and a newer batch
   while old cleanup is gated. Assert exact live identities and apply counts.
3. Causal handoff: old providers remain usable until dependent cleanup releases;
   the replacement starts afterward. An unrelated fast sibling starts before
   a held slow sibling releases. Keep both per-instance configs and failure
   recovery tests; repeat at least 100 cycles with resource baselines.
4. Watch admission and cancellation: missing/wrong host and inactive owner create
   no work. Verify duplicate token removal, queued generation invalidation,
   callback self-stop, refused self-disposal, callback panic/error recovery,
   overflow Rescan and one follow-up per running registration. Assert capture
   release and producer join rather than callback counts alone.
5. Real file smoke: immediate post-registration writes without a readiness sleep,
   initial Rescan for duplicate registration, equal-size rewrite, atomic rename,
   remove/recreate including parents, symlink refusal/recovery and size-bound
   failure/recovery. Bound test deadlines and always join shutdown.
6. Include integration: gate admitted loader cleanup, stop/drop its token and
   dispose the watcher/ancestor. Completion must wait for the gate, preserve
   original failure diagnostics and start no follow-up refresh. Repeated stopped
   registrations with held work must hit the retained-work bound and recover
   admission after release. Verify parse recovery and self-write convergence.
7. H3 final gate remains the explicit stable/1.85.0 workspace/M0/docs/examples/
   parity matrix, paired user/API docs and declaration dispositions. macOS smoke
   does not establish Linux or Windows runtime acceptance.

Only this report was written by the reviewer. No production/test file, Git
index, branch or commit was changed. Implementation acceptance requires a
separate frozen-diff review.
