# Structural groups

[Tiếng Việt](../../vi/guides/groups.md) | [English home](../README.md)

Import `Group` from `re_cordis_loader`. Upstream's group package simply
re-exports loader behavior; Rust keeps one implementation and no facade crate.
`EntrySpec::group(Group::new(children))` declares ordered nested source entries.

A group remains a structural node when its disabled verdict prevents
descendant plugins from loading. It can be Active while children are disabled;
its own missing dependencies or callback failure can still make it Pending or Failed. Nested disabled inheritance applies to every
descendant. Updating a group reconciles child source independently and settles
included work; Pending children remain a normal result.

An EntryId is stable across moves. `move_entry` changes its logical parent
exactly once. Reordering in one group preserves Fiber identity; crossing groups
or changing scope joins the old generation then rebuilds core Fibers.
The old group removes only children still logically owned by it; it cannot
dispose an adopted entry. Composite `address` values change with group paths.
These addresses are a Rust navigation convenience: upstream ':' identifies
nested source trees, while Rust uses globally unique IDs across source documents.

Apply shared/fresh isolation with typed `Scope` layers. Old exact service
providers remain retained through consumer cleanup under the core contract;
a move does not rewrite a committed consumer Context. Use
`loader.dispose()?.await?` to join current and retired generations.
See [group API](../api/groups.md) and [loader](loader.md).

```rust
use re_cordis_core::{BoxError, Plugin, Runtime};
use re_cordis_loader::{Catalog, Config, EntrySpec, Group, Loader};
use std::convert::Infallible;

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", Plugin::new("worker", |_, (): ()| async {
                Ok::<_, Infallible>(())
            }))?;
            let loader = Loader::new(&root, catalog)?;
            let first = loader.create(EntrySpec::group(Group::new(vec![
                EntrySpec::new("worker", Config::new(())).id("worker"),
            ])).id("first").disabled(true))?;
            let second = loader.create(EntrySpec::group(Group::new(vec![])).id("second"))?;
            loader.settle()?.await?;
            let worker = loader.resolve("first:worker")?;
            loader.move_entry(&worker, Some(&second), 0)?.await?;
            loader.remove(&first)?.await?;
            assert_eq!(loader.resolve("second:worker")?, worker);
            loader.dispose()?.await?;
            Ok::<_, BoxError>(())
        }).await??;
        Ok::<_, BoxError>(())
    })
}
```
