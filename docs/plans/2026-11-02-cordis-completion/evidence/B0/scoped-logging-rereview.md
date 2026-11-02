# Scoped-logging correction re-review

Review date: 2026-11-02. Reviewed integration input:
`f4382333ef5f7dd288df0237afeb357c8137c523`. Correction: `218486f`, integrated
as `c462dfd`; comparison baseline: `7fd7cde`. Isolated managed worktree:
`/Users/I756434/.codex/worktrees/b0-scoped-logging-rereview/cordis-rust`.

**Spec verdict: accepted with tracked gaps. Quality verdict: accepted.**
SL-1–SL-6 and the associated rationale/gap-action notes are resolved. No
unresolved correction finding or new breakage caused by these corrections was
found. This accepts the scoped correction, not the unfinished full B0 ledger.

## Scope and evidence reuse

Applied [B0 Tasks 2 and 3](plan.md), the
[master evidence-reuse rule](../../README.md#prior-decisions-and-evidence-reuse),
and the current [execution scope](../../progress.md). Reviewed the delta in the
six fragments and [correction report](ledger-inputs/report-scoped-logging.md)
against every finding and associated note in the
[initial review](scoped-logging-review.md). The pinned semantic source remains
TypeScript `35fdc969e4b6c48c48ed44717c7a018324e4dd88` in integration's clean
`local/cordis` checkout.

The seven correction files have identical Git blobs at `218486f`, `c462dfd`
and the reviewed HEAD. The accepted matrix and its reviews, FR-1, first-two
events, integrated-validation evidence and shared validator are unchanged from
`7fd7cde`. Their accepted scope is reused. Unchanged mappings and the M5/M6
typed representations are not reopened; no repeated full-family audit or test
suite execution was performed.

## Resolution of the six findings

| Finding | Reviewed correction and source basis | Disposition |
|---|---|---|
| SL-1 | `associate:114/a169` now records the successful authorized initial read as missing evidence. TS lines 167–171 inject `bar`, read before setting, then set/read. `acceptance.rs:571–586` initializes 0, asserts unauthorized errors, rebinds, then writes before its first successful read. The proposed regression inserts the missing authorized read at that exact point. The `:235/a242` description now accurately limits `acceptance.rs:706–710` to constructing/formatting X inside one bound callback; `/a247` names that Debug fixture for the absent second forwarding hop. | Closed; accepted typed accessor/Debug representations retained. |
| SL-2 | `isolate:55/a90` now cites `services.rs:435–444`: withdraw the root publication, await settlement, explicitly read `root.get_unchecked` as None. This supports the absent global lookup only; no immediate pre-settlement timing or isolated/surviving-realm observation is inferred. `:106/a145,a146` and `:172/a181,a182,a185,a193` are six explicit gaps for the unproved shared-label reads and own/injected/child/withdrawn-own access paths. Their descriptions agree with TS lines 118–149 and 175–196 and the actual scopes of `scoping.rs:25–103,437–563`. | Closed; exact global lookup reused, six narrower gaps retained. |
| SL-3 | `logger:37/a47,a48,a52` are three gaps. TS uses distinct destination arrays and both individual disposers. `logger.rs:299–325` uses a shared counter and root drain, proving neither survivor identity nor second-token removal. The follow-up names distinct destinations and both handles; the valid counter/drain witness remains supplemental. | Closed; no exporter-identity overclaim remains. |
| SL-4 | `invoke:5/a53` is a gap for mapped stored state affecting invocation. TS lines 51–53 extend config; the unit `Callable` at `scoping.rs:328–333,361–363` changes only its invocation argument. The regression holds arguments equal and changes mapped state while retaining definition authority. | Closed; existing callability/authority evidence retained. |
| SL-5 | `shadow:93/a149,a152` are separate root-entry and injected-consumer-entry gaps for reads through the returned holder. TS calls `create().probe()`; Rust's dependency reads at `scoping.rs:148–174` precede the map/rebind at 175 and identity/metadata checks at 176–180. The next actions require the returned holder for the new reads. | Closed; accepted map/rebind identity representation retained. |
| SL-6 | `intercept:34/a39` and `:42/a49` now carry `rust-representation` and the [M5 service-specific merge restriction](../../../../m5.md#scope-and-bound-calls). The cited closure at `scoping.rs:227–232` and results at 233–236/270–274 prove ordered typed layers. TS `service.ts:53–54` uses `Object.assign`; the revised records explicitly do not claim object-property overwrite from vector concatenation. | Closed; documented typed merge substitute is accurately classified. |

All 28 prior isolation-gap rationale/action updates identify the actual
candidate fixture, its relevant lines and the unproved observation. Checked
the three own-provider/child branches at `:7`, twelve root/fresh-realm
counter/value stages at `:55`, seven shared-label counter stages at `:106`,
and six own/injected/cleanup/diagnostic stages at `:172`. These descriptions
preserve the source-stage distinctions and do not substitute readiness,
root-bypass access or retention for the missing reads. The association
formatting follow-up now points to the correct fixture. No additional source
claim or production change is introduced by these notes.

## Accepted corrected accounting

| Family | Declarations | Variants | Obligations | Representation / mapped / gap |
|---|---:|---:|---:|---|
| associate | 6 | 6 | 24 | 22 / 0 / 2 |
| invoke | 2 | 2 | 11 | 7 / 0 / 4 |
| intercept | 7 | 7 | 8 | 2 / 5 / 1 |
| isolate | 6 | 6 | 53 | 0 / 19 / 34 |
| logger | 11 | 11 | 20 | 13 / 3 / 4 |
| shadow | 8 | 8 | 20 | 15 / 0 / 5 |
| **Total** | **40** | **40** | **136** | **59 / 27 / 50** |

All original declaration and obligation IDs, default parameters, source
assertion text/spans, titles, suites, checklist provenance and fragment
revision metadata are retained. Exactly 46 obligation records changed;
90 are unchanged as data. The 37 prior gap IDs/kinds/owners/routing are
preserved, and the 13 newly explicit gaps are exactly those listed in the
correction report. Parent classes are derived correctly. Missing evidence has
no Rust execution claim; these fragments have no paired-evidence claim.

Fourteen distinct obligation witnesses have 28 exact passing result/log joins,
one on each accepted workspace-test command. Two supplemental witnesses have
four further exact joins, outside obligation/class totals:

- `logger.rs::exporter_removal_is_exact_and_buffer_survives_reusable_root_drains`:
  counter 2→3 after one removal, then no increment after root drain. This cannot
  identify the survivor or prove explicit second-handle removal.
- `scoping.rs::same_name_publications_in_two_realms_keep_resources_until_the_exact_consumer_finishes`:
  two same-name realms, an explicitly injected first-realm consumer, and its
  retained value read during gated cleanup before provider resources close.
  This is not a no-inject child or own-provision lookup.

Together these preserve all 15 original test identities and add the existing
global-withdrawal witness: 16 identities total. Each join checks command ID,
`re-cordis-core`, exact `tests/<file>` suite, exact test symbol and `ok` result,
plus the exact passing raw-log line. Both workspace-test command records have
exit 0. Restriction documents/anchors and cited Rust source locations validate.
These are reused execution results, not new Rust or TS test runs.

All 50 gaps retain owner `core evidence maintainer` and a concrete next action.
A passing regression can close an evidence gap without production changes;
a dependent package gate and failing public-API reproducer are required for
conditional B2 entry. This review establishes neither. Full ledger assembly
and independent B0 acceptance remain integration work.

## Checks actually performed

Used CodeGraph first against integration, then exact `codegraph node` queries
for the affected TS source spans/helpers and Rust fixture bodies. Its broad
initial query included reference-port matches; those were not used as semantic
or execution evidence. Reviewed correction diffs and all changed rationale and
next-action fields. No source, fragment, matrix, validator or tracking file was
edited.

The report's complete recorded shell block was replayed from this isolated
worktree, and its stdout exactly matched the committed result block. The
wrapper below exited 0. It runs only the report's read-only structural/source/
restriction/result/log checks, scoped link checks and whitespace check.
It does not run the unassembled full ledger or any test suite.

~~~sh
python3 -B - <<'PY'
from pathlib import Path
import hashlib, subprocess
p = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs/report-scoped-logging.md')
report = p.read_text()
commands = report.split('~~~sh\n', 1)[1].split('\n~~~', 1)[0]
expected = report.split('~~~text\n', 1)[1].split('\n~~~', 1)[0] + '\n'
run = subprocess.run(['sh'], input=commands, text=True, capture_output=True)
print(run.stdout, end='')
print(run.stderr, end='')
assert run.returncode == 0, run.returncode
assert run.stdout == expected, (run.stdout, expected)
print('OK correction-report exact block replay: exit 0; stdout equals committed recorded results')
print('report_sha256', hashlib.sha256(p.read_bytes()).hexdigest())
PY
~~~

~~~text
OK shared schema/source/reference checks: 136 obligations; 71 executable hashes unchanged; TS pin clean
OK inventory: 40 declarations; 40 variants; 136 original obligations
OK classes: {'rust-representation': 59, 'source-mapped-test': 27, 'missing-evidence': 50}
OK bounded correction: 46 changed obligations; 90 unchanged obligations; 50 distinct gaps
OK Rust tests: 14 distinct obligation witnesses; 28 exact both-toolchain joins
OK supplemental witnesses: 2 original tests; 4 exact joins; excluded from obligation counts
OK command stable-workspace-test exit 0 log logs/stable-workspace-test.log
OK command 1.85.0-workspace-test exit 0 log logs/1.85.0-workspace-test.log
OK links/anchors and direct whitespace: seven owned files
OK correction-report exact block replay: exit 0; stdout equals committed recorded results
report_sha256 8077feaf23df97d421d0f98faab9feeaab71989195e6f9382133ad2ac54286a0
~~~

The excerpt omits only per-family lines and the disposition-summary line;
the wrapper compared the complete output. The replay also verified the 71
executable hashes, clean TS pin, cited Rust byte equality against integration,
and no core-path diff from `808c9f5` or accepted matrix revision `9a7633c`.
The correction report retains its earlier authoring-wrapper and supplemental-
identity-check failures with their resolutions; neither is presented as a Rust
failure. This re-review's verification commands had no failures.

A separate read-only Git-object check exited 0: clean review start at
`f438233`; exactly seven correction paths; equal blobs at original correction,
integrated correction and review HEAD; unchanged accepted evidence/validator
objects from `7fd7cde`. Before commit, the existing docs link helper and direct
newline/trailing-whitespace checks passed for this review, and
`git diff --cached --check` was silent. Only this review file is committed.
No Cargo, Rustdoc, Vitest, parity runner, matrix regeneration, subagents or push.
