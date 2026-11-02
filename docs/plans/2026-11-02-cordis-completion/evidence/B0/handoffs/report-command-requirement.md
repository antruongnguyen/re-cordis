# Executable validation provenance in B0 reports

The B0 handoffs require exact commands, actual exits and compact results.
Do not replace an inline validation program with `<inline validator>` or
`...`: that cannot reproduce the claimed check. Embed the exact runnable
standard-library program in the report, or invoke a committed script with
its exact path/arguments. Include the actual docs-helper invocation too.

Keep source-audit mapping requirements unchanged. Reuse the checks already
written; this is report provenance, not permission to rerun Cargo/parity or
expand task scope. If the earlier script was not retained, repeat only the
bounded reference/structure/log checks, record that new exact command and
actual result, and distinguish it from the earlier summarized invocation.
