# Include source-suite dispositions

Reference: TypeScript `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
The [inventory](../source-inventory.md#include) lists 89 declarations in seven
suites; parameter variants are not counted as new Rust tests. This map records
Rust scope and its evidence, not 89 executed equivalences. Accepted loader and
core evidence is reused.

| Pinned suite | Declarations | Selected Rust behavior and deliberate boundary |
|---|---:|---|
| location.spec.ts | 39 | File/path/value diagnostics, nested groups/includes and owning patch locations. Runtime values have no invented file span. YAML aliases/anchors and JS expression/module stack rewriting are excluded; typed catalog errors replace dynamic imports. |
| parse.spec.ts | 2 | Actual scalar, duplicate, alias/tag, location and canonical-output fixtures decide supported YAML. Executable expression-tag round trips are excluded; comments/style are not preserved. |
| patch.spec.ts | 11 | Ordered overrides/disable/name matching, missing-target warnings, root/group insertion, invalid non-group target and later patches addressing inserted entries. |
| path.spec.ts | 1 | Relative resolution from containing file, absolute native paths, canonical symlink identity and cycle limits. Foreign Windows syntax is not treated as a Unix absolute path. |
| reload.spec.ts | 5 | Explicit refresh reapplies/removes patches, factory replacement, invalid-to-valid recovery, and missing-only initial data. Watching is deferred. |
| sync.spec.ts | 27 | Coalesced bounded journal, three-way conflicts, both explicit refresh/update orders, ownership-aware writes, moves/changes, stable anonymous IDs, failures and disposal flush. Explicit hosted operations replace watcher races and JS mutable options. |
| write.spec.ts | 4 | Deterministic transient/permanent replacement errors, retry reread, external edit during retry, bounded attempts and retained pending changes. |

Concrete Rust test names and final outcomes are added at the corresponding
milestones. Platform claims remain limited to the executed matrix and documented
filesystem contract; no new cross-language include trace is claimed.

I1 executed evidence: read.rs covers missing-only initial, retained malformed
source/recovery/dedup and nested path/cycles. locations.rs covers scalar policy,
duplicate/tag/alias/document rejection, Unicode positions, byte/depth limits,
nested owning-file failures and escaped/literal astral JSON characters. Five
yaml_capabilities.rs fixtures expose parser capabilities without lossless claims.
Both explicit toolchains pass all 12 tests at this checkpoint.

I2/I3 evidence: patches.rs covers ordered patch application, removal and nested
provenance; reconcile.rs covers journal/coalescing/conflicts/runtime validation
and owner routing; write.rs plus private deterministic writer gates cover
failures, retries and partial multi-file recovery. read_acceptance.rs and
lifecycle_acceptance.rs add 11 independent public-API boundary checks, including
100 resource-baseline cycles. See [implementation evidence](include-implementation.md)
and [boundary regressions](include-boundary-tests.md).

I4 selected evidence: lifecycle.rs verifies final-flush ordering, original
parser causes, visible final errors, dropped/repeated joined disposal, reentry,
latest factory intent and include-descriptor moves. review_repairs.rs adds
public regressions for pending parents, subtree deletion, opaque application
JSON, replacement-patch includes and effective patch-owned field removal.
See [review corrections](include-corrections.md); final acceptance additionally
requires the scoped re-review and its residual corrections.
