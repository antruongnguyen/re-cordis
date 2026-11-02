# B0 services independent initial review

**Spec verdict: changes requested. Quality verdict: changes requested.**
The source inventory and execution joins are sound, but several mappings claim
more than their cited assertions establish. This review does not identify a
confirmed production defect or accept B0 as a whole.

Reviewed input: `d07d3eec2df3979d5413a0ed7477feebe70efe4c`, integrated byte-for-byte
at `7fd7cde471a22dd20e0f015094de2b12b684b032`. Review worktree:
`/Users/I756434/.codex/worktrees/b0-services-initial-review/cordis-rust`.
The specification remains clean at TypeScript
`35fdc969e4b6c48c48ed44717c7a018324e4dd88` under the integration checkout.
Only this review file is owned by this assignment.

## Carried evidence and reviewed boundary

I carried forward the [reuse rule](../../README.md#prior-decisions-and-evidence-reuse),
[decisions](../../../../decision.md), [limitations](../../../../limitation.md),
and completed gates in the [previous plan](../../../../implementation-plan.md).
M4's publication/committed-load distinction, M5's explicit bound views and
isolation identities, and M6's initialized factories remain accepted contracts.
The M4–M6 source maps are starting evidence, not substitutes for these new joins.

The accepted matrix/report, FR-1 correction/re-review, first-two events review,
and recovered integrated validation were reused without repeated audits. The
unchanged six-scenario/73-observation corpus is not assigned any new comparison
claim. All reviewed `paired_evidence` arrays correctly remain empty.

| Submitted scope | Count |
|---|---:|
| Declarations (`service` / `service-dispose`) | 25 (8 / 17) |
| Registered variants | 35 (8 / 27) |
| Behavioral obligation records | 81 |
| Submitted source-mapped-test / rust-representation / missing-evidence | 61 / 15 / 5 |
| Executed-comparison / unsupported | 0 / 0 |
| Distinct cited Rust test identities / exact passing result-log joins | 19 / 38 |
| Confirmed original inventory non-obligations, outside the 81 | 4 |

These class totals describe the submitted fragments; they are not an accepted
pass tally pending the findings below. Identity, variant and original assertion
accounting is complete apart from the four justified omissions described below.
No declaration or variant should be removed to address them.

## Findings

### SR-1 — Reconcile the traceable Counter helper and the actual read sites

**Priority: P1.** In `service.spec.ts:36` and `:76`, `Foo.increase()` delegates to
`Counter.increase()`. The pinned `utils.ts:52–67` helper registers an effect,
increments during acquisition and decrements during cleanup. The assertions at
`36/default/a61,a66,a72` and `76/default/a97,a101,a106` therefore exercise traced
access and effect lifetime across a consumer's disposal. The with-inject middle
assertion reads through the consumer's `ctx.foo.value` at source line 66; the
without-inject middle assertion reads through `root.foo.value` at line 101.

The sole cited [acceptance test](../../../../../crates/re-cordis-core/tests/acceptance.rs)
at lines 622–667 mutates a `Cell` directly at 654–655. It registers no increment
cleanup, reads the counter only from root at 666, and asserts only the final
value 3. Thus the claimed exact intermediate getter observations and ownership
boundary are not all represented by that citation. The no-failure/explicit
checked-access substitute and the deliberate absence of transparent JS tracing
remain valid; this finding does not reopen those choices.

Correction: retain the existing final-count evidence, explicitly reconcile the
helper path and distinguish the two read sites. The already passing
`bound_nested_calls_keep_definition_authority_but_use_caller_realm_metadata_and_effects`
in [scoping.rs](../../../../../crates/re-cordis-core/tests/scoping.rs), especially
188–200, proves a bound effect survives definition-provider disposal and belongs
to the root use site. It is a useful additional mapping, but does not itself
exercise the two exact counter/getter sequences. Join sufficient existing
assertions with their limits, and split any remaining getter/effect boundary
into a named evidence gap. Do not describe bare Cell mutation as testing the
Counter's reversible effect.

### SR-2 — Match the composed-provider variants to their real shutdown paths

**Priority: P1.** All four `core/service-dispose.spec.ts:238/{false,true}/a266,a267`
records cite `composed_publication_cleanup_retains_resources_on_manual_and_owner_disposal`.
The TS body creates an outer owner with provider and consumer children at
245–260. At 261, `ownerShutdown=false` disposes the provider; `true` disposes the
outer owner. In [service_dispose.rs](../../../../../crates/re-cordis-core/tests/service_dispose.rs)
750–813, provider and consumer are root children, `owner_shutdown=false` manually
disposes the composed effect (803), and `true` disposes the provider (798).
The parameter names are alike but the selected entry points differ.

Correction: the Rust `true` branch supplies provider-disposal evidence for the TS
`false` branch. Preserve the manual-effect test as additional evidence, without
calling it the TS false variant. Find and join existing evidence for outer-owner
shutdown of the composed provider/consumer arrangement, or retain that remaining
boundary as a gap with a precise regression action. The general root-drain and
inherited-child tests reviewed here do not create that composed sibling setup.
This is a variant-mapping correction, not proof that parent shutdown fails.

### SR-3 — Distinguish replacement values and direct versus injected reads

**Priority: P1.** `core/service-dispose.spec.ts:418/default/a440,a441` claim the
consumer resolves the replacement binding and rolls back using the exact old
binding. The TS setup publishes distinguishable versions 1 and 2; its cleanup
reads the retained Context at 426, and the source asserts the current value 2
and `[1,-1,2,-2]` at 440–441.

The cited equal-value regression at Rust 545–604 deliberately uses the same
`Rc(3)` for both publications. Its timeline is built from a local `load` counter
(567–579), and all cleanup/setup reads assert value 3 (578, 585). Distinct
publication IDs at 595 and the timeline at 600 prove distinct loads and rollback
ordering, but cannot detect a read incorrectly resolving the other equal-valued
publication. There is also no post-convergence consumer read that distinguishes
the replacement. Keep those valuable identity/order assertions without calling
them a distinguishing old/new snapshot-value oracle. The withdrawal-before-final-
disposal ordering for `a439` is supported by 597–599.

The related `:355/default/a381` claims a provider direct/current read, while its
citation at Rust 476–536 observes value 2 through the consumer during replacement
setup (505–506, 532). It never reads the provider/root current slot. Its `a380`
withdrawal-before-next-setup-settlement and `a382` old cleanup/new load sequence
are supported by the gates and `[1,1,2]` assertion.

Correction: attach exact existing assertions for the missing read paths and
initialization-time snapshot distinction; otherwise split those boundaries into
named evidence gaps. A different-value old-withdrawal test covers old cleanup
before a replacement load, not the source's in-flight initial setup. Keep these
source scenarios separate when reusing it.

### SR-4 — Bound the cleanup-error source claim to the evidence actually joined

**Priority: P2.** `core/service-dispose.spec.ts:184/default/a195` claims the cleanup
error is reported with its original identity/source. The service-specific test
at Rust 419–467 proves retained-provider cleanup, a final listener delivery and
one failure, but does not inspect that failure's source. The second citation,
`user_failure_sources_survive_and_panics_do_not_skip_other_cleanup` in
[foundations.rs](../../../../../crates/re-cordis-core/tests/foundations.rs)
221–262, downcasts `UserError` and checks field 7 on a root drain report. It does
not assert that the service consumer's logged error is the original allocation.

Correction: retain these separate count/cleanup/typed-source assertions, and
supply an exact source/diagnostic join or classify the untested identity boundary
honestly. Existing exact-pointer logger tests establish explicit ErrorValue
logging, and the accepted failure-parity helpers establish apply-failure identity;
neither is execution of this service-cleanup-error composition. Do not add a new
paired claim or silently turn a value downcast into an identity assertion.

### SR-5 — Make the recorded validation command match its claimed checks

**Priority: P2.** The command recorded in
[report-services.md](ledger-inputs/report-services.md) checks declaration identity,
counts, derived classes, reference bounds/symbol existence, test results/logs,
report links and commit file ownership. It does **not** compare variant parameters
or retained/omitted obligation IDs/assertion provenance with the inventory, and
it does not iterate `restriction` records to check their anchors. Its output
nevertheless claims those checks (including three anchors in the displayed
result). Semantic coverage cannot be established by those bounds checks either.

Correction: record and run the actual checks, then retain output that matches the
command. My independent structural check below confirms the missing mechanical
checks on the present input; it does not erase SR-1–SR-4 or retroactively change
what the worker command checked. Update report totals only after corrections.

## Confirmed non-obligation provenance for the integration owner

At the fixed pin, `service-dispose.spec.ts:123–126` encloses line 125 in
`if (action === 'restart')`. The following four original inventory IDs must be
preserved outside behavioral/gap totals with `execution_status: not-applicable`:

| Original ID | Exact parameters |
|---|---|
| `core/service-dispose.spec.ts:95/provider/a125` | `{"action":"provider"}` |
| `core/service-dispose.spec.ts:95/root/a125` | `{"action":"root"}` |
| `core/service-dispose.spec.ts:95/provider first/a125` | `{"action":"provider first"}` |
| `core/service-dispose.spec.ts:95/consumer first/a125` | `{"action":"consumer first"}` |

For each metadata record retain: original ID; declaration ID
`core/service-dispose.spec.ts:95`; source file
`local/cordis/packages/core/tests/service-dispose.spec.ts`; declaration line 95;
original assertion lines `[125]`; exact parameters above; TypeScript revision
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`; inventory revision
`9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc` and `ledger/cases.json` origin; guard
`action === 'restart'` at line 123 with guarded span 123–126; reason “guard false
for this registered variant”; and this review's guard confirmation. Use a
separate non-obligation disposition, no behavioral evidence class, no gap, and
no Rust pass/paired result. The final schema is the single integration owner's
responsibility. `restart/a125` stays behavioral and is supported by the restart
consumer-ready/available-provider assertions at Rust 265–267.

This is a correction to the old Cartesian assertion expansion, not a new
unsupported behavior. The five variants still exist; the original 85 inventory
records reconcile as 81 current obligations plus these four metadata entries.

## Accepted portions and existing gaps

Outside the findings, the reviewed mappings establish Registry-removed consumer
retention, independent publication withdrawal, five cleanup/restart orders,
transitive cleanup with unrelated progress, both in-flight success/failure
branches, final provider listener delivery, inherited/composed child access,
explicit parent-service injection, both isolation shutdown variants, and both
current/withdrawn release orders. I inspected the `fixture`, `Probe.read`,
`provider`, `gated_consumer`, `bounded`, `Counter`, `dependent`, snapshot helper,
oneshot guards and branch-specific awaited operations, not only the terminal
assertions. For example, the same-label `read` trace in the current/withdrawn test
is disambiguated by waiting for the selected first dependent before releasing
the second; it need not introduce distinct trace labels to prove that order.

The single-use gates and successful cleanup trace supply first-load evidence for
`service.spec.ts:203/a210`; the restart-only final Active state does not establish
the exact second callback count at `a214`. The isolation test's `take().unwrap()`
setup gate, awaited cleanup notification and single owned cleanup similarly
support its activation/cleanup mapping; explicit counters are not universally
required when other assertions distinguish the same failure.

The five existing gaps remain justified: `service.spec.ts:7/a33` (exact activation
count after initialized availability), `:128/a161,a162,a163` (the three initialized
DAG callbacks), and `:203/a214` (exact second apply after restart). I checked the
initialized-provider regression, repeated initialized-service cycles and
`loading_provider_blocks_consumers_and_pending_defers_validation` at
[services.rs](../../../../../crates/re-cordis-core/tests/services.rs):150–229.
The latter counts validation for one two-key consumer, not three separate
initialized service callbacks. Their named follow-up regressions and conditional
B2 routing are appropriate. They remain evidence gaps, not failed tests.

Corrections should reuse sufficient existing tests first, preserve original IDs
when splitting boundaries, and name owner/action/dependent-gate triage for any
remaining gap. No B2 production change is authorized by this review: that still
requires a failing public-API reproducer and an actual dependent gate.

## Actual review checks

CodeGraph exploration and `codegraph node` against the integration checkout
provided the complete assigned TS bodies/helpers and cited Rust bodies/guards.
The exact-path node route was used where broad exploration omitted test bodies.
No source from the independent Rust port was used as evidence.

The following standard-library command was run from the review worktree. It
checks the submitted data and execution joins only; the semantic findings above
come from source review. It also checks the accepted baseline's 71 input hashes
and empty executable delta for applicability, without rerunning its suites.

```sh
python3 -B - <<'PY'
import hashlib, importlib.util, json, re, subprocess
from collections import Counter
from pathlib import Path
root = Path.cwd()
integration = Path('/Users/I756434/Projects/private/cordis-rust')
base = root / 'docs/plans/2026-11-02-cordis-completion/evidence/B0'
files = ['service.json', 'service-dispose.json']
load = lambda p: json.loads(p.read_text())
data = [load(base / 'ledger-inputs' / name) for name in files]
original = {c['id']: c for c in load(base / 'ledger/cases.json')['declarations']}
cases = [c for d in data for c in d['declarations']]
obs = [o for c in cases for v in c['variants'] for o in v['obligations']]
assert len(cases) == len({c['id'] for c in cases}) == 25
assert sum(len(c['variants']) for c in cases) == 35
assert len(obs) == len({o['id'] for o in obs}) == 81
counts = Counter(o['class'] for o in obs)
assert counts == {'source-mapped-test': 61, 'rust-representation': 15, 'missing-evidence': 5}
missing = set()
identity = ('id', 'source_file', 'source_line', 'source_end_line', 'title', 'suite', 'historical_checklist')
for c in cases:
    old = original[c['id']]
    assert all(c[k] == old[k] for k in identity)
    assert [v['parameters'] for v in c['variants']] == [v['parameters'] for v in old['variants']]
    assert c['classes'] == sorted({o['class'] for v in c['variants'] for o in v['obligations']})
    source = (integration / c['source_file']).read_text().splitlines()
    assert 'it' in source[c['source_line'] - 1]
    assert c['title'] in '\n'.join(source[c['source_line'] - 1:c['source_line'] + 2])
    for v, ov in zip(c['variants'], old['variants']):
        old_obs = {o['id']: o for o in ov['obligations']}
        new_obs = {o['id']: o for o in v['obligations']}
        assert not new_obs.keys() - old_obs.keys()
        missing.update(old_obs.keys() - new_obs.keys())
        for o in v['obligations']:
            assert o['source_assertion_lines'] == old_obs[o['id']]['source_assertion_lines']
            assert all(c['source_line'] <= n <= c['source_end_line'] <= len(source) for n in o['source_assertion_lines'])
expected_missing = {f'core/service-dispose.spec.ts:95/{a}/a125' for a in ['provider', 'root', 'provider first', 'consumer first']}
assert missing == expected_missing
spec = importlib.util.spec_from_file_location('docs_check', root / 'tests/docs/check.py')
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
refs, restrictions = set(), set()
for o in obs:
    assert o['behavior'] and o['rationale'] and o['paired_evidence'] == []
    if o['class'] == 'missing-evidence':
        assert o['execution_status'] == 'not-executed' and not o['rust_evidence']
        assert all(o['gap'][k] for k in ('id', 'kind', 'next_action', 'owner', 'work_package'))
        assert 'failing public-API reproducer' in o['gap']['work_package']
    else:
        assert o['execution_status'] == 'passed' and o['rust_evidence'] and o['gap'] is None
    if o['class'] == 'rust-representation':
        assert o['restriction']
    if o['restriction']:
        r = o['restriction']
        assert r['reason'] and r['anchor'] in mod.anchors((root / r['document']).read_text())
        restrictions.add((r['document'], r['anchor']))
    for e in o['rust_evidence']:
        lines = (root / e['file']).read_text().splitlines()
        assert all(1 <= n <= len(lines) for n in e['lines'])
        assert len(re.findall(r'\bfn\s+' + re.escape(e['symbol']) + r'\s*\(', '\n'.join(lines))) == 1
        assert set(e['command_ids']) == {'stable-workspace-test', '1.85.0-workspace-test'}
        refs.add((e['file'], e['symbol']))
assert len(refs) == 19
results = load(base / 'matrix/results.json')
commands = {c['id']: c for c in load(base / 'matrix/commands.json')}
for file, symbol in refs:
    for command in ('stable-workspace-test', '1.85.0-workspace-test'):
        rows = [r for r in results['tests'] if r['command_id'] == command and r['test'] == symbol]
        assert len(rows) == 1
        assert rows[0]['suite'] == file.removeprefix('crates/re-cordis-core/')
        assert rows[0]['outcome'] == 'ok' and not rows[0]['compile_fail']
        assert commands[command]['exit_code'] == 0
        log = (base / 'matrix' / commands[command]['log']).read_text()
        assert re.search(r'^test ' + re.escape(symbol) + r' \.\.\. ok$', log, re.M)
for name in files + ['report-services.md']:
    path = 'docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/' + name
    assert (root / path).read_bytes() == subprocess.check_output(['git', 'show', 'd07d3eec2df3979d5413a0ed7477feebe70efe4c:' + path])
assert not subprocess.check_output(['git', 'diff', '808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f', '--', 'crates', 'tests', 'spikes', 'Cargo.toml', 'Cargo.lock', '.cargo', 'rust-toolchain.toml'])
before = load(base / 'matrix/before.json')
for path, digest in before['executable_input_sha256'].items():
    assert hashlib.sha256((root / path).read_bytes()).hexdigest() == digest
assert subprocess.check_output(['git', '-C', str(integration / 'local/cordis'), 'rev-parse', 'HEAD'], text=True).strip() == '35fdc969e4b6c48c48ed44717c7a018324e4dd88'
assert not subprocess.check_output(['git', '-C', str(integration / 'local/cordis'), 'status', '--porcelain', '--untracked-files=no'])
assert not (integration / 'local/cordis/packages/core/tests/re-cordis-lifecycle-parity.spec.ts').exists()
mod.links(base / 'ledger-inputs/report-services.md')
review = base / 'services-review.md'
if review.exists():
    mod.links(review)
    assert all(line == line.rstrip() for line in review.read_text().splitlines())
print('PASS: 25 declarations / 35 variants / 81 records; submitted classes 61 mapped + 15 representation + 5 gaps')
print('PASS: exact inventory identities/parameters/assertion provenance; only 4 guarded a125 omissions')
print(f'PASS: 19 Rust identities / 38 ok result+log joins; {len(restrictions)} restriction anchors; zero paired claims')
print(f'PASS: d07d3ee inputs identical; executable delta empty; {len(before["executable_input_sha256"])} baseline hashes match; TS clean at pin; fixture absent')
print('PASS: report/review links and review whitespace (when present); semantic verdict remains separate')
PY
```

Result: exit 0.

```text
PASS: 25 declarations / 35 variants / 81 records; submitted classes 61 mapped + 15 representation + 5 gaps
PASS: exact inventory identities/parameters/assertion provenance; only 4 guarded a125 omissions
PASS: 19 Rust identities / 38 ok result+log joins; 3 restriction anchors; zero paired claims
PASS: d07d3ee inputs identical; executable delta empty; 71 baseline hashes match; TS clean at pin; fixture absent
PASS: report/review links and review whitespace (when present); semantic verdict remains separate
```

After writing this file, the recorded command was rerun with the review present,
so its link and direct whitespace checks ran too. `git diff --check` and the
final staged-diff check exited 0. Git confirmed the commit changes only
`docs/plans/2026-11-02-cordis-completion/evidence/B0/services-review.md`.
No Cargo, documentation examples, TypeScript tests or parity run was invoked;
no matrix, original inventory, production/test or tracking file was changed.
Final corrected-ledger assembly, validator/control repair and B0 acceptance
remain separate integration gates.
