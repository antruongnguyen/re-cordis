use crate::{IncludeError, MAX_RETRIES, source::read};
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stage {
    Opened,
    Written,
    BeforeCheck,
    BeforeRename,
}
struct Temp(PathBuf);
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
fn write_all(writer: &mut impl Write, bytes: &[u8]) -> io::Result<()> {
    writer.write_all(bytes)?;
    writer.flush()
}
pub(crate) fn replace_with<F: FnMut(Stage, &Path) -> io::Result<()>>(
    path: &Path,
    bytes: &[u8],
    expected: Option<&[u8]>,
    hook: &mut F,
) -> Result<(), IncludeError> {
    let mut opened: Option<(File, Temp)> = None;
    for _ in 0..MAX_RETRIES {
        let mut name = path
            .file_name()
            .ok_or_else(|| IncludeError::new("filename missing"))?
            .to_os_string();
        name.push(format!(
            ".cordis-{}-{}.tmp",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let temp = path.with_file_name(name);
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        match options.open(&temp) {
            Ok(file) => {
                opened = Some((file, Temp(temp)));
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.into()),
        }
    }
    let (mut file, temp) =
        opened.ok_or_else(|| IncludeError::new("temporary filename retry limit exceeded"))?;
    hook(Stage::Opened, path)?;
    if expected.is_some() {
        file.set_permissions(std::fs::metadata(path)?.permissions())?;
    }
    write_all(&mut file, bytes)?;
    hook(Stage::Written, path)?;
    drop(file);
    hook(Stage::BeforeCheck, path)?;
    let latest = match read(path) {
        Ok(v) => Some(v),
        Err(e)
            if e.source()
                .and_then(|e| e.downcast_ref::<io::Error>())
                .is_some_and(|e| e.kind() == io::ErrorKind::NotFound) =>
        {
            None
        }
        Err(e) => return Err(e),
    };
    if latest.as_deref() != expected {
        return Err(IncludeError::new("source changed before replacement"));
    }
    hook(Stage::BeforeRename, path)?;
    if expected.is_none() {
        std::fs::hard_link(&temp.0, path)?;
    } else {
        std::fs::rename(&temp.0, path)?;
    }
    Ok(())
}
use std::error::Error;
#[cfg(test)]
mod tests {
    use super::*;
    struct Short {
        bytes: Vec<u8>,
        zero: bool,
    }
    impl Write for Short {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            if self.zero {
                return Ok(0);
            }
            let n = b.len().min(2);
            self.bytes.extend_from_slice(&b[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn write_all_handles_short_writes_and_refuses_zero_progress() {
        let mut s = Short {
            bytes: vec![],
            zero: false,
        };
        write_all(&mut s, b"abcdef").unwrap();
        assert_eq!(s.bytes, b"abcdef");
        s.zero = true;
        assert_eq!(
            write_all(&mut s, b"x").unwrap_err().kind(),
            io::ErrorKind::WriteZero
        );
    }
}
