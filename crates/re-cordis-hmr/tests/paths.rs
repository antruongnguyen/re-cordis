#![allow(missing_docs)]
use re_cordis_core::{BoxError, Runtime};
use re_cordis_hmr::{Change, ChangeKind, FileWatcher, SyntheticError, WatchError, WatchOptions};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
static NEXT: AtomicU64 = AtomicU64::new(0);
fn dir() -> PathBuf {
    let p = std::fs::canonicalize(std::env::temp_dir())
        .unwrap()
        .join(format!(
            "cordis-hmr-path-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir(&p).unwrap();
    p
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn normalized_missing_parent_paths_keep_one_sticky_native_identity() -> Result<(), BoxError> {
    let root = dir();
    let runtime = Runtime::new();
    runtime
        .run(async {
            let (w, _) = FileWatcher::synthetic(&runtime.context(), WatchOptions::new(&root))?;
            let t = w.watch_source("./absent/a/../b/config.json")?;
            assert_eq!(t.path(), root.join("absent/b/config.json"));
            let u = w.watch_source("absent/b/config.json")?;
            assert_eq!(t.path(), u.path());
            assert_ne!(t.id(), u.id());
            assert_eq!(w.stats().paths, 1);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    std::fs::remove_dir(root)?;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn directories_unbounded_paths_and_relative_synthetic_input_are_refused()
-> Result<(), BoxError> {
    let root = dir();
    let runtime = Runtime::new();
    runtime
        .run(async {
            let (w, input) = FileWatcher::synthetic(&runtime.context(), WatchOptions::new(&root))?;
            assert!(matches!(
                w.watch_source("."),
                Err(WatchError::InvalidPath { .. })
            ));
            assert!(matches!(
                w.watch_source("x".repeat(5000)),
                Err(WatchError::InvalidPath { .. })
            ));
            assert_eq!(
                input.emit(Change {
                    path: "relative.rs".into(),
                    kind: ChangeKind::Modified
                }),
                Err(SyntheticError::InvalidPath)
            );
            assert_eq!(w.stats().watches, 0);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    std::fs::remove_dir(root)?;
    Ok(())
}

#[cfg(unix)]
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn base_alias_is_fixed_once_but_relative_parent_and_file_symlinks_are_rejected()
-> Result<(), BoxError> {
    let root = dir();
    let real = root.join("real");
    std::fs::create_dir(&real)?;
    std::fs::write(real.join("config.json"), b"[]")?;
    std::os::unix::fs::symlink(&real, root.join("base-alias"))?;
    std::os::unix::fs::symlink(&real, real.join("relative-link"))?;
    std::os::unix::fs::symlink(real.join("config.json"), real.join("file-link.json"))?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let (w, _) = FileWatcher::synthetic(
                &runtime.context(),
                WatchOptions::new(root.join("base-alias")),
            )?;
            let t = w.watch_source("config.json")?;
            assert_eq!(t.path(), real.join("config.json"));
            assert!(matches!(
                w.watch_source("relative-link/config.json"),
                Err(WatchError::InvalidPath { .. })
            ));
            assert!(matches!(
                w.watch_source("file-link.json"),
                Err(WatchError::InvalidPath { .. })
            ));
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[cfg(unix)]
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn non_utf8_native_file_names_do_not_require_url_or_string_conversion() -> Result<(), BoxError>
{
    use std::os::unix::ffi::OsStringExt;
    let root = dir();
    let runtime = Runtime::new();
    runtime
        .run(async {
            let (w, input) = FileWatcher::synthetic(&runtime.context(), WatchOptions::new(&root))?;
            let t = w.watch_source(std::ffi::OsString::from_vec(vec![
                b'f', 0xff, b'.', b'r', b's',
            ]))?;
            input.emit(Change {
                path: t.path().into(),
                kind: ChangeKind::Modified,
            })?;
            for _ in 0..4 {
                tokio::task::yield_now().await;
            }
            tokio::time::advance(Duration::from_millis(50)).await;
            for _ in 0..8 {
                tokio::task::yield_now().await;
            }
            assert!(!w.take_events().is_empty());
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    std::fs::remove_dir(root)?;
    Ok(())
}

#[test]
fn unavailable_time_driver_retires_constructor_and_starts_no_producer() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread()
        .build()?
        .block_on(async {
            let runtime = Runtime::new();
            runtime
                .run(async {
                    let root = runtime.context();
                    let baseline = root.get_effects()?.len();
                    assert!(matches!(
                        FileWatcher::new(&root, WatchOptions::new(std::env::temp_dir())),
                        Err(WatchError::Timer(_))
                    ));
                    for _ in 0..10 {
                        tokio::task::yield_now().await;
                    }
                    assert_eq!(root.get_effects()?.len(), baseline);
                    Ok::<_, BoxError>(())
                })
                .await??;
            Ok::<_, BoxError>(())
        })
}
