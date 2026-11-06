# B0 events/effects evidence corrections

Date: 2026-11-02. Status: **EET-1–EET-9 corrected; scoped independent
re-review pending**. This report supersedes the earlier fragment’s affected
assertion claims and totals. It does not accept B0 or demonstrate a production
defect.

## Carried evidence and scope

Starting checkout: `ff414b545a2ca81405a5d4f04fb5a5b2105ba0e0`, isolated at
`/Users/I756434/.codex/worktrees/b0-events-effects-corrections/cordis-rust`.
Read the [review](events-effects-types-review.md) in full, B0
[Task 2](plan.md#task-2-complete-declaration-and-event-type-disposition),
[reuse rule](../../README.md#prior-decisions-and-evidence-reuse),
[progress/resume](../../progress.md#resume-next), historical
[decisions](../../../../decision.md), [limitations](../../../../limitation.md)
and [implementation plan](../../../../implementation-plan.md).

The specification is integration’s clean TypeScript checkout at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`. CodeGraph was queried first for
affected Rust and TS files; omitted assertion ranges were read directly.

Typed event keys/ControlFlow, explicit sync/async callback shapes, eager factory
prefixes, owned one-use Next/AsyncNext, effect ownership, local hosting and
explicit cleanup remain selected. The [six type records](ledger-inputs/event-types.json)
were independently accepted in the initial review and remain byte-identical.
First-two events, decorator/reflect and FR-1 review, matrix/report review,
recovered validation and six paired scenarios/73 observations are carried.

Only [remaining events](ledger-inputs/events-rest.json),
[dispose](ledger-inputs/dispose.json) and this report changed. No test,
production API, guide, matrix, shared ledger or tracking file changed. No
executable suite was rerun. The preserved matrix executed from `578c066` through
`e23dbac`; applicability control `9a7633c` is not its execution HEAD. Historical
M6 delivery and accepted Rust representations remain in force.

## Corrected accounting

| Family | Declarations | Registered variants | Obligations | Source-mapped test | Rust representation | Missing evidence |
|---|---:|---:|---:|---:|---:|---:|
| Remaining events | 17 | 17 | 67 | 5 | 38 | 24 |
| Dispose/effects | 13 | 13 | 43 | 0 | 29 | 14 |
| Event types, unchanged | — | — | 6 | 0 | 6 | 0 |
| **Total** | **30** | **30** | **116** | **5** | **73** | **38** |

There are 110 runtime records and six type records. All 95 original runtime
IDs/spans and four previously added empty-path IDs remain; 11 sibling records
split missing details from passing substitutes. No non-obligation, declaration
or variant was added. Parent classes are recomputed. The 28 distinct Rust
integration-test/Rustdoc identities have 56 exact passing result/log joins
across stable and 1.85.0. Joins establish execution of each bounded item, not
every source assertion. This fragment has zero executed-comparison or
unsupported records; `events.types.ts` was not executed by the B0 matrix.

The 12 pre-existing gap IDs remain, with the first-yield follow-up extended.
There are 26 additional precise gap records. All 38 name the core evidence
maintainer, a concrete observation and conditional follow-up routing. These are
not 38 production defects or necessarily 38 tests: related observations share
an extension of the existing witness.

## EET dispositions

IDs abbreviate `core/<family>.spec.ts:L/default/aN` as `family:L/aN`.
New sibling IDs append the suffix shown and retain the same source span.

| Finding | Correction and remaining boundary |
|---|---|
| EET-1 | `events:65/a72` is a precise partial-removal gap; final release `/a74` stays. `events:77/a84` now joins `removal_after_selection_does_not_cancel_in_flight_snapshots` 301–340: `first` appears in both emit traces. |
| EET-2 | Affected selection rows cite their actual filtered mode branch and `[global,visible]` at 248. Unfiltered emit cites 250–254; unfiltered parallel/serial/bail cite the named-key calls and `[1,0,2,3]` at 70. All three serial immediate rows also join `serial_has_an_immediate_first_prefix_and_stops_on_zero_or_empty_answers` 421–450: the first factory prefix is asserted before await. The awaited filter trace alone does not supply timing. |
| EET-3 | Parallel symbol label/three-failure aggregate/panic shape stay; `a128-named-label` and `a129-callback-messages` own missing details. Emit `/a161` keeps immediate failure/short-circuit with `a161-message` separate. Dispose `/a188` and `/a200` keep panic/Result/rollback with `-message` siblings. Factory-test cumulative failure count is corrected to two, including the preceding iterator-error branch. |
| EET-4 | `/a239` and `/a270` retain static double-move rejection; AsyncNext’s snippet neither awaits nor runs. `events:230/a240` now joins the runnable valid one-use sync snapshot waterfall: its selected forwarding callback is measured once at 811. `/a241`, `events:265/a274` and `/a275` own missing terminal counts/single-layer result. No compile-fail item supplies runtime results/counts. |
| EET-5 | Initial result 4 and outer `[before,after]` remain in `events:202/a209–a210`; `/a211` joins the measured one-call inner substitute in the sync snapshot branch. `/a219` retains simplified outer-plus-veto result 10 and terminal suppression; `a219-topology` owns the two forwarders/third veto/fourth listener/result 3. `/a220–a223` own the four missing counts. Nested async result 4 stays at `events:278/a290`; `/a291–a292` own missing listener/terminal counts. Snapshot evidence closes neither veto nor nested-count gaps. |
| EET-6 | `/a310`, `/a349` and `/a350` keep scoped local/global count-five observations; `-payload-sequence` siblings own exact resolved values/order across reloads. The typed-update candidate asserts `[resolved 2]` under local veto, not these reload sequences. Input arguments do not establish delivered payloads. |
| EET-7 | Both `dispose:123/a138` and `/a140` share one before-first-yield follow-up: empty immediately, then `[1,2]` and no later acquisition. Line 143’s second-in-flight evidence stays separate. |
| EET-8 | `dispose:37/a69` now joins `sync_setup_is_immediate_and_nested_effects_and_listeners_transfer_ownership` 27–65/71–72: immediate `[0]` contains only setup and no cleanup markers. Its async disposers/joined result are distinguished from sync cleanup timing at `/a71`. `/a30`, `dispose:76/a83`, `143/a156` and `222/a230` own absent immediate observations; later joined outcomes remain. |
| EET-9 | Manual/nested metadata shape/ownership/explicit labels stay at `dispose:23/a27` and `37/a53`; `-default-label` siblings own anonymous labels before naming. |

The measured one-use/initial-inner witnesses are bounded Rust substitutes in a
valid chain; they do not execute the invalid TS duplicate-next program or the
different veto/nested topology. Records preserve the
[M2 event restriction](../../../../m2.md#five-dispatch-modes-with-honest-callback-shapes)
and [effect restriction](../../../../m2.md#effects-and-resource-acquisition).
A passing follow-up closes evidence without production changes. Only a failing
public-API reproducer plus an actual dependent gate permits B2. Scoped re-review
and coordinated integration remain necessary for B0 acceptance.

## Actual bounded verification

Run from the correction checkout. The existing shared obligation helper is
used with only two workspace results/logs; the full-ledger validator and
accepted integrated validation are not repeated.

```sh
python3 -B - <<'PY'
import importlib.util, json, re, subprocess
from collections import Counter
from pathlib import Path
root = Path.cwd()
integration = Path('/Users/I756434/Projects/private/cordis-rust')
b0 = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
spec = importlib.util.spec_from_file_location('b0check', b0 / 'ledger/validate.py')
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
original = json.loads((b0 / 'ledger/cases.json').read_text())
original = {d['id']: d for d in original['declarations']}
results = json.loads((b0 / 'matrix/results.json').read_text())
command_ids = ('stable-workspace-test', '1.85.0-workspace-test')
logs = {c: (b0 / f'matrix/logs/{c}.log').read_text() for c in command_ids}
commands = {c['id']: c for c in json.loads((b0 / 'matrix/commands.json').read_text())}
assert all(commands[c]['exit_code'] == 0 for c in command_ids)
context = {'results': results, 'logs': logs, 'commands': commands,
           'paired': results.get('paired_comparisons', {})}
errors, rows, added = [], [], []
ids, gaps, rust, paired = set(), set(), set(), set()
total_declarations = total_variants = 0
families = [
    ('events-rest', [46,65,77,90,103,134,145,164,183,202,230,244,265,278,296,313,330]),
    ('dispose', [7,23,37,76,90,103,123,143,165,185,197,210,222]),
]
for name, expected_lines in families:
    data = json.loads((b0 / f'ledger-inputs/{name}.json').read_text())
    declarations = data['declarations']
    assert [d['source_line'] for d in declarations] == expected_lines
    family = []
    for d in declarations:
        old = original[d['id']]
        for field in ('id','source_file','source_line','source_end_line',
                      'title','suite','historical_checklist'):
            assert d[field] == old[field], (d['id'], field)
        source = (integration / d['source_file']).read_text().splitlines()
        assert f"it('{d['title']}'" in source[d['source_line'] - 1]
        assert d['source_end_line'] <= len(source)
        assert [v['parameters'] for v in d['variants']] == [v['parameters'] for v in old['variants']] == [{}]
        for v, ov in zip(d['variants'], old['variants']):
            oo = {o['id']: o for o in ov['obligations']}
            no = {o['id']: o for o in v['obligations']}
            assert oo.keys() <= no.keys()
            for oid in oo:
                assert oo[oid]['source_assertion_lines'] == no[oid]['source_assertion_lines']
            added += sorted(no.keys() - oo.keys())
            for o in v['obligations']:
                mod.validate_obligation(o, o['id'], errors, context, root, integration,
                    (d['source_line'], d['source_end_line']), ids, gaps, rust, paired)
                family.append(o)
            actual_expect = {i for i in range(d['source_line'], d['source_end_line'] + 1)
                             if re.search(r'\bexpect\(', source[i - 1])}
            mapped = {i for o in v['obligations'] for i in o['source_assertion_lines']}
            assert actual_expect <= mapped, (d['id'], actual_expect - mapped)
        assert d['classes'] == sorted({o['class'] for v in d['variants'] for o in v['obligations']})
        total_declarations += 1
        total_variants += len(d['variants'])
    rows += family
    print(name, len(declarations), len(family), dict(Counter(o['class'] for o in family)))
for name, _ in families:
    path = b0 / f'ledger-inputs/{name}.json'
    before = json.loads(subprocess.check_output(['git', 'show', f'ff414b5:{path.relative_to(root)}'], text=True))
    current = json.loads(path.read_text())
    for old, new in zip(before['declarations'], current['declarations']):
        for ov, nv in zip(old['variants'], new['variants']):
            prior = {o['id']: o for o in ov['obligations']}
            now = {o['id']: o for o in nv['obligations']}
            assert prior.keys() <= now.keys()
            for oid in prior:
                assert prior[oid]['source_assertion_lines'] == now[oid]['source_assertion_lines']
    old_gaps = {o['gap']['id'] for d in before['declarations'] for v in d['variants'] for o in v['obligations'] if o['gap']}
    assert old_gaps <= gaps
assert len(added) == 15
assert sum('/empty-' in oid for oid in added) == 4
assert (b0 / 'ledger-inputs/event-types.json').read_bytes() == subprocess.check_output(['git', 'show', f'ff414b5:{(b0 / "ledger-inputs/event-types.json").relative_to(root)}'])
types = json.loads((b0 / 'ledger-inputs/event-types.json').read_text())['obligations']
assert [o['source_line'] for o in types] == [13,14,17,19,21,22]
for o in types:
    line = o['source_line']
    assert o['id'] == f'core/events.types.ts:{line}'
    assert o['source_signature_line'] == 9 and o['source_assertion_lines'] == [line]
    mod.validate_obligation(o, o['id'], errors, context, root, integration,
                           (line, line), ids, gaps, rust, paired)
rows += types
assert not errors, '\n'.join(errors)
assert len(rows) == 116 and len(ids) == 116 and len(gaps) == 38 and len(rust) == 28 and not paired
print('types', len(types), dict(Counter(o['class'] for o in types)))
print(f'declarations={total_declarations} variants={total_variants} obligations={len(rows)} Rust_items={len(rust)} passing_toolchain_joins={len(rust)*2} gaps={len(gaps)} errors={len(errors)}')
print('retained_original_runtime_obligations=', len(rows)-len(types)-len(added), 'added=', added)
PY
```

Exit **0**:

```text
events-rest 17 67 {'rust-representation': 38, 'missing-evidence': 24, 'source-mapped-test': 5}
dispose 13 43 {'missing-evidence': 14, 'rust-representation': 29}
types 6 {'rust-representation': 6}
declarations=30 variants=30 obligations=116 Rust_items=28 passing_toolchain_joins=56 gaps=38 errors=0
retained_original_runtime_obligations= 95 added= ['core/events.spec.ts:103/default/a128-named-label', 'core/events.spec.ts:103/default/a129-callback-messages', 'core/events.spec.ts:103/default/empty-105', 'core/events.spec.ts:145/default/a161-message', 'core/events.spec.ts:145/default/empty-147', 'core/events.spec.ts:164/default/empty-166', 'core/events.spec.ts:183/default/empty-185', 'core/events.spec.ts:202/default/a219-topology', 'core/events.spec.ts:296/default/a310-payload-sequence', 'core/events.spec.ts:330/default/a349-payload-sequence', 'core/events.spec.ts:330/default/a350-payload-sequence', 'core/dispose.spec.ts:23/default/a27-default-label', 'core/dispose.spec.ts:37/default/a53-default-label', 'core/dispose.spec.ts:185/default/a188-message', 'core/dispose.spec.ts:197/default/a200-message']
```

Executable applicability and clean specification:

```sh
git diff --quiet 9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc -- crates Cargo.toml Cargo.lock rust-toolchain.toml tests .github
git -C /Users/I756434/Projects/private/cordis-rust/local/cordis rev-parse HEAD
git -C /Users/I756434/Projects/private/cordis-rust/local/cordis status --porcelain
```

Each exited **0**. The first and third have no output; the second returns
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`.

Scoped preservation, links, direct whitespace and final Git whitespace:

```sh
PYTHONPATH=tests/docs python3 -B - <<'PY'
import subprocess
from pathlib import Path
import check
root=Path.cwd(); b0=Path('docs/plans/2026-11-02-cordis-completion/evidence/B0')
owned={str(b0/'ledger-inputs/events-rest.json'),str(b0/'ledger-inputs/dispose.json'),str(b0/'report-events-effects-types.md')}
changed=set(subprocess.check_output(['git','diff','--name-only','ff414b5'],text=True).splitlines())
assert changed==owned,changed
for path in owned:
 lines=Path(path).read_text().splitlines()
 assert all(line==line.rstrip() for line in lines) and not any('\t' in line for line in lines)
 if path.endswith('.md'): check.links(Path(path).resolve())
for path in subprocess.check_output(['git','ls-tree','-r','--name-only','ff414b5',str(b0)],text=True).splitlines():
 if path not in owned: assert Path(path).read_bytes()==subprocess.check_output(['git','show',f'ff414b5:{path}']),path
print('owned_files=3; all other B0 bytes preserved; report links/anchors and direct whitespace passed')
PY
git diff --check
git diff --cached --check
```

Each exited **0**; Python output:

```text
owned_files=3; all other B0 bytes preserved; report links/anchors and direct whitespace passed
```

The staged list contains only these three files. Checks establish accounting,
schema, bounds and joins; scoped re-review determines semantic acceptance.

An initial report-writing shell command used `python3 -B - <<'PY'` around text
containing the same inner `PY` terminator. It failed with
`zsh:150: parse error near '}'`, exit **1**, before Python wrote the report.
The corrected unique outer terminator `EET_REPORT_WRITE` succeeded, exit **0**.
