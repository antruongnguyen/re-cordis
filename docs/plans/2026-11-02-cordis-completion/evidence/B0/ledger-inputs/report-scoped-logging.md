# Scoped and logging family reconciliation

Status: complete family fragments; independent integration validation/review pending.

## Scope and revisions

- Assignment: scoped-logging from the [remaining-reconciliation handoff](../handoffs/remaining-reconciliation.md).
- TypeScript specification: cordiverse/cordis 35fdc969e4b6c48c48ed44717c7a018324e4dd88; the local checkout reported this exact revision and no local changes.
- Rust evidence revision recorded by the accepted matrix: 9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc.
- Executable implementation revision: 808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f. git diff --name-only 808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f..dc60fedd5575f46c4ee7df2b6652ec7ca8bd6653 -- crates/re-cordis-core returned no paths, so the accepted matrix remains applicable to every cited Rust test.
- Worktree base: integration commit dc60fedd5575f46c4ee7df2b6652ec7ca8bd6653.

The carried decisions are the explicit ServiceKey/AccessorKey association adapters, ServiceInvoke, BoundService definition/use/caller views, retained isolation history, typed intercept merging, explicit logger name/error representations, and fixed 1000-record Rust buffer documented in [M5](../../../../../m5.md) and [M6](../../../../../m6.md). This reconciliation does not reopen those representations. Its uncovered delta is assertion-level provenance and execution joins.

## Completed fragments

| Family | Fragment | Declarations | Variants | Obligations | Current Rust tests | Class totals |
|---|---|---:|---:|---:|---:|---|
| associate | [associate.json](associate.json) | 6 | 6 | 24 | 3 | 23 rust-representation; 1 missing-evidence |
| invoke | [invoke.json](invoke.json) | 2 | 2 | 11 | 1 | 8 rust-representation; 3 missing-evidence |
| intercept | [intercept.json](intercept.json) | 7 | 7 | 8 | 2 | 7 source-mapped-test; 1 missing-evidence |
| isolate | [isolate.json](isolate.json) | 6 | 6 | 53 | 4 | 25 source-mapped-test; 28 missing-evidence |
| logger | [logger.json](logger.json) | 11 | 11 | 20 | 4 | 13 rust-representation; 6 source-mapped-test; 1 missing-evidence |
| shadow | [shadow.json](shadow.json) | 8 | 8 | 20 | 4 | 17 rust-representation; 3 missing-evidence |
| **Total** | six fragments | **40** | **40** | **136** | **15 distinct tests** | **61 rust-representation; 38 source-mapped-test; 37 missing-evidence** |

No obligation in these families is one of the six executed TypeScript/Rust lifecycle scenarios, so paired_evidence is empty throughout. source-mapped-test means the TypeScript assertion and Rust assertion were reviewed together and the Rust test passed; it does not mean Vitest executed that source declaration.

## Assertion evidence and matrix joins

The source audit read all six pinned spec bodies and the relevant TypeScript context.ts, reflect.ts, service.ts, and logger.ts helpers. The Rust side was traced to exact assertions in tests/scoping.rs, tests/acceptance.rs, tests/services.rs, and tests/logger.rs.

Every non-gap obligation names both accepted command IDs:

- stable-workspace-test — exit 0, logs/stable-workspace-test.log.
- 1.85.0-workspace-test — exit 0, logs/1.85.0-workspace-test.log.

The local standard-library validator joined each cited symbol to an outcome: ok row in matrix/results.json for both commands and to the exact test <symbol> ... ok line in both logs. It also checked source/Rust line bounds, source obligation identity and variant preservation, restriction anchors, derived parent classes, meaningful class-required fields, and absence of pending-matrix. Compact result:

~~~text
OK declarations=40 variants=40 obligations=136 actual_tests=15 gaps=37 classes={'missing-evidence': 37, 'rust-representation': 61, 'source-mapped-test': 38}
~~~

The 15 distinct passing Rust tests are not a declaration count or parity percentage; individual tests cover multiple obligations.

## Concrete uncovered boundaries

These are evidence gaps, not production defects. Each JSON obligation records a concrete regression, owner core evidence maintainer, and dependency triage before conditional B2 entry.

- Association (1): G-B0-associate-235-a247 needs a second/nested ordinary-Debug forwarding assertion; the existing test proves the first bound formatting path only.
- Callable invocation (3): G-B0-invoke-5-a46, G-B0-invoke-5-a56, and G-B0-invoke-5-a59 need one fixture that composes use-site intercept state with captured and mapped callable extensions.
- Intercepts (1): G-B0-intercept-34-a37 needs the zero-layer baseline. Current assertions prove ordering, nearest declaration, isolation/own-provision omission, and custom merge.
- Isolation (28): exact gaps are source assertion lines 13, 15, 18; 78, 79, 86, 87, 91, 94, 95, 98–100, 102, 103; 125, 132, 133, 140, 141, 148, 149; and 183, 188, 192, 194, 196, 200. Existing assertions prove slot separation, same-label aliasing/no fallback, realm-filtered events, and exact retained cleanup, but do not assert all intermediate three-realm callback/disposer counters, surviving values, or the final no-error count.
- Logging (1): G-B0-logger-156-a162 needs pointer/source-identity comparison for the empty aggregate allocation. The existing test asserts its one-record/message path and separately proves exact member identity for a nonempty aggregate.
- Shadows (3): G-B0-shadow-157-a191 needs the direct second-realm value/realm pair; G-B0-shadow-313-a345 and G-B0-shadow-313-a346 need the child plugin's successful injection and no-diagnostic assertions after service-shadow stripping.

A passing targeted regression can close any item without production code. None is a confirmed failure or an unconditional B2 task; a dependent package must first demonstrate its gate and a failing public-API reproducer.

## Commands and limits

Commands actually used were read-only source/log queries plus the standard-library structural/join validator summarized above. No Cargo, Rustdoc, upstream Vitest, parity fixture, production code, test, manifest, or upstream checkout was changed or rerun. The final repository checks for this owned output were:

~~~text
python3 -B <inline scoped-fragment structural/reference/matrix validator>  # exit 0; summary above
python3 -B <docs link helper invocation for this report>                   # exit 0
git diff --check -- <six JSON fragments> <this report>                    # exit 0
~~~

Integration still owns the full 144-declaration/156-variant/six-type merge validator and independent semantic review.
