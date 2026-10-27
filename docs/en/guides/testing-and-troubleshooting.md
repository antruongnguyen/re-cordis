# Testing and troubleshooting

[Tiếng Việt](../../vi/guides/testing-and-troubleshooting.md) | [English home](../README.md)

## Test observable barriers

Host each test at a Tokio entry point and drive one `Runtime::run` scope.
Use channels/gates to control async setup and cleanup, not sleeps. Assert the
returned lifecycle state when Active or Disposed is required.

Useful assertions include exact apply/cleanup counts, the final old-provider
read preceding provider close, no diagnostics on successful paths, and restored
Registry/effect baselines after teardown. `Registry::len` counts declaration
groups, while `instances` returns identities for one declaration.

For more complex cases, reuse patterns in
[updates](../../../crates/cordis-core/tests/updates.rs),
[service disposal](../../../crates/cordis-core/tests/service_dispose.rs), and
[lifecycle parity](../../../crates/cordis-core/tests/lifecycle_parity.rs).
Test code may use dev-only Tokio features; application dependencies must enable
the features they use.

## Common symptoms

| Symptom | Check / action |
|---|---|
| `HostNotRunning` | Admit work inside the owning runtime's `run` scope |
| `WrongHost` | Do not mix Contexts, bound services, or operations from independent runtimes |
| `MissingExecutor` / `SpawnedTask` | Drive `run` from Tokio block_on/main/test, not a spawned task |
| `NestedHost` / `AlreadyRunning` | Do not nest or concurrently drive Cordis hosts |
| Fiber remains Pending | Check exact names, isolation slots, readiness predicates, and provider Active state |
| `ready` returned but consumer is not Active | Pending is a valid settlement; await provider then consumer separately |
| Fiber remains Failed after provider refresh | Explicitly `update` the Failed Fiber; `restart` does not clear failure |
| Invalid update changed nothing | Loaded validation deliberately preserves the previous good source |
| `Undeclared` / `Inactive` | Declare injection; do not bypass authority to hide a lifecycle error |
| `set` did not reload consumers | Set changes a value, not publication identity; use provider lifecycle replacement |
| Resource closed despite retained `Rc<T>` | A returned value is not a provider-resource lease; use committed injection |
| Event `UnsupportedMode` | Match responder versus around shape, and sync versus async mode |
| No terminal log output | Register an exporter; the default sink is an in-memory buffer |
| `ReentrantDrain` | Do not wait on self/owning-ancestor lifecycle during attributed setup/cleanup |
| `InactiveContext` | Stop creating work through a terminal owner's captured Context |
| `RuntimeDropped` / `HostStopped` | Keep the owner alive and driven through the operation's barrier |
| Cleanup completed but resources failed to close | Inspect `CleanupReport::failures` and retained runtime failures |

## Error investigation

Propagate immediate admission errors and await started-operation results.
Inspect structured variants and `std::error::Error::source`, not only formatted
strings. Several public error enums are non-exhaustive: include a fallback arm
when matching them downstream.

Core catches user callback unwinds where documented; it does not promise to
repair invariant panics, aborts, panicking destructors, or arbitrary `Rc` cycles.
Never use a broad catch or silent fallback to turn failed teardown into success.

## Repository checks

Run from the repository root:

```sh
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo doc -p cordis-core --no-deps --locked
```

For this environment's explicit MSRV/stable selection precautions, see
[M6 verification](../../m6.md#verification). Full paired TS execution is a
separate opt-in command with a pinned local checkout and prepared dependencies;
see [parity reproduction](../../lifecycle-parity.md#reproduce).

Documentation checks are described in [the documentation home](../../README.md#maintaining-these-documents).
They do not require live GitHub access.
