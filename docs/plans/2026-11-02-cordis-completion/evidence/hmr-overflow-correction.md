# HMR H1 ingress-overflow correction

Date: 2026-11-04. Status: **implemented, verified, source frozen; independent correction review pending**.
Finding: [hmr-watch-adapter-review.md](hmr-watch-adapter-review.md).
Contracts: [hmr-watch-contract.md](hmr-watch-contract.md), H1/H3 implementation evidence.
Source baseline at dispatch: `77b50b5bdca92ab32d8acc4a9f2d58753c32e3ba`.
Final verification observed parent documentation commit
`3f5e562f0697f26ae755ec915a5192d50f54d716`; this worker made no commit.

## Trigger and root cause

The unchanged reviewer public probe registers two oversized regular files with
`queue_capacity = 1`, then holds the local host until the producer samples both.
The first failure fits; the second forces an ingress rescan. Before correction,
the producer stored both read-error coalescing signatures before checking
`Shared::send`. The rejected failure was then suppressed on every identical
subsequent sample. A rescan refreshes registered paths but cannot reproduce that
original read diagnostic on the local host.

The exact probe failed on both explicit toolchains before any production edit:
`overflow=1`, only `a.json` in the read-error path set, `events_evicted=0`,
and `joined=true`. The regression also failed on both: its bounded wait for
the second read failure expired after watcher disposal had joined.

## Narrow correction and regression

Only `crates/re-cordis-hmr/src/watch.rs` and
`crates/re-cordis-hmr/tests/filesystem.rs` changed. The producer now stores the
error signature only when `Shared::send(Message::Failure { ... })` returns
`Queued`. An unchanged error rejected with `RescanRequested` remains retryable
on the next poll; after ingress accepts it, identical failures are coalesced.
The published failure continues to carry its actual `std::io::Error` cause.

No queue, history, registration, generation or cancellation mechanism changed.
The existing bounded previous-sample map retains one delivered error signature
per registered path; no new queue, dependency, task, abstraction or core seam
was added.

The new real-filesystem test
`overflow_retries_original_read_failures_then_coalesces_delivery` forces the
same two-path, one-slot overflow using a bounded host hold. It verifies:

- Both original read-error paths become visible after host progress resumes.
- Each error is a `Read` failure with an inspectable `std::io::Error`,
  `InvalidData` kind and the exact watcher byte-limit cause.
- Several subsequent unchanged polls produce no additional failures.
- Public history evicts no events; joined disposal stops the producer and
  restores zero queued messages and resident registrations.

The test uses existing temporary-directory and bounded-wait helpers, retains
the wait result, and joins cleanup before behavioral assertions or returning
that result. The unchanged reviewer probe was then GREEN on both tools:
`overflow=1`, both `a.json` and `b.json`, `events_evicted=0`, `joined=true`.

## Executed verification

Each run independently resolved `cargo`, `rustc`, `rustdoc`,
`cargo-clippy` and `clippy-driver` via `rustup which --toolchain`.
Their toolchain bin led `PATH`, with matching `RUSTUP_TOOLCHAIN`, `RUSTC`
and `RUSTDOC`. Long logs and the command runner are in ignored
`target/hmr-h1-overflow-correction/`; separate build roots are
`build-{stable,1.85.0}`.

| Check | Stable 1.97.1 | Rust 1.85.0 |
|---|---|---|
| Unchanged exact reviewer probe before fix | RED: one path | RED: one path |
| New focused filesystem regression before fix | RED: bounded wait | RED: bounded wait |
| New focused regression after fix | Pass | Pass |
| Unchanged exact reviewer probe after fix | Pass: both paths | Pass: both paths |
| HMR all-feature/default all-target checks, locked | Pass | Pass |
| HMR all-feature tests, locked | 55 integration + 1 Rustdoc | 55 integration + 1 Rustdoc |
| HMR default tests, locked | 44 integration + 1 Rustdoc | 44 integration + 1 Rustdoc |
| Strict all-feature/default all-target Clippy | Pass | Pass |
| Strict no-deps all-feature/default Rustdoc | Pass | Pass |
| Offline `reload` example | Pass | Pass |
| Owned-source stable rustfmt and `git diff --check` | Pass | Same source |

Logs: `{probe-red,regression-red,regression-green,probe-green,check-all,
check-default,test-all,test-default,clippy-all,clippy-default,doc-all,
doc-default,example}-{stable,1.85.0}.log`.
Tool paths/versions and commands are recorded in `tools-*.json`, `run.py`
and `matrix-*.json`.

## Frozen source hashes and boundaries

SHA256:

| File | Hash |
|---|---|
| `crates/re-cordis-hmr/src/watch.rs` | `a24dbdbc5a991ac45c3a767717c38d35f62f49f7a0706be6537fdd8bc48e6936` |
| `crates/re-cordis-hmr/tests/filesystem.rs` | `dd3549be7725c9d24fb66e9a4657bf3f1608bd40be40297786b3028112b09e10` |
| Unchanged `target/hmr-final-review/probe/src/lib.rs` | `a9b4ff89d4af1961031ca1865859af2826a8b3d927f6d20794d0fd5422a5e2e4` |

The machine-readable copy is `source-freeze.json` in the log directory.
Executed on aarch64 macOS; existing polling, fingerprint, symlink, blocked-I/O
and Linux/Windows evidence boundaries remain. This correction does not claim
the parent workspace/docs/parity acceptance matrix. H2 and the accepted H3
retention mechanism were not reopened. No Git/index, publication or Create work
was performed.
