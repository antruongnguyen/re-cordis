# HMR H3 Include adapter implementation checkpoint

Date: 2026-11-04. Status: **implemented and focused verification passed; independent H3 review and shared acceptance pending**.
Implementation baseline: `e44754e2f69de19c088a35ab39fa576877a35f39`.
H2 code gate: `0e8d306b25b54600f15aac79514d45abf5431497`, independently accepted.
Reuse approved H0 [watch contract](hmr-watch-contract.md), [H0 review](hmr-h0-review.md), accepted Include I4, loader L5/H2 and timer/core task contracts.
No core, loader or Include production code changed in this slice.

## Delivered adapter and resource boundary

The optional `FileWatcher::watch_include(path, Include)` registers one explicit
file and refreshes that same Include. Include remains independent of HMR; no
source graph is discovered and the adapter does not dispose its supplied Include.

Hosted callback creation admits `Include::refresh` synchronously and immediately
stores its transformed shared local completion on the registration. The ordinary
abortable callback awaits a clone. The existing local receiver selects input
notification or completion among at most `max_watches` retained registrations.
No extra task/effect is created per refresh and no local handle crosses to the
producer thread.

The transformed completion retains original returned Include errors, every
`Report.failures` IncludeError, and contained asynchronous panic messages.
Exactly one observer takes the retained completion; error publication and
destruction occur outside mutable framework borrows. Successful callbacks return
unit without separately reporting the same failures.

One refresh runs per registration, with one coalesced follow-up. Stop/drop clears
pending state and aborts its callback clone while keeping admitted Include work.
Active plus retiring registrations count against `max_watches`. Reclamation
requires completion observation and destruction of the running callback.
Watcher/ancestor cleanup snapshots and awaits these same completions, then joins
the owned producer; it makes no new Include refresh/settlement admission after
shutdown has started.

The offline `reload` example watches a temporary Include file, refreshes config
1 to 2, replaces compiled-v1 with compiled-v2 through `ReloadPlan::replace_include`
on the same managed EntryId, then refreshes config 3 through compiled-v2.
A source watch reports RestartRequired. Watcher and Include teardown are joined
and Registry restoration is asserted. Its `serde_json = "1"` dependency is
dev-only; `required-features = ["include"]` gates the example.

## Test-first evidence and corrected draft oracle

Logs are under ignored `target/hmr-h3-resume/`.

1. Restored saved tests failed for the missing `watch_include` API on explicit
   1.85.0: `missing-api-red-1.85.0.log`.
2. The straightforward ordinary-callback adapter failed the held cleanup test:
   watcher disposal returned while Include's actual old-resource release was
   still gated. `barrier-red-1.85.0.log` preserves the behavioral RED.
3. Retained completion repaired joined cleanup and quota, but the saved fixture
   expected an ordinary core cleanup error in watcher history. A direct Include
   reproduction returned `Report { changed: true, failures: [], conflicts: [] }`;
   `Runtime::take_failures()` retained `CleanupFailure -> OriginalFailure(17)`.
   See `direct-include-error-red-1.85.0.log` and the saved probe excerpt
   `direct-include-error-probe.rs.txt` in that log directory.
4. Parent ruled that this was a draft oracle mismatch with the accepted error
   channels, not a missing production guarantee. The four held cases retain the
   OriginalFailure(17) cleanup and assert exactly one original cause through
   Runtime diagnostics. Their source now also contains two real, nonfatal
   Include patch errors. After callback abort/join, watcher history must contain
   both original typed IncludeErrors, each once, with exact messages, source
   file, patches indices 0/1 and spans. All gated release, pending barrier, quota,
   no-follow-up and no-spurious-disposed-error assertions remain.
   The existing parser recovery test still checks
   `WatchFailure -> IncludeError -> serde_json::Error`.
5. The direct error-channel control test passes on stable and 1.85.0. No
   dependency implementation was altered to satisfy the mistaken draft oracle.

Historical proposal/resume prose called the saved draft nine tests; the preserved
`hmr-resume/include_integration.rs.txt` actually declares eight. That saved
handoff is unchanged. Delivery contains ten: the eight corrected/restored cases,
one successful held-refresh/single-follow-up case, and one direct Include
error-channel control.

The ten H3 tests cover:

- Synthetic invalid bytes followed by recovery, 100-event bursts, atomic rename
  and Include-generated self-write convergence on the same EntryId.
- Token stop, token Drop, watcher disposal and ancestor disposal while old plugin
  cleanup is gated; actual release, both original error channels, retained quota
  and suppression of queued follow-up.
- Two stopped registrations exhausting quota until their completions are observed.
- A successful held refresh followed by exactly one coalesced refresh; configs
  apply as [1, 2, 3] after old-resource release.
- 100 watcher create/use/dispose cycles restoring effects and Registry, plus weak
  callback/catalog capture release.
- Bounded real polling immediate write, atomic rename, invalid file/removal
  recovery, self-write convergence and joined producer/Include cleanup.
- Direct Include cleanup-error channel control.

## Authorized H1 filesystem oracle correction

An initial default HMR run failed
`atomic_rename_remove_and_parent_recreate_keep_logical_watch`:
a queued older callback could observe deleted bytes and set `latest = None`
before the Removed event was dispatched. The old test treated missing bytes
alone as proof that Removed had arrived. The focused rerun passed; the original
failure remains in `default-test-1.85.0.log`.

Parent authorized only this test's bounded wait to require both missing bytes
and an observed Removed event. Existing event, sticky path and joined-resource
assertions remain. No production behavior, sleeps or timeout was changed.
Ten focused repeats per toolchain pass in
`h1-oracle-repeat-{stable,1.85.0}.log`, followed by the normal final suites.
This adds `tests/filesystem.rs` to the H3 review/freeze scope.

## Final executed focused verification

Every Cargo/rustc/rustdoc/cargo-clippy/clippy-driver executable was resolved with
`rustup which --toolchain stable|1.85.0`. Matching RUSTUP_TOOLCHAIN, RUSTC,
RUSTDOC and leading toolchain-bin PATH were explicit. Tool paths and versions
are in `tools-{stable,1.85.0}.json`; commands are in ignored
`run_focused.py` and `matrix-{stable,1.85.0}.json`.

| Check | Stable 1.97.1 | Rust 1.85.0 |
|---|---|---|
| HMR all-feature/all-target check, locked | Pass | Pass |
| HMR default all-target check, locked | Pass | Pass |
| HMR all-feature tests, locked | 54 integration + 1 Rustdoc | 54 integration + 1 Rustdoc |
| HMR default tests, locked | 43 integration + 1 Rustdoc | 43 integration + 1 Rustdoc |
| H3 subset including direct channel control | 10 pass | 10 pass |
| Strict all-feature/all-target Clippy | Pass | Pass |
| Strict default/all-target Clippy | Pass | Pass |
| Strict no-deps all-feature Rustdoc | Pass | Pass |
| Strict no-deps default Rustdoc | Pass | Pass |
| Runnable offline reload example | Pass | Pass |
| Corrected H1 oracle repetitions | 10/10 | 10/10 |
| Owned five Rust files, explicit stable rustfmt check | Pass | Same source |
| git diff --check | Pass | Same source |

Final log names: `{check-all,check-default,test-all,test-default,clippy-all,
clippy-default,doc-all,doc-default,example}-{stable,1.85.0}.log`.
Draft example type and crate-cfg lint-order setup failures are preserved as
`first-matrix-*` logs; corrected final commands pass.
Build roots are `target/hmr-h3-resume/build-{stable,1.85.0}`.

Full workspace `cargo fmt --all --check` flagged only the inherited H2
`crates/re-cordis-loader/src/lib.rs:2054` formatting delta. This worker left
that file untouched for parent coordination. Log:
`workspace-format-check-stable.log`. This is not a workspace-format pass.

Seven executable source/test/manifest SHA256 hashes are recorded in
`target/hmr-h3-resume/source-freeze.json` at the implementation baseline above.
No Git index operation, worker commit, push, publishing or additional agent was
performed. Parent owns shared docs, declaration accounting, shared acceptance,
independent review and commits.

## Evidence boundaries

Executed on aarch64 macOS. Linux/Windows runtime watcher acceptance remains
absent. Polling may miss changes between samples, process-local fingerprints
have their documented collision caveat, symlink checks are ordinary filesystem
behavior, and blocked OS I/O may delay joined shutdown. Source changes report
RestartRequired; no supervisor, Node cache/eval or native binary loading is
provided. Focused H3 verification does not claim the workspace/M0/docs/parity
matrix or independent acceptance.
