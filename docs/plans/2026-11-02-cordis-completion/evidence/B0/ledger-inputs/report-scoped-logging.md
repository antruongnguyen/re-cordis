# Scoped and logging family reconciliation

Status: SL-1–SL-6 corrections recorded; independent scoped re-review and final
ledger integration remain pending. This is B0 evidence correction only.

## Scope, revisions and carried evidence

- Assignment: the scoped/logging row of the [correction handoff](../handoffs/review-corrections.md), addressing findings and gap-action notes in the [independent family review](../scoped-logging-review.md).
- Isolated worktree: `/Users/I756434/.codex/worktrees/b0-scoped-logging-corrections/cordis-rust`, base `7fd7cde`; source/integration: `/Users/I756434/Projects/private/cordis-rust`.
- Pinned TypeScript: cordiverse/cordis `35fdc969e4b6c48c48ed44717c7a018324e4dd88`, with clean `local/cordis`.
- Accepted matrix Rust revision: `9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc`; executable implementation: `808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f`. No core-path diff from either revision; all 71 matrix executable-input hashes still match.
- Owned: six fragments and this report. No production/test/matrix/shared ledger/validator/tracking edits.

Carry forward explicit ServiceKey/AccessorKey associations, ServiceInvoke,
BoundService definition/use/caller views, isolation history, service-specific
typed intercept merging, logger name/error representations and fixed
1000-record buffer in [M5](../../../../../m5.md) and [M6](../../../../../m6.md).
Accepted matrix/report, decorator/reflect, first-two events and
integrated-validation reviews plus six paired lifecycle scenarios/73 observations
retain their prior scope. Their completed audits/executions are not repeated.
The reopened delta is the family review's exact assertion mismatches and
candidate-fixture descriptions, not a changed implementation or demonstrated
production defect.

The original report at `be0a08b` (data `87b41ce`) recorded 40 declarations,
40 variants, 136 obligations, 61 representations, 38 mappings, 37 gaps and
15 distinct tests/30 passing joins. The family review verified those submitted
counts/joins while requesting semantic corrections. That historical result
remains; the challenged class claims are superseded below.

## Corrected counts

| Family | Fragment | Declarations | Variants | Obligations | Distinct obligation tests | Representation / mapped / gap |
|---|---|---:|---:|---:|---:|---|
| associate | [associate.json](associate.json) | 6 | 6 | 24 | 3 | 22 / 0 / 2 |
| invoke | [invoke.json](invoke.json) | 2 | 2 | 11 | 1 | 7 / 0 / 4 |
| intercept | [intercept.json](intercept.json) | 7 | 7 | 8 | 2 | 2 / 5 / 1 |
| isolate | [isolate.json](isolate.json) | 6 | 6 | 53 | 4 | 0 / 19 / 34 |
| logger | [logger.json](logger.json) | 11 | 11 | 20 | 3 | 13 / 3 / 4 |
| shadow | [shadow.json](shadow.json) | 8 | 8 | 20 | 4 | 15 / 0 / 5 |
| **Total** | six fragments | **40** | **40** | **136** | **14 distinct tests** | **59 / 27 / 50** |

All declaration/variant identities, original obligation IDs, exact source
assertion spans/behaviors, suite/checklist provenance and fragment revision
metadata remain unchanged. Exactly 46 obligations have corrected evidence,
class or rationale/action text; 90 are unchanged as data. All 37 prior gap IDs,
kinds, owners and conditional-B2 routing remain. Thirteen overclaimed
observations become precise new gaps; two merge claims become the accepted
Rust representation. No paired evidence is claimed. Source-mapped tests join
reviewed TS assertions to passing Rust tests; they do not execute TS declarations.
Test totals are neither declaration totals nor parity percentages.

## Findings disposition

| Finding | Corrected obligations | Disposition and retained evidence |
|---|---|---|
| SL-1 | associate:114/a169; :235/a242, a247 notes | `/a169` is a gap for the successful authorized initial accessor read before writing. TS 167 injects bar; 169 reads unset state. The Rust candidate initializes `Session.secret` to 0, checks unauthorized errors, then writes before its first authorized read. Typed field/accessor representation remains. `/a242` now says X is created/formatted inside one callback; `/a247` names that actual Debug fixture for the missing second forwarding hop. |
| SL-2 | isolate:55/a90; :106/a145, a146; :172/a181, a182, a185, a193; prior isolation gap notes | `/a90` reuses the supplied `services.rs:435–444` witness: withdraw root publication, settle, then explicitly assert `root.get_unchecked` is None. It covers the absent global lookup only. The two isolated post-withdrawal reads with populated root fallback, outer injected/first own reads, no-inject own-provision child and withdrawn-own Undeclared error become six gaps. All prior isolation gaps now name the actual fixture/lines and missing value, counter, authority path or diagnostic observation. |
| SL-3 | logger:37/a47, a48, a52 | Three gaps distinguish first-destination silence, second-destination survival and explicit second-handle removal. A shared counter cannot identify which exporter survives. Its valid aggregate-count and post-root-drain silence evidence is preserved separately below. Follow-up uses distinct destinations and both individual handles. |
| SL-4 | invoke:5/a53 | A gap requires stored mapped extension state to affect invocation with equal arguments. The unit Callable's 8→9 changes an argument, not extension state. Callability/definition-authority representations and three intercept-composition gaps remain. |
| SL-5 | shadow:93/a149, a152 | Separate root-entry and injected-consumer-entry gaps require reads through the returned mapped/retraced holder. Original bound reads at 148–174 precede `retraced` creation at 175. Its definition/metadata assertions at 176–180 remain the accepted map/rebind representation at /a150 and /a153. |
| SL-6 | intercept:34/a39; :42/a49 | Both are `rust-representation` with [M5 service-specific merge restriction](../../../../../m5.md#scope-and-bound-calls). Their evidence includes the actual concatenating merge at 227–232 and ordered typed-layer results. No overlapping-object-property overwrite or default shallow merge is claimed. |

These are correction dispositions awaiting independent scoped re-review,
not B2 admission or full B0 acceptance.

## Retained passing evidence and remaining gaps

Fourteen distinct obligation witnesses have 28 exact passing joins in
[matrix/results.json](../matrix/results.json), one per witness on each of
`stable-workspace-test` and `1.85.0-workspace-test`. Both
[matrix/commands.json](../matrix/commands.json) records have exit 0; both
accepted raw logs contain every exact `test <symbol> ... ok` line. No new test
execution took place. Shared schema, restrictions, references and execution
checks pass for all 136 obligations.

The original exporter fixture remains supplemental scoped evidence:
`crates/re-cordis-core/tests/logger.rs::exporter_removal_is_exact_and_buffer_survives_reusable_root_drains`.
At 299–317 both callbacks use one counter; 318–322 assert 2 then 3 after
`first.remove`; 323–325 drain root and assert no later increment. This proves
one registration survives removal and root drain silences the remainder.
It identifies neither survivor nor explicit second-token removal. Its two
exact toolchain/result/log joins are preserved outside the 136 obligation/class
totals. The previous retained-cleanup fixture also remains supplemental:
`crates/re-cordis-core/tests/scoping.rs::same_name_publications_in_two_realms_keep_resources_until_the_exact_consumer_finishes`.
It publishes the same name in two realms at 491–504, explicitly injects a
consumer into the first at 519–532, and asserts the first value remains
readable during gated cleanup before provider resources close. This remains
valid retention evidence, not a no-inject child read or an own-provision
lookup. Its two exact toolchain/result/log joins are preserved below, outside
obligation totals. The 14 obligation witnesses and two supplemental witnesses
cover all 15 original test identities plus the reused global-withdrawal test
(16 distinct identities total).

The reused `/a90` witness reads root after awaiting withdrawal/settlement.
It does not prove immediate pre-settlement timing, isolated post-withdrawal
lookups, or values surviving in other realms. Those claims are not attached to
this witness; distinct stage/realm obligations retain their own gaps.

All 50 gaps have individual IDs, precise assertions, candidate fixtures,
next actions and owner `core evidence maintainer`. The original 37 remain.
The 13 new IDs (all prefixed `G-B0-`) are:

- `associate-114-a169`: authorized initial accessor value before setter.
- `invoke-5-a53`: configured mapped state affects invocation independently of arguments.
- `isolate-106-a145`, `isolate-106-a146`: each shared-label lookup after withdrawal, with populated root fallback.
- `isolate-172-a181`, `isolate-172-a182`, `isolate-172-a185`, `isolate-172-a193`: outer injected read alongside own realms; first own read; no-inject first-realm child; checked first-realm Undeclared after withdrawing only its own provision.
- `logger-37-a47`, `logger-37-a48`, `logger-37-a52`: exact first removal, second survivor and second individual removal handle.
- `shadow-93-a149`, `shadow-93-a152`: post-map holder probes from root and injected-consumer entry.

A passing targeted regression can close a gap without production changes.
A dependent package must establish its actual gate and a **failing public-API
reproducer** before conditional B2 entry. No new regression, failure or such
dependent gate is claimed here.

## Reproducible checks and actual results

Run this exact block from the correction worktree root, or integration root
after integration, with the pinned source checkout at the recorded path.
It imports the unchanged shared per-obligation validator; it does not validate
the unassembled full ledger or repeat a whole-family semantic audit. Additional
checks enforce fixed inventory/source text, exact crate/suite/result/log joins,
correction scope, prior gap identities/routing and both supplemental
witnesses. All three commands exited 0; whitespace output was silent.

An initial report-authoring wrapper exited 1 (`zsh: unmatched quote`) before
its Python body ran because the outer heredoc delimiter occurred in embedded
report text. No files changed in that failed call. A unique outer delimiter
fixed the wrapper; the complete embedded checks below then ran successfully.

A later original-test-identity check exited 1 when the report initially retained
only the former exporter as supplemental evidence. The removed own-child
mapping also left the valid retained-cleanup witness outside obligation rows.
The report now preserves both witnesses and checks their four passing joins;
all 15 original test identities plus the reused root-withdrawal test are covered.
This was a report-accounting failure, not a Rust test or matrix failure.

~~~sh
set -e
python3 -B - <<'PY'
import importlib.util, json, re
from collections import Counter
from pathlib import Path

root = Path.cwd()
source_root = Path('/Users/I756434/Projects/private/cordis-rust')
base = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
path = base / 'ledger/validate.py'
spec = importlib.util.spec_from_file_location('b0_validator', path)
validator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validator)
errors, context = validator.evidence_context(str(root), str(source_root))
errors = list(errors)
assert not errors, errors
inventory = {d['id']: d for d in validator.load_baseline(root)['declarations']}
expected = {'associate': 6, 'invoke': 2, 'intercept': 7, 'isolate': 6, 'logger': 11, 'shadow': 8}
counts, per_family, current, previous = Counter(), {}, {}, {}
ids, gaps, tests, paired = set(), set(), set(), set()
seen, source_files, rust_files = set(), set(), set()
command_ids = ['stable-workspace-test', '1.85.0-workspace-test']
for family, n in expected.items():
    fragment = base / 'ledger-inputs' / f'{family}.json'
    data = json.loads(fragment.read_text())
    original = json.loads(validator.git_output(root, 'show', f'7fd7cde:{fragment.relative_to(root)}'))
    assert {k: v for k, v in data.items() if k != 'declarations'} == {
        k: v for k, v in original.items() if k != 'declarations'
    }
    assert len(data['declarations']) == n
    family_counts = Counter()
    for declaration, old in zip(data['declarations'], original['declarations']):
        counts['declarations'] += 1
        assert declaration['id'] not in seen
        seen.add(declaration['id'])
        control = inventory[declaration['id']]
        assert {k: v for k, v in declaration.items() if k not in {'classes', 'variants'}} == {
            k: v for k, v in old.items() if k not in {'classes', 'variants'}
        }
        for field in ['source_file', 'source_line', 'source_end_line', 'suite', 'title', 'historical_checklist']:
            assert declaration[field] == control[field], (declaration['id'], field)
        assert declaration['historical_checklist']['checked'] is True
        assert (root / declaration['historical_checklist']['file']).is_file()
        source = source_root / declaration['source_file']
        source_files.add(source)
        source_lines = source.read_text().splitlines()
        assert 1 <= declaration['source_line'] <= declaration['source_end_line'] <= len(source_lines)
        assert repr(declaration['title']) in source_lines[declaration['source_line'] - 1]
        assert len(declaration['variants']) == len(control['variants']) == len(old['variants'])
        derived = set()
        for variant, control_variant, old_variant in zip(declaration['variants'], control['variants'], old['variants']):
            counts['variants'] += 1
            assert variant['parameters'] == control_variant['parameters'] == old_variant['parameters']
            assert [o['id'] for o in variant['obligations']] == [o['id'] for o in control_variant['obligations']]
            assert [o['id'] for o in variant['obligations']] == [o['id'] for o in old_variant['obligations']]
            for obligation, control_o, old_o in zip(variant['obligations'], control_variant['obligations'], old_variant['obligations']):
                current[obligation['id']], previous[obligation['id']] = obligation, old_o
                assert obligation['behavior'] == old_o['behavior'] == control_o['behavior']
                assert obligation['source_assertion_lines'] == old_o['source_assertion_lines'] == control_o['source_assertion_lines']
                assert obligation['behavior'] == '\n'.join(source_lines[n - 1].strip() for n in obligation['source_assertion_lines'])
                validator.validate_obligation(obligation, obligation['id'], errors, context, root, source_root,
                    (declaration['source_line'], declaration['source_end_line']), ids, gaps, tests, paired)
                counts['obligations'] += 1
                counts[obligation['class']] += 1
                family_counts[obligation['class']] += 1
                derived.add(obligation['class'])
                assert not obligation['paired_evidence']
                if obligation['class'] == 'missing-evidence':
                    assert obligation['rust_evidence'] == [] and obligation['execution_status'] == 'not-executed'
                for evidence in obligation['rust_evidence']:
                    rust_file = root / evidence['file']
                    rust_files.add(rust_file)
                    assert re.search(r'\b(?:async\s+)?fn\s+' + re.escape(evidence['symbol']) + r'\b', rust_file.read_text())
                    suite = 'tests/' + rust_file.name
                    assert set(evidence['command_ids']) == set(command_ids)
                    for command_id in command_ids:
                        rows = [r for r in context['results']['tests'] if
                            (r['command_id'], r['crate'], r['suite'], r['test'], r['outcome']) ==
                            (command_id, 're-cordis-core', suite, evidence['symbol'], 'ok')]
                        assert len(rows) == 1
                        assert re.search(r'^test ' + re.escape(evidence['symbol']) + r' \.\.\. ok$', context['logs'][command_id], re.M)
        assert declaration['classes'] == sorted(derived)
    per_family[family] = dict(sorted(family_counts.items()))
assert not errors, errors
old_tests = {(e['file'], e['symbol'], False) for o in previous.values() for e in o['rust_evidence']}
supplemental_tests = {
    ('crates/re-cordis-core/tests/logger.rs', 'exporter_removal_is_exact_and_buffer_survives_reusable_root_drains', False),
    ('crates/re-cordis-core/tests/scoping.rs', 'same_name_publications_in_two_realms_keep_resources_until_the_exact_consumer_finishes', False),
}
assert old_tests - tests == supplemental_tests
assert tests - old_tests == {('crates/re-cordis-core/tests/services.rs', 'repeated_republication_restores_baselines_and_disposed_handles_release_snapshot_values', False)}
assert (counts['declarations'], counts['variants'], counts['obligations']) == (40, 40, 136)
assert (counts['rust-representation'], counts['source-mapped-test'], counts['missing-evidence']) == (59, 27, 50)
assert not paired
# Bound the corrections; every other original obligation remains byte-equivalent as data.
changed = {k for k in current if current[k] != previous[k]}
allowed = {f'core/{f}.spec.ts:{d}/default/a{a}' for f,d,a in [
    ('associate',114,169),('associate',235,242),('associate',235,247),('invoke',5,53),
    ('intercept',34,39),('intercept',42,49),('isolate',55,90),('isolate',106,145),('isolate',106,146),
    ('isolate',172,181),('isolate',172,182),('isolate',172,185),('isolate',172,193),
    ('logger',37,47),('logger',37,48),('logger',37,52),('shadow',93,149),('shadow',93,152)]}
allowed |= {k for k,o in previous.items() if k.startswith('core/isolate.spec.ts:') and o['class'] == 'missing-evidence'}
assert changed == allowed, (changed - allowed, allowed - changed)
expected_new_gaps = {f'core/{f}.spec.ts:{d}/default/a{a}' for f,d,a in [
    ('associate',114,169),('invoke',5,53),('isolate',106,145),('isolate',106,146),
    ('isolate',172,181),('isolate',172,182),('isolate',172,185),('isolate',172,193),
    ('logger',37,47),('logger',37,48),('logger',37,52),('shadow',93,149),('shadow',93,152)]}
assert {k for k in current if current[k]['class'] == 'missing-evidence' and previous[k]['class'] != 'missing-evidence'} == expected_new_gaps
for k in ['core/intercept.spec.ts:34/default/a39','core/intercept.spec.ts:42/default/a49']:
    o = current[k]
    assert o['class'] == 'rust-representation'
    assert (o['restriction']['document'], o['restriction']['anchor']) == ('docs/m5.md','scope-and-bound-calls')
    assert 'not evidence of overlapping-property overwrite' in o['restriction']['reason']
e = current['core/isolate.spec.ts:55/default/a90']['rust_evidence'][0]
assert e['file'] == 'crates/re-cordis-core/tests/services.rs' and 444 in e['lines']
assert e['symbol'] == 'repeated_republication_restores_baselines_and_disposed_handles_release_snapshot_values'
assert 'root.get_unchecked(&key).unwrap().is_none()' in (root / e['file']).read_text().splitlines()[443]
for k,o in previous.items():
    if o['gap']:
        assert {f: current[k]['gap'][f] for f in ['id','kind','owner','work_package']} == {
            f: o['gap'][f] for f in ['id','kind','owner','work_package']}
for path in rust_files:
    assert path.read_bytes() == (source_root / path.relative_to(root)).read_bytes()
assert validator.git_output(root, 'diff', '--name-only', '808c9f5', '--', 'crates/re-cordis-core') == ''
assert validator.git_output(root, 'diff', '--name-only', '9a7633c', '--', 'crates/re-cordis-core') == ''
# Keep former exporter and retained-cleanup witnesses with their real scope, outside obligation totals.
for file, supplemental, _ in sorted(supplemental_tests):
    rust_file = root / file
    assert rust_file.read_bytes() == (source_root / file).read_bytes()
    for command_id in command_ids:
        assert sum((r['command_id'], r['crate'], r['suite'], r['test'], r['outcome']) ==
            (command_id, 're-cordis-core', 'tests/' + rust_file.name, supplemental, 'ok') for r in context['results']['tests']) == 1
        assert re.search(r'^test ' + re.escape(supplemental) + r' \.\.\. ok$', context['logs'][command_id], re.M)
print('OK shared schema/source/reference checks:', len(ids), 'obligations; 71 executable hashes unchanged; TS pin clean')
print('OK inventory:', counts['declarations'], 'declarations;', counts['variants'], 'variants;', counts['obligations'], 'original obligations')
print('OK classes:', {k: counts[k] for k in ['rust-representation','source-mapped-test','missing-evidence']})
for family, result in per_family.items():
    print('OK family', family, result)
print('OK SL-1–SL-6 dispositions: 13 new precise gaps; 2 typed-merge representations; 1 reused global-lookup witness')
print('OK bounded correction:', len(changed), 'changed obligations;', len(current)-len(changed), 'unchanged obligations;', len(gaps), 'distinct gaps')
print('OK Rust tests:', len(tests), 'distinct obligation witnesses;', 2*len(tests), 'exact both-toolchain joins')
print('OK supplemental witnesses: 2 original tests; 4 exact joins; excluded from obligation counts')
for command_id in command_ids:
    c = context['commands'][command_id]
    print('OK command', command_id, 'exit', c['exit_code'], 'log', c['log'])
PY
PYTHONPATH=tests/docs python3 -B - <<'PY'
from pathlib import Path
import check
base = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs')
paths = [base / f'{family}.json' for family in ['associate','invoke','intercept','isolate','logger','shadow']]
report = base / 'report-scoped-logging.md'
for path in paths + [report]:
    raw = path.read_bytes()
    assert raw.endswith(b'\n') and b'\r' not in raw
    assert all(line == line.rstrip() for line in path.read_text().splitlines()), path
check.links(report.resolve())
print('OK links/anchors and direct whitespace: seven owned files')
PY
git diff --check -- docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/associate.json docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/invoke.json docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/intercept.json docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/isolate.json docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/logger.json docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/shadow.json docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/report-scoped-logging.md
~~~

~~~text
OK shared schema/source/reference checks: 136 obligations; 71 executable hashes unchanged; TS pin clean
OK inventory: 40 declarations; 40 variants; 136 original obligations
OK classes: {'rust-representation': 59, 'source-mapped-test': 27, 'missing-evidence': 50}
OK family associate {'missing-evidence': 2, 'rust-representation': 22}
OK family invoke {'missing-evidence': 4, 'rust-representation': 7}
OK family intercept {'missing-evidence': 1, 'rust-representation': 2, 'source-mapped-test': 5}
OK family isolate {'missing-evidence': 34, 'source-mapped-test': 19}
OK family logger {'missing-evidence': 4, 'rust-representation': 13, 'source-mapped-test': 3}
OK family shadow {'missing-evidence': 5, 'rust-representation': 15}
OK SL-1–SL-6 dispositions: 13 new precise gaps; 2 typed-merge representations; 1 reused global-lookup witness
OK bounded correction: 46 changed obligations; 90 unchanged obligations; 50 distinct gaps
OK Rust tests: 14 distinct obligation witnesses; 28 exact both-toolchain joins
OK supplemental witnesses: 2 original tests; 4 exact joins; excluded from obligation counts
OK command stable-workspace-test exit 0 log logs/stable-workspace-test.log
OK command 1.85.0-workspace-test exit 0 log logs/1.85.0-workspace-test.log
OK links/anchors and direct whitespace: seven owned files
~~~

The checks verify cited Rust file equality against integration and no core-path
diff from executable/matrix revisions. No Cargo, Rustdoc, upstream Vitest,
parity runner, full-suite rerun, matrix regeneration, production/test/manifest
edit, shared validator/control edit, subagent or push was used. Integration owns
scoped independent re-review, shared 144-declaration/156-variant/six-type ledger
assembly and final B0 acceptance.
