//! Fixed byte oracles for the selected console contract and source cases.
use std::time::{Duration, UNIX_EPOCH};

use re_cordis_core::{ColorDepth, Level, LogRecord, Runtime};
use re_cordis_logger::{Alignment, ColorMode, ConsoleOptions, LabelStyle, Timestamp, render};

fn record(name: &str, level: Level, message: &str, millis: u64) -> LogRecord {
    let runtime = Runtime::new();
    let mut record = runtime
        .context()
        .logger(Some(name))
        .log(level, message)
        .unwrap();
    record.timestamp = UNIX_EPOCH + Duration::from_millis(millis);
    record
}

fn plain() -> ConsoleOptions {
    ConsoleOptions {
        colors: ColorMode::Plain,
        timestamp: Timestamp::Off,
        ..Default::default()
    }
}

#[test]
fn severity_empty_values_and_final_newline() {
    for (level, prefix) in [
        (Level::Error, "[E]"),
        (Level::Warn, "[W]"),
        (Level::Info, "[I]"),
        (Level::Debug, "[D]"),
    ] {
        assert_eq!(
            render(&record("test", level, "message", 0), &plain(), None).unwrap(),
            format!("{prefix} test message\n")
        );
        assert_eq!(
            render(&record("", level, "", 0), &plain(), None).unwrap(),
            format!("{prefix}  \n")
        );
    }
}

#[test]
fn source_right_alignment_and_asymmetric_left_continuations() {
    let record = record("test", Level::Info, "message\nmessage", 0);
    let mut options = ConsoleOptions {
        show_diff: true,
        label: LabelStyle {
            width: 10,
            margin: 2,
            align: Alignment::Right,
        },
        ..plain()
    };
    assert_eq!(
        render(&record, &options, Some(UNIX_EPOCH)).unwrap(),
        "      test  [I]  message\n                 message +0ms\n"
    );
    options.label.align = Alignment::Left;
    assert_eq!(
        render(&record, &options, None).unwrap(),
        "[I]  test        message\n     message +0ms\n"
    );
    options.label = LabelStyle {
        width: 1,
        margin: 0,
        align: Alignment::Right,
    };
    assert_eq!(
        render(&record, &options, None).unwrap(),
        "test[I]message\n    message +0ms\n"
    );
}

#[test]
fn utf16_width_limits_crlf_and_unicode_are_core_formatting() {
    let mut options = plain();
    options.label.width = 4;
    options.max_length = 3;
    assert_eq!(
        render(
            &record("😀", Level::Info, "a😀z\r\nb\r\n", 0),
            &options,
            None
        )
        .unwrap(),
        "[I] 😀   a😀...\n    b\n    \n"
    );
    options.max_length = 0;
    assert_eq!(
        render(&record("longer", Level::Warn, "x\n", 0), &options, None).unwrap(),
        "[W] longer ...\n    \n"
    );
    options.max_length = usize::MAX;
    assert_eq!(
        render(&record("n", Level::Info, "a\r", 0), &options, None).unwrap(),
        "[I] n    a\r\n"
    );
    assert_eq!(ConsoleOptions::default().max_length, 10240);
    assert_eq!(
        render(
            &record("test", Level::Info, &"x".repeat(10241), 0),
            &plain(),
            None
        )
        .unwrap(),
        format!("[I] test {}...\n", "x".repeat(10240))
    );
}

#[test]
fn color_bytes_reuse_core_palette_and_only_ansi256_labels_are_bold() {
    let record = record("test", Level::Info, "message", 0);
    let mut options = plain();
    options.show_diff = true;
    options.colors = ColorMode::Ansi16;
    assert_eq!(
        render(&record, &options, None).unwrap(),
        format!(
            "[I] {} message{}\n",
            record.color("test", ColorDepth::Ansi16),
            record.color(" +0ms", ColorDepth::Ansi16)
        )
    );
    options.colors = ColorMode::Ansi256;
    let bold = record
        .color("test", ColorDepth::Ansi256)
        .replacen('m', ";1m", 1);
    assert_eq!(
        render(&record, &options, None).unwrap(),
        format!(
            "[I] {bold} message{}\n",
            record.color(" +0ms", ColorDepth::Ansi256)
        )
    );
    options.timestamp = Timestamp::Utc;
    assert!(
        render(&record, &options, None)
            .unwrap()
            .starts_with("\x1b[38;5;8m1970-01-01T00:00:00.000Z \x1b[0m")
    );
    options.colors = ColorMode::Ansi16;
    assert!(
        render(&record, &options, None)
            .unwrap()
            .starts_with("\x1b[90m1970-01-01T00:00:00.000Z \x1b[0m")
    );
    options.colors = ColorMode::Auto;
    assert!(!render(&record, &options, None).unwrap().contains('\x1b'));
}

#[test]
fn record_timestamp_is_utc_milliseconds_including_pre_epoch_and_leap_day() {
    let mut options = plain();
    options.timestamp = Timestamp::Utc;
    let mut record = record("test", Level::Info, "message\nx", 1709210096123);
    assert_eq!(
        render(&record, &options, None).unwrap(),
        "2024-02-29T12:34:56.123Z [I] test message\n                             x\n"
    );
    record.timestamp = UNIX_EPOCH - Duration::from_micros(1);
    assert!(
        render(&record, &options, None)
            .unwrap()
            .starts_with("1969-12-31T23:59:59.999Z ")
    );
}

#[test]
fn delta_is_exact_milliseconds_first_zero_and_backwards_zero() {
    let options = ConsoleOptions {
        show_diff: true,
        ..plain()
    };
    let record = record("test", Level::Info, "message", 2);
    for (previous, expected) in [
        (None, 0),
        (Some(UNIX_EPOCH), 2),
        (Some(UNIX_EPOCH + Duration::from_millis(3)), 0),
    ] {
        assert_eq!(
            render(&record, &options, previous).unwrap(),
            format!("[I] test message +{expected}ms\n")
        );
    }
    let mut later = record.clone();
    later.timestamp = UNIX_EPOCH + Duration::from_millis(120_000);
    assert!(
        render(&later, &options, Some(UNIX_EPOCH))
            .unwrap()
            .ends_with(" +120000ms\n")
    );
}

#[test]
fn color_policy_uses_captured_facts_and_explicit_modes_win() {
    assert_eq!(
        ColorMode::Auto.resolve(false, None, None, None),
        ColorDepth::None
    );
    assert_eq!(
        ColorMode::Auto.resolve(true, None, None, Some("dumb")),
        ColorDepth::None
    );
    assert_eq!(
        ColorMode::Auto.resolve(true, Some("1"), None, Some("xterm-256color")),
        ColorDepth::None
    );
    assert_eq!(
        ColorMode::Auto.resolve(true, Some(""), None, Some("xterm")),
        ColorDepth::Ansi16
    );
    assert_eq!(
        ColorMode::Auto.resolve(true, None, None, Some("xterm-256color")),
        ColorDepth::Ansi256
    );
    for (force, expected) in [
        ("0", ColorDepth::None),
        ("1", ColorDepth::Ansi16),
        ("2", ColorDepth::Ansi256),
        ("3", ColorDepth::Ansi256),
        ("", ColorDepth::Ansi16),
        ("anything", ColorDepth::Ansi16),
    ] {
        assert_eq!(
            ColorMode::Auto.resolve(false, Some("1"), Some(force), Some("dumb")),
            expected
        );
    }
    for (mode, expected) in [
        (ColorMode::Plain, ColorDepth::None),
        (ColorMode::Ansi16, ColorDepth::Ansi16),
        (ColorMode::Ansi256, ColorDepth::Ansi256),
    ] {
        assert_eq!(
            mode.resolve(false, Some("1"), Some("0"), Some("dumb")),
            expected
        );
    }
}

#[test]
fn pathological_layout_returns_an_error() {
    let record = record("test", Level::Info, "message", 0);
    for label in [
        LabelStyle {
            width: usize::MAX,
            margin: 1,
            align: Alignment::Right,
        },
        LabelStyle {
            width: 0,
            margin: usize::MAX,
            align: Alignment::Left,
        },
    ] {
        assert_eq!(
            render(&record, &ConsoleOptions { label, ..plain() }, None)
                .unwrap_err()
                .kind(),
            std::io::ErrorKind::InvalidInput
        );
    }
}

#[test]
fn rust_object_and_custom_display_replace_js_inspection_and_placeholders() {
    struct Custom;
    impl std::fmt::Display for Custom {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("custom%x")
        }
    }
    let runtime = Runtime::new();
    let logger = runtime.context().logger(Some("test"));
    let object = logger
        .info(format!(
            "{:?}",
            std::collections::BTreeMap::from([("foo", "bar")])
        ))
        .unwrap();
    assert_eq!(
        render(&object, &plain(), None).unwrap(),
        "[I] test {\"foo\": \"bar\"}\n"
    );
    assert_eq!(
        render(&logger.info(Custom).unwrap(), &plain(), None).unwrap(),
        "[I] test custom%x\n"
    );
    assert_eq!(
        render(&logger.info("%x%%x").unwrap(), &plain(), None).unwrap(),
        "[I] test %x%%x\n"
    );
}
