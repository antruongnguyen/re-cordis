mod app;
use re_cordis_core::{BoxError, Runtime};

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let runtime = Runtime::new();
    let outcome = executor.block_on(runtime.run(app::run(&runtime.context(), std::io::stdout())));
    // Joined lifecycle tasks can succeed while user cleanup failed. Inspect the
    // Runtime channel as well as root CleanupReport failures checked by app::run.
    let failures = runtime.take_failures();
    for failure in &failures {
        eprintln!("cleanup error: {failure}");
    }
    outcome??;
    if !failures.is_empty() {
        return Err("application cleanup failed; see diagnostics".into());
    }
    println!("shutdown complete");
    Ok(())
}
