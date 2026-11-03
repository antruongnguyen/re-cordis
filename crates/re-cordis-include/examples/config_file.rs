//! Offline file read, JSON edit, flush and joined shutdown.
use re_cordis_core::{BoxError, Plugin, Runtime};
use re_cordis_include::{Include, IncludeOptions, JsonEdit};
use re_cordis_loader::Catalog;
use serde_json::json;
use std::{
    convert::Infallible,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() -> Result<(), BoxError> {
    let directory = std::env::temp_dir().join(format!(
        "re-cordis-include-example-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    ));
    std::fs::create_dir(&directory)?;
    let result = (|| -> Result<(), BoxError> {
        let path = directory.join("cordis.json");
        std::fs::write(
            &path,
            serde_json::to_vec_pretty(&json!([
                {"id":"worker","name":"worker","config":1}
            ]))?,
        )?;
        let executor = tokio::runtime::Builder::new_current_thread().build()?;
        executor.block_on(async {
            let runtime = Runtime::new();
            runtime
                .run(async {
                    let root = runtime.context();
                    let mut catalog = Catalog::new();
                    catalog.register_json(
                        "worker",
                        Plugin::new("worker", |_, value: u32| async move {
                            println!("worker loaded with config {value}");
                            Ok::<_, Infallible>(())
                        }),
                        |value| serde_json::from_value::<u32>(value.cloned().unwrap_or(json!(0))),
                    )?;
                    let include = Include::new(&root, catalog, IncludeOptions::new(path.clone()))?;
                    let read = include.refresh()?.await?;
                    assert!(read.failures.is_empty());
                    let changed = include
                        .edit("worker", JsonEdit::new().set("config", json!(2)))?
                        .await?;
                    assert!(changed.failures.is_empty() && changed.conflicts.is_empty());
                    let flush = include.flush()?.await?;
                    assert!(flush.failures.is_empty() && flush.conflicts.is_empty());
                    let written: serde_json::Value =
                        serde_json::from_slice(&std::fs::read(&path)?)?;
                    assert_eq!(written[0]["config"], 2);
                    include.dispose()?.await?;
                    assert!(root.registry().is_empty()?);
                    assert!(root.drain()?.await?.failures().is_empty());
                    assert!(runtime.take_failures().is_empty());
                    println!("config 2 persisted; joined shutdown restored the Registry");
                    Ok::<_, BoxError>(())
                })
                .await??;
            Ok::<_, BoxError>(())
        })
    })();
    let cleanup = std::fs::remove_dir_all(&directory);
    result?;
    cleanup?;
    Ok(())
}
