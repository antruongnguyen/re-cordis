# Independent review: first two events declarations

**Spec verdict: accept. Quality verdict: accept. No actionable findings.**
The two declarations, two default variants and five `rust-representation`
obligations have sufficient source, substitute-test and recorded execution
evidence. Zero gaps and zero paired-comparison claims are justified within
this fragment.

Reviewed commit: `1df1de192c0360a938a568fc4e4223880515859d`. The isolated
managed worktree started clean at that exact commit:
`/Users/I756434/.codex/worktrees/b0-events-two-review/cordis-rust`.
Review date: 2026-11-02. The reviewed inputs are
[events-first-two.json](ledger-inputs/events-first-two.json) and
[report-events-first-two.md](ledger-inputs/report-events-first-two.md).
Only this review file is added by the reviewer.

## Scope and carried decisions

Read repository `AGENTS.md`, [B0 Task 2](plan.md#task-2-complete-declaration-and-event-type-disposition),
the [bounded handoff](handoffs/events-first-two.md), its earlier events
handoff, and the original two records in [cases.json](ledger/cases.json).
Applied the master's [reuse rule](../../README.md#prior-decisions-and-evidence-reuse)
and the existing event decisions, limitations, prior implementation-plan
gates and [M2 source map](../../../../m2.md). The accepted matrix/report,
decorator/reflect FR-1 and integrated validation reviews remain accepted.

The only new semantic audit is the assertion provenance for
`core/events.spec.ts:18` and `core/events.spec.ts:35`. Typed event keys,
`ControlFlow`, started dispatch handles and explicit listener controls remain
the accepted Rust representation. No redesign or new exclusion is proposed.

## Source and assertion review

The specification was read from the integration checkout's `local/cordis`
at clean tracked HEAD `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
Below, TS test lines refer to `local/cordis/packages/core/tests/events.spec.ts`,
TS implementation lines to `local/cordis/packages/core/src/events.ts`, Rust
test lines to `crates/re-cordis-core/tests/events.rs`, and Rust implementation
lines to `crates/re-cordis-core/src/events.rs`. CodeGraph against integration
was used first; omitted exact source ranges were then read directly. The
prior Rust port was not used as evidence.

TS `setup` at lines 10–14 creates a Context and replaces its warning method
with a mock. Neither reviewed body calls the imported event/filter/session
helpers or `createArray`. Lines 18–33 use a synchronous identity callback
registered on one Symbol; lines 35–44 use one Symbol once-listener and two
emits. These bodies contain exactly the five recorded assertions.

| Obligation | Evidence checked and disposition |
|---|---|
| `:18/default/a25` | TS line 25 checks bail argument/result `2`. Rust test lines 95–118 register a typed symbol callback returning `Break(v)` and assert `Break(2)` through a clone of the same key. Accept the direct typed substitute. |
| `:18/default/a26` | TS line 26 checks awaited serial argument/result `3`. Rust test lines 127–132 await the same symbol listener and assert `Break(5)`. The changed sentinel preserves the exact argument-to-answer relation. |
| `:18/default/a28` | TS lines 24–28 check the exact four-mode argument sequence. Rust test lines 32–70 assert `[1, 0, 2, 3]` for emit/bail/serial/parallel on ordinary named keys; lines 95–134 separately establish symbol non-aliasing, selection across all four modes and four total invocations. Accept the explicitly disclosed combination with the shared implementation checked below. |
| `:18/default/a32` | TS lines 30–32 manually dispose the symbol registration and check no fifth callback. Rust test lines 78–83 check exact manual removal and unchanged history on the named branch; lines 135–140 check owner removal and unchanged symbol count. Accept the disclosed combination with the shared removal implementation. |
| `:35/default/a43` | TS lines 39–43 register once and check one callback after two emits. Rust test lines 152–178 use a symbol, recursively emit during the first callback, emit again at top level, then assert count one, manual removal false and zero remaining drain attempts. This covers repeated emit and adds a recursive boundary. |

The first four obligations name
`symbols_named_contracts_prototype_names_and_empty_bucket_removal`; the fifth
names `once_is_removed_before_recursive_invocation_and_manual_remove_is_exact`.
The factored assertions are sufficient for this source mapping, but the symbol
branch alone is not an exact argument-history or manual-removal assertion.
The fragment and report preserve that distinction.

The implementation supports the combination: Rust key equality/hashing at
lines 25–42 distinguish named values from opaque symbol identity; cloning at
lines 89–95 retains that identity. Registration at lines 575–635 and selection
at lines 637–700 use the same typed bucket path. The dispatch observer's only
key-specific bypass at lines 703–728 concerns `internal/` names, not the
ordinary named keys used here. Emit at lines 733–785, bail at 788–811, serial
at 814–850, parallel at 853–893 and `invoke_response` at 1014–1033 forward the
same typed arguments without a separate symbol dispatch branch.

Manual removal at Rust lines 284–295 and owner cleanup at 601–610 both call
the exact key/id removal function at 352–371. Once registration sets the
flag at 537–543; `claim_once` at 1005–1012 removes the hook and cleanup before
the synchronous emit callback at 746–750. Consequently, the recursive test's
dropped waiter does not defer or skip that callback attempt. This agrees with
TS `_resolve` at 76–87, dispatch at 95–123, registration/unregistration at
142–175, and once's dispose-before-callback wrapper at 177–183.

All five `rust-representation` classifications reference existing M2
[registration](../../../../m2.md#event-contracts-registration-and-selection)
or [dispatch](../../../../m2.md#five-dispatch-modes-with-honest-callback-shapes)
contracts. The explicit Rust replacements are tested here; no JavaScript
mechanism compatibility or additional paired scenario is inferred.

## Execution joins and validation performed

Read [matrix results](matrix/results.json), the two relevant entries in
[commands.json](matrix/commands.json), and each original events-suite log
section. Both exact Rust names join to `re-cordis-core`, `tests/events.rs`,
`outcome: ok`, `compile_fail: false` on both toolchains: four result rows for
two distinct tests. Both workspace commands have recorded exit code 0.

| Preserved command | Log evidence |
|---|---|
| `stable-workspace-test` | [Log](matrix/logs/stable-workspace-test.log): events suite at line 110; once at 117; symbol test at 123. Suite summary: 12 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out. |
| `1.85.0-workspace-test` | [Log](matrix/logs/1.85.0-workspace-test.log): events suite at line 110; once at 114; symbol test at 122. Same suite summary. |

Executed read-only Git and Python checks, without running the test commands:

- `git rev-parse HEAD`, `git status --short`, `git show --stat --oneline`
  and `git diff-tree --no-commit-id --name-only -r 1df1de1` checked the review
  base, clean start and exactly the two new input files. Upstream
  `git rev-parse HEAD` and `git status --porcelain --untracked-files=no`
  checked the pinned revision and tracked cleanliness.
- Replayed the report's embedded standard-library Python validator in memory.
  Its staging-path query was replaced with the exact reviewed commit's
  `git diff-tree` query because the inputs are already committed; all other
  checks were retained. Exit 0: two unique declarations, two variants, all
  five original obligation IDs/assertion lines, identity fields, evidence
  fields, cited TS/Rust tokens, M2 anchors/report links and four passing
  matrix/log joins checked. No pending, paired or gap claims remained.
- The same Python invocation additionally checked command metadata and
  bounded each test-name lookup to its original `tests/events.rs` log section.
  Both recorded suite summaries matched the table above.
- Python byte comparisons using `git show REV:path` confirmed both inspected
  Rust files equal integration and each of `808c9f5` (implementation),
  `578c066` / `e23dbac` (matrix endpoints), `8c9a191` (mapping base) and
  `1df1de1` (review base). All comparisons passed.
- `git diff --check` passed. Final staged ownership and whitespace checks
  apply only to this review file.

The first attempt to launch the Python check failed in zsh before execution
with `unmatched '` because an embedded heredoc marker was quoted incorrectly.
The corrected invocation used a distinct outer marker and exited 0 with all
checks above passing; no input or source files were changed by either attempt.

These are fresh reference/consistency checks of preserved execution evidence.
No Cargo, Rustdoc, docs-example, TypeScript or parity rerun was performed.
Acceptance is limited to these two declarations and five representation
obligations; it does not complete the remaining seventeen events declarations,
the event-type inventory or B0 integration, and does not expand the accepted
six-scenario paired boundary. Integration remains with the parent.
