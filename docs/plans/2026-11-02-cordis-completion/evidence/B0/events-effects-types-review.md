# B0 independent events/effects/types review

Date: 2026-11-02. **Spec verdict: changes required. Quality verdict: changes
required.** The corrected fragments pass bounded accounting and reference
checks, but several passing evidence rows cover a different branch or omit
the observation attributed to them. These are evidence corrections, not
demonstrated production defects or permission to enter B2.

## Reviewed input and carried evidence

- Review checkout: `7fd7cde471a22dd20e0f015094de2b12b684b032`, isolated at
  `/Users/I756434/.codex/worktrees/b0-events-initial-review/cordis-rust`.
- Assigned implementation: `58117f8d021bcc9e7edd0322d4faebc357dfb27a`, including
  the Rustdoc metadata correction to `d729b8a`. All four assigned input files
  are identical between that commit and the review checkout.
- Inputs: [remaining events](ledger-inputs/events-rest.json),
  [dispose](ledger-inputs/dispose.json), [event types](ledger-inputs/event-types.json)
  and the [report at B0 root](report-events-effects-types.md).
- Specification: integration `local/cordis` at
  `35fdc969e4b6c48c48ed44717c7a018324e4dd88`. The parent verified its clean pin;
  the review read that checkout. The independent Rust port was not evidence.
- Read the [resume instructions](../../progress.md#resume-next),
  [reuse rule](../../README.md#prior-decisions-and-evidence-reuse), historical
  [decisions](../../../../decision.md), [limitations](../../../../limitation.md),
  [implementation plan](../../../../implementation-plan.md), B0
  [Task 2](plan.md#task-2-complete-declaration-and-event-type-disposition), and
  both [assignment](handoffs/events-effects-types-review.md) and
  [common review](handoffs/family-review.md) handoffs. M2's source map and the
  historical checklist supplied the initial candidates; M3/M5/M6 supplied
  plugin ownership, update-router and public-type context.
- Carried without re-audit: first-two events accepted at `11d84b1`;
  decorator/reflect and FR-1 correction/re-review at `59be2db`/`b28a608`;
  matrix/report acceptance at `7d1552a`/`b87c1b9`; recovered validation accepted
  at `e9c0be3`. The seven accepted declarations and their seven open gaps are
  outside this assignment.

The uncovered delta is assertion/branch/type provenance and passing evidence
joins. Typed keys, explicit callback roles, one-use continuations, split
sync/async adapters, owned effects, local hosting and completed M0–M6 decisions
remain in force. No design gate was reopened.

## Coverage and acceptance boundary

Every assigned declaration, its helper-dependent behavior and every proposed
obligation were inspected. There are no parameter expansions in these families.
All 95 original runtime obligation IDs and assertion spans are retained. The
four added empty-dispatch obligations at event lines 105, 147, 166 and 185 are
meaningful and correctly belong in the inventory. Every source `expect` start
is included in an obligation; no non-obligation was identified here.

| Input | Declarations | Variants | Obligations | Draft mapped | Draft representation | Draft gap |
|---|---:|---:|---:|---:|---:|---:|
| Remaining events | 17 | 17 | 60 | 6 | 47 | 7 |
| Dispose | 13 | 13 | 39 | 0 | 34 | 5 |
| Types | — | — | 6 | 0 | 6 | 0 |
| Total | 30 | 30 | 105 | 6 | 87 | 12 |

These are verified **input counts**, not accepted semantic totals. The 24
distinct cited Rust test/Rustdoc items have 48 exact passing command/item joins
in the preserved stable and 1.85.0 results and logs. A passing item is not
evidence for every statement attached to it. There are zero paired-comparison
claims; the accepted six-scenario/73-observation corpus remains unchanged.

The table below accounts for the entire review. `Retain` accepts the bounded
mapping or existing tracked gap; numbered findings identify the changed part,
not a request to repeat the rest of that declaration's audit. IDs below use
`events:L/aN` or `dispose:L/aN` for
`core/<family>.spec.ts:L/default/aN`.

| Source declaration line | Obligations | Disposition |
|---|---:|---|
| events 46 | 4 | Retain prototype-name loop, unregistered emit, removal and final contract release |
| events 65 | 2 | EET-1: partial versus final bucket removal |
| events 77 | 3 | EET-1: repeated emit provenance |
| events 90 | 3 | Retain once recursion, later non-delivery and exhausted removal |
| events 103 | 8 | EET-2/3; retain the empty-path gap and all-settled gate evidence |
| events 134 | 2 | Retain structured aggregate and exact `Symbol(parallel)` label substitute |
| events 145 | 5 | EET-2/3; retain empty emit |
| events 164 | 5 | EET-2; retain empty and callback-error gaps |
| events 183 | 5 | EET-2; retain empty and callback-error gaps |
| events 202 | 8 | EET-5; retain initial composed result and observed outer frame |
| events 230 | 3 | EET-4; retain owned-Next rejection |
| events 244 | 2 | Retain bounded outer-frame/call-order gaps |
| events 265 | 3 | EET-4; retain owned-AsyncNext rejection |
| events 278 | 3 | EET-5; retain nested async result |
| events 296 | 1 | EET-6: count versus config-value trace |
| events 313 | 1 | Retain exact local removal: next update leaves local count unchanged |
| events 330 | 2 | EET-6: local/global count versus config-value trace |
| dispose 7 | 4 | Retain child ownership and the two bounded gaps |
| dispose 23 | 4 | EET-8/9; retain joined sync cleanup and second-claim ownership gap |
| dispose 37 | 4 | EET-8/9; retain transferred tree/reverse cleanup and second-claim gap |
| dispose 76 | 3 | EET-8; retain readiness `[1]`, disposal `[1,2]` |
| dispose 90 | 2 | Retain gated final acquisition/cleanup despite immediate disposal and abandoned waiter |
| dispose 103 | 3 | Retain finite-stream representation: no cleanup through readiness, all three resources cleaned in reverse |
| dispose 123 | 2 | EET-7: both rows must describe the before-first-yield boundary |
| dispose 143 | 4 | EET-8; retain second-in-flight acquisition and final `[1,3,4,2]` |
| dispose 165 | 3 | Retain finite readiness followed by joined reverse cleanup |
| dispose 185 | 2 | EET-3: callback failure details; retain no successful acquisition |
| dispose 197 | 2 | EET-3: callback failure details; retain immediate partial rollback |
| dispose 210 | 3 | Retain async setup rejection and zero cleanup attempts |
| dispose 222 | 3 | EET-8; retain rejection after rollback and final `[1]` |

All six type expressions are acceptable as documented Rust representations:

| TS expression | Rust item and recorded kind | Assessed boundary |
|---|---|---|
| 13 | `events::Event`, Rustdoc line 62, runnable | `Event<u32, String>`, numeric bail argument, string result and uppercase assertion |
| 14 | Same runnable item | Numeric listener contract; Rust explicitly annotates `count: u32`, so this is not evidence of identical TS inference mechanics |
| 17 | `events::Event`, line 55, compile-fail | String argument rejected for numeric event |
| 19 | `events::Listener`, line 162, compile-fail | String listener parameter rejected for numeric event |
| 21 | `typed_rooted_values_dynamic_events_and_context_type_guards_need_no_proxy_reflection`, acceptance.rs 712–737 | Explicit heterogeneous `Vec<hooks::Value>` emit |
| 22 | Same runtime test | Listener downcasts and checks the string and numeric payloads |

The registered TS signature at line 9 is linked by every type record. The
metadata repair correctly distinguishes runnable versus compile-fail items.
The dynamic representation is already documented in
[M6 type checks](../../../../m6.md#executable-example-and-type-checks).
`events.types.ts` itself was not executed by this review or by the B0 matrix.

## Required corrections

All findings are P2 evidence-quality issues owned by the events/effects/types
ledger executor. Correct only the affected records/report, reuse sufficient
existing assertions, and make any remaining boundary an owned gap. Do not add
production code or tests during this B0 correction. Recompute derived classes,
item counts and report totals, then request a scoped independent re-review.

### EET-1 — Preserve partial removal and repeated dispatch distinctions

`events:65/a72` asserts that a symbol bucket survives removing the first of two
listeners. Its citation, `symbols_named_contracts_prototype_names_and_empty_bucket_removal`
in [events.rs](../../../../../crates/re-cordis-core/tests/events.rs), lines
71–93, uses one named listener, observes a contract mismatch **before any
removal**, then removes the last listener and changes the contract. The final
release evidence is useful; it does not exercise the intermediate removal.
Retain that distinction as a gap unless another assertion covers it.

`events:77/a84` claims a second `emit` delivery, but its cited lines 39–83
contain one emit followed by bail/serial/parallel, then emit only after removal.
The `[1,0,2,3]` assertion does not assert two emits on a surviving listener.
Reuse `removal_after_selection_does_not_cancel_in_flight_snapshots` (lines
293–340: the surviving first listener appears in both emit traces) or the
emit branch of `prepend_and_global_filters_apply_to_each_dispatch_shape`, with
an exact explanation and both-toolchain join. Preserve the original row IDs.

### EET-2 — Cite the actual mode/filter and immediate-prefix assertions

`events:145/a152`, `/a154`, `/a156` all cite
`synchronous_emit_failure_stops_later_callbacks`, lines 521–545. That body has
no filter or successful callback-count observation. The existing
`prepend_and_global_filters_apply_to_each_dispatch_shape` **emit** branch and
trace assertions at 237–254 provide the relevant positive/negative selection
evidence; remap these rows.

Also narrow the shared assertion text used for `events:103/a110`,
`events:164/a171` and `events:183/a190`: the mode loop dispatches each mode
**with** a filter, but lines 250–254 use only `emit` for the unfiltered check.
It is false that this body itself tests unfiltered delivery for every mode.
The named-key test has unfiltered parallel/serial/bail evidence at 55–70;
join the required candidate instead of attributing it to the filter loop.

Serial source lines 170–175 additionally inspect callback counts without
awaiting the returned task. The cited filtered serial dispatch is awaited.
The existing `serial_has_an_immediate_first_prefix_and_stops_on_zero_or_empty_answers`
at 416–450 asserts the first factory prefix before awaiting its task. Reconcile
that timing evidence with the explicit eager-prefix representation; do not
claim the awaited filter loop alone verifies the immediate observations.

### EET-3 — Error shape/count is not the asserted error content

- `events:103/a128` requires the named event label; the cited parallel test
  uses a symbol and asserts `Symbol(parallel)` at 400. `events:103/a129`
  requires both original error messages; lines 397–402 check only three
  failures and the third panic flag. They do not inspect the first two sources
  or messages. Retain aggregate/all-settled evidence, but split or gap the
  unobserved named-label and callback-content boundaries.
- `events:145/a161` expects the callback's message. The cited emit test checks
  only `EventError::Callbacks { .. }` at 541–544. Its `error("stop")` input is
  not an output assertion. The source-preservation gap must be handled with
  the same precision already used for serial/bail's error gaps.
- `dispose:185/a188` and `dispose:197/a200` include `.to.throw('test')`.
  The former maps to a panic-producing factory and a generic `SetupError`
  match at effects.rs 243–247; the latter maps to generic callback matches at
  effect_metadata.rs 187/204. Neither checks the callback error message.
  Keep existing panic/Result and rollback representations, while separating
  the unverified returned-error content. Also correct the former's assertion
  text claiming one retained failure: that body asserts **two** at 247 after
  its earlier iterator-error branch.

This asks for the source's observed error details, not a new error architecture
or an assertion of exact cross-language object allocation identity.

### EET-4 — Compile-fail items cannot attest runtime counts or results

`events:230/a240`, `/a241` and `events:265/a274`, `/a275` cite only the Next or
AsyncNext compile-fail items at [events.rs](../../../../../crates/re-cordis-core/src/events.rs)
250 and 263. Those functions do not run and contain no callback/terminal
counter or numeric-result assertion. In particular, the AsyncNext snippet
does not await anything: it statically rejects moving the handle twice.

Retain `/a239` and `/a270` as one-use ownership rejection substitutes. For the
four runtime rows, attach sufficient runnable single-use substitute evidence
and explain the boundary, or record the missing evidence. Do not reinterpret
compile-fail success as a runtime count of one or a result of three. The report
and classes must preserve this separation even though the metadata joins pass.

### EET-5 — Preserve waterfall topology and counted observations

The initial numeric result in `events:202/a209` and outer before/after trace
have evidence in `waterfall_is_around_middleware_with_veto_and_nested_async_calls`,
lines 552–584. However `/a211`'s inner exact count is not observed. For
`/a219`–`/a223`, TS installs two forwarding listeners, a third veto, and a
fourth listener whose count must stay zero. Rust removes its initial inner
listener, then tests one forwarding outer plus a veto and a panic terminal
(585–598). It proves outer result composition and terminal suppression, but
has no fourth listener and no counts for that source branch.

Similarly `events:278/a291` and `/a292` claim two callback and two terminal
calls. Lines 599–638 assert a boolean prefix and result four, with constant
terminal closures; neither call count is measured. Retain the nested result
row, but split/remap the unobserved counts and topology. The snapshot-removal
test at 683–811 measures a selected inner call in another scenario; it does
not close the four-listener veto or nested-call count obligations by itself.

### EET-6 — Update hook counts do not verify the config-value sequence

`events:296/a310` and `events:330/a349`, `/a350` assert ordered config values
`[1,2,3]` or `[1,2]`. Their shared
`local_update_hooks_do_not_accumulate_and_global_hooks_survive_child_reload`
body in [updates.rs](../../../../../crates/re-cordis-core/tests/updates.rs)
424–471 ignores hook payloads (`_`, `_: Rc<u32>`) and asserts only total counts
of five. It is sufficient non-accumulation/global-lifetime evidence, not an
assertion that the hooks received the current resolved values in order.

Preserve that valid count evidence and reconcile the payload/ordering portion
with the existing typed-update tests, or split out an owned gap. Do not infer
the observed hook values from the arguments passed to `fiber.update`.

### EET-7 — Both early-abort rows share the uncovered first-yield boundary

`dispose:123/a138` correctly records the missing before-first-yield case, but
`/a140` is marked passed using
`async_stream_stops_before_next_step_but_collects_in_flight_final_yield`,
[effects.rs](../../../../../crates/re-cordis-core/tests/effects.rs) 129–175.
That test waits for the **second** step at 164, disposes at 168, and asserts
`[1,3,4,2]` at 171. TS aborts before the first yield and asserts `[1,2]`.
The extra acquired resource is the behavioral distinction under review.

Extend the existing early-abort follow-up to cover `/a140` as well, or supply
a genuinely before-first-yield assertion. Keep the existing mapping for
dispose line 143's second-in-flight case; do not duplicate that accepted work.

### EET-8 — Separate immediate observations from later joined outcomes

These rows cite a later state but claim an immediate pre-cleanup/pre-readiness
observation that the cited body never takes:

| Obligation | Cited evidence's actual first relevant observation |
|---|---|
| `dispose:23/a30` | effect_metadata.rs 55–62 observes trace `[1]` after manual disposal, not cleanup count zero beforehand |
| `dispose:37/a69` | effect_metadata.rs 154–155 observes `[3,2,1]` after group disposal |
| `dispose:76/a83` | effects.rs 330–331 awaits readiness and then observes acquisition `[1]` |
| `dispose:143/a156` | effects.rs 164–171 waits for the second step, disposes, then observes the final trace |
| `dispose:222/a230` | effects.rs 210–212 awaits failed readiness, then observes rollback `[1]` |

Existing evidence can avoid some new follow-ups: `sync_setup_is_immediate_and_nested_effects_and_listeners_transfer_ownership`
in effects.rs 27–78 checks `[0]` before disposal (only the setup marker, no
cleanup markers). It is a useful candidate for the composed no-early-cleanup
claim, but must actually be joined and its async disposer representation
distinguished from the synchronous timing test. For the remaining immediate
boundaries, provide an exact assertion or a bounded gap rather than copying
the later outcome's explanation. Keep the already proved joined outcomes.

This finding does not require artificial timestamp parity: gates and explicit
Rust scheduling adapters remain the selected representation. The finite-stream
test's empty cleanup trace **through readiness** and reverse cleanup are valid;
they do not supply acquisition-side immediate observations for a different test.

### EET-9 — A renamed metadata tree does not test the default label

`dispose:23/a27` expects a default `anonymous` label, but its cited test calls
`.with_label("resource")` before its first snapshot, then renames to `renamed`
(effect_metadata.rs 40–54). `dispose:37/a53` similarly expects default
`anonymous` outer/nested nodes, while its citation labels them `outer` and
`nested` (84–153). Keep the valid metadata shape, transferred-listener labels
and ownership assertions; split or remap the unobserved default-label portion.
Do not claim exact default metadata solely because explicit naming works.

## Existing gaps and follow-up boundary

The twelve existing gap IDs are retained, subject to EET-7 extending the
early-abort disposition. Their concrete actions, core evidence maintainer owner
and conditional B2 routing are sound. The three empty modes were checked
against the current events tests and the other direct dispatch sites in
hooks.rs; those candidates register responders first. Serial/bail source-error
checks are absent from those candidate tests. Next/AsyncNext's existing
compile-fail snippets cover direct double moves, not the named outer-frame
capture/order case. No Effect double-claim compile-fail item is currently cited
or present in its public source.

For `dispose:7/a20`, the alternative
`cleanup_failures_are_attempted_and_reported_without_blocking_disposal`
(plugins.rs 917–949) does issue a second Fiber disposal and observes no new
failures. It checks the successful cleanup count only **before** that second
request (942), so it does not close the exact repeated-cleanup-count gap.
Use that existing candidate to keep a later regression minimal. The child
ownership test's journal lengths do not close its exact named-child metadata
gap either. These are bounded missing assertions, not missing core features.

No production reproducer was executed or demonstrated. A passing follow-up
can close an evidence gap without production changes; only a failing public-API
reproducer plus an actual dependent gate admits B2. Correction/re-review of
this family is required before final B0 ledger acceptance. No final B0 verdict
or downstream package entry is implied here.

## Actual bounded checks

CodeGraph was used first against the integration project for the exact TS and
Rust paths. Its omitted test bodies/assertion ranges were read directly where
needed. Source review covered events.spec.ts 46–353, all dispose.spec.ts,
events.types.ts, Session/Filter and timer helpers, the event resolver/update
router, cited Rust bodies/Rustdoc, and the named alternative candidates above.

A standard-library Python check loaded only these fragments and the preserved
two workspace command results/logs. It compared declaration identity/title/
suite/span/checklist fields to the original inventory, retained assertion IDs
and spans, all default variants, every source `expect` start, four added empty
paths, derived classes and the six exact type lines/signature. It invoked the
existing `validate_obligation` for all 105 records with that bounded context,
checking restrictions/anchors, gap fields, source/Rust bounds and exact
file/suite/symbol/Rustdoc-kind/result/log joins. It did not call the full-ledger
validator or rerun recovered integrated validation.

The exact reproducible command is below; exit **0**. Run from the review
checkout. The helper is used only for the assigned records, with an explicitly
constructed two-command context.

```sh
python3 -B - <<'PY'
import importlib.util, json, re
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
assert len(rows) == 105 and len(ids) == 105 and len(gaps) == 12 and len(rust) == 24 and not paired
print('types', len(types), dict(Counter(o['class'] for o in types)))
print(f'declarations={total_declarations} variants={total_variants} obligations={len(rows)} Rust_items={len(rust)} passing_toolchain_joins={len(rust)*2} gaps={len(gaps)} errors={len(errors)}')
print('retained_original_runtime_obligations=', len(rows)-len(types)-len(added), 'added=', added)
PY
```

Output:

```text
events-rest 17 60 {'rust-representation': 47, 'source-mapped-test': 6, 'missing-evidence': 7}
dispose 13 39 {'missing-evidence': 5, 'rust-representation': 34}
types 6 {'rust-representation': 6}
declarations=30 variants=30 obligations=105 Rust_items=24 passing_toolchain_joins=48 gaps=12 errors=0
retained_original_runtime_obligations= 95 added= ['core/events.spec.ts:103/default/empty-105', 'core/events.spec.ts:145/default/empty-147', 'core/events.spec.ts:164/default/empty-166', 'core/events.spec.ts:183/default/empty-185']
```

The executable-input applicability and exact fragment-transfer comparisons
were run under `subprocess.run(..., check=True)` so each exit was checked:

```sh
git diff --quiet 9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc HEAD -- crates Cargo.toml Cargo.lock rust-toolchain.toml tests .github
git diff --quiet 58117f8d021bcc9e7edd0322d4faebc357dfb27a HEAD -- docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/events-rest.json docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/dispose.json docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/event-types.json docs/plans/2026-11-02-cordis-completion/evidence/B0/report-events-effects-types.md
```

Both exited **0**. This preserves the accepted matrix's applicability; the
control revision is not claimed as its execution HEAD. No Cargo, documentation
example, upstream test or parity command was rerun. Structural validation is
not a semantic acceptance result.

The final document/link/whitespace command was:

```sh
PYTHONPATH=tests/docs python3 -B - <<'PY'
from pathlib import Path
import check
path = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/events-effects-types-review.md').resolve()
check.links(path)
lines = path.read_text().splitlines()
assert all(line == line.rstrip() for line in lines)
assert not any('\t' in line for line in lines)
print('review links/anchors and direct whitespace: passed')
PY
git diff --cached --check
```

Both exited **0**; the Python output was
`review links/anchors and direct whitespace: passed`. The staged file list
contained only this review document. No other tracked file was changed.
