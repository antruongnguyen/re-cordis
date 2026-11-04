//! Offline configuration watching, compiled replacement and source restart signal.
use re_cordis_core::{BoxError, Plugin, Runtime};
use re_cordis_hmr::{
    FactoryReplacement, FileWatcher, ReloadNode, ReloadPlan, WatchEvent, WatchOptions,
};
use re_cordis_include::{Include, IncludeOptions};
use re_cordis_loader::Catalog;
use serde_json::json;
use std::{
    cell::RefCell,
    convert::Infallible,
    rc::Rc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

async fn wait_until(mut ready: impl FnMut() -> bool) -> Result<(), BoxError> {
    tokio::time::timeout(Duration::from_secs(5), async {
        while !ready() {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await?;
    Ok(())
}
fn worker(revision: &'static str, values: Rc<RefCell<Vec<(&'static str, u32)>>>) -> Plugin<u32> {
    Plugin::new(revision, move |_, config: u32| {
        values.borrow_mut().push((revision, config));
        println!("{revision}: configuration {config}");
        async { Ok::<_, Infallible>(()) }
    })
}
fn main() -> Result<(), BoxError> {
    let directory = std::env::temp_dir().join(format!(
        "re-cordis-hmr-example-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    ));
    std::fs::create_dir(&directory)?;
    let result = (|| -> Result<(), BoxError> {
        let path = directory.join("cordis.json");
        std::fs::write(&path, br#"[{"id":"worker","name":"worker","config":1}]"#)?;
        tokio::runtime::Builder::new_current_thread().enable_all().build()?.block_on(async {
            let runtime = Runtime::new();
            runtime.run(async {
                let root = runtime.context(); let values = Rc::new(RefCell::new(Vec::new()));
                let mut catalog = Catalog::new();
                catalog.register_json("worker", worker("compiled-v1", values.clone()), |value| serde_json::from_value::<u32>(value.cloned().unwrap_or(json!(0))))?;
                let include = Include::new(&root, catalog, IncludeOptions::new(path.clone()))?;
                let mut options = WatchOptions::new(&directory); options.poll_interval = Duration::from_millis(5); options.debounce = Duration::from_millis(5);
                let watcher = FileWatcher::new(&root, options)?;
                let _configuration = watcher.watch_include("cordis.json", include.clone())?;
                println!("Capabilities: {:?}", watcher.capabilities());
                let scenario = async {
                    wait_until(|| values.borrow().last() == Some(&("compiled-v1", 1))).await?;
                    let entry = include.entries()[0].id.clone();
                    std::fs::write(&path, br#"[{"id":"worker","name":"worker","config":2}]"#)?;
                    wait_until(|| values.borrow().last() == Some(&("compiled-v1", 2))).await?;
                    let plan = ReloadPlan::new(vec![ReloadNode { name: "worker-code".into(), factory: Some("worker".into()), dependencies: Vec::new() }])?;
                    let report = plan.replace_include(&include, &["worker-code".into()], vec![FactoryReplacement::new("worker", worker("compiled-v2", values.clone()))])?.await?;
                    assert!(report.failures.is_empty()); assert_eq!(include.entries()[0].id, entry);
                    assert_eq!(values.borrow().last(), Some(&("compiled-v2", 2)));
                    // Configuration fallback continues with the replacement already linked.
                    std::fs::write(&path, br#"[{"id":"worker","name":"worker","config":3}]"#)?;
                    wait_until(|| values.borrow().last() == Some(&("compiled-v2", 3))).await?;
                    assert_eq!(include.entries()[0].id, entry);
                    let _source = watcher.watch_source("worker.rs")?;
                    std::fs::write(directory.join("worker.rs"), b"// changed source requires an external rebuild/restart\n")?;
                    let mut restart = false;
                    wait_until(|| {
                        restart |= watcher.take_events().iter().any(|event| matches!(event, WatchEvent::RestartRequired(change) if change.path.ends_with("worker.rs")));
                        restart
                    }).await?;
                    println!("worker.rs: restart required; configuration watching remains available");
                    Ok::<_, BoxError>(())
                }.await;
                let watch_join = watcher.dispose()?.await; let include_join = include.dispose()?.await;
                scenario?; watch_join?; include_join?;
                assert!(!watcher.stats().producer_running); assert!(root.registry().is_empty()?);
                assert!(root.drain()?.await?.failures().is_empty());
                println!("Watcher and Include shutdown joined; Registry restored");
                Ok::<_, BoxError>(())
            }).await??; Ok::<_, BoxError>(())
        })
    })();
    let cleanup = std::fs::remove_dir_all(&directory);
    result?;
    cleanup?;
    Ok(())
}
