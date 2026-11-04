//! Offline scaffolding for one bundled local Cordis application.
//!
//! The command accepts an absent or empty directory and an explicit checkout.
//! It writes five fixed files into an owned sibling directory, checks the target
//! again, then commits with one filesystem rename. No processes or network calls
//! are made. This is ordinary-filesystem atomic visibility, not crash durability
//! or protection against hostile ancestor swaps/concurrent empty-directory races.
use std::{
    error::Error,
    fmt, fs, io,
    path::{Component, Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

/// Validation, filesystem, or failed owned-staging cleanup.
#[derive(Debug)]
pub enum CreateError {
    /// The request cannot be safely scaffolded.
    Validation(String),
    /// A filesystem operation failed, retaining its path and original cause.
    Io {
        /// The failed operation.
        operation: &'static str,
        /// The affected path.
        path: PathBuf,
        /// Original filesystem error.
        source: io::Error,
    },
    /// The original failure is preserved and owned staging could not be removed.
    Cleanup {
        /// Original scaffold failure.
        original: Box<CreateError>,
        /// Owned staging directory requiring manual cleanup.
        staging: PathBuf,
        /// Original cleanup error.
        source: io::Error,
    },
}
impl CreateError {
    /// Exit status: 2 for validation, 1 for filesystem/cleanup failure.
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Validation(_) => 2,
            Self::Io { .. } | Self::Cleanup { .. } => 1,
        }
    }
}
impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) => f.write_str(message),
            Self::Io {
                operation,
                path,
                source,
            } => write!(f, "{operation} {}: {source}", path.display()),
            Self::Cleanup {
                original,
                staging,
                source,
            } => write!(
                f,
                "{original}; could not remove owned staging {}: {source}",
                staging.display()
            ),
        }
    }
}
impl Error for CreateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Validation(_) => None,
            Self::Io { source, .. } => Some(source),
            Self::Cleanup { original, .. } => Some(original),
        }
    }
}
fn io_error(operation: &'static str, path: &Path, source: io::Error) -> CreateError {
    CreateError::Io {
        operation,
        path: path.to_path_buf(),
        source,
    }
}
fn invalid(message: impl Into<String>) -> CreateError {
    CreateError::Validation(message.into())
}

/// Generate a complete application, returning its canonical-parent absolute path.
///
/// Names are 1–64 ASCII characters: a lowercase letter followed by lowercase
/// letters, digits, hyphens or underscores. Rust/Cargo/Windows reserved names
/// are refused. The existing parent is required; its symlinks are resolved.
/// A destination file, symlink (even dangling), or nonempty directory is refused.
/// The checkout must have the current workspace layout and literal package-name
/// declarations for core, logger and loader; it is not a source authenticity audit.
/// Canonical dependency paths must be Unicode for Cargo TOML representation.
///
/// Failed operations remove only owned staging. Cleanup failure retains both
/// errors and names that staging path. Rename replacement of empty directories
/// is platform-dependent; unsupported replacement returns an I/O error.
pub fn create(name: &str, destination: &Path, cordis_path: &Path) -> Result<PathBuf, CreateError> {
    validate_name(name)?;
    // Require an ordinary final path component: never scaffold into "." or "..".
    if !matches!(
        destination.components().next_back(),
        Some(Component::Normal(_))
    ) || destination.as_os_str().is_empty()
        || destination.file_name().is_none()
    {
        return Err(invalid(
            "destination must name a directory beneath an existing parent",
        ));
    }
    // Path::components normalizes trailing ".", so refuse that spelling as well.
    if destination
        .as_os_str()
        .as_encoded_bytes()
        .rsplit(|b| *b == b'/' || (cfg!(windows) && *b == b'\\'))
        .find(|part| !part.is_empty())
        .is_some_and(|part| part == b"." || part == b"..")
    {
        return Err(invalid("destination must not end in . or .."));
    }
    let parent = destination
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let parent = canonical_input(parent, "destination parent")?;
    if !parent.is_dir() {
        return Err(invalid("destination parent must be a directory"));
    }
    let destination = parent.join(destination.file_name().expect("checked final component"));
    check_destination(&destination)?;
    let checkout = canonical_input(cordis_path, "Cordis checkout")?;
    if !checkout.is_dir() {
        return Err(invalid("Cordis checkout must be a directory"));
    }
    let files = render(name, &checkout)?;
    stage_project(&destination, |stage| {
        write_files(stage, &files)?;
        check_destination(&destination)
    })?;
    Ok(destination)
}
fn validate_name(name: &str) -> Result<(), CreateError> {
    const RESERVED: &[&str] = &[
        "as",
        "async",
        "await",
        "break",
        "const",
        "continue",
        "crate",
        "dyn",
        "else",
        "enum",
        "extern",
        "false",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "match",
        "mod",
        "move",
        "mut",
        "pub",
        "ref",
        "return",
        "self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
        "abstract",
        "become",
        "box",
        "do",
        "final",
        "gen",
        "macro",
        "override",
        "priv",
        "try",
        "typeof",
        "unsized",
        "virtual",
        "yield",
        "test",
        "build",
        "deps",
        "examples",
        "incremental",
        "con",
        "prn",
        "aux",
        "nul",
        "com1",
        "com2",
        "com3",
        "com4",
        "com5",
        "com6",
        "com7",
        "com8",
        "com9",
        "lpt1",
        "lpt2",
        "lpt3",
        "lpt4",
        "lpt5",
        "lpt6",
        "lpt7",
        "lpt8",
        "lpt9",
        "re-cordis-core",
        "re-cordis-logger",
        "re-cordis-loader",
    ];
    if !(1..=64).contains(&name.len())
        || !name.as_bytes()[0].is_ascii_lowercase()
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
        || RESERVED.contains(&name)
    {
        return Err(invalid(
            "project name must be 1–64 lowercase ASCII letters/digits/_/-, start with a letter, and not be reserved",
        ));
    }
    Ok(())
}
fn canonical_input(path: &Path, description: &str) -> Result<PathBuf, CreateError> {
    fs::canonicalize(path).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            invalid(format!("{description} does not exist: {}", path.display()))
        } else {
            io_error("canonicalize", path, e)
        }
    })
}
fn check_destination(path: &Path) -> Result<(), CreateError> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(io_error("inspect destination", path, e)),
        Ok(meta) if meta.file_type().is_symlink() || !meta.is_dir() => Err(invalid(format!(
            "destination is a file or symlink: {}",
            path.display()
        ))),
        Ok(_) => {
            let mut entries =
                fs::read_dir(path).map_err(|e| io_error("read destination", path, e))?;
            match entries.next() {
                None => Ok(()),
                Some(Ok(_)) => Err(invalid(format!(
                    "destination is not empty: {}",
                    path.display()
                ))),
                Some(Err(e)) => Err(io_error("read destination", path, e)),
            }
        }
    }
}
fn read_manifest(path: &Path) -> Result<String, CreateError> {
    fs::read_to_string(path).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            invalid(format!("checkout is missing {}", path.display()))
        } else {
            io_error("read checkout manifest", path, e)
        }
    })
}
fn render(name: &str, checkout: &Path) -> Result<Vec<(&'static str, String)>, CreateError> {
    let root = read_manifest(&checkout.join("Cargo.toml"))?;
    if !root.lines().any(|line| line.trim() == "[workspace]") {
        return Err(invalid("Cordis checkout must be a workspace root"));
    }
    let mut paths = Vec::new();
    for package in ["core", "logger", "loader"] {
        let expected = format!("re-cordis-{package}");
        let dir = checkout.join("crates").join(&expected);
        let content = read_manifest(&dir.join("Cargo.toml"))?;
        // Deliberately narrow: the bundled app targets this checkout's layout.
        let mut in_package = false;
        let identity = content.lines().any(|line| {
            let line = line.trim();
            if line.starts_with('[') {
                in_package = line == "[package]";
            }
            in_package && line == format!("name = \"{expected}\"")
        });
        if !identity || !dir.join("src/lib.rs").is_file() {
            return Err(invalid(format!(
                "checkout package identity/layout mismatch: {expected}"
            )));
        }
        let dir = canonical_input(&dir, "dependency directory")?;
        let path = dir
            .to_str()
            .ok_or_else(|| invalid("canonical dependency paths must be Unicode for Cargo TOML"))?;
        paths.push((package, toml_string(path)));
    }
    // Expand fixed template tokens once; inserted filesystem paths are data.
    let template = include_str!("../templates/basic/Cargo.toml.template");
    let mut parts = template.split("{{");
    let mut manifest = parts.next().expect("bundled template").to_owned();
    for part in parts {
        let (token, suffix) = part.split_once("}}").expect("bundled template token");
        let value = if token == "name" {
            name
        } else {
            &paths
                .iter()
                .find(|(key, _)| *key == token)
                .expect("bundled dependency token")
                .1
        };
        manifest.push_str(value);
        manifest.push_str(suffix);
    }
    Ok(vec![
        ("Cargo.toml", manifest),
        (
            "src/main.rs",
            include_str!("../templates/basic/main.rs").into(),
        ),
        (
            "src/app.rs",
            include_str!("../templates/basic/app.rs").into(),
        ),
        (
            "README.md",
            include_str!("../templates/basic/README.md").replace("{{name}}", name),
        ),
        (
            ".gitignore",
            include_str!("../templates/basic/gitignore").into(),
        ),
    ])
}
fn toml_string(value: &str) -> String {
    let mut result = String::from("\"");
    for c in value.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => result.push_str(&format!("\\u{:04X}", c as u32)),
            c => result.push(c),
        }
    }
    result.push('"');
    result
}
static NEXT: AtomicU64 = AtomicU64::new(0);
fn stage_project(
    destination: &Path,
    write: impl FnOnce(&Path) -> Result<(), CreateError>,
) -> Result<(), CreateError> {
    let parent = destination.parent().expect("canonical parent checked");
    // Opposite leading-dot status stays distinct even on case-insensitive filesystems.
    let prefix = if destination
        .file_name()
        .expect("final component checked")
        .as_encoded_bytes()
        .starts_with(b".")
    {
        "_re-cordis-create"
    } else {
        ".re-cordis-create"
    };
    let stage = (0..128)
        .find_map(|_| {
            let path = parent.join(format!(
                "{prefix}-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => Some(Ok(path)),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => None,
                Err(e) => Some(Err(io_error("create staging", &path, e))),
            }
        })
        .unwrap_or_else(|| Err(invalid("unable to allocate staging after 128 collisions")))?;
    let result = write(&stage).and_then(|()| {
        fs::rename(&stage, destination).map_err(|e| io_error("commit staging", destination, e))
    });
    if let Err(original) = result {
        if let Err(source) = fs::remove_dir_all(&stage) {
            return Err(CreateError::Cleanup {
                original: Box::new(original),
                staging: stage,
                source,
            });
        }
        return Err(original);
    }
    Ok(())
}
fn write_files(stage: &Path, files: &[(&str, String)]) -> Result<(), CreateError> {
    let src = stage.join("src");
    fs::create_dir(&src).map_err(|e| io_error("create source directory", &src, e))?;
    for (name, content) in files {
        let path = stage.join(name);
        fs::write(&path, content).map_err(|e| io_error("write template", &path, e))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "cordis-create-unit-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(fs::canonicalize(path).unwrap())
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
    fn assert_no_stage(parent: &Path) {
        assert!(fs::read_dir(parent).unwrap().all(|entry| {
            let name = entry.unwrap().file_name();
            let name = name.to_string_lossy();
            !name.starts_with(".re-cordis-create-") && !name.starts_with("_re-cordis-create-")
        }));
    }
    #[test]
    fn real_source_write_failure_cleans_staging_and_keeps_final_absent() {
        let fixture = Fixture::new();
        let dest = fixture.0.join("app");
        let error = stage_project(&dest, |stage| {
            fs::create_dir(stage.join("Cargo.toml")).unwrap();
            write_files(stage, &[("Cargo.toml", "partial".into())])
        })
        .unwrap_err();
        assert_eq!(error.exit_code(), 1);
        assert!(
            error
                .source()
                .unwrap()
                .downcast_ref::<io::Error>()
                .is_some()
        );
        assert!(!dest.exists());
        assert_no_stage(&fixture.0);
    }
    #[test]
    fn real_rename_failure_cleans_stage_and_preserves_concurrent_winner() {
        let fixture = Fixture::new();
        let dest = fixture.0.join("app");
        let error = stage_project(&dest, |stage| {
            write_files(stage, &[("Cargo.toml", "complete".into())])?;
            fs::write(&dest, "concurrent winner").unwrap();
            Ok(())
        })
        .unwrap_err();
        assert!(matches!(
            error,
            CreateError::Io {
                operation: "commit staging",
                ..
            }
        ));
        assert_eq!(fs::read_to_string(dest).unwrap(), "concurrent winner");
        assert_no_stage(&fixture.0);
    }
    #[test]
    fn cleanup_failure_names_owned_path_and_retains_both_causes() {
        let fixture = Fixture::new();
        let dest = fixture.0.join("app");
        let error = stage_project(&dest, |stage| {
            // Replace only our own staging with a file: remove_dir_all must fail.
            fs::remove_dir(stage).unwrap();
            fs::write(stage, "owned staging").unwrap();
            let source = fs::write(stage.join("child"), "fail").unwrap_err();
            Err(io_error("write template", &stage.join("child"), source))
        })
        .unwrap_err();
        assert_eq!(error.exit_code(), 1);
        let CreateError::Cleanup {
            original,
            staging,
            source,
        } = &error
        else {
            panic!("{error}")
        };
        assert!(
            original
                .source()
                .unwrap()
                .downcast_ref::<io::Error>()
                .is_some()
        );
        assert!(
            source.kind() == io::ErrorKind::NotADirectory || source.kind() == io::ErrorKind::Other
        );
        assert!(error.to_string().contains(&staging.display().to_string()));
        assert_eq!(fs::read_to_string(staging).unwrap(), "owned staging");
        fs::remove_file(staging).unwrap();
        assert!(!dest.exists());
    }
    #[test]
    fn staging_aliases_keep_final_absent_until_commit() {
        const PROBE: &str = "RE_CORDIS_CREATE_STAGE_BOUNDARY_PROBE";
        let Ok(spelling) = std::env::var(PROBE) else {
            for spelling in ["exact", "uppercase"] {
                let output = std::process::Command::new(std::env::current_exe().unwrap())
                    .args([
                        "--exact",
                        "tests::staging_aliases_keep_final_absent_until_commit",
                        "--nocapture",
                    ])
                    .env(PROBE, spelling)
                    .output()
                    .unwrap();
                assert!(
                    output.status.success(),
                    "{spelling}: {}{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr),
                );
            }
            return;
        };
        let parent =
            std::env::temp_dir().join(format!("cordis-create-boundary-{}", std::process::id(),));
        fs::create_dir(&parent).unwrap();
        let fixture = Fixture(fs::canonicalize(parent).unwrap());
        let basename = format!(".re-cordis-create-{}-0", std::process::id());
        let basename = if spelling == "uppercase" {
            basename.to_ascii_uppercase()
        } else {
            basename
        };
        let dest = fixture.0.join(&basename);
        stage_project(&dest, |stage| {
            assert!(
                !dest.exists(),
                "final directory exists before staged writes"
            );
            write_files(stage, &[("Cargo.toml", "complete".into())])?;
            assert!(
                !dest.exists(),
                "staged writes populated the final directory"
            );
            check_destination(&dest)
        })
        .unwrap();
        assert_eq!(
            fs::read_to_string(dest.join("Cargo.toml")).unwrap(),
            "complete"
        );
        assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 1);
    }
    #[test]
    fn toml_paths_escape_quotes_slashes_and_controls() {
        assert_eq!(
            toml_string("λ\\\"\n\r\t\0\u{7f}"),
            "\"λ\\\\\\\"\\n\\r\\t\\u0000\\u007F\""
        );
    }
}
