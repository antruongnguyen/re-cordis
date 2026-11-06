# D1 collector CI investigation

Status: diagnostic prepared; root cause and correction remain pending Ubuntu evidence.

The Rust pull-request run 42199681145 failed on Ubuntu with Rust 1.85.0 at
`crates/re-cordis-core/tests/github_collect.rs:112`. The 101-item rejection
assertion received an error whose text did not contain `more than 100`, but the
original assertion discarded that error. The push run 42199672146 on identical
head `242db4c9d91d27dfd64a7b1d40627f8b975bf782` passed both toolchains.

## Findings

- The collector fixture is a shell script written in place to one `gh` path.
  Each test owns a distinct PID/counter temporary directory, so parallel tests
  do not intentionally share paths.
- The preceding 100-item snapshot succeeded. The 101-item fixture contains
  valid JSON and must be rejected before snapshot publication; duplicate-item
  validation is later in the pipeline and cannot explain the asserted error.
- Query spawning, bounded pipe reads, child exit, JSON parsing and item-count
  validation can each return errors before the expected item-count error.
  No actual unexpected error is available in the failed log.
- Linux `ETXTBSY` from a writable executable fixture descriptor inherited by a
  concurrently spawned process is a hypothesis, not an established cause.
- The diagnostic preserves the same 101-item expectation and last-good snapshot
  check while printing the actual error if the expectation fails. It adds no
  accepted errors, sleeps or retries, and changes no application/framework API.

## Local verification and limits

On macOS aarch64, `rustup run 1.85.0 cargo test -p re-cordis-core --test github_collect`
and the same command with `stable` passed all three tests.
`rustup run stable cargo fmt --all --check` and `git diff --check` passed.
The workflow parsed with Ruby YAML; its extracted diagnostic passed `bash -n`,
and its MSRV condition and 100-invocation bound were checked.
The available Podman Linux VM could not start: its `vfkit` helper exited with
code 2. Repairing that unrelated host installation is outside this slice.
A macOS pass cannot establish the cause or correction of the Ubuntu failure.

## Bounded hosted next step

The temporary, clearly labeled MSRV-only step in `.github/workflows/ci.yml`
runs the full collector integration binary up to 100 times with normal parallel
test execution, prints each attempt, and stops immediately on the first failure.
It runs after the existing workspace test even if that test fails; all existing
CI gates are preserved. The controller will push this diagnostic candidate and
retain the actual Ubuntu error before choosing a correction. Remove this step
in the subsequent correction commit. This is evidence collection; a later pass
does not clear the original failed run or establish a fix. Do not merge until
the returned error supports a concrete correction and fresh CI verifies it.
If all 100 invocations pass, retain the unresolved status and use a bounded
process/file-descriptor trace rather than accepting additional errors or
introducing timing workarounds.
