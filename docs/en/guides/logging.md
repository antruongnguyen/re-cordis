# Logging and diagnostics

[Tiếng Việt](../../vi/guides/logging.md) | [English home](../README.md)

## Add a terminal exporter

Core has structured records and a bounded buffer, not an implicit console
exporter. Register an exporter on the owner that should control its lifetime.

```rust
use cordis_core::{BoxError, ExporterOptions, Runtime};
use std::convert::Infallible;

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                root.add_exporter(ExporterOptions::default(), |record| {
                    println!("[{}] {}", record.name, record.format(10240));
                    Ok::<_, Infallible>(())
                })?;
                root.logger(Some("app")).info("ready")?;
                assert!(root.log_buffer()?.iter().any(|record| record.message == "ready"));
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

## Channel names and levels

`ctx.logger(Some("name"))` wins over use-site `LoggerConfig` intercepts.
Without an explicit name, intercept name wins over the normalized definition
Fiber name. Names are cached when acquiring the Logger; nested bound service
calls attribute to the relevant definition.

The default maximum verbosity is Info. `with_level` overrides a handle's level.
Each exporter can select its own default and exact-name limits through
`ExporterOptions`. Name-specific exporter routing wins over its default.
`LogError::Configuration` surfaces incompatible intercept types; the framework
does not silently ignore them.

## Records and errors

The buffer keeps the last 1000 admitted records in chronological order and
survives reusable root drains. Records include sequence, wall-clock timestamp,
channel, level, message, Fiber identity, weak metadata, and optional original
failure. `WeakFiber::snapshot` can expire; retaining a record does not keep a
runtime or Fiber alive.

Use Rust `format!`/Display for ordinary messages. JavaScript printf/JSON
formatting is not interpreted. Use `Logger::log_error` with `ErrorValue` when
explicit cause/aggregate structure is needed:

- A cause is logged before its outer error and takes precedence over members.
- Nonempty member lists flatten recursively instead of logging the aggregate.
- Empty member lists still log the original error.

This is explicit adaptation, not automatic inspection of every native
`Error::source` graph.

## Formatting and exporter failures

`LogRecord::format` limits each line by UTF-16 units, normalizes CRLF, and never
splits a Unicode scalar. Zero is a real zero budget. `color` supports plain,
ANSI-16, and ANSI-256 modes; color remains an exporter choice.

All selected exporters are attempted even if one fails. Handle `LogError`;
failure does not imply other exporters received nothing. Framework diagnostics
also report to stderr and retain failures without recursively feeding a broken
diagnostic exporter.

`Runtime::take_failures` drains retained framework callback failures, not the
log buffer. An application must inspect/handle them as appropriate.

Reference: [logging API](../api/logging.md).
