# D5 experimental alpha publication — 2026-11-06

Status: **all seven packages published and distribution checks passed**.

The maintainer explicitly approved staged publication of
`84fa9bdf0eb4e631fce792bf163494b0a61aeba8` as `0.1.0-alpha.1`.
All uploads used that clean, frozen checkout with stable Cargo/rustc/rustdoc
1.97.1. Registry owner records identify `antruongnguyen`, user ID `447261`,
for every package. The maintainer configured Cargo authentication locally;
no token is included in this record.

## Published versions and checksums

| Package | Registry SHA-256 |
|---|---|
| [re-cordis-core](https://crates.io/crates/re-cordis-core/0.1.0-alpha.1) | `a43d1315315ec4c1833b8ca8e646838ccff834dcfde88f6aae8f9d86bf569cde` |
| [re-cordis-timer](https://crates.io/crates/re-cordis-timer/0.1.0-alpha.1) | `824275ad3b8b52fdf2dda90df39355e3daf793cd2182018a75bc281fb48a80bc` |
| [re-cordis-logger](https://crates.io/crates/re-cordis-logger/0.1.0-alpha.1) | `1a62186e1d5166e0feb514c0b366d4a1e69d7d8a519fd4ec2d65d38b82bb8bd7` |
| [re-cordis-loader](https://crates.io/crates/re-cordis-loader/0.1.0-alpha.1) | `7157ad39802c0ffc11ac6d3c5af6ac334821cd4676785a7b7ae5e1adc369b0b6` |
| [re-cordis-include](https://crates.io/crates/re-cordis-include/0.1.0-alpha.1) | `63c1d85c80cf3db4fe0c01425138e62c1692a8580d7fc5a11df1518e51be5bd4` |
| [re-cordis-hmr](https://crates.io/crates/re-cordis-hmr/0.1.0-alpha.1) | `1d845e9ac3582c7b8b3c5b35928b48fda1119c78e7ab94475682c7084a86a2e8` |
| [re-cordis-create](https://crates.io/crates/re-cordis-create/0.1.0-alpha.1) | `8c93e42f97460f76c48e3f9cd05ac41fcfca04081b7a95ae1910e117defe3889` |

Each checksum was confirmed through the crates.io version API and matched the
verified archive. Each version is not yanked. Normal Cargo verification and
`--locked` were retained; no `--allow-dirty`, `--no-verify`, path override or
registry patch was used to bypass a publication gate.

Uploads followed the approved order: core, timer, logger, loader, include,
HMR, Create. Before each dependent upload, the already-published prerequisites
were checked in the real registry/index, and the actual normalized archive was
built, inspected, tested and dry-run validated.

## Package verification

| Package | Checks per toolchain | Evidence scope |
|---|---:|---|
| core | 187 | D4 archive suite carried by exact checksum; D5 clean package/dry-run checksum reconfirmed |
| timer | 51 | Actual archive, real registry dependencies, tests/Clippy/example on both toolchains |
| logger | 21 | Actual archive, real registry dependencies, tests/Clippy/example on both toolchains |
| loader | 93 | Actual archive, real registry dependencies, tests/Clippy/example on both toolchains |
| include | 79 | Actual archive, real registry dependencies, tests/Clippy/example on both toolchains |
| HMR | 64 | Actual all-feature archive suite; no-default suite and reload/system/real-watch examples also pass |
| Create | 22 | Actual archive suite with the separate compatible checkout; installed-registry CLI checked below |

Both stable and Rust 1.85.0 were explicitly selected. For the five formerly
blocked dependent crates, normalized manifests have exact version requirements
without local paths, and Cargo metadata identifies their Cordis dependencies
as registry packages. Original extracted archive files remained byte-identical
through testing. Root/runtime/API/template source was not changed for publication.

## Actual registry consumers

The external R0 application uses all six runtime libraries at exact
`=0.1.0-alpha.1`, with HMR's Include feature. Cargo metadata contains no
Cordis path/git/patch substitute, and each lockfile checksum matches the
confirmed registry publication. Its shared application/test source is taken
from the verified HMR package.

On stable and 1.85.0, build, **eight tests including 100 lifecycle cycles**, run,
strict all-target Clippy and actual file-watch execution pass. This is real
registry consumption, not a workspace-path surrogate.

The actual published Create tool was installed separately with each toolchain
using `cargo install re-cordis-create --version '=0.1.0-alpha.1' --locked`
into task-specific temporary roots. Each installed CLI generated exactly the
five expected files and the MIT notice using a separate complete checkout at
the frozen source SHA. The resulting applications passed build, **five tests
including 100 cycles**, run and strict Clippy on both toolchains.

The generated application's own `0.0.0`/`publish=false` and three canonical
local dependency paths remain intentional. This is still the explicit-checkout
offline generator; no registry-backed generation mode was added.

## docs.rs and release source

All seven exact-version docs.rs URLs return HTTP 200 with generated Rustdoc.
HMR's `FileWatcher::watch_include` and `ReloadPlan::replace_include` anchors
are present. These hosted documentation builds do not expand the tested runtime
platform contract.

The annotated tag `v0.1.0-alpha.1` was pushed and dereferences to the approved
frozen source `84fa9bdf0eb4e631fce792bf163494b0a61aeba8`.
PR #5 integrated the reviewed candidate/evidence into main at
`4d1643049befd666eb2ece95fd1100ceae926bcb`; that later merge is not the
package/tag source.

D6 updates current documentation and release notes separately from the immutable
published packages and source tag. It does not rewrite the tag's preparation-time
status prose or republish changed package bytes.

## Preserved failures and corrections

- crates.io returned HTTP 429 for HMR and then Create. HMR's retry time was
  2026-11-06 14:14 UTC; Create's was 14:24 UTC. Each version was confirmed absent,
  and the unchanged verified archive was retried only after the stated time.
  Both retries succeeded with the expected owner/checksum. No rate-limit bypass,
  source change or version replacement occurred.
- The installed-Create control script initially compared macOS `/var` and
  `/private/var` aliases as unequal strings. File-identity checks proved they
  were the same expected checkout; canonicalizing both sides fixed the control
  assertion. The generator was correct and no package change was made.
- The account profile endpoint disallows API-token access because it is
  website-only. Actual publication and subsequent registry owner records
  established the designated publisher; no extra credential scope was requested.

## Local records

Ignored evidence is under `target/publication-20261106/`:
`publication-state.json`, `name-preflight.json`, per-package publication and
registry records, `progressive-checks.json`, retry logs,
`registry-consumer-checks.json`, `registry-consumer.json`,
`installed-create-checks.json`, `installed-create.json`,
`path-alias-diagnostic.json`, `docsrs-observation.json` and
`docsrs-final-methods.json`. Commands, exits, source SHA, owners and checksums
remain separately recorded.

No yanks, replacement versions, new runtime APIs or wider platform promises
were introduced. Current release/site completion is tracked in the D6 closure.
