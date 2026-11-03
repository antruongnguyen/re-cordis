# HMR H3 Include adapter implementation proposal

Date: 2026-11-03. Status: design/test proposal; implementation ownership pending.
Baseline: 47042d9; approved H0: 5065adc; loader batch checkpoint: bbef48b.

## Reused contracts and uncovered delta

Reuse accepted Include I4, loader L5, timer T0–T5 and the existing core hosted
callback/task bridge. H0 review HR0-3/HR0-4 requires retained admitted Include work
and bounded retirement after callback cancellation. H1 owns ordinary callbacks,
path polling, dispatch, timer cancellation and producer join. H2 owns
Include::replace_factories and ReloadPlan::replace_include. No core change.

The H3 adapter is explicit and optional:

```rust
#[cfg(feature = "include")]
pub fn watch_include(&self, path: impl AsRef<Path>, include: Include)
    -> Result<WatchToken, WatchError>;
```

It watches one supplied path and refreshes the supplied Include. It never discovers
an Include source graph or disposes the supplied Include. Parse/read/lifecycle
errors and each Report.failures cause are retained in bounded WatchEvent history.

## Minimal retained-work design

Add only cfg(include) private registration state. Hosted callback creation admits
Include::refresh synchronously and immediately retains the returned IncludeTask
as a shared local future. The callback awaits a clone; cancellation drops that
clone without dropping the retained barrier. One refresh runs per registration;
pending changes collapse into one follow-up. Stop/drop clears pending state.

The existing bounded local receiver additionally drives retained completions,
including inactive registrations whose callback has been aborted. Completion is
observed once, original Include causes are emitted outside framework borrows,
and an inactive entry is reclaimed only after callback destruction and barrier
observation. Active plus retiring entries continue to count against max_watches.
No additional task/effect is created per refresh and no local handle crosses the
producer thread boundary.

Watcher cleanup stops dispatch first and snapshots retained tasks. It awaits those
same admitted tasks and observes their results before completing; it never calls
Include::settle/refresh after owner admission has closed. Refused self/ancestor
joins remain delegated to existing core disposal before stopping watcher state.

## Test-first steps

1. Synthetic valid refresh, invalid JSON followed by recovery, and Include-generated
   rename self-write convergence: assert same managed entry/config, bounded errors,
   file bytes and no second lifecycle application for unchanged accepted data.
2. Gate old plugin cleanup after a watched refresh. Stop/drop token, queue another
   change, and dispose watcher or common ancestor. Assert joined disposal stays
   pending until actual resource release, original failed cleanup remains inspectable
   after callback abort, and queued follow-up is never admitted.
3. Fill max_watches with stopped registrations whose Include barriers remain held.
   Assert registration rejects until barrier completion is observed and then quota
   recovers; repeated cancellation/late completion cannot grow resident work.
4. Real polling supplement: immediate write, atomic rename, remove/recreate and
   invalid-file recovery under bounded deadline. Always join watcher and Include.
5. Offline example watches/refreshes the same Include-managed instance, replaces
   a linked compiled factory through ReloadPlan::replace_include, reports a source
   RestartRequired event, and joins watcher plus Include shutdown.

Tests: crates/re-cordis-hmr/tests/include_integration.rs. Example:
crates/re-cordis-hmr/examples/reload.rs; manifest required-features=["include"]
and serde_json only as a dev dependency. H1 watch.rs ownership must be handed off
before tracked adapter/manifest/example/test edits.

## Verification boundary

Targeted HMR all-feature tests, strict all-target Clippy, Rustdoc and runnable
example on explicit stable and 1.85.0 tools. Resolve Cargo/rustc/rustdoc/
cargo-clippy/clippy-driver with rustup and export matching RUSTUP_TOOLCHAIN,
RUSTC, RUSTDOC and leading toolchain PATH. Long logs under target/hmr-h3.
Parent owns workspace/M0/docs/parity matrix, paired guides and final integration.
macOS polling smoke does not establish Linux/Windows runtime acceptance.


## Paused resume checkpoint

User pause received at 2026-11-03 11:32 UTC (18:32 Asia/Ho_Chi_Minh).
No H3 adapter/source/manifest implementation was authorized or performed.
No H3 commit was made. No verification result is claimed.

Drafts exist only under ignored target/hmr-h3; the parent may preserve them:

- `/Users/I756434/Projects/private/cordis-rust/target/hmr-h3/include_integration.rs`
  is the optional test draft, with file-level cfg(feature="include") and
  futures_util::FutureExt. Nine tests cover synthetic invalid-byte recovery,
  burst/rename self-write convergence, token stop, token Drop, watcher disposal,
  ancestor disposal while old Include plugin cleanup is held, retained quota
  recovery, 100 full watcher lifecycle cycles with parent-effect and weak capture
  release checks, and bounded real polling/removal/recreation recovery.
- `/Users/I756434/Projects/private/cordis-rust/target/hmr-h3/reload.rs`
  is the runnable offline example draft. It watches a temporary Include file,
  updates its configuration, replaces compiled-v1 with compiled-v2 via
  ReloadPlan::replace_include on that same Include-managed entry, verifies later
  configuration refresh uses compiled-v2, reports source RestartRequired and
  joins watcher/Include shutdown before checking Registry restoration.

The integration draft was briefly created/restored under the crate after worker
freeze, then moved back to target at the parent's checkpoint request. Neither
`crates/re-cordis-hmr/tests/include_integration.rs` nor
`crates/re-cordis-hmr/examples/reload.rs` should exist at this pause. No watcher,
lib.rs or Cargo.toml was edited by this H3 worker. H1/H2 remain parent-owned.

### Remaining ordered work

1. Parent completes/commits the frozen H1/H2 checkpoint and explicitly hands
   watch.rs/lib.rs/Cargo.toml ownership to the H3 implementer.
2. Add serde_json as HMR dev-only dependency and example required-features=["include"].
   Restore the integration test draft. Run the expected missing watch_include
   API RED with explicit rustup tools; fix any unrelated draft compilation errors.
   No RED has been executed yet.
3. Implement cfg(include) retained IncludeTask state. Small implementation option:
   register the callback, then attach its weak Entry/State captures; immediately
   retain successful refresh admission. Receiver uses its existing Notify plus
   select_all over at most max_watches retained shared completions. Observe
   original failure causes once. RunningGuard only reclaims stopped entries when
   their retained completion is observed. Stop clears pending and aborts callback;
   watcher on_dispose snapshots and awaits the same completion without new
   Include admission. Keep local handles off the producer thread.
4. Restore the example draft into examples/reload.rs only after required-features
   manifest wiring. Run tests/example and fix real failures without weakening
   resource-release/error/no-follow-up assertions. OriginalFailure(17) is the
   cleanup-cause type asserted through Error::source in the held tests.
5. Targeted all-feature/default HMR check/tests, strict all-target Clippy, Rustdoc
   and cargo run --example reload --features include on stable and 1.85.0.
   Resolve all five tools with rustup and export matching RUSTUP_TOOLCHAIN,
   RUSTC, RUSTDOC and leading toolchain PATH. Logs target/hmr-h3.
6. Update this evidence with actual RED/GREEN commands/counts and hand off the
   frozen working result to parent for review/docs/workspace acceptance/commit.

Both stable and 1.85.0 Cargo/rustc/rustdoc/cargo-clippy/clippy-driver path probes
resolved to matching `/Users/I756434/.rustup/toolchains/<toolchain>-aarch64-apple-darwin/bin`.
These setup probes are not test acceptance. Draft tests and example have not
been compiled or executed. macOS runtime checks remain unexecuted for H3.
