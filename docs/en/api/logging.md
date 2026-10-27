# Logging

[Tiếng Việt](../../vi/api/logging.md) | [API index](README.md)

Source: [logger.rs](../../../crates/cordis-core/src/logger.rs).
Runnable guide: [logging](../guides/logging.md).

## Logger and configuration

| API | Contract |
|---|---|
| `Context::logger(Option<&str>)` | Acquire a non-owning channel; name/settings are cached |
| `Logger::with_level` | Clone a handle with explicit verbosity override |
| `log`, `error`, `warn`, `info`, `debug` | Synchronously format Display input; return `Result<LogRecord, LogError>` |
| `log_error(level, ErrorValue)` | Emit adapted error tree; return `Result<Vec<LogRecord>, LogError>` |
| `Context::log_buffer` | Snapshot last 1000 admitted records chronologically |

Logging needs a live runtime, not an active run scope. Exporter
registration/removal does require host admission.
Severity order is `Level::Error`, `Warn`, `Info`, `Debug`; a maximum verbosity
admits levels up to and including that level.

`LoggerConfig` has optional public `name` and `level` fields. Missing fields
inherit outer layers. Explicit acquisition name beats intercept name, which
beats normalized definition-Fiber name. Default level is Info.
Use logger intercepts on `ServiceKey::<Logger>::new("logger")` with
`Context::intercept`; no custom logger publication is required for this override.

## Records and formatting

`LogRecord` exposes:

| Fields | Meaning |
|---|---|
| `sequence: u64`, `timestamp: SystemTime` | Runtime-local sequence starting at one; wall-clock timestamp |
| `name`, `level`, `message` | Channel, severity, Rust-formatted text |
| `fiber: FiberId`, `fiber_ref: WeakFiber` | Attribution without lifecycle retention |
| `failure: Option<CleanupFailure>` | Original explicit/automatic error source |

`WeakFiber::snapshot` returns current `Option<hooks::FiberInfo>`; a retained
Disposed handle can still supply metadata until its allocation is released.
Root metadata belongs to the permanent root.

`LogRecord::format(max_length)` applies a per-line UTF-16 budget and appends
`...` on truncation, normalizing CRLF and preserving Unicode scalars.
Use 10240 for the upstream default. `color(value, depth)` applies optional
channel coloring with `ColorDepth::None`, `Ansi16`, or `Ansi256`. No ANSI
styling is automatically inserted into the buffer.

## ErrorValue

| Builder | Meaning |
|---|---|
| `new(error)` | Own a Rust error |
| `from_rc(Rc<dyn Error>)` | Preserve exact shared error identity |
| `with_cause(value)` | Log cause before outer error; takes precedence over members |
| `with_members(Vec<ErrorValue>)` | Flatten nonempty aggregate members recursively |

An empty member list still logs the original error. Display logging does not
automatically discover arbitrary native cause/aggregate graphs or interpret
JavaScript formatting directives.

## Exporters

`Context::add_exporter(options, callback)` registers an owner-scoped callback
`&LogRecord -> Result<(), E>` and returns `ExporterRegistration`.
`remove` removes that exact exporter synchronously; `into_effect` transfers its
cleanup into composition. Drop is inert.

`ExporterOptions` exposes `default_level: Option<Level>` and
`levels: BTreeMap<String, Level>`. Exact-name limit wins over exporter default,
which wins over the channel level. Routing is independent per exporter;
buffer admission and exporter selection are not the same operation.

All selected exporters are attempted. `LogError::Exporters` retains all failed
callback sources. Other variants cover RuntimeDropped, formatting unwind,
incompatible configuration, and sequence exhaustion.
An error does not mean every destination received nothing.
