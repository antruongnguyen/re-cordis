# Correct events fragment Rustdoc join metadata

Executor: gpt-5.6-sol/high fallback, continuing the healthy events-effects-types
task in /Users/I756434/.codex/worktrees/b0-events-effects-types/cordis-rust at
d729b8a. Keep the original four-file ownership; no unrelated edits or agents.

The integration owner ran the committed validator's validate_obligation()
on all 105 obligations using the real matrix context. It failed with 20
`no unique matching passing result` errors: both toolchains for types :13,
:14, :17, :19 and events :230/a239,a240,a241 and :265/a270,a274,a275.
The issue is friendly Rustdoc labels in rust_evidence.symbol; the fragment's
local validator translates them through a private alias table, while the
shared schema expects the actual Rustdoc item identity.

Normalize these metadata records to exact items such as events::Event,
events::Listener, events::Next and events::AsyncNext, with rustdoc_line and
compile_fail where needed. Inspect the actual recorded matrix identities
and Rustdoc source. Preserve semantic classifications and existing source
provenance; do not invent new passes or weaken the shared validator.

Use the integration checkout's ledger/validate.py public helpers to check
all obligation records (this is a partial-schema check, not full-ledger
acceptance). Its path is under
/Users/I756434/Projects/private/cordis-rust/docs/plans/2026-11-02-cordis-completion/evidence/B0.
Re-run your exact bounded checks too. Record the original integration failure
and the corrected exact commands/results in report-events-effects-types.md
at B0 root. The report's 9a7633 revision is ledger control/evidence provenance,
not the actual matrix execution HEAD; name the recorded 578c066..e23dbac
execution endpoints accurately.

Check links/whitespace, commit only changed owned files and return the
correction commit. No Cargo/docs-example/parity reruns or source re-audit.
