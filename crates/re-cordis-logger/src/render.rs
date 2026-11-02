use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use re_cordis_core::{ColorDepth, Level, LogRecord};
use time::OffsetDateTime;

use crate::{Alignment, ConsoleOptions, Timestamp};

fn invalid() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "console timestamp or layout out of range",
    )
}

fn timestamp(value: SystemTime) -> io::Result<String> {
    let nanos = match value.duration_since(UNIX_EPOCH) {
        Ok(duration) => i128::try_from(duration.as_nanos()).map_err(|_| invalid())?,
        Err(error) => -i128::try_from(error.duration().as_nanos()).map_err(|_| invalid())?,
    };
    let date = OffsetDateTime::from_unix_timestamp_nanos(nanos).map_err(|_| invalid())?;
    if !(0..=9999).contains(&date.year()) {
        return Err(invalid());
    }
    // A fixed UTC format keeps byte output independent of timezone and locale.
    let format = time::format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z ",
    )
    .expect("constant timestamp format");
    date.format(&format).map_err(|_| invalid())
}

fn spaces(count: usize) -> io::Result<String> {
    let mut output = String::new();
    output.try_reserve(count).map_err(|_| invalid())?;
    output.extend(std::iter::repeat_n(' ', count));
    Ok(output)
}

pub(crate) fn render(
    record: &LogRecord,
    options: &ConsoleOptions,
    previous: Option<SystemTime>,
) -> io::Result<String> {
    let depth = options.colors.depth();
    let prefix = match record.level {
        Level::Error => "[E]",
        Level::Warn => "[W]",
        Level::Info => "[I]",
        Level::Debug => "[D]",
    };
    let margin = spaces(options.label.margin)?;
    let mut indent = 3_usize
        .checked_add(options.label.margin)
        .ok_or_else(invalid)?;
    let mut output = String::new();
    if options.timestamp == Timestamp::Utc {
        let stamp = timestamp(record.timestamp)?;
        indent = indent.checked_add(stamp.len()).ok_or_else(invalid)?;
        if depth == ColorDepth::None {
            output.push_str(&stamp);
        } else {
            let gray = if depth == ColorDepth::Ansi16 {
                "90"
            } else {
                "38;5;8"
            };
            output.push_str(&format!("\x1b[{gray}m{stamp}\x1b[0m"));
        }
    }
    let label = record.color(&record.name, depth);
    let label = if depth == ColorDepth::Ansi256 {
        label.replacen('m', ";1m", 1)
    } else {
        label
    };
    let width = record.name.encode_utf16().count();
    let padding = spaces(options.label.width.saturating_sub(width))?;
    match options.label.align {
        Alignment::Left => output.push_str(&format!("{prefix}{margin}{label}{padding}{margin}")),
        Alignment::Right => {
            output.push_str(&format!("{padding}{label}{margin}{prefix}{margin}"));
            indent = indent
                .checked_add(options.label.width)
                .and_then(|value| value.checked_add(options.label.margin))
                .ok_or_else(invalid)?;
        }
    }
    let continuation = format!("\n{}", spaces(indent)?);
    output.push_str(
        &record
            .format(options.max_length)
            .replace('\n', &continuation),
    );
    if options.show_diff {
        let elapsed = previous
            .and_then(|time| record.timestamp.duration_since(time).ok())
            .unwrap_or_default()
            .as_millis();
        output.push_str(&record.color(&format!(" +{elapsed}ms"), depth));
    }
    output.push('\n');
    Ok(output)
}
