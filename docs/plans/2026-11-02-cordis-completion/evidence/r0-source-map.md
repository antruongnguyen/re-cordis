# R0 composition source-evidence reconciliation

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Integration input: `e8b5eb52b4c810591b24a7a51061da053e84b473`.
TypeScript specification: `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
Scope: the offline whole-program application in [the R0 plan](r0-plan.md),
under [R0 acceptance](../acceptance.md#r0--whole-program-integration-acceptance).

This report carries accepted prerequisite dispositions into the composition.
It does not re-audit every upstream assertion, alter the B0 ledger, or establish
R0 acceptance. The shared implementation is at
`crates/re-cordis-hmr/examples/system_lifecycle/app.rs`, with the runnable entry
at `examples/system_lifecycle.rs` and integration oracles at
`tests/system_lifecycle.rs` in that crate. Final review, generated application
and shared-matrix results are recorded separately in the R0 acceptance report.

## Accepted inputs and accounting boundaries

The [idiomatic Rust decision](../../../decision.md#2026-11-03--deliver-cordis-features-through-idiomatic-rust-contracts),
[difference classification](../../../limitation.md#how-to-classify-rust-differences)
and [prior-evidence reuse rule](../README.md#prior-decisions-and-evidence-reuse)
apply. TypeScript supplies intended semantics; the prior Rust port is not the
specification. Counts below keep their original units and cannot be summed into
a completion percentage or number of equivalent upstream tests.

| Prerequisite | Accepted disposition/evidence carried into R0 | Boundary |
|---|---|---|
| Core B0 | [Final independent acceptance](B0/review.md), [runtime/type ledger](B0/ledger/cases.json), [report](B0/ledger/report.md) | Accepted with tracked gaps: 144 declarations, 156 variants, eight parameterized sites, 540 runtime obligations, six separate type obligations and seven separate inventory non-obligations. |
| Timer T0–T5 | [22-declaration map](../timer.md#pinned-22-declaration-disposition), [acceptance](timer-acceptance.md) | 48 integration tests, three Rustdoc checks and six focused core task-bridge tests were executed on stable/1.85.0; these are Rust tests, not 22 paired declarations. |
| Console C0–C3 | [Five-declaration disposition](../logger.md#c3--application-and-documentation), [acceptance](logger-acceptance.md) | 21 native logger tests; core filtering/exporter/error contracts are carried. No console Rust/TS trace. |
| Loader L0–L5 / groups G0–G1 | [140-declaration map](loader-source-map.md), [acceptance](loader-acceptance.md), [scoped re-review](loader-rereview.md) | Final corrected loader `101f37b`: 71 integration, three unit and two Rustdoc checks. Two paired scenarios compare nine observations. |
| Include I0–I4 | [89-declaration, seven-suite disposition](include-source-map.md), [acceptance](include-acceptance.md), [final re-check](include-final-rereview.md) | Accepted code `0a2df1e`: 75 Rust tests plus two Rustdoc checks. Suite dispositions describe selected Rust behavior; no paired Include trace. |
| HMR H0–H3 | [53-declaration map](hmr-source-map.md), [acceptance](hmr-acceptance.md), [final watcher/adapter review](hmr-watch-adapter-review.md) | Executable `55a55d1`, acceptance `4c8f423`: 55 Rust tests plus one Rustdoc with all features; default scope has 44 plus one. No paired HMR trace. |
| Create S0–S2 | [14-declaration map](create-source-map.md), [acceptance](create-acceptance.md), [scoped re-review](create-rereview.md) | Executable/checker `8373fa3`: 22 Rust tests; the actual generated basic app passes five tests including 100 lifecycle cycles. No paired Create trace. |

Create's accepted final matrix contains 508 workspace checks per toolchain.
Generated-app tests and separate default-feature checks are outside that total;
prerequisite subset counts and repeated runs must not be added to it. This is
the R0 entry baseline, not a result for the new composition.

### Separate source dispositions

| Accepted accounting dimension | Executed comparisons | Rust representations / source-mapped witnesses | Exclusions / deferrals | Missing evidence |
|---|---|---|---|---|
| B0 runtime obligations (540) | 24 obligations; 32 paired references across four reused scenarios | 186 Rust representations; 180 source-mapped tests | Three unsupported obligations | 147 obligations remain not-executed, including gaps with passing partial witnesses |
| B0 types and inventory metadata | No TS type execution | Six passed Rust type representations | Seven exact non-obligations contribute to no behavioral/class/gap total | Separate from runtime accounting |
| Timer declarations (22) | Four bounded paired cases / 16 observations; no one-to-one declaration total inferred | 13 declarations have timeout/sleep/interval/throttle/debounce equivalent-purpose dispositions; five have single-consumer Stream representations | Four JS concurrent-reader declarations excluded | Cross-scheduled disposal has no separate targeted fixture; accepted self-dispose/nested/recursive coverage is narrower |
| Console declarations (5) | None | Aggregate/source errors, object Display/Debug, explicit formatter Display, exporter levels and exact label style have native dispositions | Browser dispatch, original JS arguments/inspection/placeholders and unsupported terminal/calendar policies are outside native scope | No actual terminal-emulator or Windows execution evidence |
| Loader declarations (140), primary map marks | Two scenarios / nine observations, separate from row marks | R: 29 relevant Rust representations; A: 25 explicit adaptations | U: 19 JS/Node exclusions; D: two deferred source-owner integrations | G: 65 declarations retain narrower missing assertions; these are not 65 proven defects |
| Include declarations (89) | None | Seven suite-level dispositions are closed within selected Rust scope | Named format/expression/module exclusions remain; watching was deferred from Include to HMR | No declaration-by-declaration equivalent count or new paired trace |
| HMR declarations (53) | None | Mixed carried, watch, Include-adapter and replacement oracles | Node mechanisms excluded; native path adaptation retains platform limits | Mixed keys overlap and are not exclusive pass counts; Windows/Linux runtime evidence remains missing |
| Create declarations (14) | None | One Cargo-equivalent purpose; one portable-failure/Cargo representation | Ten unselected remote-transport declarations and two npm/Yarn exclusions | No downloader, archive or malformed remote-template equivalent execution |

B0 runtime statuses remain **390 passed, 147 not-executed and three
unsupported/not-applicable**. The 390 combine different evidence classes and
do not mean 390 executed TS equivalents. The six type representations remain
separate. No B0 gap or loader G/D row is reclassified by this report.
The source reconciliation itself establishes no production defect; the separately
reproduced R0 snapshot correction is recorded below. Missing evidence and excluded
mechanisms are neither passing comparisons nor confirmed bugs.

## Composition-relevant behavior and the uncovered R0 delta

These rows connect the application contract to accepted evidence. They bound
what can be reused and identify the additional whole-program oracle.
Exact prerequisite test names come from their accepted reports; the new R0
witnesses are named separately below.

| Composition concern / pinned source purpose | Accepted Rust evidence and adaptation | R0 composition delta (witnesses below) |
|---|---|---|
| Boot typed providers/consumers under the local host; own console output and timer work | Core M0–M6/B0 hosting and committed injection; console owner/capture release; timer hosted task bridge and eager checked deadlines. Create's basic app already boots and updates a typed service under local Runtime/current-thread Tokio. | Boot the full Include/loader/group/timer/console application and the actual Create-generated external fixture using the shared application implementation. |
| Update config while dependencies change (`core/fiber`, `update`, `service-dispose`) | Six core paired scenarios preserve latest config, exact old publication access, Pending/readiness distinctions and original failure identity. Loader workers preserve accepted source and latest intent. | Supported file-backed config edits with actual application values, causal gates and readiness checkpoints. A successful update waiter alone is not whole-program convergence. |
| Move a provider between isolation groups (`loader/isolate-provide.spec.ts:143`; `isolate.spec.ts` transfer cases) | Loader logical EntryId survives scope/group movement while Fiber allocations rebuild. Accepted `consumer_retains_old_provider_until_cleanup_finishes_across_scope_move` moves the **consumer**, not the provider. HMR `exact_provider_survives_gated_consumer_cleanup_while_unrelated_reload_progresses` tests factory replacement. | Move the **provider** through Include while its dependent cleanup is gated; assert exact old provider remains callable until release, replacement readiness afterward, and unrelated sibling progress before release. Existing consumer-move/replacement witnesses do not close this loader G row. |
| Keep groups structural while disabling/rebuilding children (`loader/group`, `evaluate`, `rebuild`) | Loader paired structural-group scenario and accepted native group fixtures preserve group membership/EntryId, suppress disabled children and join cleanup. Extra outer owners and typed conditions are Rust adaptations. | Observe these states and resource ownership in the composed application; no new proof of every nested flag, condition, inject or intercept variant. |
| Replace linked behavior (`hmr/index.spec.ts` dependency closure, retained effects, slow sibling, nested/disabled guards) | Explicit ReloadPlan graph and already-compiled candidates; whole-batch admission before teardown; R-03/R-04/R-07–R-10 in the HMR map. `include_graph_replaces_the_same_file_managed_instances_without_writing` and `replacement_preserves_source_bytes_data_provenance_and_pending_journal` preserve the same Include-managed identities without flushing. | Replace the application's linked factory and retain config/provenance/journal outcomes across held dependent cleanup. This is executable behavior replacement, not recompilation of Rust source or Node cache rollback. |
| Controlled load/reconfiguration failure and explicit recovery (`core/update`, `fiber`; `loader/evaluate`, `validate`; `include/reload`; `hmr` post-unload failure) | Core exact source-allocation checks; loader Pending/Failed and explicit-update recovery; last-valid Include parse recovery; HMR post-unload failure/recovery. Rejected loaded validation retains accepted source; accepted apply failure can retain new source with Failed state. | Inject a controlled application failure, prove its original cause in the appropriate operation/report/history/Runtime channels, and recover explicitly. Observing one failure in several channels must not inflate distinct-failure counts. |
| Flush file-owned config/journal (`include/patch`, `sync`, `write`) | Include ordered patches, effective per-key ownership, bounded coalescing and three-way merge, visible conflicts/retries, final-flush-before-child-cleanup. Native admitted JSON operations replace implicit arbitrary config serialization. | Assert actual persisted application file content after pending work and final flush, and inspect Include final_error rather than trusting a successful cleanup waiter alone. |
| Stop/join pending watch and Include work (`hmr` config/self-write cases; core/timer ownership) | HMR I-03 retains the exact admitted IncludeTask after token stop/Drop/callback abort; watcher/ancestor disposal joins it and suppresses follow-up. WatchEvent retains returned Include/Report failures; ordinary core cleanup causes remain in Runtime diagnostics. Timer control/Drop requests differ from joined owner release. | Shut down with admitted timer/watch/Include work and a pending journal; prove request versus join and inspect all promised error/release channels. |
| Long-lived application use and release | Accepted core, timer, console, loader, Include, HMR and Create fixtures independently restore resource/capture baselines across 100 cycles. Distinct suites measure their own owned resources. | At least 100 deterministic **composed** lifecycles, actual effect/listener/Registry/publication baselines, no remaining owned tasks/watches/writers/children/services, and weak/capture release at documented barriers. Prerequisite cycles do not count as application cycles. |

## Concrete native R0 witnesses

The implemented integration suite names the following distinct oracles. Their
executed outcomes, toolchains and precise revision belong to final acceptance,
not to inferred source-equivalence counts.

| Test | Native composition witness |
|---|---|
| provider_move_keeps_exact_old_binding_and_sibling_progress | Actual provider move, exact old service before/after cleanup gate, independent sibling/timer progress, new binding and persisted move. |
| linked_replacement_and_distinct_failure_channels_recover | Linked candidates, failed load, direct update error, parser watcher history and explicit recovery. |
| application_shutdown_joins_admitted_refresh_before_disposing_include | Orderly application shutdown joins held valid refresh before closing Include; new binding completes, original provider is retained through the gate, and every resource is released. |
| shutdown_joins_pending_work_and_persists_journal | Held refresh, pending journal, cancellation request versus join, successful final persistence and resource barriers. |
| shutdown_preserves_distinct_io_and_cleanup_failures | Same pending composition with retained original parser and cleanup causes, no invalid-file overwrite. |
| hundred_composed_lifecycles_restore_baselines_and_captures | 100 actual full application cycles and effect/listener/Registry/publication/weak-capture release checks. |
| bounded_real_watcher_updates_the_shared_application | Bounded real file replacement/refresh using the same Application. |
| runnable_example_uses_the_same_application | The runnable demonstration under paused current-thread Tokio. |

R0 also produced a concrete loader defect: cancellation skipped the convergence
worker's running-flag reset, leaving a group snapshot Unloading after actual
resource release. [Correction f7a8e58](r0-loader-correction.md) and the native
shutdown_snapshot regression address that bookkeeping; no core/B0 reopening
or new public API is implied. The provider-move test is additional native
evidence for the loader G row discussed above, not a newly executed TS pair or
a blanket closure of its other assertions.

Independent finding R0-R1 is an application ordering defect, separately repaired
at `35556d5`: orderly shutdown joins watcher work before closing Include. The new
native oracle above complements the existing forced ancestor-cancellation tests;
[scoped re-review](r0-rereview.md) closes the finding on both toolchains.

## Preserved paired corpus

The [core lifecycle evidence](../../../lifecycle-parity.md) remains six executed
Rust/TypeScript scenarios with **73 observations**:

| Scenario | Observations | Relevant boundary |
|---|---:|---|
| combined | 11 | Provider/config combination follows committed lifecycle order. |
| coalesced | 14 | Exact old provider during held cleanup; latest intent skips intermediate applies; readiness can remain gated after update settlement. |
| removed | 12 | Registry removal cannot retire the old provider before dependent final use or revive queued config. |
| superseded-failure | 10 | Obsolete failure rollback completes once and does not poison newer intent. |
| latest-failure | 13 | Retained waiters see the original latest cause after rollback; abandoned waiter does not cancel cleanup. |
| explicit-recovery | 13 | Provider replacement alone does not retry Failed; explicit update recovers. |

Only four of those scenarios provide the 24 paired B0 obligation claims; the
other two remain preserved corpus evidence. These are different denominators.
The [loader paired evidence](../../../../tests/loader_parity/README.md) remains
two scenarios / **nine observations**: recovery (four), structural group (five).
It explicitly excludes isolation/provider moves, retained providers, persistence
and automatic HMR. Timer's accepted four cases / 16 observations remain a
separate bounded comparison, not a new R0 trace. No Include, HMR, console,
Create or complete-application Rust/TypeScript trace is added here.

## Adaptations, exclusions and remaining evidence

- **Rust adaptations:** typed Plugin/ServiceKey/Config/Catalog and immutable
  Scope/snapshots replace dynamic property/import/prototype machinery. Stable
  logical EntryId is distinct from rebuilt Fiber identity. Native callbacks,
  a single-consumer Stream, synchronous Write, explicit source mutation and
  linked compiled factories provide the selected useful capabilities. Async
  teardown still requires explicit joined completion.
- **Runtime constraints or unsuitable emulation:** this application does not
  execute JS expressions, Node ModuleJob/cache resolution, browser console
  arguments or native ABI/binary swapping. Rust source watch emits
  RestartRequired; an application-owned build/restart supervisor is outside R0.
  Upstream syntax/cache rollback is not implied by preflight admission.
- **Unselected/deferred work:** remote templates/registry/downloads, alternate
  Create templates, automatic Git/install/launch, live GitHub/service traffic,
  standalone B1, R1 distribution, CI/platform expansion and process supervision
  remain outside this offline composition. The basic Create production template
  stays unchanged; the R0 external fixture extends its test-owned output.
- **Missing evidence:** B0's 147 obligations and the loader map's 65 primary G
  rows retain their accepted owners/actions and narrower boundaries. Relevant
  R0 observations may become additional native evidence after actual tests,
  but cannot silently close those source IDs/variants. In particular, moving a
  provider differs from moving its consumer, and a composed successful shutdown
  does not establish every retained-realm notification or nested-child variant.
- **Filesystem and platform limits:** runtime file/watcher evidence is macOS
  only. Windows/Linux runtime acceptance, crash durability, atomic multi-file
  transactions, arbitrary concurrent writers and hostile-filesystem confinement
  are not established. Bounded synchronous OS I/O can delay joined shutdown;
  polling can miss between-sample changes, and bounded history can evict errors.
- **Evidence provenance:** historical reports' “later packages unstarted” or
  “final review required” wording describes their original checkpoints. Later
  independent acceptance records above supply the current prerequisite status;
  earlier results are neither erased nor substituted for R0's pending matrix.
  The source pin does not freeze the entire transitive JavaScript tool graph.

This reconciliation used accepted maps/reviews and CodeGraph for indexed
composition source, including the consumer-move fixture and underlying
publication/loader/journal seams. The initial source reconciliation performed no Cargo or paired-trace run.
The [implementation report](r0-implementation.md) records the executed native
oracles; current external-fixture, review and complete matrix results remain
separate acceptance evidence. No unexecuted source variant is counted as passing.
