//! Whole-program R0 integration uses the runnable application's exact implementation.
#![cfg(feature = "include")]

#[path = "../examples/system_lifecycle/app.rs"]
mod app;

use app::{Application, Directory, OriginalError, drive, has_cause};
use futures_util::{FutureExt, StreamExt};
use re_cordis_core::{BoxError, DispatchOptions, Runtime};
use re_cordis_hmr::WatchEvent;
use re_cordis_include::JsonEdit;
use re_cordis_loader::EntryState;
use serde_json::json;
use std::{cell::RefCell, io, rc::Rc, time::Duration};

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn provider_move_keeps_exact_old_binding_and_sibling_progress() -> Result<(), BoxError> {
    let directory = Directory::new()?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let app =
                Application::boot(&runtime.context(), &directory.0, io::sink(), false).await?;
            let before = app.entry("provider");
            let old = app.probe.providers.borrow()[0].upgrade().unwrap();
            let old_weak = Rc::downgrade(&old);
            let (entered, release) = app.hold_consumer();
            let mut moved = app.include.move_entry("provider", Some("right"), 0)?;
            entered.await?;
            assert!(old.alive.get());
            assert_eq!(old.call()?, "v1:1");
            assert!((&mut moved).now_or_never().is_none());
            assert_eq!(
                app.probe.providers.borrow().len(),
                1,
                "replacement waits for dependent"
            );
            app.include
                .edit("sibling", JsonEdit::new().set("config", json!(8)))?
                .await?;
            tokio::time::advance(Duration::from_millis(20)).await;
            drive().await;
            assert_eq!(app.probe.siblings.borrow().last(), Some(&8));
            assert!(
                app.probe.ticks.get() > 0,
                "unrelated owned timer progresses"
            );
            assert!(old.alive.get());
            release.send(()).unwrap();
            moved.await?;
            assert!(!old.alive.get());
            let after = app.entry("provider");
            assert_eq!(before.id, after.id);
            assert_ne!(before.fiber_id, after.fiber_id);
            assert_eq!(after.parent.unwrap().as_str(), "right");
            assert_eq!(app.entry("consumer").state, EntryState::Pending);
            app.include
                .move_entry("consumer", Some("right"), 1)?
                .await?;
            let current = app
                .probe
                .providers
                .borrow()
                .last()
                .unwrap()
                .upgrade()
                .unwrap();
            assert!(!Rc::ptr_eq(&old, &current));
            assert_eq!(app.probe.consumers.borrow().last(), Some(&current.serial));
            {
                let retained = app.probe.retained.borrow();
                assert_eq!(retained.len(), 1);
                assert_eq!(
                    retained[0],
                    (old.serial, true, "v1:1".into(), "v1:1".into())
                );
            }
            drop(old);
            assert!(
                old_weak.upgrade().is_none(),
                "old value released after cleanup and caller drop"
            );
            drop(current);
            app.include.flush()?.await?;
            let data: serde_json::Value =
                serde_json::from_slice(&std::fs::read(directory.0.join("cordis.json"))?)?;
            assert_eq!(data[1]["config"][0]["id"], "provider");
            assert_eq!(data[1]["config"][1]["id"], "consumer");
            app.shutdown().await?;
            assert!(runtime.take_failures().is_empty());
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn linked_replacement_and_distinct_failure_channels_recover() -> Result<(), BoxError> {
    let directory = Directory::new()?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let app =
                Application::boot(&runtime.context(), &directory.0, io::sink(), false).await?;
            let before = app.entry("provider");
            let bytes = std::fs::read(directory.0.join("cordis.json"))?;
            let replaced = app.replace("v2", None)?.await?;
            assert!(replaced.failures.is_empty());
            assert!(!replaced.changed);
            assert_eq!(
                app.probe
                    .providers
                    .borrow()
                    .last()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .call()?,
                "v2:1"
            );
            assert_eq!(app.entry("provider").id, before.id);
            assert_ne!(app.entry("provider").factory_id, before.factory_id);
            assert_eq!(std::fs::read(directory.0.join("cordis.json"))?, bytes);
            let failed = app.replace("broken", Some(201))?.await?;
            assert_eq!(failed.failures.len(), 1);
            assert_eq!(
                has_cause::<OriginalError>(&failed.failures[0]).unwrap().0,
                201
            );
            assert_eq!(app.entry("provider").state, EntryState::Failed);
            assert_eq!(app.entry("consumer").state, EntryState::Pending);
            assert_eq!(
                app.entry("provider").source.get::<u32>().unwrap().as_ref(),
                &1
            );
            assert!(app.replace("v3", None)?.await?.failures.is_empty());
            assert_eq!(app.entry("provider").state, EntryState::Active);
            assert_eq!(app.entry("consumer").state, EntryState::Active);
            assert_eq!(
                app.probe
                    .providers
                    .borrow()
                    .last()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .call()?,
                "v3:1"
            );
            app.include
                .edit("provider", JsonEdit::new().set("config", json!(5)))?
                .await?;
            assert!(app.include.settle()?.await?.failures.is_empty());
            let updated = app
                .probe
                .providers
                .borrow()
                .last()
                .unwrap()
                .upgrade()
                .unwrap();
            assert_eq!(updated.call()?, "v3:5");
            assert_eq!(app.probe.consumers.borrow().last(), Some(&updated.serial));
            drop(updated);

            let rejected = app
                .include
                .edit("sibling", JsonEdit::new().set("config", json!(99)))?
                .await
                .unwrap_err();
            assert_eq!(has_cause::<OriginalError>(&rejected).unwrap().0, 202);
            assert_eq!(
                app.entry("sibling").source.get::<u32>().unwrap().as_ref(),
                &99
            );
            app.include
                .edit("sibling", JsonEdit::new().set("config", json!(8)))?
                .await?;
            assert_eq!(app.probe.siblings.borrow().last(), Some(&8));
            app.include.flush()?.await?;

            std::fs::write(directory.0.join("cordis.json"), b"{broken")?;
            app.emit_config()?;
            app.fire().await;
            let errors = app
                .watcher
                .take_events()
                .into_iter()
                .filter_map(|event| match event {
                    WatchEvent::Failure(error) => Some(error),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(errors.len(), 1);
            assert!(has_cause::<serde_json::Error>(&errors[0]).is_some());
            assert!(errors[0].path.as_ref().unwrap().ends_with("cordis.json"));
            assert_eq!(app.entry("provider").state, EntryState::Active);
            let valid = serde_json::to_vec(&app.include.data())?;
            std::fs::write(directory.0.join("cordis.json"), valid)?;
            app.emit_config()?;
            app.fire().await;
            assert!(
                !app.watcher
                    .take_events()
                    .iter()
                    .any(|event| matches!(event, WatchEvent::Failure(_)))
            );
            let diagnostics = runtime.take_failures();
            // Original lifecycle apply/update diagnostics are separate from parser history.
            assert!(
                diagnostics
                    .iter()
                    .all(|error| has_cause::<serde_json::Error>(error).is_none())
            );
            assert_eq!(
                diagnostics
                    .iter()
                    .filter(|error| has_cause::<OriginalError>(*error)
                        .is_some_and(|cause| cause.0 == 201))
                    .count(),
                1
            );
            assert_eq!(diagnostics.len(), 2, "one load and one update failure");
            let load_diagnostic = diagnostics
                .iter()
                .find(|error| {
                    has_cause::<OriginalError>(*error).is_some_and(|cause| cause.0 == 201)
                })
                .unwrap();
            let update_diagnostic = diagnostics
                .iter()
                .find(|error| {
                    has_cause::<OriginalError>(*error).is_some_and(|cause| cause.0 == 202)
                })
                .unwrap();
            assert!(
                std::ptr::eq(
                    has_cause::<OriginalError>(&failed.failures[0]).unwrap(),
                    has_cause::<OriginalError>(load_diagnostic).unwrap(),
                ),
                "report and Runtime observe the same load failure"
            );
            assert!(
                std::ptr::eq(
                    has_cause::<OriginalError>(&rejected).unwrap(),
                    has_cause::<OriginalError>(update_diagnostic).unwrap(),
                ),
                "operation and Runtime observe the same update failure"
            );
            app.shutdown().await?;
            assert!(runtime.take_failures().is_empty());
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

async fn pending_shutdown(invalid_final: bool, cleanup_error: bool) -> Result<(), BoxError> {
    let directory = Directory::new()?;
    let runtime = Runtime::new();
    runtime.run(async {
        let root = runtime.context();
        let baseline = root.get_effects()?.len();
        let bytes = Rc::new(RefCell::new(Vec::new()));
        let app = Application::boot(&root, &directory.0, app::Writer(bytes.clone()), false).await?;
        let writer = app.probe.writer.upgrade().unwrap();
        drop(writer);
        let catalog = app.probe.catalog.clone();
        let callback = app.probe.callback.clone();
        let timer = app.probe.timer.clone();
        let providers = app.probe.providers.borrow().clone();
        let (entered, release) = app.hold_consumer();
        app.probe.fail_cleanup.set(cleanup_error);
        let mut source = app.include.data();
        source[0]["config"][0]["config"] = json!(2);
        std::fs::write(directory.0.join("cordis.json"), serde_json::to_vec(&source)?)?;
        app.emit_config()?;
        app.fire().await;
        entered.await?;
        assert_eq!(app.watcher.stats().callbacks_running, 1);
        let admitted = app.include.create(json!({"id":"pending","name":"sibling","config":41}), None, 3)?;
        drop(admitted);
        drive().await;
        assert!(app.include.pending() > 0);
        app.emit_config()?;
        drive().await;
        if invalid_final {
            std::fs::write(directory.0.join("cordis.json"), b"{broken-final")?;
        }
        let mut requested = app.owner.dispose()?;
        drive().await;
        assert!((&mut requested).now_or_never().is_none(), "request is not joined completion");
        assert_eq!(app.probe.live_providers.get(), 1, "exact provider retained by held consumer");
        assert_eq!(app.watcher.stats().active, 0);
        assert!(catalog.upgrade().is_some());
        if !invalid_final {
            let persisted: serde_json::Value = serde_json::from_slice(&std::fs::read(directory.0.join("cordis.json"))?)?;
            assert!(persisted.as_array().unwrap().iter().any(|entry| entry["id"] == "pending"));
        }
        release.send(()).unwrap();
        requested.await?;
        app.watcher.dispose()?.await?;
        if invalid_final {
            let error = app.include.final_error().unwrap();
            assert!(has_cause::<serde_json::Error>(&error).is_some());
            assert!(has_cause::<serde_json::Error>(&app.include.final_error().unwrap()).is_some());
            assert!(app.include.pending() > 0, "failed journal remains observable until handle drop");
            assert_eq!(std::fs::read(directory.0.join("cordis.json"))?, b"{broken-final");
        } else {
            assert!(app.include.final_error().is_none());
            assert_eq!(app.include.pending(), 0);
        }
        let stats = app.watcher.stats();
        assert_eq!((stats.watches, stats.active, stats.paths, stats.queued, stats.pending, stats.callbacks_running), (0,0,0,0,0,0));
        assert!(!stats.producer_running);
        assert!(app.include.entries().iter().all(|entry| entry.fiber_id.is_none() && entry.outer_id.is_none()), "no retained core ownership");
        assert!(app.include.entries().iter().all(|entry| entry.state == EntryState::Disposed));
        assert_eq!(app.probe.live_providers.get(), 0);
        assert_eq!(app.probe.live_consumers.get(), 0);
        assert!(providers.iter().all(|provider| provider.upgrade().is_none()));
        assert!(app.probe.writer.upgrade().is_none(), "owned writer released at owner join");
        assert!(callback.upgrade().is_none(), "watch closure released at watcher join");
        assert!(timer.upgrade().is_none(), "timer closure destroyed at owner join");
        assert!(root.registry().is_empty()?);
        assert_eq!(root.get_effects()?.len(), baseline);
        let log_end = bytes.borrow().len();
        root.logger(Some("system")).info("after shutdown")?;
        assert_eq!(bytes.borrow().len(), log_end);
        let events = app.watcher.take_events();
        let watch_errors = events.iter().filter_map(|event| match event {
                WatchEvent::Failure(error) => Some(error),
                _ => None,
            }).collect::<Vec<_>>();
            assert_eq!(watch_errors.len(), 1, "one retained canceled refresh occurrence");
            assert_eq!(watch_errors[0].stage, re_cordis_hmr::WatchFailureStage::Callback);
            assert!(watch_errors[0].path.as_ref().unwrap().ends_with("cordis.json"));
            let include_cause = has_cause::<re_cordis_include::IncludeError>(watch_errors[0]).unwrap();
            assert!(include_cause.message.contains("loader worker stopped"));
            assert!(include_cause.location.is_some());
        assert!(!events.iter().any(|event| matches!(event, WatchEvent::Failure(error) if has_cause::<OriginalError>(error).is_some())));
        let report = root.drain()?.await?;
        assert!(report.failures().is_empty(), "nested errors use Runtime diagnostics");
        let diagnostics = runtime.take_failures();
        assert_eq!(diagnostics.iter().filter(|error| has_cause::<OriginalError>(*error).is_some_and(|cause| cause.0 == 301)).count(), usize::from(cleanup_error));
        assert_eq!(diagnostics.iter().filter(|error| has_cause::<serde_json::Error>(*error).is_some()).count(), usize::from(invalid_final));
        assert_eq!(diagnostics.len(), usize::from(cleanup_error) + usize::from(invalid_final));
        assert!(catalog.upgrade().is_some(), "catalog remains explicitly owned by Include handle");
        let probe = app.probe.clone();
        let mut stream = app.stream;
        assert!(stream.next().await.unwrap().is_err());
        assert!(stream.next().await.is_none());
        drop((stream, app.include, app.watcher, app.owner, app.config, app.audit));
        drive().await;
        assert!(catalog.upgrade().is_none(), "catalog released after final application handles drop");
        assert!(probe.retained.borrow().iter().all(|record| record.1));
        assert!(runtime.take_failures().is_empty());
        Ok::<_, BoxError>(())
    }).await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn shutdown_joins_pending_work_and_persists_journal() -> Result<(), BoxError> {
    pending_shutdown(false, false).await
}
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn shutdown_preserves_distinct_io_and_cleanup_failures() -> Result<(), BoxError> {
    pending_shutdown(true, true).await
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn hundred_composed_lifecycles_restore_baselines_and_captures() -> Result<(), BoxError> {
    let directory = Directory::new()?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let effects = root.get_effects()?.len();
            let registry = root.registry().len()?;
            for cycle in 0..100 {
                let app = Application::boot(&root, &directory.0, io::sink(), false).await?;
                let probe = app.probe.clone();
                let key = app.key.clone();
                let pulse = app.pulse.clone();
                let catalog = probe.catalog.clone();
                let timer = probe.timer.clone();
                let callback = probe.callback.clone();
                app.include
                    .edit("provider", JsonEdit::new().set("config", json!(cycle + 2)))?
                    .await?;
                assert!(app.replace("v2", None)?.await?.failures.is_empty());
                app.include.flush()?.await?;
                root.emit(&pulse, (), DispatchOptions::default())?.await?;
                let observed = probe.pulses.get();
                assert!(observed > 0);
                app.shutdown().await?;
                drive().await;
                root.emit(&pulse, (), DispatchOptions::default())?.await?;
                assert_eq!(probe.pulses.get(), observed, "consumer listeners restored");
                assert!(root.get(&key)?.is_none());
                assert_eq!(probe.live_providers.get(), 0);
                assert!(
                    probe
                        .providers
                        .borrow()
                        .iter()
                        .all(|value| value.upgrade().is_none())
                );
                assert!(probe.writer.upgrade().is_none());
                assert!(catalog.upgrade().is_none());
                assert!(callback.upgrade().is_none());
                assert!(timer.upgrade().is_none());
                assert_eq!(root.get_effects()?.len(), effects);
                assert_eq!(root.registry().len()?, registry);
                assert!(runtime.take_failures().is_empty());
            }
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn bounded_real_watcher_updates_the_shared_application() -> Result<(), BoxError> {
    let directory = Directory::new()?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let app = Application::boot(&runtime.context(), &directory.0, io::sink(), true).await?;
            let mut source = app.include.data();
            source[2]["config"] = json!(12);
            let staging = directory.0.join("staging.json");
            std::fs::write(&staging, serde_json::to_vec(&source)?)?;
            std::fs::rename(staging, directory.0.join("cordis.json"))?;
            tokio::time::timeout(Duration::from_secs(5), async {
                while app.probe.siblings.borrow().last() != Some(&12) {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await?;
            let events = app.watcher.take_events();
            assert!(
                !events
                    .iter()
                    .any(|event| matches!(event, WatchEvent::Failure(_)))
            );
            app.shutdown().await?;
            assert!(runtime.take_failures().is_empty());
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn runnable_example_uses_the_same_application() -> Result<(), BoxError> {
    app::demonstration(false).await
}
