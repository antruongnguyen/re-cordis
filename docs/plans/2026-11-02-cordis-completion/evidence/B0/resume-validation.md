# B0 integrated resume validation

Bounded resume check completed on 2026-11-02. Validation verdict: **FAIL**. This report covers only the pending integrated structural/input/artifact/link check. It carries forward the accepted B0 matrix/report and FR-1 correction/re-review; it does not accept B0 as a whole, close any of the seven gaps, open dependent implementation, or establish general source-audit transport recovery.

Instruction: [resume-verification.md](handoffs/resume-verification.md). Read-only inputs: the corrected [fragment report](ledger-inputs/report-decorator-reflect.md), [fragment JSON](ledger-inputs/decorator-reflect.json), and accepted [matrix re-review](matrix-rereview.md). Only this report was written; no other edits, commits, agents or pushes.


The strict current-byte preservation check failed for commands.tsv because this checkout has 54 CRLF line endings while the accepted Git blob has LF. All 65 original committed Git blobs match the accepted baseline; the other 64 working files match byte-for-byte, and commands.tsv matches after CRLF-to-LF normalization. No artifact was edited. Structural accounting, all 71 executable hashes, the clean pinned TS checkout and all 12 scoped link inputs passed. The strict-byte failure is retained for the integration owner; general transport recovery and B0 completion remain outside scope.

## Execution context

- Checkout/cwd: /Users/I756434/Projects/private/cordis-rust
- Branch: feat/cordis-completion
- Integration HEAD: f2ff4c3b1a6249f995b7d2d96fdf61741def9d46
- Python: 3.15.2; executable: /opt/homebrew/opt/python@3.15/bin/python3.15
- Full Python version: 3.15.2 (main, Oct 25 2026, 16:57:51) [Clang 21.0.0 (clang-2100.3.34.2)]
- Started UTC: 2026-11-02T12:49:27.878947+00:00
- Validation ended UTC: 2026-11-02T12:49:30.062505+00:00
- Initial git status --short: empty

The fragment changed-files assertion is scoped to its two owned paths in 25329ad..59be2db, with current bytes also compared to 59be2db. Additional integration/review/tracking commits are expected. Link API was inspected before use: tests/docs/check.py:48 defines links(path), checks local targets and Markdown heading anchors, and raises ValueError for failures. Importing the module does not invoke its CLI main. All nine existing recovery handoffs were included.

## Exact validation commands and actual results

### Integrated fragment structure

Cwd: /Users/I756434/Projects/private/cordis-rust. Start UTC: 2026-11-02T12:49:28.008021+00:00. End UTC: 2026-11-02T12:49:28.200699+00:00. Actual exit: 0.

~~~sh
/opt/homebrew/opt/python@3.15/bin/python3.15 -B - <<'PY'
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
changed = set(subprocess.check_output(['git', 'diff', '25329ad', '59be2db', '--name-only', '--', str(p), str(report)], text=True).splitlines())
assert changed == {str(p), str(report)}
for path in (p, report):
    assert path.read_bytes() == subprocess.check_output(['git', 'show', '59be2db:' + str(path)]), 'Integrated correction drift: ' + str(path)
assert (report.parent / '../../../core.md').resolve().is_file()
print('PASS: exactly 7 conditional scheduling gates; all other JSON data unchanged; work-package IDs valid (35 known).')
print('PASS: only the 2 owned evidence files differ in the scoped 25329ad..59be2db comparison; integrated bytes match 59be2db.')
print('PASS: 5 declarations, 5 variants, 23 obligations; classes=' + str(dict(Counter(o['class'] for o in obligations))))
PY
~~~

Output:

~~~text
PASS: exactly 7 conditional scheduling gates; all other JSON data unchanged; work-package IDs valid (35 known).
PASS: only the 2 owned evidence files differ in the scoped 25329ad..59be2db comparison; integrated bytes match 59be2db.
PASS: 5 declarations, 5 variants, 23 obligations; classes={'rust-representation': 4, 'missing-evidence': 7, 'unsupported': 3, 'source-mapped-test': 9}
~~~

### Executable inputs and original matrix preservation

Cwd: /Users/I756434/Projects/private/cordis-rust. Start UTC: 2026-11-02T12:49:28.200735+00:00. End UTC: 2026-11-02T12:49:29.851234+00:00. Actual exit: 1.

~~~sh
/opt/homebrew/opt/python@3.15/bin/python3.15 -B - <<'PY'
import hashlib, json, subprocess
from pathlib import Path
matrix = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/matrix')
accepted = '7d1552a7612a6a7be18421c6a73779624ba41f28'
pin = '35fdc969e4b6c48c48ed44717c7a018324e4dd88'
snapshots = [json.loads((matrix / name).read_text()) for name in ('before.json', 'after.json', 'after-supplement.json')]
hashes = snapshots[0]['executable_input_sha256']
assert len(hashes) == 71
assert all(snapshot['executable_input_sha256'] == hashes for snapshot in snapshots)
assert all(snapshot['ts_head'] == pin for snapshot in snapshots)
mismatches = [name for name, expected in hashes.items() if not Path(name).is_file() or hashlib.sha256(Path(name).read_bytes()).hexdigest() != expected]
assert not mismatches, 'Executable input differences: ' + repr(mismatches)
print('PASS: all 71 baseline executable-input SHA-256 hashes match current files and all 3 preserved snapshots.')
delta = subprocess.check_output(['git', 'diff', '--name-only', snapshots[0]['head'], 'HEAD', '--', '.github', 'Cargo.lock', 'Cargo.toml', 'crates', 'spikes', 'tests'], text=True).strip()
assert not delta, 'Executable tree delta: ' + delta
print('PASS: no tracked executable-tree additions, removals or modifications since baseline execution HEAD.')
ts_head = subprocess.check_output(['git', '-C', 'local/cordis', 'rev-parse', 'HEAD'], text=True).strip()
ts_status = subprocess.check_output(['git', '-C', 'local/cordis', 'status', '--porcelain=v1', '--untracked-files=all'], text=True)
assert ts_head == pin and not ts_status, 'TypeScript pin/status mismatch: ' + repr((ts_head, ts_status))
print('PASS: local/cordis HEAD=' + ts_head + '; porcelain status empty (including non-ignored untracked files).')
paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', accepted, '--', str(matrix)], text=True).splitlines()
assert len(paths) == 65
changed = [name for name in paths if Path(name).read_bytes() != subprocess.check_output(['git', 'show', accepted + ':' + name])]
assert not changed, 'Original matrix artifacts changed: ' + repr(changed)
print('PASS: all 65 original matrix artifacts byte-identical to accepted commit ' + accepted + '.')
print('No Cargo, docs-example or parity rerun: recorded executable inputs and pinned TS checkout unchanged.')
PY
~~~

Output:

~~~text
PASS: all 71 baseline executable-input SHA-256 hashes match current files and all 3 preserved snapshots.
PASS: no tracked executable-tree additions, removals or modifications since baseline execution HEAD.
PASS: local/cordis HEAD=35fdc969e4b6c48c48ed44717c7a018324e4dd88; porcelain status empty (including non-ignored untracked files).
Traceback (most recent call last):
  File "<stdin>", line 24, in <module>
AssertionError: Original matrix artifacts changed: ['docs/plans/2026-11-02-cordis-completion/evidence/B0/matrix/commands.tsv']
~~~

### Scoped documentation links

Cwd: /Users/I756434/Projects/private/cordis-rust. Start UTC: 2026-11-02T12:49:29.851277+00:00. End UTC: 2026-11-02T12:49:30.062466+00:00. Actual exit: 0.

~~~sh
/opt/homebrew/opt/python@3.15/bin/python3.15 -B - <<'PY'
import importlib.util, re
from pathlib import Path
from urllib.parse import urlsplit
checker_path = Path('tests/docs/check.py').resolve()
spec = importlib.util.spec_from_file_location('resume_docs_checker', checker_path)
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)
base = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0')
paths = [base / 'ledger-inputs/decorator-reflect.json', base / 'ledger-inputs/report-decorator-reflect.md', base / 'matrix-rereview.md'] + sorted((base / 'handoffs').glob('*.md'))
assert len(paths) == 12, 'Unexpected handoff inventory: ' + str(len(paths))
local_count = 0
failures = []
for path in paths:
    try:
        checker.links(path.resolve())
        local_count += sum(not urlsplit(target).scheme and not urlsplit(target).netloc for target in re.findall(r'\[[^\]\n]+\]\(([^)\s]+)\)', checker.prose(path.read_text(encoding='utf-8'))))
        print('PASS: links ' + str(path.relative_to(base)))
    except Exception as error:
        failures.append(str(error))
        print('FAIL: ' + str(error))
assert not failures, '\n'.join(failures)
print('PASS: existing check.py links(path) helper checked ' + str(len(paths)) + ' files / ' + str(local_count) + ' local Markdown links, including heading anchors.')
print('Helper imported only; check_structure(), run_examples() and checker main were not invoked.')
PY
~~~

Output:

~~~text
PASS: links ledger-inputs/decorator-reflect.json
PASS: links ledger-inputs/report-decorator-reflect.md
PASS: links matrix-rereview.md
PASS: links handoffs/associate-invoke.md
PASS: links handoffs/events-first-two.md
PASS: links handoffs/events.md
PASS: links handoffs/fragment-fix.md
PASS: links handoffs/fragment-rereview.md
PASS: links handoffs/fragment-review.md
PASS: links handoffs/matrix-report.md
PASS: links handoffs/matrix-rereview.md
PASS: links handoffs/resume-verification.md
PASS: existing check.py links(path) helper checked 12 files / 16 local Markdown links, including heading anchors.
Helper imported only; check_structure(), run_examples() and checker main were not invoked.
~~~

### Whitespace check

Cwd: /Users/I756434/Projects/private/cordis-rust. Start UTC: 2026-11-02T12:49:30.063633+00:00. End UTC: 2026-11-02T12:49:30.074521+00:00. Actual exit: 0.

~~~sh
git diff --check
~~~

~~~text
(no output)
~~~

The report was new/untracked during git diff --check, so that command checks tracked diffs. The report has also been checked directly for trailing whitespace.

### Bounded inspection of the preservation failure

Cwd: /Users/I756434/Projects/private/cordis-rust. Start UTC: 2026-11-02T12:50:00.976474+00:00. End UTC: 2026-11-02T12:50:01.276777+00:00. Actual exit: 0.

~~~sh
/opt/homebrew/opt/python@3.15/bin/python3.15 -B - <<'PY'
import difflib, hashlib, subprocess
from pathlib import Path
p = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/matrix/commands.tsv')
accepted = subprocess.check_output(['git', 'show', '7d1552a7612a6a7be18421c6a73779624ba41f28:' + str(p)])
head = subprocess.check_output(['git', 'show', 'HEAD:' + str(p)])
current = p.read_bytes()
for name, data in [('accepted', accepted), ('integration HEAD', head), ('working tree', current)]:
    print(name + ': bytes=' + str(len(data)) + '; sha256=' + hashlib.sha256(data).hexdigest() + '; CRLF=' + str(data.count(b'\r\n')) + '; LF=' + str(data.count(b'\n')))
print('Working tree equals integration HEAD: ' + str(current == head))
print('Text diff against accepted artifact:')
print(''.join(difflib.unified_diff(accepted.decode().splitlines(True), current.decode().splitlines(True), fromfile='accepted/commands.tsv', tofile='current/commands.tsv')), end='')
print('Artifact commit history since baseline:')
print(subprocess.check_output(['git', 'log', '--oneline', '7d1552a..HEAD', '--', str(p)], text=True), end='')
PY
~~~

~~~text
accepted: bytes=145830; sha256=b93378360d1f0d4320a4688069442e86556ff9fb3f874a48ef660a54c9485813; CRLF=0; LF=54
integration HEAD: bytes=145830; sha256=b93378360d1f0d4320a4688069442e86556ff9fb3f874a48ef660a54c9485813; CRLF=0; LF=54
working tree: bytes=145884; sha256=e26746857fb5c683ea657bdf75e7df99d7328d06b41a6429842b8bcc62d921ea; CRLF=54; LF=54
Working tree equals integration HEAD: False
[Unified diff omitted from this compact report: all 54 line endings differ; original command output had 116 lines.]
Artifact commit history since baseline: (empty)
~~~

The preservation assertion failed for commands.tsv; the original artifact was not repaired. The structural/input/TypeScript/link results remain bounded as recorded. No executable difference was found, so no Cargo/docs-example/parity rerun was performed. This report retains the actual failure for the integration owner.

### Compact verification of the line-ending difference

Cwd: /Users/I756434/Projects/private/cordis-rust. Start UTC: 2026-11-02T12:50:31.306927+00:00. End UTC: 2026-11-02T12:50:32.573128+00:00. Actual exit: 0.

~~~sh
/opt/homebrew/opt/python@3.15/bin/python3.15 -B - <<'PY'
import hashlib, subprocess
from pathlib import Path
matrix = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/matrix')
accepted = '7d1552a7612a6a7be18421c6a73779624ba41f28'
paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', accepted, '--', str(matrix)], text=True).splitlines()
committed_differences = []
raw_differences = []
normalized_differences = []
for name in paths:
    original = subprocess.check_output(['git', 'show', accepted + ':' + name])
    committed = subprocess.check_output(['git', 'show', 'HEAD:' + name])
    current = Path(name).read_bytes()
    if committed != original:
        committed_differences.append(name)
    if current != original:
        raw_differences.append(name)
    if current.replace(b'\r\n', b'\n') != original:
        normalized_differences.append(name)
assert len(paths) == 65 and not committed_differences and not normalized_differences
assert raw_differences == [str(matrix / 'commands.tsv')]
print('PASS: all 65 original matrix Git blobs unchanged at integration HEAD.')
print('Raw working-tree comparison: 64 identical; commands.tsv differs only by 54 LF -> CRLF line endings.')
print('PASS: CRLF -> LF normalization yields accepted bytes for all 65 original artifacts.')
p = str(matrix / 'commands.tsv')
print('git check-attr text eol -- commands.tsv:')
print(subprocess.check_output(['git', 'check-attr', 'text', 'eol', '--', p], text=True), end='')
for key in ('core.autocrlf', 'core.eol'):
    result = subprocess.run(['git', 'config', '--show-origin', '--get', key], text=True, capture_output=True)
    print('git config --show-origin --get ' + key + ': exit ' + str(result.returncode) + '; ' + ((result.stdout + result.stderr).strip() or '(unset)'))
PY
~~~

~~~text
PASS: all 65 original matrix Git blobs unchanged at integration HEAD.
Raw working-tree comparison: 64 identical; commands.tsv differs only by 54 LF -> CRLF line endings.
PASS: CRLF -> LF normalization yields accepted bytes for all 65 original artifacts.
git check-attr text eol -- commands.tsv:
docs/plans/2026-11-02-cordis-completion/evidence/B0/matrix/commands.tsv: text: unspecified
docs/plans/2026-11-02-cordis-completion/evidence/B0/matrix/commands.tsv: eol: unspecified
git config --show-origin --get core.autocrlf: exit 0; file:/Users/I756434/.gitconfig	input
git config --show-origin --get core.eol: exit 1; (unset)
~~~
