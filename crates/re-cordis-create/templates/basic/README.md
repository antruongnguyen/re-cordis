# {{name}}

A small local Cordis application: a current-thread Tokio executor drives
`Runtime::run`; a typed loader plugin publishes a greeting service and logs it.
The application updates its typed config, reads the new service value, joins
loader disposal and the root drain, then checks Runtime cleanup diagnostics.

Run from this project directory:

```sh
cargo test --offline
cargo run --offline --locked
```

Rust 1.85 or newer is required. The first test resolves a Cargo.lock from cached
dependencies; if the cache is empty, run `cargo test` once with network access.
The generator itself performs no downloads or Cargo/Git processes.
The manifest contains canonical absolute paths to the explicit Cordis checkout.
Keep that checkout available; moving it requires editing those three paths.
The package is unpublished and has its own workspace boundary.

Edit `Settings`, `entry` and the greeting plugin in `src/app.rs` to build your
application. Preserve joined shutdown on both success and failure. Root cleanup
reports and `Runtime::take_failures` are separate error channels: a successful
joined waiter alone does not establish error-free cleanup.

The included tests exercise config update, service and writer release, cleanup
causes, and 100 deterministic lifecycle cycles. This basic template has no
file persistence, watcher, timer, signal loop, Git, install or run option.
