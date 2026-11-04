# Decisions

A lightweight log of decisions made for this project and the reasoning behind
them. Newest first. This is intentionally not a heavy ADR process; it records
*what we chose and why* so the choices aren't re-litigated silently.

## 2026-11-05 — Separate accepted behavior from distribution readiness

R1 recommends local/Git checkout consumption only while preparing a concrete
seven-package 0.1.0-alpha.1 proposal. These versions and registry changes are
proposals, not releases or approval to publish. Keep 0.0.0/publish=false.
The [readiness report](release-readiness.md) derives order from actual Cargo
normal dependencies and distinguishes HMR's dev-only logger edge.

Package inspection found omitted MIT notices and six absent crate READMEs.
Add exact root notices and minimal package-specific onboarding. Preserve the
same MIT text in the generated starter's existing README, retaining its five
files, source and manifest. This corrects redistribution documentation without
changing API/feature/dependency behavior or adding a template variant.

No facade has a demonstrated consumer benefit: accepted Create/R0 applications
already compose explicit crates, and Group belongs to loader. The proposed
Create release remains an offline tool requiring a compatible complete checkout.
Path sources are mutable despite Cargo.lock; preserve checkout revision and
location. A registry-backed template requires its own post-publication decision.

Use explicit stable/1.85.0 package/docs checks and reuse R0 execution where
source/manifest/lock inputs are unchanged. Proposed runtime support is limited
to the tested Apple Silicon macOS host; broader platform support, semver
stability and fresh registry dependency resolution are separate gates.
See [limitations](limitation.md#distribution-readiness).

## 2026-11-04 — Verify the complete offline application through shared code

R0 composes accepted core, timer, console logger, loader/groups, Include, HMR
and offline Create without redesigning their production APIs. One application
module under the HMR example is imported by its integration tests and copied
unchanged into the actual Create-generated external test fixture. The HMR crate
gains only the necessary application dev dependency; no facade crate is needed.

Include's explicit JSON edits and move_entry are the persistence boundary.
Moving a provider across isolated groups rebuilds its Fiber while preserving
EntryId. The old consumer's committed dependency retains that exact provider
through gated cleanup. Moving the consumer afterward establishes a new binding
in the destination realm; it does not mutate an old Context's scope.

Use a synthetic watcher and paused time for deterministic admission/cancellation
oracles; real registered-file polling remains bounded supplemental evidence.
ReloadPlan replaces already-linked candidates through the same Include-managed
instances. Error observations follow the operation/report/watcher-history/Runtime
contracts, and are not added together as distinct failures.

Independent R0-R1 review reproduced an application shutdown race on both
toolchains: canceling Include before the watcher joins can stop workers of an
admitted valid refresh. Orderly shutdown must join the watcher while Include
remains live, then dispose/join Include and the application owner. Forced
ancestor cancellation remains separately supported and tested with its original
retained cancellation cause. No framework change is needed for this ordering.

The documentation checker first verifies the unmodified generated basic app,
then extends only its temporary output with explicit local dependencies and
the shared R0 program/tests. Keep production templates unchanged: acceptance
does not introduce a new supported template product. Process execution uses
argument arrays; both explicit stable/1.85.0 toolchains build, test and run the
external Cargo application offline.

R0's concrete gated shutdown reproducer also found a stale loader group snapshot:
Context::task cancellation skipped the convergence worker's running=false tail.
Owner cleanup now clears that bookkeeping flag when closing entries; the native
regression separately verifies request versus join and actual resource release.
See [the narrow correction](plans/2026-11-02-cordis-completion/evidence/r0-loader-correction.md).
No core or public API redesign is involved.

Source reconciliation carries accepted prerequisite ledgers and preserves B0's
remaining gaps. New native composition evidence does not imply a new whole-program
TypeScript trace or Windows/Linux runtime evidence. See the
[integration guide](en/guides/system-lifecycle.md), [execution plan](plans/2026-11-02-cordis-completion/evidence/r0-plan.md)
and [source dispositions](plans/2026-11-02-cordis-completion/evidence/r0-source-map.md).

## 2026-11-03 — Deliver Cordis features through idiomatic Rust contracts

The user reaffirmed that this project implements Cordis in Rust, prioritizing
features, usability and feasibility in the target language. TypeScript remains
the reference for upstream intent and behavior; its syntax, object model and
every incidental runtime observation are not mandatory Rust designs. Choose
equivalent useful capabilities and document deliberate differences explicitly.

Use Rust's ownership and borrowing, typed keys, traits/generics, enums, Result
and compile-time constraints where they make the implementation simpler and
safer. A one-use continuation rejected at compile time is a valid Rust contract;
it need not reproduce a JavaScript runtime exception. Standard Rust interfaces
such as Write, Iterator and Stream are candidates when they fit the feature.
RAII can release synchronous resources; it does not replace joined async cleanup.
This direction does not itself promise Send/Sync hosting or measured performance.

Implement one usable feature or satellite slice with its necessary tests and
example, then review the working result for behavior, ergonomics, resource
safety and integration. Upfront design should settle the contracts needed for
that implementation, rather than expand into another exhaustive evidence audit.
Judge delivery against the documented Rust feature contract, with targeted TS
comparisons where they resolve meaningful uncertainty. Preserve relevant
regression and toolchain checks. B0 stays accepted; its 147 evidence gaps are
not an automatic implementation queue or a blanket satellite prerequisite.

For an adaptation or unavailable capability, record the upstream purpose,
chosen Rust approach, user-visible difference, reason and supported alternative
in this file and [limitations](limitation.md#how-to-classify-rust-differences).
Distinguish an inherent constraint from costly/unsuitable emulation, deferred
implementation and missing evidence. A lost feature must remain visible;
calling an API idiomatic does not prove equivalent behavior.

**Why:** Progress means useful Cordis capabilities delivered with clear Rust
contracts. This direction governs future execution of the existing plans and
supersedes interpretations requiring mechanical translation or B0-style
accounting before each feature. Accepted historical evidence remains intact.

## 2026-11-02 — Plan complete coverage before scheduling implementation

The user requested careful planning for remaining core fixes and all upstream
satellites, saved under docs, with implementation scheduling afterward. The
[completion program](plans/2026-11-02-cordis-completion/README.md) separates core
evidence, timer repair, console logger, loader, group, include, HMR and create.
It supplies decision gates, file boundaries, behavioral acceptance and a
schedule dependency graph. These are proposed work packages; this planning
change does not approve their future APIs, dependencies, exclusions or releases.

Source inspection found timer admission/ownership/terminal-state gaps that
need executable regressions before fixes. The recommended first sequence is
baseline, timer hardening, console logger, then loader. Upstream group is a
loader re-export; group behavior belongs in loader even if packaging later
adds a facade. Include reconciliation and HMR replacement need distinct slices.

**Why:** Cover every package without equating a minimal slice with full parity,
freezing unknown contracts, or assigning dates before prerequisites are known.
The seven satellite packages contain 323 syntactic test declaration lines;
parameter variants and supported Rust dispositions remain gate responsibilities.

## 2026-10-27 — Order satellite targets without claiming implementation

The user selected roadmap planning for `re-cordis-timer`, `re-cordis-logger`,
`re-cordis-loader` and other satellites. Use that order for the next implementation
slices, starting with pinned TypeScript timer source/tests. Each slice still
requires an explicit context map, API/dependency review, exclusions and measurable
acceptance criteria before coding. This documentation change does not add a
crate, freeze those APIs or authorize implementing every target at once.

Timer reuses scoped effects and local hosting; its source-case inventory must
include timeout/interval overloads, iterator cancellation, throttle/debounce and
deprecated aliases, even if an initial slice documents exclusions. The Rust
`re-cordis-logger` target is a console/export adapter corresponding to TS
`logger-console`, not a replacement for existing core logging. Loader begins
with explicit Rust registration/configuration contracts, not assumed native
hot reload. Logger/timer are not automatic loader dependencies.

Each delivered satellite updates both language trees, the AI-agent index,
README status/comparison ratings and its executable documentation checks.
Existing local/non-Send hosting, parent ownership, exact provider retention,
explicit recovery and cancellation boundaries remain unless a demonstrated
requirement justifies a separately reviewed change. Do not adopt independent
cordis-rs redesigns as TypeScript parity.

**Why:** Core delivery and the first consumers are complete; the next framework
work should progress through bounded satellites rather than more application
features or indefinite parity expansion. See [the roadmap](todo.md).

## 2026-10-27 — Separate current user documentation from milestone history

Add task-oriented guides and a grouped API reference in matching `docs/en/`
and `docs/vi/` trees, with counterpart links and a shared glossary. Translate
prose, preserving API identifiers and Rust/TOML/shell examples. Current source
and generated Rustdoc own exact signatures; milestone documents retain their
historical scope and evidence rather than becoming the onboarding sequence.

Use plain Markdown and a concise `docs/llms.txt` retrieval index, not a new
documentation site, dependency, generated full-book copy, or publishing system.
The agent index identifies local paths, constraints and reading routes; it is
not repository policy or authorization to perform network/remote operations.

An opt-in standard-library Python check verifies local links/anchors, bilingual
code/navigation alignment, public export/hook coverage, and compiles/runs
complete guide snippets with explicitly selected MSRV/stable tools. It reuses
locked offline Cargo artifacts and leaves production code and CI unchanged.

**Why:** Application authors need current instructions without interpreting
historical M1-M6 gaps. Equivalent language coverage and executable examples
reduce drift, while Rustdoc avoids hand-maintained copies of complex signatures.
See [the documentation map](README.md).

## 2026-10-23 — Select bounded GitHub activity consumers

After workflow selection and explicit scope/dependency approval, implement a
snapshot digest, `gh`-based collector and signed loopback webhook journal,
in that order. Keep them executable application examples, exercising the same
implementation from integration tests. Reuse typed plugins/injection, explicit
updates and scoped teardown; no production core change is currently required.
Only actually shared application contracts belong in a private example module.

JSON/network/cryptographic dependencies and expanded Tokio features belong to
dev-dependencies, not the production core. Use the installed `gh` transport and
its existing authentication rather than add an HTTP client. Do not create
credentials, GitHub webhooks or public ingress as a side effect of this work.

The first slice validates bounded snapshots and generates deterministic escaped
Markdown via same-directory temporary write/flush/rename. Destination replacement
uses existing dependency convergence; Failed processors need explicit update.
CLI execution is one-shot; live reconfiguration is a programmatic example API.
See [github-activity.md](github-activity.md).

The collector uses two bounded direct `gh` processes and publishes only a
complete validated snapshot. One injecting consumer owns and joins its periodic
worker; in-flight collection completes before provider replacement or shutdown.
Initial failure follows Failed/explicit-update recovery. Operational poll errors
are visible but leave the periodic worker running for its next scheduled attempt.
CLI configuration is startup-only; typed handles expose live updates in tests.
Unix subprocess fixtures use FIFO gates and a controlled clock, not sleeps or
live authenticated services.

The webhook journal uses library HMAC-SHA256 verification before decoding,
Axum extraction/routing, and a directly hosted Hyper HTTP/1 loop. Its local
JoinSet owns every connection and bounds header reads; this avoids hiding
connection tasks in the outer Tokio executor beyond Cordis host ownership.
A bounded Send-message channel bridges HTTP handlers to the local, non-Send
injected file writer. Success means append/flush before 204, not crash durability.
Writer errors poison the sink and reject later requests until reconfiguration.
Signed local replay is evidence only for the receiver, not real GitHub delivery.

Full acceptance checks actual spawned child identity rather than assuming a
fixture can write a PID file before the query deadline under parallel load.
Environment-secret errors deliberately discard the supplied value instead of
forwarding `VarError::NotUnicode`, whose Debug representation would expose it.
The completed consumer acceptance retains both compiler matrices and the
unchanged six paired scenarios; it adds no live GitHub or broader parity claim.

**Why:** The user identified repository activity as useful, providing a concrete
consumer instead of indefinite parity expansion. The bounded applications can
demonstrate actual I/O while preserving local hosting and resource contracts.

## 2026-10-23 — Bound failure/recovery parity to three additional compositions

Following explicit scope approval, extend the existing lifecycle fixtures and
runner rather than introduce a new harness. Gate superseded failure rollback,
latest failure rollback with multiple retained waiters and an abandoned update,
and explicit asynchronous recovery after a Failed consumer ignores provider
replacement. Both implementations produce the same 36 new observations;
the original 37 remain unchanged. Each actual load failure has one diagnostic,
rollback completes before waiter settlement, and original per-language error
sources remain identifiable rather than being reduced to comparable strings.

Rust uses a test-only Error adapter retaining the original Rc error as its
source; TS checks the original Error object in waiters and logger arguments.
Cross-language traces normalize only causal events and checkpoint outcomes,
not error allocations, stack strings, transient scheduling, or enum ordinals.
The comparisons demonstrate no production mismatch, so core code, APIs and
dependencies stay unchanged.

**Why:** These paths already had Rust-only coverage; the missing evidence was
paired execution. This completes the approved bounded slice, not every upstream
failure/recovery protocol or a preassigned M7. Prefer concrete consuming
application needs before selecting more framework work. See
[lifecycle-parity.md](lifecycle-parity.md) for reproduction and limits.

## 2026-10-23 — Harden lifecycle evidence before choosing a satellite

The approved post-M6 scope is three executable Rust/TypeScript lifecycle
comparisons, not a preassigned M7 package. Exercise concurrent provider/config
updates, latest-target coalescing during gated cleanup, and Registry-removed
consumer retention with terminal disposal. Keep fixtures small, use the
existing upstream Yarn/Vitest setup at the exact source pin, and compare shared
plain-text observations without adding Rust dependencies or a generic harness.

The comparisons confirm existing behavior, so production code stays unchanged.
Both implementations allow consumer update waiters to settle Pending while
their provider is unavailable. Provider arrival starts a distinct lifecycle
pass; an explicit consumer readiness wait joins that pass. A gate holds the
latest consumer load to prove this distinction. Do not "fix" this into a
runtime-wide convergence barrier.

**Why:** No concrete application need selects timer, console logging or loader.
M6 explicitly had source-backed rather than differential evidence. These
three executable checks strengthen that evidence while preserving documented
ownership/cancellation and explicit-adapter contracts. See
[lifecycle-parity.md](lifecycle-parity.md) for the exact scope and reproduction.

## 2026-10-23 — M6 completes explicit Rust logging and acceptance

Logger acquisition caches explicit name, then use-site logger intercept, then
the normalized definition-Fiber name. Nested bound calls restore outer names.
Malformed typed intercepts remain an error in the handle and fail logging before
formatting/export, rather than falling back silently. Records retain stable
identity plus a weak current lifecycle snapshot, never a strong Fiber.

Use finite owned `ErrorValue` trees to represent JS cause/aggregate metadata.
Nonempty aggregates flatten recursively, empty aggregates log themselves, and
causes precede their outer error. Records retain the exact error allocation.
Rust Display/Debug and exporter closures replace JS printf/formatter dictionaries;
per-line UTF-16 budgets and the pinned ANSI palettes have measured tests.
Never split a Rust Unicode scalar merely to mimic a lone JS surrogate.
Default name tokenization follows cosmokit 1.8.1's source (not a guessed regex).

`Plugin::initialized` explicitly awaits an initializer's acquired disposer.
`Context::inject_method` explicitly binds an Rc target to a dependency-owned
child, replacing decorator/constructor discovery without proc macros.
Forwarding an already-normalized callback failure preserves its diagnostic
claim, so failed initializers report once.

**Why:** Native Rust types cannot transparently implement JS property/prototype
reflection or inspect arbitrary Error properties. Explicit typed adapters keep
the useful lifecycle/configuration contracts while making representation
restrictions testable and honest. The same production example runs standalone
and under integration tests. See [M6 audit](m6.md), not a blanket TS parity claim.

## 2026-10-23 — M5 uses explicit bound views, not transparent Rust pointers

Keep M4 `Rc<T>` reads unchanged. Add `BoundService<T>` with an explicit method
adapter receiving the service and traced Context. Checked authority follows
the publication's definition; realm/intercept/metadata/effect ownership follows
the use site. Nested calls retain separate caller and definition views, with
one flattened shadow. Associated access uses the original accessing code's
authority, not the service's own injections. Callable/holder extensions and
accessor/mixin receivers have explicit typed adapters.

Isolation slots have opaque allocation identity and retained per-name boundary
history. Reusing a label still stops inherited inject authority; slots have no
parent fallback. Publications remember their exact selected slot independently
of Context derivation. Cross-name shared slots use checked contracts.

**Why:** A transparent `Rc<T>` cannot rebind arbitrary Rust methods. Explicit
adapters preserve existing callers and avoid Proxy simulation or proc macros.
Notifications also refresh alias declarations selecting the changed slot:
this is a deliberate safety strengthening over upstream's name-only notify,
preventing aliased consumers from trapping a withdrawing publication.

## 2026-10-23 — M5 preserves source validation timing and owns update continuations

Retain source config separately from validated apply input. Loaded updates
validate before commit/source mutation or middleware; Pending updates defer
validation. Commit hooks see source; update hooks see resolved input. noSave
suppresses only commit notification. Local update hooks precede ordinary global
hooks, while prepended globals precede the local router; all remain scoped
effects. Calling the owned update continuation starts restart now. Veto retains
the new source without interrupting in-flight apply. Recovery clears failure
only through update, and every waiter follows the latest lifecycle pass.

Typed `hooks` values use checked heterogeneous payloads rather than unchecked
downcasts. Config/get/set and listener replacement preserve their explicit
contracts. Observer failures are diagnosed without stopping cleanup: unlike
upstream's throwing observer path, an observer cannot strand retained providers.
Duplicate propagation of one callback failure shares a diagnostic claim.

Owner-level weak acquisition stop flags prevent advancing an obsolete effect
stream while still collecting its in-flight final disposer. They do not abort
native apply futures or retain the host. This closes the update/generator race
using the existing setup and cleanup implementations.

See [M5 contracts and evidence](m5.md); M6 owns logger/type/example acceptance.

## 2026-10-23 — M4 separates service slots, publications, and committed loads

Use explicit `ServiceKey<T>` named slots with checked internal type erasure,
opaque `PublicationId` occurrences, and Fiber-owned publication effects.
`set` replaces only the value: it preserves the occurrence and does not notify
consumers. Withdrawal removes only the exact current occurrence before waiting;
republication always creates a new occurrence, even for the same `Rc` value.
Checked access walks own provisions and the nearest declaring ancestor's
committed snapshot. Root access and explicit direct lookups remain distinct.
Captured plugin Contexts do not acquire root authority when used from root.

Injection lists are captured per Fiber, independently of shared Plugin identity.
Resolve affected candidates on service notifications, not on set or arbitrary
restart. A lifecycle target combines the restart revision and exact candidate
identities. Commit and retain its dependencies before validation/apply, and
release only after setup, rollback, children, and resource cleanup finish.
Missing dependencies remain Pending without validation. Failed Fibers do not
retry on refresh; config-update recovery remains M5.

**Why:** Value equality cannot detect replacement or safely identify the load
whose teardown must finish. Per-name resolution preserves upstream readiness
check timing without adding config/intercept machinery prematurely.

## 2026-10-23 — Provider retention is independent of Registry residency

Each publication tracks exact dependent Fiber identities until their committed
load releases them. Provider owners retain current and already-withdrawn
publications and wait for their dependents before draining resources/listeners.
Individual withdrawal waits only for its own occurrence, not the provider's
other publications or a consumer's replacement load. Self-provisions do not add
a self-dependency.

Whole-journal drains temporarily stop the owner's services admitting consumers,
including root-owned publications, while remaining reusable snapshot drains.
Unrelated owner cleanup and independent sibling effects stay concurrent;
composition still uses the existing reverse sequential effect protocol.
Publication/Context links to owners/runtime are weak. Lifecycle drains release
their temporary executor handle before awaiting cleanup, so host destruction
is cancellation rather than a host/task reference cycle.

**Why:** Removed consumers still need their providers, and transitive teardown
must preserve final service calls/events. Scanning only the Registry, waiting for
a consumer's latest ready barrier, or serializing every root effect is incorrect.
See [M4 contracts and evidence](m4.md). Full isolation/use-site tracing and
internal service/status hook protocols remain M5/M6.

## 2026-10-23 — M3 separates resource ownership from runtime residency

Each Fiber owns its own cleanup journal and observable state. Registry groups
retain live/failed instances; Context and registration tokens retain weak
runtime/owner links. A child's disposal is a parent effect, removed immediately
by explicit child disposal. Child-to-parent links are weak and used for callback
attribution, not lifetime retention. Root remains permanent and reusable.

**Why:** Existing M2 resources must follow their registering Fiber without a
runtime-to-callback-to-Context cycle or a parent accumulating retired children.
Group retirement/pruning uses exact allocation checks so old teardown cannot
delete a new group of the same reusable declaration. Scoped cleanup still allows
final event dispatch/logging but refuses new terminal-owner registrations.

## 2026-10-23 — Typed plugin declarations and one supervised lifecycle driver

Use reusable `Plugin<C, R>` declarations with per-load synchronous validation
and explicit Context-based acquisitions. Clones share grouping identity; new
declarations do not collapse by Rust callback type or name. Retain typed source
config separately from resolved apply input. Invalid plugin/config shapes are
compile errors; runtime callback errors retain their original Error source.

One in-flight driver coalesces restart revisions; terminal disposal wins over
queued work and waits for in-flight apply/cleanup. Failed instances remain
resident until disposal; restart does not silently recover them. Config update
and recovery remain M5. Self/owning-ancestor lifecycle waits are refused from
attributed callbacks instead of deadlocking their subtree.

**Why:** This preserves the TS parent/resource ownership, validation timing,
and observable failure semantics without JavaScript callable-object discovery
or the prior port's parent-unowned spawn model. Callback definitions are
immutable, so the TS first-spawn metadata snapshot has no mutable counterpart.
Runtime host lifetime remains explicit; Drop is not successful async teardown.
See [M3 contracts and evidence](m3.md).

## 2026-10-23 — Complete root M2 timing with explicit synchronous cleanup

Use one cleanup representation with ready and pending outcomes. Sync manual
chains and all-sync setup rollback run before the request returns. Mixed
composition runs in reverse order up to the first async child, then awaits
that child before advancing; independent root cleanups remain concurrent and
supervised. Expose `Disposer::sync` and `Context::on_dispose_sync` rather than
force-polling arbitrary async futures or treating an async block as sync code.

Expose effect labels and `Context::get_effects()` value-only snapshots.
Transferred effects become nested metadata children in acquisition order;
raw disposers do not add synthetic effect nodes. Async acquisitions update
their existing parent node. Active setup cannot transfer itself or an ancestor
containing it: refuse before removing the root claim. Metadata never owns
callbacks or resources.

**Why:** This closes the remaining root M2 timing/introspection gaps against
`fiber.ts` and `dispose.spec.ts`, without reference cycles or a second cleanup
implementation. M3 still owns plugin disposal; M5 owns internal-update hooks.
See [M2 contracts and evidence](m2.md).

## 2026-10-23 — M2 uses explicit callback roles and owned resource composition

Admit root effect ownership before user setup. Collect acquired disposers from
iterators/streams, transfer existing root registrations into composition, and
await children sequentially in reverse order. Root drains remain concurrent.
Stop stream advancement on disposal, but retain the first/in-flight final
acquisition and await its cleanup. Ready observes async setup and rollback,
not disposal. The completed timing decision above distinguishes immediate
all-sync rollback from async rollback that must continue on the host.

Typed named/symbol events use checked live buckets and callback snapshots.
Global/filter/prepend/once behavior is explicit; removal never erases a callback
already selected. Responders use ControlFlow, while middleware owns one-use
Next/AsyncNext continuations. Bail is synchronous-only and waterfall has
explicit sync/async callback shapes. Duplicate next becomes a compile error.

**Why:** These contracts preserve the proven ownership and dispatch semantics
without JS Proxy/callable-object emulation or accidental occurrence rechecks.
Root introspection and synchronous cleanup timing are now covered by the
decision above; plugin-owned/internal-update cases need their lifecycle owners.
See [M2 APIs and evidence](m2.md), not a blanket upstream parity claim.

## 2026-10-23 — Automatic failures flow through core diagnostics

Keep original callback sources, record structured diagnostics in the default
1000-record chronological buffer, and scope exporters through root cleanup.
Each exporter selects name/default verbosity independently. Attempt all selected
exporters, including on diagnostic failures, without recursively logging broken
exporters through themselves. Preserve sources and stderr fallback reporting.

**Why:** Dropped waiters must not hide callback failures. Attempt-all exporters
are an explicit diagnostic safety strengthening over TS's unguarded export
loop. Full traced naming and JS formatting remain M5/M6 work.

## 2026-10-23 — M1 owns the host; Context is a non-owning view

The root workspace contains only the unpublished `re-cordis-core` crate. Use
Rust 2024, resolver 3, and an initially tested MSRV of Rust 1.85. Commit both
workspace lockfiles for stable/MSRV CI. Adopt MIT licensing for this project.

`Runtime` owns the local task host and root cleanup journal. `Context` and
manual effect tokens hold weak runtime links; metadata layers own only their
values and parent metadata layers. Dropping Context does not tear down work,
and surviving Context values cannot admit work after their host is destroyed.

**Why:** Capturing a derived Context in its own cleanup must not form an
internal strong-reference cycle. Integrated M1 tests prove release both after
drain and on host destruction. Separating typed ancestry from ownership is
the basis for later definition/use-site service views; those are not yet built.

## 2026-10-23 — Root cleanup is eager, explicitly hosted, and supervised

Admit work only inside the owning `Runtime::run` scope. Root drains/manual
claims commit synchronously and return wait-only completion handles. Repeated
drains share one current snapshot; later registrations belong to the next
pass. Root stays reusable. Host destruction cancels work rather than pretending
async Drop succeeded. Refuse nested/spawned/concurrent host driving and
self-drain requests before admission.

**Why:** This preserves the proven M0 local-first direction without unsafe
implicit LocalSet entry or cancellation-prone lazy cleanup. Report every user
failure to stderr and retain its typed error source/report, even if the waiter
is abandoned. Supervise unexpected driver task failures separately from user
callback unwinds; no blanket invariant recovery. Full effects, plugins, and
Logger integration remain M2 onward. See [M1 contracts](m1.md).

## 2026-10-23 — M0 experiments stay outside the production workspace

Keep the disposable, unpublished Rust 2024 experiments under `spikes/m0`,
with their own resolver-3 manifest and lockfile. Their behavioral models are
not `re-cordis-core`. Record the source case inventory and executable findings
in [m0-cases.md](m0-cases.md) and [m0.md](m0.md); production parity remains
unchecked until integrated tests exist.

**Why:** We can prove risky Rust mechanisms before freezing framework APIs
without confusing a deliberately incomplete model with M1's implementation.
The spike passes on Rust 1.85.0. The subsequent M1 decision above confirms the
production MSRV, root workspace, and CI; M0's models stay separate.

## 2026-10-23 — Local-first execution and explicit lifecycle completion

Start M1 with Tokio, an explicitly hosted LocalSet, Rc handles, and local
non-Send futures. Admission and ownership commit happen in synchronous
request methods, which return completion handles; abandoning a waiter does
not cancel started work. Parent Fibers own children, exact publication
bindings retain dependents through cleanup, and root disposal is reusable.

**Why:** The M0 lifecycle probes demonstrate these behaviors without a
multi-thread runtime, executor abstraction, or hidden completion runtime.
Destroying the executor cancels work; applications must await cleanup before
stopping it. Do not advertise executor-independent or Send/Sync guarantees.
M1 must make the host requirement explicit before admitting work and integrate
definition ancestry without strong ownership cycles. See [M0 ceilings](m0.md).

## 2026-10-23 — Explicit typed access and honest sync/async event APIs

Use named typed keys, checked internal downcasts, retained isolation-boundary
history, and bound service calls carrying definition and use views separately.
Prefer typed metadata and explicit methods/extension traits over property or
callable-object emulation. Preserve symbol-selected slots, including cross-name
shared-label aliasing; report incompatible Rust contracts explicitly.

Use ControlFlow for bail/serial and owned FnOnce continuations for waterfall.
An explicit split adapter runs an async listener's prefix synchronously and
schedules its continuation. Restrict bail to sync listeners, refusing mixed
snapshots rather than returning promise-like values. Keep validation synchronous,
with separate typed source/resolved config and structured validation issues.

**Why:** M0 proves these shapes while surfacing Rust's lazy-future and ownership
differences. Single-use-next violations become compile errors; async bail and
automatic async-function prefix execution are deliberately not promised.
These directions are not a frozen API or evidence of complete TS parity.

## 2026-10-22 — Rust edition 2024 baseline

Use Rust edition **2024** for the initial workspace, inherited by every crate,
with Cargo resolver **3**. A newer stable edition can be adopted explicitly;
an older edition does not satisfy the project requirement.

**Why:** This is a new implementation with no legacy-edition compatibility
obligation. Edition 2024 requires Rust 1.85 or later, but the exact supported
compiler minimum will be selected and tested when dependencies and language
features are chosen. The prior port's Rust 1.88 minimum is not ours by default.

## 2026-10-22 — Plan from pinned behavior, not the prior port's architecture

The initial [implementation plan](implementation-plan.md) reviews TypeScript
Cordis at `35fdc969e4b6c48c48ed44717c7a018324e4dd88` and the independent Rust
port at `1754062cef67cfc119fc7a2084c0321a231b9092`. Preserve the TS behavior
of parent-owned children, reusable-root disposal, exact binding retention,
concurrent top-level cleanup, and the five dispatch modes. Runtime ownership,
typed tracing, and callback/config API choices require design spikes before
their public contracts are fixed.

**Why:** The prior port intentionally diverges in these areas and its parity
ledger uses an older TS revision. Its implementation is useful evidence for
Rust engineering, not authority to silently narrow this project's semantics.

## 2026-10-22 — TypeScript `cordis` is the source of truth

The original [cordiverse/cordis](https://github.com/cordiverse/cordis)
(TypeScript) is the specification. When references disagree or behavior is
ambiguous, the TS implementation wins.

**Why:** The goal is a faithful reimplementation of the original framework, not
a variation on an existing Rust port.

## 2026-10-22 — `dshbox/cordis-rs` is reference-only

The existing Rust port [dshbox/cordis-rs](https://github.com/dshbox/cordis-rs)
is kept locally for comparison and inspiration but is not a target. Its API
shape and architectural decisions are not authoritative and may be diverged
from freely.

**Why:** It is an independent effort whose distance from the upstream semantics
is unknown; adopting its decisions wholesale would undermine the goal of
verifying the port against the original.

## 2026-10-22 — Mirror semantics, not mechanisms

We reimplement the *behavior* of Cordis, not its implementation techniques. The
original leans on JavaScript-only mechanisms that do not exist in Rust.

**Why:** A line-by-line transliteration is impossible and undesirable. See
`docs/limitation.md` for the specific mechanisms affected.

## 2026-10-22 — Cargo workspace mirroring the TS packages

The planned layout is a Cargo workspace whose crates mirror the original
`cordis` packages (`re-cordis-core` first, then `re-cordis-loader`, `re-cordis-timer`,
`re-cordis-logger`, …), rather than a single monolithic crate.

**Why:** It keeps the correspondence to the upstream package structure explicit
and lets satellite capabilities stay optional, as they are upstream.

## Timer satellite

The original 2026-10-27 delivery uses a sealed TimerExt extension on Context,
with no TimerService/install step and no deprecated setTimeout/setInterval
aliases. The 2026-11-03 T0–T5 repair keeps these Rust representations and the
pinned TypeScript timer implementation as the semantic reference. The pinned
spec has 22 declaration lines, not the earlier reported 20; this is an
inventory count, not a parity or Rust-test count.

### Hosted ownership and admission-time clocks

Concrete regressions showed raw spawned callbacks lacked HOST authority,
refused work retained callbacks, and completed timers retained effects.
The smallest required core bridge is Context::task: a local future returns its
terminal synchronous callback; core hosts and contains the future, retires its
exact registration, then invokes the callback without another await.
TaskControl only marks cancellation and requests abort. The existing owner
cleanup starts cancellation synchronously and awaits the supervisor's join.
Effect Drop and transfer/composition contracts remain unchanged.
Context::invoke_callback validates the actual host/owner and preserves one
original CallbackError diagnostic, without granting authority to raw Tokio tasks.

Check host/owner admission before constructing Tokio time or spawning work.
Deadlines use checked monotonic Instant arithmetic and are pinned eagerly at
successful construction, including Sleep whose polling is delayed. Scheduled
calls pin their trailing deadline at successful call admission. Constructors
return InvalidDelay for unrepresentable deadlines and zero repeat periods.
Positive periods below a millisecond are accepted with Tokio millisecond precision.
Zero timeout/sleep/debounce remain asynchronous; zero throttle leads immediately.
Tokio's ordinary Skip behavior still bursts for lags below 5ms, so repeat delivery
explicitly resets to the next original phase after each tick. If the next phase
cannot be represented, callback repetition stops and a stream ends with InvalidDelay.

### Owned wrappers, bounded streams, and callbacks

Core Effect/TaskControl Drop remains inert. Sleep, IntervalStream and Scheduled
Drop explicitly request cancellation; destruction and effect retirement require
the original host to be driven or the owner's cancellation barrier awaited.
Idle Scheduled captures release synchronously; captures in an executing callback
release when it returns. Separately claimed manual Effect completions remain
independent waiters under the existing core contract.

IntervalStream is a single-consumer Stream with one coalesced pending tick.
This intentionally differs from TS, which only serves pending next() calls.
A committed terminal gate discards stale ticks: stop produces clean None,
fail/error produces one Err then stable None. The first committed gate wins;
owner disposal requests and delivered owner cancellation are distinct moments.
Concurrent JS return(value)/throw(reason) multi-reader semantics remain omitted.

Callbacks execute outside framework mutable borrows. Positive-delay recursive
calls coalesce through the ordinary trailing window; recursive zero-delay calls
are ignored to bound recursion/zero loops. Self-disposal and nested Context
work are supported. Panics stop only the affected repeating/scheduled operation
and retain one core diagnostic; leading throttle returns its original failure.
Existing self/ancestor lifecycle waits are refused during task/callback scopes.

Tokio exposes no public driver-presence query. Catch its synchronous timer
construction panic as TimeDriverUnavailable; the standard panic hook still runs.
Do not replace the global hook or create a detached panic/retained callback
diagnostic for this ordinary construction refusal. See
[repair evidence](plans/2026-11-02-cordis-completion/timer.md#executed-repair-evidence)
and [acceptance](plans/2026-11-02-cordis-completion/evidence/timer-acceptance.md).

## Console logger satellite

C0–C3 implement `re-cordis-logger` on 2026-11-03, corresponding to pinned
TypeScript `packages/logger-console` at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`. The shorter Rust crate name follows
the planned satellite layout; the existing core Logger remains the logging system.
The [console contract](plans/2026-11-02-cordis-completion/logger.md) records the
selected scope, targeted results and final review/acceptance status.

- Free `register`/`register_stdout` functions take existing `ExporterOptions`
  and return the existing `ExporterRegistration`. An owned `Write + 'static`
  has no Send bound. Reuse core host/owner admission, removal, filtering,
  buffering, error trees and snapshot lifetime; add no service or global logger.
- All severities use stdout by default, following the Node exporter. Each
  record uses `write_all` then one `flush`; I/O is synchronous backpressure,
  with no queue or hidden task. Partial bytes cannot be rolled back. Writer
  failures propagate through core's attempt-all `LogError::Exporters`.
  Recursive export uses `try_borrow_mut` and returns an I/O error; destructor
  release stays in core's established callback snapshot/removal path.
- Default Auto samples stdout once. Explicit Plain/ANSI16/ANSI256 wins;
  Auto precedence is FORCE_COLOR (0/1/2/3; other present values mean ANSI16),
  nonempty NO_COLOR, terminal/TERM=dumb refusal, TERM containing 256color,
  then terminal ANSI16. Depth 3 caps at ANSI256. Non-Unicode facts are ignored.
  Custom writers and pure render use plain Auto; caller-supplied facts can be
  resolved explicitly. Plain adds no escapes and does not sanitize message text.
- Fixed record-time UTC RFC3339 milliseconds replaces local render-time
  templates. `time = 0.3.36` is pinned, with std/formatting only, to use a
  maintained calendar formatter compatible with Rust 1.85; core dependencies
  stay unchanged. Timestamp can be disabled. Invalid UTC year/layout returns
  an I/O InvalidInput error. No timezone/template API is introduced.
- Elapsed output is integer milliseconds. First and backward deltas are zero,
  with no registration-time clock read. Only successful write+flush advances
  the baseline; a successful backward record becomes the next baseline.
  This deliberately differs from constructor/render-time state and humanized
  upstream durations, making record output reproducible and failures visible.
- Core `LogRecord::format/color` supplies UTF-16 line limits and channel
  palettes. Default max length 10240, width 0, margin 1 and left alignment
  follow upstream. Preserve the asymmetric continuation indentation and longer
  labels. Label width counts UTF-16 units, not terminal cells; no surrogate
  splitting. ANSI256 alone adds upstream bold decoration. ANSI16 timestamp
  gray uses SGR90 rather than upstream's 256-color gray escape.
- Rust Display/Debug/format! replaces JS `util.inspect`, custom printf
  placeholders and browser argument rendering: core records no longer contain
  original JS arguments. These are deliberate language/API representations,
  not output equivalence. The browser console's error/warn method dispatch is
  unavailable in this native writer adapter.

The five pinned source declarations are covered proportionally: aggregate
error output and exact identity through `tests/exporter.rs`; object/custom
formatter purpose through explicit Display/Debug in `tests/render.rs`;
levels through independent core exporter filtering; label style through the
exact two-line right-aligned oracle (17 continuation spaces).
The 8 existing core logger tests remain carried evidence for buffer capacity,
source identities, normalization and core palettes. They are not reopened.

## Loader and structural groups

L0–L5/G0–G1 deliver `re-cordis-loader` as an unpublished Rust 2024/MSRV 1.85
crate. The semantic reference is pinned TypeScript
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`, not the prior Rust port.
A private checked adapter retains each original `Plugin<C,R>` and its core
allocation identity. Native C needs Clone for replay, not Serialize/Send.
Core `hooks::Value` is reused as Config; decoder contract checks stay separate
from the plugin validator's dependency-ready timing.

One dedicated loader owner Fiber owns current entries and hosted convergence
workers. Entry-level injection uses a reusable outer Plugin; the original
catalog Plugin is its inner child and keeps its own requirements. Conditions
and evaluated intercepts use entry authority; config mapping uses the exact
stored inner Context after plugin requirements commit. User callbacks,
clones and resource destruction run outside mutable loader borrows and reuse
core admission/panic containment.

Same-factory loaded config reaches `update_with(no_save:true)` before the
loader source commit. A single bounded rebuild worker follows the latest
accepted revision while joining old cleanup; remove/disable wins. Every
manually detached lifecycle barrier is retained independently of driver tasks,
and removed-but-retiring entries count against 1024 admission capacity.
Only the first completed retirement-task error is retained, bounding that
failure memory; all barriers are still joined. Ordinary callback cleanup
errors follow core Runtime::take_failures diagnostics, not a new loader
cleanup report or a changed core disposal result.
Direct updates retain separate barriers in a loader-wide queue of at most 1024
outstanding operations, including reentrant admission reservations. Later
completion cannot replace an older barrier. Subtree removal joins its own
updates; settlement and owner disposal join all. Completed original waiters or
successful settlement delivery acknowledge results, as does completed subtree
removal for its updates; dropping a waiter does not. Disposal may report the
same retained failure again. The first
unobserved direct-update failure is retained globally, bounding failure history.
Parent disposal joins those barriers even after Loader/waiter Drop. Entry/Fiber
source cycles are broken during retirement/owner cleanup.

Groups are structural loader nodes, including when their verdict disables
descendants; no facade crate or duplicate registry is added. Global EntryId
survives adoption, while mutable group addresses aid navigation. Crossing
groups/scopes rebuilds core Fibers after joined cleanup; same-group reorder is
inert. Groups carrying condition/intercept callbacks reevaluate through an
outer lifecycle rebuild, so their descendant Fiber IDs may change while
logical EntryIds remain stable; pure structural groups retain the incremental
path. This deliberately replaces JS mutable-prototype reparenting.
Upstream ':' denotes nested source trees, not ordinary group ownership;
Rust globally unique IDs across source documents are a namespace adaptation.

Programmatic mutations preflight complete entry-option structure before commit.
This is not a transaction over arbitrary plugin validators: operational child
failures are reported independently and healthy siblings proceed. A loaded
validation rejection retains that child's accepted source; an apply failure
after acceptance retains the new source and Failed state. Retaining the rejected
child snapshot is a Rust adaptation of TS's earlier entry.options mutation.
Read-only source reconciliation preserves valid siblings independently,
including nested JSON children, and emits no write-back. A per-loader core
event reports accepted runtime CommitRecord values with old/new/target/origin;
there is no unbounded history. SourceId/SourceDocument, optional span locators
and typed replacement provide useful in-memory include/HMR integration seams,
without file I/O/watchers or JS module execution.

JSON uses serde_json with explicit limits: 1 MiB, depth 32,
65536 nodes/string bytes, group depth 32, 1024 resident/retiring entries,
128-byte nonempty IDs without ':', 256-byte nonempty factory names.
Absent/null config reaches an explicit decoder unchanged. Unknown fields,
factories and service contracts reject explicitly. Disabled expressions are rejected;
config/inject/intercept values belong to explicit typed decoders and are never
executed as JavaScript. A permissive decoder may accept expression-shaped data.
Typed service registrations map serialized inject/isolate/intercept to core
keys/configs. FieldPatch Keep/Set/Clear distinguishes retention and removal.
Named JSON realms are shared while their defining Scope tokens live (including
snapshots); the loader keeps weak entries, prunes dead labels and permits at
most 1024 simultaneously live named realms. Rejected decodes release their
tokens. Reusing a label after its last defining scope drops may create a fresh
identity; old committed Contexts keep their exact old views. Empty/obsolete
source records are pruned. Anonymous positional reuse requires current source
ownership and root position, so it cannot reclaim an entry adopted elsewhere.
Duplicate IDs within a source are reported per colliding definition; the first
valid identity and healthy siblings survive across nested groups.

Snapshots separate retained native source/options, optional original JSON
transport and last evaluated C. Initial resolved R is not exposed by core's
original Plugin; it stays plugin-owned rather than being fabricated.
Typed condition/intercept failures are Failed and require explicit update,
instead of JavaScript truthiness/evaluation failure becoming a disabled verdict.
All entries use an outer owner, including empty injection; this simplifies
consistent teardown at the cost of an extra core Fiber. Entry injection values
are typed static Dependency configs; no dynamic JS inject-value evaluator is
provided. Explicit settle replaces the implicit loader service's await intercept.
Self/ancestor lifecycle requests keep core's reentrant refusal. See the
[loader API](en/api/loader.md), [group API](en/api/groups.md) and
[work package](plans/2026-11-02-cordis-completion/loader.md).

## Include files and persistence

The 2026-11-03 include I0–I4 delivery is selected over accepted loader
101f37b. Include owns file snapshots, locations, ordered patches, a bounded
journal and explicit refresh/flush; loader/core continue owning plugin
lifecycle. A dedicated include owner serializes local operations. Standard
synchronous file I/O keeps accepted writes bounded in count without detached
background writers; it can block the local executor on slow storage.

JSON is the first format. YAML is selected as a canonical value format,
subject to the executed parser capability gate in the
[include contract](plans/2026-11-02-cordis-completion/evidence/include-contract.md).
Comment/style retention is not claimed. Unsupported aliases, anchors and custom
or executable tags must fail with source diagnostics before expansion or
execution. Structured locations distinguish persisted source from runtime data.

File precedence applies only to conflicting runtime changes in a three-way
comparison of the prior snapshot, current file and pending journal. Unrelated
keys survive. Patch-owned keys and embedded entries write to their actual owner.
Sibling temporary files and replacement isolate individual file writes;
atomic visibility, power-loss durability, multi-file transactions and races
with uncooperative external writers are separate guarantees. Every retry
rereads and reconciles; an invalid current file is never overwritten.
Pending work remains available after a write failure, and joined shutdown
must expose the final write result as well as loader cleanup.

Arbitrary typed loader Config values are not implicitly serializable. Include's
persistable mutation boundary uses explicit JSON values rather than imposing
Serialize on accepted native loader plugins. Exact signatures, limits and
platform boundaries are recorded with the contract and acceptance evidence.

I3 adds bounded immutable pending snapshots and original placement so partial
multi-file success does not turn an already-written removal into accidental
loss of a pending move. Permission bits are retained on replacement; absent
roots are installed exclusively with a hard link to the complete sibling temp.
Include descriptors in moved subtrees are rebased to canonical absolute paths.
Operation errors retain their original sources and write-failure conflicts.

The IR5 correction adds explicit patch `unset` field names, applied after
overrides, to preserve effective absence without changing hidden base values.
It keeps absent config distinct from JSON null. Runtime removal routes this
marker to the actual patch owner; a later set removes the marker. Structural
control keys cannot be unset. This is a necessary Rust/JSON representation
for field deletion, not JavaScript undefined serialization.

## HMR watching and linked replacement

The selected H0–H3 scope starts at 47042d9 and reuses accepted include 0a2df1e,
loader 101f37b and timer/core task ownership. Exact APIs and execution gates
live in the [HMR contract](plans/2026-11-02-cordis-completion/evidence/hmr-contract.md).

Watch explicitly registered file paths with a standard-library polling thread
whose JoinHandle belongs to the watcher. Reviewed notify 8.2.0/8.2.1 compile on
1.85.0, but PollWatcher does not expose a joined producer shutdown. Keeping our
own bounded registered-file poller avoids a detached producer or platform-specific
join claim. Accepted timer debounce and core hosted tasks handle local dispatch.
Only owned paths, numeric generations, I/O errors and synchronization state
cross threads. There is no directory traversal or inferred source graph.

Use bounded queue overflow to request a rescan of all active registered paths;
logical path generations and exact registration tokens reject obsolete delivery.
File inode replacement does not change the registered path. Missing files can
recover; symlinks are refused under the documented ordinary-filesystem policy.
Cancellation requests are distinct from joined watcher/owner disposal, which
waits for producer termination and local task/capture release.

Factory replacement takes an explicit dependency graph plus already-compiled
typed candidates. A minimal loader batch seam checks every declaration and
retained source contract before catalog publication or reload teardown. Optional
raw-source preflight runs with same-loader mutation admission guarded; core
Plugin validation stays on the actual dependency-ready replacement Context.
Loader retains each instance's configuration and exact providers through old
cleanup, then follows current identity/intent. It owns admitted work even after
a replacement waiter is dropped. External effects cannot be rolled back after
unload; Failed results and explicit recovery remain visible.

The optional HMR-side `watch_include(path, include)` registers one explicit
trigger path and invokes refresh on the supplied Include, including its source
graph. It retains the admitted IncludeTask outside the cancellable callback;
stop suppresses follow-up and watcher/ancestor cleanup joins that same task
without new Include admission. Active and unfinished retired registrations share
one quota. Returned Include errors and every Report failure retain their original
causes in bounded WatchEvent history; ordinary core cleanup failures retain their
existing Runtime diagnostics channel.

`ReloadPlan::replace_include` forwards the exact linked factory batch through
that same Include's private loader. It preserves journal, data, provenance and
file bytes and does not flush. The offline reload example demonstrates watched
config updates before and after compiled replacement on one stable EntryId.
Include has no dependency on HMR. Changed Rust source produces restart-required;
the application owns any build/restart process. Node caches, JS execution and
native ABI loading are outside H0–H3. H0–H3 is independently accepted with
[full shared verification](plans/2026-11-02-cordis-completion/evidence/hmr-acceptance.md)
on stable and 1.85.0. Exact replacement failures preserve distinct occurrences;
read-error coalescing begins only after successful ingress admission.

## Create offline scaffolding

The user's 2026-11-04 selection is minimal offline S0–S2. The unpublished
`re-cordis-create` library/binary generates one useful Rust application from
a local checkout, using the accepted core, console logger and loader.
The [work package](plans/2026-11-02-cordis-completion/create.md) records the
selected slice and acceptance progress; the
[source map](plans/2026-11-02-cordis-completion/evidence/create-source-map.md)
records upstream dispositions. No core hosting contracts are reopened.

- **Rust representation:** a synchronous `create` function with `Path` inputs,
  contextual `CreateError` and a small native-argument CLI replaces interactive
  JavaScript/package-manager scaffolding. The generator uses only std, runs
  no subprocess and requires no framework host.
- **Name admission:** reject the three bundled Cordis dependency package names
  before staging. Reusing one at version `0.0.0` creates a Cargo lockfile identity
  collision at different paths; accepting it would generate an unusable project.
- **Dependency choice:** unpublished `0.0.0` Cordis crates use validated canonical
  absolute Unicode checkout paths with TOML escaping. Validation checks this
  repository's literal manifest/layout spelling, not arbitrary Cargo TOML or source. An application `[workspace]` with resolver 3
  prevents accidental membership in an enclosing workspace. Rust 2024/MSRV 1.85
  and unpublished status remain explicit. Dependency installation and lockfile
  creation happen only through the user's subsequent Cargo commands.
- **Application contract:** a current-thread Tokio entry point drives the existing
  local Runtime, typed service/plugin configuration, scoped console output,
  loader updates and explicit joined shutdown. The generated program checks
  operation errors, cleanup reports and runtime diagnostics; meaningful tests
  verify resource release and error channels.
- **Filesystem choice:** validate name/checkout/destination before rendering all
  five files in an owned sibling staging directory, recheck destination and
  commit through one rename. Destinations are absent or empty ordinary
  directories, their parents already exist, and failed staging cleanup is visible.
  This is a narrow local operation, not a hostile-filesystem confinement or
  power-loss durability protocol; empty-directory rename behavior is platform-dependent.
- **Deferred scope:** remote templates, downloaders, archives/cache, Git/install/
  launch options and Include/HMR templates are unselected. They are separate
  product choices, not inherent Rust limitations.
- **Evidence:** paired guides/API and documentation checks execute the actual
  generator and generated application offline using both explicit toolchains.
  Runtime filesystem evidence is macOS only. Independent S0–S2 acceptance is complete;
  existing core and satellite acceptance retains its original scope.

Offline S0–S2 is independently accepted at executable revision `8373fa3`;
see [final verification](plans/2026-11-02-cordis-completion/evidence/create-acceptance.md).
Publication and additional template capabilities remain separate decisions.
