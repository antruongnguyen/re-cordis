# Lifecycle/update B0 evidence reconciliation

This report covers only the assigned lifecycle families:
[plugin.json](plugin.json), [fiber.json](fiber.json), and
[update.json](update.json). It does not modify the integrated
[cases inventory](../ledger/cases.json), matrix artifacts, production source,
tests, manifests, progress tracking, or any other family output.

| Family | Declarations | Registered variants | Obligations |
|---|---:|---:|---:|
| plugin.spec.ts | 10 | 10 | 46 |
| fiber.spec.ts | 22 | 22 | 88 |
| update.spec.ts | 10 | 12 | 54 |
| **Total** | **42** | **44** | **188** |

| Evidence class | Obligations |
|---|---:|
| executed-comparison | 23 |
| source-mapped-test | 96 |
| rust-representation | 49 |
| missing-evidence | 20 |
| unsupported | 0 |

The pinned TypeScript specification is Cordis at
[35fdc969e4b6c48c48ed44717c7a018324e4dd88](https://github.com/cordiverse/cordis/tree/35fdc969e4b6c48c48ed44717c7a018324e4dd88).
The ledger control revision is
9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc and the accepted matrix records
unchanged executable implementation revision
808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f. This worktree started from
integration revision 11d84b185ea0d9e9fabe948c5e23415dd216ee7d.
The three family commits are 755a1a7 (plugin), 9cc0a64 (fiber), and 1626c54
(update). The matrix ran earlier; no Cargo, Rustdoc, TypeScript, documentation,
or parity command was rerun for this reconciliation.

## Carried evidence and uncovered delta

The completed M3-M6 decisions remain authoritative. Typed Plugin declarations,
opaque/stable Fiber identity, explicit lifecycle tasks, synchronous typed
validation, source/resolved configuration separation, local update middleware,
exact provider bindings, explicit effect streams, and typed status hooks are
not reopened. The deliberate Rust representations and restrictions are linked
per obligation to [M3](../../../../../m3.md),
[M5](../../../../../m5.md), or [M6](../../../../../m6.md).

The accepted matrix supplies exact passing rows and original log lines for all
31 distinct current Rust test/Rustdoc identities cited here on both
stable-workspace-test and 1.85.0-workspace-test. The joins come from
[matrix results](../matrix/results.json), the
[stable log](../matrix/logs/stable-workspace-test.log), and the
[Rust 1.85.0 log](../matrix/logs/1.85.0-workspace-test.log). They are reused
historical results, not reruns.

The uncovered delta was assertion-level provenance. The reconciliation reads
all in-body branches and preserves both registered update parameter sets:
inject=false/true and mode=restart/config ABA. It does not infer a pass from a
test name, a historical checkmark, successful compilation alone, or a nearby
state assertion.

## Family findings

Plugin obligations map typed functional/object adapters, inactive-owner
admission, Context naming, Registry snapshots, nested parent-owned cleanup,
repeated snapshot restoration, reusable-root drain, and explicit initialization.
Runtime rejection of malformed JavaScript plugin shapes is represented by the
Plugin compile-fail Rustdoc contract at the exact item/line on both toolchains.
Three Service.init start/pre-cleanup count boundaries remain unasserted; the
final disposer side effect is mapped.

Fiber obligations map all three gated inertia cases, failure rollback,
Failed-refresh inertia, explicit recovery, failed/dropped update delivery,
cleanup-error isolation, typed status sequences, update/restart handles,
validation, resolved/source separation, and accepted-update retention. The
explicit-recovery, latest-failure, and combined paired scenarios are used only
for the exact obligations they exercise. JavaScript uid/inertia and wrapper
own/prototype checks are recorded as typed FiberHandle representations, not
mechanism parity.

Update obligations map all 12 registered variants. Existing Rust tests directly
cover latest-target coalescing, unload gating, obsolete/latest failure, owned
stream final-yield cleanup, dependency-deferred validation, accepted update
middleware, and disposal winning over queued config. The superseded-failure and
latest-failure paired scenarios are attached only to their corresponding
failure/rollback/waiter obligations. The accepted six-scenario runner does not
execute the other update declarations.

## Concrete gaps and disposition

There are 20 explicit gaps: 17 missing exact assertions and three observed
inventory/source gaps. None is a confirmed production failure, none reopens an
M0-M6 design decision, and none by itself enters B2. Each JSON gap names the
exact regression or inventory action, the core evidence maintainer, and the
conditional dependency gate requiring a failing public-API reproducer before
B2.

| Group | Obligation IDs | Required next action |
|---|---|---|
| Inventory false positives / inactive branch (3) | plugin :77/default/a84; fiber :71/default/a86; update :7/false/a21 | Preserve original provenance, then correct the extractor/control for a closing brace, commented assertion, and inject=false branch. No production regression. |
| Explicit initialization counts (3) | plugin :165/default/a178, a179, a181 | Add initializer invocation and pre-disposal cleanup counters to the existing explicit-initialization regression. |
| Fiber exact boundaries (3) | fiber :71/default/a90; :144/default/a153; :292/default/a308 | Add healthy-sibling post-failure dispatch, post-failed-recovery state, and third sequential apply-count assertions. |
| Concurrent-update dependency/waiters (3) | update :7/true/a21; :7/false/a42; :7/true/a42 | Assert injected service value and poll all three waiters while the latest load is gated in both variants. |
| Unload and ABA terminal state/source (5) | update :69/default/a92; :96/restart/a116, a117; :96/config aba/a116, a117 | Assert final Active state and retained original source after each forced unchanged-source reload. |
| Pending dependency and accepted loading update (3) | update :212/default/a225, a230; :234/default/a255 | Assert no pending apply before provider arrival and explicit final Active states after readiness. |

## Executed-comparison limits

Twenty-three obligations use bounded executed-comparison evidence across four
of the six accepted scenarios: combined, superseded-failure, latest-failure,
and explicit-recovery. Their paired evidence identifies exact retained trace
observations and verifies the corresponding line in the
[parity log](../matrix/logs/parity.log). Coalesced and removed remain accepted
matrix scenarios but are not attached here where a finer declaration assertion
did not match exactly. Source-mapped tests do not execute TypeScript. Rust
representations are not JavaScript mechanism compatibility. No percentage is
reported.

## Exact validation commands and results

The following exact standard-library command was run from the isolated
worktree root after staging this report. It validates identity/variant/source
provenance, derived classes, gap ownership, source/Rust bounds, current test
symbols, Rustdoc item/line/kind joins, both original workspace result/log joins,
paired observations, TypeScript pin/cleanliness, and exclusive owned paths.

~~~sh
python3 -B - <<'PY'
import importlib.util
import json
from collections import Counter
from pathlib import Path
import re
import subprocess

root = Path.cwd()
integration = Path('/Users/I756434/Projects/private/cordis-rust')
base = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
inputs = base / 'ledger-inputs'
family_files = ('plugin.json', 'fiber.json', 'update.json')
expected_counts = {
    'plugin.spec.ts': (10, 10, 46),
    'fiber.spec.ts': (22, 22, 88),
    'update.spec.ts': (10, 12, 54),
}
required_commands = {'stable-workspace-test', '1.85.0-workspace-test'}
valid_classes = {
    'executed-comparison', 'source-mapped-test', 'rust-representation',
    'unsupported', 'missing-evidence',
}
bad_text = ('initial inventory', 'placeholder', 'pending-matrix', 'in progress', 'under review')

source = json.loads((base / 'ledger/cases.json').read_text())
source_by_id = {item['id']: item for item in source['declarations']}
ledgers = [json.loads((inputs / name).read_text()) for name in family_files]
results_doc = json.loads((base / 'matrix/results.json').read_text())
results = results_doc['tests']
commands = {item['id']: item for item in json.loads((base / 'matrix/commands.json').read_text())}
logs = {
    command: (base / 'matrix' / commands[command]['log']).read_text()
    for command in required_commands | {'parity'}
}

spec = importlib.util.spec_from_file_location('cordis_docs_check', root / 'tests/docs/check.py')
docs = importlib.util.module_from_spec(spec)
spec.loader.exec_module(docs)

def resolved(path_text):
    return ((integration if path_text.startswith('local/cordis/') else root) / path_text).resolve()

def final_text(value):
    return isinstance(value, str) and value.strip() and not any(token in value.casefold() for token in bad_text)

def rustdoc_match(row, evidence):
    prefix = f"{evidence['file']} - {evidence['symbol']} (line {evidence['rustdoc_line']})"
    return (row['suite'] == 'rustdoc' and row['test'].startswith(prefix)
            and row['compile_fail'] is evidence['compile_fail'])

declarations = []
for ledger in ledgers:
    assert ledger['schema_version'] == 1
    assert ledger['rust_revision'] == source['rust_revision'] == '9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc'
    assert ledger['implementation_revision'] == source['implementation_revision'] == '808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f'
    assert ledger['typescript_revision'] == source['typescript_revision'] == '35fdc969e4b6c48c48ed44717c7a018324e4dd88'
    assert final_text(ledger['status'])
    declarations.extend(ledger['declarations'])

assert len(declarations) == 42
assert len({item['id'] for item in declarations}) == 42
family_counter = Counter(Path(item['source_file']).name for item in declarations)
assert family_counter == Counter({name: counts[0] for name, counts in expected_counts.items()})

obligations = []
gap_ids = set()
rust_identities = set()
paired_scenarios = set()
for declaration in declarations:
    original = source_by_id[declaration['id']]
    for field in ('source_file', 'source_line', 'source_end_line', 'title', 'suite', 'historical_checklist'):
        assert declaration[field] == original[field]
    source_path = resolved(declaration['source_file'])
    source_lines = source_path.read_text().splitlines()
    assert 1 <= declaration['source_line'] <= declaration['source_end_line'] <= len(source_lines)
    assert [variant['parameters'] for variant in declaration['variants']] == [
        variant['parameters'] for variant in original['variants']
    ]
    retained = {
        item['id']: item['source_assertion_lines']
        for variant in declaration['variants'] for item in variant['obligations']
    }
    baseline = {
        item['id']: item['source_assertion_lines']
        for variant in original['variants'] for item in variant['obligations']
    }
    assert all(retained[item_id] == lines for item_id, lines in baseline.items())
    derived = sorted({
        item['class'] for variant in declaration['variants'] for item in variant['obligations']
    })
    assert declaration['classes'] == derived
    obligations.extend(item for variant in declaration['variants'] for item in variant['obligations'])

assert sum(len(item['variants']) for item in declarations) == 44
assert len(obligations) == len({item['id'] for item in obligations}) == 188
for obligation in obligations:
    assert obligation['class'] in valid_classes
    assert final_text(obligation['behavior']) and final_text(obligation['rationale'])
    assert obligation['execution_status'] != 'pending-matrix'
    if obligation['class'] == 'missing-evidence':
        assert obligation['execution_status'] in {'not-executed', 'failed'}
        assert obligation['gap'] and not obligation['rust_evidence'] and not obligation['paired_evidence']
        gap = obligation['gap']
        assert gap['id'] not in gap_ids
        gap_ids.add(gap['id'])
        assert gap['kind'] in {'missing-evidence', 'observed-source-gap', 'investigation', 'confirmed-failure'}
        assert all(final_text(gap[field]) for field in ('id', 'next_action', 'owner', 'work_package'))
        work = gap['work_package'].casefold()
        assert all(token in work for token in ('conditional b2', 'depend', 'failing public-api reproducer'))
    else:
        assert obligation['execution_status'] == 'passed'
        assert obligation['rust_evidence'] and obligation['gap'] is None
    if obligation['class'] == 'rust-representation':
        restriction = obligation['restriction']
        assert restriction and final_text(restriction['reason'])
        document = resolved(restriction['document'])
        assert restriction['anchor'] in docs.anchors(document.read_text())
    if obligation['class'] == 'executed-comparison':
        assert obligation['paired_evidence']
    else:
        assert not obligation['paired_evidence']

    for evidence in obligation['rust_evidence']:
        path = resolved(evidence['file'])
        lines = path.read_text().splitlines()
        assert evidence['file'].startswith('crates/re-cordis-core/')
        assert all(1 <= line <= len(lines) for line in evidence['lines'])
        if '/tests/' in evidence['file']:
            assert re.search(rf"\b(?:async\s+)?fn\s+{re.escape(evidence['symbol'])}\b", path.read_text())
        for command in required_commands:
            rows = [row for row in results if row['command_id'] == command and row['outcome'] == 'ok']
            if '/tests/' in evidence['file']:
                suite = evidence['file'].split('crates/re-cordis-core/', 1)[1]
                matches = [row for row in rows if row['suite'] == suite and row['test'] == evidence['symbol']]
            else:
                matches = [row for row in rows if rustdoc_match(row, evidence)]
            assert len(matches) == 1, (obligation['id'], command, evidence)
            assert f"test {matches[0]['test']} ... ok" in logs[command]
        rust_identities.add((evidence['file'], evidence['symbol'], evidence.get('compile_fail', False)))

    for evidence in obligation['paired_evidence']:
        scenario = evidence['scenario']
        assert scenario in results_doc['paired_comparisons']
        trace = (root / f'tests/parity/{scenario}.txt').read_text()
        assert evidence['observation'] in trace
        count = results_doc['paired_comparisons'][scenario]
        assert f'{scenario}: Rust == TypeScript ({count} observations)' in logs['parity']
        paired_scenarios.add(scenario)

classes = Counter(item['class'] for item in obligations)
assert classes == Counter({
    'executed-comparison': 23,
    'source-mapped-test': 96,
    'rust-representation': 49,
    'missing-evidence': 20,
})
assert Counter(item['gap']['kind'] for item in obligations if item['gap']) == Counter({
    'missing-evidence': 17,
    'observed-source-gap': 3,
})
assert len(rust_identities) == 31
assert paired_scenarios == {'combined', 'explicit-recovery', 'latest-failure', 'superseded-failure'}

upstream = integration / 'local/cordis'
assert subprocess.check_output(['git', '-C', str(upstream), 'rev-parse', 'HEAD'], text=True).strip() == source['typescript_revision']
assert not subprocess.check_output(['git', '-C', str(upstream), 'status', '--porcelain'], text=True).strip()

owned = {
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/plugin.json',
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/fiber.json',
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/update.json',
    'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/report-lifecycle-update.md',
}
history = set(subprocess.check_output([
    'git', 'diff', '--name-only', '11d84b185ea0d9e9fabe948c5e23415dd216ee7d..HEAD'
], text=True).splitlines())
staged = set(subprocess.check_output(['git', 'diff', '--cached', '--name-only'], text=True).splitlines())
assert history | staged == owned
assert not (history | staged) - owned

print("PASS: 42 declarations, 44 variants, 188 obligations; classes=" + str(dict(sorted(classes.items()))))
print("PASS: 31 Rust identities join exact ok results/log lines on stable and 1.85.0; Rustdoc item/line/kind joins included")
print("PASS: 23 exact executed-comparison obligations use four accepted scenarios; no other paired claims")
print("PASS: 20 owned gaps (17 missing-evidence, 3 observed-source-gap); TS pin clean at 35fdc969e4b6")
print("PASS: committed-plus-staged paths are exactly the four owned lifecycle/update outputs")
PY
~~~

~~~text
PASS: 42 declarations, 44 variants, 188 obligations; classes={'executed-comparison': 23, 'missing-evidence': 20, 'rust-representation': 49, 'source-mapped-test': 96}
PASS: 31 Rust identities join exact ok results/log lines on stable and 1.85.0; Rustdoc item/line/kind joins included
PASS: 23 exact executed-comparison obligations use four accepted scenarios; no other paired claims
PASS: 20 owned gaps (17 missing-evidence, 3 observed-source-gap); TS pin clean at 35fdc969e4b6
PASS: committed-plus-staged paths are exactly the four owned lifecycle/update outputs
~~~

The report/link check below invokes the repository documentation helper's
anchor parser directly. It was run from the same root after report staging.

~~~sh
python3 -B - <<'PY'
import importlib.util
from pathlib import Path
import re

root = Path.cwd()
report = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/report-lifecycle-update.md'
spec = importlib.util.spec_from_file_location('cordis_docs_check', root / 'tests/docs/check.py')
docs = importlib.util.module_from_spec(spec)
spec.loader.exec_module(docs)
checked = 0
anchored = 0
for link in re.findall(r'\[[^]]+\]\(([^)]+)\)', report.read_text()):
    if '://' in link:
        continue
    raw, _, anchor = link.partition('#')
    target = (report.parent / raw).resolve()
    assert target.is_file(), link
    checked += 1
    if anchor:
        assert anchor in docs.anchors(target.read_text()), link
        anchored += 1
print(f'PASS: docs helper resolved {checked} local report links, including {anchored} anchored targets')
PY
git diff --cached --check
~~~

The docs-helper command exited 0 and printed the result recorded below.

~~~text
PASS: docs helper resolved 11 local report links, including 0 anchored targets
~~~

# git diff --cached --check exited 0 with no output.

These checks establish accounting, concrete references, and preserved execution
joins. They do not rerun implementation tests, execute additional TypeScript,
or convert source mappings/representations into paired parity.
