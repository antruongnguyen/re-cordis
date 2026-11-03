# HMR H3 pinned source disposition map

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
TypeScript pin: `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
Rust design baseline: `47042d9`; approved H0 commit: `5065adc`.
Status: declaration accounting complete; H1–H3 execution and acceptance pending.

This map records every test declaration in pinned `packages/hmr/tests/index.spec.ts`
and `path.spec.ts`, plus the HMR implementation surfaces and relevant fixtures.
CodeGraph supplied source first; IDE VFS reads filled omitted bodies. This task
did not execute the TS suite or the evolving Rust HMR suite. Line numbers below
are at the pin, relative to `local/cordis/packages/hmr/`.

The unchanged ownership, provider retention, hosting and entry lifecycle claims
reuse [loader acceptance](loader-acceptance.md), [loader scoped re-review](loader-rereview.md),
[Include acceptance](include-acceptance.md), [Include final re-review](include-final-rereview.md)
and accepted timer/core task contracts carried by [H0 review](hmr-h0-review.md).
No B0 re-audit is selected. The new delta is the owned watcher, retained Include
completion, explicit graph selection and whole-batch factory admission.
    H3's optional `ReloadPlan::replace_include` uses the same selection checks and
    forwards the exact `Include::replace_factories` loader batch completion for
    entries managed by the same Include. It preserves journal, data, provenance
    and file bytes and performs no flush; execution remains pending.

## Disposition keys

| Key | Rust representation and required oracle |
|---|---|
| A | Carried core/loader behavior, exercised through an HMR replacement in R. Initial registration/event/service behavior alone adds no new HMR equivalence claim. |
| W | [Watch contract](hmr-watch-contract.md): exact registrations; synthetic duplicate/burst/overflow tests, generation suppression, explicit ignore override, callback error/panic recovery, hosted cancellation and joined producer/capture release. Real bounded file tests supplement it. |
| I | Optional HMR-side Include adapter: explicit source path, last-valid parse recovery, rename-based self-write convergence, retained already-admitted IncludeTask completion, no follow-up after stop and resident-work quota. H3 integration execution remains required. |
| R | [Replacement contract](hmr-reload-contract.md): explicit graph/candidates, admission before teardown, original per-entry configs, exact old resource/provider release, independent siblings, latest-intent/nested host guards, post-unload diagnostics/recovery and 100-cycle baselines. Execution remains required. |
| N | Node-specific mechanism unsupported: imports/exports, ModuleJob discovery, ESM/CJS caches, syntax compilation, JS filename resolution and process exit. W/I/R may represent the useful resulting behavior, without executing that mechanism. |
| P | Native path adaptation with missing Windows runtime evidence. macOS file smoke is insufficient; Linux runtime evidence is also not established by it. |

No key asserts one-to-one trace parity. In particular, supplying an already-linked
Rust factory replaces executable behavior; it does not recompile changed Rust
source. Source registration emits `RestartRequired`; the application owns any
build/restart supervisor. There is no native ABI swap or binary hot loading.

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
Table entries are required or carried contract oracles, not executed Rust test
results; final acceptance must add actual frozen Rust test names and logs.

| ID | index.spec.ts line | Exact declaration name | Disposition |
|---|---:|---|---|
| I01 | 171 | should load plugin and respond to events | A/N |
| I02 | 176 | should reload plugin when file changes | R/W/N |
| I03 | 189 | should handle reverting file back to original | R/W/N |
| I04 | 202 | should emit hmr/reload event on reload | R; Settlement adaptation |
| I05 | 216 | should properly dispose old plugin effects on reload | R/A |
| I06 | 259 | should load multiple plugins | A/N |
| I07 | 264 | should reload changed plugin without affecting others | R/N |
| I08 | 276 | should handle simultaneous changes to both plugins | R/W/N |
| I09 | 317 | should load plugin with dependency | A/N |
| I10 | 321 | should reload plugin when its dependency changes | R/N |
| I11 | 334 | should reload plugin when plugin file itself changes | R/N |
| I12 | 349 | should not reload a sibling that only shares an unchanged module | R/N |
| I13 | 397 | should load plugin normally | A/N |
| I14 | 401 | should rollback on syntax error and keep old plugin working | N; R admission analogue only |
| I15 | 415 | should recover after fixing the error | N; R valid-candidate recovery |
| I16 | 458 | should batch rapid changes within debounce window | W/R/N |
| I17 | 504 | should handle config file update to disable plugin | I/A |
| I18 | 565 | keeps watching the file across its own rename-based writes | I/W |
| I19 | 590 | logs instead of rejecting when the config file is unparsable | I; retained failure event adaptation |
| I20 | 663 | should load service plugin | A/N |
| I21 | 672 | should reload service plugin on change | R/A/N |
| I22 | 705 | should maintain entry association after reload | R/A |
| I23 | 750 | should handle many rapid reloads without crashing | W/R; stronger 100-cycle resource oracle required |
| I24 | 785 | should register new event handlers after reload | R/A/N |
| I25 | 811 | should remove old event handlers after reload | R/A/N |
| I26 | 856 | should return linked dependencies for a loaded file | N; explicit R graph replaces discovery |
| I27 | 865 | should return empty array for unknown file | N; R rejects unknown unit |
| I28 | 894 | should handle apply error gracefully without crashing | R/A |
| I29 | 960 | should clear stashed files after successful reload | W/R; bounded dirty state adaptation |
| I30 | 996 | should load the nested entry | A/N |
| I31 | 1001 | should instantiate a nested entry exactly once when its host reloads too | R/A/N |
| I32 | 1019 | should skip a nested entry whose host has not started unloading | R/A; latest host/entry oracle |
| I33 | 1069 | should load both plugins | A/N |
| I34 | 1074 | should not start the new instance before the old one released | R/A |
| I35 | 1085 | should not delay an unrelated plugin behind a slow one | R/A |
| I36 | 1142 | should leave the running plugin untouched | R/N; malformed JS export has no Rust shape analogue |
| I37 | 1161 | should recover after fixing the export | R/N |
| I38 | 1207 | should rebuild every fiber, each keeping its own config | R/A; per-plugin log dedup is not claimed |
| I39 | 1272 | watches a path that `ignored` would skip | W; explicit component override |
| I40 | 1281 | runs every callback registered for the same path | W |
| I41 | 1292 | matches paths that differ in case on Windows | P/W; Windows-only, unexecuted |
| I42 | 1347 | should start and warn once | W; pure capability snapshot replaces warning |
| I43 | 1351 | should emit hmr/change for a source file instead of reloading it | W/N; RestartRequired adaptation |
| I44 | 1361 | should still reload config files | I/W |
| I45 | 1427 | does not rebuild an entry disabled during unload | R/A |
| I46 | 1448 | does not rebuild an entry removed during unload | R/A |
| I47 | 1469 | waits for a second re-enable init before deciding whether to rebuild | R/A |
| I48 | 1532 | rebuilds a child of an active host that reused the plugin | N/R boundary: arbitrary core child of unchanged host is unsupported; supply declared host candidate |
| I49 | 1563 | skips a child whose host is replaced during unload | R/A; stale host-child guard |

| ID | path.spec.ts line | Exact declaration name | Disposition |
|---|---:|---|---|
| P01 | 39 | accepts a drive-letter base | P/W; Windows-only, unexecuted |
| P02 | 46 | resolves a relative base against the context | W/P; explicit native base replaces context file URL |
| P03 | 52 | defaults to the context directory | W/P; caller must supply WatchOptions::new(base) |
| P04 | 63 | does not watch a nested ignored directory | W/P; exact observed-path filtering, no recursive root walk |

I39/I40 deliberately use a file no module imports (`index.spec.ts:1267–1270`).
The pin leaves callback-plus-module-reload behavior unpinned. This map does not
infer a stronger ordering guarantee. I18/I19 use an inline atomic-config fixture;
I20/I21 generate a Service fixture. I48 proves a capability Rust deliberately
does not provide through an unchanged compiled host.

## Fixture and helper scope

Read the complete inline fixture variants in both spec files. The 19 source
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

The final H3 gate must attach actual Rust W/I/R test references and explicit
stable/1.85.0 results, plus independent frozen-diff review. Existing H0 dependency
probe results remain in the watch contract; they do not execute this HMR crate.
Keep missing Windows and Linux runtime evidence visible. Portable std polling
and native paths do not establish those platforms by compilation alone.

Polling samples only registered regular files, reads a bounded prefix and digest,
and may miss changes entirely between samples or digest collisions. Symlinks are
refused without claiming hostile-filesystem confinement. Missing files and
parent remove/recreate retain their logical registrations; parent renames do not
move them. Queue overflow rescans only the bounded registration set.
