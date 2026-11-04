#![allow(missing_docs)]
use futures_util::FutureExt;
use re_cordis_core::{BoxError, Runtime};
use re_cordis_hmr::{ChangeKind, FileWatcher, WatchEvent, WatchFailureStage, WatchOptions};
use std::{
    cell::{Cell, RefCell},
    path::PathBuf,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        let p = std::fs::canonicalize(std::env::temp_dir())
            .unwrap()
            .join(format!(
                "cordis-hmr-fs-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn options(dir: &Dir) -> WatchOptions {
    let mut o = WatchOptions::new(&dir.0);
    o.poll_interval = Duration::from_millis(5);
    o.debounce = Duration::from_millis(2);
    o
}
async fn until(mut condition: impl FnMut() -> bool) -> Result<(), BoxError> {
    tokio::time::timeout(Duration::from_secs(3), async {
        while !condition() {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await?;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn immediate_write_and_duplicate_registration_receive_initial_rescan() -> Result<(), BoxError>
{
    let d = Dir::new();
    let path = d.path("config.json");
    std::fs::write(&path, b"old")?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let mut o = options(&d);
            o.poll_interval = Duration::from_secs(60);
            let w = FileWatcher::new(&runtime.context(), o)?;
            let latest = Rc::new(RefCell::new(String::new()));
            let output = latest.clone();
            let t = w.watch(&path, move |c| {
                *output.borrow_mut() = std::fs::read_to_string(c.path).unwrap();
                async { Ok(()) }.boxed_local()
            })?;
            std::fs::write(&path, b"new")?;
            until(|| *latest.borrow() == "new").await?;
            let calls = Rc::new(Cell::new(0));
            let output = calls.clone();
            let u = w.watch(&path, move |_| {
                output.set(output.get() + 1);
                async { Ok(()) }.boxed_local()
            })?;
            until(|| calls.get() > 0).await?;
            assert_eq!(t.path(), u.path());
            tokio::time::timeout(Duration::from_secs(2), w.dispose()?).await??;
            assert!(!w.stats().producer_running);
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn fingerprint_detects_same_size_rewrite_with_restored_modified_time() -> Result<(), BoxError>
{
    let d = Dir::new();
    let path = d.path("same.json");
    std::fs::write(&path, b"aaaa")?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let w = FileWatcher::new(&runtime.context(), options(&d))?;
            let latest = Rc::new(RefCell::new(String::new()));
            let output = latest.clone();
            let _t = w.watch(&path, move |c| {
                *output.borrow_mut() = std::fs::read_to_string(c.path).unwrap();
                async { Ok(()) }.boxed_local()
            })?;
            until(|| *latest.borrow() == "aaaa").await?;
            std::fs::write(&path, b"warm")?;
            until(|| *latest.borrow() == "warm").await?;
            let modified = std::fs::metadata(&path)?.modified()?;
            std::fs::write(&path, b"bbbb")?;
            std::fs::File::options()
                .write(true)
                .open(&path)?
                .set_times(std::fs::FileTimes::new().set_modified(modified))?;
            until(|| *latest.borrow() == "bbbb").await?;
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn atomic_rename_remove_and_parent_recreate_keep_logical_watch() -> Result<(), BoxError> {
    let d = Dir::new();
    let parent = d.path("parent");
    std::fs::create_dir(&parent)?;
    let path = parent.join("config.json");
    std::fs::write(&path, b"one")?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let w = FileWatcher::new(&runtime.context(), options(&d))?;
            let latest = Rc::new(RefCell::new(None));
            let output = latest.clone();
            let kinds = Rc::new(RefCell::new(vec![]));
            let k = kinds.clone();
            let t = w.watch(&path, move |c| {
                *output.borrow_mut() = std::fs::read_to_string(&c.path).ok();
                k.borrow_mut().push(c.kind);
                async { Ok(()) }.boxed_local()
            })?;
            until(|| latest.borrow().as_deref() == Some("one")).await?;
            let staged = parent.join("stage.json");
            std::fs::write(&staged, b"two")?;
            std::fs::rename(staged, &path)?;
            until(|| latest.borrow().as_deref() == Some("two")).await?;
            std::fs::remove_file(&path)?;
            std::fs::remove_dir(&parent)?;
            until(|| latest.borrow().is_none() && kinds.borrow().contains(&ChangeKind::Removed))
                .await?;
            assert!(kinds.borrow().contains(&ChangeKind::Removed));
            std::fs::create_dir(&parent)?;
            std::fs::write(&path, b"three")?;
            until(|| latest.borrow().as_deref() == Some("three")).await?;
            assert_eq!(t.path(), path);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn oversized_file_failure_preserves_original_io_cause_and_recovers() -> Result<(), BoxError> {
    use std::error::Error;
    let d = Dir::new();
    let path = d.path("bounded.json");
    std::fs::write(&path, b"ok")?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let mut o = options(&d);
            o.max_file_bytes = 16;
            let w = FileWatcher::new(&runtime.context(), o)?;
            let count = Rc::new(Cell::new(0));
            let output = count.clone();
            let _t = w.watch(&path, move |_| {
                output.set(output.get() + 1);
                async { Ok(()) }.boxed_local()
            })?;
            until(|| count.get() > 0).await?;
            w.take_events();
            let baseline = count.get();
            std::fs::write(&path, vec![b'x'; 17])?;
            let failures = Rc::new(RefCell::new(vec![]));
            let f = failures.clone();
            until(|| {
                f.borrow_mut()
                    .extend(w.take_events().into_iter().filter_map(|e| {
                        if let WatchEvent::Failure(x) = e {
                            Some(x)
                        } else {
                            None
                        }
                    }));
                !f.borrow().is_empty()
            })
            .await?;
            assert_eq!(failures.borrow()[0].stage, WatchFailureStage::Read);
            assert!(
                failures.borrow()[0]
                    .source()
                    .unwrap()
                    .is::<std::io::Error>()
            );
            std::fs::write(&path, b"recovered")?;
            until(|| count.get() > baseline).await?;
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[cfg(unix)]
#[tokio::test(flavor = "current_thread")]
async fn symlink_replacement_reports_failure_then_regular_file_recovers() -> Result<(), BoxError> {
    let d = Dir::new();
    let path = d.path("link.json");
    let target = d.path("elsewhere.json");
    std::fs::write(&path, b"ok")?;
    std::fs::write(&target, b"elsewhere")?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let w = FileWatcher::new(&runtime.context(), options(&d))?;
            let count = Rc::new(Cell::new(0));
            let output = count.clone();
            let _t = w.watch(&path, move |_| {
                output.set(output.get() + 1);
                async { Ok(()) }.boxed_local()
            })?;
            until(|| count.get() > 0).await?;
            w.take_events();
            let n = count.get();
            std::fs::remove_file(&path)?;
            std::os::unix::fs::symlink(&target, &path)?;
            until(|| {
                w.take_events()
                    .iter()
                    .any(|e| matches!(e, WatchEvent::Failure(_)))
            })
            .await?;
            std::fs::remove_file(&path)?;
            std::fs::write(&path, b"back")?;
            until(|| count.get() > n).await?;
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn one_hundred_real_producer_cycles_join_and_restore_parent_baseline() -> Result<(), BoxError>
{
    let d = Dir::new();
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let baseline = root.get_effects()?.len();
            for _ in 0..100 {
                let mut o = options(&d);
                o.poll_interval = Duration::from_secs(60);
                let w = FileWatcher::new(&root, o)?;
                let t = w.watch_source("missing.rs")?;
                drop(t);
                tokio::time::timeout(Duration::from_secs(2), w.dispose()?).await??;
                assert!(!w.stats().producer_running);
                assert_eq!(root.get_effects()?.len(), baseline);
            }
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}
