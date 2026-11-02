# Decorator and reflection B0 fragment

Completed five declarations / five default variants / 23 assertion obligations from decorator.spec.ts (one declaration) and reflect.spec.ts (four). Schema and declaration/obligation IDs come from [cases.json](../ledger/cases.json); the merge input is [decorator-reflect.json](decorator-reflect.json).

| Evidence class | Obligations |
|---|---:|
| source-mapped-test | 9 |
| rust-representation | 4 |
| unsupported | 3 |
| missing-evidence | 7 |
| executed-comparison | 0 |

The fragment claims no paired parity. Four distinct current Rust tests substantiate the included behavioral mappings/substitutes; a fifth passing test documents concrete Any type detection alongside the excluded subclass-brand assertion. Those five test names appear as successful re-cordis-core tests in both stable-workspace-test and 1.85.0-workspace-test in [matrix/results.json](../matrix/results.json), [stable log](../matrix/logs/stable-workspace-test.log) and [MSRV log](../matrix/logs/1.85.0-workspace-test.log). This worker ran no Cargo/docs/parity commands.

The TS pin is 35fdc969e4b6c48c48ed44717c7a018324e4dd88. The ledger inventory revision is 9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc. Matrix before/after HEADs are 578c066b8618bec9b1b2afc0a9757bb9b0f3a87e and e23dbac846454d4573eee2ba3eebbc5e33085d17; their executable-input hashes match and their TS HEADs agree with the fixed pin. These are attributable existing results, not newly executed runs at the fragment-writing revision.

The method adapter explicitly tests zero activation before foo, one activation on its first arrival, and one cleanup on its first withdrawal. It does not assert cleanup is zero before arrival, cleanup is zero after first activation, or callback count remains one immediately after first withdrawal. Those three boundaries are gaps, despite final aggregate calls/stops equality.

Reflection deliberately excludes JS subclass branding and prototype/constructor interception using the existing [M6 restrictions](../../../../../m6.md#remaining-restrictions-and-safety-deviations) and [source-map audit](../../../../../m6.md#source-map-audit). Any::is::<Context> tests concrete type detection only. Typed checked-access tests cover undeclared/inactive errors, provision admission, duplicate rejection and successful owner set; they assert Rust variants rather than JS error strings. The absent-publication set test runs at root, where the explicit API has the same provision requirement; the duplicate test attempts from another Context rather than the exact same plugin. Those mapping/setup differences are disclosed in the obligations, not promoted to executed comparisons.

Accessor-versus-service lookup is explicitly represented and tested. Remaining reflection gaps are ordinary metadata-versus-service namespace distinction, the reset warning-spy baseline, metadata inherited into a plugin created from an extended injecting Context, and warning silence after that nested metadata read. Direct metadata-layer and unrelated logger tests do not close these combined boundaries.

All seven gaps carry concrete regression actions, owner core evidence maintainer, and B2 targeted evidence regression before the dependent satellite gate:

- G-B0-decorator.spec.ts:7/default/a32
- G-B0-decorator.spec.ts:7/default/a35
- G-B0-decorator.spec.ts:7/default/a37
- G-B0-reflect.spec.ts:30/default/a43
- G-B0-reflect.spec.ts:30/default/a47
- G-B0-reflect.spec.ts:30/default/a51
- G-B0-reflect.spec.ts:30/default/a52

These gaps identify missing assertions, not confirmed production failures. Review should check whether the intentional Any/accessor substitutes and disclosed typed-access setup differences adequately dispose of the exact source obligations before merging. Only the two ledger-inputs fragment files are owned; ledger, progress, production, tests and manifests were untouched.

Validation: local Python accounting/reference check confirmed five unique expected declarations, 23 unique original assertion obligations, valid evidence fields/anchors/line bounds, and every cited Rust test in both passing matrix results and original logs. The full ledger validator remains the ledger owner's integration check.
