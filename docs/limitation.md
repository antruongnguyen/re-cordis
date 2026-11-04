# Limitations & Open Tensions

Known gaps, unresolved questions, and places where a faithful port is hard.
This is where we are honest about what does *not* translate cleanly from the
TypeScript original to Rust.

## How to classify Rust differences

Under the [2026-11-03 implementation direction](decision.md#2026-11-03--deliver-cordis-features-through-idiomatic-rust-contracts),
a different Rust API is not automatically a missing feature. Describe the
upstream purpose, supported Rust contract and any capability the user loses.
Record the reason and alternative in decision.md as well. Use distinct labels:

- **Rust adaptation:** a supported feature expressed through Rust types,
  ownership, traits or explicit operations; test that documented contract.
- **Language/runtime constraint or unsuitable emulation:** no suitable native
  equivalent, or compatibility would require disproportionate machinery;
  explain the actual constraint and the supported alternative.
- **Deferred feature:** feasible work that is not implemented yet.
- **Missing evidence:** behavior not sufficiently tested; neither a proven
  defect nor proof that Rust cannot support it.

Do not label a capability impossible merely because Rust lacks the same syntax
or built-in mechanism. Conversely, an adapter with reduced behavior must state
that reduction. Existing guarantees and explicit async-cleanup boundaries
continue to apply; this policy adds no new API or performance guarantee.

## JavaScript mechanisms requiring explicit Rust designs

The original Cordis core derives much of its ergonomics from features Rust does
not have. These require real design, not translation:

- **Proxy-based reflection.** In TS, a `Context` is a `Proxy` (`ReflectService`)
  that intercepts property access to resolve services, apply isolation, and
  trace callers. Rust has no runtime property interception. The `get` / `set` /
  `provide` / `mixin` surface must be re-expressed through explicit APIs,
  traits, or a typed registry.
- **Prototype-chain inheritance.** `isolate` and `intercept` build scoped child
  contexts by creating objects whose prototype is the parent, so lookups walk
  the chain. Rust has no prototype chain; scope inheritance must be modeled
  explicitly (e.g. parent links, layered maps).
- **Callable service objects.** A TS `Service` can itself be callable (a
  function with properties). Rust closures/`Fn` traits can approximate this but
  not with the same transparency.
- **Dynamic, string-keyed, heterogeneously-typed services.** TS stores services
  in string-keyed dictionaries with types attached via declaration merging.
  Rust needs a typed strategy (type-erased storage, downcasting, or a
  trait-object registry) to recover both dynamism and type safety.

## M6 current boundary

The unpublished core includes [M6 logging/injection/type/example acceptance](m6.md),
[M5 scoped reflection/config updates](m5.md),
[M4 typed services/injection](m4.md),
[M3 base plugin lifecycle/Registry](m3.md), and Fiber-scoped
[effects, typed events, and diagnostics](m2.md) on the [M1 host](m1.md).
The final audit accounts for all pinned declarations through integrated tests
or explicit Rust representations. This is not unrestricted TS compatibility;
the [M6 restriction/deviation list](m6.md#remaining-restrictions-and-safety-deviations)
is part of the supported contract.

The post-M6 [lifecycle-parity slices](lifecycle-parity.md) add cross-language
execution for six gated provider/config/cleanup/failure/recovery scenarios only.
They do not establish full upstream suite parity, transparent JavaScript compatibility,
or reproducibility of upstream's entire JavaScript dependency graph (the pin
does not track a Yarn lockfile). No production API/behavior was changed.
In both implementations, consumer update settlement may be Pending before
a later provider arrival. It is not a runtime-wide readiness guarantee.
Three failure/recovery compositions now have paired execution: obsolete/latest
load failures with gated rollback and exactly-once diagnostics, an abandoned
update waiter, and explicit gated recovery after ignored provider refresh.
This is not exhaustive failure/recovery, cancellation, observer-error, or
exporter-error protocol parity. No further slice or satellite is selected.

- Services use explicit `ServiceKey<T>` access, not JS properties. M5 supports
  isolated slots with retained boundary history and explicit `BoundService`
  adapters carrying separate definition/use Contexts. An ordinary `Rc<T>`
  cannot transparently rebind arbitrary Rust methods. The two realm-dependent
  service-disposal declarations now have integrated evidence.
- Reads return `Rc<T>` snapshots of a publication's current value. Holding a
  returned value is not a lease on the provider's effects/resources; only
  committed injection retains those through cleanup. Existing `Rc<T>` reads
  do not change when set replaces the stored value.
- Injection declares required names/types, typed config metadata and synchronous
  provider checks. Intercepts merge through an explicit service-specific callback.
  Automatic Service.init/constructor discovery is not implemented; explicit
  initialized factories provide the initialization barrier and method-injection
  adapters replace decorators. `set` preserves publication
  identity and does not notify dependencies or recheck readiness.

- Plugin declarations are immutable and reusable with explicit allocation
  identity; JS function/object/constructor behavior is adapted to typed callbacks.
  Apply registers effects rather than implicitly interpreting arbitrary return
  shapes. Dynamic callback metadata mutation is not implemented.
- Missing dependencies now produce Pending without validation; exact publication
  replacements drive unload/reload. Typed status-event sequences are now tested.
  Failed Fibers stay resident and recover only through config update.
- Restart/dispose self-waits and owning-ancestor waits are refused from directly
  attributed callbacks. User-spawned task graphs do not gain that attribution
  or general deadlock detection automatically.

- Synchronous cleanup is explicit through `Disposer::sync`/`on_dispose_sync`.
  All-sync manual chains/partial rollback complete before returning; an async
  child remains an awaited barrier. Async setup still uses readiness/cleanup
  futures. Labels and tree snapshots contain only metadata, not resources.
- Recursive self/ancestor composition transfers are refused before claiming
  their root registration, avoiding a self-wait or metadata reference cycle.
- Listener responder and around callback roles are explicit. Bail is sync-only;
  sync and async waterfall have separate owned continuation types. Wrong mode
  or live name/type contracts return errors rather than silently skipping hooks.
- Diagnostic logging has its bounded buffer/exporters, traced normalized names,
  typed intercepts, owner IDs and expiring weak lifecycle metadata. Explicit
  ErrorValue trees represent cause/aggregate metadata. Rust formatting/exporter
  callbacks replace dynamic JS formatter dictionaries and Error properties.
  Per-line formatting preserves Unicode scalars rather than lone surrogates.
- Context is non-owning: after Runtime drop, metadata/IDs are readable but
  operations return `RuntimeDropped`. Arbitrary user-created Rc cycles are
  not collected. Retained Fiber handles can retain source/metadata and pending
  cleanup captures until dropped; host destruction does not run async cleanup.
  Tracing requires explicit bound adapters; native property/method interception
  and automatic tracing of arbitrary returned objects are not provided.
- Only its explicit run scope admits work. Missing/wrong hosts, nested or
  concurrent driving, and hosting inside spawned tasks are refused. Pausing
  a run pauses local tasks; host destruction is cancellation, not cleanup.
- Root drain covers a snapshot, not an executor-wide shutdown barrier. Later
  registrations require another pass; already-claimed manual cleanup has
  its own completion handle that must be awaited separately.
- User callback failures preserve their sources and are reported/retained;
  diagnostic sinks do not imply full Logger parity. Panic-abort, panicking
  destructors, and framework invariant failures cannot promise completion.

## M0 boundaries selected for the initial implementation

The [M0 experiments](m0.md) select local-first Tokio/LocalSet execution,
non-Send handles/futures, explicit definition/use-bound calls, synchronous
validation, and owned one-use continuations. They do not constitute an
integrated framework or resolve every behavioral gap.

- Started operations outlive dropped waiters only while the host executor
  remains driven. Shutdown is not async cleanup; explicitly await teardown.
- An async emit listener needs an explicit eager-prefix/continuation adapter.
  Passing a native async closure does not automatically run its body at call time.
- Bail is synchronous-only; async responses use serial. Mixed bail snapshots
  are refused before invocation rather than mimicking JS promise-valued results.
- Duplicate next is a move error, not an upstream-style runtime exception.
- Name/type or cross-name shared-label type mismatches return explicit errors;
  unchecked JS heterogeneous aliasing is not a Rust compatibility promise.
- Same-owner lifecycle requests during cleanup are explicitly refused. Task-local
  attribution does not propagate through arbitrary user-spawned tasks. M3 covers
  setup/restart/dispose and owning-ancestor refusal; update is still M5 work.

## Unresolved design questions

- **Async model.** `parallel` and `serial` return promises; `emit` and `bail`
  dispatch synchronously, and continuation-based `waterfall` returns whatever
  its middleware chain returns (including promises). Rust futures are lazy,
  so returning or dropping an unpolled future does not reproduce JS start
  behavior. M3 integrates base non-root lifecycle and per-Fiber ownership;
  M5 adds typed internal protocols and explicit bound service views, while native
  lazy futures still require eager-prefix adapters where immediate execution
  matters. See design gates A and C in the
  [implementation plan](implementation-plan.md).
- **Ownership and teardown.** Rust Drop cannot await cleanup. M3 implements
  parent-owned children and reusable-root disposal with weak backreferences.
  M4 keeps exact provider bindings usable through dependent cleanup, even
  after consumers leave the Registry, and preserves unrelated concurrent
  cleanup. M5 covers the isolation-dependent retention cases too; remaining
  syntax/tracing representation differences are explicit in its contract.
- **Definition site versus use site.** Typed shared service pointers alone
  cannot retain upstream shadow/caller tracing. Dependency authority,
  use-site intercept/effect behavior and nested service calls use M5's bound
  adapters, not transparent pointer reads. Re-selecting the same
  isolation label is still an injection-inheritance boundary; retaining only
  final realm values loses that information.
- **Config validation.** TS Cordis uses Standard Schema for plugin config
  validation (`ValidationError`) and rejects asynchronous validators. Source
  and resolved config differ, validation is deferred while Pending, and
  invalid updates to loaded Fibers preserve the running configuration. M3 has
  synchronous typed validation/source separation, and M4 defers it until
  dependencies resolve. M5 adds typed updates/recovery and structured issue paths.
  Automatic schema discovery remains unsupported; serde is not an
  automatic core dependency.
- **Logging representation.** Use Rust Display/Debug, ErrorValue metadata and
  exporter callbacks. Arbitrary Error::source graphs require explicit owned
  cause adaptation; `.error(Display)` does not inspect dynamic properties.
  Callable logger syntax and JS printf/JSON argument semantics are not promised.
- **Error model.** Original Rust error sources and typed validation paths are
  retained. JS stack-string rewriting and hostile destructor recovery are not
  implemented. Typed config middleware cannot replace the validator's Rust
  contract, and virtual get-only values have no publication identity for bind.

## Scope limitations (current)

- [English](en/README.md) and [Vietnamese](vi/README.md) user documentation
  covers the current unpublished core. The API reference complements generated
  Rustdoc; it does not establish a stable release, translated Rust identifiers,
  full upstream compatibility or satellite availability. `llms.txt` is a local
  retrieval index, not a published documentation service or agent permissions.
- Approved [GitHub activity consumers](github-activity.md) are application
  examples, not production satellites. The digest supports bounded GitHub.com
  snapshots and programmatic updates/recovery; its CLI is one-shot. Atomic
  replacement is not crash durability or multi-process coordination.
  The collector stores a latest bounded snapshot, not complete history; its two
  queries are not a GitHub transaction. Programmatic updates join periodic work;
  CLI configuration is startup-only and subprocess tests require Unix utilities.
  Signed webhook journaling is loopback-only, with bounded envelope validation
  rather than every GitHub schema field. Append/flush is not crash durability:
  timeouts/disconnects may precede a later append, duplicate deliveries remain,
  and failed writes may require manual partial-line repair.
  Public ingress, live webhook delivery, mutations, and durable/exactly-once
  delivery are excluded. See the consumer document for exact resource/time limits.
- `re-cordis-core`, `re-cordis-timer`, `re-cordis-logger`, `re-cordis-loader`,
  `re-cordis-include` and `re-cordis-hmr` are implemented within their documented
  Rust scope. Structural Group belongs to loader. HMR H0–H3 is independently
  accepted with shared stable/1.85.0 verification. Selected offline Create S0–S2
  is pending independent acceptance; remote scaffolding remains unselected.
  See [the roadmap](todo.md).
- No stability guarantees — the API will change freely until the core is proven.
- `spikes/m0` uses separate test-only models: one dependency per lifecycle
  Fiber, synchronous setup, and no integrated update/recovery/logger protocol.
  Its passing tests are design evidence, not production parity coverage.

This document should shrink as tensions are resolved (move resolved items into
`docs/decision.md`) and grow as new ones surface.

Core safety deviations: observer hook failures are diagnosed without interrupting
retained-provider cleanup; shared-slot aliases also receive dependency refresh.
These strengthen error/teardown safety rather than claiming identical upstream
observer-failure or name-only alias notification behavior.

## Timer satellite

T0–T5 repairs admission, eager deadlines, hosted callbacks, registration retirement,
bounded tick storage, callback reentry and joined owner cancellation. Executed
regressions and their original failure reasons are recorded in the
[timer repair plan](plans/2026-11-02-cordis-completion/timer.md#executed-repair-evidence);
[final acceptance](plans/2026-11-02-cordis-completion/evidence/timer-acceptance.md)
separates implementation evidence from shared verification.

- **Language/API representation:** sealed TimerExt on Context, owned 'static
  FnOnce/FnMut callbacks and explicit Scheduled::call replace JS mixins/callable
  objects. Contexts, wrappers and futures remain local and non-Send. Deprecated
  setTimeout/setInterval aliases and TimerService installation are omitted.
- **Deliberate clock adaptation:** checked monotonic Tokio deadlines replace
  Date.now() wall-clock windows. Deadline admission is eager; late first polling
  does not extend the requested delay. Zero one-shots/debounce are asynchronous,
  throttle leads immediately, and zero repeat periods/unrepresentable deadlines
  return InvalidDelay. Positive sub-millisecond periods retain Tokio millisecond
  precision. Missed ticks skip to the next original phase even below Tokio's
  native 5ms skip threshold. Exhausted next-phase arithmetic stops callback
  repetition or terminates a stream with InvalidDelay.
- **Deliberate stream adaptation:** one logical consumer and one coalesced pending
  tick; TS serves only pending next() calls. Committed terminal state wins over
  queued ticks. stop gives None, fail gives one original Err then stable None,
  and delivered owner cancellation gives Disposed once. The first committed gate
  wins; an owner lifecycle request alone has not yet committed that gate.
  JS concurrent-reader return(value)/throw(reason), return values and repeated
  rejection after termination are unavailable. Use callback interval for fan-out.
- **Ownership/hosting constraint:** Sleep/Stream/Scheduled Drop requests abort;
  synchronous Drop is not an asynchronous join. Drive the original host or
  await owner cancellation to join captures/registration retirement. Core Effect
  and TaskControl Drop remain inert, and separately claimed manual Effect
  completions are separate barriers. TaskControl cancellation cannot admit work.
  Raw user-spawned tasks still lack HOST authority; timer callbacks have it.
- **Callback policy:** user code runs outside mutable framework borrows.
  Positive-delay recursive calls coalesce; recursive zero-delay calls are ignored.
  Panic stops the affected repeating/scheduled operation and reports once while
  siblings remain usable. Self/ancestor lifecycle waits are refused.
- **Runtime environment:** Tokio has no public time-driver-presence query.
  Missing-driver construction returns TimeDriverUnavailable after containing its
  synchronous panic; the normal panic hook is still visible. No detached timer
  task or retained core callback diagnostic is created.

Remaining satellites remain future work; timer repair does not implement loader,
group/include/HMR/create or settle unrelated B0 evidence gaps.

## Console logger satellite

`re-cordis-logger` is implemented as an unpublished synchronous native writer
adapter. See [the paired console API](en/api/console-logger.md) and
[decisions](decision.md#console-logger-satellite) for its full supported contract.

- **Language/API representation:** owned `Write + 'static`, free registration
  functions and the existing core removal token replace a JS exporter class.
  Rust Display/Debug/format! supplies formatting; JS object inspection,
  exporter custom placeholders and browser-console original arguments cannot
  be recovered from a core `LogRecord`. Use explicit `ErrorValue` trees for
  aggregate/cause errors; the adapter does not rewrite JS stack strings.
- **Deliberate output adaptation:** UTC record-time milliseconds replaces
  local render-time templates; custom timezone/template support is deferred.
  Integer millisecond deltas replace humanized upstream durations. The first
  and backward deltas are zero, and only successful write+flush commits the
  baseline. UTC years outside 0–9999/layout overflow fail explicitly.
- **Terminal capability boundary:** Auto uses captured stdout/Unicode
  FORCE_COLOR/NO_COLOR/TERM facts, not the full supports-color heuristic or
  browser/Windows console machinery. Explicit colors override those facts.
  Truecolor requests cap at ANSI256; ANSI16 timestamps use SGR90 gray.
  Auto is plain for custom writers and pure render, unless callers resolve
  their own facts into an explicit policy. Plain adds no escapes but preserves
  user text, including existing ANSI. Unicode label padding counts UTF-16,
  not terminal display cells or grapheme clusters.
- **Runtime/ownership contract:** synchronous slow I/O blocks the local log
  call. There is no asynchronous sink, stderr split, buffer or retry queue.
  Standard short/interrupted writes are handled; write/flush failures remain
  visible and partial output cannot be rolled back or guaranteed atomic
  across independent stdout writers. Flush is not filesystem durability.
- **Reentry/snapshot contract:** recursive calls into an exporting writer get
  an I/O error; core still attempts other exporters. The adapter creates no
  automatic recursive error log. Removal affects future records; an already
  selected callback may finish before its writer capture is released.
  A writer panic follows existing core callback panic handling; hostile/panicking
  destructors retain the core's documented limits.
- **Evidence boundary:** deterministic Rust tests cover the five source test
  areas with explicitly classified representations; they do not establish a
  new executed cross-language console trace or unrestricted upstream parity.
  Final review and the explicit stable/MSRV acceptance are recorded separately
  from implementation in the [console work package](plans/2026-11-02-cordis-completion/logger.md).

## Loader and structural groups

The unpublished loader/group delivery is useful native Rust loading and
reconfiguration, not unrestricted execution of the eleven upstream JS suites.
See [loader API](en/api/loader.md), [groups](en/api/groups.md) and the
[source dispositions](plans/2026-11-02-cordis-completion/loader.md#source-case-dispositions).

- **Runtime boundary and unsuitable emulation:** this Rust runtime supplies no JS import/export objects,
  callable constructors, expression evaluator, mutable prototype chain,
  Node module cache or V8 stack-rewriting protocol. Explicit typed catalog
  declarations, decoder/condition/mapper closures and typed service keys replace
  those mechanisms. Unsupported disabled expressions and unregistered contracts
  fail clearly; decoder-owned config can contain expression-shaped data but
  nothing executes it. Embedding a JavaScript engine or a native ABI loader is
  a separate project, not an inherent impossibility in Rust.
- **Deliberate ownership adaptation:** cross-group/scope moves preserve EntryId
  but rebuild outer/inner Fiber allocations after old cleanup. Use immutable
  snapshots for new Fiber IDs; old committed consumer Contexts retain exact
  providers through cleanup. Same-group reorder does not restart.
  Callback-bearing group reevaluation also rebuilds its outer lifecycle and
  may replace descendant Fiber IDs; logical entry identity remains stable.
  Framework self/ancestor lifecycle calls remain refused inside callbacks;
  loaders do not silently acquire deferred authority.
- **Namespace adaptation:** IDs are globally unique within a loader across
  SourceDocuments. Mutable group addresses are navigation paths; they do not
  reproduce upstream nested-source-local duplicate IDs or subtree namespaces.
  Callers combining sources must assign unique explicit IDs.
- **Typed-config boundary:** C needs Clone, not Serialize; arbitrary native
  equality is unavailable. Explicit updates can replay equal source. Config
  mapping snapshots evaluated C before validation; original Plugin<C,R> does
  not expose initial resolved R, which remains in application apply code.
  Condition/intercept callback errors leave Failed state and require explicit
  update; JavaScript false-like/error-to-disabled semantics are not emulated.
  Entry injection configuration is static typed Dependency data; changing it
  requires an update rather than a JS expression refresh. All entries use an
  outer owner even when injection is empty. Explicit settle replaces implicit
  loader-service awaiting. JSON isolate false/null means inherit, a deliberate
  convenience beyond the pinned TS true/string-only field contract.
  Snapshots intentionally retain source Rc values, not independent resource
  leases or mutable Fiber control.
- **Satellite boundaries:** SourceDocument/reconcile/commits and replace_factory
  are tested in-memory seams. Include now supplies bounded file reading/writing,
  patches and reconciliation in its own crate; HMR supplies explicit file
  watching and application-declared linked replacement in a separate crate.
  Per-entry source failures are independent; runtime structural preflight is
  whole-input, while async plugin failures do not roll back healthy siblings.
- **Explicit bounds:** JSON syntax/global boundary failures reject the document.
  JSON entry decode failures preserve healthy siblings. Native group depth is
  32; admission counts removed-but-retiring entries until their joined barrier
  is observed. No unbounded commit journal or queued rebuild history is stored.
  Source metadata for empty/obsolete ownership is pruned. At most 1024 named
  realms can have live defining Scope tokens; snapshots can retain those tokens.
  A label used after all defining scopes drop can acquire a fresh isolation ID,
  while old committed Contexts keep their old identity. Anonymous positional
  source matching only reuses entries still owned at that source root.
  At most 1024 direct updates may be outstanding across the loader; admission
  refuses more until completed work is pruned. All admitted barriers are joined,
  but only the first unobserved direct-update failure is retained globally.
  Async cleanup needs the original host; Drop cannot complete it.
  Loader keeps the first detached lifecycle-task error, not an unlimited error
  history. User cleanup callback errors retain core diagnostic reporting;
  inspect Runtime::take_failures, not just Disposed or a successful waiter.
- **Evidence boundary:** deterministic Rust tests cover identity, admission,
  source updates/adoption, latest rebuild intent, failure, evaluation and
  teardown compositions including 100 group cycles and retained provider moves.
  Final MSRV/stable review and new paired traces are recorded separately;
  the six existing core traces retain their original scope.

## Include files and persistence

The selected include I0–I4 scope uses explicit JSON-valued source changes and
local hosted operations over accepted loader/core. See the
[include contract](plans/2026-11-02-cordis-completion/evidence/include-contract.md)
for exact limits and the capability gate; delivery evidence is recorded when
verification completes.

- **Format adaptation:** canonical value YAML does not retain comments, anchor
  identities, quoting or style. Aliases/anchors, custom tags and executable
  nodes are rejected; JavaScript execution and stack rewriting are unavailable.
  A value parser does not establish lossless round-trip compatibility.
- **Ownership adaptation:** globally unique loader EntryIds and explicit
  serialized changes replace upstream proxy-mediated mutable options. Arbitrary
  native Config values cannot be persisted without a caller-selected encoding.
- **I/O boundary:** synchronous bounded file operations may block on slow
  storage. Byte/count bounds do not guarantee a wall-clock deadline. Sibling
  replacement does not provide a multi-file transaction, a complete protocol
  against concurrent external writers or a power-loss guarantee.
- **Integration boundary:** explicit refresh requires no watcher. The optional
  HMR adapter triggers refresh on the supplied Include; each watched path must
  be registered explicitly. Create, JS evaluation and native binary hot loading
  remain outside the delivered scope.

I3 persistence is implemented. Writes remain non-atomic across files, but bounded
full pending snapshots recover partial move writes on a later retry. Missing-root
creation requires hard-link support. Canonical absolute descriptor rebasing on
moves preserves the currently resolved child source; later retargeting of the
old symlink spelling is no longer followed by that rewritten descriptor.

Patch `unset` is an explicit Rust/JSON adaptation for effective field absence.
It differs from config:null and prevents a removed override from revealing a
hidden base value. Existing value-only YAML, path and durability limits remain.

## HMR watching and linked replacement

H0–H3 watching, optional Include integration and linked replacement are
implemented and independently accepted. The full stable/1.85.0 shared matrix
passes; see [acceptance](plans/2026-11-02-cordis-completion/evidence/hmr-acceptance.md). The
[contract and progress](plans/2026-11-02-cordis-completion/evidence/hmr-contract.md)
separate delivery evidence from the following chosen boundaries.

- **Rust adaptation:** application-supplied dependency nodes select catalog
  factory candidates already compiled into the process. Nested core plugins
  rebuild through their explicitly affected host entry; arbitrary closures
  inside an unchanged host are not discovered or rewritten.
- **Runtime boundary:** changed Rust source requires rebuilding/restarting.
  No process supervisor, inferred Rust/Cargo module graph, Node cache/import
  emulation, JS evaluator or native binary/ABI swapping is supplied.
- **Watcher adaptation:** only registered regular file paths are sampled.
  Ignored names are native path components, with explicit registration override;
  no recursive root discovery or glob engine is provided. Polling can miss
  changes entirely between samples; its bounded content digest is not a
  collision-free or security guarantee.
- **Path/platform boundary:** native logical paths survive inode replacement.
  Symlinks are refused, but portable metadata/open operations are not a hostile
  filesystem confinement protocol. Windows case spellings may remain distinct
  registrations; macOS execution cannot establish Windows/Linux runtime evidence.
- **Shutdown boundary:** token Drop/stop requests cancellation; joined watcher
  or ancestor disposal releases callback captures and waits for the producer.
  An OS read that is already blocked must return before joining; there is no
  hard wall-clock shutdown deadline or rollback of callback external effects.
- **Include boundary:** one registered path triggers refresh of the supplied
  Include's source graph; paths are not discovered automatically. Stop/Drop
  suppresses follow-up but retained admitted work must finish before watcher or
  ancestor cleanup joins. Returned Include errors and Report failures use bounded
  WatchEvent history; ordinary core cleanup failures use Runtime diagnostics.
  The adapter does not dispose Include, and linked replacement does not flush.
- **Replacement boundary:** whole-batch declaration/type/optional preflight
  precedes unload. Core dependency-ready mapping/validation can still fail
  afterward. This is not a transaction over arbitrary side effects; failed
  replacement requires explicit update or another candidate to recover.

## Create offline scaffolding

Selected Create S0–S2 supplies a small local-checkout generator, pending
independent acceptance. See the [guide](en/guides/create.md) and
[decisions](decision.md#create-offline-scaffolding).

- **Deliberate Rust adaptation:** explicit name/native path arguments and a
  synchronous Result-based API replace interactive JS/package-manager flows.
  A fixed core/logger/loader template supplies an application the user edits;
  this does not emulate every upstream Create option.
- **Publication boundary:** Cordis crates remain unpublished. Generated canonical
  absolute path dependencies require the checkout to remain available; moving it
  requires manifest edits. Canonical dependency paths must be Unicode; destination
  arguments preserve native OS bytes but the filesystem may reject them
  (macOS EILSEQ was exercised). The three bundled Cordis dependency names are
  unavailable as project names because their `0.0.0` Cargo identities collide.
  Checkout validation recognizes the current
  literal manifest/layout spelling, not arbitrary TOML or source integrity.
  Offline Cargo still requires cached third-party
  dependencies, and dependency restoration is an explicit user operation.
- **Filesystem boundary:** a destination must be absent or empty, must not be a
  symlink, and must have an existing parent. Staging and one rename avoid placing
  a partially rendered template at the final path; an empty-directory rename may
  fail on a platform/filesystem. Portable metadata/rename operations do not
  guarantee confinement against hostile parent replacement, protection against
  every concurrent writer, or durability through power loss.
- **Deferred capability:** Git initialization, dependency installation,
  application launch, remote templates/downloaders, archive/cache management and
  Include/HMR variants are unselected. Rust does not inherently prevent those
  features; they are outside this minimal delivery.
- **Missing evidence:** runtime filesystem execution is macOS only. The selected
  toolchain checks do not establish Linux/Windows execution or a stable published
  API. Create adds no new cross-language core lifecycle evidence.
