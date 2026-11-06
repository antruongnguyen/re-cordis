# B0 coordinated core ledger

Date: 2026-11-02. Executor: fresh `gpt-6.1-sol/high`.
**Reviewed fragments assembled and mechanically validated; final independent
Astra acceptance remains required.** B0 is not marked done; T0 is unstarted.

## Baselines and carried evidence

- Integration input: `a54d0452ef19afe28996e0e9d9ae95b6e702b824`.
- Isolated worktree:
  `/Users/I756434/.codex/worktrees/b0-final-ledger-integration/cordis-rust`.
  Source checkout for ignored TypeScript references:
  `/Users/I756434/Projects/private/cordis-rust`.
- Immutable inventory control: `8c9a191bff8201265b17eb82e7b9a6e3a59c5ad2`,
  loaded from Git, never from the assembled output. Its Rust provenance
  `9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc` and implementation provenance
  `808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f` are retained.
- TypeScript specification:
  `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
- Matrix execution began at
  `578c066b8618bec9b1b2afc0a9757bb9b0f3a87e`; accepted command/log evidence
  is integrated at `7d1552a`. All 71 recorded executable hashes match this
  worktree and the recorded before/after snapshots.

The [master reuse rule](../../../README.md#prior-decisions-and-evidence-reuse),
[B0 Tasks 2–3](../plan.md), [accepted decisions](../../../../../decision.md),
[limitations](../../../../../limitation.md) and
[previous completion boundary](../../../../../implementation-plan.md)
remain the contracts. The uncovered delta is assembly, inventory accounting
and validator controls. M0–M6 local hosting, typed adapters, provider retention,
Pending/readiness/Failed recovery, started cleanup and Effect/Fiber Drop
decisions were carried forward. No accepted semantic audit was repeated.

## Accepted review chain and inputs

| Evidence | Accepted review / integration |
|---|---|
| [Matrix and report](../matrix-rereview.md) | `b87c1b9`; commands/logs `7d1552a` |
| [Decorator/reflect FR-1](../fragment-rereview.md) | `b28a608`; correction `59be2db` |
| [Recovered integrated validation](../resume-validation-review.md) | `e9c0be3` |
| [First two events](../events-first-two-review.md) | `11d84b1` |
| [Scoped/logging SL-1–SL-6](../scoped-logging-rereview.md) | `b515447`; correction integration `c462dfd` |
| [Lifecycle/update LR2–LR5](../lifecycle-update-rereview.md) | `6945d90`; correction integration `f438233`; LR1 accounting below |
| [Services SR-1–SR-5](../services-rereview.md) | `1083dfa`; correction integration `a1c6ef7` |
| [Events/effects EET-1–EET-9](../events-effects-types-rereview.md) | `a54d045`; correction integration `1ac2986` |
| [Six event types](../events-effects-types-review.md) | `ff414b5`; unchanged accepted type records |

The fifteen merge inputs are decorator-reflect, events-first-two, events-rest,
associate, invoke, intercept, isolate, logger, shadow, plugin, fiber, update,
dispose, service and service-dispose. Each resulting declaration equals its
accepted fragment after removing only the three embedded LR1 records and
recomputing parent classes. The obsolete events.json was not used.
[Event types](event-types.json) is a byte-for-byte copy of the accepted input.

## Accounting

| Denominator | Count |
|---|---:|
| Original declarations / registered variants | 144 / 156 |
| Original syntactic runtime inventory records | 532 |
| Original records retained as behavioral obligations | 525 |
| Confirmed inventory non-obligations, separately retained | 7 |
| Additional prior empty paths / reviewed splits | 4 / 11 |
| Final meaningful runtime obligations | 540 |
| Separate type obligations | 6 |
| Unique Rust test/Rustdoc identities | 94 |
| Rust evidence references / both-toolchain result-and-log joins | 417 / 834 |
| Obligations with paired evidence / paired references / reused scenarios | 24 / 32 / 4 |

Arithmetic: **532 − 7 + 4 + 11 = 540**. The input fragments contain 543 raw
records because they still include three LR1 artifacts and already omit the
four inactive service-dispose records. The final runtime ledger has 540
behavioral records plus seven metadata records. Its original 532 IDs are
exactly partitioned into 525 behavioral IDs and seven non-obligation IDs.

All eight original parameter sets are unchanged: service-dispose :31/:95
actions, :156 fail, :238 ownerShutdown, :323 rootShutdown, :386 withdrawnFirst;
update :7 inject and :96 restart/config ABA. There is no inferred Cartesian
product. Empty-path additions retain :103/empty-105, :145/empty-147,
:164/empty-166 and :183/empty-185; the other eleven additions preserve reviewed
label/message, topology and payload-sequence splits.

| Evidence class | Runtime | Types |
|---|---:|---:|
| Source-mapped test | 180 | 0 |
| Rust representation | 186 | 6 |
| Executed comparison | 24 | 0 |
| Unsupported | 3 | 0 |
| Missing evidence | 147 | 0 |
| **Total** | **540** | **6** |

Runtime execution statuses are 390 passed, 147 not-executed and three
unsupported/not-applicable. Seven non-obligations contribute to none of these
counts. Type records are six passed Rust representations, not TS executions.
Combined class output includes 192 representations; runtime and type columns
remain separately available in cases.json.summary and validator output.

The 417 references include valid partial witnesses retained in gaps. Their
passing Rust joins do not close the 147 missing assertion/variant boundaries.
All 15 distinct restriction anchors remain checked. The three original
unsupported reflect obligations (:6/a9 and :12/a16,a17) retain their exact
existing M6 restriction dispositions; none is parity.

## Seven exact non-obligations

Each record in cases.json.inventory_non_obligations retains its original
ID, inventory commit/file/Rust revision, declaration/source/assertion lines,
variant parameters, original source text, context lines, review reference,
reason and `execution_status=not-applicable`. It has no behavioral class,
gap or execution evidence.

| Original ID | Confirmed reason / review |
|---|---|
| core/plugin.spec.ts:77/default/a84 | Closing token; [LR1](../lifecycle-update-review.md#lr1--separate-three-non-obligations-from-behavioral-gaps-p2) |
| core/fiber.spec.ts:71/default/a86 | Commented expectation; [LR1](../lifecycle-update-review.md#lr1--separate-three-non-obligations-from-behavioral-gaps-p2) |
| core/update.spec.ts:7/false/a21 | Inactive inject guard; [LR1](../lifecycle-update-review.md#lr1--separate-three-non-obligations-from-behavioral-gaps-p2) |
| core/service-dispose.spec.ts:95/provider/a125 | Inactive restart guard at 123–126; [services confirmation](../services-review.md#confirmed-non-obligation-provenance-for-the-integration-owner) |
| core/service-dispose.spec.ts:95/root/a125 | Same guard, exact root variant |
| core/service-dispose.spec.ts:95/provider first/a125 | Same guard, exact provider-first variant |
| core/service-dispose.spec.ts:95/consumer first/a125 | Same guard, exact consumer-first variant |

The inject=true/a21 and restart/a125 obligations remain behavioral. The
seven dispositions have independent review. The validator's exact allowlist
uses immutable Git provenance, including JSON type distinctions; its
implementation awaits final independent review. Arbitrary omitted IDs cannot
become metadata. This satisfies the sole LR1 integration condition without
altering the accepted lifecycle correction.

## Gap owners, actions and dependency routing

All **147 gaps** have kind missing-evidence and owner **core evidence
maintainer**. Each exact ID, concrete regression action and conditional
work-package route remains in [cases.json](cases.json), unchanged from its
accepted fragment. Family counts and representative concrete work are:

| Family | Gaps | Required follow-up |
|---|---:|---|
| associate | 2 | Initial authorized accessor read :114/a169 and nested Debug forwarding :235/a247 |
| decorator | 3 | Zero stop count after owner/child readiness; exact call count after first withdrawal |
| reflect | 4 | Typed metadata versus service access, warning reset/read checkpoints and child metadata propagation |
| invoke | 4 | Callable intercept/extension composition and mapped-state results at :5/a46,a53,a56,a59 |
| intercept | 1 | Zero-layer config baseline at :34/a37 |
| isolate | 34 | Own-provider/no-inject access, multi-realm publication/withdrawal counts and values, retained cleanup reads and no-error checkpoints |
| logger | 4 | Exact first/second exporter removals and destination counts; retained empty-aggregate error identity |
| shadow | 5 | Returned-holder probes from root and injected consumer, direct isolated-realm inspection, loader-bound child injection and no-error checkpoints |
| plugin | 14 | Admission callback zero count, returned-disposer/initialization counts, repeated-drain Registry/journal checkpoints |
| fiber | 10 | Healthy-sibling dispatch after failed rollback, retry update result from Failed, throwing-cleanup counter, update-input vectors and final Active state |
| update | 13 | Active inject=true value, all pending waiters, explicit final Active and update/config boundary checks |
| dispose | 14 | Effect metadata/default labels, second-disposal count, consumed-handle rejection, callback messages and gated acquisition/stream traces |
| events | 24 | Bucket survivors, empty settlement, named/callback errors, veto/Next-frame and terminal-count boundaries, complete payload journals |
| service | 9 | Initialization/DAG counts, reversible root-owned Counter effects and intermediate getters |
| service-dispose | 6 | Original cleanup error allocation, composed outer-owner shutdown, provider/current and in-flight value-selection observations |

The seven original decorator/reflect gaps are retained object-identically:
decorator :7/a32,a35,a37 and reflect :30/a43,a47,a51,a52. In particular, add
stop counters after each relevant ready barrier, a call count after withdrawal,
typed Baz metadata checks in the child, and exporter warning snapshots at the
exact reset/read boundaries. These gaps were not erased during non-obligation
accounting.

A passing targeted regression closes its evidence gap without production
changes. A dependent satellite gate must identify its needed boundary; if
that boundary fails through the public API, the failing reproducer and
dependency open **conditional B2**. Assembly establishes no production failure
and admits no B2 package. Timer T0 must assess required admission/cleanup
boundaries; loader L0/L1/L2/L3 and later include/HMR gates must assess their
needed ownership, service-retention, lifecycle or payload boundaries.
This names triage points without asserting that every gap blocks every gate.
T0 and all subsequent packages remain unstarted.

## Current checks and evidence limits

From the isolated worktree, the normal complete-data command was run:

~~~sh
python3 -B docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/validate.py --source-repo /Users/I756434/Projects/private/cordis-rust
~~~

Exit **0**, actual output:

~~~text
B0 ledger validation passed: {"added_runtime_obligations":15,"classes":{"executed-comparison":24,"missing-evidence":147,"rust-representation":192,"source-mapped-test":180,"unsupported":3},"declarations":144,"inventory_non_obligations":7,"obligations":540,"original_runtime_inventory":532,"paired_scenarios":4,"retained_runtime_inventory":525,"runtime_classes":{"executed-comparison":24,"missing-evidence":147,"rust-representation":186,"source-mapped-test":180,"unsupported":3},"runtime_execution_statuses":{"not-applicable":3,"not-executed":147,"passed":390},"rust_test_identities":94,"type_classes":{"rust-representation":6},"type_obligations":6,"variants":156}
~~~

The focused self-test command adds `--self-test` to the same invocation.
It exited **0** with the real complete dataset as its positive case and
**36 expected negative checks**. See [validator report](report-validator.md)
for exact command, output, earlier red attempts and the independent assembly
check. That check confirms exact fragment equality, original-ID partition,
the unchanged seven original gaps, 53 zero-exit command/log records, all 71
current hashes, clean TS pin, absent temporary fixture and ledger-only diff.

Applicable matrix results remain stable and 1.85.0: 164 core integration tests,
15 timer integration tests and 17 core Rustdoc checks per toolchain, with zero
failed/ignored/filtered workspace results. M0, docs, examples and paired-run
results retain the accepted [matrix report](../matrix/report.md) scope.
No Cargo, docs-example, TS or parity suite was rerun in this task.

The retained paired corpus is six scenarios / 73 observations (combined 11,
coalesced 14, removed 12, superseded-failure 10, latest-failure 13,
explicit-recovery 13). The ledger's four claimed scenarios are combined,
explicit-recovery, latest-failure and superseded-failure. No new execution or
parity percentage is claimed. Mechanical validation proves accounting,
references and evidence applicability; accepted family semantics and final
independent acceptance are separate gates.
