# Console logger satellite

[Tiếng Việt](../../vi/api/console-logger.md) | [API index](README.md)

Source: [lib.rs](../../../crates/re-cordis-logger/src/lib.rs).
Guide: [console logging](../guides/console-logging.md).
Example: [console.rs](../../../crates/re-cordis-logger/examples/console.rs).

`re-cordis-logger` (`0.1.0-alpha.1`, unpublished, Rust 2024, MSRV 1.85) corresponds
to TypeScript `logger-console`. It adds no Logger service, filter store,
buffer, hidden runtime or background task. Calendar formatting uses
`time = 0.3.36` with only std/formatting features; core dependencies are unchanged.

## Registration and pure rendering

| Function | Signature / result | Contract |
|---|---|---|
| `register` | `(&Context, W, ConsoleOptions, ExporterOptions) -> Result<ExporterRegistration, AdmissionError>`, `W: Write + 'static` | Owned writer; local values allowed; Auto is plain. |
| `register_stdout` | `(&Context, ConsoleOptions, ExporterOptions) -> Result<ExporterRegistration, AdmissionError>` | All levels to stdout; resolves Auto once at registration. |
| `render` | `(&LogRecord, &ConsoleOptions, Option<SystemTime>) -> io::Result<String>` | Pure byte rendering including final newline; caller supplies elapsed baseline. |

Registration requires the correct `Runtime::run` host and a live owner.
Refusal releases the supplied writer without exporting. The returned core
token has inert Drop; `remove()` consumes it and removes its exact occurrence.
Owner cleanup/restart removes the occurrence and releases its captures.
Core snapshot selection permits an already-selected callback to finish after
removal. A disposed occurrence's token can subsequently return `false` on removal.

## `ConsoleOptions`, `LabelStyle`, `Alignment`, `Timestamp`

| Field | Default | Behavior |
|---|---|---|
| `colors` | `ColorMode::Auto` | Stdout capability detection; plain for custom writers/render. |
| `max_length` | `10240` | Each message line in UTF-16 units, excluding appended `...`; zero is a real limit. |
| `timestamp` | `Timestamp::Utc` | Record time, fixed UTC `yyyy-MM-ddTHH:mm:ss.SSSZ `; `Off` omits it. |
| `show_diff` | `false` | Append elapsed integer milliseconds, including ` +0ms` for the first or a backward record. |
| `label.width` | `0` | Minimum UTF-16 width; preserve longer names. |
| `label.margin` | `1` | Spaces between prefix, label and message. |
| `label.align` | `Alignment::Left` | Prefix first; `Right` puts padded label before prefix. |

Severity prefixes are `[E]`, `[W]`, `[I]`, `[D]`. Core formatting
normalizes CRLF and avoids splitting Unicode scalars. Left continuations
indent by timestamp length + 3 + margin; right continuations additionally
use nominal label width + margin, matching the upstream asymmetric layout.
UTF-16 padding is deterministic, not terminal-cell/grapheme width measurement.
ANSI256 labels are bold; ANSI16 labels retain ordinary weight.
Timestamp gray uses SGR90 in ANSI16 and SGR38;5;8 in ANSI256.
Plain adds no escapes and preserves escapes already present in user text.

UTC uses record time instead of upstream local render-time templates. Custom
time templates/timezones are outside this slice. UTC years outside 0–9999
and unrepresentable layouts return `InvalidInput`. Submilliseconds are truncated.
Elapsed output always uses integer milliseconds rather than upstream's
humanized duration formatter; successful backward writes become the next baseline.

## `ColorMode`

`Plain`, `Ansi16` and `Ansi256` are explicit and override environment facts.
`Auto` on stdout captures `IsTerminal` and Unicode environment values once:

1. Present `FORCE_COLOR`: `0` plain, `1` ANSI16, `2`/`3` ANSI256;
   other values (including empty) request ANSI16.
2. Nonempty `NO_COLOR` disables color.
3. Non-terminal stdout or `TERM=dumb` disables color.
4. `TERM` containing `256color` selects ANSI256; other terminals select ANSI16.

Non-Unicode values are ignored. Truecolor depth 3 is capped at the core ANSI256
palette; browser-console/Windows-specific support heuristics are not emulated.
`ColorMode::resolve(terminal, no_color, force_color, term)` applies the same
policy to captured facts without process-state reads, returning core `ColorDepth`.
Pure rendering and custom writers treat unresolved Auto as plain.

## I/O and reentry

The adapter synchronously calls `write_all`, then one `flush`. It has no queue:
slow writers block the log call. Standard short/interrupted writes are handled
by `write_all`; write-zero, write and flush failures remain inspectable original
I/O sources in `LogError::Exporters`. Independent exporters still run.
Partial bytes and a failed flush cannot be rolled back and are not retried by
the adapter. The previous record timestamp advances only after both succeed.

A recursive writer export returns an I/O error via core exporter failure
handling. It does not acquire a second mutable borrow or create automatic
recursive diagnostics. Writer destructors run when core releases its callback
snapshot; synchronous removal normally releases immediately, but an in-flight
snapshot delays release until its selected callback finishes.

## Example

```rust
use std::time::{Duration, UNIX_EPOCH};
use re_cordis_core::{BoxError, Runtime};
use re_cordis_logger::{ColorMode, ConsoleOptions, render};

fn main() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    let mut record = runtime.context().logger(Some("app")).info("hello\nworld")?;
    record.timestamp = UNIX_EPOCH + Duration::from_millis(123);
    let options = ConsoleOptions {
        colors: ColorMode::Plain,
        show_diff: true,
        ..Default::default()
    };
    let text = render(&record, &options, Some(UNIX_EPOCH))?;
    assert_eq!(
        text,
        "1970-01-01T00:00:00.123Z [I] app hello\n                             world +123ms\n"
    );
    Ok(())
}
```
