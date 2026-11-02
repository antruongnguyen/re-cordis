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
python3 -B - <<'PY'
import importlib.util, json, re, subprocess
from collections import Counter
from pathlib import Path

root = Path.cwd()
integration = Path('/Users/I756434/Projects/private/cordis-rust')
base = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
inputs = [json.loads((base / f'ledger-inputs/{name}').read_text())
          for name in ['service.json', 'service-dispose.json']]
inventory = json.loads((base / 'ledger/cases.json').read_text())
original = {item['id']: item for item in inventory['declarations']}
declarations = [item for data in inputs for item in data['declarations']]
obligations = [item for declaration in declarations
               for variant in declaration['variants']
               for item in variant['obligations']]
assert len(declarations) == len({item['id'] for item in declarations}) == 25
assert sum(len(item['variants']) for item in declarations) == 35
assert len(obligations) == len({item['id'] for item in obligations}) == 81
assert Counter(item['class'] for item in obligations) == {
    'source-mapped-test': 61, 'rust-representation': 15, 'missing-evidence': 5}
identity = ('id', 'source_file', 'source_line', 'source_end_line',
            'title', 'suite', 'historical_checklist')
for declaration in declarations:
    assert all(declaration[key] == original[declaration['id']][key]
               for key in identity)
    derived = sorted({item['class'] for variant in declaration['variants']
                      for item in variant['obligations']})
    assert declaration['classes'] == derived
for item in obligations:
    assert item['behavior'] and not item['behavior'].startswith('expect(')
    assert item['paired_evidence'] == []
    if item['class'] == 'missing-evidence':
        assert not item['rust_evidence'] and item['gap']
        assert item['execution_status'] == 'not-executed'
    else:
        assert item['rust_evidence'] and item['gap'] is None
        assert item['execution_status'] == 'passed'
        if item['class'] == 'rust-representation':
            assert item['restriction']

ts_head = subprocess.check_output([
    'git', '-C', str(integration / 'local/cordis'), 'rev-parse', 'HEAD'
], text=True).strip()
assert ts_head == '35fdc969e4b6c48c48ed44717c7a018324e4dd88'
assert not subprocess.check_output([
    'git', '-C', str(integration / 'local/cordis'), 'status',
    '--porcelain', '--untracked-files=no'
], text=True).strip()
symbols = set()
for declaration in declarations:
    source = (integration / declaration['source_file']).read_text().splitlines()
    for variant in declaration['variants']:
        for item in variant['obligations']:
            assert all(declaration['source_line'] <= line <= declaration['source_end_line']
                       for line in item['source_assertion_lines'])
            for evidence in item['rust_evidence']:
                rust = (root / evidence['file']).read_text().splitlines()
                assert all(1 <= line <= len(rust) for line in evidence['lines'])
                assert sum(bool(re.search(
                    rf'fn\s+{re.escape(evidence["symbol"])}\s*\(', line))
                    for line in rust) == 1
                symbols.add((evidence['file'], evidence['symbol']))

results = json.loads((base / 'matrix/results.json').read_text())
commands = {'stable-workspace-test', '1.85.0-workspace-test'}
for file, symbol in symbols:
    suite = file.removeprefix('crates/re-cordis-core/')
    rows = [item for item in results['tests']
            if item['test'] == symbol and item['command_id'] in commands]
    assert {(item['command_id'], item['test']) for item in rows} == {
        (command, symbol) for command in commands}
    assert all(item['suite'] == suite and item['outcome'] == 'ok'
               and not item['compile_fail'] for item in rows)
    for command in commands:
        log = (base / f'matrix/logs/{command}.log').read_text()
        assert f'test {symbol} ... ok' in log

spec = importlib.util.spec_from_file_location('docs_check', root / 'tests/docs/check.py')
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
module.links(base / 'ledger-inputs/report-services.md')
owned = {
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/service.json',
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/service-dispose.json',
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/report-services.md',
}
assert set(subprocess.check_output(
    ['git', 'diff', '--name-only', 'HEAD^'], text=True).splitlines()) == owned
print('PASS: 25 declarations, 35 variants, 81 obligations; classes=' +
      str(dict(Counter(item['class'] for item in obligations))))
print('PASS: TS pin clean at ' + ts_head[:12] +
      '; exact source/Rust bounds, restriction anchors and report links resolve')
print(f'PASS: {len(symbols)} exact Rust tests join to {len(symbols) * 2} ok matrix rows and both original workspace logs; paired claims=0, gaps=5')
print('PASS: commit-to-parent paths are exactly the 3 owned service outputs')
PY
git diff HEAD^ --check
~~~

Result: the Python command and `git diff HEAD^ --check` both exited 0.

~~~text
PASS: 25 declarations, 35 variants, 81 obligations; classes={'rust-representation': 15, 'missing-evidence': 5, 'source-mapped-test': 61}
PASS: TS pin clean at 35fdc969e4b6; exact source/Rust bounds, 3 restriction anchors and report links resolve
PASS: 19 exact Rust tests join to 38 ok matrix rows and both original workspace logs; paired claims=0, gaps=5
PASS: commit-to-parent paths are exactly the 3 owned service outputs
~~~

`git diff HEAD^ --check` had no output. Structural/reference validation establishes the recorded joins; it
does not execute TypeScript or prove transparent mechanism parity. Independent
review and final ledger integration remain with the integration owner.
