# File changes and linked replacement

[Tiếng Việt](../../vi/guides/hmr.md) | [English home](../README.md)

The experimental `re-cordis-hmr` crate coordinates explicit file watches and
replacement of already-linked Rust plugins, with optional Include integration.
H0–H3 is independently accepted with the full stable/1.85.0 shared matrix. See the [API](../api/hmr.md)
and [source disposition map](../../plans/2026-11-02-cordis-completion/evidence/hmr-source-map.md)
for the exact scope and evidence boundary.

Choose the operation that matches your input:

| Input | Operation | Outcome |
|---|---|---|
| Configuration file | `FileWatcher::watch` with your callback; optional `watch_include` adapter | Refresh the registered source and retain visible failures |
| New plugin implementation already compiled into the process | `ReloadPlan::replace` with `FactoryReplacement` candidates | Loader rebuilds selected entries after their old cleanup |
| Changed Rust source file | `FileWatcher::watch_source` | A `WatchEvent::RestartRequired` for your build/restart supervisor |

The watcher does not discover Rust modules, walk directory roots recursively,
compile code, reload a native binary or start a replacement process.
Declare every watched file and every code dependency you need.

## Watch one file and keep its token

Create the watcher under a live Cordis Context on its original local host, with
Tokio time enabled. `FileWatcher::new` owns a polling producer; `synthetic`
uses the same bounded queue, debounce and local dispatch but samples no files.
The example below is deterministic and emits a source notification explicitly.

```rust
use std::time::Duration;
use re_cordis_core::{BoxError, Runtime};
use re_cordis_hmr::{
    Change, ChangeKind, FileWatcher, ReloadNode, ReloadPlan, WatchEvent, WatchOptions,
};
use re_cordis_timer::TimerExt;

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let plan = ReloadPlan::new(vec![
                ReloadNode {
                    name: "shared".into(),
                    factory: None,
                    dependencies: vec![],
                },
                ReloadNode {
                    name: "worker-code".into(),
                    factory: Some("worker".into()),
                    dependencies: vec!["shared".into()],
                },
            ])?;
            assert_eq!(plan.affected(&["shared".into()])?, vec!["worker"]);

            let mut options = WatchOptions::new(std::env::current_dir()?);
            options.debounce = Duration::ZERO;
            let (watcher, events) = FileWatcher::synthetic(&root, options)?;
            let source = watcher.watch_source("src/main.rs")?;
            events.emit(Change {
                path: source.path().to_path_buf(),
                kind: ChangeKind::Modified,
            })?;
            tokio::time::timeout(Duration::from_secs(2), async {
                loop {
                    if watcher.take_events().iter().any(|event| {
                        matches!(event, WatchEvent::RestartRequired(_))
                    }) {
                        break Ok::<_, BoxError>(());
                    }
                    root.sleep(Duration::from_millis(1)).await?;
                }
            }).await??;

            source.stop();
            watcher.dispose()?.await?;
            root.drain()?.await?;
            Ok::<_, BoxError>(())
        }).await??;
        Ok::<_, BoxError>(())
    })
}
```

`watch(path, callback)` accepts a local `FnMut(Change)` returning a
`LocalFuture<Result<(), BoxError>>`. Capture owned data; no Send bound is
required. Each registration runs at most one callback future and coalesces
changes arriving during that future into one follow-up. Multiple tokens on
one path retain separate callbacks.

Keep the WatchToken for as long as the registration is needed. Its `stop()`
and Drop request cancellation; they do not synchronously join an active
callback. Await `watcher.dispose()` or the owner's lifecycle barrier to
observe callback destruction and producer completion. Calls that would drain
the watcher from its own callback are refused before stopping it.

## Paths, polling and failure recovery

Relative paths resolve against the fixed `WatchOptions::base`. The logical
file path survives atomic replacement and missing-file/parent recreation.
Renaming a parent does not relocate the registration. Existing symlinks are
refused; a sampled symlink, directory, read failure or oversized file produces
a visible failure and later regular-file recovery remains usable.

`watch`/`watch_source` explicitly override ignored components and may register
outside base. `observe`/`observe_source` filter the exact native component names
and return `Ok(None)` for ignored paths. The default ignored names are
`.git`, `target` and `node_modules`; there is no glob language or root scan.

Real registrations receive an initial Rescan, including a new callback on an
already-watched path. Polling and debounce are separate clocks. Polling compares
bounded file metadata and a streaming noncryptographic digest; it can miss
changes entirely between samples or digest collisions. Queue overflow requests
a rescan of the bounded registered set.

Drain `take_events()` to inspect Change, RestartRequired and Failure records.
History evicts its oldest entry at capacity; use `stats()` to observe overflows
and evictions. Failure keeps its original error source while retained.
`capabilities()` is a pure value snapshot and produces no repeated warnings.
Windows and Linux runtime acceptance remain pending; macOS smoke does not
establish their path or watcher behavior.

## Replace explicitly declared factories

`ReloadNode` names an application code unit, optionally maps it to a catalog
factory, and lists other declared units it uses. A dependency-only node has
`factory: None`. `affected(changed)` follows reverse dependencies; unchanged
shared dependencies do not select their other consumers.

Supply exactly one `FactoryReplacement::new(name, plugin)` per affected factory
to `plan.replace(&loader, changed, replacements)`. Await the returned
`LoaderTask<Settlement>` and inspect its per-entry failures. Graph construction,
candidate coverage, source types and optional raw-config preflight are checked
before any replacement teardown. Preflight must be pure; same-loader mutations
are refused during it. Plugin validation still runs on the actual
dependency-ready Context.

Each entry retains its own source config and exact old providers until cleanup
finishes. An unrelated fast entry can rebuild while a slow sibling is still
draining. Config edits, disable/remove and newer host intent during cleanup win
over stale replacement work. Nested core plugins rebuild through a supplied
host candidate; arbitrary nested declarations in an unchanged host are outside
this scope.

An apply failure after unload remains visible and needs an explicit update or
later candidate to recover. Completed cleanup and external effects cannot be
rolled back transactionally. A supplied candidate is already linked into the
running process; it is not a compilation of the changed source file.

## Optional Include integration

Enable the HMR crate's `include` feature and call
`watch_include(path, include.clone())`. This registers one explicit trigger path
and calls refresh on the supplied Include, including its source graph. Register
each file that should trigger refresh; the adapter does not discover paths.
Include remains independent of HMR.

For replacement of factories used by that same Include instance, call
`plan.replace_include(&include, changed, replacements)`. Await
its `IncludeTask<Report>`. It forwards the exact loader batch completion and
preserves Include journal, data, provenance and file bytes; it does not flush.
This path has the same explicit graph and candidate checks as `replace`.

The adapter retains the already-admitted IncludeTask outside its cancellable
callback. Stop suppresses follow-up refreshes; watcher/ancestor cleanup joins
that retained completion without making a new Include call after admission
closes. Unfinished retired registrations still consume the watch quota.
Returned Include errors and every `Report.failures` cause enter bounded
`WatchEvent::Failure` history once per completed refresh. Ordinary core cleanup
failures retain their existing `Runtime::take_failures` channel; inspect both.
Parse failures preserve the last valid source and allow later valid refreshes.
Rename-based Include self-writes converge without reapplying unchanged accepted
data. See [Include](include.md) for persistence and durability limits.

## Offline example

Run the [complete reload example](../../../crates/re-cordis-hmr/examples/reload.rs)
from the repository root:

```sh
cargo run -p re-cordis-hmr --example reload --features include --locked --offline
```

It creates its own temporary files, watches configuration with one Include,
replaces compiled-v1 with compiled-v2 while preserving EntryId and current
config, and verifies later config refresh uses compiled-v2. It prints the
capability snapshot once, reports a source `RestartRequired`, then joins watcher
and Include shutdown and checks Registry restoration. No network or external
build process is needed.
