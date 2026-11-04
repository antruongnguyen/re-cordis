//! Black-box command and local filesystem contract.
use re_cordis_create::create;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "cordis-create-test-{}-{}",
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
fn checkout() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}
fn cli(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_re-cordis-create"))
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap()
}
fn names(path: &Path) -> Vec<String> {
    let mut entries: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();
    entries
}
#[test]
fn complete_file_set_and_repeat_refusal() {
    let fixture = Fixture::new();
    let dest = fixture.0.join("app");
    assert_eq!(create("my-app", &dest, &checkout()).unwrap(), dest);
    assert_eq!(
        names(&dest),
        [".gitignore", "Cargo.toml", "README.md", "src"]
    );
    assert_eq!(names(&dest.join("src")), ["app.rs", "main.rs"]);
    let manifest = fs::read_to_string(dest.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("name = \"my-app\""));
    assert!(manifest.contains("[workspace]"));
    assert!(!manifest.contains("version = \"0.0.0\", path"));
    let files = fs::read_to_string(dest.join("src/app.rs")).unwrap();
    assert!(create("my-app", &dest, &checkout()).is_err());
    assert_eq!(fs::read_to_string(dest.join("src/app.rs")).unwrap(), files);
    assert_eq!(names(&fixture.0), ["app"]);
}
#[test]
fn invalid_names_leave_no_destination() {
    let fixture = Fixture::new();
    for name in [
        "",
        ".",
        "..",
        "../escape",
        "two words",
        "ápp",
        "2app",
        "MyApp",
        "app/name",
        "app\\name",
        "fn",
        "gen",
        "test",
        "build",
        "con",
        "nul",
        "com1",
        "lpt9",
    ] {
        let dest = fixture.0.join("app");
        assert_eq!(
            create(name, &dest, &checkout()).unwrap_err().exit_code(),
            2,
            "{name}"
        );
        assert!(!dest.exists());
    }
    assert!(create(&"a".repeat(65), &fixture.0.join("app"), &checkout()).is_err());
    assert!(names(&fixture.0).is_empty());
}
#[test]
fn empty_directory_and_native_unicode_spaces_destination() {
    let fixture = Fixture::new();
    let dest = fixture.0.join("Ứng dụng space");
    fs::create_dir(&dest).unwrap();
    create("app_1", &dest, &checkout()).unwrap();
    assert!(dest.join("Cargo.toml").is_file());
    assert_eq!(names(&fixture.0), ["Ứng dụng space"]);
}
#[test]
fn destination_file_nonempty_and_missing_parent_are_refused() {
    let fixture = Fixture::new();
    let file = fixture.0.join("file");
    fs::write(&file, "keep").unwrap();
    assert_eq!(
        create("app", &file, &checkout()).unwrap_err().exit_code(),
        2
    );
    let directory = fixture.0.join("directory");
    fs::create_dir(&directory).unwrap();
    fs::write(directory.join("keep"), "untouched").unwrap();
    assert_eq!(
        create("app", &directory, &checkout())
            .unwrap_err()
            .exit_code(),
        2
    );
    assert_eq!(
        create("app", &fixture.0.join("missing/app"), &checkout())
            .unwrap_err()
            .exit_code(),
        2
    );
    assert_eq!(fs::read_to_string(file).unwrap(), "keep");
    assert_eq!(
        fs::read_to_string(directory.join("keep")).unwrap(),
        "untouched"
    );
    assert_eq!(names(&fixture.0), ["directory", "file"]);
}
#[test]
fn checkout_identity_is_required() {
    let fixture = Fixture::new();
    assert_eq!(
        create("app", &fixture.0.join("app"), &fixture.0)
            .unwrap_err()
            .exit_code(),
        2
    );
    assert_eq!(
        create("app", &fixture.0.join("app"), &fixture.0.join("absent"))
            .unwrap_err()
            .exit_code(),
        2
    );
    assert!(!fixture.0.join("app").exists());
}
#[test]
fn help_and_cli_validation_are_noninteractive() {
    let fixture = Fixture::new();
    let help = cli(&fixture.0, &["--help"]);
    assert_eq!(help.status.code(), Some(0));
    assert!(
        String::from_utf8(help.stdout)
            .unwrap()
            .contains("--cordis-path")
    );
    for args in [
        &[][..],
        &["app"][..],
        &["app", "--unknown"][..],
        &["app", "--path"][..],
        &["app", "--git"][..],
        &["app", "--install"][..],
        &["app", "--run"][..],
        &["app", "other"][..],
    ] {
        let output = cli(&fixture.0, args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(String::from_utf8(output.stderr).unwrap().contains("error:"));
    }
    assert!(names(&fixture.0).is_empty());
}
#[test]
fn cli_success_uses_argument_arrays_and_explicit_checkout() {
    let fixture = Fixture::new();
    let result = Command::new(env!("CARGO_BIN_EXE_re-cordis-create"))
        .current_dir(&fixture.0)
        .arg("app")
        .arg("--path")
        .arg("native space λ")
        .arg("--cordis-path")
        .arg(checkout())
        .output()
        .unwrap();
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8(result.stdout)
            .unwrap()
            .contains("Created")
    );
    assert!(fixture.0.join("native space λ/Cargo.toml").is_file());
    assert!(!fixture.0.join("native space λ/.git").exists());
    assert!(!fixture.0.join("native space λ/target").exists());
}
#[cfg(unix)]
#[test]
fn destination_symlinks_are_refused_including_dangling() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    for name in ["link", "dangling"] {
        let dest = fixture.0.join(name);
        symlink(
            if name == "link" {
                checkout()
            } else {
                fixture.0.join("absent")
            },
            &dest,
        )
        .unwrap();
        assert_eq!(
            create("app", &dest, &checkout()).unwrap_err().exit_code(),
            2
        );
        assert!(fs::symlink_metadata(dest).unwrap().file_type().is_symlink());
    }
    assert_eq!(names(&fixture.0), ["dangling", "link"]);
}

fn minimal_checkout(path: &Path) {
    fs::create_dir(path).unwrap();
    fs::write(path.join("Cargo.toml"), "[workspace]\n").unwrap();
    for package in ["core", "logger", "loader"] {
        let dir = path.join(format!("crates/re-cordis-{package}"));
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(dir.join("Cargo.toml"), format!("[package]\nname = \"re-cordis-{package}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n")).unwrap();
        fs::write(dir.join("src/lib.rs"), "").unwrap();
    }
}
#[test]
fn checkout_file_and_wrong_package_identity_are_validation_errors() {
    let fixture = Fixture::new();
    let file = fixture.0.join("file");
    fs::write(&file, "not a checkout").unwrap();
    assert_eq!(
        create("app", &fixture.0.join("app"), &file)
            .unwrap_err()
            .exit_code(),
        2
    );
    let root = fixture.0.join("checkout");
    minimal_checkout(&root);
    fs::write(
        root.join("crates/re-cordis-core/Cargo.toml"),
        "[package]\nname = \"unrelated\"\n",
    )
    .unwrap();
    assert_eq!(
        create("app", &fixture.0.join("app"), &root)
            .unwrap_err()
            .exit_code(),
        2
    );
    assert!(!fixture.0.join("app").exists());
}
#[test]
fn trailing_dot_destination_is_refused_before_creating_anything() {
    let fixture = Fixture::new();
    let dest = fixture.0.join("child").join(".");
    assert_eq!(
        create("app", &dest, &checkout()).unwrap_err().exit_code(),
        2
    );
    assert!(names(&fixture.0).is_empty());
}
#[test]
fn canonical_checkout_paths_are_not_recursively_expanded() {
    let fixture = Fixture::new();
    let root = fixture.0.join("checkout λ space {{loader}}");
    minimal_checkout(&root);
    let dest = fixture.0.join("app");
    create("app", &dest, &root).unwrap();
    let manifest = fs::read_to_string(dest.join("Cargo.toml")).unwrap();
    assert!(manifest.contains(&format!(
        "re-cordis-core = {{ path = \"{}/crates/re-cordis-core\" }}",
        root.display()
    )));
    let output = Command::new(env!("CARGO"))
        .arg("metadata")
        .arg("--manifest-path")
        .arg(dest.join("Cargo.toml"))
        .args(["--no-deps", "--offline", "--format-version", "1"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
#[cfg(unix)]
#[test]
fn quoted_control_and_backslash_checkout_paths_are_valid_cargo_toml() {
    let fixture = Fixture::new();
    let root = fixture.0.join("quote\" slash\\ newline\n tab\t λ");
    minimal_checkout(&root);
    let dest = fixture.0.join("app");
    create("app", &dest, &root).unwrap();
    let output = Command::new(env!("CARGO"))
        .arg("metadata")
        .arg("--manifest-path")
        .arg(dest.join("Cargo.toml"))
        .args(["--no-deps", "--offline", "--format-version", "1"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
#[cfg(unix)]
#[test]
fn native_nonunicode_destination_obeys_host_filesystem_and_cleans_failure() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    let fixture = Fixture::new();
    let dest = fixture.0.join(OsString::from_vec(b"app-\xff".to_vec()));
    #[cfg(target_os = "macos")]
    {
        let error = create("app", &dest, &checkout()).unwrap_err();
        assert_eq!(error.exit_code(), 1);
        assert_eq!(
            std::error::Error::source(&error)
                .unwrap()
                .downcast_ref::<std::io::Error>()
                .unwrap()
                .raw_os_error(),
            Some(92)
        );
        assert!(names(&fixture.0).is_empty());
    }
    #[cfg(not(target_os = "macos"))]
    {
        create("app", &dest, &checkout()).unwrap();
        assert!(dest.join("Cargo.toml").is_file());
    }
}
