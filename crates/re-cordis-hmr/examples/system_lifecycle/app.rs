//! One local, offline application shared by the runnable example and integration tests.
use re_cordis_core::{
    BoxError, Context, Event, ExporterOptions, FiberHandle, Listener, Plugin, Runtime, ServiceKey,
};
use re_cordis_hmr::{
    Change, ChangeKind, FactoryReplacement, FileWatcher, ReloadNode, ReloadPlan, SyntheticEvents,
    WatchOptions, WatchToken,
};
use re_cordis_include::{Include, IncludeOptions, IncludeTask, JsonEdit, Report};
use re_cordis_loader::{Catalog, EntrySnapshot};
use re_cordis_logger::{ColorMode, ConsoleOptions, Timestamp};
use re_cordis_timer::{IntervalStream, TimerExt};
use serde_json::{Value, json};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    error::Error,
    fmt,
    io::{self, Write},
    ops::ControlFlow,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::oneshot;

/// Stable original causes allow callers to distinguish load, update and cleanup failures.
#[derive(Debug)]
pub struct OriginalError(pub u32);
impl fmt::Display for OriginalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "system lifecycle original error {}", self.0)
    }
}
impl Error for OriginalError {}

pub fn has_cause<'a, E: Error + 'static>(mut error: &'a (dyn Error + 'static)) -> Option<&'a E> {
    loop {
        if let Some(cause) = error.downcast_ref::<E>() {
            return Some(cause);
        }
        error = error.source()?;
    }
}

/// The service is callable only while its provider's resources are alive.
pub struct Provider {
    pub serial: usize,
    pub alive: Rc<Cell<bool>>,
    revision: &'static str,
    config: u32,
}
impl Provider {
    pub fn call(&self) -> Result<String, OriginalError> {
        if !self.alive.get() {
            return Err(OriginalError(401));
        }
        Ok(format!("{}:{}", self.revision, self.config))
    }
}

type Gate = (oneshot::Sender<()>, oneshot::Receiver<()>);
/// Application observations are values/weak pointers, never service leases.
#[derive(Default)]
pub struct Probe {
    pub providers: RefCell<Vec<Weak<Provider>>>,
    pub consumers: RefCell<Vec<usize>>,
    pub retained: RefCell<Vec<(usize, bool, String, String)>>,
    pub siblings: RefCell<Vec<u32>>,
    pub live_providers: Cell<usize>,
    pub live_consumers: Cell<usize>,
    pub pulses: Cell<usize>,
    pub ticks: Cell<usize>,
    pub fail_cleanup: Cell<bool>,
    pub writer: Weak<()>,
    pub catalog: Weak<()>,
    pub callback: Weak<()>,
    pub timer: Weak<()>,
    gate: RefCell<Option<Gate>>,
    next_provider: Cell<usize>,
}

pub struct Writer(pub Rc<RefCell<Vec<u8>>>);
impl Write for Writer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
struct OwnedWriter<W> {
    inner: W,
    _capture: Rc<()>,
}
impl<W: Write> Write for OwnedWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.inner.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Caller creates a dedicated temporary directory; Include owns only admitted files.
pub struct Directory(pub PathBuf);
impl Directory {
    pub fn new() -> io::Result<Self> {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?;
        let path = std::fs::canonicalize(std::env::temp_dir())?.join(format!(
            "cordis-system-{}-{}-{}",
            std::process::id(),
            nonce.as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path)?;
        Ok(Self(path))
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn initial_source() -> Value {
    json!([
        {"id":"left","group":true,"isolate":{"system-provider":"left"},"config":[
            {"id":"provider","name":"provider","config":1},
            {"id":"consumer","name":"consumer","config":1}
        ]},
        {"id":"right","group":true,"isolate":{"system-provider":"right"},"config":[]},
        {"id":"sibling","name":"sibling","config":7}
    ])
}
fn decode(value: Option<&Value>) -> Result<u32, serde_json::Error> {
    serde_json::from_value(value.cloned().unwrap_or(json!(0)))
}

pub struct Application {
    pub owner: FiberHandle<()>,
    pub include: Include,
    pub watcher: FileWatcher,
    pub config: WatchToken,
    pub audit: WatchToken,
    pub stream: IntervalStream,
    pub probe: Rc<Probe>,
    pub key: ServiceKey<Provider>,
    pub pulse: Event<()>,
    input: Option<SyntheticEvents>,
    plan: ReloadPlan,
    catalog_capture: Weak<()>,
}
impl Application {
    /// Construct the same composition for tests and examples. The real polling
    /// option changes ingress only; all services and lifecycle operations are shared.
    pub async fn boot(
        root: &Context,
        directory: &Path,
        writer: impl Write + 'static,
        real_watch: bool,
    ) -> Result<Self, BoxError> {
        let owner = root.plugin(
            &Plugin::new("system", |_, ()| async { Ok::<_, Infallible>(()) }),
            (),
        )?;
        owner.ready()?.await?;
        let context = owner.context();
        let writer_capture = Rc::new(());
        let catalog_capture = Rc::new(());
        let callback_capture = Rc::new(());
        let timer_capture = Rc::new(());
        let probe = Rc::new(Probe {
            writer: Rc::downgrade(&writer_capture),
            catalog: Rc::downgrade(&catalog_capture),
            callback: Rc::downgrade(&callback_capture),
            timer: Rc::downgrade(&timer_capture),
            ..Default::default()
        });
        re_cordis_logger::register(
            &context,
            OwnedWriter {
                inner: writer,
                _capture: writer_capture,
            },
            ConsoleOptions {
                colors: ColorMode::Plain,
                timestamp: Timestamp::Off,
                ..Default::default()
            },
            ExporterOptions::default(),
        )?;
        let key = ServiceKey::new("system-provider");
        let pulse = Event::symbol("system-pulse");
        let mut catalog = Catalog::new();
        catalog.register_service(&key)?;
        catalog.register_json(
            "provider",
            provider(&key, &probe, catalog_capture.clone(), "v1", None),
            decode,
        )?;
        catalog.register_json(
            "consumer",
            consumer(&key, &pulse, &probe, catalog_capture.clone()),
            decode,
        )?;
        catalog.register_json("sibling", sibling(&probe, catalog_capture), decode)?;
        let path = directory.join("cordis.json");
        std::fs::write(&path, serde_json::to_vec_pretty(&initial_source())?)?;
        let include = Include::new(&context, catalog, IncludeOptions::new(path))?;
        let report = include.refresh()?.await?;
        check_report(&report)?;
        let ticks = probe.clone();
        context.interval(Duration::from_millis(10), move || {
            ticks.ticks.set(ticks.ticks.get() + 1)
        })?;
        context.timeout(Duration::from_secs(3600), move || drop(timer_capture))?;
        let stream = context.interval_stream(Duration::from_millis(10))?;
        let mut options = WatchOptions::new(directory);
        options.poll_interval = Duration::from_millis(5);
        options.debounce = Duration::from_millis(5);
        options.max_watches = 4;
        options.queue_capacity = 4;
        options.event_capacity = 16;
        let (watcher, input) = if real_watch {
            (FileWatcher::new(&context, options)?, None)
        } else {
            let (watcher, input) = FileWatcher::synthetic(&context, options)?;
            (watcher, Some(input))
        };
        let config = watcher.watch_include("cordis.json", include.clone())?;
        std::fs::write(directory.join("audit.txt"), b"offline audit")?;
        let audit = watcher.watch("audit.txt", move |_| {
            let capture = callback_capture.clone();
            Box::pin(async move {
                drop(capture);
                Ok(())
            })
        })?;
        let plan = ReloadPlan::new(vec![
            ReloadNode {
                name: "provider-code".into(),
                factory: Some("provider".into()),
                dependencies: vec![],
            },
            ReloadNode {
                name: "consumer-code".into(),
                factory: Some("consumer".into()),
                dependencies: vec!["provider-code".into()],
            },
        ])?;
        context
            .logger(Some("system"))
            .info("booted provider, consumer, timer, groups and Include")?;
        Ok(Self {
            owner,
            include,
            watcher,
            config,
            audit,
            stream,
            catalog_capture: probe.catalog.clone(),
            probe,
            key,
            pulse,
            input,
            plan,
        })
    }
    pub fn entry(&self, id: &str) -> EntrySnapshot {
        self.include
            .entries()
            .into_iter()
            .find(|entry| entry.id.as_str() == id)
            .expect("application entry")
    }
    /// Hold the next injecting consumer cleanup at an explicit channel barrier.
    pub fn hold_consumer(&self) -> (oneshot::Receiver<()>, oneshot::Sender<()>) {
        let (entered, wait) = oneshot::channel();
        let (release, gate) = oneshot::channel();
        assert!(self.probe.gate.borrow().is_none());
        *self.probe.gate.borrow_mut() = Some((entered, gate));
        (wait, release)
    }
    pub fn emit_config(&self) -> Result<(), BoxError> {
        self.input
            .as_ref()
            .ok_or("synthetic ingress unavailable for real watcher")?
            .emit(Change {
                path: self.config.path().into(),
                kind: ChangeKind::Modified,
            })?;
        Ok(())
    }
    pub async fn fire(&self) {
        drive().await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        drive().await;
    }
    /// Already-linked candidates, with a controlled apply failure used by the demo.
    pub fn replace(
        &self,
        revision: &'static str,
        failure: Option<u32>,
    ) -> Result<IncludeTask<Report>, BoxError> {
        let capture = self
            .catalog_capture
            .upgrade()
            .ok_or("application catalog released")?;
        Ok(self.plan.replace_include(
            &self.include,
            &["provider-code".into()],
            vec![
                FactoryReplacement::new(
                    "provider",
                    provider(&self.key, &self.probe, capture.clone(), revision, failure),
                ),
                FactoryReplacement::new(
                    "consumer",
                    consumer(&self.key, &self.pulse, &self.probe, capture),
                ),
            ],
        )?)
    }
    /// Request cancellation once, then observe every subsystem's joined result.
    pub async fn shutdown(self) -> Result<(), BoxError> {
        let watch = self.watcher.dispose()?;
        let include = self.include.dispose()?;
        let owner = self.owner.dispose()?;
        let owner = owner.await;
        let watch = watch.await;
        let include = include.await;
        let final_error = self.include.final_error();
        let watch_failure = self
            .watcher
            .take_events()
            .into_iter()
            .find_map(|event| match event {
                re_cordis_hmr::WatchEvent::Failure(error) => Some(error),
                _ => None,
            });
        owner?;
        watch?;
        include?;
        if let Some(error) = final_error {
            return Err(Box::new(error));
        }
        if let Some(error) = watch_failure {
            return Err(Box::new(error));
        }
        assert_eq!(self.include.pending(), 0);
        let stats = self.watcher.stats();
        assert_eq!(stats.watches, 0);
        assert_eq!(stats.callbacks_running, 0);
        assert!(!stats.producer_running);
        assert_eq!(self.probe.live_providers.get(), 0);
        assert_eq!(self.probe.live_consumers.get(), 0);
        Ok(())
    }
}
fn provider(
    key: &ServiceKey<Provider>,
    probe: &Rc<Probe>,
    capture: Rc<()>,
    revision: &'static str,
    failure: Option<u32>,
) -> Plugin<u32> {
    let key = key.clone();
    let probe = probe.clone();
    Plugin::new(revision, move |ctx, config: u32| {
        let _capture = capture.clone();
        let key = key.clone();
        let probe = probe.clone();
        async move {
            if let Some(code) = failure {
                return Err::<(), BoxError>(Box::new(OriginalError(code)));
            }
            let serial = probe.next_provider.get() + 1;
            probe.next_provider.set(serial);
            let alive = Rc::new(Cell::new(true));
            ctx.provide(
                &key,
                Provider {
                    serial,
                    alive: alive.clone(),
                    revision,
                    config,
                },
            )?;
            let value = ctx.get(&key)?.ok_or("provider publication missing")?;
            probe.providers.borrow_mut().push(Rc::downgrade(&value));
            probe.live_providers.set(probe.live_providers.get() + 1);
            ctx.on_dispose_sync(move || {
                alive.set(false);
                probe.live_providers.set(probe.live_providers.get() - 1);
                Ok::<_, Infallible>(())
            })?;
            Ok::<_, BoxError>(())
        }
    })
}
fn consumer(
    key: &ServiceKey<Provider>,
    pulse: &Event<()>,
    probe: &Rc<Probe>,
    capture: Rc<()>,
) -> Plugin<u32> {
    let key = key.clone();
    let pulse = pulse.clone();
    let probe = probe.clone();
    let dependency = key.required();
    Plugin::new("consumer", move |ctx, _: u32| {
        let _capture = capture.clone();
        let key = key.clone();
        let pulse = pulse.clone();
        let probe = probe.clone();
        async move {
            let current = ctx.get(&key)?.ok_or("injected provider missing")?;
            probe.consumers.borrow_mut().push(current.serial);
            probe.live_consumers.set(probe.live_consumers.get() + 1);
            let pulses = probe.clone();
            ctx.on(
                &pulse,
                Listener::sync(move |_, ()| {
                    pulses.pulses.set(pulses.pulses.get() + 1);
                    Ok::<_, Infallible>(ControlFlow::Continue(()))
                }),
            )?;
            ctx.logger(Some("system"))
                .info(format!("consumer uses {}", current.call()?))?;
            let expected = Rc::downgrade(&current);
            ctx.clone().on_dispose(move || async move {
                let before = ctx.get(&key)?.ok_or("retained provider missing")?;
                let first = before.call()?;
                let gate = probe.gate.borrow_mut().take();
                if let Some((entered, gate)) = gate {
                    entered.send(()).map_err(|_| "cleanup observer dropped")?;
                    gate.await?;
                    let after = ctx
                        .get(&key)?
                        .ok_or("retained provider missing after gate")?;
                    let exact = Rc::ptr_eq(&before, &after)
                        && expected
                            .upgrade()
                            .is_some_and(|value| Rc::ptr_eq(&value, &after));
                    let last = after.call()?;
                    probe
                        .retained
                        .borrow_mut()
                        .push((before.serial, exact, first, last));
                }
                probe.live_consumers.set(probe.live_consumers.get() - 1);
                if probe.fail_cleanup.replace(false) {
                    return Err::<(), BoxError>(Box::new(OriginalError(301)));
                }
                Ok::<_, BoxError>(())
            })?;
            Ok::<_, BoxError>(())
        }
    })
    .injecting([dependency])
}
fn sibling(probe: &Rc<Probe>, capture: Rc<()>) -> Plugin<u32> {
    let probe = probe.clone();
    Plugin::new("sibling", move |ctx, config: u32| {
        let _capture = capture.clone();
        let probe = probe.clone();
        async move {
            probe.siblings.borrow_mut().push(config);
            ctx.on_update::<u32, _, _, BoxError>(|_, value, next| async move {
                if *value == 99 {
                    return Err(Box::new(OriginalError(202)) as BoxError);
                }
                Ok(next.call().await?)
            })?;
            Ok::<_, BoxError>(())
        }
    })
}
pub async fn drive() {
    for _ in 0..32 {
        tokio::task::yield_now().await;
    }
}
fn check_report(report: &Report) -> Result<(), BoxError> {
    if let Some(error) = report.failures.first() {
        return Err(Box::new(error.clone()));
    }
    if !report.conflicts.is_empty() {
        return Err("unexpected source conflict".into());
    }
    Ok(())
}

/// One-shot offline walkthrough; --watch enables bounded real polling ingress.
pub async fn demonstration(real_watch: bool) -> Result<(), BoxError> {
    let directory = Directory::new()?;
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let work = async {
                let app = Application::boot(&root, &directory.0, io::stdout(), real_watch).await?;
                app.include
                    .edit("provider", JsonEdit::new().set("config", json!(2)))?
                    .await?;
                app.include
                    .move_entry("provider", Some("right"), 0)?
                    .await?;
                app.include
                    .move_entry("consumer", Some("right"), 1)?
                    .await?;
                check_report(&app.replace("v2", None)?.await?)?;
                let failed = app.replace("controlled-failure", Some(201))?.await?;
                assert_eq!(
                    has_cause::<OriginalError>(&failed.failures[0]).unwrap().0,
                    201
                );
                check_report(&app.replace("recovered-v3", None)?.await?)?;
                app.include.flush()?.await?;
                // Join any real refresh before checking its retained outcome.
                // It may observe the same controlled load failure during recovery.
                let watch_join = app.watcher.dispose()?.await;
                let history = app.watcher.take_events();
                watch_join?;
                for event in history {
                    if let re_cordis_hmr::WatchEvent::Failure(error) = event {
                        let expected = has_cause::<OriginalError>(&failed.failures[0]).unwrap();
                        let observed = has_cause::<OriginalError>(&error);
                        if !observed.is_some_and(|cause| std::ptr::eq(expected, cause)) {
                            return Err(Box::new(error) as BoxError);
                        }
                    }
                }
                root.logger(Some("system")).info(
                    "moved provider, replaced linked factory, recovered original load failure",
                )?;
                app.shutdown().await
            }
            .await;
            let cleanup = root.drain()?.await?;
            let diagnostics = runtime.take_failures();
            for failure in cleanup.failures() {
                eprintln!("shutdown cleanup: {failure}");
            }
            if work.is_err() {
                for failure in &diagnostics {
                    eprintln!("shutdown diagnostic: {failure}");
                }
            }
            work?;
            assert!(cleanup.failures().is_empty());
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(has_cause::<OriginalError>(&diagnostics[0]).unwrap().0, 201);
            assert!(root.registry().is_empty()?);
            assert!(root.get_effects()?.is_empty());
            println!(
                "Joined shutdown: journal persisted; providers, timers, watches and writer released"
            );
            Ok::<_, BoxError>(())
        })
        .await??;
    Ok(())
}
