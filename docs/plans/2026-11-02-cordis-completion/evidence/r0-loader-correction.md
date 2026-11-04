# R0 loader canceled-worker snapshot correction

Date: 2026-11-04. Scope: one loader owner-cleanup bookkeeping line and one
public-API integration regression. This correction is part of R0; accepted core
hosting and all production APIs remain unchanged.

## Concrete integration failure and cause

The R0 application changes a provider while an injecting consumer cleanup is
held, then shuts down its ancestor with an admitted Include/HMR refresh and a
journal. Ancestor and watcher joins restore Registry/effect baselines and
release providers, timer callbacks, watch callbacks and writers. Nevertheless,
the retained structural group snapshot reports `Unloading` after completion
with both core allocation IDs absent.

A reduced native Loader reproduction creates a group with one worker, changes
the group's isolation while the old worker cleanup is held, then disposes and
joins the ancestor. Both stable 1.97.1 and Rust 1.85.0 reproduce
`Unloading != Disposed`. Original RED logs and standalone source/manifest are
under `target/r0-implementation/group-repro/` and
`group-repro-{stable,1.85.0}-red.log`. These logs contain the actual failed
assertion; an earlier standalone-manifest workspace setup error is not counted
as the reproduction.

The loader convergence worker sets `entry.running = true` before admission
and clears it only after `drive(...).await`. Owner cancellation of
`Context::task` can destroy that future before its tail. Owner cleanup already
marks each entry non-live and clears its instance, outer Fiber and view, but
left the canceled worker's flag set. The snapshot then preferred the stale
running flag over the non-live state.

## Surgical repair and regression

Owner cleanup now clears `entry.running` alongside `entry.live`. This reflects
closed loader ownership; it admits no new work and changes no cancellation,
resource join, error, source or provider-retention contract.

`crates/re-cordis-loader/tests/shutdown_snapshot.rs` retains the reduced public
scenario and asserts that a disposal request stays pending while cleanup is
held, the exact cleanup capture remains alive before release, and the joined
barrier destroys it. Registry and root effect baselines and empty Runtime
diagnostics are asserted independently of the terminal snapshot. The group
finally reports `Disposed` with no inner or outer allocation. The full R0
oracle retains the same terminal-state assertion.

## Verification

All five tools were resolved with `rustup which --toolchain stable|1.85.0`;
the matching toolchain bin leads PATH and RUSTUP_TOOLCHAIN, RUSTC and RUSTDOC
are explicit. Tool paths/versions are saved in
`target/r0-implementation/tools.json`; commands/exits are in
`loader-stages.json`. The commands are:

```sh
cargo test -p re-cordis-loader --locked --offline
cargo clippy -p re-cordis-loader --all-targets --locked --offline -- -D warnings
```

Both toolchains pass the entire loader suite and strict all-target Clippy.
The stable-resolved rustfmt checks the changed production file and new
regression with edition 2024. Long logs:
`loader-test-{stable,1.85.0}.log` and
`loader-clippy-{stable,1.85.0}.log` under the same target directory.
No shared workspace/docs/parity matrix is claimed by this focused correction;
R0 independent review and shared acceptance remain the controller's gate.

No push, dependency, public API or Create template change is part of this
correction.
