# Include implementation evidence

Prerequisite loader/core reuse: accepted loader 101f37b; no loader/core production
changes. Contract: [include-contract](include-contract.md).
All package writes use the project IDE's VFS; tests use explicit rustup toolchain
executables. Detailed local logs live under target/include-20261103/ (ignored).

## I0

Five yaml-rust2 0.10.4 capability fixtures pass stable/MSRV: scalar text/style and
actual marks, duplicates visible, aliases/anchors visible, explicit tags visible,
comments absent. Wrapper rejects unsupported events before expansion.
JSON uses its own duplicate/position scan so literal astral UTF-8 and escaped
surrogate pairs remain valid JSON.

## I1

Seven read/location regressions plus five capability tests pass stable and 1.85.0:
missing-only initial data with no eager write, invalid existing content never
replaced, dedup and last-valid recovery, nested canonical relative includes and
cycles, Unicode scalar positions, strict format failures and nested owning-file
diagnostics. Initial persistence is subsequently strengthened by I3's guarded
staging writer. Public admissions validate the host/owner before file access.
Parsed entry invalidation deliberately delegates to accepted loader source
contracts. File bytes, values, source graph and effective JSON are bounded.

## I2

Four added patch regressions pass both toolchains, covering ordered override,
disable/name guards, skipped/missing targets, root/group inserts, later patches
targeting prior inserts, group replacement clearing old owners, removing patches,
nested file entry plus parent patch key ownership, and stable anonymous IDs
that skip explicit root/nested/patch IDs without eager writes.
The anonymous cache is pruned to current raw file identities and capped at 1024.
A small eager-operation guard refuses wrapper reentry before outer source
publication; no mutable state borrow crosses loader callbacks.

I3/I4 evidence will be added after their tests execute.
