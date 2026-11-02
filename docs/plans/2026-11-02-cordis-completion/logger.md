# Console logger implementation plan

**Goal:** Deliver `re-cordis-logger` as the console/export satellite over core logging.
**Status:** C0–C3 complete with independent approval and final stable/1.85.0
acceptance. No release/publication is selected.
**Baseline:** Rust `a59edfc` on `feat/cordis-completion`; TypeScript
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`, clean pinned checkout.
**Architecture:** An owned writer and a pure renderer use core's existing
exporter registration. No core/timer/loader/B0 evidence edits are needed.
**Scope:** [Master design](README.md), [shared acceptance](acceptance.md),
[decisions](../../decision.md#console-logger-satellite) and
[limits](../../limitation.md#console-logger-satellite).

## C0 — Selected contract

- [x] `register(&Context, W, ConsoleOptions, ExporterOptions)` and
  `register_stdout(&Context, ConsoleOptions, ExporterOptions)` return
  `Result<ExporterRegistration, AdmissionError>`; `W: Write + 'static`,
  with no Send bound. Use core manual removal, inert token Drop, owner cleanup,
  host admission, filtering, error trees, buffer and snapshot lifetime.
- [x] Node-compatible stdout routing for all severities. Synchronous `write_all`
  and one `flush` per selected record; slow output blocks the local call.
  Short/interrupted writes follow std; partial bytes cannot be rolled back.
- [x] `ColorMode` Plain/Ansi16/Ansi256/Auto: explicit modes win. Stdout Auto
  captures terminal/environment once; FORCE_COLOR 0/1/2/3 (other present values
  ANSI16) > nonempty NO_COLOR > nonterminal/TERM=dumb plain > TERM containing
  256color ANSI256 > terminal ANSI16. Ignore non-Unicode facts. Truecolor caps
  at ANSI256. Custom writers/pure render use plain Auto; `resolve` is pure.
  Plain preserves user-supplied ANSI. ANSI16 gray uses SGR90; ANSI256 uses
  upstream SGR38;5;8 and bold channel labels.
- [x] Default record-time UTC RFC3339 milliseconds; Timestamp::Off optional.
  Pin `time = 0.3.36` with std/formatting only; no handwritten calendar.
  UTC years 0–9999; invalid ranges/layout return InvalidInput. Diff defaults off;
  first/backward +0ms, integer milliseconds always, state commits only after
  write+flush succeeds. Successful backward records become the next baseline.
- [x] Default max_length 10240 UTF-16 units per line; width 0/margin 1/left.
  Core format preserves scalars, normalizes CRLF and appends ellipsis beyond
  limit. Pad label by UTF-16 units, preserve longer labels, retain upstream
  asymmetric multiline indentation and final newline.
- [x] Reentrant writer export returns a visible I/O error from try_borrow_mut.
  Core attempts independent exporters; adapter creates no recursive diagnostics.

C0 was accepted for implementation by the parent after a concise API/default
handoff. Routine choices follow the user's 2026-11-03 useful-Rust direction.
The exact signatures/defaults are documented in
[English](../../en/api/console-logger.md) and [Vietnamese](../../vi/api/console-logger.md).

## C1 — Deterministic renderer

- [x] Create unpublished Rust 2024/MSRV 1.85/MIT/version 0.0.0 crate, workspace
  member and logger-only calendar dependency.
- [x] Pure `render(&LogRecord, &ConsoleOptions, Option<SystemTime>)`.
  9 fixed byte/policy tests cover all levels, empty values, ANSI palettes,
  Unicode/UTF-16/CRLF/limits, UTC/leap day/pre-epoch, deltas, labels, layout
  failures and explicit Display/Debug representations.
- [x] Preserve the pinned right-label oracle exactly:
  `      test  [I]  message\n                 message +0ms\n`.
  Timestamp is disabled here; continuation is exactly 17 spaces.

## C2 — Scoped owned writer

- [x] Use core admission and registration only. One mutable writer/delta state;
  no extra filter/buffer/service/global logger/background resource.
- [x] 11 integration tests cover two exporters, default/name level precedence,
  default buffer independence, short/interrupted/write-zero/partial/flush
  failures with original I/O sources and attempt-all delivery.
- [x] Manual removal, inert token Drop, owner restart/disposal, repeated disposal,
  missing/wrong host/inactive owner/runtime-drop refusal and capture release.
  Destructor logging and recursive Write verify supported release/reentry paths.
- [x] In-flight removal retains the already-selected writer callback, releases its
  capture after it finishes, and omits future records.
- [x] 100 deterministic create/use/dispose cycles restore effect/Registry
  baselines, release every weak capture and stop future output. The adapter
  registers no listener/publication or task; those resources remain unchanged.
- [x] 1 unit regression runs the production writer path with fixed records:
  first +0ms, failed write/flush does not advance state, next successful record
  uses the last success, and a successful backward timestamp commits.

## C3 — Application and documentation

- [x] Runnable plugin setup/logging/owner disposal/manual removal/root drain
  example produces exactly three stdout lines with explicit plain/no-time policy.
- [x] Paired guide/API, public export indexes, glossary and AI-agent reading
  paths; documentation checker covers logger exports and snippet dependencies.
- [x] Classify all five source declarations without reopening carried core/B0
  evidence: aggregate error/source identity, object Display/Debug representation,
  custom formatter Display representation, exporter levels, exact label style.
  Browser dispatch/original arguments and JS inspection/placeholders are unavailable
  in this native record writer; fixed UTC/delta policy is a deliberate adaptation.
- [x] Complete independent review, both explicit toolchain matrices, unchanged
  six-scenario/73-observation parity, both-language executable docs and commit.

Final integration and reviewer evidence is recorded in
[logger acceptance](evidence/logger-acceptance.md).

## Targeted verification

Executed while implementing (all passed after final edits):

```sh
cargo test -p re-cordis-logger --locked
cargo clippy -p re-cordis-logger --all-targets --locked -- -D warnings
cargo fmt --all
cargo run -p re-cordis-logger --example console --locked
```

21 logger tests = 1 writer unit + 9 renderer + 11 exporter integration tests.
The existing 8 core logger tests are carried buffer/error/palette evidence,
not reclassified as newly implemented console behavior. Final review, explicit
stable/1.85.0 workspace/M0/docs checks and commit are owned by the parent task;
targeted results do not by themselves mark integrated acceptance complete.
