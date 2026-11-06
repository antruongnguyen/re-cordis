# Release readiness and initial release proposal

Date: 2026-11-05 (Asia/Ho_Chi_Minh).
Scope: accepted R1 plus bounded B1/local release preparation on `feat/cordis-completion`.
The [dated follow-up](plans/2026-11-02-cordis-completion/evidence/2026-11-05-b1-closure.md)
records the Create fixture correction, provenance notices and current verification.
The assessment and proposal are independently reviewed and accepted; see
[R1 acceptance](plans/2026-11-02-cordis-completion/evidence/r1-acceptance.md).
This does not approve publication. The subsequent
[documentation delivery](plans/2026-11-05-documentation-site/plan.md) prepares
local Fumadocs/Rustdoc output, documentation CI and manual Pages deployment;
it does not establish a reachable deployed site or hosted docs.rs release.

**Recommendation: ready only for local/Git checkout consumption today.**
The seven crates remain unpublished, unstable `0.0.0` with `publish=false`.
The implementation is useful within its documented Rust contracts, but the
current manifests cannot publish the complete package set. A successful
standalone archive build is not registry readiness.

**Proposal, requiring a subsequent release decision:** a coordinated
`0.1.0-alpha.1` experimental release of the six runtime libraries and the
explicitly offline Create library/CLI described below. Do not publish a stable
API, facade, remote generator or new platform promise as part of that release.
No version, dependency, feature, release tag or publication change is made by R1.

See [R1 package evidence](plans/2026-11-02-cordis-completion/evidence/r1-package-audit.md)
for commands and stage-specific results, [R1 execution plan](plans/2026-11-02-cordis-completion/evidence/r1-plan.md)
for scope, and [R0 acceptance](plans/2026-11-02-cordis-completion/evidence/r0-acceptance.md)
for the carried runtime baseline.

## Current consumption and compatibility

Use a complete checkout at a reviewed commit and the documented local Cargo
paths. “Git consumption” here means obtaining and pinning that complete
checkout, then following the [quick start](en/getting-started.md) or
[Create guide](en/guides/create.md). R1 does not establish a remote Cargo Git
installation workflow, repository access rights, or online availability of
these unpushed changes. Keep all Cordis crates from the same revision/source;
mixing core package identities can produce incompatible public Rust types.

Cargo.lock pins registry resolution; it does not freeze the contents of a
path dependency. Moving the checkout breaks Create's canonical absolute paths;
editing/updating it changes the next build even if its version remains 0.0.0.
Keep the checkout present, pin its Git revision and commit the application's
lockfile. Offline builds additionally require cached third-party dependencies.

Current contracts include local Tokio/Runtime hosting, typed service/config
APIs, exact committed provider retention, explicit Failed recovery and joined
async cleanup. They do not promise Send/Sync handles, executor independence,
transparent JS Proxy/decorator behavior, complete upstream equivalence,
binary hot loading, global readiness, or successful async cleanup from Drop.
Public errors, traits, fields and re-exports are API surface, not just type
names. The [paired API index](en/api/README.md) maps exports; generated Rustdoc
owns exact signatures and bounds. TimerExt is intentionally sealed. Group
belongs to loader; HMR re-exports the same loader FactoryReplacement.

The accepted six core paired scenarios/73 observations and two loader
scenarios/nine observations remain bounded comparisons. B0's 147 evidence
gaps and the loader map's narrower gaps are neither proven defects nor an
automatic R1 implementation queue. See [limitations](limitation.md).

## Proposed release scope

| Cargo package / Rust import | Selected capability | Important boundary |
|---|---|---|
| re-cordis-core / re_cordis_core | Local lifecycle, services, effects, events, diagnostics and config updates | Explicit hosting and operation/cleanup diagnostics |
| re-cordis-timer / re_cordis_timer | TimerExt timeout/sleep/interval/stream/throttle/debounce | Tokio time driver; local ownership and joined cancellation |
| re-cordis-logger / re_cordis_logger | Scoped native console writer/rendering adapter | Core Logger remains separate; synchronous I/O, UTC and explicit color policy |
| re-cordis-loader / re_cordis_loader | Typed catalog/source entries, structural groups, JSON and reconciliation | No JS imports/eval, implicit serialization or file watching |
| re-cordis-include / re_cordis_include | Bounded files, patches/provenance, journals and persistence | Restricted value YAML; synchronous I/O and explicit durability limits |
| re-cordis-hmr / re_cordis_hmr | Registered-file polling and linked factory replacement | Default features empty; optional include adapter; source edits require restart |
| re-cordis-create / re_cordis_create; binary re-cordis-create | One offline local-checkout scaffold | Required --cordis-path; no remote transport, install, launch or additional template |

No facade is proposed. The basic generated app already composes three crates
and R0 composes six with explicit imports and working examples. There is no
demonstrated consumer obstacle that a re-export-only facade solves. A new
package would add a version/publication surface and potentially obscure which
optional features are needed. Likewise, a separate group crate would duplicate
a loader-owned export without adding functionality.

## Actual dependency and feature audit

All seven inherit Rust 2024, MSRV 1.85, MIT and the same repository URL from
the workspace; resolver is 3. Every package has a description and remains
publish=false. No workspace package uses a target-specific dependency or
build script. Create has no Cargo dependencies: its generated application's
dependencies are a separate graph.

| Package | Direct normal third-party dependencies | Local Cordis dependencies |
|---|---|---|
| core | futures-util, tokio | None |
| timer | futures-util, tokio | core |
| logger | time =0.3.36 | core |
| loader | futures-util, tokio, serde_json | core |
| include | futures-util, serde_json, yaml-rust2 | core, loader |
| hmr | futures-util, tokio | core, timer, loader; optional include |
| create | None | None; generated app uses core, logger, loader |

Core's production Tokio features are rt/sync; timer adds time/macros. Workspace
test graphs unify additional features and therefore do not by themselves prove
core's minimal production graph. R1 inspects normal feature trees separately.
JSON/YAML/calendar dependencies belong to satellites; core's GitHub HTTP,
cryptographic, JSON and extra Tokio dependencies are dev-only. HMR's console
dependency is also dev-only for the reference application.

The full all-feature metadata graph contains 74 resolved packages, with
target-conditioned transitive edges. That inventory is not a claim that all
packages compile on every target. Normal, build and dev edges are recorded
separately in package evidence. The committed resolution passes the accepted
1.85.0 matrix; broad semver ranges do not prove that every future resolution
will keep that MSRV. Logger's exact time pin is deliberate accepted policy.

## Package findings and focused corrections

1. All seven baseline package file lists omitted the root MIT LICENSE.
   Core/Create archives actually constructed by Cargo omitted it too.
   R1 copies the exact repository notice into each package root; Cargo includes
   those files without changing manifest/dependency behavior.
2. Six packages had no crate-local README and no resolved README metadata.
   R1 adds concise package-specific contracts, local documentation instructions
   and accurate unpublished status. Create's existing README gains distribution
   boundaries. Repository guides are not bundled in crate archives; these
   READMEs identify checkout paths instead of claiming a published docs site.
3. The generated basic application declared MIT without carrying its template's
   copyright/permission notice. R1 appends the original notice to the existing
   generated README. Its five-file layout, Rust source, dependency manifest
   and lifecycle tests remain unchanged.

There is no discovered runtime/API defect requiring redesign in this audit.
Package contents retain tests, examples and Create's embedded basic template;
they do not bundle local reference repositories, target output or the complete
guide tree. Root export documentation is checked in both languages. R1 does
not introduce archive trimming or remove test fixtures to make packaging appear
successful.

The focused follow-up identifies Cordis-derived logger palettes/channel hash,
cosmokit 1.8.1-derived default-name tokenization, and literal upstream test
excerpts in source evidence. The exact “Copyright (c) 2021-present Shigma” MIT
notice is now retained in root and core package UPSTREAM-NOTICES.md; existing
repository/package/template MIT notices remain intact. See
[provenance evidence](plans/2026-11-02-cordis-completion/evidence/2026-11-05-provenance.md).
This closes those identified local omissions, not a legal certification or
an exhaustive third-party compliance/security audit. Future binary/vendor or
changed-source distributions still need their applicable notice review.

## Distribution checks and their limits

| Stage | Actual result / interpretation |
|---|---|
| Cargo metadata and normal/all-feature dependency trees | Executed against the actual workspace on stable and 1.85.0; captures names, features, paths, targets and resolved dependencies |
| cargo package --list | Executed for every crate; establishes intended file lists, not successful archive creation or compilation |
| cargo package --no-verify | Core/Create construct archives; five satellites fail because their core path dependency has no version requirement |
| cargo package with verification | Core/Create archive verification builds pass on stable and 1.85.0; this is narrower than running their archive test suites |
| Create tests from an extracted archive | R1 stable failure reproduced on both toolchains; after the test-only explicit-checkout correction, five library and all 17 scaffold tests pass on stable and 1.85.0 with unchanged extracted archive bytes |
| cargo publish --dry-run | Every attempt is refused by publish=false before publish validation; no successful registry dry-run is claimed |
| Exact crates.io name queries | All seven API queries returned HTTP 404 on the audit date; point-in-time absence only, not ownership, reservation or guaranteed availability |
| Actual generated app | Real CLI output, metadata, build/test/run and subsequent full docs-checker coverage are recorded separately from crate packaging |

Final package results include the corrected notices/READMEs; baseline and
intermediate results remain separately labeled in [package evidence](plans/2026-11-02-cordis-completion/evidence/r1-package-audit.md).
No production path dependency was patched, versioned, vendored or published
to bypass an expected packaging refusal. A future release must validate
normalized manifests and archives against the actual registry versions.

The B1/local-preparation follow-up resolves the fixture decision with test-only
`CORDIS_CREATE_TEST_CHECKOUT`. Its absence preserves repository tests; the CLI
still requires `--cordis-path`. Actual baseline archives reproduce the same six
failures on stable and 1.85.0, extracted outside any source workspace so Cargo's
parent-workspace discovery is not the cause. Corrected Cargo archives pass all
22 tests using a separate complete checkout pinned to accepted R1
`03d255029db6a57ccc17c0b1e26facdb491c058b`; all extracted file bytes remain
unchanged before/after tests. Each actual archive-built CLI produces exactly
five files with MIT, and generated-app build, five tests including 100 cycles,
run and strict Clippy pass on both toolchains. See
[the reproduction and commands](plans/2026-11-02-cordis-completion/evidence/2026-11-05-create-archive.md).
Historical [R1 review](plans/2026-11-02-cordis-completion/evidence/r1-review.md)
retains its original failure and narrower scope. The archive contains the tool
and template, not a self-contained framework; an external compatible checkout
and cached application dependencies remain required.

## Proposed publication order and manifest changes

The normal dependency DAG permits this sequence:

1. core;
2. timer, logger and loader (independent after core);
3. include (after core and loader);
4. hmr (after core, timer and loader, plus include for the optional adapter);
5. create (no Cargo prerequisite; place last to validate its documented
   checkout/application workflow against the completed set).

Logger is not an HMR production dependency. Its dev-only edge matters for
the packaged examples/tests: Cargo strips a versionless path dev-dependency
from a normalized registry manifest. The release change should give that
edge a version as well and preserve archive example/test usability; placing
logger before HMR supports that validation. No build-dependency cycle exists.

For the proposed first alpha, use coordinated package version
0.1.0-alpha.1 and exact matching internal requirements
`version = "=0.1.0-alpha.1"` alongside each local `path`, including optional
HMR Include and HMR's dev logger edge. Paths continue serving local workspace
development; normalized published manifests use registry versions. Change
publish=false only for the separately approved package set. Re-resolve and
review Cargo.lock with both explicit toolchains, verify all intended feature
sets and inspect Cargo's normalized archive metadata. Preflight the complete
release checkout, then verify each dependent package/dry-run against its
already-published prerequisites before proceeding in that order.

Create is proposed only as an offline tool requiring a compatible complete
checkout. Its own binary package contains the template, not the framework.
Its generated package may remain 0.0.0/publish=false with absolute path
dependencies; publishing the tool does not make those dependencies portable.
Test the released tool against the explicitly documented checkout revision.
Changing only package versions does not test generated source compatibility;
Create's literal name/layout admission is not a version or authenticity check.

A registry-backed Create mode is deferred. If separately selected after
prerequisites exist, replace the three generated Cordis path entries with
tested versions, revise --cordis-path/checkout expectations and prove generation,
resolution, build, tests and shutdown from outside the repository. Do not
advertise that mode or preemptively change the current manifest/template here.

## Proposed stability, MSRV and platforms

During alpha.N development, allow intentional API changes only with release
notes and a new coordinated alpha number; recommend exact pins to consumers.
The first alpha is not 1.0 compatibility. Before removing the prerelease
suffix, define the 0.x policy: compatible fixes in patch releases, breaking
changes in the next minor, migration notes for public types/traits/errors,
and explicit treatment of feature/default and MSRV changes. Do not silently
claim that today's 0.0.0 already follows this proposed policy.

Keep 1.85 as the initial MSRV and verify with 1.85.0; use stable for formatting.
A future MSRV increase requires a recorded release decision and fresh dependency
resolution evidence, rather than inferring support from edition metadata.

Propose initial verified runtime support only for the tested Apple Silicon
macOS host/target (`aarch64-apple-darwin`, macOS 26.7.1 in carried acceptance).
Do not infer all macOS versions/architectures. Linux and Windows remain
unverified for runtime filesystem/watcher behavior, not proven broken.
Expanding that support requires its own platform tests; R1 does not add CI.
No real terminal-emulator, crash-durability, network filesystem, hostile-path,
arbitrary concurrent-writer or hard shutdown-latency guarantee is established.

## Release blockers, limits and remaining decisions

**Blockers for registry publication of the proposed set:**

- The current publish=false/version policy has not been changed by a release
  decision; versionless local dependencies prevent five satellite archives.
- A final versioned release checkout has not been resolved, packaged and
  verified against actual published prerequisites. Current dry-runs stop at
  publish=false. Packaged dev-dependency/example closure also remains a gate.
  The Create fixture decision is now implemented and locally verified; retest
  the final release tool against its declared compatible checkout.
- Publisher/owner authority and exact normalized crate-name claims must be
  checked at release time; the HTTP 404 snapshot cannot establish them.
- Release documentation URLs and the compatible Create checkout must be made
  reachable at the chosen release revision. R1's local commits are not pushed.

**Nonblocking limitations for the proposed explicitly experimental scope:**
local/non-Send hosting, required joined teardown, bounded parity evidence,
macOS-only runtime evidence, synchronous bounded I/O, polling gaps, value YAML,
and the offline Create checkout requirement. These must remain visible to
consumers. Missing notice/README defects identified above are corrected by R1;
they are not left as implementation blockers.

**Deferred capabilities:** stable API/1.0, a facade, wider platform acceptance,
remote or registry-backed Create templates, extra templates, native binary hot
loading, process supervision and live integrations. Bounded B1 status/link
closure and local archive/provenance preparation are covered by the dated
follow-up. Documentation CI and the gated Pages workflow are now implemented
locally, with current action pins and a supported macOS runner; actual hosted
execution/deployment and platform expansion remain unverified/separate gates.

The next release decision must approve the seven-package scope, alpha version
and coordination policy, macOS/MSRV support statement, publisher/name ownership,
published documentation location and exact checkout supported by offline Create.
Only then perform the version/dependency/publication work and its full shared
stable/1.85.0 verification. R1 completes the assessment and proposal; it grants
no publication approval.
