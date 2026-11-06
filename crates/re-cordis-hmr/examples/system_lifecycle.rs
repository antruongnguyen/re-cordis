//! Run: cargo run -p re-cordis-hmr --features include --example system_lifecycle --locked --offline
//! Add -- --watch for the same one-shot walkthrough with bounded filesystem polling.
#[path = "system_lifecycle/app.rs"]
#[allow(dead_code)]
mod app;

fn main() -> Result<(), re_cordis_core::BoxError> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if !arguments.is_empty() && arguments != ["--watch"] {
        return Err("usage: system_lifecycle [--watch]".into());
    }
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(app::demonstration(!arguments.is_empty()))
}
