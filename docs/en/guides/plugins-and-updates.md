# Plugins, configuration, and recovery

[Tiếng Việt](../../vi/guides/plugins-and-updates.md) | [English home](../README.md)

## Choose a declaration

Use `Plugin::new` when source and apply input are the same cloneable type.
Use `Plugin::validated` for a synchronous `&C -> Result<R, E>` transformation;
source need not be Clone in that form. Use `Plugin::initialized` when async
initialization naturally produces one `Disposer`.

Apply callbacks register resources through their Context and return
`Result<(), E>`. They are not interpreted as JavaScript constructors, generators,
or arbitrary disposer-returning functions. Validation is synchronous; no schema
library or serde discovery is implicit.

## A complete validated update

This example uses only the dependencies from [getting started](../getting-started.md).

```rust
use cordis_core::{BoxError, FiberState, Plugin, Runtime};
use std::io;

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                let plugin = Plugin::validated(
                    "BatchWorker",
                    |source: &u32| {
                        if *source == 0 {
                            Err(io::Error::new(io::ErrorKind::InvalidInput, "batch must be positive"))
                        } else {
                            Ok(*source)
                        }
                    },
                    |ctx, batch: u32| async move {
                        ctx.logger(None).info(format!("batch={batch}"))?;
                        Ok::<_, BoxError>(())
                    },
                );
                let fiber = root.plugin(&plugin, 1)?;
                assert_eq!(fiber.ready()?.await?, FiberState::Active);
                assert!(fiber.update(0).is_err());
                assert_eq!(*fiber.config(), 1);
                assert_eq!(fiber.update(2)?.await?, FiberState::Active);
                assert_eq!(*fiber.config(), 2);
                assert_eq!(fiber.dispose()?.await?, FiberState::Disposed);
                Ok::<_, BoxError>(())
            }.await;
            let cleanup = root.drain()?.await?;
            if let Some(failure) = cleanup.failures().first() {
                return Err(failure.clone().into());
            }
            result
        }).await?
    })
}
```

## Admission is not readiness

`Context::plugin` synchronously commits Registry/parent ownership and returns a
handle; apply happens on the host. Initial validation failure leaves an
addressable Failed instance. Await `ready` to observe it, then dispose it or
explicitly update it.

Each Fiber stores source `C`; apply receives validated `R`. Loaded candidates
are validated before source commit or update middleware. Invalid candidates
preserve the running source/resources. Pending Fibers retain the latest source
and defer validation until required services become available.

## Restart, update, and middleware

`restart` forces reapplication of the same source, but does **not** clear Failed.
`update` accepts a new source and enables explicit recovery, even when the source
value is unchanged. Provider withdrawal/republication alone does not retry Failed
consumers.

`on_update` receives the resolved input and an owned `AsyncNext<bool>`. Calling
next starts the restart path. Omitting it accepts the source without restarting.
`on_update_sync` returns true to continue or false to accept without restarting.
These listeners are scoped and removed on reload.

`UpdateOptions { no_save: true }` suppresses the commit notification only.
It does not suppress updating the retained source or applying configuration.

## Concurrency and shutdown

One lifecycle driver converges toward the latest configuration and exact
dependencies. In-flight apply is not forcibly aborted. Cleanup completes before
the latest available target loads; intermediate updates can be skipped.
Terminal disposal wins over queued reloads.

An update waiter can settle Pending while a provider is unavailable. Await the
provider's completion and then the consumer's readiness separately. None is a
runtime-wide convergence barrier.

Use `Registry::remove` to retire a declaration's current group; await each
returned task. New registrations of that declaration form a fresh group.
`Registry::len` counts groups, not instances.

Reference: [plugins and Registry API](../api/plugins-registry.md).
Evidence: [updates tests](../../../crates/cordis-core/tests/updates.rs) and
[paired lifecycle scenarios](../../lifecycle-parity.md).
