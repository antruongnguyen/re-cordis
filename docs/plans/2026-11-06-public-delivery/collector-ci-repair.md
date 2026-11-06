# D1 collector CI repair

Status: test-only correction verified locally; fresh hosted CI and independent
review remain the controller's gates.

## Hosted failure evidence

The original pull-request Rust run 42199681145 failed on Ubuntu with 1.85.0 at
`github_collect.rs:112`: the 101-item rejection received an unexpected error
whose text the assertion discarded. Push run 42199672146 passed on identical
head `242db4c9d91d27dfd64a7b1d40627f8b975bf782`.

Diagnostic commit `ab982f9`, integrated by the controller as `2d77092`, printed
the actual assertion error and temporarily sampled the complete three-test
collector binary up to 100 times with normal parallel test execution, stopping
on its first failure. Two independent Ubuntu 1.85.0 runs reproduced fixture
execution contention:

| Run | First failing invocation | Location | Actual error |
| --- | --- | --- | --- |
| [Push 42201029103](https://github.com/antruongnguyen/re-cordis/actions/runs/42201029103) | 33/100 | Timeout/output-limit test, line 135 | `Text file busy (os error 26)` |
| [PR 42201037491](https://github.com/antruongnguyen/re-cordis/actions/runs/42201037491) | 92/100 | Snapshot/last-good test, line 101 | `Os { code: 26, kind: ExecutableFileBusy, message: "Text file busy" }` |

The controller retained the full logs as `collector-diagnostic-failure.log`
and `collector-diagnostic-pr-failure.log` under
`target/public-delivery-20261106` in the integration checkout.
The PR reproducer failed at the earlier 100-item collection call in the same
test; it did not recover the hidden error from the original line 112 failure.

## Cause and correction

Each test writes its own shell executable in place. Temporary directories are
unique, but parallel process creation can fork while another test holds its
script open for writing. The child inherits that writable descriptor until
exec closes it; Linux rejects executing the script during that interval with
`ETXTBSY`. The hosted errors identify fixture execution rather than JSON/item
validation as the failure boundary.

One test-local Tokio mutex now covers the full lifetime of each of the three
fixture-owning tests. This prevents executable writes and subprocess creation
in separate tests from overlapping, without blocking a runtime worker thread.
The lifecycle test acquires the mutex before its existing ten-second timeout.
Its FIFO barriers, in-flight updates and shutdown concurrency remain intact.
All assertions, child reaping, timeout and byte/item limits remain unchanged.
The useful actual-error diagnostic stays; the temporary CI sample is removed.
The final Rust workflow is identical to the workflow before the diagnostic.

## Local verification and limits

On macOS aarch64:

- `rustup run 1.85.0 cargo test -p re-cordis-core --test github_collect --locked`: 3 passed.
- The same focused command with `stable`: 3 passed.
- `rustup run stable cargo clippy -p re-cordis-core --test github_collect --locked -- -D warnings`: passed.
- `rustup run stable cargo fmt --all --check` and `git diff --check`: passed.
- `python3 -B tests/docs/test_check.py`: 9 regressions passed, including the
  current documentation structure and local-link checks.

Production collector/framework code, APIs, dependencies, version/publication
settings and release/Pages gates are unchanged. This host's available Podman VM
could not start because `vfkit` exited with code 2; repairing it is outside this
slice. Local passes do not prove absence of the Linux race. The controller must
run fresh complete hosted CI and obtain the scoped independent review before
merging the correction.
