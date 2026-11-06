# First two events declarations: B0 bounded fragment

This fragment completes only `core/events.spec.ts:18` and
`core/events.spec.ts:35`: two declarations, two default variants and five
assertion obligations. The merge input is
[events-first-two.json](events-first-two.json); its IDs, source spans, variants
and assertion lines are copied from [cases.json](../ledger/cases.json). The
other seventeen events declarations remain outside this bounded task.

| Evidence class | Obligations |
|---|---:|
| rust-representation | 5 |
| source-mapped-test | 0 |
| executed-comparison | 0 |
| unsupported | 0 |
| missing-evidence | 0 |

The pinned TypeScript specification is
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`; its checkout was clean when
reviewed. The ledger inventory revision is
`9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc`, the unchanged executable
implementation revision recorded by the matrix is
`808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f`, and this mapping was prepared
from repository HEAD `8c9a191bff8201265b17eb82e7b9a6e3a59c5ad2`. The accepted matrix ran between
`578c066b8618bec9b1b2afc0a9757bb9b0f3a87e` and
`e23dbac846454d4573eee2ba3eebbc5e33085d17` with unchanged executable inputs.
Those are reused historical results, not commands rerun for this fragment.

## Assertion disposition

The line-18 declaration maps to
`symbols_named_contracts_prototype_names_and_empty_bucket_removal`. Its opaque
`Event::symbol` branch proves non-aliasing of equal descriptions, exact bail
and serial answers, invocation by emit/bail/serial/parallel and removal by
owner cleanup. The same regression's generic named-key branch records the exact
argument sequence across those four modes and proves exact manual registration
removal prevents later delivery. The split is stated in each obligation; it is
not presented as paired TypeScript execution.

The line-35 declaration maps to
`once_is_removed_before_recursive_invocation_and_manual_remove_is_exact`. It
strengthens the source's repeated-emit boundary: the first callback recursively
emits the same symbol event, a second top-level emit follows, the count remains
one, manual removal reports that the registration was already claimed, and
owner drain has no cleanup left.

All five obligations are classified as `rust-representation` because Rust uses
typed opaque `Event<A, R>` keys, `ControlFlow<R>`, started `EventTask` handles
and `ListenerRegistration` controls instead of JavaScript Symbols, raw callback
returns/promises and disposer closures. The representation and its callback
selection/removal contracts are documented in
[M2](../../../../../m2.md#event-contracts-registration-and-selection) and its
[dispatch table](../../../../../m2.md#five-dispatch-modes-with-honest-callback-shapes).
No exclusion or new gap is needed for these two declarations.

Two current Rust test names substantiate the fragment. Both occur with
`outcome: ok` in [matrix results](../matrix/results.json) for
`stable-workspace-test` and `1.85.0-workspace-test`, and both original logs
contain their exact `test ... ok` lines: [stable](../matrix/logs/stable-workspace-test.log)
and [Rust 1.85.0](../matrix/logs/1.85.0-workspace-test.log). The fragment claims
zero executed comparisons: the accepted six paired lifecycle scenarios do not
execute either upstream declaration.

## Commands, validation and limits

Source inspection used the integration checkout's CodeGraph index, its pinned
`local/cordis` checkout, and `codegraph node` for the two exact Rust test
bodies. Read-only Git/Python/ripgrep checks examined revisions, inventory,
matrix rows and preserved logs. No Cargo, Rustdoc, documentation, TypeScript or
parity command was run. No production source, tests, matrix artifacts, ledger
integration, progress file or other family fragment was changed.

The final standard-library validator checks exactly two unique expected
declarations; the five original obligation IDs and source lines; valid class
fields; existing line/anchor/link targets; assertion tokens at the cited Rust
lines; the pinned clean TypeScript checkout; exact successful test rows and log
joins for both toolchains; absence of pending/paired claims; and exclusive
ownership of these two output paths. `git diff --cached --check` separately
checks the staged patch. The following exact standard-library command was run
from the isolated worktree root. Result: exit 0.

~~~sh
python3 -B - <<'PY'
import json, re, subprocess
from collections import Counter
from pathlib import Path

work = Path.cwd()
integration = Path('/Users/I756434/Projects/private/cordis-rust')
base = work / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
path = base / 'ledger-inputs/events-first-two.json'
report = path.with_name('report-events-first-two.md')
data = json.loads(path.read_text())
source = json.loads((base / 'ledger/cases.json').read_text())
ids = ['core/events.spec.ts:18', 'core/events.spec.ts:35']
obligation_ids = {
    'core/events.spec.ts:18/default/a25',
    'core/events.spec.ts:18/default/a26',
    'core/events.spec.ts:18/default/a28',
    'core/events.spec.ts:18/default/a32',
    'core/events.spec.ts:35/default/a43',
}
assert [item['id'] for item in data['declarations']] == ids
assert len({item['id'] for item in data['declarations']}) == 2
original = {item['id']: item for item in source['declarations']}
identity = ('id', 'source_file', 'source_line', 'source_end_line',
            'title', 'suite', 'historical_checklist')
for item in data['declarations']:
    assert all(item[key] == original[item['id']][key] for key in identity)
    assert [variant['parameters'] for variant in item['variants']] == [{}]
    assert item['classes'] == ['rust-representation']
obligations = [
    obligation
    for item in data['declarations']
    for variant in item['variants']
    for obligation in variant['obligations']
]
original_obligations = {
    obligation['id']: obligation
    for item_id in ids
    for variant in original[item_id]['variants']
    for obligation in variant['obligations']
}
assert len(obligations) == len({item['id'] for item in obligations}) == 5
assert {item['id'] for item in obligations} == obligation_ids
for item in obligations:
    assert item['source_assertion_lines'] == original_obligations[item['id']]['source_assertion_lines']
    assert item['class'] == 'rust-representation'
    assert item['rust_evidence'] and item['restriction']
    assert item['gap'] is None and item['paired_evidence'] == []
    assert item['execution_status'] == 'passed'
    assert {command for evidence in item['rust_evidence']
            for command in evidence['command_ids']} == {
                'stable-workspace-test', '1.85.0-workspace-test'}
assert 'pending-matrix' not in path.read_text()

ts_head = subprocess.check_output([
    'git', '-C', str(integration / 'local/cordis'), 'rev-parse', 'HEAD'
], text=True).strip()
assert ts_head == data['typescript_revision'] == '35fdc969e4b6c48c48ed44717c7a018324e4dd88'
assert not subprocess.check_output([
    'git', '-C', str(integration / 'local/cordis'), 'status',
    '--porcelain', '--untracked-files=no'
], text=True).strip()
ts_lines = (integration / 'local/cordis/packages/core/tests/events.spec.ts').read_text().splitlines()
for line, token in {
    25: 'root.bail(event, 2)', 26: 'root.serial(event, 3)',
    28: '[1, 2, 3, 4]', 32: 'to.have.length(4)',
    43: 'to.have.length(1)',
}.items():
    assert token in ts_lines[line - 1]

rust_lines = (work / 'crates/re-cordis-core/tests/events.rs').read_text().splitlines()
checks = {
    25: [(116, 'ctx.bail(&symbol.clone(), 2'), (117, 'ControlFlow::Break(2)')],
    26: [(128, 'ctx.serial(&symbol, 5'), (132, 'ControlFlow::Break(5)')],
    28: [(70, '[1, 0, 2, 3]'), (110, 'ctx.emit(&other, 1'),
         (114, 'calls.get(), 0'), (134, 'calls.get(), 4')],
    32: [(78, 'registration.remove().unwrap()'), (83, '[1, 0, 2, 3]'),
         (135, 'ctx.drain()'), (140, 'calls.get(), 4')],
    43: [(162, 'ctx.emit(&event'), (168, 'ctx.emit(&event'),
         (172, 'ctx.emit(&event'), (176, 'calls.get(), 1'),
         (177, '!registration.remove()'), (178, 'attempted(), 0')],
}
for entries in checks.values():
    for line, token in entries:
        assert token in rust_lines[line - 1], (line, token)

def anchors(file):
    return {
        re.sub(r'[^a-z0-9 -]', '', line.lstrip('#').strip().lower()).replace(' ', '-')
        for line in file.read_text().splitlines() if line.startswith('#')
    }

for item in obligations:
    document = work / item['restriction']['document']
    assert document.is_file()
    assert item['restriction']['anchor'] in anchors(document)
    evidence = item['rust_evidence'][0]
    lines = (work / evidence['file']).read_text().splitlines()
    assert all(1 <= line <= len(lines) for line in evidence['lines'])
    assert any(re.search(rf'fn\s+{re.escape(evidence["symbol"])}\s*\(', line)
               for line in lines)
for link in re.findall(r'\[[^]]+\]\(([^)]+)\)', report.read_text()):
    if '://' in link:
        continue
    raw, _, anchor = link.partition('#')
    target = (report.parent / raw).resolve()
    assert target.is_file(), link
    if anchor:
        assert anchor in anchors(target), link

results = json.loads((base / 'matrix/results.json').read_text())
tests = {
    'symbols_named_contracts_prototype_names_and_empty_bucket_removal',
    'once_is_removed_before_recursive_invocation_and_manual_remove_is_exact',
}
commands = {'stable-workspace-test', '1.85.0-workspace-test'}
rows = [item for item in results['tests']
        if item['test'] in tests and item['command_id'] in commands]
assert len(rows) == 4
assert {(item['command_id'], item['test']) for item in rows} == {
    (command, test) for command in commands for test in tests}
assert all(item['crate'] == 're-cordis-core'
           and item['suite'] == 'tests/events.rs'
           and item['outcome'] == 'ok' and not item['compile_fail']
           for item in rows)
for command in commands:
    log = (base / f'matrix/logs/{command}.log').read_text()
    assert all(f'test {test} ... ok' in log for test in tests)

owned = {
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/events-first-two.json',
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/report-events-first-two.md',
}
staged = set(subprocess.check_output([
    'git', 'diff', '--cached', '--name-only'
], text=True).splitlines())
assert staged == owned
assert not subprocess.check_output([
    'git', 'ls-files', '--others', '--exclude-standard'
], text=True).strip()
print('PASS: 2 unique declarations, 2 default variants, 5 preserved obligations; classes=' + str(dict(Counter(item['class'] for item in obligations))))
print('PASS: exact TS/Rust lines, M2 anchors and report links resolve; TS pin is clean at ' + ts_head[:12])
print('PASS: 2 exact Rust tests join to 4 ok matrix rows and both original workspace logs; paired claims=0, gaps=0')
print('PASS: staged paths are exactly the 2 owned events-first-two outputs')
PY
git diff --cached --check
~~~

~~~text
PASS: 2 unique declarations, 2 default variants, 5 preserved obligations; classes={'rust-representation': 5}
PASS: exact TS/Rust lines, M2 anchors and report links resolve; TS pin is clean at 35fdc969e4b6
PASS: 2 exact Rust tests join to 4 ok matrix rows and both original workspace logs; paired claims=0, gaps=0
PASS: staged paths are exactly the 2 owned events-first-two outputs
~~~

`git diff --cached --check` also exited 0 with no output. These structural and
reference checks establish the recorded joins; they do not turn mapped Rust
representations into executed TypeScript comparisons or broaden the accepted
six-scenario parity boundary. Independent review and ledger integration remain
with the orchestrator.
