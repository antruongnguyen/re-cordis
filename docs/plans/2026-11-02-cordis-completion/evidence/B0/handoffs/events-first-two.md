# B0 bounded events recovery

Executor: gpt-6.1-sol/high. Repository:
`/Users/I756434/Projects/private/cordis-rust`. Read this first, then AGENTS.md,
the events.md handoff beside this file, and the Task 2 schema in ../plan.md.
This task narrows events.md to exactly `core/events.spec.ts:18` and
`core/events.spec.ts:35`. The nineteen-case attempt failed with HTTP 400 on
2026-11-02 at 09:28:15 UTC; its worktree is clean and has no new output.

Create an isolated managed worktree from integration HEAD using create_worktree.
Read pinned TS and indexed source from the integration checkout's local/cordis
and CodeGraph project. Do not modify upstream or create an index. Reuse the
accepted matrix and prior schema. No Cargo/docs/parity rerun or production edits.

Own only ../ledger-inputs/events-first-two.json and report-events-first-two.md.
Copy these two records from ../ledger/cases.json, finish exact assertion-level
mappings against current Rust bodies and both passing workspace logs. Preserve
IDs, variants, source lines, and split mixed obligations when needed. Classify
each as an evidenced mapping/substitute/existing exclusion or concrete gap with
owner and conditional gate. Missing evidence alone does not open B2. Save each
completed record promptly; do not fill the other seventeen declarations.

Validate inventory, evidence fields, cited lines, passing-test joins and links
with a small standard-library check. Report actual commands/results and limits.
Commit only owned files in the isolated worktree, overriding the old handoff's
no-commit instruction. No subagents, progress.md edits, pushes or other tasks.
Return a short result with worktree, commit, two-case counts and any blocker.
