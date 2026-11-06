# Scoped-logging independent review

Reviewed input: be0a08b4b7c22525c13a7ae464921e045161b515 (data from
87b41ce; report command-provenance correction from be0a08b).
Review date: 2026-11-02. Assignment: scoped-logging only.

**Spec verdict: changes required. Quality verdict: changes required.**
The inventory and execution joins check out, but some obligations claim an
observation that their cited assertions do not establish. These are B0 ledger
findings, not demonstrated production failures. No B2 admission, full-B0
acceptance, or reversal of the accepted M5/M6 representations follows.

This bounded review is finished. Corrections and their re-review remain with
the integration owner after the user resumes. No correction cycle was started.

## Scope and accepted accounting

Reviewed all six assigned fragments and the
[executor report](ledger-inputs/report-scoped-logging.md), all 40 pinned TS
declarations and their helpers, and the complete bodies of the 15 cited Rust
tests. The accepted decorator/reflect records, first-two events records,
independent reviews, and six-scenario/73-observation matrix were carried inputs.
They were not reopened.

| Family | Declarations | Variants | Obligations | Submitted representation / mapped / gap |
|---|---:|---:|---:|---|
| associate | 6 | 6 | 24 | 23 / 0 / 1 |
| invoke | 2 | 2 | 11 | 8 / 0 / 3 |
| intercept | 7 | 7 | 8 | 0 / 7 / 1 |
| isolate | 6 | 6 | 53 | 0 / 25 / 28 |
| logger | 11 | 11 | 20 | 13 / 6 / 1 |
| shadow | 8 | 8 | 20 | 17 / 0 / 3 |
| Total | 40 | 40 | 136 | 61 / 38 / 37 |

These are verified submitted counts, not accepted semantic class totals.
Corrections may change classes or split mixed obligations. All variants are
default/nonparameterized. All 136 source assertion starts are represented;
multiline expectation spans are preserved. Declaration IDs, titles, suites,
source spans, checklist provenance, original obligation IDs and parameters
match the carried inventory.

The 15 distinct cited tests have 30 exact passing joins: one per test on each
of stable-workspace-test and 1.85.0-workspace-test. Both command records have
exit 0; both accepted raw logs contain each exact test name with outcome ok.
There are no Rustdoc evidence references or paired-evidence claims in these
fragments. Source-mapped Rust results do not execute their TS counterparts.

## Findings

### SL-1 — P1: authorized unset accessor read is mapped to the opposite authority state

[associate.json](ledger-inputs/associate.json), line 655,
core/associate.spec.ts:114/default/a169, says the read is before associated
authority is injected and cites acceptance.rs:555–575 errors. The TS assertion
at associate.spec.ts:169 is inside ctx.inject(['bar']) at line 167: authorized
access succeeds and returns the receiver's initially unset value. The earlier
assertion at line 165 is the unauthorized throwing case. The fragment assigns
both to the same unauthorized-error evidence.

The Rust fixture creates Session { secret: Cell::new(0) } at
[acceptance.rs](../../../../../crates/re-cordis-core/tests/acceptance.rs):571–575,
then rebinds under the required authority at 576–584. It writes 100 before its
first successful read at 585–586; it never asserts the authorized initial
value. Correct the authority/timing rationale and either join a successful
initial typed-value assertion or record that narrower evidence gap. Ordinary
Rust field/default representation remains accepted; it does not turn a
successful authorized read into an unauthorized error.

### SL-2 — P1: isolation mappings replace exact lookup outcomes and own-access paths with other observations

[isolate.json](ledger-inputs/isolate.json), lines 462, 1022, 1060 and 1425,
marks :55/a90, :106/a145, :106/a146 and :172/a193 source-mapped. All cite
scoping.rs:75–96. That fixture withdraws a shared publication and asserts only
consumer.ready() == Pending at 92–96. It never reads either isolated view after
withdrawal, has no populated root fallback during the withdrawal, and does
not assert ServiceError::Undeclared. Pending cannot substantiate either an
absent direct value or the checked own-provision access error at TS line 193.

There are also mismatched access paths in :172/a181, /a182 and /a185
(fragment lines 1267, 1296, 1345). TS lines 175–188 exercise an injected
consumer that itself provides the same name in two isolated realms and
children that inherit its own provisions without declaring inject. The first
two mappings cite root reads in scoping.rs:53–57; /a185 cites an explicitly
injected consumer's retained read during provider cleanup at 519–532. Neither
assertion exercises the claimed own-versus-injected view or own-provision
inheritance path. The existing sibling gaps /a183 and /a188 correctly call out
missing analogous observations, making the covered/gap distinction inconsistent.

Use exact existing post-withdrawal read evidence where available (for example,
services.rs:435–445 explicitly reads the withdrawn global publication), with
its real scope; retain a gap for an unproved isolated/authority distinction.
Split the own/injected/child paths where necessary. Do not promote a final
Fiber state or a root-bypass read into a checked lookup assertion.

### SL-3 — P1: exporter-removal evidence cannot identify the surviving exporter

[logger.json](ledger-inputs/logger.json), lines 246–359, claims all three
obligations at core/logger.spec.ts:37 are source-mapped to
[logger.rs](../../../../../crates/re-cordis-core/tests/logger.rs):309–325.
The Rust fixture registers two wrappers around the same callback and both
increment one shared counter (299–317). After first.remove(), observing the
counter rise from 2 to 3 proves that one exporter survived; removing the wrong
exporter would produce the same result. TS lines 40–48 use separate arrays
and specifically prove that the first stopped and the second survived.

The TS second-disposer assertion at line 52 also invokes disposeSecond()
explicitly, whereas the cited Rust sequence removes the remaining registrations
through ctx.drain() at 323. Keep the tested aggregate-count/root-drain behavior
as scoped evidence, but leave exact identity and second-token removal unproved
unless another existing assertion supplies them. A targeted follow-up needs
distinct destinations and both individual removal handles.

### SL-4 — P2: callable extension state is confused with invocation arguments

[invoke.json](ledger-inputs/invoke.json), line 182,
core/invoke.spec.ts:5/default/a53, claims TS extension state { a: 1, c: 3 }
is covered by scoping.rs:361–363. The Rust Callable is a unit struct, and
bound.map(|_| Callable) supplies the same stateless value. The result changes
from 8 to 9 because invoke's argument changes from 1 to 2; the helper at
328–333 reads dependency 7 and adds that argument. A map implementation that
ignored changed extension state would not be challenged by this assertion.

The three reported invocation/intercept composition gaps are justified, but
this separate extension-state boundary is not covered. Retain evidence for
callability and definition authority, and split or reclassify the configured
extension-state observation. This does not request JS callable-object syntax.

### SL-5 — P2: captured-holder dependency reads are attributed to calls before the holder exists

[shadow.json](ledger-inputs/shadow.json), lines 403 and 483,
core/shadow.spec.ts:93/default/a149 and /a152, claim that a captured/retraced
holder resolves its dependencies. The cited nested read at scoping.rs:148–174
runs on the original bound service. The holder-like map/rebind occurs later
at 175; only definition identity and metadata are inspected at 176–180. There
is no dependency read through retraced, and no injected-consumer entry into
that holder path. TS lines 149 and 152 both call create().probe() on the
returned holder, with root and injected-consumer entry respectively.

Keep map/rebind identity evidence for the representation, but do not describe
pre-map reads as post-map reads. Join the actual holder read if it exists or
record the precise missing root/consumer holder observations. The accepted
explicit map/rebind substitute itself remains unchanged.

### SL-6 — P2: shallow-merge obligations need the accepted typed-merge restriction

[intercept.json](ledger-inputs/intercept.json), lines 46 and 102,
:34/a39 and :42/a49, have source-mapped-test with restriction null. TS
intercept.spec.ts:38–49 deliberately includes colliding b values, and
Service.mergeIntercept in TS service.ts:53–54 uses Object.assign. The Rust
fixture's merge closure at scoping.rs:227–232 only concatenates vectors;
274 and 233–236 establish layer ordering, not overlapping-property overwrite.

The existing [M5 contract](../../../../m5.md#scope-and-bound-calls) explicitly
makes merging service-specific in Rust. Carry that documented representation
into these obligations and state that the tested substitute supplies ordered
layers to a caller-provided merge. If claiming the concrete object overwrite
result, join an actual overwrite assertion or split that unproved boundary.
No new default object-merge implementation is requested.

## Review of the submitted 37 gaps

I checked each submitted gap against the cited/prior family fixtures rather
than treating a missing similarly named test as sufficient. The following
observations are absent from those existing assertions. This preserves their
evidence status; the mappings above require additional correction before any
final gap/class totals are accepted.

| Submitted gap IDs (G-B0- prefix) | Candidate evidence and uncovered observation |
|---|---|
| associate-235-a247 | acceptance.rs:706–710 formats X inside one bound callback; no second nested forwarding call exists. Also correct the /a242 rationale: X is constructed/referenced inside the callback, not passed through an argument adapter. The named follow-up fixture is the association test, which currently contains no Debug path; point to the actual formatting fixture or explain the intended extension. |
| invoke-5-a46, a56, a59 | scoping.rs:328–363 has dependency arithmetic and map callability; no use-site intercept is composed into a captured callable or configured extension. |
| intercept-34-a37 | scoping.rs:233–274 always retains at least outer/inner layers; it has no zero-layer resolve_intercept result. |
| isolate-7-a13, a15, a18 | scoping.rs:40–51 checks inherited injection cut by boundaries and 53–57 uses root ownership; neither tests an own isolated provision viewed from its same-Fiber outer context and both child branches. |
| isolate-55-a78, a79, a86, a87, a94, a95, a102, a103 | scoping.rs has no three-realm callback/disposer counters for these successive publication/withdrawal steps. A single consumer becoming Active/Pending is insufficient. |
| isolate-55-a91, a98, a99, a100 | The root/isolated initial pair at scoping.rs:53–57 does not assert values surviving withdrawal of the root slot and later publication into a third slot. |
| isolate-106-a125, a132, a133, a140, a141, a148, a149 | scoping.rs:65–103 has one alias consumer, not two shared-label consumers plus the root consumer; it does not assert the requested activation/disposal counts. |
| isolate-172-a183, a188, a192, a194, a196, a200 | scoping.rs:437–479 exercises cleanup completion after injecting and providing another realm but performs no corresponding reads or diagnostic-count assertion. The two-realm retention fixture at 483–563 reads only its first injected realm during cleanup; it does not assert the second own realm, outer injected value after withdrawing only one own realm, or the final zero-error count. |
| logger-156-a162 | logger.rs:134–141 checks the empty aggregate's count/message; exact pointer identity at 151–155 is only for a nonempty aggregate member. |
| shadow-157-a191 | scoping.rs:173–174 asserts first-definition value plus second use realm; there is no direct second-definition value/second-realm pair. |
| shadow-313-a345, a346 | scoping.rs:181–187 checks a shadow-free empty child and denied undeclared read. It does not load a service and a successful injecting child through the bound loader context or assert zero diagnostics for that composition. |

The gap owner is present in all 37 records. Each next action is concrete about
the requested source observation, although isolation entries should identify
the particular fixture and candidate lines instead of relying on the repeated
generic rationale. The shared routing text correctly says a passing regression
can close evidence without production changes, and requires an actual dependent
gate plus failing public-API reproducer before B2. No such reproducer or gate
was established by this review.

## Actual checks and limits

- Read the assignment, common family-review and remaining-reconciliation
  handoffs, B0 Task 2, master reuse rule, and relevant M5/M6 decisions,
  limitations and previous-plan maps.
- Used CodeGraph first against the integration checkout. Its broad exploration
  trimmed sections, so codegraph node supplied complete assigned TS files and
  Rust fixture bodies. Read TS context.ts, reflect.ts, service.ts, logger.ts
  and utils.ts to resolve helper/authority/merge/forwarding behavior.
- Independently ran a Python standard-library inventory/reference/result/log
  join check from the isolated review worktree. First attempt exited 1 because
  the reviewer check compared multiline assertion spans to expectation starts;
  corrected it to compare each obligation's first assertion line. The corrected
  check exited 0 with the output below. This was a checker correction, not an
  input edit or a test failure.
- Verified byte identity of each cited Rust file against integration. git diff
  --name-only from both 808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f and accepted
  matrix revision 9a7633c0b3b98e5f37e9f2211f3ffa67fdac5edc to the reviewed HEAD,
  restricted to crates/re-cordis-core, returned no files (exit 0).
- Verified pinned TS HEAD 35fdc969e4b6c48c48ed44717c7a018324e4dd88 and clean
  upstream status. Review worktree was clean at the specified input commit
  before writing this report.
- The read-only checker validated source titles/identity/spans, original
  obligation IDs/default variants, derived parent classes, required fields,
  restriction file/anchor existence, Rust source bounds/symbol presence,
  executable-input identity, both command exits, crate/suite/test/result rows,
  exact raw-log lines, distinct gaps, and absence of pending-matrix.
- No Cargo, docs examples, parity runner, Vitest, baseline regeneration,
  production/test/manifest/input edits, subagents, or push. Only this review
  file is committed. Existing accepted six/73 execution remains outside this
  assignment's new evidence.

~~~text
{'declarations': 40, 'variants': 40, 'obligations': 136, 'rust-representation': 61, 'missing-evidence': 37, 'source-mapped-test': 38}
distinct_tests 15 both_toolchain_joins 30
stable-workspace-test exit 0 log logs/stable-workspace-test.log
1.85.0-workspace-test exit 0 log logs/1.85.0-workspace-test.log
~~~

Report links and whitespace are checked separately before commit using the
existing docs link helper and git diff --cached --check. The full merged
ledger validator and correction review remain integration work.
