//! Boot, Pending activation, config failure/recovery and joined shutdown.
use re_cordis_core::{BoxError, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, Group, Loader};
use std::convert::Infallible;
fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                let settings = ServiceKey::<u32>::new("settings");
                let worker = Plugin::validated("worker", |source: &i32| u32::try_from(*source), {
                    let settings = settings.clone();
                    move |ctx, value: u32| {
                        let settings = settings.clone();
                        async move {
                            if value == 13 {
                                return Err::<(), BoxError>(
                                    "deliberate application failure".into(),
                                );
                            }
                            println!("worker {value}, settings {}", ctx.get(&settings)?.unwrap());
                            ctx.on_dispose_sync(move || {
                                println!("closed worker {value}");
                                Ok::<_, Infallible>(())
                            })?;
                            Ok::<_, BoxError>(())
                        }
                    }
                })
                .injecting([settings.required()]);
                let mut catalog = Catalog::new();
                catalog.register("worker", worker)?;
                let loader = Loader::new(&root, catalog)?;
                loader.create(
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(1_i32)).id("worker"),
                    ]))
                    .id("application"),
                )?;
                println!(
                    "before provider: {:?}",
                    loader
                        .settle()?
                        .await?
                        .entries
                        .iter()
                        .map(|entry| entry.state)
                        .collect::<Vec<_>>()
                );
                root.provide(&settings, 7)?;
                let ready = loader.settle()?.await?;
                assert!(ready.failures.is_empty());
                let id = loader.resolve("application:worker")?;
                loader
                    .update(&id, EntrySpec::new("worker", Config::new(2_i32)))?
                    .await?;
                assert!(
                    loader
                        .update(&id, EntrySpec::new("worker", Config::new(-1_i32)))
                        .is_err()
                );
                assert_eq!(*loader.snapshot(&id).unwrap().source.get::<i32>()?, 2);
                assert!(
                    loader
                        .update(&id, EntrySpec::new("worker", Config::new(13_i32)))?
                        .await
                        .is_err()
                );
                loader
                    .update(&id, EntrySpec::new("worker", Config::new(14_i32)))?
                    .await?;
                loader.dispose()?.await?;
                root.drain()?.await?;
                assert!(root.registry().is_empty()?);
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}
