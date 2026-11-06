# D3 alpha preparation decision — 2026-11-06

Status: **approved for candidate preparation; publication is not authorized**.

The maintainer explicitly approved preparation of the proposed seven-package
`0.1.0-alpha.1` candidate after D2 and supplied
[crates.io account antruongnguyen](https://crates.io/users/antruongnguyen).
The public API profile returned HTTP 200 with user ID `447261`, login
`antruongnguyen`, name An Nguyen, and matching GitHub identity.
This confirms the public account designation, not possession of publishing credentials.

## Approved contract

- Packages: `re-cordis-core`, `re-cordis-timer`, `re-cordis-logger`,
  `re-cordis-loader`, `re-cordis-include`, `re-cordis-hmr`,
  `re-cordis-create`.
- Coordinated version: `0.1.0-alpha.1`; all ten internal dependency edges use
  exact `=0.1.0-alpha.1` requirements alongside local paths, including HMR's
  optional Include dependency and dev-only logger dependency.
- Candidate manifests may restrict publication to `["crates-io"]`; this
  enables preparation and validation, not an upload.
- Rust 2024/resolver 3, MSRV 1.85 verified with 1.85.0; the initial runtime
  support statement remains the tested Apple Silicon macOS scope.
- Preserve local/non-Send hosting, explicit joined cleanup, dependency features
  and pins, empty HMR defaults, optional Include integration and all notices.
- Create remains an offline tool requiring `--cordis-path` and a complete
  compatible checkout. Its generated application's own `0.0.0`/
  `publish=false` and five-file template remain unchanged.
- During alpha development, intentional breaking changes require release notes
  and a new coordinated alpha version. Recommend exact pins to consumers.
- Proposed release tag: `v0.1.0-alpha.1`. D4 records the final candidate SHA
  and exact compatible checkout after commit/review; no tag is created now.
- Public development documentation:
  [antruongnguyen.github.io/re-cordis](https://antruongnguyen.github.io/re-cordis/).
  Immutable release-guide and exact-version docs.rs destinations belong in the
  release draft; planned links are not evidence that those destinations exist.

## Boundaries and subsequent gate

Preparation starts from main `3bc9de7193641625446cf390586dc3d54f38d696`
on `feat/alpha-release`. All seven exact name queries returned HTTP 404 on
the decision date; this is a point-in-time observation, not ownership,
reservation or guaranteed future availability.

No runtime/API redesign, new dependency, platform expansion, facade, template
variant or registry-backed generator is selected. Current and candidate docs
must distinguish a versioned candidate from an available registry release;
dated acceptance evidence keeps its original versions and scope.

D4 must complete the relevant full stable/1.85.0 verification, package inventory,
actual core/Create archive verification, independent review and a concrete
candidate/release-notes report. Dependent archive/dry-run verification that needs
unpublished prerequisites remains explicitly progressive in D5.

Before any upload, recheck publisher authorization and name state and obtain the
maintainer's approval of the exact candidate. Do not publish, create release
tags, or replace the accepted live Pages snapshot during candidate preparation.
See the [delivery plan](../../superpowers/plans/2026-11-06-public-delivery.md)
and [readiness assessment](../../release-readiness.md).
