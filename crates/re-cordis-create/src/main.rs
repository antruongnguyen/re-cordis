//! Standalone offline Cargo application generator.
use re_cordis_create::{CreateError, create};
use std::{ffi::OsString, path::PathBuf, process::ExitCode};

const HELP: &str = "Offline Cordis Cargo application generator
Usage: re-cordis-create NAME [--path DEST] --cordis-path CHECKOUT
  NAME           1–64 lowercase ASCII letters/digits/_/-, starting with a letter
  --path         Absent or empty destination (default: NAME); parent must exist
  --cordis-path  Required local re-cordis workspace checkout
  -h, --help     Show this help
One bundled basic template. No network, Git initialization, install, or launch.
";
fn arguments(
    args: impl IntoIterator<Item = OsString>,
) -> Result<Option<(String, PathBuf, PathBuf)>, CreateError> {
    let mut args = args.into_iter().peekable();
    let mut name = None;
    let mut path = None;
    let mut checkout = None;
    while let Some(arg) = args.next() {
        if arg == "--help" || arg == "-h" {
            return Ok(None);
        }
        if arg == "--path" || arg == "--cordis-path" {
            let target = if arg == "--path" {
                &mut path
            } else {
                &mut checkout
            };
            if target.is_some() {
                return Err(CreateError::Validation(format!(
                    "duplicate {}",
                    arg.to_string_lossy()
                )));
            }
            let value = args.next().ok_or_else(|| {
                CreateError::Validation(format!("missing value for {}", arg.to_string_lossy()))
            })?;
            if value.is_empty() || value.to_string_lossy().starts_with("--") {
                return Err(CreateError::Validation(format!(
                    "missing path value for {}",
                    arg.to_string_lossy()
                )));
            }
            *target = Some(PathBuf::from(value));
        } else if name.is_none() && !arg.to_string_lossy().starts_with('-') {
            name = Some(
                arg.into_string()
                    .map_err(|_| CreateError::Validation("project name must be ASCII".into()))?,
            );
        } else {
            return Err(CreateError::Validation(format!(
                "unexpected argument: {}",
                arg.to_string_lossy()
            )));
        }
    }
    let name = name.ok_or_else(|| CreateError::Validation("project NAME is required".into()))?;
    let path = path.unwrap_or_else(|| PathBuf::from(&name));
    let checkout = checkout
        .ok_or_else(|| CreateError::Validation("--cordis-path CHECKOUT is required".into()))?;
    Ok(Some((name, path, checkout)))
}
fn main() -> ExitCode {
    let result = arguments(std::env::args_os().skip(1)).and_then(|request| {
        let Some((name, path, checkout)) = request else {
            print!("{HELP}");
            return Ok(());
        };
        let destination = create(&name, &path, &checkout)?;
        println!(
            "Created {name} in {}\nOpen README.md for Cargo test/run instructions.",
            destination.display()
        );
        Ok(())
    });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            if error.exit_code() == 2 {
                eprintln!("Use --help for usage.");
            }
            ExitCode::from(error.exit_code())
        }
    }
}
