# Scoped logging report provenance correction

Executor: b0_scoped_logging, same authorized gpt-5.6-sol/high context.
In your existing worktree at 87b41ce, update only
ledger-inputs/report-scoped-logging.md under B0. Read
report-command-requirement.md beside this handoff. Your report currently
uses `<inline scoped-fragment structural/reference/matrix validator>` and
`<docs link helper invocation>` instead of the required actual commands.

Preserve all six JSON fragments and mapping claims. Embed the complete
validation program actually run (or run a newly recorded bounded check if
necessary), exact docs-helper invocation and exits/results. Do not rerun
the source audit, Cargo or parity. Check report links/whitespace, commit
only the report, and return the correction commit; independent semantic
review will cover the full family plus the corrected report.
