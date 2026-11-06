# Executable examples

[Tiếng Việt](../../vi/guides/examples.md) | [English home](../README.md)

Run these commands from the repository root. Framework integration uses the
same production core as applications; JSON, HTTP, cryptography, and expanded
Tokio features in crate examples are **dev-dependencies**, not core APIs.
A generated application declares its own dependencies.

## Whole-program integration

Run `cargo run -p re-cordis-hmr --example system_lifecycle --features include --locked --offline`.
The [integration guide](system-lifecycle.md) covers the shared application,
provider moves, persistence, recovery, pending work and generated-project checks.

## Lifecycle: the framework tour

```sh
cargo run -p re-cordis-core --example lifecycle --locked
```

[Source](../../../crates/re-cordis-core/examples/lifecycle.rs).
Initializes an isolated store, injects a consumer, updates provider and consumer
configuration, and asserts final consumer reads precede exact provider cleanup.
This is the best complete example for learning framework semantics.

## Digest: offline file processing

```sh
cargo run -p re-cordis-core --example github_digest --locked -- \
  crates/re-cordis-core/tests/fixtures/github_snapshot.json /tmp/cordis-report.md
```

[Source](../../../crates/re-cordis-core/examples/github_digest.rs).
Validates a bounded snapshot and writes deterministic escaped Markdown through
atomic same-directory replacement. Destination is a service; the injecting
processor retains it during processing. Input/destination updates are
programmatic through example-private handles, not public core APIs.

The fixture is synthetic. The report is a bounded activity index, not full
history or a day-over-day diff.

## Collector: optional authenticated read-only access

The collector invokes installed `gh` using existing authentication. Do not run
it as an automatic documentation check; it makes authenticated network requests.
With authorization for the selected repository, its command is:

```sh
cargo run -p re-cordis-core --example github_collect --locked -- \
  OWNER/REPO /tmp/cordis-snapshot.json
```

[Source](../../../crates/re-cordis-core/examples/github_collect.rs).
Two bounded queries publish one complete latest snapshot. An optional interval
enables periodic mode; shutdown joins an admitted poll. No login, GitHub
mutation, or notification delivery is built in.

## Webhook: local signed replay only

[Source](../../../crates/re-cordis-core/examples/github_webhook.rs).
The loopback-only receiver owns HTTP tasks and a serialized journal writer.
Use the [consumer contract and replay instructions](../../github-activity.md)
for the exact secret setup, limits, and commands. Never put a real secret into
source, documentation examples, or shell history.

HTTP 204 follows append/flush, not fsync. Duplicate deliveries remain;
disconnect/deadline can occur before a later append. Public ingress and actual
GitHub webhook configuration are outside this example's scope.

## Evidence boundary

Offline integration tests exercise these exact implementations. Separately,
on 2026-10-27, authorized one-shot collector/digest pilots observed an empty
snapshot from `antruongnguyen/cordis-rust` (the project's former name, since
renamed to `antruongnguyen/re-cordis`) and a nonempty snapshot from
`cordiverse/cordis` (52 issues, 100 PRs). The PR cap was reached.

Those observations are point-in-time application evidence, not committed
fixtures, complete repository inventory, new core parity evidence, or live
webhook delivery. Private session artifacts are not documentation dependencies.

## Console output

[console.rs](../../../crates/re-cordis-logger/examples/console.rs) demonstrates
plugin-owned stdout, explicit formatting, owner disposal, manual removal and
root drain. Run `cargo run -p re-cordis-logger --example console --locked`.
See [console logging](console-logging.md) for the same complete program.

## Loader boot and reconfiguration

Run `cargo run -p re-cordis-loader --example boot_reconfigure --locked`.
The [example](../../../crates/re-cordis-loader/examples/boot_reconfigure.rs)
boots a structural group, waits Pending, admits a provider, updates source,
fails/recover explicitly and joins shutdown. No files or network are used.

## Include read, change and persistence

Run `cargo run -p re-cordis-include --example config_file --locked --offline`.
The [example](../../../crates/re-cordis-include/examples/config_file.rs) reads
a temporary file, changes a typed plugin config, flushes and joins shutdown.
See [configuration files](include.md) for the identical executable program.

## HMR configuration and compiled replacement

Run `cargo run -p re-cordis-hmr --example reload --features include --locked --offline`.
The [example](../../../crates/re-cordis-hmr/examples/reload.rs) watches a temporary
configuration through Include, replaces its compiled factory on the same EntryId,
verifies later config refresh, reports source RestartRequired and joins shutdown.
See [HMR](hmr.md#offline-example) for the supported layers and failure channels.

## Create a local application

Use [Create](create.md) to generate the fixed offline core/logger/loader
application from this checkout, then run its own tests and executable explicitly.
The [generator](../../../crates/re-cordis-create/src/lib.rs) includes typed
services/plugins, scoped console output, programmatic configuration, updates
and joined shutdown. Its generated tests cover release and error channels.
This is application scaffolding; Include/HMR variants and automatic Git/install/
launch are outside the selected S0–S2 scope.
