# Services: B0 SR-1–SR-5 evidence corrections

The two merge inputs, [service.json](service.json) and
[service-dispose.json](service-dispose.json), retain **25 declarations
(8 service / 17 service-dispose), 35 registered variants (8 / 27), and 81
original-ID behavioral obligation records**. The original 85 inventory records
reconcile as these 81 obligations plus four guarded non-obligations. No
original declaration, variant or applicable assertion ID/line was removed.

| Evidence class | Obligations |
|---|---:|
| source-mapped-test | 55 |
| rust-representation | 11 |
| missing-evidence | 15 |
| executed-comparison | 0 |
| unsupported | 0 |

This corrects the service inputs/report at `d07d3eec2df3979d5413a0ed7477feebe70efe4c`
(integrated at `7fd7cde471a22dd20e0f015094de2b12b684b032`) in response to the
[initial review](../services-review.md). Correction worktree:
`/Users/I756434/.codex/worktrees/b0-services-corrections/cordis-rust`, based on
`c462dfdf732bf2ed056eefb7832654efd9629e3a`. The fixed TypeScript specification is
`35fdc969e4b6c48c48ed44717c7a018324e4dd88` in the integration checkout's
`local/cordis`; the inventory retains its recorded Rust revision
`9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc`. Executable inputs remain unchanged
from `808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f`.

The [reuse rule](../../../README.md#prior-decisions-and-evidence-reuse),
[historical decisions](../../../../../decision.md),
[limitations](../../../../../limitation.md), and
[completed plan](../../../../../implementation-plan.md) carry forward M4
publication/committed-load retention, M5 explicit bound views and M6 initialized
factories. The accepted matrix/report, FR-1, first-two events and integrated
validation remain unchanged. No Cargo, TS, documentation-example or parity
suite was rerun; the six paired scenarios/73 observations receive no new claim.

## SR-1: Counter helper, getter sites and root effect lifetime

At the pin, `service.spec.ts:48–49,86–87` delegates to `utils.ts:62–66`:
Counter acquisition increments its value and registers cleanup to decrement it.
Every increase in these two scenarios is called through `root.foo`, including
inside the consumer callback. The middle getter differs: source line 66 reads
`ctx.foo.value` through the consumer; line 101 reads `root.foo.value`.

[acceptance.rs](../../../../../../crates/re-cordis-core/tests/acceptance.rs):622–667
uses direct Cell mutation at 654–655, performs three root-bound calls, disposes
the consumer at 664 and asserts root count 3/no failures at 666–667. It supplies
the final-count and approved checked-access/no-failure substitute, not the
reversible Counter helper or intermediate getters. The joined
[scoping.rs](../../../../../../crates/re-cordis-core/tests/scoping.rs):188–200
registers a bound cleanup that survives definition-relay disposal and runs on
root drain. That separately establishes root-use-site effect ownership.

The final-count records `:36/a72` and `:76/a106` retain this bounded Rust
representation join. The warning records retain the approved whole-sequence
no-failure substitute. Four precise getter/effect gaps retain original IDs:
`:36/a61,a66` and `:76/a97,a101`. Their partial evidence is kept, but their
complete source obligations have `execution_status: not-executed`. Each action
names reversible Counter acquisition/cleanup, both injection declarations,
the actual getter site, consumer disposal and final root count.

## SR-2: Composed-provider shutdown variants

`service-dispose.spec.ts:261` selects provider disposal for `ownerShutdown=false`
and non-root outer-owner disposal for `true`; both siblings are created under
that owner at 245–260. In
[service_dispose.rs](../../../../../../crates/re-cordis-core/tests/service_dispose.rs):750–820,
Rust `owner_shutdown=true` selects provider disposal at 798 and maps to TS
**false**, with gated open/read-before-close assertions at 808–812. Rust false
manually disposes the composed effect at 803 and remains additional evidence.
The Rust siblings are root children; it has no equivalent non-root outer owner.

TS `:238/false/a266,a267` retain source mappings to the Rust true branch.
`:238/true/a266,a267` retain the useful provider/manual-effect assertions as
partial evidence and name the missing composed-sibling outer-owner shutdown
regression. General root-drain and inherited-child tests do not close this gap.

## SR-3: Replacement values and read paths

The old-withdrawal test at Rust 476–536 distinguishes versions 1 and 2 through
consumer setup/cleanup reads (505–506,516), and asserts `[1,1,2]` at 532 while
withdrawal has settled before replacement setup is released (529–534).
`:355/a380,a382` remain supported. It does not read the provider/root current
slot as TS 376/381 does; `:355/a381` keeps its partial evidence and a precise
provider-current read gap before replacement setup release.

The initialization test at Rust 545–610 uses the **same Rc(3)** twice. Distinct
publication IDs at 595 and the load-ordinal timeline `[1,-1,2,-2]` at 600 prove
two loads and rollback ordering. Awaiting old withdrawal before final consumer
disposal (597–599) supports `:418/a439`; root lookup remaining Some (601–604)
proves withdrawal did not erase the current slot. Its reads of 3 at 578/585
cannot distinguish old versus new publications, and there is no consumer read
after convergence. `:418/a440,a441` retain those identity/order assertions with
separate gaps for the actual post-convergence consumer value and the actual
setup/rollback/cleanup value timeline. Their actions use distinguishable values
1/2 and gate the initial setup during replacement. The different-value
old-withdrawal scenario begins with an Active old load and is not substituted
for this in-flight initialization scenario.

## SR-4: Service cleanup error identity

`:184/a195` keeps two distinct partial witnesses. Rust service cleanup at
451–467 proves provider read-before-close, final listener delivery and one
failure; it never inspects its source or diagnostic allocation.
[foundations.rs](../../../../../../crates/re-cordis-core/tests/foundations.rs):251–258
separately downcasts a root-drain UserError source and recovers field 7. That
is typed-source/value preservation, not allocation identity. The original-ID
gap names a service-dependent cleanup returning an Rc source adapter and exact
pointer checks on the single failure and single diagnostic, alongside the
retention trace. Explicit ErrorValue pointer tests and apply-failure parity
helpers do not execute this composition and are not assigned new paired claims.

## Retained obligations, gaps and non-obligations

Outside SR-1–SR-4, the initial review's accepted service mappings and evidence
limits are carried forward. The five original gaps remain unchanged:
`service.spec.ts:7/a33`, `:128/a161,a162,a163`, and `:203/a214` (initialization
activation count, three initialized DAG callbacks, exact second restart apply).
The ten new gaps above produce 15 total. Partial Rust witnesses within a gap
are permitted by the existing class schema and still join to passing matrix
results; they do not make the full source obligation passed. Every gap names
an owner, regression action and dependency triage. Passing regression evidence
can close a gap without production changes. Conditional B2 requires a failing
public-API reproducer and an actual dependent gate.

The following four original inventory IDs stay omitted only because the guard
at source line 123 is false for their exact registered parameters; source line
125 lies inside guarded span 123–126. The initial review confirms this provenance.
The later shared-ledger owner retains their final `not-applicable` metadata,
source file/declaration/assertion lines, exact parameters, TS/inventory revisions
and guard reason. They have no behavioral class, gap, Rust pass or paired claim.

| Original ID | Exact parameters |
|---|---|
| `core/service-dispose.spec.ts:95/provider/a125` | `{"action":"provider"}` |
| `core/service-dispose.spec.ts:95/root/a125` | `{"action":"root"}` |
| `core/service-dispose.spec.ts:95/provider first/a125` | `{"action":"provider first"}` |
| `core/service-dispose.spec.ts:95/consumer first/a125` | `{"action":"consumer first"}` |

`restart/a125` stays behavioral. Its existing restart-ready/provider-available
witness remains unchanged. Validator/control and shared ledger files are not
edited by this correction.

## SR-5: Actual scoped validation and recorded results

The previous report command did not execute all the parameter, original-ID
provenance and restriction-anchor checks it claimed. The initial review records
that discrepancy; the command/output below replace those unsupported claims.
This command checks the complete owned inputs, including their partial Rust
witnesses, against the untouched inventory and existing class schema. It joins
**20 distinct Rust test identities to 40 exact passing matrix rows and original
log lines**, and checks all three used restriction anchors. These are recorded
Rust execution joins, not new test execution or proof of semantic equivalence.

Run from the correction worktree (also valid after its commit). The schema call
uses the existing `validate_obligation` function without running the full-ledger
validator, its self-tests, or another matrix audit. Markdown checks call only
the existing link/anchor helpers. Owned-file and executable-delta checks compare
the correction against its fixed base and implementation revision.

~~~sh
set -e
python3 -B - <<'PY'
import importlib.util, json, re, subprocess
from collections import Counter
from pathlib import Path

root = Path.cwd()
integration = Path('/Users/I756434/Projects/private/cordis-rust')
base_revision = 'c462dfdf732bf2ed056eefb7832654efd9629e3a'
ts_pin = '35fdc969e4b6c48c48ed44717c7a018324e4dd88'
inventory_revision = '9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc'
implementation = '808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f'
base = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
files = ['service.json', 'service-dispose.json']
load = lambda p: json.loads(p.read_text())
git = lambda *args: subprocess.check_output(['git', *args], text=True)
original = {c['id']: c for c in load(base / 'ledger/cases.json')['declarations']}
data = [load(base / 'ledger-inputs' / name) for name in files]
cases = [c for d in data for c in d['declarations']]
obs = [o for c in cases for v in c['variants'] for o in v['obligations']]
assert len(cases) == len({c['id'] for c in cases}) == 25
assert sum(len(c['variants']) for c in cases) == 35
assert len(obs) == len({o['id'] for o in obs}) == 81
counts = Counter(o['class'] for o in obs)
assert counts == {'source-mapped-test': 55, 'rust-representation': 11, 'missing-evidence': 15}
assert Counter(c['source_file'].split('/')[-1] for c in cases) == {
    'service.spec.ts': 8, 'service-dispose.spec.ts': 17}
assert (base / 'ledger/cases.json').read_text() == git(
    'show', base_revision + ':docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/cases.json')

spec = importlib.util.spec_from_file_location('b0_validator', base / 'ledger/validate.py')
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
doc_spec = importlib.util.spec_from_file_location('docs_check', root / 'tests/docs/check.py')
docs = importlib.util.module_from_spec(doc_spec)
doc_spec.loader.exec_module(docs)
results = load(base / 'matrix/results.json')
commands = {c['id']: c for c in load(base / 'matrix/commands.json')}
command_ids = {'stable-workspace-test', '1.85.0-workspace-test'}
logs = {}
for command in command_ids:
    assert commands[command]['exit_code'] == 0
    logs[command] = (base / 'matrix' / commands[command]['log']).read_text()
context = {'results': results, 'commands': commands, 'logs': logs}
errors, omitted, refs, restrictions = [], set(), set(), set()
obligation_ids, gap_ids, rust_ids, paired = set(), set(), set(), set()
identity = ('id', 'source_file', 'source_line', 'source_end_line', 'title', 'suite', 'historical_checklist')
for d in data:
    mod.check_keys(d, ('schema_version', 'rust_revision', 'typescript_revision',
                      'implementation_revision', 'status', 'declarations'), 'input', errors)
    assert d['schema_version'] == 1 and d['rust_revision'] == inventory_revision
    assert d['typescript_revision'] == ts_pin and d['implementation_revision'] == implementation
    assert mod.final_text(d['status'])
for c in cases:
    mod.check_keys(c, (*identity, 'variants', 'classes'), c['id'], errors)
    old = original[c['id']]
    assert all(json.dumps(c[k], sort_keys=True) == json.dumps(old[k], sort_keys=True) for k in identity)
    assert json.dumps([v['parameters'] for v in c['variants']], sort_keys=True) == json.dumps([v['parameters'] for v in old['variants']], sort_keys=True)
    assert c['classes'] == sorted({o['class'] for v in c['variants'] for o in v['obligations']})
    source = (integration / c['source_file']).read_text().splitlines()
    assert 'it' in source[c['source_line'] - 1]
    assert c['title'] in '\n'.join(source[c['source_line'] - 1:c['source_line'] + 2])
    for v, ov in zip(c['variants'], old['variants']):
        mod.check_keys(v, ('parameters', 'obligations'), c['id'] + ' variant', errors)
        old_obs = {o['id']: o for o in ov['obligations']}
        new_obs = {o['id']: o for o in v['obligations']}
        assert not new_obs.keys() - old_obs.keys()
        omitted.update(old_obs.keys() - new_obs.keys())
        assert v['obligations']
        for o in v['obligations']:
            assert o['source_assertion_lines'] == old_obs[o['id']]['source_assertion_lines']
            assert all(c['source_line'] <= n <= c['source_end_line'] <= len(source)
                       for n in o['source_assertion_lines'])
            mod.validate_obligation(o, o['id'], errors, context, root, integration,
                                    (c['source_line'], c['source_end_line']),
                                    obligation_ids, gap_ids, rust_ids, paired)
            assert o['paired_evidence'] == []
            if o['class'] == 'missing-evidence':
                assert o['execution_status'] == 'not-executed' and o['gap']
                assert all(o['gap'][k] for k in ('id', 'kind', 'next_action', 'owner', 'work_package'))
                assert 'failing public-API reproducer' in o['gap']['work_package']
            else:
                assert o['execution_status'] == 'passed' and o['rust_evidence'] and o['gap'] is None
            if o['restriction']:
                r = o['restriction']
                assert r['reason'] and r['anchor'] in docs.anchors((root / r['document']).read_text())
                restrictions.add((r['document'], r['anchor']))
            for e in o['rust_evidence']:
                lines = (root / e['file']).read_text().splitlines()
                assert all(1 <= n <= len(lines) for n in e['lines'])
                assert len(re.findall(r'\bfn\s+' + re.escape(e['symbol']) + r'\s*\(', '\n'.join(lines))) == 1
                assert set(e['command_ids']) == command_ids
                refs.add((e['file'], e['symbol']))
assert not errors, '\n'.join(errors)
expected_omitted = {f'core/service-dispose.spec.ts:95/{a}/a125'
                    for a in ['provider', 'root', 'provider first', 'consumer first']}
assert omitted == expected_omitted
source = (integration / 'local/cordis/packages/core/tests/service-dispose.spec.ts').read_text().splitlines()
assert source[122].strip() == "if (action === 'restart') {"
assert source[124].strip() == 'expect(consumer.ctx.foo).toBeDefined()'
assert source[125].strip() == '}'
for original_id in omitted:
    variant = next(v for v in original['core/service-dispose.spec.ts:95']['variants']
                   if any(o['id'] == original_id for o in v['obligations']))
    assert variant['parameters']['action'] != 'restart'
    assert next(o for o in variant['obligations'] if o['id'] == original_id)['source_assertion_lines'] == [125]
assert len(gap_ids) == 15 and not paired
assert len(refs) == len(rust_ids) == 20
for file, symbol in refs:
    for command in command_ids:
        rows = [r for r in results['tests'] if r['command_id'] == command and r['test'] == symbol]
        assert len(rows) == 1 and rows[0]['suite'] == file.removeprefix('crates/re-cordis-core/')
        assert rows[0]['outcome'] == 'ok' and not rows[0]['compile_fail']
        assert re.search(r'^test ' + re.escape(symbol) + r' \.\.\. ok$', logs[command], re.M)
assert git('-C', str(integration / 'local/cordis'), 'rev-parse', 'HEAD').strip() == ts_pin
assert not git('-C', str(integration / 'local/cordis'), 'status', '--porcelain', '--untracked-files=no').strip()
assert not (integration / 'local/cordis/packages/core/tests/re-cordis-lifecycle-parity.spec.ts').exists()
docs.links(base / 'ledger-inputs/report-services.md')
assert all(line == line.rstrip() for line in (base / 'ledger-inputs/report-services.md').read_text().splitlines())
owned = {'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/' + name
         for name in files + ['report-services.md']}
assert set(git('diff', '--name-only', base_revision).splitlines()) == owned
assert not git('ls-files', '--others', '--exclude-standard').strip()
assert not git('diff', implementation, '--', 'crates', 'tests', 'spikes', 'Cargo.toml',
               'Cargo.lock', '.cargo', 'rust-toolchain.toml')
print('PASS: 25 declarations / 35 variants / 81 original-ID obligations; 55 mapped + 11 representation + 15 gaps')
print('PASS: exact inventory identity/parameters/assertion provenance; only 4 confirmed guarded a125 omissions')
print('PASS: class/schema checks; 15 owned gap actions; partial evidence does not give gaps passing obligation status')
print(f'PASS: 20 Rust identities / 40 exact ok result+log joins; {len(restrictions)} restriction anchors; zero paired claims')
print('PASS: clean pinned TS source; fixture absent; report links/whitespace; exactly 3 owned changed files; executable delta empty')
PY
git diff c462dfdf732bf2ed056eefb7832654efd9629e3a --check
~~~

The recorded Python command and whitespace check both exited 0:

~~~text
PASS: 25 declarations / 35 variants / 81 original-ID obligations; 55 mapped + 11 representation + 15 gaps
PASS: exact inventory identity/parameters/assertion provenance; only 4 confirmed guarded a125 omissions
PASS: class/schema checks; 15 owned gap actions; partial evidence does not give gaps passing obligation status
PASS: 20 Rust identities / 40 exact ok result+log joins; 3 restriction anchors; zero paired claims
PASS: clean pinned TS source; fixture absent; report links/whitespace; exactly 3 owned changed files; executable delta empty
~~~

The whitespace check produced no output. These scoped corrections are ready for
independent re-review; final shared-ledger assembly and B0 acceptance remain
separate integration gates.
