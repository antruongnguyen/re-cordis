# Console logging

[Tiếng Việt](../../vi/guides/console-logging.md) | [English home](../README.md)

`re-cordis-logger` is the experimental console satellite over the existing core
logger. Register stdout during plugin setup or a hosted root operation. The
owning Context removes the exporter during cleanup; the core still selects
levels, buffers records and preserves error identities. See the
[console API](../api/console-logger.md) for all defaults and exact behavior.

## Install and remove output

Add a local path dependency on `crates/re-cordis-logger` alongside
`re-cordis-core` and Tokio. This complete program explicitly chooses plain
output without timestamps so it produces reproducible stdout. For ordinary
terminal use, `ConsoleOptions::default()` enables automatic color detection
and record-time UTC timestamps.

```rust
use re_cordis_core::{BoxError, ExporterOptions, Plugin, Runtime};
use re_cordis_logger::{ColorMode, ConsoleOptions, Timestamp, register_stdout};

fn options() -> ConsoleOptions {
    ConsoleOptions { colors: ColorMode::Plain, timestamp: Timestamp::Off, ..Default::default() }
}

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let plugin = Plugin::new("console", |ctx, ()| async move {
                register_stdout(&ctx, options(), ExporterOptions::default())?;
                ctx.logger(Some("app")).info("plugin ready")?;
                Ok::<_, BoxError>(())
            });
            let fiber = root.plugin(&plugin, ())?;
            fiber.ready()?.await?;
            root.logger(Some("app")).warn("owned output")?;
            fiber.dispose()?.await?;
            let output = register_stdout(&root, options(), ExporterOptions::default())?;
            root.logger(Some("app")).info("manual output")?;
            output.remove()?;
            root.logger(Some("app")).info("buffer only")?;
            let report = root.drain()?.await?;
            assert!(report.failures().is_empty());
            assert!(root.get_effects()?.is_empty());
            Ok::<_, BoxError>(())
        }).await??;
        Ok::<_, BoxError>(())
    })
}
```

Output:

```text
[I] app plugin ready
[W] app owned output
[I] app manual output
```

Run the same application with
`cargo run -p re-cordis-logger --example console --locked` from the checkout.
All severities use stdout. Dropping the registration token is inert;
`remove()` releases that exact occurrence synchronously. Plugin disposal,
restart and root drain release their owned registrations. An already-selected
core exporter snapshot may finish after removal; future records omit it.

## Custom writers and failures

`register(&ctx, writer, options, filtering)` takes an owned `Write + 'static`
value, including local `Rc`-based test writers. Auto is plain for custom writers
and pure rendering; choose `Ansi16`/`Ansi256` explicitly or use
`ColorMode::resolve` with captured terminal facts. Plain means the adapter adds
no ANSI escapes; user-supplied escape sequences are preserved.

Each selected record uses `write_all` and one `flush`. Short writes and
interrupted writes follow the standard library contract. Slow I/O blocks that
local log call. A write/flush error appears in `LogError::Exporters`, and core
still attempts independent exporters. Partial bytes cannot be rolled back.
The elapsed baseline advances only after successful write and flush.
If a writer logs recursively, its nested export gets a visible I/O error;
the adapter neither panics on a borrow nor automatically logs that error again.

Use Rust `Display`, `format!` and `Debug` for message formatting. JavaScript
printf placeholders, object inspection and browser-console argument rendering
are not reconstructed from the core's already-formatted message. Error
cause/aggregate handling remains the existing `ErrorValue` API.
