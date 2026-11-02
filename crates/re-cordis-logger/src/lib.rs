//! Synchronous, owner-scoped console output over the core logger.
//!
//! Register during `Runtime::run`. All filtering, record buffering, error trees,
//! and removal remain core contracts. A slow writer blocks the local log call.
//! See the console guide and the `console` example for complete hosted usage.

mod render;
mod writer;

use std::io::{self, IsTerminal, Write};
use std::time::SystemTime;

use re_cordis_core::{
    AdmissionError, ColorDepth, Context, ExporterOptions, ExporterRegistration, LogRecord,
};

/// Output color policy. Truecolor is deliberately reduced to the core 256 palette.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorMode {
    /// Detect stdout capability once at registration; plain for custom writers/rendering.
    #[default]
    Auto,
    /// Emit no ANSI escapes.
    Plain,
    /// Use the core six-color foreground palette.
    Ansi16,
    /// Use the core 256-color palette.
    Ansi256,
}

impl ColorMode {
    /// Resolve captured terminal/environment facts without reading process state.
    ///
    /// Explicit modes win. Auto precedence: FORCE_COLOR, nonempty NO_COLOR,
    /// non-terminal/TERM=dumb, TERM containing 256color, then ANSI16.
    /// FORCE_COLOR accepts 0, 1, 2, 3; other present values request ANSI16.
    /// Depth 3 is capped at ANSI256 because core has no truecolor helper.
    pub fn resolve(
        self,
        terminal: bool,
        no_color: Option<&str>,
        force_color: Option<&str>,
        term: Option<&str>,
    ) -> ColorDepth {
        match self {
            Self::Plain => ColorDepth::None,
            Self::Ansi16 => ColorDepth::Ansi16,
            Self::Ansi256 => ColorDepth::Ansi256,
            Self::Auto => {
                if let Some(force) = force_color {
                    return match force {
                        "0" => ColorDepth::None,
                        "2" | "3" => ColorDepth::Ansi256,
                        _ => ColorDepth::Ansi16,
                    };
                }
                if no_color.is_some_and(|value| !value.is_empty())
                    || !terminal
                    || term == Some("dumb")
                {
                    ColorDepth::None
                } else if term.is_some_and(|value| value.contains("256color")) {
                    ColorDepth::Ansi256
                } else {
                    ColorDepth::Ansi16
                }
            }
        }
    }

    pub(crate) fn depth(self) -> ColorDepth {
        self.resolve(false, None, None, None)
    }
}

/// Label alignment; affects continuation indentation as in the upstream renderer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Alignment {
    /// Severity before the padded label; continuations indent to the label start.
    #[default]
    Left,
    /// Padded label before severity; continuations indent to the nominal message start.
    Right,
}

/// Minimum label width and the spaces around its severity prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelStyle {
    /// Minimum width in UTF-16 units. Longer names are preserved.
    pub width: usize,
    /// Spaces between prefix, label and message.
    pub margin: usize,
    /// Label position relative to severity.
    pub align: Alignment,
}

impl Default for LabelStyle {
    fn default() -> Self {
        Self {
            width: 0,
            margin: 1,
            align: Alignment::Left,
        }
    }
}

/// Timestamp rendering policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Timestamp {
    /// UTC RFC3339 with exactly three fractional digits, using the record timestamp.
    #[default]
    Utc,
    /// Omit the timestamp and its following space.
    Off,
}

/// Renderer configuration. Export filtering uses core `ExporterOptions` separately.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsoleOptions {
    /// Default Auto: detected for stdout, plain for custom writers and pure rendering.
    pub colors: ColorMode,
    /// Per-line UTF-16 limit, excluding the ellipsis; default 10240, zero is a real limit.
    pub max_length: usize,
    /// Timestamp policy; default fixed UTC rather than a local-time template.
    pub timestamp: Timestamp,
    /// Append elapsed record time; first/backward deltas are zero milliseconds.
    pub show_diff: bool,
    /// Label width, margin and alignment.
    pub label: LabelStyle,
}

impl Default for ConsoleOptions {
    fn default() -> Self {
        Self {
            colors: ColorMode::Auto,
            max_length: 10240,
            timestamp: Timestamp::Utc,
            show_diff: false,
            label: LabelStyle::default(),
        }
    }
}

/// Render one record, including a final newline, without changing elapsed-time state.
///
/// `previous` is the last successfully written/flushed record timestamp. Passing
/// None yields +0ms when enabled. Backward timestamps also yield +0ms. Custom
/// renderers use explicit color; Auto is plain. Timestamp or layout values outside
/// the representable range return `io::ErrorKind::InvalidInput`.
pub fn render(
    record: &LogRecord,
    options: &ConsoleOptions,
    previous: Option<SystemTime>,
) -> io::Result<String> {
    render::render(record, options, previous)
}

/// Register an owned local writer on the Context's owner.
///
/// Requires core host/owner admission; no Send bound. Uses `write_all`, then one
/// `flush` per selected record. Partial/interrupted writes follow std's Write
/// contract. Errors propagate through `LogError::Exporters`; failed bytes cannot
/// be rolled back and delta state advances only after a successful flush.
/// Recursive calls into this writer return an error instead of panicking.
/// Dropping the returned token is inert; call `remove` or dispose the owner.
/// Core's selected callback snapshot may finish after removal.
pub fn register<W: Write + 'static>(
    ctx: &Context,
    writer: W,
    options: ConsoleOptions,
    filtering: ExporterOptions,
) -> Result<ExporterRegistration, AdmissionError> {
    writer::register(ctx, writer, options, filtering)
}

/// Register stdout for every severity, detecting Auto color once.
///
/// FORCE_COLOR > nonempty NO_COLOR > terminal/TERM capability; explicit color
/// bypasses these facts. Environment values which are not Unicode are ignored.
/// There is no stderr split, global installation, worker or background queue.
pub fn register_stdout(
    ctx: &Context,
    mut options: ConsoleOptions,
    filtering: ExporterOptions,
) -> Result<ExporterRegistration, AdmissionError> {
    let stdout = io::stdout();
    let depth = options.colors.resolve(
        stdout.is_terminal(),
        std::env::var("NO_COLOR").ok().as_deref(),
        std::env::var("FORCE_COLOR").ok().as_deref(),
        std::env::var("TERM").ok().as_deref(),
    );
    options.colors = match depth {
        ColorDepth::None => ColorMode::Plain,
        ColorDepth::Ansi16 => ColorMode::Ansi16,
        ColorDepth::Ansi256 => ColorMode::Ansi256,
    };
    register(ctx, stdout, options, filtering)
}
