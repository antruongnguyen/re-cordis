//! Run with: cargo run -p re-cordis-logger --example console --locked
use re_cordis_core::{BoxError, ExporterOptions, Plugin, Runtime};
use re_cordis_logger::{ColorMode, ConsoleOptions, Timestamp, register_stdout};

fn options() -> ConsoleOptions {
    ConsoleOptions {
        colors: ColorMode::Plain,
        timestamp: Timestamp::Off,
        ..Default::default()
    }
}

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                let plugin = Plugin::new("console", |ctx, ()| async move {
                    register_stdout(&ctx, options(), ExporterOptions::default())?;
                    ctx.logger(Some("app")).info("plugin ready")?;
                    Ok::<_, BoxError>(())
                });
                let fiber = root.plugin(&plugin, ())?;
                fiber.ready()?.await?;
                root.logger(Some("app")).warn("owned output")?;
                fiber.dispose()?.await?;
                let output = register_stdout(&root, options(), ExporterOptions::default())?;
                root.logger(Some("app")).info("manual output")?;
                output.remove()?;
                root.logger(Some("app")).info("buffer only")?;
                let report = root.drain()?.await?;
                assert!(report.failures().is_empty());
                assert!(root.get_effects()?.is_empty());
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}
