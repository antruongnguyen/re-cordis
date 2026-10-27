# Plugins, Fibers, and Registry

[Tiếng Việt](../../vi/api/plugins-registry.md) | [API index](README.md)

Source: [plugin.rs](../../../crates/re-cordis-core/src/plugin.rs).
Runnable guide: [plugins and updates](../guides/plugins-and-updates.md).

## Plugin declarations and admission

| Operation | Contract |
|---|---|
| `Plugin::<C>::new(name, apply)` | `C: Clone`; identity validation; apply receives `(Context, C)` and returns a future of `Result<(), E>` |
| `Plugin::<C, R>::validated(name, validate, apply)` | Synchronous `&C -> Result<R, VE>`; apply receives resolved `R`; source need not be Clone |
| `Plugin::<C>::initialized(name, initialize)` | Cloneable source; await a future producing one `Disposer`, owning it through an effect |
| `Plugin::injecting(dependencies)` | Set required injections on this declaration view; preserve shared grouping identity |
| `Context::plugin(&plugin, source)` | Admit, attach parent cleanup, and return `Result<FiberHandle<C>, LifecycleError>` before load completion |
| `Context::inject(dependencies, apply)` | Parent-owned no-config consumer; callback receives Context |
| `Context::inject_method(dependencies, Rc<T>, method)` | Explicit method adapter receiving `(Rc<T>, Context)` in an injecting child |

Callbacks/futures are owned and `'static`; their errors convert into `BoxError`.
Cloning a declaration shares identity; independently creating another declaration
does not. Repeated injection names retain first position and last contract/config.
Required declarations may carry typed `Dependency::with_config`.

## FiberHandle

| Operation | Contract |
|---|---|
| `id`, `name` | Stable instance identity and captured diagnostic name |
| `state` | Current `FiberState` snapshot |
| `config` | Retained source `Rc<C>`, not resolved `R` |
| `context` | Non-owning Context for this Fiber |
| `ready` | Wait for the current pass without creating work |
| `restart` | Force reapply of same source; does not recover Failed |
| `update(source)` | Validate/commit source and converge; explicitly enables failure recovery |
| `update_with(source, options)` | Same operation with explicit commit-notification choice |
| `dispose` | Mark terminal/unlink residency now, then await owned teardown |

`ready`, `restart`, `update`, `update_with`, and `dispose` return
`Result<LifecycleTask, LifecycleError>`. The task yields
`Result<FiberState, LifecycleError>`. Dropping tasks/handles is not cancellation
or disposal.

`UpdateOptions` has public `no_save: bool`, default false. True suppresses only
source commit notification. Local `Context::on_update` receives `(Context,
Rc<R>, AsyncNext<bool>)`; omitting next accepts without restarting.
`on_update_sync` receives `Rc<R>` and returns true to continue or false to
accept without restart. Both return scoped listener registrations.

## States and convergence

`FiberState` variants: `Pending`, `Loading`, `Active`, `Failed`, `Unloading`,
`Disposed`. Pending means unavailable requirements; Failed means apply/validation
failed after rollback. Only Disposed is terminal.

Loaded update validation precedes source changes and middleware. Pending defers
validation. One driver coalesces updates; old apply/cleanup finishes before the
latest available load. Disposal defeats queued reloads. Obsolete load errors
are diagnosed without poisoning the latest target.

A readiness or update waiter can settle Pending. Await provider completion
then consumer readiness separately; no waiter is a global convergence barrier.
Failed consumers ignore dependency refresh; only update enables recovery.

## Registry

| Operation | Contract |
|---|---|
| `Context::registry` | Non-owning Registry view |
| `len`, `is_empty` | Count/check resident declaration groups |
| `contains(&plugin)` | Test this declaration's residency |
| `instances(&plugin)` | Snapshot `Vec<FiberId>` of this current group |
| `remove(&plugin)` | Retire current group and start all members' disposal; returns tasks to join |

Failed instances stay resident until disposal. New spawns after removal form a
new group; old cleanup cannot remove that new group. Parent cleanup owns children
independently of public handle lifetime.

## Failures

`LifecycleError` distinguishes `Admission`, `Callback(CleanupFailure)`, hook
errors, `HostStopped`, and `DriverPanicked`. Original callback sources remain
inspectable. Cleanup errors are diagnosed/retained without stranding unrelated
cleanup; a Disposed result is not proof every disposer succeeded.

Self/owning-ancestor lifecycle waits from attributed setup/cleanup are refused.
See [errors and hooks](errors-hooks.md) and [resource cleanup](effects.md).
