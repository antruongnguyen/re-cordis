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

## Ordered patches and provenance

An object source can add an ordered `patches` array:

```json
{
  "entries": [{"id": "app", "name": "application", "config": {"port": 3000}}],
  "patches": [
    {"id": "app", "name": "application", "config": {"port": 4000}},
    {"insert": [{"id": "extra", "name": "application"}]},
    {"id": "extra", "disabled": true}
  ]
}
```

A patch's `name` is a matching guard. It does not rename the target.
Without `insert`, `id` is required; with `insert`, no `id` means append
at the source root, while an `id` selects a structural group.
Missing targets, name mismatches and non-group insertion are reported and skipped.
Later patches see earlier insertions. A group config replacement discards the
old children's ownership before registering the replacement entries.

`ownership(id)` separates the entry owner from each overridden key.
An entry may live in a nested file while its config belongs to a patch in the
parent file. Removing the patch restores the base value on refresh.
Anonymous identity is matched by canonical file/raw position; the live cache
is pruned and bounded. IDs skip explicit IDs in both source and patch entries.


Patch `unset` removes effective fields after overrides. For example,
`{"id":"app","unset":["config","disabled"]}` hides those fields even if a
lower layer supplies them. `config: null` remains an explicit null value;
it is not equivalent to `unset: ["config"]`. Control keys `id`, `name`,
`insert` and `unset` cannot be unset. `JsonEdit::remove` records this absence
at the patch owner; a later `JsonEdit::set` clears the corresponding marker.

## Runtime changes and flushing

Use `edit(id, JsonEdit::new().set(key, value))` for a complete top-level entry
field. `JsonEdit::remove(key)` removes a field; setting config to JSON null
retains an explicit null value. `create(value, parent, position)`,
`remove(id)` and `move_entry(id, parent, position)` edit structure.
A parent is an optional entry ID; None selects the root. These operations
return a waiter, and `settle` joins admitted loader work.

Changes enter a bounded in-memory journal before persistence. Repeated edits
coalesce by entry; create-then-remove leaves no pending write.
`pending` reports the number of pending entry records. Admission refuses
overflow before changing the runtime. The adapter does not expose a mutable
loader or serialize arbitrary native Config values.

Await `flush()?.await?` to reread, reconcile, write and join application of the
reconciled view. `refresh` uses the same reconciliation policy. A file change
wins when both sides change the same top-level key differently; `config`
is one whole key, not a recursive merge of application fields. Identical edits
are not conflicts. File removal wins over a runtime edit; modifying an entry
in the file conflicts with runtime removal. Incompatible parent changes are
reported as structural conflicts. Inspect `Report::conflicts`.

Write-back follows the owner of each entry/key, including nested files,
patch overrides and entries embedded in patch inserts or replacement group
configs. Source reconciliation does not itself create a write-back journal.
An include descriptor's generated group/config cannot be replaced by generic
runtime edits; change its path in the source and refresh.

On a write failure, the operation returns an error and pending runtime changes
remain in memory. Correct the external cause and retry refresh/flush, or await
disposal's final attempt. Never replace an unparseable current file just to
force a successful save.

Each file is written through a sibling temporary file with bounded attempts.
Each retry rereads and reconciles current content. Replacement is per file;
there is no multi-file transaction, fsync crash-durability promise or complete
concurrent-writer protocol. An uncooperative writer can still race the final
content check and rename.

A partially successful multi-file flush retains bounded full snapshots so a
later retry can recover unchanged fields of a moved entry, whichever file was
written first. Missing roots are installed by exclusively hard-linking the
complete sibling temporary file; this requires filesystem hard-link support.
Existing-file replacement preserves permission bits. Moving an include-bearing
subtree rebases its descriptors to canonical absolute paths. Conflicts remain
available on a write error and in the next report.

## Offline example

This complete program reads a real file, applies a typed runtime change, flushes it and joins shutdown. It creates and removes its own temporary directory; no network or external configuration is needed.

```sh
cargo run -p re-cordis-include --example config_file --locked --offline
```

```rust
//! Offline file read, JSON edit, flush and joined shutdown.
use re_cordis_core::{BoxError, Plugin, Runtime};
use re_cordis_include::{Include, IncludeOptions, JsonEdit};
use re_cordis_loader::Catalog;
use serde_json::json;
use std::{
    convert::Infallible,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() -> Result<(), BoxError> {
    let directory = std::env::temp_dir().join(format!(
        "re-cordis-include-example-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    ));
    std::fs::create_dir(&directory)?;
    let result = (|| -> Result<(), BoxError> {
        let path = directory.join("cordis.json");
        std::fs::write(
            &path,
            serde_json::to_vec_pretty(&json!([
                {"id":"worker","name":"worker","config":1}
            ]))?,
        )?;
        let executor = tokio::runtime::Builder::new_current_thread().build()?;
        executor.block_on(async {
            let runtime = Runtime::new();
            runtime
                .run(async {
                    let root = runtime.context();
                    let mut catalog = Catalog::new();
                    catalog.register_json(
                        "worker",
                        Plugin::new("worker", |_, value: u32| async move {
                            println!("worker loaded with config {value}");
                            Ok::<_, Infallible>(())
                        }),
                        |value| serde_json::from_value::<u32>(value.cloned().unwrap_or(json!(0))),
                    )?;
                    let include = Include::new(&root, catalog, IncludeOptions::new(path.clone()))?;
                    let read = include.refresh()?.await?;
                    assert!(read.failures.is_empty());
                    let changed = include
                        .edit("worker", JsonEdit::new().set("config", json!(2)))?
                        .await?;
                    assert!(changed.failures.is_empty() && changed.conflicts.is_empty());
                    let flush = include.flush()?.await?;
                    assert!(flush.failures.is_empty() && flush.conflicts.is_empty());
                    let written: serde_json::Value =
                        serde_json::from_slice(&std::fs::read(&path)?)?;
                    assert_eq!(written[0]["config"], 2);
                    include.dispose()?.await?;
                    assert!(root.registry().is_empty()?);
                    assert!(root.drain()?.await?.failures().is_empty());
                    assert!(runtime.take_failures().is_empty());
                    println!("config 2 persisted; joined shutdown restored the Registry");
                    Ok::<_, BoxError>(())
                })
                .await??;
            Ok::<_, BoxError>(())
        })
    })();
    let cleanup = std::fs::remove_dir_all(&directory);
    result?;
    cleanup?;
    Ok(())
}
```
