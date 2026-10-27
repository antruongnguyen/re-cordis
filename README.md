# cordis-rust

A Rust reimplementation of [Cordis](https://github.com/cordiverse/cordis) — a
meta-framework for spatiotemporal composability.

> **Status: M0-M6 Rust core delivery and acceptance implemented.**
> The unpublished `cordis-core` has local
> hosting, typed plugins/Registry grouping, scoped effects and diagnostics,
> typed events, validation, restart, parent-owned disposal, typed service
> publication/lookup, and dependency-driven injection convergence.
> It also has isolation boundaries, explicit bound service calls/intercepts,
> typed reflection/hooks, config updates/recovery, and latest-target convergence.
> M6 adds traced/configured logging, error cause/aggregate adapters, weak lifecycle
> metadata, explicit initialization/method injection, type checks, and an end-to-end example.
> This is an unpublished explicit Rust API, not transparent JavaScript
> Proxy/decorator compatibility or a claim of unrestricted TS parity.
> Post-M6 hardening now executes six paired lifecycle scenarios
> against the pinned TypeScript core; see [lifecycle parity](docs/lifecycle-parity.md).
> This includes superseded/latest load failures, abandoned waits, and explicit
> recovery. Those slices needed no production changes or new dependencies.
> The core/parity checkpoint has 152 integration tests and 17 Rustdoc checks.
> Approved GitHub-activity consumers add a snapshot digest, bounded `gh` collector
> and signed loopback webhook journal (12 application tests on Unix, for 164
> workspace integration tests and the same 17 Rustdoc checks).
> See [GitHub activity consumers](docs/github-activity.md).
> Application-only dependencies do not extend the production core.
> Next roadmap targets are timer, console logger, then loader; no satellite is
> implemented yet. See [the ordered roadmap](docs/todo.md#next-implementation-targets-satellites).
> The concept map below remains the framework target, not a shipped feature list.

## User guides and API documentation

Start with the task-oriented documentation in
**[English](docs/en/README.md)** or **[Tiếng Việt](docs/vi/README.md)**.
Both include setup, runnable examples, lifecycle/resource guidance, and a
grouped API reference. The [documentation map](docs/README.md) separates these
current user docs from historical milestone evidence.

For AI-assisted application development, give your coding agent
[`docs/llms.txt`](docs/llms.txt) and access to this checkout. Exact API signatures
are available locally through `cargo doc -p cordis-core --no-deps --locked`;
the crate is not published to crates.io/docs.rs.

## Why

Cordis is a TypeScript meta-framework for building long-lived, plugin-oriented
applications: components (plugins) can load and unload at runtime, services can
appear and disappear, configuration can change, and resources must still be
cleaned up deterministically.

This project reimplements that model in Rust, with the **TypeScript `cordis` as
the source of truth**. A separate, independent Rust port already exists
([dshbox/cordis-rs](https://github.com/dshbox/cordis-rs)); we treat it as a
reference point for comparison, not as a target to match. The aim here is to
understand the original framework deeply enough to re-express its semantics
faithfully in idiomatic Rust.

## Concept map

These are the core concepts of the original Cordis (`packages/core`) that this
port will mirror. Names and shapes may be adapted to Rust idioms; the semantics
are the contract.

| Concept      | What it does |
|--------------|--------------|
| **Context**  | A cheap, traceable view into one runtime. Exposes the core services and derives scoped child views via `extend` / `isolate` / `intercept`. |
| **Service**  | A typed, named capability provided into the runtime, with isolation and per-context configuration (intercept). |
| **Fiber**    | The lifecycle unit — one plugin instance: config validation, disposables, and a state machine governing load/unload. |
| **Registry** | Plugin registration and dependency (`inject`) resolution. |
| **Reflect**  | The reflection layer powering `get` / `set` / `provide` / `mixin` on a Context. (In TS this is a `Proxy`; in Rust it will be expressed differently — see limitations.) |
| **Events**   | A typed event system with multiple dispatch modes (`emit`, `parallel`, `serial`, `bail`, `waterfall`) and scoped hooks. |
| **Logger**   | Structured, context-aware logging. |

## Comparison with TypeScript Cordis and cordis-rs

Source-reviewed on 2026-10-27 against TypeScript Cordis
[`35fdc969`](https://github.com/cordiverse/cordis/tree/35fdc969e4b6c48c48ed44717c7a018324e4dd88)
(`cordis` `4.0.0-rc.12`) and independent `dshbox/cordis-rs`
[`1754062c`](https://github.com/dshbox/cordis-rs/tree/1754062cef67cfc119fc7a2084c0321a231b9092)
(v3 architecture; core `0.6.1`, facade `0.11.1`). These are revision-specific
comparisons, not claims about latest upstream, relative performance, or complete
cross-project parity. `cordis-rs` is an independent port/redesign, not this
project's compatibility target.

**Star guide:** **★★★** strong fit/support for the row's stated criterion;
**★★☆** useful with material differences; **★☆☆** limited fit/support;
**☆☆☆** not provided; **—** not fairly scored from the available evidence.
These are editorial, source-based ratings, not benchmarks or measured quality
scores. Higher is better **for that row**, not universally. Do not total the
stars: priorities differ and several rows overlap.

### Pros and cons matrix

Scores reflect the priority named in the first column. Each cell retains the
benefit and its constraint or alternative design; fewer stars need not mean
worse engineering.

| Priority / criterion | This project: cordis-rust | Original: TypeScript Cordis | Independent: cordis-rs |
|---|---|---|---|
| JS-style API convenience | **★★☆** Typed keys, callbacks and config contracts; explicit adapters require more application code than JS property/decorator syntax. | **★★★** Native Proxy, class and decorator ergonomics; dynamic access and lifecycle errors still require runtime checks. | **★☆☆** Typed Service/Plugin traits and sealed prepared inputs; applications adopt a redesigned API rather than a direct TS translation. |
| Send-based multi-thread integration | **☆☆☆** Accepts local `Rc` resources and non-Send futures instead; requires explicit Tokio/LocalSet driving and is not a multi-thread API. | **☆☆☆** Natural JavaScript event-loop/Promise execution instead; not a Rust Send/Sync API. | **★★★** Send + Sync Context/Fiber handles and conditionally Send operation futures; introduces synchronization and completion-runtime machinery. |
| Automatic parent/child teardown | **★★★** Parent-owned children and reusable root drains follow the TS model; drains are snapshots, not global shutdown barriers. | **★★★** Parent cascade and reusable root disposal; applications must still await asynchronous cleanup. | **☆☆☆** Independent Fiber lifetimes can outlive their spawn origin instead; applications must arrange collective teardown explicitly. |
| Concurrent independent cleanup | **★★★** Concurrent top-level cleanup avoids imposing a global sequential dependency; sibling completion order is not guaranteed. Composed cleanup is reverse sequential. | **★★★** Concurrent top-level disposal and reverse sequential composed effects; Promise completion must be handled explicitly. | **☆☆☆** Strict sequential reverse-commit, attempt-all cleanup instead gives a fixed order; later cleanups wait for earlier ones. |
| Original callback-error inspection | **★★★** Preserves original Rust error sources and explicit cause/aggregate trees; callers must inspect operation results and cleanup reports. | **★★★** Retains native Error objects and cause/aggregate behavior; JavaScript formatting and reflective error handling do not translate transparently to Rust. | **★☆☆** Operation-specific typed failures retain callback kind/diagnostic text; normalized callback failures do not expose the original error object for downcasting. |
| Ready-to-adopt package breadth | **★☆☆** Small unpublished core, MSRV 1.85, plus application examples; no stable API or timer/loader satellites yet. | **★★★** Broader timer, loader, console-logger and other JS packages; the reviewed core is an unstable release candidate. | **★★☆** Versioned facade, core, timer and declarative loader crates; requires Rust 1.88 and remains pre-1.0. |
| Compatibility evidence (not ranked) | **—** Six executed Rust/TS scenarios compare 73 observations; this is bounded evidence, not full upstream-suite parity. | **—** Behavioral specification and original tests; its JavaScript mechanisms remain the reference, not a Rust API prescription. | **—** Own parity ledger and concurrency/type-checking tests; its ledger targets a different, older TS pin, so classifications are not directly interchangeable. |

**Quick choice:** favor TypeScript Cordis for native JS ergonomics and its wider
satellite set; this project for local Rust applications seeking TS-like core
lifecycle semantics; `cordis-rs` for Send-based Rust integration and existing
timer/loader crates when its redesigned contracts suit the application.

### Ported features and concepts matrix

**Implemented** means delivered behavior in this project's explicit Rust API,
not identical JavaScript syntax. **Adapter** identifies an explicit Rust
replacement. **Redesigned** identifies a deliberate `cordis-rs` contract change;
**deferred** does not imply a missing requirement in the completed core scope.
Stars rate support for the described TS-derived concept, including useful
explicit Rust equivalents—not syntax identity or full behavioral parity.
The last row instead rates the additional identity-breaking replacement
capability. A deliberate redesign can be preferable despite a lower TS-fit score.

| Feature / concept | Original: TypeScript Cordis | This project: cordis-rust | Independent: cordis-rs |
|---|---|---|---|
| Context derivation and metadata | **★★★** Prototype-derived `extend`, root view, isolation and intercept layers. | **★★★ Implemented:** typed `extend`/metadata, root views and retained scope boundaries. | **★★☆ Redesigned:** orthogonal isolate, Event Scope and intercept axes; no general Context ancestry or metadata-extension API. |
| Plugin identity and Registry | **★★★** Function/class/object plugin shapes; callback-identity grouping and public Registry. | **★★★ Implemented:** reusable `Plugin<C, R>` allocation identity, typed Fiber handles and public Registry controls. | **★★☆ Redesigned:** Plugin trait, prepared wrappers and typed group removal; Registry is private. |
| Fiber states and convergence | **★★★** Pending, Loading, Active, Failed, Unloading, Disposed; latest-target convergence. | **★★★ Implemented:** all six states, coalescing, readiness and committed dependency snapshots. | **★★★** All six states and exact SemanticTarget convergence, using its own transaction/settlement protocol. |
| Parent-owned plugin disposal | **★★★** Spawning registers child disposal as a parent effect. | **★★★ Implemented:** parent unload/disposal joins owned children. | **☆☆☆ Redesigned:** spawn origin is provenance, not parent ownership; no disposal cascade. |
| Reusable root cleanup | **★★★** Root disposal drains resources/children and leaves root usable. | **★★★ Implemented:** reusable `Context::drain` snapshot barriers; runtime destruction is cancellation. | **☆☆☆** Not provided by design; permanent root generation and explicit per-Fiber teardown instead. |
| Validation and config updates | **★★★** Synchronous source-to-resolved validation, deferred while Pending; update hooks and `noSave`. | **★★★ Implemented:** typed validators, retained source/resolved types, update middleware and `no_save`. | **★★☆ Redesigned:** preparation before admission, sealed `PreparedChange`, precommit policy/veto and `UpdateOutcome`. |
| Failed-plugin recovery | **★★★** Provider refresh does not retry Failed; explicit update clears failure. | **★★★ Implemented:** explicit update recovery, obsolete/latest-failure rollback and diagnostics. | **★★☆ Redesigned:** failure parks against a target; changed input/assignment or explicit restart can permit another pass. |
| Services and injection | **★★★** Named publications, checked definition-site access and exact dependency retention. | **★★★ Implemented:** `ServiceKey<T>`, per-Fiber injection, exact-publication retention through consumer cleanup. | **★★☆** Typed Service traits, InjectSpec and exact realm/publication assignments; lookup does not require inject membership. |
| Isolation and intercept configuration | **★★★** Symbol-selected slots, injection boundaries and traced configuration. | **★★★ Implemented:** `IsolationId`, boundary-preserving lookup and explicit typed intercept merging. | **★★☆** Exact ServiceRealm placement and ConfigurableService layers; no ancestor fallback lookup. |
| Definition/use-site tracing and reflection | **★★★** Proxy-bound methods, accessors, mixins, associated/callable services and method injection. | **★★☆ Adapter:** `BoundService`, typed Accessor/ServiceInvoke, initialized plugins and `inject_method`; no transparent Proxy/decorator syntax. | **★☆☆** Explicit Service lookup/configuration, not a Proxy/accessor/mixin/decorator layer. |
| Effects, rollback and introspection | **★★★** Sync/async/generator effects, partial setup rollback, labels and effect trees. | **★★★ Implemented:** disposer composition, iterator/stream acquisition, rollback and `EffectMeta` snapshots. | **★★☆ Redesigned:** generation-owned effect registrations and generation-level rollback; no cleanup-journal introspection. |
| Event dispatch modes | **★★★** `emit`, `parallel`, `serial`, `bail`, `waterfall`. | **★★☆ Implemented/adapter:** same five roles; sync-only bail and separate sync/async waterfall forms. | **★★☆ Redesigned:** awaited `emit`, `emit_parallel`, `query`, `waterfall`; serial/bail collapse into query. |
| Event routing, snapshots and once | **★★★** Explicit filters/global listeners; selected callback snapshots survive removal. | **★★★ Implemented:** typed filters/global listeners, service-realm filters and TS-style selected snapshots. | **★★☆ Redesigned:** ancestor-or-self Event Scope routing and invocation-time occurrence claims; removal can skip unclaimed delivery. |
| Core logging | **★★★** Contextual names, exporter levels, default 1000-record buffer, weak Fiber metadata and error flattening. | **★★☆ Implemented/adapter:** buffer, scoped exporters, weak metadata and explicit ErrorValue trees; Rust formatting replaces JS formatters. | **★★☆** Named channels, scoped exporters and opt-in BufferExporter; no default retained log history. |
| Internal hooks and observations | **★★★** Internal events for plugin/status/service/config/update/get/set/listener/dispatch. | **★★☆ Adapter:** public typed `hooks` payloads and explicit update adapters, with documented observer-error safety differences. | **★★☆ Redesigned:** typed postcommit runtime observations and a separate, non-dispatchable precommit update pipeline. |
| Timer, console exporter, loader and HMR packages | **★★★** Separate JS satellite packages, including timer, loader and HMR. | **☆☆☆ Deferred:** only core is a workspace package; GitHub consumers are examples, not satellites. | **★★☆** Timer and immutable declarative loader crates plus facade; not a native hot-loading or full JS satellite port. |
| Identity-breaking Fiber replacement | **☆☆☆** No corresponding lifecycle operation at the reviewed TS pin. | **☆☆☆** Not selected; restart/update preserve Fiber identity. | **★★★** Additional `era_swap` operation creates a successor with a fresh Fiber identity. |

Our contracts and evidence are detailed in the [M6 audit](docs/m6.md),
[lifecycle comparisons](docs/lifecycle-parity.md), and
[source-backed design review](docs/implementation-plan.md). The independent
port's [architecture](https://github.com/dshbox/cordis-rs/blob/1754062cef67cfc119fc7a2084c0321a231b9092/docs/v3-architecture.md)
and [parity ledger](https://github.com/dshbox/cordis-rs/blob/1754062cef67cfc119fc7a2084c0321a231b9092/docs/v3-upstream-parity-ledger.md)
explain its intentional redesign; its ledger uses TS `8cc9e33f`, not our
`35fdc969` baseline. Reference-port comparisons here are source/documentation
reviews, not a claim that we reran its acceptance suite.

## Goals

- Faithfully mirror the **semantics** of TypeScript Cordis core.
- Idiomatic, safe Rust — no attempt to transliterate JS-only mechanisms.
- Rust edition **2024** as the initial baseline; no older edition.
- Deterministic lifecycle and resource cleanup.
- A clear mapping, documented, from each TS concept to its Rust expression.

## Non-goals (for now)

- Matching the exact API surface of `dshbox/cordis-rs`.
- Feature parity with every satellite Cordis package before the core is solid.
- A stable public API — this will change freely until the core is proven.

## Workspace and planned layout

A Cargo workspace mirroring the original `cordis` packages, built up in stages:

```
crates/
  cordis-core      # Plugins/Fibers/Context/services/injection/effects/events/diagnostics
  cordis-loader    # (later) plugin loading
  cordis-timer     # (later) timer service
  cordis-logger    # (later) console logger
  ...
```

Only `cordis-core` currently exists in the workspace. The next implementation
targets are `cordis-timer`, `cordis-logger` (console exporter), then
`cordis-loader`; other satellites follow demonstrated needs. The
[ordered roadmap](docs/todo.md#next-implementation-targets-satellites) records
per-slice source reviews, design gates, tests and bilingual documentation
requirements. These targets are not delivered crates or a stable API promise.
The [implementation plan](docs/implementation-plan.md) preserves the core's
source review, Rust design gates, milestone dependencies and acceptance criteria.

## Build and develop

Requires Rust **1.85 or newer**, edition **2024**. The root workspace uses
Cargo resolver 3. From the repository root:

```sh
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo run -p cordis-core --example lifecycle --locked
```

See [M6 acceptance and compatibility](docs/m6.md), [M5 scope/update APIs](docs/m5.md),
[M4 service APIs](docs/m4.md), [M3 lifecycle APIs](docs/m3.md),
[M2 APIs](docs/m2.md), and the examples in the
[`cordis-core` crate docs](crates/cordis-core/src/lib.rs). The runtime must be
explicitly driven from Tokio; Context values do not keep their host alive.
CI covers Rust 1.85.0 and stable, including the independent M0 experiments.

The Rust lifecycle-parity fixtures run in the normal workspace suite. To also
execute their TypeScript counterparts, prepare the isolated pinned checkout
and its Yarn dependencies as described in
[lifecycle parity](docs/lifecycle-parity.md), then run:

```sh
python3 tests/parity/run.py
```

The first application consumer converts a bounded GitHub activity snapshot to
Markdown:

```sh
cargo run -p cordis-core --example github_digest --locked -- \
  crates/cordis-core/tests/fixtures/github_snapshot.json /tmp/cordis-report.md
```

See [the consumer contract](docs/github-activity.md) for the format, scoped
ownership, reconfiguration/recovery, and output/cancellation limitations.

## M0 experiments

Run the unpublished design spikes independently of the production workspace:

```sh
cargo test --manifest-path spikes/m0/Cargo.toml --locked
```

See [M0 results](docs/m0.md) for evidence, selected design directions, and limits.

## Documentation

- [`docs/README.md`](docs/README.md) — documentation map, language selection,
  source precedence and repeatable documentation checks.
- [`docs/en/README.md`](docs/en/README.md) /
  [`docs/vi/README.md`](docs/vi/README.md) — English/Vietnamese user guides,
  API reference and shared terminology.
- [`docs/llms.txt`](docs/llms.txt) — concise retrieval index and framework
  constraints for AI-assisted application development.
- [`docs/github-activity.md`](docs/github-activity.md) — approved GitHub activity
  consumers, executable commands, application contracts and acceptance.
- [`docs/lifecycle-parity.md`](docs/lifecycle-parity.md) — six executable
  Rust/TypeScript lifecycle/failure/recovery comparisons, boundaries, and reproduction.
- [`docs/m6.md`](docs/m6.md) — logger/error APIs, explicit injection adapters, executable example, and parity/deviation audit.
- [`docs/m5.md`](docs/m5.md) — isolation, bound calls, typed reflection/hooks, and configuration convergence.
- [`docs/m4.md`](docs/m4.md) — typed services, checked/direct lookup, injection convergence, and exact-provider retention.
- [`docs/m3.md`](docs/m3.md) — typed plugins, Registry, scoped resources, and base lifecycle.
- [`docs/m1.md`](docs/m1.md) — supported foundation APIs, ownership, and limits.
- [`docs/m2.md`](docs/m2.md) — effect/event/diagnostic APIs and source-case coverage.
- [`docs/m0.md`](docs/m0.md) — executable design-spike findings and M1 handoff.
- [`docs/m0-cases.md`](docs/m0-cases.md) — pinned case-by-case behavioral checklist.
- [`docs/implementation-plan.md`](docs/implementation-plan.md) — source-backed
  core implementation plan, reference comparison, and validation strategy.
- [`docs/todo.md`](docs/todo.md) — the porting roadmap.
- [`docs/decision.md`](docs/decision.md) — decisions and their rationale.
- [`docs/limitation.md`](docs/limitation.md) — known gaps and open tensions.
- [`AGENTS.md`](AGENTS.md) — conventions for contributors and agents.
- [`CHANGELOG.md`](CHANGELOG.md) — notable changes.

## References

- [cordiverse/cordis](https://github.com/cordiverse/cordis) — the original
  TypeScript framework (the spec).
- [dshbox/cordis-rs](https://github.com/dshbox/cordis-rs) — an independent prior
  Rust port (reference only).

## License

MIT; see [LICENSE](LICENSE). The TypeScript upstream is also MIT-licensed.
