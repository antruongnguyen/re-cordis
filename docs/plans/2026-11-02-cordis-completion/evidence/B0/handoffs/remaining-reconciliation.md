# Remaining B0 evidence reconciliation

Executor: gpt-5.6-sol/high, explicitly authorized fallback (progress D008).
Use a fresh isolated managed worktree from the integration checkout:
/Users/I756434/Projects/private/cordis-rust. No agents, shared-file edits,
production/test/manifest edits, Cargo/docs-example/parity reruns or push.
The dispatch names ONE assignment below. Do not take another assignment.

Read AGENTS.md, B0 plan Task 2, the master reuse rule, docs/decision.md,
docs/limitation.md and docs/implementation-plan.md. These documents were
already reconciled: completed A–C/M0–M6 decisions remain valid, and old
unresolved headings do not reopen them. Use the existing family source maps
in docs/m2.md through docs/m6.md and docs/m0-cases.md as the starting evidence.
Read only the relevant pinned TS and Rust bodies/helpers needed to attach
exact missing provenance or resolve contradictions. The TS checkout under
integration local/cordis at 35fdc969e4b6c48c48ed44717c7a018324e4dd88 is the
specification; local/cordis-rs is not. Use CodeGraph first for indexed code,
with projectPath naming integration. Do not index or modify upstream.

Carried evidence: accepted matrix at 7d1552a, report re-review b87c1b9,
decorator/reflect FR-1 correction 59be2db and review b28a608, recovered
integrated validation accepted at e9c0be3. Do not repeat those audits.
All seven accepted fragment gaps stay open. The uncovered delta is finer
assertion/variant/type provenance and both-toolchain execution joins, not
missing implementations. Reuse sufficient mappings and existing restrictions.

## Exclusive assignments

All paths are under this B0 evidence directory in your worktree. Copy only
assigned records from ledger/cases.json; save each completed family promptly.

| Assignment | Declaration scope | Owned outputs |
|---|---|---|
| events-effects-types | 17 remaining events declarations, excluding lines 18 and 35; dispose 13; six events.types.ts expressions | ledger-inputs/events-rest.json, dispose.json, event-types.json, report-events-effects-types.md |
| lifecycle-update | plugin 10, fiber 22, update 10 declarations / 12 variants | ledger-inputs/plugin.json, fiber.json, update.json, report-lifecycle-update.md |
| services | service 8, service-dispose 17 declarations / 27 variants | ledger-inputs/service.json, service-dispose.json, report-services.md |
| scoped-logging | associate 6, invoke 2, intercept 7, isolate 6, logger 11, shadow 8 | ledger-inputs/associate.json, invoke.json, intercept.json, isolate.json, logger.json, shadow.json, report-scoped-logging.md |

Only the five decorator/reflect records and first-two events are excluded.
No edits to their accepted outputs, ledger/cases.json, ledger/report.md,
validator, progress, other reports or tracking documents.

## Required record and evidence quality

Follow the accepted decorator-reflect.json schema. Preserve declaration IDs,
source lines/titles/suites/checklist provenance, exact variant parameters and
original obligation IDs/assertion lines. Split/add meaningful obligations if
the inventory omitted a path, while retaining its original provenance. Parent
classes are derived from obligations. Never leave inventory placeholders or
pending-matrix links in a completed family.

Each obligation needs a concrete source behavior, exact current Rust assertion
file/symbol/lines and explanation, existing restriction where appropriate,
and both stable/MSRV command IDs whose actual results AND logs contain that
test as passed. Rustdoc evidence must join item/file/line and compile-fail
versus runnable result correctly. Inspect helpers, parameters and branch
conditions; test-name similarity or a compiled file is not sufficient.
Only six/73 paired scenarios are executed comparisons; use those only for
the exact exercised obligation. Source-mapped tests do not execute TS.

If evidence remains insufficient after checking the applicable prior map and
candidate assertions, state the concrete uncovered boundary and a specific
regression/decision next action, owner and dependency triage. A passing new
regression can close evidence without production change. Only a failing
public-API reproducer plus an actual dependent gate opens B2. Do not fabricate
blanket exclusions or turn all unjoined records into generic missing evidence.

Type output schema: top-level schema_version, rust_revision, typescript_revision,
status and obligations (six). Each obligation uses the ordinary evidence
fields plus source_file, source_line, source_signature_line=9 and
source_assertion_lines=[source_line]. IDs are core/events.types.ts:13, :14,
:17, :19, :21 and :22. Explain typed-symbol/result/listener positives,
wrong dispatch/listener argument rejection and dynamic heterogeneous Values
separately. Inventory is not a new TS type-test execution.

## Checks and handoff

Verify the assigned identities/counts/variants, meaningful fields and derived
classes, actual source and Rust reference bounds, links/anchors, exact passing
test/log joins on both toolchains, and executable-input applicability. Use
standard-library scripts and the existing docs link helper. Record exact
commands, exits, compact results, revisions and limits in the owned report;
no invented test passes. The final full validator/review belongs to integration.

Commit only the owned completed files after self-review, with no
Co-authored-by trailer. Return worktree/commit, declaration/variant/obligation
counts separately, class totals and any concrete gaps. A readable complete
handoff is required; preserve partial output if transport fails.
