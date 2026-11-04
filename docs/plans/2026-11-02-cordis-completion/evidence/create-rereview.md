# Create CR1/CR2 scoped independent re-review

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Previously reviewed implementation: `d44620c9c28acfa5fd2e4b93c761af08126580f4`.
Reviewed correction: `8373fa33ea165cf7d02ba7e515ce9ac085248ecf`,
on `feat/cordis-completion`. The working tree was clean when review began.

**CR1: ADDRESSED. CR2: ADDRESSED.** No new critical or important regression
was found in the correction delta. The final selected offline Create S0–S2
specification and code quality are **approved**, carrying the accepted generated
application/lifecycle review from [the initial independent review](create-review.md).
This is the scoped independent review gate; final S2 acceptance still requires
the integration owner's fresh shared matrix and documentation closeout.

## Correction assessment

**CR1.** The shared `validate_name` rejects exactly the three additional fixed
template dependency names: `re-cordis-core`, `re-cordis-logger`, and
`re-cordis-loader`. It runs first in `create`, before checkout inspection,
rendering or staging. This directly prevents the previously reproduced Cargo
`0.0.0` package-identity collisions. Both library and CLI regressions preserve
absent/empty destinations, return validation/exit code 2, and leave no stage.
The four other Cordis satellite names remain valid positive controls.
The paired guide/API pages and contract state the exact restriction; decision
and limitation records explain the Cargo reason.

**CR2.** Before `create_dir`, staging chooses `_re-cordis-create-…` for a
dot-leading final basename and `.re-cordis-create-…` otherwise. The opposite
leading-dot status prevents the original exact and case-folded alias without
depending on lexical equality. Its fixed prefix also keeps stage length
independent of destination length. Exclusive creation, 128 collision attempts,
owned cleanup/error retention, destination recheck and single-rename commit are
unchanged. The new boundary test runs fresh child processes for exact/uppercase
names and checks that the final directory is absent before and after staged
writes. Public tests check complete final files and no leftover siblings.

Within production code, only the name set and stage-prefix choice changed.
The CLI, Cargo manifests/lockfile and all five template inputs are byte-identical
to `d44620c`, confirmed by `git diff --exit-code` over those paths.
The correction remains appropriately small for the fixed offline template;
no new dependency, abstraction, subprocess, network or platform scope appears.

## Independently executed evidence

The saved runner and probes are in `target/create-rereview-20261104/`:
`run.py`, `probe.rs`, relocated `original-*.rs`, and full command/result logs
`stable.log` and `1.85.0.log`. Invocation:

```sh
python3 -B target/create-rereview-20261104/run.py stable
python3 -B target/create-rereview-20261104/run.py 1.85.0
```

For each toolchain, the runner resolves cargo, rustc, rustdoc, cargo-clippy and
clippy-driver with `rustup which --toolchain`, verifies a common bin directory,
and sets matching `RUSTUP_TOOLCHAIN`, `RUSTC`, `RUSTDOC` and leading `PATH`.
Build artifacts use separate `target/create-rereview-{stable,1.85.0}` directories.
No shared parity runner or lockfile was changed.

| Executed check | Stable | Rust 1.85.0 |
|---|---|---|
| `cargo test -p re-cordis-create --locked --offline` | 22 passed: 5 unit + 17 integration | Same |
| Strict all-target Create Clippy and explicit Create build | Passed | Passed |
| Independent public-library name rejection: three names × absent/empty | 6 cases passed | 6 cases passed |
| Independent CLI name rejection: three names × absent/empty | 6 cases passed; exit 2, empty stdout | Same |
| Independent complete generation: exact, uppercase, alternate, alternate-uppercase, 240-byte basename × absent/empty | 10 cases passed; exact five files, only final sibling | Same |
| Original review exact/uppercase public probes, changing only fixture parent | Both return Ok; complete project, no stage leakage | Same |

The original probes were compiled with the selected rustc and linked against the
fresh selected-toolchain library. Host case-fold aliasing was independently
confirmed with differently cased paths on the same fixture volume. The
precommit visibility checks passed as part of the new unit regression.
The prior RED reproductions and real Cargo-collision evidence remain carried
evidence; no production source was reverted or modified for this re-review.

Versions: stable cargo 1.97.1 (`c980f4866`), rustc/rustdoc 1.97.1
(`8bab26f4f`), Clippy 0.1.97; MSRV cargo 1.85.0 (`d73d2caf9`),
rustc/rustdoc 1.85.0 (`4d91de4e4`), Clippy 0.1.85.
Both source hashes match the correction freeze:
`src/lib.rs` = `d8922cbf090e661eb6337ba5aef856a2d225914f4204778f5aeb8d2652f7da61`;
`tests/scaffold.rs` = `2cd77f20038c2a764675e7ae9936a6a91d84d990299ee65e587fc54db9e17762`
(paths relative to `crates/re-cordis-create`).

Also independently passed: eight documentation-checker regressions,
`check_structure()` (33 matched page pairs, including two Create exports), and
`git diff --check` over the reviewed revision range. Full executable docs and
the shared final matrix belong to the integration owner; this report does not
claim them as reviewer executions.

## Checker, source and scope consistency

The checker/reproduction delta keeps generated-app build, tests, run and strict
Clippy on both explicit toolchains. Formatting is now explicitly stable-only,
consistent with the shared matrix: stable cargo-fmt/rustfmt are resolved through
rustup and checked against the selected compiler directory before Cargo uses the
leading toolchain PATH. Stable formatter resolution/version succeeded here;
full generated-app formatting execution is carried from the integration owner's
documentation runs. The correction report transparently records the worker's
earlier installation of Rust 1.85.0 rustfmt as setup, not a required MSRV gate.

The TypeScript pin remains `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
The unchanged [source map](create-source-map.md) retains one Cargo-equivalent
purpose, one portable-failure/Cargo representation, ten unselected remote
declarations and two Yarn exclusions. Exclusions remain dispositions, not passing
Rust/TypeScript tests. Native destination bytes may be rejected by the host
filesystem; the docs now say so. The resolver-3 workspace description matches
the unchanged generated manifest.

Accepted core/satellite and generated-application evidence is reused.
This review adds no Linux/Windows runtime, hostile-filesystem confinement,
crash durability, remote scaffolding, R0 or publication claim.
