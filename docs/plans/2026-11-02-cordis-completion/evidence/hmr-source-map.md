# HMR H3 pinned source disposition map

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
TypeScript pin: `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
Rust design baseline: `47042d9`; approved H0 commit: `5065adc`.
H2 accepted code: `0e8d306`; H3 implementation checkpoint: `77b50b5`.
Status: declaration accounting and focused execution complete; H1/H3 independent
review and shared acceptance remain pending.

This map records every test declaration in pinned `packages/hmr/tests/index.spec.ts`
and `path.spec.ts`, plus the HMR implementation surfaces and relevant fixtures.
CodeGraph supplied source first; IDE VFS reads filled omitted bodies. The oracle
index cites executed Rust checkpoints; this documentation task did not rerun them
or execute the TS HMR suite. Line numbers below
are at the pin, relative to `local/cordis/packages/hmr/`.

The unchanged ownership, provider retention, hosting and entry lifecycle claims
reuse [loader acceptance](loader-acceptance.md), [loader scoped re-review](loader-rereview.md),
[Include acceptance](include-acceptance.md), [Include final re-review](include-final-rereview.md)
and accepted timer/core task contracts carried by [H0 review](hmr-h0-review.md).
No B0 re-audit is selected. The new delta is the owned watcher, retained Include
completion, explicit graph selection and whole-batch factory admission.
H3's optional `ReloadPlan::replace_include` uses the same selection checks and
forwards the exact `Include::replace_factories` loader batch completion for
entries managed by the same Include. R-10 and the offline example exercise this
path; it preserves journal, data, provenance and file bytes and performs no flush.

## Disposition keys

| Key | Rust representation and required oracle |
|---|---|
| A | Carried core/loader behavior, exercised through an HMR replacement in R. Initial registration/event/service behavior alone adds no new HMR equivalence claim. |
| W | [Watch contract](hmr-watch-contract.md): exact registrations; synthetic duplicate/burst/overflow tests, generation suppression, explicit ignore override, callback error/panic recovery, hosted cancellation and joined producer/capture release. Real bounded file tests supplement it. |
| I | Optional HMR-side Include adapter: an explicit trigger path refreshes the supplied Include's source graph, with last-valid parse recovery, rename-based self-write convergence, retained already-admitted IncludeTask completion, no follow-up after stop and resident-work quota. I-01–I-06 record focused execution; independent H3 acceptance remains pending. |
| R | [Replacement contract](hmr-reload-contract.md): explicit graph/candidates, admission before teardown, original per-entry configs, exact old resource/provider release, independent siblings, latest-intent/nested host guards, post-unload diagnostics/recovery and 100-cycle baselines. R-01–R-10 record concrete tests; H2 is independently accepted. |
| N | Node-specific mechanism unsupported: imports/exports, ModuleJob discovery, ESM/CJS caches, syntax compilation, JS filename resolution and process exit. W/I/R may represent the useful resulting behavior, without executing that mechanism. |
| P | Native path adaptation with missing Windows runtime evidence. macOS file smoke is insufficient; Linux runtime evidence is also not established by it. |

No key asserts one-to-one trace parity. In particular, supplying an already-linked
Rust factory replaces executable behavior; it does not recompile changed Rust
source. Source registration emits `RestartRequired`; the application owns any
build/restart supervisor. There is no native ABI swap or binary hot loading.

## Executed Rust oracle index

IDs below name concrete Rust tests or accepted carried evidence. They support
the stated Rust adaptation, not a replay of the TypeScript declaration. W/R
tests exercise separate watch and replacement inputs; there is no automatic
file-to-Rust-module replacement pipeline.

| Oracle ID | Exact test or carried evidence |
|---|---|
| A-01 | Accepted core hosting, event/effect ownership, service publication and loader entry behavior: [M6 evidence](../../../m6.md), [loader acceptance](loader-acceptance.md) and [H0 carried assessment](hmr-h0-review.md). Initial TS imports/service syntax remain N. |
| A-02 | Accepted Include file/source reconciliation, including disabled intent and last-valid state: [Include acceptance](include-acceptance.md) and [final re-review](include-final-rereview.md). |
| W-01 | [watch.rs](../../../../crates/re-cordis-hmr/tests/watch.rs): `synthetic_event_executes_admitted_local_callback_after_debounce`, `duplicate_burst_restarts_debounce_and_does_not_keep_history`. |
| W-02 | watch.rs: `exact_stop_keeps_sibling_and_suppresses_stale_generation`. |
| W-03 | watch.rs: `observed_ignore_can_be_overridden_by_explicit_registration`. |
| W-04 | watch.rs: `overflow_rescans_all_registered_paths_and_history_is_bounded`. |
| W-05 | watch.rs: `source_tokens_emit_restart_required_once_per_path_batch`; [offline example](../../../../crates/re-cordis-hmr/examples/reload.rs) prints capabilities once and continues config watching. |
| W-06 | watch.rs: `original_callback_error_and_panics_leave_future_refresh_usable`, `callback_and_error_destructors_can_reenter_watcher_without_borrows`. |
| W-07 | watch.rs: `running_callbacks_coalesce_followup_and_unrelated_callbacks_progress`; W-01/W-02 separately prove duplicate callback residency and exact cancellation. |
| W-08 | watch.rs: `one_hundred_watch_dispose_cycles_restore_parent_effect_baseline`, `stopped_running_registration_keeps_quota_until_capture_destruction`, `concurrent_input_cannot_repopulate_queue_after_joined_disposal`. |
| W-09 | [paths.rs](../../../../crates/re-cordis-hmr/tests/paths.rs): `normalized_missing_parent_paths_keep_one_sticky_native_identity`, `base_alias_is_fixed_once_but_relative_parent_and_file_symlinks_are_rejected`, `non_utf8_native_file_names_do_not_require_url_or_string_conversion`. These execute native macOS/Unix adaptations, not Windows drive/case behavior. |
| W-10 | [filesystem.rs](../../../../crates/re-cordis-hmr/tests/filesystem.rs): `immediate_write_and_duplicate_registration_receive_initial_rescan`, `fingerprint_detects_same_size_rewrite_with_restored_modified_time`, `atomic_rename_remove_and_parent_recreate_keep_logical_watch`, `oversized_file_failure_preserves_original_io_cause_and_recovers`, `symlink_replacement_reports_failure_then_regular_file_recovers`, `one_hundred_real_producer_cycles_join_and_restore_parent_baseline`. |
| R-01 | [reload.rs](../../../../crates/re-cordis-hmr/tests/reload.rs): `explicit_reverse_closure_preserves_unchanged_shared_dependencies_and_cycles`, `malformed_graphs_and_bounds_reject_explicitly`. |
| R-02 | reload.rs: `instances_keep_configs_and_candidate_coverage_is_exact`. |
| R-03 | reload.rs: `exact_provider_survives_gated_consumer_cleanup_while_unrelated_reload_progresses`. |
| R-04 | reload.rs: `post_unload_failure_keeps_source_and_later_candidate_recovers`, `candidate_preflight_is_separate_from_dependency_ready_core_validation`. |
| R-05 | reload.rs: `one_hundred_replacements_restore_resource_and_capture_baselines`. This acquires handlers/effects through 101 applications and checks final resource/Registry/capture release; initial event dispatch and ownership are carried by A-01. |
| R-06 | [loader replacement_batch.rs](../../../../crates/re-cordis-loader/tests/replacement_batch.rs): `every_candidate_is_checked_before_catalog_or_teardown`, `preflight_blocks_same_loader_mutation_and_dispose`, `preflight_panic_releases_guard_without_accepting_candidate`. Original-cause occurrence fixes are [independently accepted at 0e8d306](hmr-h2-accepted-review.md). |
| R-07 | [nested.rs](../../../../crates/re-cordis-hmr/tests/nested.rs): `latest_config_wins_during_unload`, `disabled_entry_is_not_resurrected`, `removed_entry_is_not_resurrected`, `reenable_uses_latest_config_once`, `replaced_parent_does_not_resurrect_old_child`, `owner_disposal_supersedes_started_replacement`, `recreated_identity_is_not_reloaded_a_second_time`. |
| R-08 | nested.rs: `explicit_host_candidate_rebuilds_nested_core_plugin_once`. This requires the supplied host candidate and does not establish unchanged-host nested discovery. |
| R-09 | nested.rs: `second_reenable_init_supersedes_the_first_without_hmr_duplicate`. |
| R-10 | reload.rs: `include_graph_replaces_the_same_file_managed_instances_without_writing`; [Include linked_replacement.rs](../../../../crates/re-cordis-include/tests/linked_replacement.rs): `replacement_preserves_source_bytes_data_provenance_and_pending_journal`, `preflight_rejection_keeps_old_instances_and_include_remains_usable`. |
| I-01 | [include_integration.rs](../../../../crates/re-cordis-hmr/tests/include_integration.rs): `watched_include_recovers_invalid_bytes_and_self_writes_converge`. |
| I-02 | include_integration.rs: `held_refresh_runs_one_coalesced_followup_after_resource_release`. |
| I-03 | include_integration.rs: `token_stop_keeps_admitted_include_barrier_until_join`, `token_drop_keeps_admitted_include_barrier_until_join`, `watcher_disposal_joins_include_after_callback_abort`, `ancestor_disposal_joins_include_without_new_admission`, `direct_include_refresh_routes_cleanup_failure_to_runtime`. Held cases retain two actual Report failures in WatchEvent once each, with source locations; ordinary cleanup OriginalFailure remains once in Runtime diagnostics. |
| I-04 | include_integration.rs: `retired_include_work_counts_against_quota_and_reclaims_after_observation`. |
| I-05 | include_integration.rs: `repeated_include_registration_releases_callback_and_work_effects` (100 cycles and weak capture release). |
| I-06 | include_integration.rs: `real_polling_refreshes_include_rename_and_invalid_file_recovery`. |

W-01–W-10 are H1 tests. R-01–R-10 use H2 tests and the accepted carried
loader/Include seams. I-01–I-06 name all ten H3 declarations. The focused
[H3 checkpoint](hmr-include-implementation.md) records 54 HMR integration tests
plus one Rustdoc check with all features, 43 plus one with default features,
and the offline example passing on explicit stable 1.97.1 and 1.85.0.
These are execution results; H1/H3 independent review and shared acceptance
remain pending.

## Implementation surfaces

| Pinned declaration or stage | Disposition |
|---|---|
| `src/index.ts:16` driveLetter; `:18–27` Context.hmr and Events.hmr/change/hmr/reload | P/W/R. Native Path/OsString and explicit FileWatcher replace URL/service augmentation. WatchEvent carries Change, RestartRequired or Failure; reload completion is loader Settlement, not a TS stale-plugin Map event. |
| `:29` WatchCallback; `:49` StalePlugin (filename/runtime); `:56` StaleFiber (path/replacement/fiber) | W/R. Owned WatchToken and local FnMut future; catalog names/compiled candidates and loader entry identities replace Node filename/runtime records. |
| `:35` loadDependencies including nested traverse; `:242` getLinked | N/R. ModuleJob.linked is unavailable. Application-declared ReloadNode edges provide a bounded reverse closure; unknown nodes are errors, not an empty discovery result. |
| `:75` hasInactiveAncestor | R/A. Loader rechecks logical entry/parent identity and latest intent; host candidates own nested core plugins. Arbitrary nested declarations in an unchanged host are not discovered/replaced. |
| `:83–114` Hmr/Inject and fields baseDir, internal, watcher, externals, accepted, declined, stashed, watchers | W/R/N. Explicit local coordinator, bounded registrations/queue/history and immutable ReloadPlan replace the service, Node caches and stashed module sets. No inferred externals or recursive root enumeration. |
| `:115` constructor; `:128` _resolve | W/P/N. Fixed native base/path normalization; Node 22–24 resolver version dispatch is unsupported. |
| `:136` Service.init and cleanup; `:163–174` Chokidar/picomatch/debounce setup | W. Dedicated owner Fiber, accepted timer debounce, owned standard-library polling JoinHandle, joined disposal. Ignore policy is exact native component names, not glob patterns. |
| `:174–204` change handler: externals exit, awaited callbacks, cached-source stash, hmr/change | W/I/N. Local hosted callback futures and bounded events; source registration reports RestartRequired. There is no loader.exit/process ownership or callback-to-module-import ordering guarantee. |
| `:214` watch and returned effect disposer | W/P. Multiple exact tokens; last-token removal releases producer membership. Unlike upstream's ever-growing Chokidar watch set, Rust scans only resident registered unique paths. Explicit watch can lie outside base or override ignored components. |
| `:238` getOuterStack | N. JS stack hiding is not reproduced; Rust original error sources and lifecycle diagnostics are retained. |
| `:256` analyzeChanges, local isExcluded and accepted/declined traversal | R/N. Explicit bounded reverse dependency closure supports cycles and preserves input order. Unchanged shared dependencies do not expand selection sideways; no Node/node_modules module analysis. |
| `:311–357` partialReload, nameMap/candidates/stalePlugins selection | R/N. Explicit factory-bearing/dependency-only units and exact candidate coverage. No import-name resolution from Include URLs or Cargo/module metadata. |
| `:375–403` esmBackup/cjsBackup/createRequire/rollback | N/R. No Node caches exist. All compiled declaration/type/optional raw-config preflight checks must pass before catalog publication/teardown; this is admission, not module-cache rollback. |
| `:405–423` import all replacements and applicability validation | R/N. Plugin<C,R> guarantees callable shape; optional pure hosted preflight checks retained raw configs. Dependency-ready core validation keeps its existing timing. Syntax/import failure is an external-build concern. |
| `:425–459` unload, staleFibers snapshot and ancestor filter | R/A. Loader owns exact old cleanup; parent replacement cannot instantiate a stale child twice. Shared provider retention is carried evidence. |
| `:461–503` concurrent rebuild, inertia/_initTask waits, disabled/removed/live-root guards and failure | R/A. Independent entry workers read latest source after joined cleanup. Apply failure after unload remains Failed with original cause and requires explicit update/replacement recovery; external side effects are not rolled back. |
| `:505–506` hmr/reload and stash reset | R/W. Settlement observes entry outcomes; bounded dirty state/coalesced follow-up clears as work finishes. No identical stale-plugin payload or cache-stash protocol. |
| `:510–531` namespace Hmr, Config interface/schema/i18n; `:534` default export | W/N/P. Rust exported types/options, Duration and typed errors. No ChokidarOptions/root glob schema or locale schema export. Defaults and bounds are in the watch contract. |
| `src/error.ts:6` isBuildFailure; `:10` handleError and per-error Babel code-frame rendering | N/W/R. Esbuild/Babel classification and Node synchronous source-file code frames are unsupported. WatchFailure preserves I/O/callback sources; loader Settlement preserves apply/validation failures. Rendering remains application policy. |

## Complete test declaration ledger

There are **53 declarations: 49 in index.spec.ts and 4 in path.spec.ts**.
Each appears exactly once below. Two declarations have `it.runIf(win32)`;
there are no `it.each`, `test.each` or `describe.each` expansions in these files.
The count includes their declarations regardless of whether a runner skips them.
Each ledger row cites the exact Rust oracle index above or accepted carried
evidence. Node mechanisms remain unsupported even where useful Rust outcomes
are tested. P rows identify unavailable platform evidence explicitly.

| ID | index.spec.ts line | Exact declaration name | Disposition | Rust oracle / boundary |
|---|---:|---|---|---|
| I01 | 171 | should load plugin and respond to events | A/N | A-01 |
| I02 | 176 | should reload plugin when file changes | R/W/N | W-01, W-10; R-02 (separate compiled candidate) |
| I03 | 189 | should handle reverting file back to original | R/W/N | R-05 (repeat analogue); W-10 (file rewrite) |
| I04 | 202 | should emit hmr/reload event on reload | R; Settlement adaptation | R-02/R-04 (Settlement) |
| I05 | 216 | should properly dispose old plugin effects on reload | R/A | R-03, R-05 |
| I06 | 259 | should load multiple plugins | A/N | A-01; R-02 |
| I07 | 264 | should reload changed plugin without affecting others | R/N | R-01, R-02 |
| I08 | 276 | should handle simultaneous changes to both plugins | R/W/N | W-01, W-04; R-03 |
| I09 | 317 | should load plugin with dependency | A/N | A-01; R-03 |
| I10 | 321 | should reload plugin when its dependency changes | R/N | R-01, R-03 |
| I11 | 334 | should reload plugin when plugin file itself changes | R/N | R-01, R-02 |
| I12 | 349 | should not reload a sibling that only shares an unchanged module | R/N | R-01, R-02 |
| I13 | 397 | should load plugin normally | A/N | A-01; R-02 |
| I14 | 401 | should rollback on syntax error and keep old plugin working | N; R admission analogue only | R-06 (admission only); N syntax/cache rollback |
| I15 | 415 | should recover after fixing the error | N; R valid-candidate recovery | R-04, R-06 (valid-candidate recovery); N compilation |
| I16 | 458 | should batch rapid changes within debounce window | W/R/N | W-01, I-02; R-05 |
| I17 | 504 | should handle config file update to disable plugin | I/A | A-02 (disabled source intent); I-01/I-06 (watch-to-refresh) |
| I18 | 565 | keeps watching the file across its own rename-based writes | I/W | I-01, I-06, W-10 |
| I19 | 590 | logs instead of rejecting when the config file is unparsable | I; retained failure event adaptation | I-01, I-03, I-06 |
| I20 | 663 | should load service plugin | A/N | A-01 |
| I21 | 672 | should reload service plugin on change | R/A/N | R-03; A-01 |
| I22 | 705 | should maintain entry association after reload | R/A | R-02, R-10 |
| I23 | 750 | should handle many rapid reloads without crashing | W/R; 100-cycle resource/capture oracles | W-08, W-10, R-05, I-05 |
| I24 | 785 | should register new event handlers after reload | R/A/N | R-05; A-01 (event ownership/dispatch) |
| I25 | 811 | should remove old event handlers after reload | R/A/N | R-05; A-01 (event cleanup) |
| I26 | 856 | should return linked dependencies for a loaded file | N; explicit R graph replaces discovery | R-01 (declared graph); N discovery |
| I27 | 865 | should return empty array for unknown file | N; R rejects unknown unit | R-01 (unknown unit rejects); N empty module result |
| I28 | 894 | should handle apply error gracefully without crashing | R/A | R-04, R-06 |
| I29 | 960 | should clear stashed files after successful reload | W/R; bounded dirty state adaptation | W-01, W-04, I-02; N cache stash |
| I30 | 996 | should load the nested entry | A/N | A-01; R-08 |
| I31 | 1001 | should instantiate a nested entry exactly once when its host reloads too | R/A/N | R-08, R-07 |
| I32 | 1019 | should skip a nested entry whose host has not started unloading | R/A; latest host/entry oracle | R-07/R-09 (latest intent); unchanged-host boundary |
| I33 | 1069 | should load both plugins | A/N | A-01; R-02 |
| I34 | 1074 | should not start the new instance before the old one released | R/A | R-03; I-02/I-03 |
| I35 | 1085 | should not delay an unrelated plugin behind a slow one | R/A | R-03, W-07 |
| I36 | 1142 | should leave the running plugin untouched | R/N; malformed JS export has no Rust shape analogue | R-06 (typed admission); N malformed JS export |
| I37 | 1161 | should recover after fixing the export | R/N | R-04, R-06; N JS export repair |
| I38 | 1207 | should rebuild every fiber, each keeping its own config | R/A; per-plugin log dedup is not claimed | R-02, R-10 |
| I39 | 1272 | watches a path that `ignored` would skip | W; explicit component override | W-03 |
| I40 | 1281 | runs every callback registered for the same path | W | W-01, W-02, W-07 |
| I41 | 1292 | matches paths that differ in case on Windows | P/W; Windows-only, unexecuted | No Windows runtime oracle; P/N remains |
| I42 | 1347 | should start and warn once | W; pure capability snapshot replaces warning | W-05 (capability adaptation) |
| I43 | 1351 | should emit hmr/change for a source file instead of reloading it | W/N; RestartRequired adaptation | W-05 |
| I44 | 1361 | should still reload config files | I/W | I-01, I-06; offline example |
| I45 | 1427 | does not rebuild an entry disabled during unload | R/A | R-07 (disabled_entry) |
| I46 | 1448 | does not rebuild an entry removed during unload | R/A | R-07 (removed_entry) |
| I47 | 1469 | waits for a second re-enable init before deciding whether to rebuild | R/A | R-09 |
| I48 | 1532 | rebuilds a child of an active host that reused the plugin | N/R boundary: arbitrary core child of unchanged host is unsupported; supply declared host candidate | Unsupported unchanged-host discovery; R-08 requires explicit host candidate |
| I49 | 1563 | skips a child whose host is replaced during unload | R/A; stale host-child guard | R-07 (replaced_parent) |

| ID | path.spec.ts line | Exact declaration name | Disposition | Rust oracle / boundary |
|---|---:|---|---|---|
| P01 | 39 | accepts a drive-letter base | P/W; Windows-only, unexecuted | No Windows runtime oracle; P remains |
| P02 | 46 | resolves a relative base against the context | W/P; explicit native base replaces context file URL | W-09 (explicit native base) |
| P03 | 52 | defaults to the context directory | W/P; caller must supply WatchOptions::new(base) | W-09; explicit base required, no context default |
| P04 | 63 | does not watch a nested ignored directory | W/P; exact observed-path filtering, no recursive root walk | W-03 (component filtering); no recursive discovery |

I39/I40 deliberately use a file no module imports (`index.spec.ts:1267–1270`).
The pin leaves callback-plus-module-reload behavior unpinned. This map does not
infer a stronger ordering guarantee. I18/I19 use an inline atomic-config fixture;
I20/I21 generate a Service fixture. I48 proves a capability Rust deliberately
does not provide through an unchanged compiled host.

## Fixture and helper scope

The source-accounting review read the complete inline fixture variants in both spec files. The 19 source
fixtures read are: `plugin.ts`, `plugin-a.ts`, `plugin-b.ts`, `plugin-config.ts`,
`plugin-dep.ts`, `plugin-dep-sibling.ts`, `plugin-error.ts`, `plugin-event.ts`,
`plugin-fast.ts`, `plugin-host.ts`, `plugin-multi.ts`, `plugin-nested.ts`,
`plugin-slow.ts`, `plugin-tree.ts`, `plugin-disabled-hmr.ts`, `dep.ts`,
`dep-common.ts`, `tree-dep.ts`, and `drain-dep.ts` (including
the four dependency-only files). They establish event/effect ownership, one
unchanged shared module, two labelled configs, an exclusive slow resource,
a two-level tree host and gated unload/shared-child races.

The 10 YAML fixtures are `cordis.yml`, `cordis-dep.yml`, `cordis-disabled.yml`,
`cordis-drain.yml`, `cordis-error.yml`, `cordis-event.yml`, `cordis-multi-entry.yml`,
`cordis-multi.yml`, `cordis-nested.yml`, and `cordis-no-internal.yml`.
Their timer/HMR roots and plugin declarations support the ledger above.
Inline generated files are `cordis-atomic.yml`, `plugin-service.ts`,
`cordis-service.yml`, `.claimed.yml` and the `__ignored__` path tree;
these are fixture mechanisms, not extra test declarations.

Helpers `backupFile`, `waitFor`, `waitForEvent`, `createContext`, `Gate`,
`configTree`, `findEntry`, `installGate`, `arm`, `disarm`, `liveUids`,
`dropCached`, `SETTLE_MS`, inline `configWith`/`include`, and path-spec
`testDir`/`fixture`/`createContext` were read. Their polling sleeps, file
mutations, global counters and Node-cache eviction are TS harness details.
Rust must use bounded synthetic/gated oracles and joined resource assertions;
a callback count or cleanup waiter alone cannot prove release.

## Acceptance boundary

The exact W/I/R test references and focused explicit stable/1.85.0 results above
come from [H1 implementation](hmr-watch-implementation.md), the
[accepted H2 review](hmr-h2-accepted-review.md) and
[H3 implementation](hmr-include-implementation.md). H3's final gate still requires
independent frozen-diff watcher/integration review and the shared matrix.
Existing H0 dependency probes remain in the watch contract; they do not execute
this HMR crate. Historical resume/R1/R2/R3 reports are retained unchanged.
Keep missing Windows and Linux runtime evidence visible. Portable std polling
and native paths do not establish those platforms by compilation alone.

Polling samples only registered regular files, reads a bounded prefix and digest,
and may miss changes entirely between samples or digest collisions. Symlinks are
refused without claiming hostile-filesystem confinement. Missing files and
parent remove/recreate retain their logical registrations; parent renames do not
move them. Queue overflow rescans only the bounded registration set.
