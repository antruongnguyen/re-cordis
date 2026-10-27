# Runtime, Context, and identities

[Tiếng Việt](../../vi/api/runtime-context.md) | [API index](README.md)

Sources: [runtime.rs](../../../crates/cordis-core/src/runtime.rs),
[context.rs](../../../crates/cordis-core/src/context.rs),
[identity.rs](../../../crates/cordis-core/src/identity.rs).

## Runtime

| Operation | Contract |
|---|---|
| `Runtime::new`, `Default` | Create an independent host and permanent root; construction needs no executor |
| `context` | Return a cheap non-owning root Context |
| `run(future)` | Drive local host tasks until the supplied future finishes; returns `Result<F::Output, HostError>` |
| `take_failures` | Take retained framework callback failures as `Vec<CleanupFailure>` |

`Runtime` is the explicit, non-Clone owner. It must be driven from a Tokio
entry point, not nested, concurrently driven, or driven from a spawned task.
If the supplied future returns `Result`, `run` adds a separate outer host-error
layer; handle both.

Returning from `run` pauses rather than cleans up remaining local work.
Destroying Runtime cancels it. Contexts and Logger handles do not keep it alive.

## Context identity and metadata

| Operation | Contract |
|---|---|
| `runtime_id`, `fiber_id` | Borrow opaque identities for the selected runtime/owner |
| `name` | Read the selected Fiber's diagnostic name |
| `extend(value)` | Add an immutable typed metadata layer |
| `metadata::<T>` | Return nearest same-type metadata as `Option<Rc<T>>` |
| `root` | Select the permanent root and discard derived metadata/scope layers |
| `definition_site` | Dependency-authority view of a traced call |
| `use_site` | Unshadowed use-site view for metadata, isolation, intercepts, effects |
| `caller_site` | Definition view of the calling code |

Metadata and identities remain inspectable after host destruction, but operations
requiring runtime state return `RuntimeDropped`. Never equate Context cloning
with resource ownership or host lifetime.

## Context scope

| Operation | Contract |
|---|---|
| `isolate(&key)` | Derive a fresh service-slot boundary |
| `isolate_with(&key, &id)` | Derive a boundary selecting a shared slot |
| `isolation(&key)` | Resolve selected `IsolationId`; requires the runtime to exist |
| `intercept(&key, config)` | Add typed use-site configuration without dynamic method rewriting |

Reusing a slot ID does not remove the boundary for inherited injection authority.
No isolated slot falls back to a parent slot. See
[services/reflection](services-reflection.md) for merge and tracing rules.

## Context extensions by subsystem

| Concern | Methods |
|---|---|
| [Plugins](plugins-registry.md) | `plugin`, `inject`, `inject_method`, `registry`, `on_update`, `on_update_sync` |
| [Resources](effects.md) | `on_dispose`, `on_dispose_sync`, `effect`, `effect_iter`, `effect_async`, `effect_stream`, `get_effects`, `drain` |
| [Services](services-reflection.md) | `provide`, `provide_checked`, `set`, `get`, `get_direct`, `get_unchecked`, `bind`, `bind_direct`, `service_filter`, `accessor`, `read_accessor`, `write_accessor` |
| [Events](events.md) | `on`, `on_with`, `once`, `emit`, `parallel`, `serial`, `bail`, `waterfall`, `waterfall_async` |
| [Logging](logging.md) | `logger`, `log_buffer`, `add_exporter` |

## Identities and thread boundaries

`RuntimeId`, `FiberId`, and `PublicationId` are framework-issued. Construct
shareable isolation identities with `IsolationId::new`/`Default`. Clone an
identity to share that exact identity; equality is not based on display text,
Rust value type, or pointer strings.

Framework Contexts/handles/futures are local, not a `Send`/`Sync` API.
User-spawned tasks do not inherit framework admission. See
[concepts](../concepts.md) and [host/admission errors](errors-hooks.md).
