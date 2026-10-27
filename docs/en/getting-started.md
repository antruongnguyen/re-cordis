# Getting started

[Tiếng Việt](../vi/getting-started.md) | [English home](README.md)

## Requirements and dependency setup

Use Rust 1.85 or newer. `re-cordis-core` is not published to crates.io; do not
install an unrelated package with a similar name.

For an application next to a local `re-cordis` checkout:

```text
workspace/
  re-cordis/
  my-app/
    Cargo.toml
    src/main.rs
```

Use this application manifest:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"

[dependencies]
re-cordis-core = { path = "../re-cordis/crates/re-cordis-core" }
tokio = { version = "1.48", default-features = false, features = ["rt", "sync"] }
```

Adjust the path to your checkout. The crate import is `re_cordis_core`, with an
underscore. If your application uses Tokio timers, I/O, signals, or macros,
enable those features in **your application's** Tokio dependency; core's minimal
feature set does not enable them for you.

## A complete first application

Put this in `src/main.rs`, then run `cargo run` from `my-app`.

```rust
use re_cordis_core::{BoxError, Disposer, FiberState, Plugin, Runtime};
use std::convert::Infallible;

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                let worker = Plugin::initialized("Worker", |ctx, name: String| async move {
                    ctx.logger(None).info(format!("hello, {name}"))?;
                    Ok::<_, BoxError>(Disposer::sync(|| Ok::<_, Infallible>(())))
                });
                let fiber = root.plugin(&worker, String::from("Cordis"))?;
                assert_eq!(fiber.ready()?.await?, FiberState::Active);
                Ok::<_, BoxError>(())
            }.await;
            let cleanup = root.drain()?.await?;
            if let Some(failure) = cleanup.failures().first() {
                return Err(failure.clone().into());
            }
            result
        }).await?
    })
}
```

The logger records the greeting in the built-in buffer; it does not print normal
application messages to a terminal by default. See [logging](guides/logging.md)
to add an exporter.

## What happens

`Runtime` owns a local task host. Tokio supplies the executor;
`Runtime::run` drives the host while its supplied future runs. The root Context
is a cheap, non-owning view, not a second runtime.

`Plugin::initialized` defines a reusable component whose initialization returns
a `Disposer`. `root.plugin` admits a Fiber immediately and returns its handle.
`ready()?.await?` observes completion of that Fiber's current lifecycle pass.

The example attempts root cleanup after ordinary setup failure as well as
success. `drain` returns a report: reaching a cleanup barrier does not mean every
callback succeeded. When both application work and cleanup fail, this example
returns the cleanup failure; core diagnostics retain callback failures.

Returning from `run` does not automatically drain anything. Destroying `Runtime`
cancels its remaining tasks. Arrange explicit cleanup for every exit path your
application controls; panics, process termination, and host destruction are not
successful shutdown paths.

## Next

Read [concepts](concepts.md), then [plugins and updates](guides/plugins-and-updates.md).
For a complete provider/consumer tree, run the repository's existing example:

```sh
cargo run -p re-cordis-core --example lifecycle --locked
```

Run repository commands from the `re-cordis` root, not from `my-app`.
