# B0 events, effects, and event-type reconciliation

Status: **fragment complete; independent integration review pending**. This is
an evidence disposition for the assigned families, not B0 acceptance, a new
TypeScript execution, or a production compatibility claim.

## Scope and revisions

- Rust execution revision: `9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc`.
  `git diff --quiet` confirms that current HEAD has no executable-source, test,
  manifest, lockfile, or toolchain delta from that matrix input.
- TypeScript specification: clean
  `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
- Carried evidence: accepted matrix/report review, accepted first-two events,
  accepted decorator/reflect fragment, and accepted recovered validation. The
  event declarations at source lines 18 and 35 were not re-audited.
- Outputs: `ledger-inputs/events-rest.json`, `dispose.json`, and
  `event-types.json`. No production, test, manifest, shared ledger, tracking,
  accepted-fragment, or matrix artifact was edited.

| Family | Declarations | Registered variants | Obligations | Source-mapped test | Rust representation | Missing evidence |
|---|---:|---:|---:|---:|---:|---:|
| Remaining events | 17 | 17 | 60 | 6 | 47 | 7 |
| Dispose/effects | 13 | 13 | 39 | 0 | 34 | 5 |
| Event type expressions | — | — | 6 | 0 | 6 | 0 |
| **Total** | **30** | **30** | **105** | **6** | **87** | **12** |

The mapped evidence uses 24 distinct Rust integration-test/Rustdoc items. Each
claimed item is present as `ok` in `matrix/results.json` and in both full
`stable-workspace-test.log` and `1.85.0-workspace-test.log`. The six event-type
rows are inventory dispositions: the B0 matrix did not execute
`events.types.ts`. Dynamic values use the documented checked
`Vec<hooks::Value>` representation.

## Concrete evidence gaps

These are missing assertion/compile-evidence boundaries, not confirmed
production defects. Their JSON records name the core evidence maintainer, the
specific regression, and the B2 dependency gate.

- Events: add no-listener success assertions for parallel, serial, and bail;
  add exact source-preserving callback-error assertions for serial and bail;
  add a compile-fail ownership case for retaining an outer `Next` and a
  separate runnable ordering trace.
- Plugin disposal: assert the exact named child `EffectMeta`, then issue a
  second disposal request and prove cleanup remains once.
- Manual/composed effects: add compile-fail Rustdoc coverage showing a consumed
  `Effect` cannot be claimed twice.
- Early stream abort: gate before the first yield, assert the immediate trace is
  empty, then release and clean the in-flight first acquisition.

A passing regression closes each evidence gap without a production change. A
failing public-API reproducer plus an actual dependent package gate is still
required before conditional B2 entry.

## Reproducible bounded validation

No Cargo, docs-example, parity, or upstream suite command was rerun. An initial
tool-wrapper attempt had a JavaScript parse error before any shell program ran;
the successful bounded validator below is the evidence command.

```sh
python3 -B - <<'PY'
import json
from pathlib import Path
root = Path.cwd()
b0 = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
tsroot = Path('/Users/I756434/Projects/private/cordis-rust')
events = json.loads((b0 / 'ledger-inputs/events-rest.json').read_text())
dispose = json.loads((b0 / 'ledger-inputs/dispose.json').read_text())
types = json.loads((b0 / 'ledger-inputs/event-types.json').read_text())
assert [d['source_line'] for d in events['declarations']] == [46,65,77,90,103,134,145,164,183,202,230,244,265,278,296,313,330]
assert [d['source_line'] for d in dispose['declarations']] == [7,23,37,76,90,103,123,143,165,185,197,210,222]
assert [o['source_line'] for o in types['obligations']] == [13,14,17,19,21,22]
assert sum(len(d['variants']) for d in events['declarations']) == 17
assert sum(len(d['variants']) for d in dispose['declarations']) == 13
obligations = [o for data in (events, dispose) for d in data['declarations'] for v in d['variants'] for o in v['obligations']] + types['obligations']
assert len(obligations) == 105
for data in (events, dispose):
    for d in data['declarations']:
        assert d['classes'] == sorted({o['class'] for v in d['variants'] for o in v['obligations']})
        source_count = len((tsroot / d['source_file']).read_text().splitlines())
        assert d['source_end_line'] <= source_count
        assert all(max(o['source_assertion_lines']) <= source_count for v in d['variants'] for o in v['obligations'])
for o in types['obligations']:
    assert o['source_signature_line'] == 9 and o['source_assertion_lines'] == [o['source_line']]
for o in obligations:
    assert o['class'] in {'source-mapped-test','rust-representation','missing-evidence'}
    assert o['behavior'] and o['rationale'] and 'pending-matrix' not in json.dumps(o)
    assert (o['class'] == 'missing-evidence') == bool(o['gap'])
    assert (o['class'] == 'missing-evidence') == (not o['rust_evidence'])
results = json.loads((b0 / 'matrix/results.json').read_text())['tests']
logs = {c: (b0 / f'matrix/logs/{c}.log').read_text() for c in ('stable-workspace-test','1.85.0-workspace-test')}
rustdoc = {
 'Event runnable Rustdoc example':'crates/re-cordis-core/src/events.rs - events::Event (line 62)',
 'Event compile-fail Rustdoc example':'crates/re-cordis-core/src/events.rs - events::Event (line 55) - compile fail',
 'Listener argument compile-fail Rustdoc example':'crates/re-cordis-core/src/events.rs - events::Listener (line 162) - compile fail',
 'Next rustdoc compile-fail example':'crates/re-cordis-core/src/events.rs - events::Next (line 250) - compile fail',
 'AsyncNext rustdoc compile-fail example':'crates/re-cordis-core/src/events.rs - events::AsyncNext (line 263) - compile fail',
}
joins = set()
for o in obligations:
    for e in o['rust_evidence']:
        assert max(e['lines']) <= len((root / e['file']).read_text().splitlines()) and e['assertion']
        item = rustdoc.get(e['symbol'], e['symbol'])
        for command in e['command_ids']:
            assert any(t['command_id'] == command and t['test'] == item and t['outcome'] == 'ok' for t in results)
            assert f'test {item} ... ok' in logs[command]
            joins.add((command, item))
print(f'validated declarations=30 variants=30 obligations={len(obligations)} joined_command_items={len(joins)} gaps={sum(o["class"] == "missing-evidence" for o in obligations)}')
PY
```

Exit `0`:

```text
validated declarations=30 variants=30 obligations=105 joined_command_items=48 gaps=12
```

Executable-input applicability:

```sh
git diff --quiet 9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc HEAD -- crates/re-cordis-core/src crates/re-cordis-core/tests Cargo.toml Cargo.lock rust-toolchain.toml
```

Exit `0`, no output.

The local-link helper and final whitespace checks are recorded after this file
exists; their exact commands and outputs follow in the final section.

## Limits

This fragment joins current Rust evidence to source assertions. It does not run
the TypeScript source declarations, repeat the accepted matrix, prove every
possible dynamic event program, or accept B0 as a whole.

## Final link and whitespace checks

```sh
PYTHONPATH=tests/docs python3 -B - <<'PY'
from pathlib import Path
import check
path = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/report-events-effects-types.md').resolve()
check.links(path)
print(f'checked local links and anchors: {path.relative_to(Path.cwd())}')
PY
```

Exit `0`:

```text
checked local links and anchors: docs/plans/2026-11-02-cordis-completion/evidence/B0/report-events-effects-types.md
```

```sh
git diff --cached --check
```

Exit `0`, no output.
