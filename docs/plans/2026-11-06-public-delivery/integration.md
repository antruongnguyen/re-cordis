# D1 integration progress

Date: 2026-11-06 (Asia/Ho_Chi_Minh).
Status: **in progress; collector MSRV diagnostic required before merge**.

The maintainer requested execution of the [public delivery plan](../../superpowers/plans/2026-11-06-public-delivery.md) after the README/reference cleanup.
This report records observed results; it does not declare deployment or release readiness.

## Revision and pull request

- Base main: `808c9f573dd96c56fa2ec43e0b1bfb9ed39b2f3f`.
- Reviewed candidate: `c373a814a366751d5e48978522b58dd825ab6ca4`.
- Integration branch: `feat/cordis-completion`, 124 commits ahead of main with no divergence at D1 entry.
- [PR #1 — Complete Rust satellites and bilingual documentation delivery](https://github.com/antruongnguyen/re-cordis/pull/1).
- The seven unpublished packages retain `0.0.0`/`publish=false`.
- GitHub permits merge commits; main has no configured required status checks. This execution nevertheless requires both Rust and Documentation workflows to pass before merging, as specified by D1.

## Fresh local preflight

At the reviewed candidate, explicitly resolved Rustup stable and 1.85.0 toolchains each passed
`cargo test --workspace --all-features --locked --offline`: **517 checks per toolchain**, no failures.
All nine documentation regressions passed, covering 35 EN/VI page pairs, public exports, hooks and local links.
Logs are retained under `target/public-delivery-20261106/preflight-{stable,1.85.0}.log`.
These runs do not replace the broader accepted B1 matrix or establish a new platform claim.

## Independent integration review

An independent gpt-6-astra/high reviewer approved specification compliance and quality for
`808c9f5..c373a814`, with merge conditional on exact-revision hosted checks.
No material correctness or scope finding was reported.

The review covered the commit/file inventory, D1 contracts, accepted satellite/R0/R1/B1/site evidence,
later workflow and website changes, README/reference cleanup, and targeted core-task/composed-shutdown code.
It confirmed that runtime code/tests are unchanged after B1; the only later crate metadata change selects
HMR's Include feature for docs.rs. Applicable licenses and accepted commit ancestry remain intact.

This was not a new line-by-line audit of all 469 changed files. The reviewer did not run Cargo, website,
browser or hosted checks. Its nonblocking observation was historical evidence whitespace in the full
branch diff, including `hmr-watch-contract.md` lines 364/372 and blank EOF lines. No unrelated formatting
cleanup was introduced.

## Hosted candidate verification

All runs below identify candidate `c373a814a366751d5e48978522b58dd825ab6ca4`.

| Event | Workflow | Result |
|---|---|---|
| push | [Rust](https://github.com/antruongnguyen/re-cordis/actions/runs/42196744001) | stable and 1.85.0 pass |
| pull request | [Rust](https://github.com/antruongnguyen/re-cordis/actions/runs/42196762752) | stable and 1.85.0 pass |
| push | [Documentation](https://github.com/antruongnguyen/re-cordis/actions/runs/42196744104) | guides/Rustdoc pass; static-site check fails |
| pull request | [Documentation](https://github.com/antruongnguyen/re-cordis/actions/runs/42196762710) | guides/Rustdoc pass; static-site check fails |

The site builds successfully and passes type checking, lint and all seven script tests.
The final exported-artifact check fails at `website/scripts/check.ts:229`:
the hosted unresolved dependency-prose set contains **96 entries**, while the recorded set contains **97**.
Both events reproduce the failure. The exact compiler-artifact distinction must be identified before a fix;
a blanket subset/ignore rule is not acceptable. The original log is
`target/public-delivery-20261106/documentation-failure.log`.

## Corrected documentation candidate

Correction `242db4c9d91d27dfd64a7b1d40627f8b975bf782` incorporates the
[precise hidden-token rendering repair](hosted-link-fix.md), independently approved in scoped review.
Both real compiler artifacts pass locally; unrelated omission/count negatives still fail.
The hosted [push Documentation run](https://github.com/antruongnguyen/re-cordis/actions/runs/42199672203)
and [PR Documentation run](https://github.com/antruongnguyen/re-cordis/actions/runs/42199681149)
both pass on that exact candidate.

The [Rust push run](https://github.com/antruongnguyen/re-cordis/actions/runs/42199672146)
also passes both toolchains. The [Rust PR run](https://github.com/antruongnguyen/re-cordis/actions/runs/42199681145)
passes stable but fails 1.85.0 in
`executable_collects_complete_snapshot_and_preserves_last_good_on_query_failure`,
at `crates/re-cordis-core/tests/github_collect.rs:112`. The expected item-limit
error text was absent; the assertion did not print the actual error.
This differing result on identical source requires diagnosis, not a blind rerun.
The full failed log is `target/public-delivery-20261106/rust-msrv-failure.log`.

## Remaining D1 work

- Capture the collector's unexpected error with unchanged assertion semantics and a bounded hosted reproducer.
- Apply only an evidence-backed correction, independently review it, and pass Rust/Documentation at the final PR head.
- Merge with source/acceptance history preserved.
- Verify both workflows at the merged main revision and record its SHA/run URLs.

Pages, its deployment environment and its opt-in variable were absent at initial inspection.
Their setup belongs to D2 after successful integration. Alpha/version/publication decisions remain later stages.
