use crate::WatchError;
use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};
pub(crate) const MAX_PATH_BYTES: usize = 4096;
pub(crate) fn absolute(base: &Path, path: &Path) -> PathBuf {
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    };
    let mut result = PathBuf::new();
    for c in joined.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                result.pop();
            }
            other => result.push(other.as_os_str()),
        }
    }
    result
}
pub(crate) fn symlink_component(path: &Path) -> io::Result<bool> {
    let mut prefix = PathBuf::new();
    for c in path.components() {
        prefix.push(c.as_os_str());
        match fs::symlink_metadata(&prefix) {
            Ok(m) if m.file_type().is_symlink() => return Ok(true),
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(false),
            Err(e) => return Err(e),
        }
    }
    Ok(false)
}
pub(crate) fn resolve(base: &Path, path: &Path) -> Result<PathBuf, WatchError> {
    let full = absolute(base, path);
    if full.as_os_str().len() > MAX_PATH_BYTES || full.file_name().is_none() {
        return Err(WatchError::InvalidPath {
            path: full,
            reason: "a bounded file path is required",
        });
    }
    if symlink_component(&full).map_err(|source| WatchError::Io {
        path: full.clone(),
        source,
    })? {
        return Err(WatchError::InvalidPath {
            path: full,
            reason: "symlink components are unsupported",
        });
    }
    if fs::symlink_metadata(&full).is_ok_and(|m| m.is_dir()) {
        return Err(WatchError::InvalidPath {
            path: full,
            reason: "watch a file, not a directory",
        });
    }
    let mut ancestor = full.parent().unwrap().to_path_buf();
    let mut suffix = vec![full.file_name().unwrap().to_os_string()];
    let canonical = loop {
        match fs::canonicalize(&ancestor) {
            Ok(p) => break p,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let Some(name) = ancestor.file_name() else {
                    return Err(WatchError::Io {
                        path: full,
                        source: e,
                    });
                };
                suffix.push(name.to_os_string());
                ancestor.pop();
            }
            Err(source) => return Err(WatchError::Io { path: full, source }),
        }
    };
    let mut result = canonical;
    for c in suffix.into_iter().rev() {
        result.push(c);
    }
    if result.as_os_str().len() > MAX_PATH_BYTES {
        return Err(WatchError::InvalidPath {
            path: result,
            reason: "path exceeds 4096 native bytes",
        });
    }
    Ok(result)
}
