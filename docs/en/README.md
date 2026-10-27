# re-cordis documentation

[Tiếng Việt](../vi/README.md) | [Documentation home](../README.md) | [AI-agent index](../llms.txt)

Cordis is a framework for applications whose plugins, services, configuration,
and resources change over time. `re-cordis` expresses the TypeScript Cordis
core's semantics through explicit, typed Rust APIs.

The current deliverable is **unpublished `re-cordis-core`**, version `0.0.0`, Rust
2024, requiring Rust 1.85 or newer. The API is unstable. Timer, console-logger,
and loader satellite crates are not implemented. This is not transparent
JavaScript Proxy or decorator compatibility.
The [next-target roadmap](../todo.md#next-implementation-targets-satellites)
orders timer, console logger, then loader, with per-slice source/API/dependency
review and matching documentation. Planned crates are not usable APIs yet.

## Choose a reading path

| Your goal | Start here |
|---|---|
| Build your first application | [Getting started](getting-started.md), then [concepts](concepts.md) |
| Create reloadable components | [Plugins and updates](guides/plugins-and-updates.md) |
| Connect components safely | [Services and scoping](guides/services-and-scoping.md) |
| Own resources and shut down | [Effects and cleanup](guides/effects-and-cleanup.md) |
| Dispatch notifications and middleware | [Events](guides/events.md) |
| Export application diagnostics | [Logging](guides/logging.md) |
| Debug a Pending or Failed plugin | [Testing and troubleshooting](guides/testing-and-troubleshooting.md) |
| Study complete applications | [Examples](guides/examples.md) |
| Look up a type or operation | [API reference](api/README.md) |
| Resolve terminology | [Glossary](glossary.md) |

## The essential rules

Drive the framework through `Runtime::run` from a Tokio executor entry point.
Contexts and framework handles/futures are local, not `Send`/`Sync`.
Keep the runtime alive and driven until the work you need has finished.

Register resource cleanup with the owning Context. Dropping handles is not
disposal; destroying the runtime is cancellation, not asynchronous cleanup.
Check cleanup reports as well as operation errors.

Readiness is per Fiber, not global. A waiter can finish `Pending` when a
dependency is missing. Await provider completion and consumer readiness
separately. Failed consumers require explicit `update` to recover.

## Reference and evidence

The new guides describe the **current** Rust API. Generated Rustdoc provides
exact signatures and trait bounds; see the [API index](api/README.md).
The older [milestone documents](../README.md#engineering-references) preserve
historical scope and acceptance evidence. An old milestone's “not implemented
yet” statement is not a statement about today's API.

Compatibility is scoped to the pinned TypeScript core and documented Rust
representations. See [M6 restrictions](../m6.md#remaining-restrictions-and-safety-deviations)
and [executed lifecycle parity](../lifecycle-parity.md), not a claim of complete
upstream-suite or latest-upstream compatibility.
