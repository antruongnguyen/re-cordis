# Loading and reconfiguring plugins

[Tiếng Việt](../../vi/guides/loader.md) | [English home](../README.md)

The unpublished `re-cordis-loader` crate manages named, already-linked Rust
plugins and an ordered entry tree. Add its local path dependency alongside
`re-cordis-core` and Tokio. It owns a dedicated core Fiber under the supplied
Context, so parent cleanup also shuts down the loader.

## Boot, recover and shut down

`Catalog::register` retains a typed `Plugin<C, R>`; `Config::new` holds native
configuration without serialization. Its source type must match the catalog.
Core still validates at its readiness boundary, after required services arrive.

```rust
use re_cordis_core::{BoxError, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Group, Loader};

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let settings = ServiceKey::<u32>::new("settings");
            let worker = Plugin::new("worker", |_, value: u32| async move {
                if value == 13 { return Err::<(), BoxError>("example failure".into()); }
                Ok(())
            }).injecting([settings.required()]);
            let mut catalog = Catalog::new();
            catalog.register("worker", worker)?;
            let loader = Loader::new(&root, catalog)?;
            loader.create(EntrySpec::group(Group::new(vec![
                EntrySpec::new("worker", Config::new(1_u32)).id("worker"),
            ])).id("app"))?;
            let first = loader.settle()?.await?;
            assert!(first.failures.is_empty());
            let id = loader.resolve("app:worker")?;
            assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Pending);
            root.provide(&settings, 7)?;
            assert!(loader.settle()?.await?.failures.is_empty());
            loader.update(&id, EntrySpec::new("worker", Config::new(2_u32)))?.await?;
            assert!(loader.update(&id, EntrySpec::new("worker", Config::new(13_u32)))?.await.is_err());
            loader.update(&id, EntrySpec::new("worker", Config::new(14_u32)))?.await?;
            loader.dispose()?.await?;
            assert!(root.drain()?.await?.failures().is_empty());
            assert!(root.registry().is_empty()?);
            Ok::<_, BoxError>(())
        }).await??;
        Ok::<_, BoxError>(())
    })
}
```

Run the more verbose application from the repository:

```sh
cargo run -p re-cordis-loader --example boot_reconfigure --locked
```

Admission and completion are separate: `create` returns an identity immediately;
`update`, `remove`, `move_entry` and `dispose` start work and return waiters.
Dropping a waiter does not undo the operation. `settle` observes admitted work,
including structural descendants, and reports per-entry failures.
A successful settlement can contain Pending entries. It does not promise
future providers or permanently stable global readiness. Await cleanup before
destroying the runtime and inspect both operation errors and cleanup reports.

## Configuration and source ownership

`EntrySpec` describes a complete replacement; `EntryPatch` uses
`FieldPatch::Keep`, `Set` and `Clear` for partial edits.
Snapshots retain original typed source separately from the last evaluated
value. Config mapping runs in the exact plugin injection Context; conditions
and dynamic intercepts use the entry injection Context. Disabled entries do
not evaluate. Typed callback failures are visible failures; an explicit update
can recover them.

Register an explicit decoder with `Catalog::register_json` before using
`decode_json` or `create_json`. The decoder receives `None` for absent config
and `Some(Null)` for JSON null. Unknown entry fields and unsupported disabled
expressions are rejected. Config remains decoder-owned data, never executable
JavaScript. See the [API](../api/loader.md) for transport fields and bounds.

`SourceDocument::typed` or `SourceDocument::json` supplies a read-only snapshot
to `reconcile`. Valid source siblings are processed independently; runtime
mutations validate before commit. `SourceId` is a caller-managed label, not a
path to read. `with_locations` can add actual spans to structured diagnostic
paths. Runtime data has no invented filename.

Listen to `loader.commits()` for accepted runtime source edits.
Reconciliation suppresses write-back commits. The caller owns serialization
and persistence; the loader does not read or write files.
`replace_factory` replaces an already-linked declaration and rebuilds affected
entries after cleanup. For dependency-selected batches and optional Include
file watching on the same managed entries, see [HMR](hmr.md). Source changes
report restart-required; compiled candidates are supplied by the application.

See [groups](groups.md), [the API](../api/loader.md), and
[limitations](../../limitation.md#loader-and-structural-groups).
