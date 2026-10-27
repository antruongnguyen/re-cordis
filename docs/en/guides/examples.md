# Executable examples

[Tiếng Việt](../../vi/guides/examples.md) | [English home](../README.md)

Run these commands from the repository root. Framework integration uses the
same production core as applications; JSON, HTTP, cryptography, and expanded
Tokio features in these examples are **dev-dependencies**, not core APIs.

## Lifecycle: the framework tour

```sh
cargo run -p cordis-core --example lifecycle --locked
```

[Source](../../../crates/cordis-core/examples/lifecycle.rs).
Initializes an isolated store, injects a consumer, updates provider and consumer
configuration, and asserts final consumer reads precede exact provider cleanup.
This is the best complete example for learning framework semantics.

## Digest: offline file processing

```sh
cargo run -p cordis-core --example github_digest --locked -- \
  crates/cordis-core/tests/fixtures/github_snapshot.json /tmp/cordis-report.md
```

[Source](../../../crates/cordis-core/examples/github_digest.rs).
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
cargo run -p cordis-core --example github_collect --locked -- \
  OWNER/REPO /tmp/cordis-snapshot.json
```

[Source](../../../crates/cordis-core/examples/github_collect.rs).
Two bounded queries publish one complete latest snapshot. An optional interval
enables periodic mode; shutdown joins an admitted poll. No login, GitHub
mutation, or notification delivery is built in.

## Webhook: local signed replay only

[Source](../../../crates/cordis-core/examples/github_webhook.rs).
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
snapshot from `antruongnguyen/cordis-rust` and a nonempty snapshot from
`cordiverse/cordis` (52 issues, 100 PRs). The PR cap was reached.

Those observations are point-in-time application evidence, not committed
fixtures, complete repository inventory, new core parity evidence, or live
webhook delivery. Private session artifacts are not documentation dependencies.
