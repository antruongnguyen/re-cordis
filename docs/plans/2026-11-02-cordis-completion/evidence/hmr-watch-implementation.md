# HMR H1 watcher implementation checkpoint

Date: 2026-11-03. Status: **implemented and verified; parent checkpoint review pending**.
Contract: [hmr-watch-contract.md](hmr-watch-contract.md).
This evidence covers H1 file watching. H2 belongs to its separate worker;
the optional H3 Include adapter remains unimplemented at this checkpoint.

## Delivered behavior

The new unpublished re-cordis-hmr package uses existing core and timer APIs
without core changes or a watcher/hash dependency. One owned standard-library
thread samples only registered paths with bounded streaming fingerprints.
A Send/Sync synthetic input exercises the same bounded queue, generation checks,
overflow rescan, trailing debounce and local callback dispatcher.

Each exact token suppresses queued/follow-up dispatch on stop or Drop.
Retiring callback futures count against the resident quota until destruction.
One callback per registration runs at a time; unrelated registrations progress.
Queue overflow marks every currently active path dirty. Bounded public history
preserves original callback/I/O causes while retained and exposes eviction counts.

First real registration/observation emits Rescan, including duplicate callbacks,
so an immediate post-registration write is refreshed. Synthetic mode is driven
only by explicit inputs. A fixed canonical existing base accepts native base
aliases (including macOS /var); symlink components below that base and explicit
symlink file/parent registrations remain refused. Missing parents, atomic
rename, remove/recreate and non-UTF8 Unix names keep sticky logical paths.
Sample failures/oversized files are observable and recover on later valid data.

The watcher owns a dedicated Fiber. Explicit disposal first uses core admission
and refuses self-join without stop side effects, then cancels receiver/callback
tasks, disposes debounce, closes producer input and wakes the producer.
The cleanup future joins its JoinHandle in tokio::task::spawn_blocking so the
local executor can continue. The owner lifecycle barrier joins all core task
destruction/retirement and producer completion. A host-owned capture Drop guard
also requests stop when Runtime is destroyed while the FileWatcher survives;
runtime Drop does not claim asynchronous joined shutdown.

## Executed tests and regression evidence

H1 contributes **27 integration tests + one compile-fail Rustdoc check**:

- 16 synthetic/lifecycle tests: duplicate debounce, exact sibling/generation
  cancellation, ignored observe/explicit override, bounded queue/history,
  restart-required batching, original errors and sync/async panic recovery,
  one running callback plus coalesced follow-up, resident quota, self-stop and
  self-join refusal, cross-thread input, ancestor disposal, host destruction,
  destructor reentry, concurrent input closure and 100 synthetic disposal cycles.
- Five native path/admission tests: normalized missing parents, path/input bounds,
  canonical base alias versus refused symlinks, Unix non-UTF8 and missing time
  driver rollback.
- Six real filesystem tests: immediate writes/initial duplicate callback,
  same-size rewrite with deliberately restored mtime after an observed producer
  warmup, atomic rename/remove/parent recreation, byte-limit recovery, symlink
  replacement recovery and 100 joined real-producer disposal cycles.

The first no-op implementation failed behaviorally (callback count 0 versus 1)
on explicit 1.85.0 before production dispatch. A parent draft review reproduced
two additional failures: user error destruction under RefCell borrow, and a
producer surviving runtime destruction. Both were repaired and green. An
unrepresentable poll_interval regression failed before checked Instant
validation was added. Logs are in ignored target/hmr-h1:

- first-red-1.85.0.log
- lifecycle-red-1.85.0.log
- poll-deadline-red-1.85.0.log
- synthetic-green-1.85.0.log
- full-green-1.85.0.log

## Final executed verification

All final package commands ran after exact source restoration and formatting:

| Check | Explicit stable 1.97.1 | Explicit 1.85.0 |
|---|---|---|
| cargo check -p re-cordis-hmr --all-features --all-targets --locked | Pass | Pass |
| cargo test -p re-cordis-hmr --all-features --locked | 43 integration + 1 Rustdoc pass | 43 integration + 1 Rustdoc pass |
| H1 subset of the above | 27 + 1 Rustdoc | 27 + 1 Rustdoc |
| H2 subset, independently owned | 16 integration | 16 integration |
| Strict all-feature/all-target Clippy (-D warnings) | Pass | Pass |
| No-deps all-feature Rustdoc (RUSTDOCFLAGS=-D warnings) | Pass | Pass |
| Stable rustfmt applied to six owned Rust files | Pass | Same source |
| git diff --check | Pass | Same source |

Cargo, rustc, rustdoc, cargo-clippy and clippy-driver were resolved independently
using rustup which --toolchain for each run. Their toolchain bin led PATH, with
RUSTUP_TOOLCHAIN/RUSTC/RUSTDOC explicit. No Homebrew tool counted. Logs:
target/hmr-h1/{toolchain,check,test,clippy,doc}-{stable,1.85.0}.log.
Separate build roots are target/hmr-h1/build-{stable,1.85.0}.

The 1.85.0 toolchain lacks its rustfmt component. A formatter staging mistake
temporarily wrote empty owned files; exact pre-format source buffers were restored
through VFS before any completion claim, then formatted with explicitly resolved
stable rustfmt 1.9.0 and verified with the full checks above. No shared/H2 files
were overwritten. Future formatter application prechecks executable presence,
successful exit and every output's nonempty content before any durable write.

## Scope and freeze boundary

Executed on aarch64 macOS. Linux/Windows runtime watcher evidence is absent;
Unix-only tests are gated appropriately. Polling can miss changes wholly between
samples; process-local digests have a collision caveat. Symlink checks are ordinary
filesystem behavior rather than atomic security confinement. Blocked OS I/O can
delay producer join; no hard wall-clock shutdown deadline is promised.

H3 must retain and join already-admitted IncludeTask completions outside abortable
callbacks and preserve all Report failures. No new Include admission may be
attempted first during disposal. H1 makes no native executable hot-swap promise.

The seven owned production/manifest/test file SHA256 values are recorded at
target/hmr-h1/freeze-sha256.txt. Parent may review/stage this H1 checkpoint;
no worker commit or push was made. Subsequent H3 changes supersede this freeze
and require their own integrated verification.
