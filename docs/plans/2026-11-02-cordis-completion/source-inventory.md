# Pinned satellite source inventory

Date: 2026-11-02. TypeScript pin: 35fdc969e4b6c48c48ed44717c7a018324e4dd88.

This is a **syntactic declaration inventory**, not an executed test count or
a claim of complete case-level parity review. It records each line beginning
with it/test followed by a call or modifier in the package spec files.
Parameterized declarations, loops, runIf and multiple assertions expand
into additional behavioral variants. At each design gate read the complete
test body and record each variant as tested equivalent, Rust representation,
reviewed exclusion or named remaining work. A declaration is not done merely
because its package has a plan.

Core has its existing [144-declaration ledger](../../m0-cases.md) and
additional events.types.ts checks; B0 reconciles that evidence separately.
The seven packages below exhaust packages other than core at this pin.
The root package is private workspace tooling; its external/* workspace glob
has no tracked external directory at this pin. Tooling and transitive packages
are not extra satellites. Inventory any later additions before expanding scope.

| Package | Spec files | Declaration lines | Workstream |
|---|---:|---:|---|
| timer | 1 | 22 | [T0–T5](timer.md) |
| logger-console | 1 | 5 | [C0–C3](logger.md) |
| loader | 11 | 140 | [L0–L5 / G0–G1](loader.md) |
| group | 0 | 0 | [G0–G1](group.md) |
| include | 7 | 89 | [I0–I4](include.md) |
| hmr | 2 | 53 | [H0–H3](hmr.md) |
| create | 1 | 14 | [S0–S2](create.md) |

**Total: 323 declaration lines across 23 spec files.** Group has no separate
suite; its three-line entry point re-exports loader Group. Its behavioral
coverage lives in loader group/evaluate/commit/isolation suites.

The old timer design mentions 20 tests; this pinned file has 22 declaration
lines. Neither number is interchangeable with the 15 existing Rust tests.

## Source reading map

| Package | Primary source |
|---|---|
| timer | [src/index.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/src/index.ts) |
| logger-console | [src/index.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/src/index.ts), [src/shared.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/src/shared.ts), [src/browser.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/src/browser.ts) |
| loader | [src/index.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/src/index.ts), [src/config/entry.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/src/config/entry.ts), [src/config/tree.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/src/config/tree.ts), [src/config/group.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/src/config/group.ts), [src/config/isolate.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/src/config/isolate.ts), [src/config/utils.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/src/config/utils.ts), [src/resolve.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/src/resolve.ts), [src/internal.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/src/internal.ts) |
| group | [src/index.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/group/src/index.ts) |
| include | [src/index.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/src/index.ts), [src/patch.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/src/patch.ts), [src/journal.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/src/journal.ts) |
| hmr | [src/index.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/src/index.ts), [src/error.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/src/error.ts) |
| create | [src/index.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/src/index.ts), [src/bin.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/src/bin.ts) |

The table is the source reading map for the workstream, not a claim that every
line has undergone a formal audit. Planning inspected the main ownership,
rendering, entry, patch/journal and reload paths; implementation gates must
finish detailed review of all paths and variants they change.

## timer

Plan: [T0–T5](timer.md). Status: inventoried; execution/disposition is tracked by that plan.

### index.spec.ts

[packages/timer/tests/index.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts) — 22 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [22](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L22) | it(&#x27;basic support&#x27;, withContext(async (ctx) =&gt; { |
| [32](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L32) | it(&#x27;dispose&#x27;, withContext(async (ctx) =&gt; { |
| [41](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L41) | it(&#x27;promise&#x27;, withContext(async (ctx) =&gt; { |
| [59](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L59) | it(&#x27;basic support&#x27;, withContext(async (ctx) =&gt; { |
| [72](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L72) | it(&#x27;async iterator (manual return)&#x27;, withContext(async (ctx) =&gt; { |
| [95](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L95) | it(&#x27;async iterator (concurrent reads)&#x27;, withContext(async (ctx) =&gt; { |
| [117](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L117) | it(&#x27;async iterator (return with concurrent reads)&#x27;, withContext(async (ctx) =&gt; { |
| [130](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L130) | it(&#x27;async iterator (throw with concurrent reads)&#x27;, withContext(async (ctx) =&gt; { |
| [140](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L140) | it(&#x27;async iterator (manual throw)&#x27;, withContext(async (ctx) =&gt; { |
| [163](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L163) | it(&#x27;async iterator (break return)&#x27;, withContext(async (ctx) =&gt; { |
| [187](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L187) | it(&#x27;async iterator (break throw)&#x27;, withContext(async (ctx) =&gt; { |
| [211](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L211) | it(&#x27;async iterator (context dispose)&#x27;, withContext(async (ctx) =&gt; { |
| [240](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L240) | it(&#x27;async iterator (context dispose with concurrent reads)&#x27;, withContext(async (ctx) =&gt; { |
| [257](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L257) | it(&#x27;basic support&#x27;, withContext(async (ctx) =&gt; { |
| [272](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L272) | it(&#x27;trailing mode&#x27;, withContext(async (ctx) =&gt; { |
| [286](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L286) | it(&#x27;no trailing mode&#x27;, withContext(async (ctx) =&gt; { |
| [300](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L300) | it(&#x27;context dispose&#x27;, withContext(async (ctx) =&gt; { |
| [320](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L320) | it(&#x27;disposed&#x27;, withContext(async (ctx) =&gt; { |
| [332](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L332) | it(&#x27;disposed with noTrailing&#x27;, withContext(async (ctx) =&gt; { |
| [345](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L345) | it(&#x27;basic support&#x27;, withContext(async (ctx) =&gt; { |
| [360](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L360) | it(&#x27;disposed&#x27;, withContext(async (ctx) =&gt; { |
| [370](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/timer/tests/index.spec.ts#L370) | it(&#x27;context dispose&#x27;, withContext(async (ctx) =&gt; { |

## logger-console

Plan: [C0–C3](logger.md). Status: inventoried; execution/disposition is tracked by that plan.

### index.spec.ts

[packages/logger-console/tests/index.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/tests/index.spec.ts) — 5 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [27](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/tests/index.spec.ts#L27) | it(&#x27;format error&#x27;, () =&gt; { |
| [36](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/tests/index.spec.ts#L36) | it(&#x27;format object&#x27;, () =&gt; { |
| [42](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/tests/index.spec.ts#L42) | it(&#x27;custom formatter&#x27;, () =&gt; { |
| [49](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/tests/index.spec.ts#L49) | it(&#x27;log levels&#x27;, () =&gt; { |
| [59](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/logger-console/tests/index.spec.ts#L59) | it(&#x27;label style&#x27;, () =&gt; { |

## loader

Plan: [L0–L5 / G0–G1](loader.md). Status: inventoried; execution/disposition is tracked by that plan.

### commit.spec.ts

[packages/loader/tests/commit.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts) — 10 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [26](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L26) | it(&#x27;create reports a created entry with its id assigned&#x27;, async () =&gt; { |
| [43](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L43) | it(&#x27;update reports the new options together with the previous ones&#x27;, async () =&gt; { |
| [55](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L55) | it(&#x27;move reports the target group&#x27;, async () =&gt; { |
| [65](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L65) | it(&#x27;update in place does not report a move&#x27;, async () =&gt; { |
| [72](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L72) | it(&#x27;fiber.update from inside the plugin reports a config change&#x27;, async () =&gt; { |
| [82](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L82) | it(&#x27;remove reports the removed entry&#x27;, async () =&gt; { |
| [93](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L93) | it(&#x27;does not mistake a child fiber disposing itself for the entry (#19)&#x27;, async () =&gt; { |
| [107](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L107) | it(&#x27;does not report anything for reads&#x27;, async () =&gt; { |
| [138](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L138) | it(&#x27;keeps an entry that another group adopted&#x27;, async () =&gt; { |
| [159](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/commit.spec.ts#L159) | it(&#x27;does not persist child disposal while the tree loses its loader&#x27;, async () =&gt; { |

### evaluate.spec.ts

[packages/loader/tests/evaluate.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts) — 37 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [39](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L39) | it(&#x27;creates no outer fiber for an entry without inject&#x27;, async () =&gt; { |
| [48](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L48) | it(&#x27;nests the inner fiber under the outer fiber&#x27;, async () =&gt; { |
| [63](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L63) | it(&#x27;evaluates disabled with the entry inject&#x27;, async () =&gt; { |
| [83](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L83) | it(&#x27;decides disabled before the plugin dependencies&#x27;, async () =&gt; { |
| [100](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L100) | it(&#x27;waits for the entry inject while it is missing&#x27;, async () =&gt; { |
| [118](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L118) | it(&#x27;treats a failing expression as disabled&#x27;, async () =&gt; { |
| [130](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L130) | it(&#x27;evaluates inject values in the parent context&#x27;, async () =&gt; { |
| [147](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L147) | it(&#x27;cannot read the entry inject from an inject value&#x27;, async () =&gt; { |
| [160](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L160) | it(&#x27;keeps an inject value on the outer fiber when the plugin declares the service itself&#x27;, async () =&gt; { |
| [175](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L175) | it(&#x27;evaluates intercept with the entry inject, for the plugin only&#x27;, async () =&gt; { |
| [189](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L189) | it(&#x27;updates intercept in place&#x27;, async () =&gt; { |
| [205](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L205) | it(&#x27;keeps the options when a dependency is lost and recovered&#x27;, async () =&gt; { |
| [228](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L228) | it(&#x27;reloads the outer fiber when the verdict changes&#x27;, async () =&gt; { |
| [249](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L249) | it(&#x27;rebuilds the entry when inject changes&#x27;, async () =&gt; { |
| [270](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L270) | it(&#x27;disposes the outer fiber when the plugin disposes itself&#x27;, async () =&gt; { |
| [286](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L286) | it(&#x27;disposes both fibers when the entry is removed&#x27;, async () =&gt; { |
| [301](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L301) | it(&#x27;keeps a group structural and passes its verdict to the children&#x27;, async () =&gt; { |
| [321](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L321) | it(&#x27;keeps the entry isolate without an inject&#x27;, async () =&gt; { |
| [339](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L339) | it(&#x27;rebuilds without a write-back when inject is added&#x27;, async () =&gt; { |
| [352](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L352) | it(&#x27;drops an activation whose dependency is lost during the import&#x27;, async () =&gt; { |
| [380](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L380) | it(&#x27;re-evaluates a failing inject value on refresh&#x27;, async () =&gt; { |
| [394](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L394) | it.each([false, true])(&#x27;caches the verdict independently of reads and service mutations (group=%s)&#x27;, async (group) =&gt; { |
| [427](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L427) | it(&#x27;does not repeat a failing disabled expression when its verdict is read&#x27;, async () =&gt; { |
| [438](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L438) | it(&#x27;evaluates a changed disabled expression once in either direction&#x27;, async () =&gt; { |
| [462](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L462) | it(&#x27;invalidates the verdict before an asynchronous dependency unload&#x27;, async () =&gt; { |
| [496](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L496) | it(&#x27;keeps a disabled group verdict undecided when its dependency disappears&#x27;, async () =&gt; { |
| [513](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L513) | it.each([&#x27;disabled&#x27;, &#x27;inject&#x27;])(&#x27;re-evaluates %s in the new parent context after a move&#x27;, async (field) =&gt; { |
| [540](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L540) | it(&#x27;re-evaluates the verdict after changing the entry isolation&#x27;, async () =&gt; { |
| [558](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L558) | it.each([&#x27;import&#x27;, &#x27;patch&#x27;])(&#x27;drops entries removed or disabled during %s&#x27;, async (window) =&gt; { |
| [649](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L649) | it(&#x27;does not start the plugin when intercept cannot be evaluated&#x27;, async () =&gt; { |
| [661](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L661) | it(&#x27;disposes the plugin when intercept stops evaluating on an update&#x27;, async () =&gt; { |
| [689](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L689) | it(&#x27;stops a group whose intercept stops evaluating on an update&#x27;, async () =&gt; { |
| [708](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L708) | it(&#x27;does not evaluate intercept while disabled&#x27;, async () =&gt; { |
| [721](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L721) | it(&#x27;cannot read a service outside the parent context and the entry inject in disabled&#x27;, async () =&gt; { |
| [734](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L734) | it(&#x27;cannot read a service provided by a sibling in an inject value&#x27;, async () =&gt; { |
| [745](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L745) | it(&#x27;evaluates config with the plugin inject&#x27;, async () =&gt; { |
| [759](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/evaluate.spec.ts#L759) | it(&#x27;cannot read a service outside every inject in config&#x27;, async () =&gt; { |

### group.spec.ts

[packages/loader/tests/group.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts) — 13 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [27](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L27) | it(&#x27;initialize&#x27;, async () =&gt; { |
| [52](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L52) | it(&#x27;disable inner&#x27;, async () =&gt; { |
| [60](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L60) | it(&#x27;disable outer&#x27;, async () =&gt; { |
| [68](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L68) | it(&#x27;enable inner&#x27;, async () =&gt; { |
| [76](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L76) | it(&#x27;enable outer&#x27;, async () =&gt; { |
| [109](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L109) | it(&#x27;initialize&#x27;, async () =&gt; { |
| [138](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L138) | it(&#x27;enabled -&gt; enabled&#x27;, async () =&gt; { |
| [146](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L146) | it(&#x27;enabled -&gt; disabled&#x27;, async () =&gt; { |
| [154](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L154) | it(&#x27;disabled -&gt; disabled&#x27;, async () =&gt; { |
| [162](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L162) | it(&#x27;disabled -&gt; enabled&#x27;, async () =&gt; { |
| [187](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L187) | it(&#x27;initialize&#x27;, async () =&gt; { |
| [202](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L202) | it(&#x27;config update settles children&#x27;, async () =&gt; { |
| [253](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/group.spec.ts#L253) | it(&#x27;initialize&#x27;, async () =&gt; { |

### import.spec.ts

[packages/loader/tests/import.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/import.spec.ts) — 2 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [25](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/import.spec.ts#L25) | it(&#x27;keep the hints of Node after the first line&#x27;, async () =&gt; { |
| [42](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/import.spec.ts#L42) | it(&#x27;leave a module imported by another one to Node&#x27;, async () =&gt; { |

### index.spec.ts

[packages/loader/tests/index.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts) — 19 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [10](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L10) | it(&#x27;evaluates the expression instead of treating the node as truthy&#x27;, async () =&gt; { |
| [43](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L43) | it(&#x27;loader initiate&#x27;, async () =&gt; { |
| [71](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L71) | it(&#x27;loader update&#x27;, async () =&gt; { |
| [90](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L90) | it(&#x27;plugin self-update&#x27;, async () =&gt; { |
| [103](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L103) | it(&#x27;plugin self-dispose&#x27;, async () =&gt; { |
| [139](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L139) | it(&#x27;pending&#x27;, async () =&gt; { |
| [162](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L162) | it(&#x27;resolved&#x27;, async () =&gt; { |
| [188](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L188) | it(&#x27;on load&#x27;, async () =&gt; { |
| [201](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L201) | it(&#x27;on config update&#x27;, async () =&gt; { |
| [215](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L215) | it(&#x27;on an id containing the separator&#x27;, async () =&gt; { |
| [247](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L247) | it(&#x27;wait for dependencies&#x27;, async () =&gt; { |
| [261](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L261) | it(&#x27;evaluate after dependencies are ready&#x27;, async () =&gt; { |
| [279](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L279) | it(&#x27;re-evaluate when dependencies reload&#x27;, async () =&gt; { |
| [287](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L287) | it(&#x27;keep the source config unevaluated&#x27;, () =&gt; { |
| [307](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L307) | it(&#x27;skip nested plugins&#x27;, async () =&gt; { |
| [344](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L344) | it(&#x27;fail with dependencies available&#x27;, async () =&gt; { |
| [360](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L360) | it(&#x27;recover through update&#x27;, async () =&gt; { |
| [383](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L383) | it(&#x27;persist an update to a pending fiber&#x27;, async () =&gt; { |
| [436](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/index.spec.ts#L436) | it(&#x27;keep children enabled when the parent loses a dependency&#x27;, async () =&gt; { |

### internal.spec.ts

[packages/loader/tests/internal.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/internal.spec.ts) — 1 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [5](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/internal.spec.ts#L5) | it(&#x27;tags the running loader with the resolver signature it accepts&#x27;, () =&gt; { |

### isolate-provide.spec.ts

[packages/loader/tests/isolate-provide.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts) — 7 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [51](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts#L51) | it(&#x27;keeps self access while the provision context changes realms&#x27;, async () =&gt; { |
| [69](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts#L69) | it(&#x27;reloads a self-provider after an isolate-only move&#x27;, async () =&gt; { |
| [106](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts#L106) | it(&#x27;keeps an impl that reused the abandoned symbol&#x27;, async () =&gt; { |
| [143](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts#L143) | it(&#x27;retains a moved provider until its consumers finish cleanup&#x27;, async () =&gt; { |
| [186](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts#L186) | it(&#x27;keeps impls on intermediate symbols across repeated moves&#x27;, async () =&gt; { |
| [245](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts#L245) | it(&#x27;does not delete another realm when one impl moves&#x27;, async () =&gt; { |
| [280](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate-provide.spec.ts#L280) | it.each([&#x27;fiber&#x27;, &#x27;provide&#x27;])(&#x27;notifies the retained realm on %s disposal after a blocked move&#x27;, async (mode) =&gt; { |

### isolate.spec.ts

[packages/loader/tests/isolate.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts) — 21 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [38](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L38) | it(&#x27;initiate&#x27;, async () =&gt; { |
| [47](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L47) | it(&#x27;add isolate on injector (relevant)&#x27;, async () =&gt; { |
| [59](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L59) | it(&#x27;add isolate on injector (irrelevant)&#x27;, async () =&gt; { |
| [72](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L72) | it(&#x27;remove isolate on injector (relevant)&#x27;, async () =&gt; { |
| [84](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L84) | it(&#x27;remove isolate on injector (irrelevant)&#x27;, async () =&gt; { |
| [94](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L94) | it(&#x27;add isolate on provider (relevant)&#x27;, async () =&gt; { |
| [106](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L106) | it(&#x27;add isolate on provider (irrelevant)&#x27;, async () =&gt; { |
| [119](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L119) | it(&#x27;remove isolate on provider (relevant)&#x27;, async () =&gt; { |
| [131](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L131) | it(&#x27;remove isolate on provider (irrelevant)&#x27;, async () =&gt; { |
| [172](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L172) | it(&#x27;add isolate group&#x27;, async () =&gt; { |
| [201](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L201) | it(&#x27;update isolate group (no change)&#x27;, async () =&gt; { |
| [218](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L218) | it(&#x27;realm reference&#x27;, async () =&gt; { |
| [251](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L251) | it(&#x27;special case: nested realms&#x27;, async () =&gt; { |
| [326](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L326) | it(&#x27;special case: change provider&#x27;, async () =&gt; { |
| [390](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L390) | it(&#x27;special case: change injector&#x27;, async () =&gt; { |
| [490](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L490) | it(&#x27;initiate&#x27;, async () =&gt; { |
| [507](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L507) | it(&#x27;transfer injector into group&#x27;, async () =&gt; { |
| [515](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L515) | it(&#x27;transfer provider into group&#x27;, async () =&gt; { |
| [523](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L523) | it(&#x27;transfer injector out of group&#x27;, async () =&gt; { |
| [531](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L531) | it(&#x27;transfer provider out of group&#x27;, async () =&gt; { |
| [541](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/isolate.spec.ts#L541) | it(&#x27;keeps the committed views of a parent and a pinned child across a realm move&#x27;, async () =&gt; { |

### rebuild.spec.ts

[packages/loader/tests/rebuild.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts) — 12 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [44](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L44) | it.each([undefined, [&#x27;settings&#x27;]])(&#x27;rebuilds an active entry when its name changes (inject=%s)&#x27;, async (inject) =&gt; { |
| [58](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L58) | it.each([&#x27;import&#x27;, &#x27;patch&#x27;])(&#x27;uses the new name when it changes during the %s&#x27;, async (window) =&gt; { |
| [97](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L97) | it.each([undefined, [&#x27;settings&#x27;]])(&#x27;joins a second rename while the first disposal is pending (inject=%s)&#x27;, async (inject) =&gt; { |
| [117](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L117) | it(&#x27;applies nothing when the entry is removed during the disposal&#x27;, async () =&gt; { |
| [139](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L139) | it(&#x27;rebuilds when the plugin renames its own entry&#x27;, async () =&gt; { |
| [159](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L159) | it(&#x27;starts a pending entry when inject is cleared&#x27;, async () =&gt; { |
| [186](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L186) | it(&#x27;reapplies when inject gains a service that is already provided&#x27;, async () =&gt; { |
| [213](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L213) | it(&#x27;keeps the new outer fiber pending and drops the old listener when inject is missing&#x27;, async () =&gt; { |
| [243](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L243) | it(&#x27;joins a second inject update while the first rebuild is disposing&#x27;, async () =&gt; { |
| [280](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L280) | it.each([&#x27;import&#x27;, &#x27;failed import&#x27;, &#x27;context patch&#x27;])(&#x27;uses the latest name during a pending %s&#x27;, async (stage) =&gt; { |
| [322](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L322) | it(&#x27;does not apply a fiber removed while rebuild is disposing&#x27;, async () =&gt; { |
| [349](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/rebuild.spec.ts#L349) | it(&#x27;keeps a rebuild started during apply until its disposer settles&#x27;, async () =&gt; { |

### resolve.spec.ts

[packages/loader/tests/resolve.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts) — 12 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [47](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L47) | it(&#x27;resolves bare specifiers against the project, not the loader&#x27;, async () =&gt; { |
| [57](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L57) | it(&#x27;keeps subpath imports and self references working&#x27;, async () =&gt; { |
| [64](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L64) | it(&#x27;anchors the helper at the package.json, not at the base url&#x27;, async () =&gt; { |
| [71](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L71) | it(&#x27;makes the helper directory ignore itself&#x27;, async () =&gt; { |
| [78](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L78) | it(&#x27;reuses one helper per scope&#x27;, async () =&gt; { |
| [85](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L85) | it(&#x27;declines when there is no project to anchor to&#x27;, async () =&gt; { |
| [133](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L133) | it(&#x27;reuses an identical helper without attempting rename&#x27;, async () =&gt; { |
| [146](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L146) | it.each([&#x27;EACCES&#x27;, &#x27;EPERM&#x27;, &#x27;EBUSY&#x27;])(&#x27;retries a stale helper after %s&#x27;, async (code) =&gt; { |
| [156](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L156) | it.each([ |
| [187](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L187) | it(&#x27;routes bare specifiers to the project helper&#x27;, async () =&gt; { |
| [194](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L194) | it(&#x27;still resolves relative specifiers against the base url&#x27;, async () =&gt; { |
| [202](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/resolve.spec.ts#L202) | it.runIf(process.platform === &#x27;win32&#x27;)(&#x27;accepts a drive-letter path&#x27;, async () =&gt; { |

### validate.spec.ts

[packages/loader/tests/validate.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/validate.spec.ts) — 6 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [9](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/validate.spec.ts#L9) | it(&#x27;accept valid options&#x27;, () =&gt; { |
| [27](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/validate.spec.ts#L27) | it(&#x27;reject a missing or invalid field&#x27;, () =&gt; { |
| [41](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/validate.spec.ts#L41) | it(&#x27;accept an expression only for disabled&#x27;, () =&gt; { |
| [52](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/validate.spec.ts#L52) | it(&#x27;reject invalid options at runtime before they are committed&#x27;, async () =&gt; { |
| [71](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/validate.spec.ts#L71) | it(&#x27;check the entries of a group passed at runtime as a whole&#x27;, async () =&gt; { |
| [96](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/loader/tests/validate.spec.ts#L96) | it(&#x27;unload an entry whose options turn invalid, and reload it once fixed&#x27;, async () =&gt; { |

## group

Plan: [G0–G1](group.md). Status: inventoried; execution/disposition is tracked by that plan.

No standalone spec files. Review loader Group behavior before deciding a Rust facade.

## include

Plan: [I0–I4](include.md). Status: inventoried; execution/disposition is tracked by that plan.

### location.spec.ts

[packages/include/tests/location.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts) — 39 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [22](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L22) | it(&#x27;points a validation issue at the value in the file&#x27;, async () =&gt; { |
| [39](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L39) | it(&#x27;points a missing value at the key that should hold it&#x27;, async () =&gt; { |
| [58](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L58) | it(&#x27;locates entries nested in groups&#x27;, async () =&gt; { |
| [76](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L76) | it(&#x27;locates patched values in the file that declares the patch&#x27;, async () =&gt; { |
| [100](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L100) | it(&#x27;locates overrides of inserted entries in the override&#x27;, async () =&gt; { |
| [123](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L123) | it(&#x27;locates a config value at its anchor&#x27;, async () =&gt; { |
| [146](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L146) | it(&#x27;points a missing value under an alias at the key that holds it&#x27;, async () =&gt; { |
| [160](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L160) | it(&#x27;locates an entry option at its anchor&#x27;, async () =&gt; { |
| [174](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L174) | it(&#x27;locates an expression at its anchor&#x27;, async () =&gt; { |
| [192](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L192) | it(&#x27;locates entries of a group config replaced by a patch&#x27;, async () =&gt; { |
| [215](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L215) | it(&#x27;ignores patches that do not apply&#x27;, async () =&gt; { |
| [234](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L234) | it(&#x27;follows patches across nested includes&#x27;, async () =&gt; { |
| [274](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L274) | it(&#x27;leaves a config passed at runtime unlocated&#x27;, async () =&gt; { |
| [280](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L280) | it(&#x27;leaves entries outside any file untraced&#x27;, async () =&gt; { |
| [290](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L290) | it(&#x27;points invalid fields at their values&#x27;, async () =&gt; { |
| [317](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L317) | it(&#x27;accepts an expression for disabled only&#x27;, async () =&gt; { |
| [331](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L331) | it(&#x27;unloads an entry that turns invalid, without writing to the file&#x27;, async () =&gt; { |
| [349](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L349) | it(&#x27;locates entries nested in groups&#x27;, async () =&gt; { |
| [368](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L368) | it(&#x27;rejects a file with an entry that is not an object&#x27;, async () =&gt; { |
| [393](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L393) | it(&#x27;leaves invalid options passed at runtime unlocated&#x27;, async () =&gt; { |
| [402](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L402) | it(&#x27;points a plugin that cannot be found at its name&#x27;, async () =&gt; { |
| [432](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L432) | it(&#x27;leaves a module the plugin imports to the plugin&#x27;, async () =&gt; { |
| [440](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L440) | it(&#x27;tells the two apart without the internal loader&#x27;, async () =&gt; { |
| [470](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L470) | it(&#x27;leaves other import errors to point into the plugin&#x27;, async () =&gt; { |
| [479](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L479) | it(&#x27;leaves entries outside any file untraced&#x27;, async () =&gt; { |
| [490](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L490) | it.each([ |
| [510](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L510) | it(&#x27;maps positions across lines and escapes&#x27;, async () =&gt; { |
| [540](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L540) | it(&#x27;names the anonymous functions of an expression by their location&#x27;, async () =&gt; { |
| [556](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L556) | it(&#x27;keeps the frames of what the expression calls&#x27;, async () =&gt; { |
| [571](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L571) | it(&#x27;points an expression that does not parse at its start&#x27;, async () =&gt; { |
| [586](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L586) | it(&#x27;locates entries nested in groups&#x27;, async () =&gt; { |
| [606](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L606) | it(&#x27;locates an expression written as an object in JSON&#x27;, async () =&gt; { |
| [618](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L618) | it(&#x27;points a failure on update at the new expression&#x27;, async () =&gt; { |
| [635](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L635) | it(&#x27;leaves an expression changed at runtime unlocated&#x27;, async () =&gt; { |
| [646](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L646) | it(&#x27;leaves entries outside any file untraced&#x27;, async () =&gt; { |
| [655](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L655) | it(&#x27;points parse errors at the source&#x27;, async () =&gt; { |
| [667](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L667) | it(&#x27;points a validation issue at the value in the file&#x27;, async () =&gt; { |
| [680](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L680) | it(&#x27;points syntax errors at the source&#x27;, async () =&gt; { |
| [691](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/location.spec.ts#L691) | it(&#x27;points a non-array file at its value&#x27;, async () =&gt; { |

### parse.spec.ts

[packages/include/tests/parse.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/parse.spec.ts) — 2 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [9](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/parse.spec.ts#L9) | it(&#x27;resolves scalars by the core schema whatever version the file declares&#x27;, async () =&gt; { |
| [27](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/parse.spec.ts#L27) | it(&#x27;writes expressions back with their tag&#x27;, async () =&gt; { |

### patch.spec.ts

[packages/include/tests/patch.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts) — 11 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [23](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L23) | it(&#x27;should load without patches&#x27;, async () =&gt; { |
| [39](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L39) | it(&#x27;should disable an entry via patch&#x27;, async () =&gt; { |
| [59](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L59) | it(&#x27;should override config via patch&#x27;, async () =&gt; { |
| [79](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L79) | it(&#x27;should warn on name mismatch and skip patch&#x27;, async () =&gt; { |
| [103](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L103) | it(&#x27;should warn on non-existent id&#x27;, async () =&gt; { |
| [123](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L123) | it(&#x27;should insert entries into root group&#x27;, async () =&gt; { |
| [148](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L148) | it(&#x27;should insert entries into a specific group&#x27;, async () =&gt; { |
| [172](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L172) | it(&#x27;should warn when inserting into a non-group entry&#x27;, async () =&gt; { |
| [197](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L197) | it(&#x27;should apply multiple patches&#x27;, async () =&gt; { |
| [223](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L223) | it(&#x27;should let a later patch address a row inserted by an earlier patch&#x27;, async () =&gt; { |
| [244](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/patch.spec.ts#L244) | it(&#x27;should validate name consistency (matching name is ok)&#x27;, async () =&gt; { |

### path.spec.ts

[packages/include/tests/path.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/path.spec.ts) — 1 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [12](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/path.spec.ts#L12) | it.runIf(process.platform === &#x27;win32&#x27;)(&#x27;accepts a drive-letter path&#x27;, async () =&gt; { |

### reload.spec.ts

[packages/include/tests/reload.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/reload.spec.ts) — 5 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [66](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/reload.spec.ts#L66) | it(&#x27;re-applies patches on every reload&#x27;, async () =&gt; { |
| [83](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/reload.spec.ts#L83) | it(&#x27;rebuilds an entry whose name changes in the file&#x27;, async () =&gt; { |
| [96](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/reload.spec.ts#L96) | it(&#x27;reverts an entry when its patch is removed&#x27;, async () =&gt; { |
| [116](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/reload.spec.ts#L116) | it(&#x27;keeps the tree reloadable after an invalid file&#x27;, async () =&gt; { |
| [134](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/reload.spec.ts#L134) | it(&#x27;never overwrites an existing but unparsable file with initial&#x27;, async () =&gt; { |

### sync.spec.ts

[packages/include/tests/sync.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts) — 27 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [12](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L12) | it(&#x27;coalesces consecutive writes into the latest state&#x27;, async () =&gt; { |
| [25](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L25) | it(&#x27;keeps file and tree consistent when a refresh lands inside a write&#x27;, async () =&gt; { |
| [37](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L37) | it(&#x27;does not write back when the file removes an entry&#x27;, async () =&gt; { |
| [51](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L51) | it(&#x27;does not write back when the file disables an entry&#x27;, async () =&gt; { |
| [62](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L62) | it(&#x27;merges a file edit and a runtime edit on different entries&#x27;, async () =&gt; { |
| [81](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L81) | it(&#96;merges a watcher-driven file edit racing a runtime edit (${order})&#96;, async () =&gt; { |
| [101](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L101) | it(&#x27;lets the file win a conflict on the same entry and reports it&#x27;, async () =&gt; { |
| [116](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L116) | it(&#x27;combines a move in the file with an edit at runtime&#x27;, async () =&gt; { |
| [132](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L132) | it(&#x27;never writes over a file it cannot parse&#x27;, async () =&gt; { |
| [151](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L151) | it(&#x27;routes changes to patch-owned keys into the parent config&#x27;, async () =&gt; { |
| [179](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L179) | it(&#x27;routes changes to inserted entries into their patch&#x27;, async () =&gt; { |
| [243](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L243) | it(&#96;persists each key to its effective owner (${scenario.name})&#96;, async () =&gt; { |
| [272](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L272) | it(&#x27;routes past a name-mismatched override&#x27;, async () =&gt; { |
| [326](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L326) | it(&#x27;routes changes into the patch&#x27;, async () =&gt; { |
| [337](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L337) | it(&#x27;places new entries into the patch&#x27;, async () =&gt; { |
| [348](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L348) | it(&#x27;removes entries from the patch&#x27;, async () =&gt; { |
| [359](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L359) | it(&#x27;keeps anonymous entries across file edits&#x27;, async () =&gt; { |
| [370](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L370) | it(&#x27;routes an edit to an entry whose group moved in the same batch&#x27;, async () =&gt; { |
| [401](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L401) | it(&#x27;keeps anonymous entries stable across reloads without writing&#x27;, async () =&gt; { |
| [425](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L425) | it(&#x27;does not give a root anonymous plugin the nested plugin\&#x27;s id&#x27;, async () =&gt; { |
| [457](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L457) | it(&#x27;keeps an anonymous group and its child stable when a neighbor changes&#x27;, async () =&gt; { |
| [483](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L483) | it(&#x27;does not reuse an id that the file now writes explicitly&#x27;, async () =&gt; { |
| [504](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L504) | it(&#x27;does not reuse an id owned by an explicit patch insert&#x27;, async () =&gt; { |
| [532](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L532) | it(&#x27;keeps runtime changes in memory when the file is read-only&#x27;, async () =&gt; { |
| [555](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L555) | it(&#x27;removes a nested entry through its composite id&#x27;, async () =&gt; { |
| [568](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L568) | it(&#x27;retries a write that failed earlier when disposing&#x27;, async () =&gt; { |
| [584](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/sync.spec.ts#L584) | it(&#x27;flushes pending changes on dispose&#x27;, async () =&gt; { |

### write.spec.ts

[packages/include/tests/write.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/write.spec.ts) — 4 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [50](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/write.spec.ts#L50) | it(&#x27;retries a rename that fails transiently&#x27;, async () =&gt; { |
| [64](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/write.spec.ts#L64) | it(&#x27;never overwrites a change that lands while it waits to retry&#x27;, async () =&gt; { |
| [83](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/write.spec.ts#L83) | it(&#x27;does not retry other errors, but still flushes on dispose&#x27;, async () =&gt; { |
| [102](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/include/tests/write.spec.ts#L102) | it(&#x27;gives up after a bounded number of attempts and keeps the change&#x27;, async () =&gt; { |

## hmr

Plan: [H0–H3](hmr.md). Status: inventoried; execution/disposition is tracked by that plan.

### index.spec.ts

[packages/hmr/tests/index.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts) — 49 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [171](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L171) | it(&#x27;should load plugin and respond to events&#x27;, () =&gt; { |
| [176](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L176) | it(&#x27;should reload plugin when file changes&#x27;, async () =&gt; { |
| [189](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L189) | it(&#x27;should handle reverting file back to original&#x27;, async () =&gt; { |
| [202](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L202) | it(&#x27;should emit hmr/reload event on reload&#x27;, async () =&gt; { |
| [216](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L216) | it(&#x27;should properly dispose old plugin effects on reload&#x27;, async () =&gt; { |
| [259](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L259) | it(&#x27;should load multiple plugins&#x27;, () =&gt; { |
| [264](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L264) | it(&#x27;should reload changed plugin without affecting others&#x27;, async () =&gt; { |
| [276](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L276) | it(&#x27;should handle simultaneous changes to both plugins&#x27;, async () =&gt; { |
| [317](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L317) | it(&#x27;should load plugin with dependency&#x27;, () =&gt; { |
| [321](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L321) | it(&#x27;should reload plugin when its dependency changes&#x27;, async () =&gt; { |
| [334](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L334) | it(&#x27;should reload plugin when plugin file itself changes&#x27;, async () =&gt; { |
| [349](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L349) | it(&#x27;should not reload a sibling that only shares an unchanged module&#x27;, async () =&gt; { |
| [397](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L397) | it(&#x27;should load plugin normally&#x27;, () =&gt; { |
| [401](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L401) | it(&#x27;should rollback on syntax error and keep old plugin working&#x27;, async () =&gt; { |
| [415](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L415) | it(&#x27;should recover after fixing the error&#x27;, async () =&gt; { |
| [458](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L458) | it(&#x27;should batch rapid changes within debounce window&#x27;, async () =&gt; { |
| [504](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L504) | it(&#x27;should handle config file update to disable plugin&#x27;, async () =&gt; { |
| [565](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L565) | it(&#x27;keeps watching the file across its own rename-based writes&#x27;, async () =&gt; { |
| [590](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L590) | it(&#x27;logs instead of rejecting when the config file is unparsable&#x27;, async () =&gt; { |
| [663](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L663) | it(&#x27;should load service plugin&#x27;, async () =&gt; { |
| [672](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L672) | it(&#x27;should reload service plugin on change&#x27;, async () =&gt; { |
| [705](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L705) | it(&#x27;should maintain entry association after reload&#x27;, async () =&gt; { |
| [750](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L750) | it(&#x27;should handle many rapid reloads without crashing&#x27;, async () =&gt; { |
| [785](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L785) | it(&#x27;should register new event handlers after reload&#x27;, async () =&gt; { |
| [811](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L811) | it(&#x27;should remove old event handlers after reload&#x27;, async () =&gt; { |
| [856](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L856) | it(&#x27;should return linked dependencies for a loaded file&#x27;, async () =&gt; { |
| [865](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L865) | it(&#x27;should return empty array for unknown file&#x27;, async () =&gt; { |
| [894](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L894) | it(&#x27;should handle apply error gracefully without crashing&#x27;, async () =&gt; { |
| [960](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L960) | it(&#x27;should clear stashed files after successful reload&#x27;, async () =&gt; { |
| [996](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L996) | it(&#x27;should load the nested entry&#x27;, async () =&gt; { |
| [1001](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1001) | it(&#x27;should instantiate a nested entry exactly once when its host reloads too&#x27;, async () =&gt; { |
| [1019](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1019) | it(&#x27;should skip a nested entry whose host has not started unloading&#x27;, async () =&gt; { |
| [1069](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1069) | it(&#x27;should load both plugins&#x27;, async () =&gt; { |
| [1074](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1074) | it(&#x27;should not start the new instance before the old one released&#x27;, async () =&gt; { |
| [1085](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1085) | it(&#x27;should not delay an unrelated plugin behind a slow one&#x27;, async () =&gt; { |
| [1142](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1142) | it(&#x27;should leave the running plugin untouched&#x27;, async () =&gt; { |
| [1161](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1161) | it(&#x27;should recover after fixing the export&#x27;, async () =&gt; { |
| [1207](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1207) | it(&#x27;should rebuild every fiber, each keeping its own config&#x27;, async () =&gt; { |
| [1272](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1272) | it(&#x27;watches a path that &#96;ignored&#96; would skip&#x27;, async () =&gt; { |
| [1281](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1281) | it(&#x27;runs every callback registered for the same path&#x27;, async () =&gt; { |
| [1292](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1292) | it.runIf(process.platform === &#x27;win32&#x27;)(&#x27;matches paths that differ in case on Windows&#x27;, async () =&gt; { |
| [1347](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1347) | it(&#x27;should start and warn once&#x27;, () =&gt; { |
| [1351](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1351) | it(&#x27;should emit hmr/change for a source file instead of reloading it&#x27;, async () =&gt; { |
| [1361](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1361) | it(&#x27;should still reload config files&#x27;, async () =&gt; { |
| [1427](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1427) | it(&#x27;does not rebuild an entry disabled during unload&#x27;, async () =&gt; { |
| [1448](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1448) | it(&#x27;does not rebuild an entry removed during unload&#x27;, async () =&gt; { |
| [1469](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1469) | it(&#x27;waits for a second re-enable init before deciding whether to rebuild&#x27;, async () =&gt; { |
| [1532](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1532) | it(&#x27;rebuilds a child of an active host that reused the plugin&#x27;, async () =&gt; { |
| [1563](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/index.spec.ts#L1563) | it(&#x27;skips a child whose host is replaced during unload&#x27;, async () =&gt; { |

### path.spec.ts

[packages/hmr/tests/path.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/path.spec.ts) — 4 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [39](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/path.spec.ts#L39) | it.runIf(process.platform === &#x27;win32&#x27;)(&#x27;accepts a drive-letter base&#x27;, async () =&gt; { |
| [46](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/path.spec.ts#L46) | it(&#x27;resolves a relative base against the context&#x27;, async () =&gt; { |
| [52](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/path.spec.ts#L52) | it(&#x27;defaults to the context directory&#x27;, async () =&gt; { |
| [63](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/hmr/tests/path.spec.ts#L63) | it(&#x27;does not watch a nested ignored directory&#x27;, async () =&gt; { |

## create

Plan: [S0–S2](create.md). Status: inventoried; execution/disposition is tracked by that plan.

### scaffold.spec.ts

[packages/create/tests/scaffold.spec.ts](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts) — 14 declaration lines.

| Source line | Declaration excerpt |
|---:|---|
| [134](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L134) | it(&#x27;scaffolds the default template to completion&#x27;, async () =&gt; { |
| [143](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L143) | it(&#x27;reports a registry without a scheme&#x27;, async () =&gt; { |
| [149](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L149) | it(&#x27;reports an unknown dist-tag&#x27;, async () =&gt; { |
| [156](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L156) | it(&#x27;reports a failed tarball request with its url&#x27;, async () =&gt; { |
| [163](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L163) | it(&#x27;reports a template without package.json&#x27;, async () =&gt; { |
| [171](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L171) | it(&#x27;reports invalid registry json&#x27;, async () =&gt; { |
| [178](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L178) | it(&#x27;reports an invalid tarball&#x27;, async () =&gt; { |
| [186](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L186) | it(&#x27;reports a tarball response without a body&#x27;, async () =&gt; { |
| [193](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L193) | it(&#x27;reports a registry that never sends headers&#x27;, async () =&gt; { |
| [201](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L201) | it(&#x27;reports a registry body timeout as a download failure&#x27;, async () =&gt; { |
| [209](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L209) | it(&#x27;reports a tarball body timeout&#x27;, async () =&gt; { |
| [216](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L216) | it(&#x27;distinguishes a dropped download from an invalid archive&#x27;, async () =&gt; { |
| [228](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L228) | it(&#x27;fails when automatic yarn setup cannot download its release&#x27;, async () =&gt; { |
| [238](https://github.com/cordiverse/cordis/blob/35fdc969e4b6c48c48ed44717c7a018324e4dd88/packages/create/tests/scaffold.spec.ts#L238) | it(&#x27;reports a pinned yarn download timeout without changing the rc&#x27;, async () =&gt; { |
