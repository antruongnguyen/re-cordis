//! Writer failures and actual core owner/snapshot semantics.
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    convert::Infallible,
    io::{self, Write},
    rc::Rc,
};

use re_cordis_core::{
    AdmissionError, BoxError, ErrorValue, ExporterOptions, ExporterRegistration, Level, LogError,
    Logger, Plugin, Runtime,
};
use re_cordis_logger::{ColorMode, ConsoleOptions, Timestamp, register};

#[derive(Clone, Default)]
struct Bytes(Rc<RefCell<Vec<u8>>>);
impl Bytes {
    fn text(&self) -> String {
        String::from_utf8(self.0.borrow().clone()).unwrap()
    }
}
impl Write for Bytes {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn plain() -> ConsoleOptions {
    ConsoleOptions {
        timestamp: Timestamp::Off,
        colors: ColorMode::Plain,
        ..Default::default()
    }
}

#[tokio::test(flavor = "current_thread")]
async fn routing_defaults_overrides_two_exporters_and_manual_removal() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let first = Bytes::default();
            let second = Bytes::default();
            let registration = register(
                &root,
                first.clone(),
                plain(),
                ExporterOptions {
                    default_level: Some(Level::Debug),
                    levels: BTreeMap::from([("quiet".into(), Level::Error)]),
                },
            )
            .unwrap();
            register(&root, second.clone(), plain(), ExporterOptions::default()).unwrap();
            root.logger(Some("test")).debug("debug").unwrap();
            root.logger(Some("quiet")).info("info").unwrap();
            root.logger(Some("quiet")).error("error").unwrap();
            assert_eq!(first.text(), "[D] test debug\n[E] quiet error\n");
            assert_eq!(second.text(), "[I] quiet info\n[E] quiet error\n");
            assert_eq!(
                root.log_buffer().unwrap().len(),
                2,
                "buffer filtering stays in core"
            );
            assert!(registration.remove().unwrap());
            root.logger(Some("test")).info("after remove").unwrap();
            assert_eq!(first.text(), "[D] test debug\n[E] quiet error\n");
            let report = root.drain().unwrap().await.unwrap();
            assert!(report.failures().is_empty());
            root.logger(Some("test")).info("after drain").unwrap();
            assert_eq!(
                second.text(),
                "[I] quiet info\n[E] quiet error\n[I] test after remove\n"
            );
            assert!(root.get_effects().unwrap().is_empty());
        })
        .await
        .unwrap();
}

struct ShortWriter {
    bytes: Bytes,
    interrupt: bool,
    flushes: Rc<Cell<usize>>,
}
impl Write for ShortWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.interrupt {
            self.interrupt = false;
            return Err(io::ErrorKind::Interrupted.into());
        }
        self.bytes.write(&bytes[..bytes.len().min(2)])
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flushes.set(self.flushes.get() + 1);
        Ok(())
    }
}

#[tokio::test(flavor = "current_thread")]
async fn short_and_interrupted_writes_complete_with_one_flush_per_record() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let bytes = Bytes::default();
            let flushes = Rc::new(Cell::new(0));
            register(
                &root,
                ShortWriter {
                    bytes: bytes.clone(),
                    interrupt: true,
                    flushes: flushes.clone(),
                },
                plain(),
                ExporterOptions::default(),
            )
            .unwrap();
            for (level, msg) in [
                (Level::Error, "error"),
                (Level::Warn, "warning"),
                (Level::Info, "info"),
            ] {
                root.logger(Some("test")).log(level, msg).unwrap();
            }
            assert_eq!(
                bytes.text(),
                "[E] test error\n[W] test warning\n[I] test info\n"
            );
            assert_eq!(flushes.get(), 3);
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

struct FailureWriter {
    bytes: Bytes,
    kind: io::ErrorKind,
    partial: bool,
    flush: bool,
}
impl Write for FailureWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.flush {
            return self.bytes.write(bytes);
        }
        if self.partial {
            self.partial = false;
            return self.bytes.write(&bytes[..2]);
        }
        if self.kind == io::ErrorKind::WriteZero {
            Ok(0)
        } else {
            Err(self.kind.into())
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        if self.flush {
            Err(self.kind.into())
        } else {
            Ok(())
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn write_partial_zero_and_flush_failures_are_visible_and_do_not_skip_other_exporters() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            for (kind, partial, flush, expected) in [
                (io::ErrorKind::BrokenPipe, false, false, ""),
                (io::ErrorKind::BrokenPipe, true, false, "[I"),
                (io::ErrorKind::WriteZero, false, false, ""),
                (io::ErrorKind::Other, false, true, "[I] test message\n"),
            ] {
                let failed = Bytes::default();
                let good = Bytes::default();
                register(
                    &root,
                    FailureWriter {
                        bytes: failed.clone(),
                        kind,
                        partial,
                        flush,
                    },
                    plain(),
                    ExporterOptions::default(),
                )
                .unwrap();
                register(&root, good.clone(), plain(), ExporterOptions::default()).unwrap();
                let LogError::Exporters(errors) =
                    root.logger(Some("test")).info("message").unwrap_err()
                else {
                    panic!("export error");
                };
                assert_eq!(errors.len(), 1);
                assert_eq!(
                    std::error::Error::source(&errors[0])
                        .unwrap()
                        .downcast_ref::<io::Error>()
                        .unwrap()
                        .kind(),
                    kind
                );
                assert_eq!(failed.text(), expected);
                assert_eq!(good.text(), "[I] test message\n");
                assert!(root.drain().unwrap().await.unwrap().failures().is_empty());
            }
            assert_eq!(root.log_buffer().unwrap().len(), 4);
            assert!(
                runtime.take_failures().is_empty(),
                "explicit log results are the error channel"
            );
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn custom_auto_is_plain_and_user_ansi_is_preserved() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let bytes = Bytes::default();
            register(
                &root,
                bytes.clone(),
                ConsoleOptions {
                    timestamp: Timestamp::Off,
                    ..Default::default()
                },
                ExporterOptions::default(),
            )
            .unwrap();
            root.logger(Some("test")).info("plain").unwrap();
            root.logger(Some("test"))
                .info("\x1b[31muser\x1b[0m")
                .unwrap();
            assert_eq!(
                bytes.text(),
                "[I] test plain\n[I] test \x1b[31muser\x1b[0m\n"
            );
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

struct CaptureWriter {
    guard: Rc<()>,
    drops: Rc<Cell<usize>>,
    bytes: Bytes,
    on_drop: Option<Logger>,
}
impl Write for CaptureWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.bytes.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Drop for CaptureWriter {
    fn drop(&mut self) {
        let _ = Rc::strong_count(&self.guard);
        self.drops.set(self.drops.get() + 1);
        if let Some(logger) = &self.on_drop {
            logger.info("writer dropped").unwrap();
        }
    }
}
fn captured(
    drops: &Rc<Cell<usize>>,
    on_drop: Option<Logger>,
) -> (CaptureWriter, std::rc::Weak<()>) {
    let guard = Rc::new(());
    let weak = Rc::downgrade(&guard);
    (
        CaptureWriter {
            guard,
            drops: drops.clone(),
            bytes: Bytes::default(),
            on_drop,
        },
        weak,
    )
}

#[tokio::test(flavor = "current_thread")]
async fn failed_admission_releases_owned_writer_without_invoking_it() {
    let runtime = Runtime::new();
    let ctx = runtime.context();
    let drops = Rc::new(Cell::new(0));
    let (writer, weak) = captured(&drops, None);
    assert!(matches!(
        register(&ctx, writer, plain(), ExporterOptions::default()),
        Err(AdmissionError::HostNotRunning)
    ));
    assert!(weak.upgrade().is_none());
    let other = Runtime::new();
    other
        .run(async {
            let (writer, weak) = captured(&drops, None);
            assert!(matches!(
                register(&ctx, writer, plain(), ExporterOptions::default()),
                Err(AdmissionError::WrongHost)
            ));
            assert!(weak.upgrade().is_none());
        })
        .await
        .unwrap();
    runtime
        .run(async {
            let fiber = ctx
                .plugin(
                    &Plugin::new("inactive", |_, ()| async { Ok::<_, Infallible>(()) }),
                    (),
                )
                .unwrap();
            fiber.ready().unwrap().await.unwrap();
            fiber.dispose().unwrap().await.unwrap();
            let (writer, weak) = captured(&drops, None);
            assert!(matches!(
                register(
                    &fiber.context(),
                    writer,
                    plain(),
                    ExporterOptions::default()
                ),
                Err(AdmissionError::InactiveContext)
            ));
            assert!(weak.upgrade().is_none());
            assert!(ctx.get_effects().unwrap().is_empty());
        })
        .await
        .unwrap();
    drop(runtime);
    let (writer, weak) = captured(&drops, None);
    assert!(matches!(
        register(&ctx, writer, plain(), ExporterOptions::default()),
        Err(AdmissionError::RuntimeDropped)
    ));
    assert!(weak.upgrade().is_none());
    assert_eq!(drops.get(), 4);
}

#[tokio::test(flavor = "current_thread")]
async fn inert_token_drop_owner_cleanup_and_manual_release_allow_destructor_logging() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let drops = Rc::new(Cell::new(0));
            let (writer, weak) = captured(&drops, Some(root.logger(Some("drop"))));
            let registration =
                register(&root, writer, plain(), ExporterOptions::default()).unwrap();
            assert!(registration.remove().unwrap());
            assert!(weak.upgrade().is_none());
            assert_eq!(drops.get(), 1);
            assert_eq!(
                root.log_buffer().unwrap().last().unwrap().message,
                "writer dropped"
            );
            let (writer, weak) = captured(&drops, Some(root.logger(Some("drop"))));
            drop(register(&root, writer, plain(), ExporterOptions::default()).unwrap());
            assert!(weak.upgrade().is_some());
            assert_eq!(root.get_effects().unwrap().len(), 1);
            assert!(root.drain().unwrap().await.unwrap().failures().is_empty());
            assert!(weak.upgrade().is_none());
            assert_eq!(drops.get(), 2);
            assert!(root.get_effects().unwrap().is_empty());
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn restart_and_disposal_release_each_writer_and_reapply_once() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let drops = Rc::new(Cell::new(0));
            let guards = Rc::new(RefCell::new(Vec::new()));
            let bytes = Bytes::default();
            let plugin = Plugin::new("console", {
                let drops = drops.clone();
                let guards = guards.clone();
                let bytes = bytes.clone();
                move |ctx, ()| {
                    let (mut writer, weak) = captured(&drops, None);
                    writer.bytes = bytes.clone();
                    guards.borrow_mut().push(weak);
                    async move {
                        register(&ctx, writer, plain(), ExporterOptions::default())?;
                        Ok::<_, BoxError>(())
                    }
                }
            });
            let fiber = root.plugin(&plugin, ()).unwrap();
            fiber.ready().unwrap().await.unwrap();
            root.logger(Some("test")).info("first").unwrap();
            fiber.restart().unwrap().await.unwrap();
            assert_eq!(drops.get(), 1);
            assert!(guards.borrow()[0].upgrade().is_none());
            assert!(guards.borrow()[1].upgrade().is_some());
            root.logger(Some("test")).info("second").unwrap();
            fiber.dispose().unwrap().await.unwrap();
            fiber.dispose().unwrap().await.unwrap();
            assert_eq!(drops.get(), 2);
            assert!(guards.borrow().iter().all(|weak| weak.upgrade().is_none()));
            root.logger(Some("test")).info("after").unwrap();
            assert_eq!(bytes.text(), "[I] test first\n[I] test second\n");
            assert!(root.registry().is_empty().unwrap());
            assert!(root.get_effects().unwrap().is_empty());
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

struct RecursiveWriter {
    logger: Logger,
    errors: Rc<Cell<usize>>,
    bytes: Bytes,
}
impl Write for RecursiveWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let LogError::Exporters(errors) = self.logger.info("nested").unwrap_err() else {
            panic!("recursive failure");
        };
        assert_eq!(errors.len(), 1);
        let source = std::error::Error::source(&errors[0])
            .unwrap()
            .downcast_ref::<io::Error>()
            .unwrap();
        assert!(source.to_string().contains("recursive log"));
        self.errors.set(self.errors.get() + 1);
        self.bytes.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[tokio::test(flavor = "current_thread")]
async fn recursive_export_is_bounded_visible_and_other_exporters_still_run() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let errors = Rc::new(Cell::new(0));
            let outer = Bytes::default();
            let good = Bytes::default();
            register(
                &root,
                RecursiveWriter {
                    logger: root.logger(Some("test")),
                    errors: errors.clone(),
                    bytes: outer.clone(),
                },
                plain(),
                ExporterOptions::default(),
            )
            .unwrap();
            register(&root, good.clone(), plain(), ExporterOptions::default()).unwrap();
            root.logger(Some("test")).info("outer").unwrap();
            assert_eq!(errors.get(), 1);
            assert_eq!(outer.text(), "[I] test outer\n");
            assert_eq!(good.text(), "[I] test nested\n[I] test outer\n");
            assert!(runtime.take_failures().is_empty());
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn removal_during_snapshot_finishes_selected_writer_then_releases_it() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let token = Rc::new(RefCell::new(None::<ExporterRegistration>));
            root.add_exporter(ExporterOptions::default(), {
                let token = token.clone();
                move |_| {
                    if let Some(registration) = token.borrow_mut().take() {
                        assert!(registration.remove().unwrap());
                    }
                    Ok::<_, Infallible>(())
                }
            })
            .unwrap();
            let drops = Rc::new(Cell::new(0));
            let (writer, weak) = captured(&drops, None);
            let bytes = writer.bytes.clone();
            *token.borrow_mut() =
                Some(register(&root, writer, plain(), ExporterOptions::default()).unwrap());
            root.logger(Some("test")).info("selected").unwrap();
            assert_eq!(bytes.text(), "[I] test selected\n");
            assert!(weak.upgrade().is_none());
            assert_eq!(drops.get(), 1);
            root.logger(Some("test")).info("future").unwrap();
            assert_eq!(bytes.text(), "[I] test selected\n");
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn source_error_aggregate_renders_core_error_without_losing_source_identity() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let bytes = Bytes::default();
            register(
                &root,
                bytes.clone(),
                ConsoleOptions {
                    show_diff: true,
                    ..plain()
                },
                ExporterOptions::default(),
            )
            .unwrap();
            let source = Rc::new(io::Error::other("message"));
            let records = root
                .logger(Some("test"))
                .log_error(
                    Level::Error,
                    ErrorValue::new(io::Error::other("outer"))
                        .with_members(vec![ErrorValue::from_rc(source.clone())]),
                )
                .unwrap();
            assert_eq!(bytes.text(), "[E] test message +0ms\n");
            assert!(std::ptr::eq(
                std::error::Error::source(records[0].failure.as_ref().unwrap())
                    .unwrap()
                    .downcast_ref::<io::Error>()
                    .unwrap(),
                Rc::as_ref(&source)
            ));
            root.drain().unwrap().await.unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn one_hundred_create_use_dispose_cycles_restore_resource_baselines() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let baseline = root.get_effects().unwrap().len();
            let drops = Rc::new(Cell::new(0));
            let guards = Rc::new(RefCell::new(Vec::new()));
            let bytes = Bytes::default();
            let plugin = Plugin::new("console-cycles", {
                let drops = drops.clone();
                let guards = guards.clone();
                let bytes = bytes.clone();
                move |ctx, ()| {
                    let (mut writer, weak) = captured(&drops, None);
                    writer.bytes = bytes.clone();
                    guards.borrow_mut().push(weak);
                    async move {
                        register(&ctx, writer, plain(), ExporterOptions::default())?;
                        Ok::<_, BoxError>(())
                    }
                }
            });
            for index in 0..100 {
                let fiber = root.plugin(&plugin, ()).unwrap();
                fiber.ready().unwrap().await.unwrap();
                assert_eq!(fiber.context().get_effects().unwrap().len(), 1);
                root.logger(Some("test")).info(index).unwrap();
                fiber.dispose().unwrap().await.unwrap();
                assert!(fiber.context().get_effects().unwrap().is_empty());
                assert!(guards.borrow()[index].upgrade().is_none());
                assert_eq!(drops.get(), index + 1);
                assert!(root.registry().is_empty().unwrap());
                assert_eq!(root.get_effects().unwrap().len(), baseline);
            }
            assert_eq!(bytes.text().lines().count(), 100);
            root.logger(Some("test")).info("after cycles").unwrap();
            assert_eq!(bytes.text().lines().count(), 100);
            assert!(runtime.take_failures().is_empty());
            assert!(root.drain().unwrap().await.unwrap().failures().is_empty());
        })
        .await
        .unwrap();
}
