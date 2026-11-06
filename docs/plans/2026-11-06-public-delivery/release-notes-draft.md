# Draft release notes — re-cordis 0.1.0-alpha.1

Status: **unpublished candidate; preparation approved, publication not authorized**.
Proposed release tag: `v0.1.0-alpha.1`. No tag has been created.
The final reviewed candidate SHA and compatible complete Create checkout are
recorded separately after commit/review; this draft does not identify its own commit.
Publisher designated for the proposed crates.io release: antruongnguyen.

## Scope

The first experimental alpha coordinates seven MIT packages. This candidate
changes release metadata/documentation and makes parity test/example inputs
package-local. The accepted runtime/API semantics, trace expectations,
dependency features/pins and five-file Create template remain unchanged.

| Package | Capability | Planned exact-version API |
|---|---|---|
| re-cordis-core | Local runtime/context, plugins, services, events, effects and diagnostics | [core](https://docs.rs/re-cordis-core/0.1.0-alpha.1/re_cordis_core/) |
| re-cordis-timer | Owner-scoped timeout, sleep, interval/stream, throttle and debounce | [timer](https://docs.rs/re-cordis-timer/0.1.0-alpha.1/re_cordis_timer/) |
| re-cordis-logger | Scoped console writers, color policy, UTC rendering and visible I/O failures | [logger](https://docs.rs/re-cordis-logger/0.1.0-alpha.1/re_cordis_logger/) |
| re-cordis-loader | Typed catalogs, structural groups, bounded JSON and source reconciliation | [loader](https://docs.rs/re-cordis-loader/0.1.0-alpha.1/re_cordis_loader/) |
| re-cordis-include | Bounded files, patches/provenance, persistence and joined flush | [include](https://docs.rs/re-cordis-include/0.1.0-alpha.1/re_cordis_include/) |
| re-cordis-hmr | File polling and linked factory replacement; optional Include integration | [hmr](https://docs.rs/re-cordis-hmr/0.1.0-alpha.1/re_cordis_hmr/) |
| re-cordis-create | Offline library/CLI generating one local-checkout application | [create](https://docs.rs/re-cordis-create/0.1.0-alpha.1/re_cordis_create/) |

These docs.rs destinations are planned, not reported reachable or successfully
built. HMR defaults remain empty; its `include` feature enables the Include
adapter and is selected by its docs.rs metadata.

## Consumption and compatibility

Current supported consumption uses a complete, reviewed checkout and local
Cargo paths. Keep all Cordis crates from one source revision; Cargo.lock does
not freeze local path contents. Candidate manifests preserve local paths and
use exact `=0.1.0-alpha.1` requirements for all ten internal edges.

After separately verified publication, consumers should pin exact alpha versions,
for example `re-cordis-core = "=0.1.0-alpha.1"`. Registry install/consumer
results remain a later gate. No available registry release is claimed here.

Rust 2024 requires Rust 1.85 or newer; verification uses explicit stable and
1.85.0 toolchains. Runtime filesystem/watcher support is limited to the tested
Apple Silicon macOS environment (`aarch64-apple-darwin`); Linux/Windows runtime
acceptance remains separate. The local Tokio host supports non-`Send` resources.
Keep it driven through explicit joined cleanup and inspect operation/cleanup
diagnostics. Dropping handles does not complete asynchronous teardown.

The API is experimental. Intentional breaking changes during alpha development
require release notes and a new coordinated alpha version. Stable 1.0 APIs,
complete upstream equivalence, transparent JS Proxy/decorator compatibility,
native binary hot loading and broader platform acceptance are outside scope.
Include/console I/O remains bounded and synchronous; YAML is value-oriented
and polling can miss changes.

## Offline Create

The tool always requires `--cordis-path` and a separate compatible complete
checkout, including after future registry installation. The final publication
report must identify the exact checkout revision. The archive contains the tool
and template, not framework crates; cached dependencies are needed for offline
application builds. Its generated application's own `0.0.0`/`publish=false`
and three absolute Cordis paths remain intentional. No remote templates,
registry-backed scaffold mode, install/launch behavior or extra template is added.
Preserve the original MIT notice in its generated README.

## Documentation destinations

The [public development site](https://antruongnguyen.github.io/re-cordis/)
follows its deployed revision and remains distinct from the release source.
Candidate preparation does not replace the accepted live Pages snapshot.

Planned immutable release guides, to be verified only after authorized tag creation:

- [English index](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/README.md)
  and [Vietnamese index](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/vi/README.md).
- [English installation](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/getting-started.md)
  and [Vietnamese installation](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/vi/getting-started.md).
- [English Create](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/guides/create.md)
  and [Vietnamese Create](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/vi/guides/create.md).
- [English API contracts](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/api/README.md)
  and [Vietnamese API contracts](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/vi/api/README.md).

These links identify intended immutable destinations; they do not establish
availability. Repository-only plans/design evidence is excluded from website,
search, Markdown/LLM exports and crate/Rustdoc content.

## Verification and progressive publication gates

Record fresh full candidate stable/1.85.0 results, package inventory,
core/Create archive checks, independent review and exact candidate SHA in the
`release-candidate.md` report before publication approval.
Dependent archive/dry-run checks must then use actual published prerequisites.
Workspace path builds, patch overrides and no-verify archives are not proof of
registry consumption. After all packages exist, prove a real registry R0 consumer,
the installed Create CLI with the declared checkout, and all exact docs.rs builds.

Publish only the approved immutable candidate and only after explicit authorization,
in this order: core, timer, logger, loader, include, hmr, create. Stop on a package,
registry-consumer or docs.rs failure and report exactly what is already published.
Published bytes are immutable; do not automatically yank or claim rollback.
Check registry state before retrying an ambiguous upload. A source/dependency fix
requires review and an explicit version/partial-publication decision.
