# Include I0 contract (2026-11-03)

Prerequisite: accepted loader/group implementation 101f37b and scoped rereview.
No loader/core changes are planned. Include is an unpublished Rust 2024/MSRV
1.85 satellite, local to the core Runtime host.

## API and schema

`Include::new(&Context, Catalog, IncludeOptions)` admits a dedicated owner
before filesystem access. Options supply a native `PathBuf` root and optional
JSON initial value. Initial creation uses create_new only when absent.
`refresh`, JSON `edit/create/remove/move_entry`, `flush`, and `dispose`
return explicit errors/reports and joined loader tasks as appropriate.
Observation exposes entry snapshots, effective JSON, ownership, pending count
and final I/O errors; it exposes no raw mutable Loader.

A file is a bare entry array or `{entries: [...], patches: [...]}`.
Entries use the accepted loader JSON schema. A nested source is
`{id?, name:"include", config:{path:"relative-or-native-absolute"}}`;
it becomes a structural group. Relative paths resolve from the containing file.
Patches are ordered objects `{id?, insert?: [...], name?, ...overrides}`:
insert at root without id, or in an existing group; name is a match guard.
Missing/mismatched targets are reported and own nothing. Later patches see
prior inserts. Group config replacement retires stale child ownership.
All file/entry/key locations retain actual canonical path and structured raw
path; mapped loader failure spans come from actual parser events.

## Formats and capability evidence

JSON is strict serde_json syntax, with a JSON-native duplicate/location pass
that accepts escaped surrogate pairs independently of YAML syntax. YAML uses yaml-rust2 0.10.4 marked parser events and a
restricted YAML 1.2 core scalar resolver. Null, booleans, finite JSON numbers,
strings, sequences and string-key mappings are supported. Duplicate keys,
anchors/aliases, all explicit tags (including !js) and multiple documents
reject before value expansion. YAML aliases are never expanded.
JSON-shaped executable data is never evaluated; loader rejects unsupported
option expressions through its accepted schema.

Five executed low-level capability fixtures in `tests/yaml_capabilities.rs`
confirm scalar text/style and actual marks, visible duplicate keys (the parser
alone does not reject them), visible aliases/anchors and explicit tags, and
that comment text is absent from events. JSON and YAML writes use canonical
pretty JSON; JSON is valid YAML 1.2. This deliberately preserves values rather
than comments, whitespace, quoting or YAML style. No lossless claim is made.
Columns exposed by the marked parser are one-based Unicode scalar columns,
not UTF-16 units; strict JSON syntax errors retain serde_json byte columns.

## Identity, limits and validation

Canonical native OS paths define file identity and cycle detection; symlinks
resolve to their actual target for reads and writes. Windows syntax is native
only on Windows, and a backslash is an ordinary filename character on Unix.
Missing roots resolve their canonical parent plus filename. Each canonical
source mounts once; repeated mounts/cycles reject. Anonymous IDs persist
in-memory by file/raw-position identity, never force a read-only rewrite, and
skip all explicit IDs including nested and inserted entries.

Bounds: 1 MiB per file and journal serialized bytes, 32 value/include/group
depth, 65536 value nodes and string bytes, 64 files, 1024 effective entries,
1024 patches, 1024 pending entry records, three replacement attempts.
Admission rejects new journal records or bytes before changing running state;
coalesced same-entry deltas remain bounded. Structurally malformed/read-failed
graphs preserve last valid running state. Parsed invalid individual entries
follow accepted loader source policy: valid siblings continue, invalid existing
source entries are removed, initial decode failures are located. Refresh
deduplicates identical complete source graphs and still retries pending work.

## Journal, conflict and durability

JSON mutations preflight with loader before committing. Native non-Serialize
Config changes cannot cross this API implicitly. The journal records immutable
JSON deltas at admission, coalesces update/move, erases create-then-remove,
and routes entry/key changes to actual file/patch/embedded owners.

Three-way reconciliation compares the previous accepted file snapshot, current
file graph and pending journal at entry-key granularity. File modifications
win a conflicting key; equal edits do not conflict. File removal wins runtime
update; a changed file entry defeats runtime removal; incompatible parent moves
are structural conflicts. Every conflict is reported. Both refresh/update
orderings use the same snapshot/journal reconciliation and never feed source
reconciliation back into a write journal.

One serialized synchronous writer rereads/reconciles on every bounded retry.
It creates a sibling temp with create_new, writes with write_all, rereads the
source bytes before rename, and atomically renames per file on platforms that
support replacement. Invalid latest files are never overwritten. Failures
retain pending changes and clean owned staging files. Multi-file edits are not
a filesystem transaction; successful files update their baselines before later
files fail. Bounded retries handle transient replacement/reread races;
persistent failures are explicit and disposal reports final errors.

Atomic replacement is not fsync crash durability or an arbitrary
concurrent-writer protocol. Uncooperative writers can still modify a file in
the final check-to-rename window. No cross-process lock protocol, watcher,
HMR, dynamic JavaScript/native loading or create package is implemented.

## Joined ownership

All public admissions check host/owner before I/O or mutation. Writes finish
synchronously, so dropping a waiter cannot abandon a write. Disposal stops
admission, flushes admitted deltas, then starts loader subtree disposal; its
retained joined cleanup includes slow children and exposes final I/O errors.
Parent owner cleanup also stops admission and performs the bounded final flush;
the dedicated owner retains cleanup independently of Include/waiter drop.

## I3 implementation refinements

The journal byte budget includes immutable full JSON fallback snapshots and
original placement metadata, so a partial multi-file write cannot erase unchanged
fields needed by a pending move. Current provenance is recomputed at JSON admission
and between factory/group transitions and later option keys. A successfully written
file regenerates locations from its actual canonical bytes.

Existing-file permission bits are retained before replacement (Unix 0600 is tested).
New Unix stages use 0600. Missing-only creation hard-links a complete sibling stage
into a still-absent destination; it never replaces a newly appeared file.
Existing destinations use rename. Other ownership/ACL/xattr/hardlink metadata and
fsync durability are outside this policy.

A failed operation exposes file-winning decisions in `IncludeError.conflicts`
and retains them for the next successful Report. IncludeError's location is an
immutable `Option<Rc<Location>>`; callback/admission/lifecycle causes are preserved.

Runtime moves carry a rebased absolute canonical path for include descriptors,
including descriptors nested inside moved groups. Source-path replacement through
generic config/group/name editing remains explicitly refused before mutation;
edit a source file and refresh to replace an include path. Only the public Include
JSON mutation APIs journal changes; native core/loader updates are not implicitly
serialized. Include owns one stable SourceId across canonical root target changes.

The complete source graph's compact effective JSON must fit the accepted loader's
1 MiB limit in addition to each file's individual bound. MAX_PATCHES is per file.
Anonymous identity cache entries are pruned to live raw file positions and bounded
at 1024. Multi-file visibility is non-atomic, with tested pending recovery in both
write orders and after parent-patch/child-file partial failure.

## I4 effective field removal

Override patches also accept `unset: ["key", ...]`, applied after overrides.
The array contains only strings; `id`, `name`, `insert` and `unset` are forbidden
control names. Invalid markers skip the entire patch with a located warning.
An absent effective key retains its patch owner, so runtime removal writes
back to that owner without revealing a hidden base value. A later set clears
the marker. Config absence differs from explicit JSON null. Ordinary plugin
config remains opaque; only actual structural entry collections receive IDs
and source discovery.
