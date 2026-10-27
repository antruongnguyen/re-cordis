# Services, injection, and scoping

[Tiếng Việt](../../vi/guides/services-and-scoping.md) | [English home](../README.md)

## Publish a capability, declare its consumers

`ServiceKey<T>` names a slot and checks its value type. Name and isolation
select identity; `T` alone does not. Two incompatible contracts for the same
slot are errors, not independent services.

Use injection rather than an unrestricted direct read for a consumer that needs
provider resources through setup and teardown.

```rust
use cordis_core::{BoxError, FiberState, Runtime, ServiceKey};
use std::io;

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                let key = ServiceKey::<u32>::new("count");
                let consumer = root.inject([key.required()], {
                    let key = key.clone();
                    move |ctx| {
                        let key = key.clone();
                        async move {
                            let value = ctx.get(&key)?.ok_or_else(|| {
                                io::Error::new(io::ErrorKind::NotFound, "count unavailable")
                            })?;
                            assert_eq!(*value, 7);
                            Ok::<_, BoxError>(())
                        }
                    }
                })?;
                assert_eq!(consumer.ready()?.await?, FiberState::Pending);
                let publication = root.provide(&key, 7)?;
                assert_eq!(consumer.ready()?.await?, FiberState::Active);
                let bound = root.bind(&key)?;
                assert_eq!(bound.call(|value, _ctx| *value), 7);
                publication.withdraw()?.await?;
                assert_eq!(consumer.ready()?.await?, FiberState::Pending);
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

Root-owned provision is convenient for small applications. For an asynchronously
initialized resource, publish from `Plugin::initialized`: injecting consumers do
not activate until initialization settles successfully. Unchecked direct reads
can still see a publication during Loading; they are not readiness barriers.

## Choose a lookup deliberately

| Operation | Authority / availability |
|---|---|
| `get` | Own provisions or committed injection/ancestor authority; root has an explicit bypass |
| `get_direct` | Bypass injection checks but require an Active, available provider |
| `get_unchecked` | Direct lookup including Loading/Unloading providers still in the slot |
| `bind` | Checked lookup with explicit definition/use-site tracing |
| `bind_direct` | Direct available-provider lookup with tracing |

A non-root undeclared access is an error. An inactive nearest declaration does
not fall back to another provider. Use the supplied Context in bound adapters;
ordinary `Rc<T>` method calls cannot automatically trace Rust code.

## Value replacement is not provider replacement

Only the providing Fiber can `set` its publication. Set preserves publication
identity and does not reload consumers or rerun readiness predicates.
Previously returned `Rc<T>` values remain snapshots of the old value.

Withdraw/reprovide creates a new publication. Old injected cleanup retains the
exact old occurrence until final use finishes, including consumers already
removed from the Registry. A plain `Rc<T>` or bound handle does not independently
retain provider resources.

## Isolation and intercepts

`ctx.isolate(&key)` selects a fresh slot. Share an `IsolationId` with
`isolate_with` when two Contexts should select the same isolated slot. Isolated
slots have no ancestor fallback. Even reselecting the same label creates a
boundary for inherited injection authority.

`Dependency::with_config` supplies typed injection configuration.
`ctx.intercept(&key, config)` adds a use-site layer. A bound service's
`intercepts::<C>()` reads nearest admitting injection config first, then Context
layers outer-to-inner. `resolve_intercept` supplies your merge operation;
the framework does not guess a JSON/shallow merge.

Bound calls use the provider definition's dependency authority and the caller's
use-site configuration/effect ownership. Calling from root does not give a
plugin-defined service root authority.

## Advanced adapters

Use `AccessorKey`/`Accessor` for explicit typed properties, `Accessor::from_service`
for mixins, and `ServiceInvoke` for callable services. Bound `map`, `rebind`,
`associated`, and `associated_key` preserve explicit tracing rather than create
dynamic prototypes. `service_filter`/`dispatch_options` select service-realm
event listeners; global listeners bypass the filter.

Reference: [services and reflection API](../api/services-reflection.md).
Complete resource-backed example: [lifecycle.rs](../../../crates/cordis-core/examples/lifecycle.rs).
