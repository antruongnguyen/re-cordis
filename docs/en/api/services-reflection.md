# Services, bound calls, and reflection adapters

[Tiếng Việt](../../vi/api/services-reflection.md) | [API index](README.md)

Sources: [service.rs](../../../crates/re-cordis-core/src/service.rs),
[reflect.rs](../../../crates/re-cordis-core/src/reflect.rs).
Runnable guide: [services and scoping](../guides/services-and-scoping.md).

## Keys, requirements, and publications

| Operation | Contract |
|---|---|
| `ServiceKey::<T>::new(name)`, `name` | Name selects a slot; T checks its value contract |
| `ServiceKey::required` | Create a required `Dependency` |
| `Dependency::with_config(config)` | Attach typed injection configuration |
| `Context::provide(&key, value)` | Register a Fiber-owned exact publication; duplicate live providers fail |
| `Context::provide_checked(&key, value, check)` | Add a synchronous `&T -> Result<bool, E>` readiness predicate |
| `Context::set(&key, value)` | Replace current value; only exact providing Fiber may do so |
| `ServiceRegistration::id` | Borrow this occurrence's `PublicationId` |
| `ServiceRegistration::withdraw` | Claim withdrawal now; return a Completion joining exact dependents' teardown |
| `ServiceRegistration::into_effect` | Transfer cleanup through `Disposer::from_effect` |

Readiness false blocks injection. Predicate errors/panics are diagnosed against
the provider and block injection, not converted into successful readiness.
Publication during provider Loading is not available to injecting consumers.

Set preserves occurrence identity, does not notify consumers, and does not
recheck readiness. Withdraw/reprovide creates a new occurrence. Missing required
providers leave consumers Pending, not Failed.

## Lookup and lifetime

| Operation | Result / behavior |
|---|---|
| `Context::get` | `Result<Option<Rc<T>>, ServiceError>` using own/nearest declared committed authority; root bypass is explicit |
| `get_direct` | Same result shape; bypass authority, require available Active provider |
| `get_unchecked` | Direct read including Loading/Unloading providers still in the slot |
| `bind` | `Result<BoundService<T>, ServiceError>` with checked authority and exact definition |
| `bind_direct` | Bound read bypassing authority but requiring availability |

Checked lookup prefers matching own provisions, then the nearest declaration's
committed view, walking definition ancestry. An inactive declaration is an
error, not permission to fall back. Root absent reads return None.
An `Option<T>` service value can represent an explicitly optional value while
publication presence stays distinct.

Returned pointers are value snapshots, not leases on resource lifetime.
Committed injection retains the exact provider through setup/rollback/cleanup,
including retired Registry consumers. Old cleanup can still use its withdrawn
publication until it releases that dependency.

## BoundService and scope

| Operation | Contract |
|---|---|
| `call` | Run an adapter receiving `(&T, Context)` |
| `context`, `caller` | Inspect traced Context and accessing code's definition Context |
| `call_unshadowed` | Adapter receives value, unshadowed use Context, and caller |
| `rebind` | Retrace at a new use site in the same runtime; no nested definition shadow |
| `map` | Extend a value/holder while preserving definition/use views |
| `intercepts::<C>` | Nearest injection config first, then outer-to-inner use-site layers |
| `resolve_intercept` | Pass the typed layers to your explicit merge function |
| `dispatch_options` | Service-realm listener filter for this use site |
| `associated(property)` | Bind dotted service name with original accessing authority |
| `associated_key(&key)` | Bind explicit extension namespace with original authority |
| `read_accessor`, `write_accessor` | Use original authority and typed receiver |
| `invoke(args)` | Call `ServiceInvoke<A>` through the traced Context |

`ServiceInvoke<A>` declares associated `Output` and
`invoke(&self, Context, A) -> Result<Self::Output, BoxError>`.
Native Rust method calls are not automatically rebound.

Context `isolate`/`isolate_with` select slots with no parent fallback, including
an injection-authority boundary when reusing an existing ID.
`intercept` adds typed layers. Own provisions do not borrow injected config.
Config-contract mismatches return errors rather than silently skipping layers.
`Context::service_filter` creates the equivalent realm dispatch filter;
global listeners still bypass filters.

## Typed accessors

| Operation | Contract |
|---|---|
| `AccessorKey::<R, T>::new(name)`, `name` | Global property name with receiver/value contract |
| `Accessor::new(get)` | Read-only adapter `(Context, Rc<R>) -> Result<T, E>` |
| `with_setter(set)` | Add `(Context, Rc<R>, T) -> Result<(), E>` |
| `from_service(key, get)` | Typed mixin adapter receiving `(&S, Context, Rc<R>)` |
| `Context::accessor(&key, accessor)` | Register an owner-scoped declaration and return its Effect |
| `read_accessor(&key, Rc<R>)` | Read using explicit receiver and accessing authority |
| `write_accessor(&key, Rc<R>, value)` | Write through the declared setter |

Use ordinary interior mutability for receiver mutation. These are explicit
adapters, not dynamic prototypes or arbitrary property reflection.

## Errors

`ServiceError`: admission/hook failure, duplicate provider, type mismatch,
undeclared/inactive access, missing provision, or wrong owner.
`ReflectError`: admission, duplicate/missing declaration, contract mismatch,
readonly access, or callback failure.

See [errors](errors-hooks.md) for source inspection and matching guidance.
