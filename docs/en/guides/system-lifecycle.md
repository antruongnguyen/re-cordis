# Whole-program lifecycle

[Tiếng Việt](../../vi/guides/system-lifecycle.md) | [English home](../README.md)

The R0 reference application combines local core hosting, console logging, a
typed provider and injecting consumer, owner-scoped timers, structural loader
groups, Include configuration/persistence and optional HMR watching. It uses
existing satellite APIs. Its application module is example-private, not a new
crate, facade, public API or distributed Create template.

## Run the application and tests

From the checkout:

```sh
cargo run -p re-cordis-hmr --example system_lifecycle --features include --locked --offline
cargo run -p re-cordis-hmr --example system_lifecycle --features include --locked --offline -- --watch
cargo test -p re-cordis-hmr --test system_lifecycle --features include --locked --offline
```

The [entry point](../../../crates/re-cordis-hmr/examples/system_lifecycle.rs),
[shared application](../../../crates/re-cordis-hmr/examples/system_lifecycle/app.rs)
and [tests](../../../crates/re-cordis-hmr/tests/system_lifecycle.rs) are the
executable reference. A current-thread Tokio executor drives Runtime::run
until joined cleanup finishes. Files belong to temporary offline fixtures;
no credentials, network, compiler daemon or process supervisor are needed.

The integration suite uses paused time and explicit cleanup gates for causal
assertions. Repeated lifecycle tests exercise at least 100 cycles. Real file
polling is a bounded supplemental check, not the scheduler for deterministic tests.

## Follow reconfiguration through cleanup

Include edits serialized configuration through JsonEdit and moves entries
through move_entry. The provider moves from one isolated structural group to
another. An old injecting consumer holds cleanup at a gate and reads its exact
committed provider while that provider is retiring. An unrelated sibling can
update and its timer can progress before the gate opens.

After release, the provider rebuilds in the destination scope. The old
consumer is Pending until it is moved to the destination scope; it then binds
the new provider. Stable loader EntryId and changed Fiber/publication identity
have different purposes. Holding an Rc alone is not a lease on a provider's
effects; the committed injection provides the retention guarantee.

See [groups](groups.md), [services and scoping](services-and-scoping.md) and
[Include](include.md) for the underlying contracts.

## Inspect each error channel

| Observation | Meaning |
|---|---|
| Operation Result | Admission, decoding, reconfiguration or joined-operation failure |
| Include/linked-replacement Report failures | Independent failed entries; healthy siblings can succeed |
| WatchEvent::Failure history | Returned refresh errors and Report failures from watched work |
| Include::final_error | Retained final persistence error, including when a waiter was dropped |
| CleanupReport and Runtime::take_failures | Cleanup outcome and original hosted callback/cleanup causes |

A successful waiter does not establish error-free cleanup. The application and
tests inspect these channels at their documented barriers. One cause observed
through more than one channel is still one failure occurrence. Explicit update
or a new linked candidate recovers Failed state; provider arrival alone is not
implicit recovery.

ReloadPlan supplies already-compiled candidates to the same Include-managed
loader. It does not compile Rust or replace native binary code. See
[HMR](hmr.md) and the [error reference](../api/errors-hooks.md).

## Shut down admitted work

Orderly Application::shutdown first stops and joins the watcher while Include
can finish already-admitted refresh work. It then disposes/joins Include and the
application owner, including timers and writer; every stage's errors remain visible.
A stop request or dropped token alone does not join a refresh. The tests separately
exercise forced ancestor cancellation and inspect the retained cancellation cause.
Include disposal flushes its pending journal before child cleanup; inspect
persistence errors and the resulting file bytes. Release retained handles before
asserting capture/weak-reference destruction.

Synchronous Write and ordinary filesystem calls can block the local executor.
Flush/replacement is not fsync, a multi-file transaction, or protection against
hostile filesystem races. A blocked OS read can delay the watcher join.
Runtime filesystem/watcher evidence covers macOS only.

## Verify a real generated application

```sh
python3 -B tests/docs/test_check.py
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
```

The checker first invokes the delivered Create CLI with an explicit checkout
path, validates and builds/tests/runs the unchanged basic application, then
extends only that temporary external project with explicit local timer,
Include and HMR dependencies. It copies the exact R0 entry point, shared
application and tests into that project and builds/tests/runs them with the
selected rustup toolchain. Process arguments are arrays; paths with spaces and
Vietnamese characters are exercised. Cargo dependency resolution is offline
and requires dependencies already cached.

The production basic template remains unchanged. There is no additional
distributed template variant. [Create](create.md) explains ordinary scaffolding;
[R0 evidence](../../plans/2026-11-02-cordis-completion/evidence/r0-acceptance.md)
records the executed matrix, review closure and source dispositions. Existing paired core and loader traces
retain their own bounded evidence; this application is not a new TS parity suite.
