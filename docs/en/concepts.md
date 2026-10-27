# Concepts and execution model

[Tiếng Việt](../vi/concepts.md) | [English home](README.md)

## The object model

| Concept | Meaning |
|---|---|
| `Runtime` | Explicit owner of one local task host and framework state |
| `Context` | Non-owning view selecting metadata, scope, and a resource owner |
| `Plugin<C, R>` | Reusable declaration: source configuration `C`, validated apply input `R` |
| Fiber / `FiberHandle<C>` | One admitted plugin instance and its lifecycle control |
| `Registry` | Groups resident Fibers by reusable Plugin identity |
| `ServiceKey<T>` | Named service slot with a checked value contract `T` |
| Publication | One exact provision occurrence, identified by `PublicationId` |
| Effect | Owner-registered resource setup/cleanup |
| Event | Typed notification, query, or around-middleware contract |

A cloned Plugin retains declaration identity. Two separately constructed Plugins
are different even if their names and closure types match. Restarting a Fiber
preserves its `FiberId`; withdrawing and republishing a service creates a new
`PublicationId`, even when the value is equal.

## Hosting and ownership are separate

```text
Tokio executor
  -> Runtime::run drives local framework tasks
       -> Registry retains resident Fibers
       -> each Fiber owns effects and child-plugin cleanup

Context --weak--> runtime and selected lifecycle owner
```

Derived Contexts do not own their runtime. `extend` adds typed metadata;
`root` selects the permanent root and discards derived metadata. Metadata
ancestry, lifecycle ownership, service isolation, and dependency authority
are distinct relations.

Ordinary spawned Tokio tasks do not inherit framework host admission or
callback attribution. Do not solve a non-`Send` compiler error by assuming
`tokio::spawn` is an alternative hosting API.

## Lifecycle states

| State | Interpretation |
|---|---|
| `Pending` | Required publications are unavailable; validation is deferred |
| `Loading` | Validation/apply is in progress |
| `Active` | Apply completed successfully |
| `Failed` | Validation/apply failed and partial effects were rolled back |
| `Unloading` | Owned resources and children are being drained |
| `Disposed` | Terminal disposal completed |

Not every operation passes through all states. Pending/Failed disposal can go
directly to Disposed. `state()` is a snapshot, not a synchronization barrier.

## Started operations versus native futures

Methods such as `update`, `restart`, `dispose`, and `drain` admit/start work at
the method call. Their returned handles wait for already-started work.
Dropping a waiter does not cancel that work while the host remains driven.

Native Rust async bodies are still lazy. Callback factories may run an eager
prefix, but returning an async block does not execute its body immediately.
Use the explicit synchronous APIs when synchronous timing matters.

`ready` observes the current Fiber pass. It can return Pending before a later
provider arrival starts another pass. A provider waiter does not join every
consumer. Await the relevant provider, then the relevant consumer, and inspect
the returned state when Active is required.

## Resource lifetime

Children belong to their spawning owner. Effects, listeners, exporters, and
publications belong to their registering owner unless explicitly transferred.
Root drain is reusable and snapshot-based, not permanent runtime shutdown.

Committed injection retains the exact provider through a consumer's setup,
rollback, and final cleanup. A returned `Rc<T>` or `BoundService<T>` alone is
**not** a lease on provider resources.

See [cleanup](guides/effects-and-cleanup.md) and
[services](guides/services-and-scoping.md) before sharing resource-backed values.

## Compatibility boundary

Rust uses explicit bound calls, accessors, initialization, and method injection.
There is no automatic Proxy, prototype, decorator, class discovery, native hot
loading, async Drop, or thread-safe/executor-independent runtime.

These are intentional API boundaries. See [the compatibility audit](../m6.md)
for representations and safety deviations, and [the glossary](glossary.md) for
terminology shared across both languages.
