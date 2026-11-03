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

I4 evidence will be added after its tests execute.

## I3

48 package tests pass on stable and 1.85.0, including 11 independent read/lifecycle
boundary tests, with strict package Clippy on both toolchains. Logs:
`target/include-20261103/i3-{stable,1.85.0}.log` and
`i3-clippy-{stable,1.85.0}.log`. No core/loader changes.

JSON admission preflights accepted loader runtime decoding and loaded validation:
a rejected loaded validator preserves its source, retains its cause, remains
unlocated, and never enters the journal. Changes coalesce by entry and are bounded
by 1024 records and 1 MiB including immutable fallback snapshots. Source refresh
and write-back never create a commit-event feedback journal. Only Include's JSON
APIs persist; arbitrary native core/loader C is never claimed serializable.

Tests cover create/remove cancellation, dropped waiters, both refresh/update
orderings, key-level and structural file precedence with explicit conflicts,
file/patch/insert/embedded ownership, moves plus later updates, and name-guard
transitions that return later config edits to their effective new owner.
Ownership observations use the admitted planned provenance without filesystem I/O.

The staged writer uses create_new sibling files, write_all, permission retention
(existing Unix 0600 remains 0600), reread/check before every bounded replacement,
and cleanup of owned temps. Missing-only creation hard-links the complete staged
file into an absent destination, preserving atomic create-new semantics; existing
files use rename. No fsync crash-durability or multi-file atomicity is promised.

Deterministic gates verify short/zero writes, errors after open/write/before rename,
three attempts, interrupted replacement, external edits during retry, invalid
latest content refusal, pending preservation, and conflict reports retained across
a later write failure. Real symlink/permission tests supplement the gates.

A regression first reproduced loss of unchanged entry fields when source-file
removal succeeded but destination-file writing failed. Bounded immutable pending
snapshots, original placement and dependency-aware routing now recover the full
entry in both write orders. Tests also cover a child edit before a partially
persisted parent move, and successful parent-patch writing followed by a failed
child-file write. Partial filesystem visibility remains explicitly non-atomic;
successful files update their baselines and remaining work stays recoverable.

Canonical output positions are regenerated from actual written bytes. Independently
reproduced scalar-row location and signed YAML nonfinite failures are corrected;
see [boundary evidence](include-boundary-tests.md). Group replacement ID collisions
are rejected before removing children, so skipped patches own nothing.

Manual disposal performs at most one final flush occurrence (up to three attempts)
and shares its joined barrier. Parent cleanup's synchronously constructed final
flush runs before the earlier-registered loader child cleanup is begun. I4 provides
the explicit cleanup-order/final-error/example evidence.
