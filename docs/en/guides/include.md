# Configuration files

[Tiếng Việt](../../vi/guides/include.md) | [English home](../README.md)

The unpublished `re-cordis-include` satellite reads local JSON and restricted
YAML into the accepted typed loader. Register linked plugins and their JSON
decoders in a `Catalog`, then construct `Include::new` inside `Runtime::run`.
`IncludeOptions::new(path)` selects the root. Construction admits ownership;
`refresh()?.await?` reads the source and joins its application.

A source can be a bare array or an object with `entries` and `patches`.
Ordinary entries follow the [loader schema](../api/loader.md). For example:

```json
[
  {"id": "app", "name": "application", "config": {"port": 3000}},
  {"id": "extra", "name": "include", "config": {"path": "extra.yaml"}}
]
```

Nested paths resolve relative to the file containing them. Canonical native
paths identify files, including symlink targets; duplicate mounts and cycles
fail. A nested include appears as a structural group in loader snapshots.
Anonymous entries receive stable in-memory IDs without forcing a write.
Assign explicit IDs when identity must survive structural source rearrangement.

Use `IncludeOptions::initial(value)` only for a missing root file. It never
replaces an unreadable or malformed existing source. A failed read/parse leaves
the last accepted state available. Once the file is corrected, call `refresh`
again. Invalid individual entries follow loader's independent source policy:
healthy siblings continue, and the report identifies rejected entries.
Inspect `Report::failures`; completing a waiter does not imply all plugins are
Active. Identical source bytes avoid repeated reconciliation.

JSON uses strict syntax and rejects duplicate keys. YAML accepts JSON-compatible
values, finite numbers and string keys. Anchors, aliases, explicit tags,
non-finite numbers and multiple documents are rejected. Canonical writing
produces pretty JSON, also valid YAML 1.2; comments, quoting and style are lost.
This is value-oriented configuration, with no JavaScript execution.

`entries` returns immutable loader snapshots; `data` returns effective JSON.
`Location` identifies the real owning file, structured path and available
one-based position. Successful JSON/YAML node positions count Unicode scalars;
strict JSON syntax errors retain serde_json byte columns. These differ from
upstream UTF-16 columns. Runtime inputs have no invented file positions.

File I/O is synchronous and may block on slow storage. Keep the original
runtime driven until lifecycle waiters finish. Explicitly await `dispose`;
check its result, `final_error` and core cleanup diagnostics. Dropping a handle
does not complete asynchronous cleanup.

See the [include API](../api/include.md), [decisions](../../decision.md#include-files-and-persistence)
and [limitations](../../limitation.md#include-files-and-persistence).
