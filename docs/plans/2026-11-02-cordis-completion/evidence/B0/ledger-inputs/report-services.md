# Services: B0 family reconciliation

This bounded fragment completes the eight declarations in `service.spec.ts` and
the seventeen declarations in `service-dispose.spec.ts`: **25 declarations,
35 registered variants, and 81 meaningful assertion obligations**. The
merge inputs are [service.json](service.json) and
[service-dispose.json](service-dispose.json); declaration identity, source spans,
suite/checklist provenance, exact parameters, and applicable original obligation
IDs/assertion lines come from [cases.json](../ledger/cases.json). Source line 125
of the five-way disposal case is guarded by `action === 'restart'`, so that
obligation remains only on the restart variant rather than being duplicated
onto four unreachable branches.

| Evidence class | Obligations |
|---|---:|
| source-mapped-test | 61 |
| rust-representation | 15 |
| executed-comparison | 0 |
| unsupported | 0 |
| missing-evidence | 5 |

The pinned TypeScript revision is
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`; the ledger inventory revision is
`9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc`; and the matrix's unchanged
implementation revision is `808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f`.
This mapping was prepared from worktree HEAD
`55f0fef7de9f7d742af02ddbe03d712a9837ab27`. Accepted matrix execution is reused
from its committed logs; no Cargo, TypeScript, documentation, or parity command
was rerun.

## Findings

Service initialization and traceability use the approved explicit Rust
representations: `Plugin::initialized`, typed dependency lists, `ServiceKey<T>`,
and `BoundService` callbacks carrying the traced Context. Exact availability,
three-call counter sequence, no-diagnostic outcome, repeated initialized-service
baselines, multi-key activation, and shared isolation behavior are linked to
[M6 initialization](../../../../../m6.md#initialization-and-method-injection),
[M5 bound calls](../../../../../m5.md#scope-and-bound-calls), and their named
current assertions. These are representation mappings, not transparent
Service/Proxy/class compatibility.

Service disposal has direct source-mapped evidence for removed Registry
consumers, independent publications, all five cleanup/restart actions,
transitive/in-flight/failed cleanup, provider listeners, unrelated concurrency,
composed effects, inherited and explicit child dependencies, both realm
shutdown variants, replacement races, both current/withdrawn release orders,
and equal/same-Rc publication replacement. Gates and exact traces establish the
partial orders; test-name similarity was not used as evidence. The cleanup-error
identity obligation combines the service-specific cleanup regression with the
generic current-core assertion that downcasts the retained original error source.

The accepted six paired lifecycle scenarios are not claimed for this fragment.
Their nearby removed/coalesced behaviors do not execute these upstream
declarations assertion-by-assertion, so all `paired_evidence` arrays remain empty.

## Concrete gap

Five fine-grained obligations remain `missing-evidence`. Source line 33 has no
consumer-apply counter joining the asynchronous initialization gate to exactly
one activation. Source lines 161-163 have no current three-plugin initialized
dependency-DAG regression counting Foo, Bar, and Qux once each. Source line 214
has no apply counter proving restart changes the consumer count from one to two.
Each JSON gap names the exact regression addition. Passing results close these
evidence gaps without production change; conditional B2 still requires a failing
public-API reproducer and an actual dependent gate.

## Matrix joins, commands, and limits

The fragment cites **19 distinct current Rust tests**. Every cited test
has an `outcome: ok` row in [matrix results](../matrix/results.json) for both
`stable-workspace-test` and `1.85.0-workspace-test`, and both original logs
contain its exact `test ... ok` line: [stable](../matrix/logs/stable-workspace-test.log)
and [Rust 1.85.0](../matrix/logs/1.85.0-workspace-test.log).

The final standard-library validation command checks exact declaration counts,
variant parameters, applicable obligation provenance, derived classes, source
and Rust line bounds/symbols, restriction anchors, clean TS pin, both result/log
joins, resolved Markdown links, absence of pending/paired overclaims, and owned
file scope. It is recorded and run from the worktree as:

~~~sh
python3 -B .tmp_validate_services.py
git diff --cached --check
~~~

Result: `python3 -B .tmp_validate_services.py` exited 0.

~~~text
PASS: 25 declarations, 35 variants, 81 obligations; classes={'rust-representation': 15, 'missing-evidence': 5, 'source-mapped-test': 61}
PASS: TS pin clean at 35fdc969e4b6; exact source/Rust bounds, 3 restriction anchors and report links resolve
PASS: 19 exact Rust tests join to 38 ok matrix rows and both original workspace logs; paired claims=0, gaps=5
PASS: untracked non-temporary paths are exactly the 3 owned service outputs
~~~

`git diff --cached --check` exited 0 with no output after the temporary scripts were removed and the
owned outputs are staged. Structural/reference validation establishes the recorded joins; it
does not execute TypeScript or prove transparent mechanism parity. Independent
review and final ledger integration remain with the integration owner.
