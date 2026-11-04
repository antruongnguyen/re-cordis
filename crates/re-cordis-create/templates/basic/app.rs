use re_cordis_core::{BoxError, CleanupReport, Context, ExporterOptions, Plugin, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntryId, EntrySpec, Loader};
use re_cordis_logger::{ColorMode, ConsoleOptions, Timestamp};
use std::io::Write;

#[derive(Clone)]
struct Settings {
    message: String,
}
fn entry(message: &str) -> EntrySpec {
    EntrySpec::new(
        "greeting",
        Config::new(Settings {
            message: message.into(),
        }),
    )
}
struct Application {
    loader: Loader,
    greeting: ServiceKey<String>,
    id: EntryId,
}
fn check_cleanup(report: &CleanupReport) -> Result<(), BoxError> {
    if let Some(failure) = report.failures().first() {
        return Err(Box::new(failure.clone()));
    }
    Ok(())
}
impl Application {
    async fn boot(root: &Context, writer: impl Write + 'static) -> Result<Self, BoxError> {
        re_cordis_logger::register(
            root,
            writer,
            ConsoleOptions {
                colors: ColorMode::Plain,
                timestamp: Timestamp::Off,
                ..Default::default()
            },
            ExporterOptions::default(),
        )?;
        let greeting = ServiceKey::<String>::new("greeting");
        let plugin = Plugin::new("greeting", {
            let greeting = greeting.clone();
            move |ctx, settings: Settings| {
                let greeting = greeting.clone();
                async move {
                    ctx.provide(&greeting, settings.message.clone())?;
                    ctx.logger(Some("app")).info(&settings.message)?;
                    Ok::<_, BoxError>(())
                }
            }
        });
        let mut catalog = Catalog::new();
        catalog.register("greeting", plugin)?;
        let loader = Loader::new(root, catalog)?;
        let id = loader.create(entry("hello from Cordis"))?;
        let report = loader.settle()?.await?;
        if let Some(failure) = report.failures.first() {
            return Err(Box::new(failure.error.clone()));
        }
        Ok(Self {
            loader,
            greeting,
            id,
        })
    }
    async fn update(&self, message: &str) -> Result<(), BoxError> {
        self.loader.update(&self.id, entry(message))?.await?;
        Ok(())
    }
    async fn shutdown(self, root: &Context) -> Result<(), BoxError> {
        // Even a loader operation failure must not skip the root's joined drain.
        let disposed = self.loader.dispose()?.await;
        let drained = root.drain()?.await;
        disposed?;
        check_cleanup(&drained?)?;
        Ok(())
    }
}
pub(crate) async fn run(root: &Context, writer: impl Write + 'static) -> Result<(), BoxError> {
    let work = async {
        let app = Application::boot(root, writer).await?;
        app.update("updated greeting").await?;
        let value = root.get(&app.greeting)?.ok_or("greeting service missing")?;
        root.logger(Some("app")).info(format!("service: {value}"))?;
        drop(value);
        app.shutdown(root).await
    }
    .await;
    // Covers partial setup/update failure as well as a successful shutdown.
    let drained = root.drain()?.await;
    work?;
    check_cleanup(&drained?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use re_cordis_core::Runtime;
    use std::{
        cell::RefCell,
        error::Error,
        io::{self, Write},
        rc::Rc,
    };

    struct Writer {
        bytes: Rc<RefCell<Vec<u8>>>,
        _capture: Rc<()>,
    }
    impl Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.bytes.borrow_mut().extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn executor() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
    }
    #[test]
    fn hundred_cycles_join_update_and_release_service_writer_and_catalog() {
        let runtime = Runtime::new();
        executor()
            .block_on(runtime.run(async {
                let root = runtime.context();
                let effects = root.get_effects().unwrap().len();
                let groups = root.registry().len().unwrap();
                for _ in 0..100 {
                    let bytes = Rc::new(RefCell::new(Vec::new()));
                    let capture = Rc::new(());
                    let writer_weak = Rc::downgrade(&capture);
                    let app = Application::boot(
                        &root,
                        Writer {
                            bytes: bytes.clone(),
                            _capture: capture,
                        },
                    )
                    .await
                    .unwrap();
                    assert!(writer_weak.upgrade().is_some());
                    let old = root.get(&app.greeting).unwrap().unwrap();
                    assert_eq!(&*old, "hello from Cordis");
                    let old_weak = Rc::downgrade(&old);
                    drop(old);
                    app.update("new message").await.unwrap();
                    assert!(
                        old_weak.upgrade().is_none(),
                        "old provider released at update"
                    );
                    let new = root.get(&app.greeting).unwrap().unwrap();
                    assert_eq!(&*new, "new message");
                    let new_weak = Rc::downgrade(&new);
                    drop(new);
                    let key = app.greeting.clone();
                    app.shutdown(&root).await.unwrap();
                    assert!(
                        new_weak.upgrade().is_none(),
                        "new provider released at shutdown"
                    );
                    assert!(
                        writer_weak.upgrade().is_none(),
                        "owned writer capture released"
                    );
                    assert!(root.get(&key).unwrap().is_none());
                    assert_eq!(root.get_effects().unwrap().len(), effects);
                    assert_eq!(root.registry().len().unwrap(), groups);
                    assert!(runtime.take_failures().is_empty());
                    let before = bytes.borrow().len();
                    root.logger(Some("app")).info("after shutdown").unwrap();
                    assert_eq!(bytes.borrow().len(), before, "exporter removed");
                }
            }))
            .unwrap();
    }
    #[test]
    fn run_uses_updated_config_and_releases_writer() {
        let runtime = Runtime::new();
        let bytes = Rc::new(RefCell::new(Vec::new()));
        let capture = Rc::new(());
        let weak = Rc::downgrade(&capture);
        executor()
            .block_on(runtime.run(run(
                &runtime.context(),
                Writer {
                    bytes: bytes.clone(),
                    _capture: capture,
                },
            )))
            .unwrap()
            .unwrap();
        let text = String::from_utf8(bytes.borrow().clone()).unwrap();
        assert!(text.contains("hello from Cordis"));
        assert!(text.contains("updated greeting"));
        assert!(text.contains("service: updated greeting"));
        assert!(weak.upgrade().is_none());
        assert!(runtime.context().get_effects().unwrap().is_empty());
        assert!(runtime.context().registry().is_empty().unwrap());
        assert!(runtime.take_failures().is_empty());
    }
    #[test]
    fn root_report_and_runtime_diagnostics_retain_original_cleanup_cause() {
        let runtime = Runtime::new();
        executor()
            .block_on(runtime.run(async {
                let root = runtime.context();
                root.on_dispose_sync(|| {
                    Err::<(), _>(io::Error::new(io::ErrorKind::BrokenPipe, "root cleanup"))
                })
                .unwrap();
                let report = root.drain().unwrap().await.unwrap();
                assert_eq!(report.failures().len(), 1);
                let error = check_cleanup(&report).unwrap_err();
                let cause = error.source().unwrap().downcast_ref::<io::Error>().unwrap();
                assert_eq!(cause.kind(), io::ErrorKind::BrokenPipe);
                let diagnostics = runtime.take_failures();
                assert_eq!(diagnostics.len(), 1);
                assert_eq!(
                    diagnostics[0]
                        .source()
                        .unwrap()
                        .downcast_ref::<io::Error>()
                        .unwrap()
                        .kind(),
                    io::ErrorKind::BrokenPipe
                );
                assert_eq!(
                    report.failures().len(),
                    1,
                    "taking Runtime diagnostics does not consume report"
                );
                assert!(root.drain().unwrap().await.unwrap().failures().is_empty());
                assert!(runtime.take_failures().is_empty());
            }))
            .unwrap();
    }
    #[test]
    fn successful_loader_disposal_can_still_have_runtime_cleanup_errors() {
        let runtime = Runtime::new();
        executor()
            .block_on(runtime.run(async {
                let root = runtime.context();
                let capture = Rc::new(());
                let weak = Rc::downgrade(&capture);
                let plugin = Plugin::new("failing cleanup", move |ctx, (): ()| {
                    let capture = capture.clone();
                    async move {
                        ctx.on_dispose_sync(move || {
                            drop(capture);
                            Err::<(), _>(io::Error::new(
                                io::ErrorKind::BrokenPipe,
                                "plugin cleanup",
                            ))
                        })?;
                        Ok::<_, BoxError>(())
                    }
                });
                let mut catalog = Catalog::new();
                catalog.register("failing", plugin).unwrap();
                let loader = Loader::new(&root, catalog).unwrap();
                loader
                    .create(EntrySpec::new("failing", Config::new(())))
                    .unwrap();
                assert!(loader.settle().unwrap().await.unwrap().failures.is_empty());
                loader.dispose().unwrap().await.unwrap();
                drop(loader);
                let report = root.drain().unwrap().await.unwrap();
                assert!(
                    report.failures().is_empty(),
                    "nested callback errors use Runtime diagnostics"
                );
                assert!(weak.upgrade().is_none());
                let diagnostics = runtime.take_failures();
                assert_eq!(diagnostics.len(), 1);
                assert_eq!(
                    diagnostics[0]
                        .source()
                        .unwrap()
                        .downcast_ref::<io::Error>()
                        .unwrap()
                        .kind(),
                    io::ErrorKind::BrokenPipe
                );
                assert!(root.get_effects().unwrap().is_empty());
                assert!(root.registry().is_empty().unwrap());
            }))
            .unwrap();
    }
    #[test]
    fn failed_log_setup_still_drains_partial_resources() {
        struct Broken;
        impl Write for Broken {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::ErrorKind::BrokenPipe.into())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let runtime = Runtime::new();
        let outcome = executor()
            .block_on(runtime.run(run(&runtime.context(), Broken)))
            .unwrap();
        assert!(outcome.is_err());
        assert!(runtime.context().get_effects().unwrap().is_empty());
        assert!(runtime.context().registry().is_empty().unwrap());
        assert!(!runtime.take_failures().is_empty());
    }
}
