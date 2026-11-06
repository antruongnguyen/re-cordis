# Loader declaration disposition map

Pinned source: `local/cordis` at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`, confirmed with `git rev-parse HEAD`.
This maps **140 declaration lines in 11 loader spec files**. Parameterized
declarations remain one row; this is not a count of expanded assertions.

The inventory supplies declaration identity only. The actual eleven test files
and loader implementation were inspected through CodeGraph (`explore` and
whole-file `node` where callback-only suites were not surfaced by `explore`).
Initial Rust implementation and fixture bodies were inspected on 2026-11-03
while the new crate was under review. Those initial references are inspection
evidence unless the post-review supplement explicitly identifies an execution.
Acceptance runs and later review fixes belong in the loader acceptance record. No new core/B0 audit is
claimed, and this map is not a parity claim.

## Reading the dispositions

| Mark | Meaning |
|---|---|
| R | Rust representation has a relevant current loader fixture; the note bounds what it actually observes. |
| A | Explicit Rust adaptation: lifecycle, ownership or data representation differs. Any untested portion is stated. |
| U | Unsupported JS/Node mechanism; no behavior is claimed from similarly named Rust APIs. |
| D | Deferred integration behavior belonging to include/HMR or a separate source-tree owner. |
| G | Suitable Rust behavior exists or is intended, but this declaration lacks focused loader evidence. A related fixture is not a substitute. |

Rust uses checked heterogeneous `Config`, typed `Plugin<C, R>` Catalog adapters,
`EntrySpec`/`Group`, immutable typed `Scope`, logical loader-wide `EntryId`, and
snapshot/commit values. Structural addresses can change; cross-group moves
preserve EntryId and rebuild Fiber identity after joined cleanup. Every enabled
entry owns a loader outer Fiber. Missing dependencies settle as Pending; typed
condition failures surface as Failed and require explicit update. These are
Rust contracts, not transparent JS import/prototype/expression emulation.

`SourceDocument`/`reconcile` and `commits()` are in-memory seams: they do not
provide files, YAML, overlays, journal writeback, watchers or include ownership.
`replace_factory()` is an in-memory typed replacement seam: Node ModuleJob,
loadCache invalidation, dependency analysis, automatic HMR and rollback are
still deferred. None of the 140 loader declarations accepts those integrations.

## Exact current Rust fixture references

Paths below are relative to `crates/re-cordis-loader/tests/`. A fixture name alone
is not evidence for every word in its title; only assertions described in the
map count.

| Ref | File | Exact test name |
|---|---|---|
| E1 | entries.rs | `eager_admission_identity_checked_configs_and_joined_removal` |
| U1 | updates.rs | `pending_validation_failed_recovery_and_invalid_loaded_source` |
| U2 | updates.rs | `latest_rename_and_remove_while_cleanup_is_gated` |
| G1 | groups.rs | `disabled_groups_are_structural_moves_adopt_children_and_reorder_is_inert` |
| G2 | groups.rs | `entry_injection_and_plugin_mapping_use_their_own_checked_context` |
| G3 | groups.rs | `typed_isolation_moves_preserve_entry_identity_and_rebuild_fiber` |
| L1 | lifecycle.rs | `nested_child_disposal_is_not_entry_disposal_but_exact_inner_disposal_commits_disabled` |
| L2 | lifecycle.rs | `consumer_retains_old_provider_until_cleanup_finishes_across_scope_move` |
| L3 | lifecycle.rs | `removal_from_create_commit_callback_cannot_leave_resurrected_work` |
| S1 | source.rs | `json_preserves_absence_null_and_original_decode_errors_and_source_siblings` |
| S2 | source.rs | `runtime_json_group_preflights_all_children_and_replacement_catalog_keeps_identity` |
| S3 | source.rs | `json_document_limits_remain_global_and_entry_ids_are_deferred` |
| R1 | regressions.rs | `typed_serialized_inject_isolate_and_intercept_are_checked` |
| R2 | regressions.rs | `group_reconfigure_disable_enable_preserves_children_identity_for_100_cycles` |
| R3 | regressions.rs | `parent_disposal_joins_manually_detached_cleanup_after_loader_handles_drop` |
| R4 | regressions.rs | `source_nested_invalid_child_preserves_healthy_sibling_and_source_location` |
| A1 | admission.rs | `retiring_entry_counts_toward_capacity_and_parent_disposal_keeps_shared_barrier` |
| A2 | admission.rs | `runtime_group_capacity_preflight_preserves_all_existing_sources` |
| A3 | admission.rs | `failing_scope_callback_is_contained_and_does_not_admit_a_partial_entry` |
| A4 | admission.rs | `failed_partial_apply_releases_listener_and_healthy_sibling_continues` |
| A5 | admission.rs | `json_depth_nodes_string_and_unknown_field_bounds_are_explicit` |
| Q1 | reentrant.rs | `inner_config_hook_removal_is_refused_before_entry_unlink` |
| Q2 | reentrant.rs | `wrong_host_and_unhosted_mutations_are_typed_refusals` |
| Q3 | reentrant.rs | `disabled_entry_never_evaluates_and_failed_condition_recovers_on_explicit_update` |

S1 asserts absent/null config and an unknown-factory sibling, not original
error downcasting. R4 asserts healthy/invalid nested membership and failure
count, not source path/span. A5 asserts depth/node/string limits, not unknown
field rejection. V6/V7/V12 below add source provenance and original validation
cause assertions; JSON decoder cause downcasting and unknown-field rejection
remain separate missing assertions.

## Post-review supplement (2026-11-03)

The worktree review added focused fixtures below. Their bodies were inspected
through the live IDE; this worktree has no CodeGraph index. Final execution of
the changing review/acceptance suites belongs to the frozen loader acceptance
record; their names below do not imply a fresh final pass. Before the final
review edits, the paired runner executed two scenarios on stable and 1.85.0:
recovery (4 observations) and structural groups (5), with upstream ESLint,
strict type checking, Vitest and byte-for-byte trace comparison. The final
acceptance reruns those paired fixtures after the freeze.
See [the paired evidence limits](../../../../tests/loader_parity/README.md).

| Ref | File under loader tests | Exact test name | Bounded observation |
|---|---|---|---|
| N1 | nested.rs | `all_nested_parent_child_disabled_combinations_keep_groups_structural` | Eight initial parent/group/child flag combinations keep both groups active and suppress the child whenever any flag disables it. |
| N2 | nested.rs | `latest_entry_injection_wins_during_gated_old_cleanup` | Two missing-requirement updates followed by cleared requirements coalesce during gated disposal; applies are only 0 and 3. |
| N3 | nested.rs | `intercept_failure_waits_for_entry_injection_and_explicit_update_recovers` | Intercept waits for the entry provider, reads its value, fails once without plugin apply, then explicit update recovers. |
| V1 | review_repairs.rs | `entry_dependency_withdrawal_recovers_without_disabled_commit` | Withdrawal/rearrival preserves source disabled=false, emits no disabled writeback, and applies twice. |
| V2 | review_repairs.rs | `pending_group_update_defers_condition_until_entry_dependencies_commit` | A pending group update cannot evaluate its condition; provider arrival evaluates once and activates. |
| V3 | review_repairs.rs | `same_failed_group_update_explicitly_recovers_outer` | Explicit update of the same failed group spec recovers once its callback succeeds. |
| V4 | review_repairs.rs | `source_adoption_moves_nested_identity_to_root_and_preserves_runtime_edit_owner` | Cross-source nested-to-root adoption retains EntryId; old-source removal cannot delete it, and the adopting source still owns removal after a runtime edit. |
| V5 | review_repairs.rs | `runtime_group_options_preflight_is_atomic_but_loaded_validation_is_independent` | Unknown factory rejects a runtime group update without commit/mutation; a loaded child validation failure preserves its source while valid siblings update. |
| V6 | review_repairs.rs | `typed_source_group_keeps_valid_siblings_and_locates_contract_failure` | Wrong typed child contract is rejected independently with exact SourceId, structured path and caller-provided span. |
| V7 | review_repairs.rs | `source_initial_and_loaded_validation_failures_keep_original_cause_and_location` | Initial and loaded validation errors retain source/path/span and downcastable original validation cause; invalid loaded source retains the accepted config. |
| V12 | review_repairs.rs | `nested_json_async_failure_keeps_original_uncompacted_source_path_and_transport` | Async failure after an earlier decode failure retains the original JSON child index, source identity, original validation cause and JSON transport. |
| V13 | review_repairs.rs | `same_source_root_adoption_into_group_survives_previous_root_cleanup` | Same-source root-to-group adoption preserves EntryId through previous root cleanup. |
| V14 | review_repairs.rs | `same_source_root_adoption_joins_gated_cleanup_without_losing_child` | The same adoption joins gated cleanup, settles without failures and preserves child identity/address. |
| M1 | sources_moves.rs | `source_adoption_keeps_global_identity_and_old_group_cannot_remove_it` | Old-source group cleanup cannot remove an adopted root entry. |
| M2 | sources_moves.rs | `reconciliation_reorders_explicit_root_entries_without_changing_entry_ids` | Read-only explicit root ordering changes without changing EntryIds. |
| P1 | loader_parity.rs | `dependency_arrival_failure_and_explicit_recovery` | Executed Rust/TypeScript trace: Pending/no apply, arrival/failure, explicit recovery and removal cleanup. |
| P2 | loader_parity.rs | `structural_disabled_group_controls_child_cleanup` | Executed Rust/TypeScript trace: group/child states, membership, apply/dispose counts through disable/enable and shutdown. |
| V15 | review_repairs.rs | `invalid_old_source_cannot_remove_identity_adopted_by_another_source` | Invalid options in a former source cannot delete an entry already adopted by another source. |
| V16 | review_repairs.rs | `malformed_json_source_ids_fail_per_entry_while_valid_siblings_survive` | Reserved/overlong IDs fail independently with exact source/id paths, leaving a valid sibling; runtime create still rejects its invalid ID. |
| F1 | acceptance.rs | `direct_update_middleware_blocks_settlement_of_nested_grandchild` | Dropped direct-update waiter remains joined by settlement while nested grandchild middleware is gated; latest apply and Active state follow release. |
| F2 | acceptance.rs | `verdict_is_invalidated_by_dependency_republication_and_explicit_update` | False condition suppresses apply; provider withdrawal/republication with true re-evaluates; static disable skips condition and explicit enable re-evaluates. |
| F3 | acceptance.rs | `repeated_realm_moves_join_self_provider_cleanup_in_original_view` | Five moves create six distinct Fiber generations; each self-provider disposer reads its original realm/value, and final registry is empty. |
| F4 | acceptance.rs | `exact_commit_payloads_cover_create_update_move_remove_and_source_suppression` | Four runtime records have exact IDs/origin, create/update source payloads, move parent/target and remove new=None; initial source read emits no record. |
| F5 | acceptance.rs | `hundred_cycles_restore_registry_effect_baselines_and_release_native_captures` | 100 create/remove cycles restore registry/effect baselines, release native captures, remove listeners and end with empty registry/effects. |

These additions close bounded source adoption, malformed source-ID isolation,
create/update payload and typed false-condition activation dispositions, and
establish the typed failing-intercept adaptation. They strengthen several G
rows without closing unobserved topology, remaining commit fields, verdict
reads during gated unload, external-consumer provider migration or
source-options invalidation cases. Source provenance and original validation
cause assertions are now present; original JSON decoder cause downcasting and
the unknown-field rejection assertion remain distinct missing evidence.

## Declaration map

Each suite heading links to the pinned source. Line numbers identify the exact
declaration; shortened titles retain the behavior being mapped.

### [commit.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts) — 10 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 26 | create reports a created entry with its id assigned | R | F4 asserts the exact runtime entry ID/origin, root group None, old None and newly stored source value; L3 proves ID visibility during the create callback. Rust records are value snapshots. |
| 43 | update reports the new options together with the previous ones | R | F4 asserts the exact runtime ID/origin and accepted source change 1→2 in old/new snapshots. Mutable JS options-reference identity is replaced by immutable snapshot values. |
| 55 | move reports the target group | G | F4 asserts target group and old/new parent addresses for root→group. The `from` field and a move from a concrete old group are unasserted; immutable entry/Fiber rebuild is a Rust adaptation. |
| 65 | update in place does not report a move | G | F4 exercises in-place update and asserts its source payload; it does not assert `from: None`. |
| 72 | fiber.update from inside the plugin reports a config change | G | Exact inner-Fiber `hooks::commit` updates source and emits a record. No loader self-update fixture. |
| 82 | remove reports the removed entry | G | F4 asserts exact runtime ID/origin, removed target group and new None. The old removed snapshot/name/source is unasserted. |
| 93 | does not mistake a child fiber disposing itself for the entry (#19) | R | L1 distinguishes child registry removal from exact inner removal; only the latter commits disabled. |
| 107 | does not report anything for reads | G | S1/F4 assert no commits during initial source admission. Read-driven removal/update suppression remains unasserted. |
| 138 | keeps an entry that another group adopted | R | V13/V14 prove same-source root-to-group adoption preserves EntryId/address through previous root cleanup, including a gated disposer. M1/V4 cover nested-to-root adoption and old-source cleanup. No TS no-restart or mutable Fiber-identity equivalence is claimed. |
| 159 | does not persist child disposal while the tree loses its loader | D | A separate source tree losing its injected loader, with include flushing before stop, is deferred. R3 verifies parent drain joins detached cleanup, not write suppression. |

### [evaluate.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts) — 37 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 39 | creates no outer fiber for an entry without inject | A | Rust always owns an outer Fiber for enabled entries, even with empty inject (`spawn_outer`); E1 proves basic activation, not TS topology. |
| 48 | nests the inner fiber under the outer fiber | G | Outer entry dependencies and inner plugin are separately owned (`spawn_outer`/`start`). G2 exercises authority; exact parent Fiber topology is unasserted. |
| 63 | evaluates disabled with the entry inject | R | F2 reads its declared entry service: missing provider evaluates nothing, false verdict prevents apply, then true provider republication evaluates again and applies. Typed enabled conditions replace JS disabled expressions. |
| 83 | decides disabled before the plugin dependencies | G | G2 separates entry-condition and plugin readiness; F2 proves false condition suppresses apply. False condition with a missing plugin-level service remains untested. |
| 100 | waits for the entry inject while it is missing | R | G2 has no condition/mapping/apply before entry provider arrival; plugin provider arrival then activates independently. |
| 118 | treats a failing expression as disabled | A | Q3: typed condition failure produces Failed with visible error and explicit-update recovery. TS logs and treats expression failure as disabled; JS evaluation is unsupported. |
| 130 | evaluates inject values in the parent context | U | No parent-scoped dynamic injection-value evaluator. Supply static typed Dependency config or decode JSON before admission; JS `__jsExpr` is not executed. |
| 147 | cannot read the entry inject from an inject value | U | No dynamic inject-value expression or its evaluation Context; typed requirements are constructed explicitly. This TS expression rejection is not reproduced. |
| 160 | keeps an inject value on the outer fiber when the plugin declares the service itself | G | Entry Dependency configs and plugin declarations remain distinct. G2 uses different keys; same-key conflicting configs on outer/inner lack a fixture. |
| 175 | evaluates intercept with the entry inject, for the plugin only | G | `Scope::intercept_with` evaluates after entry injection and applies an inner view; R1 proves static typed intercept. Evaluation authority/outer exclusion are unasserted. |
| 189 | updates intercept in place | A | Immutable intercept layers replace mutable JS tables. Changing layer identity rebuilds Fibers; preserving a live table and avoiding apply is unsupported. |
| 205 | keeps the options when a dependency is lost and recovered | G | V1 proves entry dependency withdrawal/rearrival preserves disabled=false, emits no writeback, and reactivates twice. The expression/typed-condition source and saved outer identity of this TS case remain unasserted. |
| 228 | reloads the outer fiber when the verdict changes | A | Changing condition callback rebuilds the owner, unlike TS keeping the same outer and reconciling only the inner. Q3 proves explicit condition recovery; same-verdict inertness is untested. |
| 249 | rebuilds the entry when inject changes | G | N2 proves injection updates and clearing coalesce during gated retirement. Added/cleared-injection outer topology and final declared requirement shape remain unasserted. |
| 270 | disposes the outer fiber when the plugin disposes itself | G | L1 proves exact inner disposal disables; it does not configure entry injection or assert outer retirement. |
| 286 | disposes both fibers when the entry is removed | G | E1 proves dropped remove waiter still releases resources; joined injected outer/inner disposal lacks a focused fixture. |
| 301 | keeps a group structural and passes its verdict to the children | R | G1 and R2 keep groups structural, disable children, preserve IDs, and re-enable. Typed conditions replace JS expressions. |
| 321 | keeps the entry isolate without an inject | G | Typed isolation precedes requirement resolution; G3 tests scope move with plugin injection, not no-entry-inject condition isolation. |
| 339 | rebuilds without a write-back when inject is added | G | Rebuild retirement detaches exact inner before disposing, avoiding self-disposal writeback. No added-injection commit assertion. |
| 352 | drops an activation whose dependency is lost during the import | A | Catalog resolution and Scope callbacks are synchronous; no asynchronous import window. Latest revision guards cover typed activation; this Node import race is unavailable. |
| 380 | re-evaluates a failing inject value on refresh | U | Dynamic inject expressions and `Entry.refresh()` are unavailable. Explicit source update is the Rust retry path. |
| 394 | caches the verdict independently of reads and service mutations (group=%s) | G | F2 counts condition evaluations across provider republication and explicit updates. Repeated snapshot reads, in-place service mutation and the structural-group variant remain unasserted. |
| 427 | does not repeat a failing disabled expression when its verdict is read | G | Q3 proves failed condition, but no repeated-read counter/error assertion. JS side-effect expressions are unsupported. |
| 438 | evaluates a changed disabled expression once in either direction | G | F2 asserts static disable skips condition and explicit enable evaluates once; Q3 covers failing-condition recovery. Changed/failing callback evaluation counts in both directions remain unasserted. |
| 462 | invalidates the verdict before an asynchronous dependency unload | G | Loader snapshots derive Fiber state; no gated dependency-unload fixture observes verdict before cleanup or preserved outer identity. |
| 496 | keeps a disabled group verdict undecided when its dependency disappears | G | Structural pending group/verdict recovery lacks a focused loader fixture; static disabled-group coverage G1/R2 is narrower. |
| 513 | re-evaluates %s in the new parent context after a move | G | G3 proves new provider after group move; condition-in-new-parent is untested. Dynamic inject-value variant is unsupported. |
| 540 | re-evaluates the verdict after changing the entry isolation | G | G3 proves scope move rebuild, not condition verdict changes between isolated providers. |
| 558 | drops entries removed or disabled during %s | A | Async import/patch waterfall windows do not exist. L3 covers removal from create commit and U2 removal during cleanup; disabled-during-start variant remains untested. |
| 649 | does not start the plugin when intercept cannot be evaluated | A | N3 evaluates typed intercept only after its declared entry provider arrives, reads that provider, reports one failure without apply, and recovers through explicit update. Rust surfaces a failed settlement; TS logs and disables the entry. JS evaluation remains unsupported. |
| 661 | disposes the plugin when intercept stops evaluating on an update | G | Changing typed intercept rebuilds; failed intercept/repeated update/recovery after an active plugin has no fixture. |
| 689 | stops a group whose intercept stops evaluating on an update | G | No failed group-intercept fixture. R2 covers a disabled group, not failed intercept removal of its children. |
| 708 | does not evaluate intercept while disabled | G | Q3 proves disabled skips condition; it does not count intercept evaluation. Typed `start` skips intercept for disabled non-groups. |
| 721 | cannot read a service outside the parent context and the entry inject in disabled | G | G2 proves allowed entry service access; out-of-scope condition access refusal lacks a loader fixture. |
| 734 | cannot read a service provided by a sibling in an inject value | U | No sibling-reading dynamic inject-value expression. Static typed input and JSON decoding have no expression service access. |
| 745 | evaluates config with the plugin inject | R | G2 maps typed config in the inner plugin Context after its declared plugin service arrives; source remains unchanged. |
| 759 | cannot read a service outside every inject in config | G | G2 exercises allowed mapper access only. Undeclared service access in mapper lacks a negative fixture. |

### [group.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts) — 13 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 27 | initialize | R | G1/R2 create structural groups and children; N1 proves eight nested parent/group/child flag combinations with both groups active. The explicit Group representation replaces an imported group plugin. |
| 52 | disable inner | G | R2 proves disabling one group releases its child; N1 proves initial nested disabled combinations. Changing the nested inner group with exact disposer counts remains unexercised. |
| 60 | disable outer | G | N1 proves initial ancestor disable suppresses nested children while groups remain active; R2 proves a one-group disable transition. Nested ancestor transition counts remain untested. |
| 68 | enable inner | G | N1 proves an initially enabled inner group cannot bypass a disabled outer group; the explicit nested enable transition and zero-disposer assertion remain untested. |
| 76 | enable outer | R | R2 disables/re-enables a group and preserves child EntryId; nested descendants are not exercised. |
| 109 | initialize | G | G1 creates disabled/enabled groups and a child; three nested transfer targets lack a fixture. |
| 138 | enabled -> enabled | A | Cross-group Rust move rebuilds Fiber even between enabled groups; same-group reorder is inert (G1). TS enabled-to-enabled transfer has no apply/dispose. |
| 146 | enabled -> disabled | G | G1 tests disabled-to-enabled move; enabled-to-disabled target requires a focused fixture. |
| 154 | disabled -> disabled | G | Moving between two disabled ancestor chains is untested. |
| 162 | disabled -> enabled | R | G1 moves a child out of a disabled group into an enabled group; identity survives and applies once. |
| 187 | initialize | R | G1/R2 initialize children through Group and settle them; group is a typed structural entry, not an imported plugin. |
| 202 | config update settles children | R | R2 awaits child updates through 100 group cycles. F1 proves nested grandchild direct middleware remains joined by settlement even after its waiter drops. Newly added/gated child during a group config update remains a narrower missing scenario. |
| 253 | initialize | G | Immutable typed Context layers replace prototype-chain intercept tables. R1 covers one intercept; three-level group layering is untested. |

### [import.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/import.spec.ts) — 2 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 25 | keep the hints of Node after the first line | U | No Node module import, ERR_MODULE_NOT_FOUND hints, or V8 stack rewriting. UnknownFactory/typed errors are the Rust diagnostic boundary. |
| 42 | leave a module imported by another one to Node | U | No transitive JS module imports; registered Rust plugins own their errors. Node dependency/importer attribution is unavailable. |

### [index.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts) — 19 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 10 | evaluates the expression instead of treating the node as truthy | A | Typed bool/condition replaces truthy JS expression nodes; JSON disabled accepts bool/null and rejects expression objects (S1). |
| 43 | loader initiate | R | S1 and R4 reconcile source siblings; G1/R2 create groups/disabled children. Implicit group detection by imported plugin identity is replaced by EntrySpec::group. |
| 71 | loader update | G | `reconcile` updates/removes by source identity; fixtures cover initial reads and group updates, not unchanged-sibling apply count during source replacement. |
| 90 | plugin self-update | G | Exact inner commit hook preserves self-updated source. No loader self-update fixture. |
| 103 | plugin self-dispose | R | L1 exact inner registry removal commits disabled; nested child removal does not. |
| 139 | pending | A | No provided Loader service with `{await:true}` readiness intercept. Explicit `settle()` waits admitted work and permits Pending (U1/G2). |
| 162 | resolved | A | Use explicit settle/entry snapshots instead of implicit service readiness. U1/G2 show Pending and later provider activation; no loader-service check exists. |
| 188 | on load | R | A4 proves failed partial setup is contained, its listener removed, and healthy sibling continues. |
| 201 | on config update | G | V5 proves independent loaded-child validation in runtime group updates; V7 proves source validation failures. A repeatedly failing source-driven apply update alongside a healthy sibling remains untested. |
| 215 | on an id containing the separator | R | S3/V16 defer source ID validation per entry. Reserved/overlong source IDs yield located failures while the healthy sibling survives; runtime create rejects the malformed ID before admission. |
| 247 | wait for dependencies | R | G2 waits entry requirements before condition/mapping/inner activation; JS interpolation is replaced by typed mapper. |
| 261 | evaluate after dependencies are ready | R | G2 activates after entry and plugin requirements arrive and maps source config using plugin Context. |
| 279 | re-evaluate when dependencies reload | G | Mapper re-evaluation on provider replacement is delegated to core config hooks, without a loader provider-reload fixture. |
| 287 | keep the source config unevaluated | R | G2 snapshot source is 1 while evaluated config is 5; no JS expression execution. |
| 307 | skip nested plugins | G | Loader config hook matches exact inner FiberId, so child config is not mapped; no nested child-config identity fixture. |
| 344 | fail with dependencies available | R | U1 delays validation until plugin service exists, reports Failed, and avoids eager validator call. |
| 360 | recover through update | R | U1 and Q3 recover Failed through explicit accepted update; mapper-plus-failed-apply recovery remains untested. |
| 383 | persist an update to a pending fiber | G | Exact pending inner commit hook should preserve source; no pending Fiber self-update/commit fixture. |
| 436 | keep children enabled when the parent loses a dependency | D | Nested EntryTree plugin with its own injected source owner is deferred to include. Parent dependency-loss/no-disabled-writeback needs an integration fixture. |

### [internal.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/internal.spec.ts) — 1 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 5 | tags the running loader with the resolver signature it accepts | U | Node internal ESM loader discovery/signature tagging and ModuleJob/loadCache APIs are unavailable. In-memory replace_factory is not Node HMR. |

### [isolate-provide.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts) — 7 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 51 | keeps self access while the provision context changes realms | A | F3 proves six self-provider generations clean up through their original realm/value. Scope moves rebuild providers rather than mutating a provision Context during an in-place update; TS implementation migration is unavailable. |
| 69 | reloads a self-provider after an isolate-only move | A | F3 proves scope/group moves rebuild with new Fiber identity and original-view self-provider cleanup. Rust does not transfer a JS symbol/impl without apply, as this TS scenario does. |
| 106 | keeps an impl that reused the abandoned symbol | A | No abandoned JS symbol reuse or reflect.store mutation. Typed realm/publication ownership delegates cleanup to core; unrelated replacement-provider survival is untested here. |
| 143 | retains a moved provider until its consumers finish cleanup | G | L2 proves consumer retains old provider while the consumer moves. Moving the provider, gated consumer cleanup, then replacement publication is a different untested loader scenario. |
| 186 | keeps impls on intermediate symbols across repeated moves | A | F3 proves repeated typed realm moves create distinct generations and release each through its original view. Intermediate JS symbol/impl migration and unrelated publication survival are not emulated or covered by that fixture. |
| 245 | does not delete another realm when one impl moves | G | Typed realms select distinct core publications; moving one provider must preserve another realm. No focused multi-provider loader fixture. |
| 280 | notifies the retained realm on %s disposal after a blocked move | G | No blocked provider-move fixture asserting retained-realm notifications and exact consumer states for both Fiber/provide disposal variants. |

### [isolate.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts) — 21 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 38 | initiate | R | G3 starts provider and required consumer in inherited root scope. |
| 47 | add isolate on injector (relevant) | G | Typed scope change rebuilds into a fresh realm; G3 moves to a populated isolated group, not fresh missing-provider Pending. |
| 59 | add isolate on injector (irrelevant) | A | Scope layers are compared by identity; a changed irrelevant layer can rebuild. TS relevant-service-only inertness is not a Rust promise. |
| 72 | remove isolate on injector (relevant) | G | Removing a relevant layer should rejoin inherited provider through rebuild; no focused fixture. |
| 84 | remove isolate on injector (irrelevant) | A | Changing immutable Scope may rebuild even for an irrelevant service. No TS no-restart guarantee. |
| 94 | add isolate on provider (relevant) | G | Provider scope change rebuilds and publication loss/recovery delegates to core; provider-moving/consumer-unload fixture is missing. |
| 106 | add isolate on provider (irrelevant) | A | Changed irrelevant provider Scope may rebuild; TS inertness requires mutable realm-diff machinery that is not emulated. |
| 119 | remove isolate on provider (relevant) | G | Removing provider isolation and restarting waiting root consumers is untested at loader level. |
| 131 | remove isolate on provider (irrelevant) | A | Clearing irrelevant provider Scope may rebuild; TS unchanged apply/dispose counts are not promised. |
| 172 | add isolate group | R | G3 creates an isolated structural group with its own provider, alongside root provider; typed explicit realms replace labels. |
| 201 | update isolate group (no change) | A | Reusing the same cloned Scope preserves layer identity; constructing a new fresh-isolation Scope allocates a new realm. Equivalent JSON true is not stable realm identity. |
| 218 | realm reference | G | Scope::isolate_with shares explicit IsolationId and inherited layers; fresh/shared/nested selection combinations need a focused fixture. |
| 251 | special case: nested realms | A | No mutable prototype realm update with inert nested owners; Rust group Scope changes rebuild descendants. Shared nested selection evidence is missing. |
| 326 | special case: change provider | G | G3 proves consumer selects new provider after its move; changing ancestor group Scope under an existing consumer is untested. |
| 390 | special case: change injector | G | Moving group provider scope to switch two external consumers is untested. |
| 490 | initiate | R | G3 creates isolated group plus root provider/consumer with expected root value. |
| 507 | transfer injector into group | G | G3 moves consumer into populated group; TS target initially lacks provider and becomes Pending. Missing-provider variant untested. |
| 515 | transfer provider into group | G | Moving provider into waiting consumer group lacks fixture; no publication migration without rebuild is promised. |
| 523 | transfer injector out of group | G | Consumer transfer out of group is not directly exercised; G3 has one inward move. |
| 531 | transfer provider out of group | G | Provider transfer back to root/recovery of root consumer is untested. |
| 541 | keeps the committed views of a parent and a pinned child across a realm move | G | L2 verifies one moved consumer old view; F3 verifies repeated self-provider old views. Parent plus pinned nested child cleanup across realm move remains missing loader evidence. |

### [rebuild.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts) — 12 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 44 | rebuilds an active entry when its name changes (inject=%s) | R | U2 rebuilds active factory a→latest c; prior apply completes before next generation. Entry-injection variant is untested. |
| 58 | uses the new name when it changes during the %s | A | No asynchronous module import or patch waterfall. U2 tests latest factory during joined cleanup, not these unavailable windows. |
| 97 | joins a second rename while the first disposal is pending (inject=%s) | R | U2 gates first cleanup, accepts b then c, and observes only a,c. Injected variant remains untested. |
| 117 | applies nothing when the entry is removed during the disposal | G | U2 removes during a later rebuild; removal while first disposer is gated, with entry injection, is narrower missing evidence. |
| 139 | rebuilds when the plugin renames its own entry | G | No plugin-triggered rename-during-apply fixture. Q1 tests reentrant config mutation refusal, not this scenario. |
| 159 | starts a pending entry when inject is cleared | G | EntryPatch clears requirements and revision work rebuilds; no focused Pending→Active injection-clear fixture. |
| 186 | reapplies when inject gains a service that is already provided | G | No focused active entry gaining already-present entry service fixture. |
| 213 | keeps the new outer fiber pending and drops the old listener when inject is missing | G | A4 covers failed partial listener cleanup, not active→missing-injection rebuild old-listener cleanup/Pending. |
| 243 | joins a second inject update while the first rebuild is disposing | G | N2 gates old cleanup and accepts two missing-requirement updates followed by clearing; only values 0 and 3 apply. The latest-already-provided requirement and single active Fiber assertions in the TS fixture remain untested. |
| 280 | uses the latest name during a pending %s | A | No pending import/failed-import/context-patch window; catalog checks and Scope construction are synchronous. U2 covers pending disposal only. |
| 322 | does not apply a fiber removed while rebuild is disposing | G | U2 proves removal prevents recreation during rebuild, but no injection-change/gated-cleanup variant. |
| 349 | keeps a rebuild started during apply until its disposer settles | G | No fixture for a rebuild requested during apply, a gated disposer, and a third update; core reentrancy may refuse the originating mutation. |

### [resolve.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts) — 12 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 47 | resolves bare specifiers against the project, not the loader | U | No npm/project bare-specifier resolution; Catalog names refer to statically registered typed plugins. |
| 57 | keeps subpath imports and self references working | U | No package imports/exports or JS package self references. |
| 64 | anchors the helper at the package.json, not at the base url | U | No package.json-anchored resolve.mjs helper or filesystem side effects. |
| 71 | makes the helper directory ignore itself | U | No generated .cordis helper directory/.gitignore. |
| 78 | reuses one helper per scope | U | No Node resolver helper cache per package scope. |
| 85 | declines when there is no project to anchor to | U | No URL/project-scope lookup or native-import fallback; missing Catalog name returns UnknownFactory. |
| 133 | reuses an identical helper without attempting rename | U | No resolver helper writes or rename avoidance. |
| 146 | retries a stale helper after %s | U | No EACCES/EPERM/EBUSY resolver-helper rename retries/backoff. |
| 156 | cleans up after $code and recovers on a later call | U | No resolver temp-file cleanup/retry; declaration title is “cleans up after $code and recovers on a later call”. |
| 187 | routes bare specifiers to the project helper | U | No Node import fallback through a project helper. |
| 194 | still resolves relative specifiers against the base url | U | No relative JS specifier/baseUrl import resolution. File source transport belongs to future include, independently of module loading. |
| 202 | accepts a drive-letter path | U | No Windows drive-letter JS imports. Platform path transport is not exercised by loader catalog tests. |

### [validate.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/validate.spec.ts) — 6 declarations

| Line | Declaration | Mark | Rust representation, evidence and boundary |
|---:|---|:---:|---|
| 9 | accept valid options | A | Typed EntrySpec plus JSON decoder replace loose options. S1/R1/S2 cover valid forms; Rust rejects JS disabled objects and permits isolate false/null as inherited layers. |
| 27 | reject a missing or invalid field | G | S3 verifies reserved IDs; S2/R1 verify child decode/unknown service/bad intercept. Full missing/wrong field path matrix is unasserted. |
| 41 | accept an expression only for disabled | A | S1 rejects JSON disabled expression objects; arbitrary JS expression execution is unavailable. Other fields go to typed decoders, which may accept markers as opaque data. |
| 52 | reject invalid options at runtime before they are committed | R | E1 rejects unknown factory, wrong typed config, duplicate ID before admission; U1 rejects invalid loaded config retaining prior source. |
| 71 | check the entries of a group passed at runtime as a whole | R | S2 rejects invalid runtime child decode before any group entry exists; A2 verifies whole-group capacity preflight. |
| 96 | unload an entry whose options turn invalid, and reload it once fixed | G | S1/R4/V6 prove independent initial invalid sibling exclusion. V7 proves validation failure/recovery/retained loaded config, not valid-to-invalid entry-options removal followed by fixed-source reload with zero writeback. |

## Remaining evidence and functional boundaries

The remaining useful fixtures are move/from/remove-old commit fields; inner
self-update (including Pending); entry-injection add/clear topology; false/failing
condition read caches and observations during gated dependency unload; failed
intercept on active plugin/group updates; source valid→invalid→fixed options
reconciliation; provider realm moves with separate retained consumers and pinned
nested child cleanup. F3 covers repeated self-provider generations, and F5 now
covers 100 registry/effect/capture restoration cycles. These are focused loader gaps; they
are not grounds for reopening accepted B0 evidence wholesale.

V6/V7 now assert SourceId, structured paths, caller-provided SourceSpan and
original validation error cause. V12 asserts uncompacted nested JSON paths and
retained transport. Original JSON decoder cause downcasting remains unasserted. Unknown JSON fields are rejected by loader decoding,
but A5 does not assert that policy. S3/V16 now distinguish document-wide
byte/depth/node/string limits from per-entry ID validation: reserved/overlong
source IDs preserve valid siblings, while runtime create rejects invalid IDs.
JSON isolate values
false/null add no layer; TS validation rejects those per-service values.
Repeated serialized `isolate: true` decodes allocate fresh realms, unlike TS
local-realm identity retained on the Entry. Explicit `IsolationId`/cloned Scope
is the stable Rust sharing boundary.

The loader does not recursively detect or execute expression markers within
factory/service payloads: registered decoders may accept `__jsExpr` as ordinary
data. Only disabled-expression objects have an explicit current rejection
fixture. Dynamic injection-value evaluation/refresh, mutable live intercept
tables, symbol/implementation migration without rebuild, arbitrary JS imports,
Node resolver helpers/cache internals and V8 stack editing are unsupported.
File persistence and automatic HMR remain separate deferred work.

Coverage accounting (primary row dispositions, not acceptance counts): **R: 29**, **A: 25**, **U: 19**, **D: 2**, **G: 65**; **total: 140**.
