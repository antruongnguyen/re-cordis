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

All seven gaps retain their concrete regression actions and owner core evidence maintainer. Scheduling is core evidence follow-up and dependency triage: the evidence owner first writes/runs the named regression. A passing result closes missing evidence without production change. A relevant satellite gate must establish that its slice depends on the gap and supply a failing public-API reproducer before [conditional B2 entry](../../../core.md#b2--conditional-core-bridge-or-targeted-regression-fix). No dependent gate is assumed or selected by this fragment. B0 accounting remains separate from implementing these regressions; while B0 is incomplete, dependent implementation stays closed. After B0 accounting completes, a gap blocks only a slice shown to depend on it. The seven open gap IDs remain:

- G-B0-decorator.spec.ts:7/default/a32
- G-B0-decorator.spec.ts:7/default/a35
- G-B0-decorator.spec.ts:7/default/a37
- G-B0-reflect.spec.ts:30/default/a43
- G-B0-reflect.spec.ts:30/default/a47
- G-B0-reflect.spec.ts:30/default/a51
- G-B0-reflect.spec.ts:30/default/a52

These gaps identify missing assertions, not confirmed production failures. Review should check whether the intentional Any/accessor substitutes and disclosed typed-access setup differences adequately dispose of the exact source obligations before merging. Only the two ledger-inputs fragment files are owned; ledger, progress, production, tests and manifests were untouched.

Validation: local Python accounting/reference check confirmed five unique expected declarations, 23 unique original assertion obligations, valid evidence fields/anchors/line bounds, and every cited Rust test in both passing matrix results and original logs. The full ledger validator remains the ledger owner's integration check.

## FR-1 scheduling correction — 2026-11-02

At baseline commit 25329ad995ba21b326e116e801f7f57816d4ed0b, the independent review identified unconditional B2 routing in all seven missing-evidence records. The correction changes only their work_package scheduling text and the matching report paragraph. The original gap IDs, next actions, owner, classifications, execution status, source mappings and existing matrix references remain unchanged. All seven gaps remain open pending their regressions; independent re-review is still required. The accepted matrix is reused for unchanged executable inputs. No Cargo, documentation-example or parity runs were performed.

Fresh bounded structural validation command (run from the isolated worktree root):

~~~sh
python3 -B - <<'PY'
import copy, json, re, subprocess
from collections import Counter
from pathlib import Path
base = Path('docs/plans/2026-11-02-cordis-completion')
p = base / 'evidence/B0/ledger-inputs/decorator-reflect.json'
report = p.with_name('report-decorator-reflect.md')
original = json.loads(subprocess.check_output(['git', 'show', '25329ad:' + str(p)], text=True))
current = json.loads(p.read_text())
expected = {'G-B0-decorator.spec.ts:7/default/a' + str(n) for n in (32, 35, 37)} | {'G-B0-reflect.spec.ts:30/default/a' + str(n) for n in (43, 47, 51, 52)}
obligations = [o for d in current['declarations'] for v in d['variants'] for o in v['obligations']]
gaps = [o['gap'] for o in obligations if o['gap']]
assert len(current['declarations']) == 5 and sum(len(d['variants']) for d in current['declarations']) == 5 and len(obligations) == 23
assert len(gaps) == 7 and {g['id'] for g in gaps} == expected
routing = 'Core evidence follow-up and dependency triage: first the core evidence maintainer writes/runs the named regression. A passing result closes missing evidence without production change. The relevant satellite gate must establish dependency and supply a failing public-API reproducer before conditional B2 entry.'
assert all(g['work_package'] == routing for g in gaps)
restored = copy.deepcopy(current)
for d in restored['declarations']:
    for v in d['variants']:
        for o in v['obligations']:
            if o['gap']:
                o['gap']['work_package'] = 'B2 (targeted evidence regression before dependent satellite gate)'
assert restored == original, 'Non-scheduling data changed'
known = set(re.findall(r'^\| ([A-Z][0-9]+) \|', (base / 'schedule.md').read_text(), re.M))
assert len(known) == 35 and set(re.findall(r'\b[A-Z][0-9]+\b', routing)) <= known
changed = set(subprocess.check_output(['git', 'diff', '25329ad', '--name-only'], text=True).splitlines())
assert changed == {str(p), str(report)} and not subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard'], text=True).strip()
assert (report.parent / '../../../core.md').resolve().is_file()
print('PASS: exactly 7 conditional scheduling gates; all other JSON data unchanged; work-package IDs valid (35 known); only the 2 owned evidence files changed.')
print('PASS: 5 declarations, 5 variants, 23 obligations; classes=' + str(dict(Counter(o['class'] for o in obligations))))
PY
~~~

Result (exit 0):

~~~text
PASS: exactly 7 conditional scheduling gates; all other JSON data unchanged; work-package IDs valid (35 known); only the 2 owned evidence files changed.
PASS: 5 declarations, 5 variants, 23 obligations; classes={'rust-representation': 4, 'missing-evidence': 7, 'unsupported': 3, 'source-mapped-test': 9}
~~~

Additional command: git diff --check. Result: exit 0, no output. The worktree is isolated; integration and independent re-review remain with the orchestrator.
