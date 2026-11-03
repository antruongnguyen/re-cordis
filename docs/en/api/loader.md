# Loader API

[Tiếng Việt](../../vi/api/loader.md) | [English home](../README.md)

Generate exact signatures with `cargo doc --workspace --no-deps --locked`;
open `target/doc/re_cordis_loader/index.html`. All exports are local/non-Send,
unstable and unpublished. [Runnable guide](../guides/loader.md).

## Catalog, entries and snapshots

| Export | Contract |
|---|---|
| `Catalog` | Registers named typed Plugin declarations; duplicate names fail. JSON and typed service-name decoders are opt-in. |
| `Config` | Alias of core hooks::Value; immutable shared typed source, checked with get. Native source requires no serde traits. |
| `Loader` | Owns entry tree and factory catalog beneath one dedicated core owner. |
| `EntryId` | Stable, loader-wide logical identity; generated entry-N or explicit ID. |
| `FactoryId` | Catalog allocation identity, distinct from EntryId and core FiberId. |
| `Scope` | Immutable typed isolation/intercept layers; fresh isolate, explicit isolate_with, static intercept and evaluated intercept_with. |
| `EntrySpec`, `EntryKind` | Complete plugin/group options; typed injecting, scope, intercept, condition and map_config builders. |
| `EntrySnapshot` | Immutable source/options, evaluated config, transport, state, parent, ordered children, address and current Fiber IDs. |
| `EntryState` | Pending, Loading, Active, Failed, Unloading, Disabled or Disposed. Missing services are Pending. |
| `LoaderTask` | Waiter for already-started mutation; dropping it does not cancel. |
| `Settlement`, `EntryFailure` | Current entry snapshots and per-entry errors; settlement is not a guarantee that every entry is Active. |
| `LoaderError` | Catalog/identity/config/admission/callback/lifecycle or located error, retaining underlying causes. |
| `CommitRecord`, `Origin` | Old/new snapshots, exact entry, old/target group and Runtime/Source origin. Source reads do not emit write-back events. |

`create`/`create_in` admit checked entries. `update` replaces full options;
`remove` unlinks and starts teardown. `snapshot`/`entries` are reads and do not
expose mutable Fiber control. `resolve` looks up a current composite address.
`move_entry` preserves EntryId and changes the address; see [groups](groups.md).
`settle` waits admitted generations and every outstanding direct update; later
updates completing first do not release older barriers. `remove` joins the
removed subtree's updates; `dispose` and owner shutdown join all updates and
detached cleanup. Dropping an operation or settlement waiter does not cancel
work or acknowledge its result. Loader retains the first unobserved direct-update
failure, rather than an unlimited error history. Completed original/removal
waiters and successful settlement delivery acknowledge results; disposal can
report the same retained failure again.
Use the correct Runtime::run host for mutation and keep it alive through waits.
Loader retains the first detached lifecycle-task error while still joining all
cleanup. Ordinary user cleanup errors follow core diagnostics:
inspect Runtime::take_failures as well as operation results/CleanupReport;
Disposed alone does not imply error-free cleanup.

Each factory retains its original Plugin identity. Configuration decoding is
separate from core validation; Pending entries do not eagerly validate plugins.
A rejected loaded update keeps the accepted source. Failed entries need an
explicit update to recover. Conditions use the entry injection Context;
map_config uses the exact inner plugin injection Context before validation.

## Partial edits and source transport

| Export | Contract |
|---|---|
| `EntryPatch` | Partial factory/config/disabled/inject/group edits. |
| `FieldPatch` | Keep preserves, Set replaces, Clear restores default; clearing factory is invalid, config clears to native unit, disabled to false, injection/children to empty. |
| `SourceId`, `SourceDocument` | Caller-managed read-only typed/JSON source; reconciliation owns its entries and handles valid siblings independently. |
| `SourceLocation`, `SourceSpan` | Optional source label, structured PathSegment path, optional real one-based line/column supplied by a locator. |

JSON entry fields are `id`, `name`, `config`, `group`, `disabled`, `inject`,
`isolate`, `intercept`. Structural `group: true` uses a child array as config.
Unknown fields fail. Register service contracts for inject/isolate/intercept
names; unknown contracts fail. Named realms share a loader-local isolation ID while their Scope definitions
remain live. After the last defining scope is released, reusing the label may
allocate a fresh ID; committed old Contexts keep their old service views.
Absent config differs from explicit null at the decoder boundary.
Decoder-owned config may contain arbitrary data; no field runs JavaScript.

| Bound | Value |
|---|---:|
| `MAX_ENTRIES` (resident plus retiring entries) | 1024 |
| Simultaneously live named realms (same limit) | 1024 |
| Outstanding direct updates, globally (same limit) | 1024 |
| `MAX_JSON_BYTES` | 1048576 |
| `MAX_JSON_DEPTH` | 32 |
| `MAX_JSON_NODES` | 65536 |
| `MAX_STRING_BYTES` (also object keys) | 65536 |
| Explicit entry ID | 1–128 UTF-8 bytes, no colon |
| Factory name | 1–256 UTF-8 bytes |
| Native group nesting | 32 |
| Isolation label | 1–128 UTF-8 bytes |

Native Config payload sizes are application-owned. Bounds are admission limits,
not a wall-clock timeout or a serializer size promise.

`replace_factory` supports replacement by another linked typed declaration.
`commits`, `reconcile` and snapshots are in-memory integration seams for future
persistence/watch adapters. No filesystem, Node resolution, JS eval or native
binary reload is provided. See [limitations](../../limitation.md#loader-and-structural-groups).
