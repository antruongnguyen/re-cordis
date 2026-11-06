# Create CR1/CR2 verified correction

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Review base: `d44620c9c28acfa5fd2e4b93c761af08126580f4`.
The commit containing this report is the correction checkpoint.
[Independent review](create-review.md) requires these two repairs before S2 acceptance.

## Corrected delta

- **CR1:** the shared name validator rejects exactly the three fixed local
  dependency names `re-cordis-core`, `re-cordis-logger`, `re-cordis-loader`.
  Their separate path packages also use 0.0.0, so Cargo cannot represent both
  identities in one lockfile. Admission fails before any staging write.
  Other Cordis satellite names remain available.
- **CR2:** select a fixed staging prefix with the opposite leading-dot status
  from the final basename. Dot-prefixed destinations use
  `_re-cordis-create-…`; other destinations use `.re-cordis-create-…`.
  This distinguishes exact and case-folded aliases before exclusive create_dir.
  Staging length remains independent of destination length; the existing
  128-collision bound, owned cleanup and single-rename commit remain unchanged.

Only the generator's `src/lib.rs` and `tests/scaffold.rs` changed.
The CLI, manifests and five bundled template inputs match `d44620c` byte-for-byte.
No prerequisite production source changed. The paired guide/API, decision and
limitation records document CR1's additional name restriction.

## RED and GREEN evidence

A fresh sol/high correction worker reproduced four new bug tests before changing
production code on both explicit stable and 1.85.0: library and CLI package
admission, the staged-write visibility boundary, and public exact/uppercase
destination aliases. The independently saved Cargo collision and staging probes
remain unchanged. A positive control keeps the other four satellite names valid.

Five added tests bring the package total to **22: 5 unit and 17 integration**.
Fresh child processes isolate the staging counter and test both exact and uppercase
paths. The boundary test asserts that the final directory does not exist before
or after staged writes. Public tests verify exactly the five final files, no
staging leakage, the alternate prefix and a 240-character destination.

Both toolchains pass focused CR1/CR2 checks, the full 22 package tests, all-target
check, strict all-target Clippy, warnings-denied Rustdoc and build.
Explicit stable formatting and diff whitespace checks pass.
Fixture-parent-only relocated copies of both original reviewer staging probes
return Ok on both toolchains, with a complete final project and no leftover stage.

The parent inspected the small production/test diff, confirmed the worker freeze
hashes and reran both full documentation checkers. Each passed 33 page pairs,
2 Create exports, 13 snippets, the quick-start app, and actual Create generation,
Cargo metadata, external app build/test/run and strict Clippy. Each external app
passed the five unchanged tests including 100 lifecycle cycles and displayed the
updated service value before successful joined shutdown. Eight checker regressions
also pass.

## Tool precision and reproduction

All five required Rust executables are resolved with rustup for stable 1.97.1 and
1.85.0, with matching RUSTUP_TOOLCHAIN, RUSTC, RUSTDOC and leading PATH.
Formatting is the shared **stable-only** source check; the checker and saved matrix
now also resolve stable cargo-fmt/rustfmt explicitly and reject an outside-toolchain
path. The first implementation report's both-toolchain formatting wording is
superseded by this exact final policy; it is not MSRV formatter evidence.
Compiler, Rustdoc and Clippy evidence uses the explicit selected tools throughout.

Setup fact: the correction worker installed the missing Rust 1.85.0 rustfmt
component through rustup before the parent clarified the stable-only formatting
gate. No application dependencies were downloaded by Create; the generator
continues to have no subprocess/network path.

Logs and runners: `target/create-20261104/corrections/`
(`red-{stable,1.85.0}.log`, `green-{stable,1.85.0}.log`,
`reviewer-probes-green.log`, and `verified-inputs.sha256.json`).
Parent logs: `docs-correction-{stable,1.85.0}.log`.
The initial complete 34-stage matrix for `d44620c` remains under
`target/create-20261104/before-review/`; its 503 workspace checks per toolchain
do not count as verification of this changed revision.

Frozen SHA256:

| File | SHA256 |
|---|---|
| `src/lib.rs` | `d8922cbf090e661eb6337ba5aef856a2d225914f4204778f5aeb8d2652f7da61` |
| `tests/scaffold.rs` | `2cd77f20038c2a764675e7ae9936a6a91d84d990299ee65e587fc54db9e17762` |

Paths above are relative to `crates/re-cordis-create`.
Scoped independent re-review and a fresh complete shared matrix remain required.
