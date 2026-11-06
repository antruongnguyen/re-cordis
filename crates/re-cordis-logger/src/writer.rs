use std::cell::RefCell;
use std::io::{self, Write};
use std::time::SystemTime;

use re_cordis_core::{AdmissionError, Context, ExporterOptions, ExporterRegistration, LogRecord};

use crate::{ConsoleOptions, render};

fn export<W: Write>(
    state: &mut (W, Option<SystemTime>),
    record: &LogRecord,
    options: &ConsoleOptions,
) -> io::Result<()> {
    let output = render(record, options, state.1)?;
    state.0.write_all(output.as_bytes())?;
    state.0.flush()?;
    state.1 = Some(record.timestamp);
    Ok(())
}

pub(crate) fn register<W: Write + 'static>(
    ctx: &Context,
    writer: W,
    options: ConsoleOptions,
    filtering: ExporterOptions,
) -> Result<ExporterRegistration, AdmissionError> {
    let state = RefCell::new((writer, None));
    ctx.add_exporter(filtering, move |record| {
        let mut state = state
            .try_borrow_mut()
            .map_err(|_| io::Error::other("console writer is already exporting (recursive log)"))?;
        export(&mut state, record, &options)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ColorMode, Timestamp};
    use std::time::{Duration, UNIX_EPOCH};

    struct Writer {
        bytes: Vec<u8>,
        fail_write: bool,
        fail_flush: bool,
    }
    impl Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_write {
                return Err(io::ErrorKind::BrokenPipe.into());
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            if self.fail_flush {
                Err(io::ErrorKind::Other.into())
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn delta_state_advances_only_after_successful_write_and_flush() {
        let runtime = re_cordis_core::Runtime::new();
        let mut record = runtime
            .context()
            .logger(Some("test"))
            .info("message")
            .unwrap();
        let options = ConsoleOptions {
            colors: ColorMode::Plain,
            timestamp: Timestamp::Off,
            show_diff: true,
            ..Default::default()
        };
        let mut state = (
            Writer {
                bytes: Vec::new(),
                fail_write: true,
                fail_flush: false,
            },
            None,
        );
        record.timestamp = UNIX_EPOCH;
        assert!(export(&mut state, &record, &options).is_err());
        assert_eq!(state.1, None);
        state.0.fail_write = false;
        state.0.fail_flush = true;
        assert!(export(&mut state, &record, &options).is_err());
        assert_eq!(state.1, None);
        state.0.fail_flush = false;
        state.0.bytes.clear();
        export(&mut state, &record, &options).unwrap();
        assert_eq!(state.1, Some(UNIX_EPOCH));
        record.timestamp = UNIX_EPOCH + Duration::from_millis(2);
        state.0.fail_write = true;
        assert!(export(&mut state, &record, &options).is_err());
        assert_eq!(state.1, Some(UNIX_EPOCH));
        state.0.fail_write = false;
        state.0.fail_flush = true;
        assert!(export(&mut state, &record, &options).is_err());
        assert_eq!(state.1, Some(UNIX_EPOCH));
        state.0.fail_flush = false;
        state.0.bytes.clear();
        record.timestamp = UNIX_EPOCH + Duration::from_millis(3);
        export(&mut state, &record, &options).unwrap();
        assert_eq!(
            String::from_utf8(state.0.bytes.clone()).unwrap(),
            "[I] test message +3ms\n"
        );
        record.timestamp = UNIX_EPOCH;
        export(&mut state, &record, &options).unwrap();
        assert_eq!(
            state.1,
            Some(UNIX_EPOCH),
            "successful backward record becomes next baseline"
        );
    }
}
