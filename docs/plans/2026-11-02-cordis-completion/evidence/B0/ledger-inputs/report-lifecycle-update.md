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
| executed-comparison | 24 |
| source-mapped-test | 84 |
| rust-representation | 40 |
| missing-evidence | 40 |
| unsupported | 0 |

The pinned TypeScript specification is Cordis at
[35fdc969e4b6c48c48ed44717c7a018324e4dd88](https://github.com/cordiverse/cordis/tree/35fdc969e4b6c48c48ed44717c7a018324e4dd88).
The ledger control revision is
9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc and the accepted matrix records
unchanged executable implementation revision
808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f. The original reconciliation started from
integration revision 11d84b185ea0d9e9fabe948c5e23415dd216ee7d.
LR2–LR5 corrections use an isolated managed worktree from integration
1c3816596d919302ee352233acd57e95950a63d2 at
`/Users/I756434/.codex/worktrees/b0-lifecycle-update-corrections/cordis-rust`.
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
Four Service.init initializer/returned-disposer count boundaries remain
unasserted. The availability barrier remains tested; the separately registered
cleanup flag cannot measure the no-op returned disposer.

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

There are **37 meaningful missing-evidence obligations** after LR2–LR5, plus
three unchanged LR1 inventory non-obligations. For compatibility with the
current shared validator/control, the three LR1 records temporarily remain
in the raw 40 gap/class count. They are not behavioral omissions. The later
single integration owner must preserve their IDs/source/variant provenance in
not-applicable metadata outside behavioral class/gap totals. This correction
does not edit the shared control or validator.

The raw class total is 188; excluding those three records gives **185 meaningful
obligations: 24 comparisons, 84 source mappings, 40 Rust representations and
37 missing evidence**. The fixed 42 declarations and 44 variants are unchanged.
No production failure or B2 entry was established. All genuine gaps retain
specific next actions, an evidence owner and conditional dependent-gate routing.

| Existing gap group | Obligation IDs | Remaining action |
|---|---|---|
| LR1 inventory artifacts (3) | plugin :77/default/a84; fiber :71/default/a86; update :7/false/a21 | Coordinated not-applicable inventory metadata only; unchanged in this correction. |
| Initializer/returned-disposer counts (4) | plugin :165/default/a178, a179, a181, a182 | Observe initializer counts and the disposer actually returned, before and after disposal. |
| Healthy sibling and failed recovery (3) | fiber :71/default/a90; :144/default/a152, a153 | Preserve healthy-sibling dispatch action; retain a failing recovery update from Failed and observe its own error/state. |
| Sequential callback inputs/counts (4) | fiber :292/default/a303, a304, a308, a309 | One callback journal with exact initial/first-update/second-update inputs and counts. |
| Concurrent-update dependency/waiters (3) | update :7/true/a21; :7/false/a42; :7/true/a42 | Assert injected value and all three gated waiters in each registered variant. |
| Unload and ABA terminal state/source (5) | update :69/default/a92; :96/restart/a116, a117; :96/config aba/a116, a117 | Assert final Active and original retained source after forced unchanged-source reload. |
| Pending dependency and accepted loading update (3) | update :212/default/a225, a230; :234/default/a255 | Assert no pending apply before provider arrival and explicit final Active after readiness. |
| Additional LR3/LR5 boundaries (15) | Exact IDs in the subtables below | Rejected callback, pre-dispatch/disposal counts, repeated-disposal/root-journal checkpoints and three final Active states. |

## LR2–LR5 correction rationale

This is a scoped response to [lifecycle-update-review.md](../lifecycle-update-review.md),
not another whole-family audit. The supplied source/witness paths and the
challenged exact checkpoints were checked with CodeGraph against integration.
The accepted matrix/report, FR-1, first-two-events and recovered validation
remain carried evidence. No design gate, source suite or accepted paired scenario
was reopened, and no production/test/fixture input was edited or rerun.

### LR2 — Callback input and invocation identity

| Obligation IDs | Correction and exact rationale |
|---|---|
| plugin :8/default/a14, a15; fiber :292/default/a298, a299 | Rejoined plugins.rs `typed_object_method_adapter_keeps_config_and_cleanup_with_its_instance`: Plugin::new forwards config to Behavior::apply at 1161–1162; 1144 records it; the complete sorted vector at 1170–1172 is exactly first/second. Distinct admitted inputs at 1166–1167 establish one invocation with each input. The explicit method adapter remains a Rust representation for the functional-plugin entries; the fiber entries retain source-mapped-test. |
| fiber :292/default/a303, a304, a309 | Changed to precise gaps. acceptance.rs:165 ignores config; 172–174 and 189 observe shared handles, not apply inputs/counts. Preserve the alias test for its selected handle representation. |
| fiber :292/default/a308 | Existing gap retained; its next action now covers the whole sequential callback journal, including the first updated input/count and second updated input. |

### LR3 — Cleanup identity and operation checkpoints

| Obligation IDs | Correction and exact rationale |
|---|---|
| plugin :165/default/a182 | Changed to gap: closed at acceptance.rs:359 measures separately registered on_dispose_sync at 319–325; returned disposer 335–337 is a no-op. Repeated-initialization candidate 771 also returns a no-op. Initialization barrier is retained. |
| plugin :165/default/a178, a179, a181 | Existing actions expanded to count the actual returned disposer as well as initialization, with pre/post-disposal observations. |
| plugin :36/default/a47 | Gap: late child callback at plugins.rs:419 is empty. Admission rejection at a41 remains valid; empty diagnostics cannot prove zero invocations. Add a callback counter to that candidate. |
| plugin :86/default/a103 | Gap: first calls assertion is four at plugins.rs:380, not zero after readiness/before emit. |
| plugin :86/default/a118, a120 | Gaps: Registry empty at 383 and root-only increment at 389 precede second disposal at 390. Add the same observations after that completed disposal, before root drain. First-disposal records a111/a113 remain mapped; their shared rationale now states the actual first-disposal scope. |
| plugin :145/default/a150 | Rejoined plugins.rs:43–53, including explicit child/sibling/root ID comparisons at 49–50, for the typed identity substitute. |
| plugin :145/default/a152 | Gap: `dropping_handles_is_inert_and_explicit_disposal_unlinks_parent_ownership` directly observes the child-disposal label at 103, but not journal length exactly one as pinned source 152 asserts. The root-drain candidate never reads the journal. Retain that presence evidence as a candidate, and add the exact one-entry snapshot before root drain. |
| plugin :145/default/a157, a162 | Gaps: root-drain candidate never reads its journal. Empty snapshots at plugins.rs:113 and 1019–1020 follow Registry removal or explicit child disposal. Add root snapshots after the first and second completed root drains. |
| plugin :145/default/a159, a160, a161 | Gaps: drains at 835–841 overlap; identity/Disposed state at 842–843 are before later drain 846; no disposer counter is read. Preserve the valid overlapping-drain evidence and observe a second drain begun only after first completion, before a fresh child. |
| fiber :185/default/a197, a200 | Gaps: cleaned is read only after disposal at 942 and belongs to the successful sibling at 926–929. Throwing callback at 930 has no invocation counter. Diagnostics at 943–946 remain valid in a201; add the throwing callback's own pre/post/repeated-disposal counts. |

### LR4 — Retained failing recovery update

| Obligation IDs | Correction and exact rationale |
|---|---|
| fiber :144/default/a152 | Gap: updates.rs:888 is Active before awaited failing -2 reload at 889. Failed-to-Failed recovery -3 at 891 drops its update waiter; 892 awaits ready(). Neither matches the source's retained recovery-update waiter returning retry failure. |
| fiber :144/default/a153 | Existing gap/action corrected: establish Failed, retain the subsequent failing recovery update, assert its own callback error and final Failed. Merely checking state after -2 covers Active-to-Failed reload. Preserve dropped-waiter evidence separately. |

### LR5 — Consistent final-state evidence

| Obligation IDs | Correction and exact rationale |
|---|---|
| update :54/default/a65 | Gap: updates.rs:172–177 joins tasks and asserts sole value 3, with no returned/current Active comparison. |
| update :188/default/a208 | Gap: updates.rs:752–757 asserts stream trace and no failures, with no Active comparison. The stream representation at a207 remains accepted. |
| fiber :396/default/a409 | Gap: updates.rs:239–259 rejects invalid update and checks source/apply/middleware, with no Active check at that rejection boundary. |
| fiber :326/default/a364 | Reclassified to executed-comparison using `combined_provider_and_consumer_update`. Rust helper 233–273 compares provider/consumer Active after readiness; wrapper 292–293 selects Combined. TS fixture 132–137 compares both Active, and combined.txt contains the full settled line. This adds one obligation mapping, not a scenario/observation. Remove unrelated acceptance.rs:185 middleware-error state and the wrapper restriction from this behavioral checkpoint. |

## Executed-comparison limits

Twenty-four obligations use bounded executed-comparison evidence across four
of the six accepted scenarios: combined, superseded-failure, latest-failure,
and explicit-recovery. Their paired evidence identifies exact retained trace
observations and verifies the corresponding line in the
[parity log](../matrix/logs/parity.log). Coalesced and removed remain accepted
matrix scenarios but are not attached here where a finer declaration assertion
did not match exactly. Source-mapped tests do not execute TypeScript. Rust
representations are not JavaScript mechanism compatibility. No percentage is
reported.

## Exact correction validation commands and results

Run from the correction worktree root. The shared obligation helper checks the
assigned fragments' schema/references; this is not a complete-ledger validator
pass. The command also preserves original source/variant provenance and checks
exact result/log rows on both toolchains. It separately accounts for LR1's three
unchanged records. No Cargo, Rustdoc, docs-example or parity suite is rerun.

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
start = '1c3816596d919302ee352233acd57e95950a63d2'
base = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
inputs = base / 'ledger-inputs'
names = ('plugin.json', 'fiber.json', 'update.json')
control = json.loads((base / 'ledger/cases.json').read_text())
original = {d['id']: d for d in control['declarations']}
fragments = [json.loads((inputs / name).read_text()) for name in names]
results = json.loads((base / 'matrix/results.json').read_text())
commands = {c['id']: c for c in json.loads((base / 'matrix/commands.json').read_text())}
logs = {}
for command in ('stable-workspace-test', '1.85.0-workspace-test', 'parity'):
    assert commands[command]['exit_code'] == 0
    assert commands[command]['start_utc'] and commands[command]['end_utc']
    logs[command] = (base / 'matrix' / commands[command]['log']).read_text()
context = {'results': results, 'logs': logs, 'paired': results['paired_comparisons']}
spec = importlib.util.spec_from_file_location('b0_validator', base / 'ledger/validate.py')
v = importlib.util.module_from_spec(spec)
spec.loader.exec_module(v)
errors = []
ids, gap_ids, rust_ids, scenarios = set(), set(), set(), set()
obligations = []
counts = {}
refs = 0
for fragment in fragments:
    v.check_keys(fragment, ('schema_version', 'rust_revision', 'typescript_revision',
        'implementation_revision', 'status', 'declarations'), 'fragment', errors)
    assert fragment['schema_version'] == 1 and v.final_text(fragment['status'])
    for field in ('rust_revision', 'typescript_revision', 'implementation_revision'):
        assert fragment[field] == control[field]
    variants = total = 0
    for d in fragment['declarations']:
        v.check_keys(d, ('id', 'source_file', 'source_line', 'source_end_line', 'title',
            'suite', 'historical_checklist', 'variants', 'classes'), d['id'], errors)
        old = original[d['id']]
        for field in ('source_file', 'source_line', 'source_end_line', 'title', 'suite', 'historical_checklist'):
            assert d[field] == old[field]
        source = (integration / d['source_file']).read_text().splitlines()
        lo, hi = d['source_line'], d['source_end_line']
        assert 1 <= lo <= hi <= len(source)
        assert d['title'] in '\n'.join(source[lo-1:hi])
        assert [a['parameters'] for a in d['variants']] == [a['parameters'] for a in old['variants']]
        for actual, baseline in zip(d['variants'], old['variants']):
            v.check_keys(actual, ('parameters', 'obligations'), d['id'], errors)
            assert [(o['id'], o['source_assertion_lines']) for o in actual['obligations']] == [
                (o['id'], o['source_assertion_lines']) for o in baseline['obligations']]
            for o in actual['obligations']:
                v.validate_obligation(o, o['id'], errors, context, root, integration,
                    (lo, hi), ids, gap_ids, rust_ids, scenarios)
                obligations.append(o)
                refs += len(o['rust_evidence'])
                if o['class'] == 'missing-evidence':
                    assert not o['rust_evidence'] and not o['paired_evidence']
                for e in o['rust_evidence']:
                    for command in v.REQUIRED_TOOLCHAINS:
                        rows = [r for r in results['tests'] if r['command_id'] == command and r['outcome'] == 'ok']
                        if '/tests/' in e['file']:
                            matches = [r for r in rows if r['suite'] == e['file'].split('crates/re-cordis-core/')[1] and r['test'] == e['symbol']]
                        else:
                            matches = [r for r in rows if v.rustdoc_result_matches(r, e) and r['compile_fail'] is e['compile_fail']]
                        assert len(matches) == 1
                        assert f"test {matches[0]['test']} ... ok" in logs[command].splitlines()
                for e in o['paired_evidence']:
                    assert e['observation'] in (root / f"tests/parity/{e['scenario']}.txt").read_text().splitlines()
            variants += 1
            total += len(actual['obligations'])
        assert d['classes'] == sorted({o['class'] for a in d['variants'] for o in a['obligations']})
    counts[Path(fragment['declarations'][0]['source_file']).name] = (len(fragment['declarations']), variants, total)
assert not errors, '\n'.join(errors)
assert counts == {'plugin.spec.ts': (10, 10, 46), 'fiber.spec.ts': (22, 22, 88), 'update.spec.ts': (10, 12, 54)}
assert len(ids) == len(obligations) == 188
classes = Counter(o['class'] for o in obligations)
assert classes == Counter({'executed-comparison': 24, 'source-mapped-test': 84, 'rust-representation': 40, 'missing-evidence': 40})
non_ids = {'core/plugin.spec.ts:77/default/a84', 'core/fiber.spec.ts:71/default/a86', 'core/update.spec.ts:7/false/a21'}
by_id = {o['id']: o for o in obligations}
for name in names:
    old = json.loads(subprocess.check_output(['git', 'show', f'{start}:docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/{name}'], text=True))
    for d in old['declarations']:
        for a in d['variants']:
            for o in a['obligations']:
                if o['id'] in non_ids:
                    assert by_id[o['id']] == o
assert Counter(o['class'] for o in obligations if o['id'] not in non_ids) == Counter({
    'executed-comparison': 24, 'source-mapped-test': 84, 'rust-representation': 40, 'missing-evidence': 37})
assert Counter(o['gap']['kind'] for o in obligations if o['gap']) == Counter({'missing-evidence': 37, 'observed-source-gap': 3})
assert refs == 148 and len(rust_ids) == 31
assert scenarios == {'combined', 'superseded-failure', 'latest-failure', 'explicit-recovery'}
assert len(results['paired_comparisons']) == 6 and sum(results['paired_comparisons'].values()) == 73
joined = by_id['core/fiber.spec.ts:326/default/a364']
assert joined['class'] == 'executed-comparison' and joined['restriction'] is None
assert joined['rust_evidence'][0]['symbol'] == 'combined_provider_and_consumer_update'
assert joined['paired_evidence'][0]['observation'] == 'settled provider=Active consumer=Active'
ts = (root / 'tests/parity/lifecycle.spec.ts').read_text().splitlines()
rs = (root / 'crates/re-cordis-core/tests/lifecycle_parity.rs').read_text().splitlines()
assert 'expect(provider.state).toBe(FiberState.ACTIVE)' in ts[133]
assert 'expect(consumer.state).toBe(mode' in ts[135]
assert 'assert_eq!(provider.state(), FiberState::Active)' in rs[268]
assert 'assert_eq!(consumer.state(), consumer_state)' in rs[269]
upstream = integration / 'local/cordis'
assert subprocess.check_output(['git', '-C', str(upstream), 'rev-parse', 'HEAD'], text=True).strip() == v.TS_PIN
assert not subprocess.check_output(['git', '-C', str(upstream), 'status', '--porcelain'], text=True).strip()
owned = {str((inputs / name).relative_to(root)) for name in (*names, 'report-lifecycle-update.md')}
assert set(subprocess.check_output(['git', 'diff', '--name-only', start], text=True).splitlines()) == owned
subprocess.run(['git', 'diff', '--exit-code', control['implementation_revision'], '--',
    'crates', 'tests', 'Cargo.toml', 'Cargo.lock', 'spikes/m0'], check=True)
subprocess.run(['git', 'diff', '--exit-code', start, '--', str((base / 'matrix').relative_to(root)),
    str((base / 'ledger').relative_to(root))], check=True)
print('PASS shared obligation schema/source/variant provenance: 42 declarations, 44 variants, 188 raw records')
print('PASS raw classes: comparisons=24 source-mapped=84 representations=40 missing=40')
print('PASS meaningful obligations=185 missing=37; three LR1 records unchanged for integration')
print('PASS 148 Rust references, 296 both-toolchain exact result/log joins, 31 identities; Rustdoc item/line/kind checked')
print('PASS 24 paired obligations across four existing scenarios; combined final-state join exact; accepted corpus six/73 unchanged')
print('PASS clean TS pin; exactly four owned paths; executable inputs, shared ledger/validator and matrix unchanged')
PY
~~~

A refinement run of the same command still expected 32 Rust identities and
exited 1 at `assert refs == 148 and len(rust_ids) == 32`. Removing the label-only
a152 mapping also removed that distinct witness; deriving identities from the
corrected JSON gives 31. The final command above uses that derived count and
exited 0 with this actual output:

~~~text
PASS shared obligation schema/source/variant provenance: 42 declarations, 44 variants, 188 raw records
PASS raw classes: comparisons=24 source-mapped=84 representations=40 missing=40
PASS meaningful obligations=185 missing=37; three LR1 records unchanged for integration
PASS 148 Rust references, 296 both-toolchain exact result/log joins, 31 identities; Rustdoc item/line/kind checked
PASS 24 paired obligations across four existing scenarios; combined final-state join exact; accepted corpus six/73 unchanged
PASS clean TS pin; exactly four owned paths; executable inputs, shared ledger/validator and matrix unchanged
~~~

Report links use the docs helper without executing examples:

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
for link in re.findall(r'\[[^]]+\]\(([^)]+)\)', report.read_text()):
    if '://' in link:
        continue
    raw, _, anchor = link.partition('#')
    target = (report.parent / raw).resolve()
    assert target.is_file(), link
    if anchor:
        assert anchor in docs.anchors(target.read_text()), link
    checked += 1
print(f'PASS local report links={checked}; restriction anchors checked by shared obligation validator')
PY
git diff --check
git diff --cached --check
~~~

All three commands exited 0. The link output was:

~~~text
PASS local report links=12; restriction anchors checked by shared obligation validator
~~~

Both whitespace commands produced no output. These are source/reference and
reused-result checks, not suite reruns or complete-B0 acceptance. LR2–LR5 are
ready for scoped independent re-review; LR1 coordinated accounting remains
with the integration owner.
