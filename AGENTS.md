# AGENTS.md

Instructions for agents (and humans) working in this repository.

## What this project is

`re-cordis` is a from-scratch Rust reimplementation of the
[Cordis](https://github.com/cordiverse/cordis) meta-framework.

**Status: M0-M6 Rust core delivery and acceptance implemented.**
The completed post-M6 lifecycle-parity slices add six executed Rust/TypeScript
scenarios (152 Rust integration tests, 17 Rustdoc checks), including bounded
failure/recovery evidence. See `docs/lifecycle-parity.md` for reproduction and
evidence boundaries.
The approved first-consumer scope is implemented: snapshot digest, bounded `gh`
collector and signed loopback webhook journal (12 additional application tests
on Unix). See `docs/github-activity.md`. Application dependencies are dev-only.
Timer, console logger and loader/structural groups are implemented; include,
HMR and create remain future targets. The **`re-cordis-timer`** satellite is
delivered (unpublished `crates/re-cordis-timer`: owner-scoped timeout, sleep,
interval/stream, throttle and debounce over core effects, via a sealed
`TimerExt` trait; the `tokio` `time`/`macros` features live in the timer crate,
not the core; 48 deterministic integration tests and three Rustdoc checks).
The T0–T5 repair adds eager checked deadlines, hosted callbacks, bounded stream
state and joined owner cancellation; six focused core task-bridge tests cover
the minimal Context::task/TaskControl/Context::invoke_callback seam. The unpublished
**`re-cordis-logger`** console adapter is implemented (owned scoped writers,
plain/ANSI/Auto policy, UTC record rendering, visible I/O errors; 21 tests).
Console C0–C3 is independently accepted; see its logger work package. The
unpublished `re-cordis-loader` owns typed catalogs, structural groups, bounded
JSON transport, source reconciliation and joined rebuilds; it performs no file
persistence, JavaScript imports/eval or watching. Timer repair verification is recorded in
`docs/plans/2026-11-02-cordis-completion/evidence/timer-acceptance.md`; see
`docs/plans/2026-11-02-cordis-completion/README.md` for the proposed core/all-satellite
work packages and scheduling gates. See `docs/todo.md` for source review,
per-slice design gates and documentation acceptance, and
`docs/superpowers/specs`/`docs/superpowers/plans` for the timer design and plan.
The root Rust 2024 workspace contains the unpublished `crates/re-cordis-core`,
`crates/re-cordis-timer`, `crates/re-cordis-logger` and `crates/re-cordis-loader` (Rust 1.85 minimum).
Use `cargo test --workspace`.
Acceptance, logging, examples and compatibility restrictions are in `docs/m6.md`.
Isolation/bound-call/update contracts are in `docs/m5.md`;
publication/access/dependency-retention contracts are in `docs/m4.md`;
Plugin/Registry/scoped-resource contracts are in `docs/m3.md`, with effects,
events, and diagnostics in `docs/m2.md`. This is an explicit Rust API, not
transparent JavaScript Proxy/decorator compatibility. The isolated `spikes/m0`
package remains research evidence; run it with its own `--manifest-path`.

## Source of truth

The **TypeScript `cordis`** project is the reference for Cordis concepts and
intended behavior. When interpreting upstream behavior, its implementation
wins over the prior Rust port. Rust API and execution choices follow the
documented Rust contracts and deliberate adaptations in `docs/decision.md`.

Two reference checkouts live under `local/` (git-ignored, cloned locally):

- `local/cordis` — the original TypeScript Cordis. **This is the spec.**
  Core concepts live in `local/cordis/packages/core/src/`.
- `local/cordis-rs` — an independent prior Rust port
  ([dshbox/cordis-rs](https://github.com/dshbox/cordis-rs)). **Reference only.**
  Useful for seeing how others solved a problem in Rust; it is not a target and
  may diverge from upstream. Do not treat its decisions as authoritative.

We mirror the *semantics* of TS Cordis, not its mechanisms. The original relies
on JavaScript-only features (Proxy-based reflection, prototype-chain
inheritance, callable objects) that have no direct Rust equivalent. See
`docs/limitation.md`.

## Implementation priorities

The user's 2026-11-03 direction is to deliver a useful, idiomatic Rust Cordis.
Prioritize working features, application usability and language suitability.
Do not translate TypeScript byte-for-byte or recreate JS machinery for syntax
parity. Use ownership/borrowing, typed keys, enums, Result, traits/generics and
compile-time constraints where they simplify the feature. Use RAII for
synchronous release; asynchronous teardown still needs explicit completion.
Preserve current hosting contracts unless a concrete feature requires change.

For each selected feature, briefly establish its Rust contract, implement it
with meaningful tests and a usable example, then review the working result.
Keep review effort proportional to risk; do not make exhaustive upstream
assertion accounting a prerequisite for every feature. Reuse accepted B0
evidence and investigate gaps only when relevant to the feature being built.
Record deliberate adaptations, unavailable capabilities and their user impact
in both `docs/decision.md` and `docs/limitation.md`. Distinguish language/runtime
constraints, unsuitable emulation, deferred work and missing evidence.

## Keep the docs current

Document the decisions needed to implement a feature, and keep documentation
with delivery. Whenever you make a meaningful change, update the relevant doc
in the same change:

- `README.md` — project front door: intent, concept map, planned layout.
- `CHANGELOG.md` — notable changes (Keep a Changelog style).
- `docs/todo.md` — the porting roadmap.
- `docs/decision.md` — decisions made and why.
- `docs/limitation.md` — known gaps and unresolved tensions.
- `docs/en/` and `docs/vi/` — matching current user guides and API reference.
- `docs/llms.txt` — concise AI-agent reading paths and framework constraints.

Run `python3 -B tests/docs/test_check.py` and the documentation checker for
explicit stable/1.85.0 toolchains after guide/API changes; see `docs/README.md`.
Its current export/snippet checks cover core, timer, logger and loader; extend them for each
new delivered satellite.

## Tooling conventions

- After implementing and verifying each milestone, commit its code and related
  documentation on the current branch. Do not push unless explicitly requested.
- Never include a `Co-authored-by` trailer in git commit messages.
- Use `rg` (ripgrep) instead of `grep` for searching.
- If a `.codegraph/` directory appears at the repo root, prefer CodeGraph
  (`codegraph explore ...`) over grep/find for understanding code. Indexing is
  the maintainer's decision — do not run it unprompted.

These conventions are intentionally lean and will grow as the project does.
