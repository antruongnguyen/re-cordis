#![allow(missing_docs)]
use futures_util::FutureExt;
use re_cordis_core::{AdmissionError, BoxError, LifecycleError, Runtime};
use re_cordis_hmr::{
    Change, ChangeKind, FileWatcher, SyntheticEvents, WatchError, WatchEvent, WatchOptions,
    WatchToken,
};
use std::{
    cell::{Cell, RefCell},
    error::Error,
    rc::Rc,
    time::Duration,
};

fn options() -> WatchOptions {
    WatchOptions::new(std::fs::canonicalize(std::env::temp_dir()).unwrap())
}
async fn pump() {
    for _ in 0..12 {
        tokio::task::yield_now().await;
    }
}
async fn fire() {
    pump().await;
    tokio::time::advance(Duration::from_millis(50)).await;
    pump().await;
}
fn emit(input: &SyntheticEvents, token: &WatchToken) {
    input
        .emit(Change {
            path: token.path().into(),
            kind: ChangeKind::Modified,
        })
        .unwrap();
}
fn counter(
    c: Rc<Cell<usize>>,
) -> impl FnMut(Change) -> re_cordis_core::LocalFuture<Result<(), BoxError>> {
    move |_| {
        c.set(c.get() + 1);
        async { Ok(()) }.boxed_local()
    }
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn synthetic_event_executes_admitted_local_callback_after_debounce() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let count = Rc::new(Cell::new(0));
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let t = w.watch("cordis-hmr-a.json", counter(count.clone()))?;
            emit(&input, &t);
            fire().await;
            assert_eq!(count.get(), 1);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn duplicate_burst_restarts_debounce_and_does_not_keep_history() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let count = Rc::new(Cell::new(0));
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let t = w.watch("burst.json", counter(count.clone()))?;
            emit(&input, &t);
            pump().await;
            tokio::time::advance(Duration::from_millis(40)).await;
            pump().await;
            assert_eq!(count.get(), 0);
            for _ in 0..100 {
                emit(&input, &t);
            }
            pump().await;
            tokio::time::advance(Duration::from_millis(49)).await;
            pump().await;
            assert_eq!(count.get(), 0);
            tokio::time::advance(Duration::from_millis(1)).await;
            pump().await;
            assert_eq!(count.get(), 1);
            assert_eq!(w.stats().pending, 0);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn exact_stop_keeps_sibling_and_suppresses_stale_generation() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let a = Rc::new(Cell::new(0));
            let b = Rc::new(Cell::new(0));
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let t = w.watch("shared.json", counter(a.clone()))?;
            let u = w.watch("shared.json", counter(b.clone()))?;
            emit(&input, &t);
            t.stop();
            fire().await;
            assert_eq!((a.get(), b.get()), (0, 1));
            emit(&input, &u);
            drop(u);
            let v = w.watch("shared.json", counter(a.clone()))?;
            fire().await;
            assert_eq!(a.get(), 0);
            emit(&input, &v);
            fire().await;
            assert_eq!(a.get(), 1);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn observed_ignore_can_be_overridden_by_explicit_registration() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let count = Rc::new(Cell::new(0));
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            assert!(
                w.observe("target/config.json", counter(count.clone()))?
                    .is_none()
            );
            assert!(w.observe_source("node_modules/source.rs")?.is_none());
            assert_eq!(w.stats().watches, 0);
            let t = w.watch("target/config.json", counter(count.clone()))?;
            emit(&input, &t);
            fire().await;
            assert_eq!(count.get(), 1);
            let _outside = w.watch(
                std::fs::canonicalize(std::env::temp_dir())
                    .unwrap()
                    .join("external.json"),
                counter(count),
            )?;
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn overflow_rescans_all_registered_paths_and_history_is_bounded() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let mut o = options();
            o.queue_capacity = 1;
            o.event_capacity = 1;
            let seen = Rc::new(RefCell::new(vec![]));
            let (w, input) = FileWatcher::synthetic(&runtime.context(), o)?;
            let mut tokens = vec![];
            for p in ["one.json", "two.json"] {
                let s = seen.clone();
                tokens.push(w.watch(p, move |c| {
                    s.borrow_mut().push(c);
                    async { Ok(()) }.boxed_local()
                })?);
            }
            for _ in 0..20 {
                emit(&input, &tokens[0]);
            }
            fire().await;
            assert_eq!(seen.borrow().len(), 2);
            assert!(seen.borrow().iter().all(|c| c.kind == ChangeKind::Rescan));
            assert!(w.stats().queue_overflows > 0);
            assert_eq!(w.stats().events, 1);
            assert!(w.stats().events_evicted > 0);
            assert_eq!(w.take_events().len(), 1);
            assert!(w.take_events().is_empty());
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn source_tokens_emit_restart_required_once_per_path_batch() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let t = w.watch_source("main.rs")?;
            let _u = w.watch_source("main.rs")?;
            emit(&input, &t);
            fire().await;
            let events = w.take_events();
            assert_eq!(
                events
                    .iter()
                    .filter(|e| matches!(e, WatchEvent::RestartRequired(_)))
                    .count(),
                1
            );
            assert!(w.capabilities().source_restart_required);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[derive(Debug)]
struct OriginalFailure;
impl std::fmt::Display for OriginalFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "original failure")
    }
}
impl Error for OriginalFailure {}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn original_callback_error_and_panics_leave_future_refresh_usable() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let n = Rc::new(Cell::new(0));
            let c = n.clone();
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let t = w.watch("recovery.json", move |_| {
                c.set(c.get() + 1);
                let step = c.get();
                if step == 2 {
                    panic!("synchronous callback");
                }
                async move {
                    if step == 1 {
                        return Err(Box::new(OriginalFailure) as BoxError);
                    }
                    if step == 3 {
                        panic!("asynchronous callback");
                    }
                    Ok(())
                }
                .boxed_local()
            })?;
            for _ in 0..4 {
                emit(&input, &t);
                fire().await;
            }
            assert_eq!(n.get(), 4);
            let events = w.take_events();
            let failures = events
                .iter()
                .filter_map(|e| {
                    if let WatchEvent::Failure(f) = e {
                        Some(f)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            assert_eq!(failures.len(), 3);
            assert!(failures[0].source().unwrap().is::<OriginalFailure>());
            assert_eq!(runtime.take_failures().len(), 1);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn running_callbacks_coalesce_followup_and_unrelated_callbacks_progress()
-> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let gate = Rc::new(tokio::sync::Notify::new());
            let g = gate.clone();
            let n = Rc::new(Cell::new(0));
            let c = n.clone();
            let other = Rc::new(Cell::new(0));
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let t = w.watch("slow.json", move |_| {
                c.set(c.get() + 1);
                let g = g.clone();
                async move {
                    g.notified().await;
                    Ok(())
                }
                .boxed_local()
            })?;
            let u = w.watch("other.json", counter(other.clone()))?;
            emit(&input, &t);
            fire().await;
            assert_eq!(n.get(), 1);
            for _ in 0..100 {
                emit(&input, &t);
            }
            emit(&input, &u);
            fire().await;
            assert_eq!(n.get(), 1);
            assert_eq!(other.get(), 1);
            gate.notify_one();
            pump().await;
            assert_eq!(n.get(), 2);
            w.dispose()?.await?;
            assert_eq!(w.stats().callbacks_running, 0);
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn stopped_running_registration_keeps_quota_until_capture_destruction() -> Result<(), BoxError>
{
    struct Probe(Rc<Cell<bool>>);
    impl Drop for Probe {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    let runtime = Runtime::new();
    runtime
        .run(async {
            let mut o = options();
            o.max_watches = 1;
            let dropped = Rc::new(Cell::new(false));
            let p = Rc::new(Probe(dropped.clone()));
            let (w, input) = FileWatcher::synthetic(&runtime.context(), o)?;
            let t = w.watch("parked.json", move |_| {
                let p = p.clone();
                async move {
                    let _p = p;
                    std::future::pending::<()>().await;
                    Ok(())
                }
                .boxed_local()
            })?;
            emit(&input, &t);
            fire().await;
            drop(t);
            assert_eq!(w.stats().watches, 1);
            assert_eq!(w.stats().active, 0);
            assert!(matches!(
                w.watch_source("next.rs"),
                Err(WatchError::LimitExceeded(_))
            ));
            w.dispose()?.await?;
            assert!(dropped.get());
            assert_eq!(w.stats().watches, 0);
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn callback_self_stop_has_no_borrow_panic_and_self_dispose_is_refused() -> Result<(), BoxError>
{
    let runtime = Runtime::new();
    runtime
        .run(async {
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let w = Rc::new(w);
            let weak = Rc::downgrade(&w);
            let slot: Rc<RefCell<Option<WatchToken>>> = Rc::new(RefCell::new(None));
            let s = slot.clone();
            let t = w.watch("self.json", move |_| {
                let w = weak.upgrade().unwrap();
                assert!(matches!(
                    w.dispose(),
                    Err(WatchError::Lifecycle(LifecycleError::Admission(
                        AdmissionError::ReentrantDrain
                    )))
                ));
                s.borrow().as_ref().unwrap().stop();
                async { Ok(()) }.boxed_local()
            })?;
            *slot.borrow_mut() = Some(t);
            emit(&input, slot.borrow().as_ref().unwrap());
            fire().await;
            assert_eq!(w.stats().active, 0);
            assert!(runtime.take_failures().is_empty());
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn synthetic_cross_thread_delivery_and_owner_disposal_close_input() -> Result<(), BoxError> {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<SyntheticEvents>();
    let runtime = Runtime::new();
    runtime
        .run(async {
            let count = Rc::new(Cell::new(0));
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let t = w.watch("thread.json", counter(count.clone()))?;
            let sender = input.clone();
            let path = t.path().to_path_buf();
            std::thread::spawn(move || {
                sender
                    .emit(Change {
                        path,
                        kind: ChangeKind::Modified,
                    })
                    .unwrap()
            })
            .join()
            .unwrap();
            fire().await;
            assert_eq!(count.get(), 1);
            runtime.context().drain()?.await?;
            assert_eq!(w.stats().watches, 0);
            assert!(input.rescan().is_err());
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn one_hundred_watch_dispose_cycles_restore_parent_effect_baseline() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let baseline = root.get_effects()?.len();
            for _ in 0..100 {
                let (w, input) = FileWatcher::synthetic(&root, options())?;
                let t = w.watch_source("cycle.rs")?;
                emit(&input, &t);
                drop(t);
                w.dispose()?.await?;
                assert_eq!(w.stats().watches, 0);
                assert_eq!(root.get_effects()?.len(), baseline);
            }
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn invalid_options_and_raw_host_admission_create_no_work() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    assert!(FileWatcher::synthetic(&runtime.context(), options()).is_err());
    runtime
        .run(async {
            let mut o = options();
            o.queue_capacity = 0;
            assert!(matches!(
                FileWatcher::synthetic(&runtime.context(), o),
                Err(WatchError::InvalidOptions(_))
            ));
            let mut o = options();
            o.poll_interval = Duration::MAX;
            assert!(matches!(
                FileWatcher::new(&runtime.context(), o),
                Err(WatchError::InvalidOptions(_))
            ));
            let (w, _) = FileWatcher::synthetic(&runtime.context(), options())?;
            let w = Rc::new(w);
            let raw = w.clone();
            assert!(
                tokio::task::spawn_local(async move { raw.watch_source("raw.rs").is_err() })
                    .await?
            );
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn callback_and_error_destructors_can_reenter_watcher_without_borrows() -> Result<(), BoxError>
{
    struct Reenter {
        watcher: std::rc::Weak<FileWatcher>,
        dropped: Rc<Cell<usize>>,
    }
    impl std::fmt::Debug for Reenter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("reenter")
        }
    }
    impl std::fmt::Display for Reenter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("reenter")
        }
    }
    impl Error for Reenter {}
    impl Drop for Reenter {
        fn drop(&mut self) {
            if let Some(w) = self.watcher.upgrade() {
                w.take_events();
            }
            self.dropped.set(self.dropped.get() + 1);
        }
    }
    let runtime = Runtime::new();
    runtime
        .run(async {
            let mut o = options();
            o.event_capacity = 1;
            let (w, input) = FileWatcher::synthetic(&runtime.context(), o)?;
            let w = Rc::new(w);
            let dropped = Rc::new(Cell::new(0));
            let capture = Reenter {
                watcher: Rc::downgrade(&w),
                dropped: dropped.clone(),
            };
            let idle = w.watch("idle.json", move |_| {
                let _ = &capture;
                async { Ok(()) }.boxed_local()
            })?;
            idle.stop();
            assert_eq!(dropped.get(), 1);
            let weak = Rc::downgrade(&w);
            let d = dropped.clone();
            let t = w.watch("error.json", move |_| {
                let error = Reenter {
                    watcher: weak.clone(),
                    dropped: d.clone(),
                };
                async move { Err(Box::new(error) as BoxError) }.boxed_local()
            })?;
            emit(&input, &t);
            fire().await;
            emit(&input, &t);
            fire().await;
            assert!(dropped.get() >= 2);
            w.dispose()?.await?;
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn host_destruction_stops_real_producer_with_surviving_watcher() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    let w = runtime
        .run(async {
            let mut o = options();
            o.poll_interval = Duration::from_secs(60);
            let w = FileWatcher::new(&runtime.context(), o)?;
            let t = w.watch_source("host-drop.rs")?;
            Ok::<_, BoxError>((w, t))
        })
        .await??;
    assert!(w.0.stats().producer_running);
    drop(runtime);
    tokio::time::timeout(Duration::from_secs(2), async {
        while w.0.stats().producer_running {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await?;
    assert_eq!(w.0.stats().active, 0);
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn concurrent_input_cannot_repopulate_queue_after_joined_disposal() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let (w, input) = FileWatcher::synthetic(&runtime.context(), options())?;
            let t = w.watch_source("race.rs")?;
            let input_thread = input.clone();
            let path = t.path().to_path_buf();
            let gate = std::sync::Arc::new(std::sync::Barrier::new(2));
            let g = gate.clone();
            let thread = std::thread::spawn(move || {
                g.wait();
                for _ in 0..20000 {
                    let _ = input_thread.emit(Change {
                        path: path.clone(),
                        kind: ChangeKind::Modified,
                    });
                }
            });
            gate.wait();
            w.dispose()?.await?;
            thread.join().unwrap();
            assert_eq!(w.stats().queued, 0);
            assert!(input.rescan().is_err());
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}
