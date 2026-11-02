# Console logger implementation plan

> For execution: use superpowers:executing-plans, or subagent-driven-development
> when delegation is selected. Track the checkboxes per work package.

**Goal:** Deliver re-cordis-logger as the console/export satellite over core logging.
**Architecture:** An owned writer plus a renderer registered through existing
core exporters. Core retains filtering, buffering, error trees and attribution.
**Tech stack:** Rust 2024, std::io::Write/IsTerminal, re-cordis-core; any calendar
formatting dependency is selected at C0.
**Spec:** [Master design](README.md), docs/todo.md's console scope, pinned
packages/logger-console/src/{index,shared,browser}.ts and tests/index.spec.ts.

## Global constraints

Rust edition 2024; resolver 3; minimum Rust 1.85, tested with 1.85.0. MIT;
version 0.0.0 and publish=false. Crate name re-cordis-logger corresponds to
upstream logger-console; it does not replace the logging subsystem in core.

## Source contract and decisions

Upstream shared.ts renders a severity prefix, colored channel label, optional
timestamp and elapsed delta, label width/margin/alignment, and indented multiline
messages. The Node entry uses util.inspect and supports-color; the browser
entry calls console.error/warn/log with original arguments. Node's shared
export method writes all severities through console.log.

Core already provides Logger, LogRecord, ExporterOptions, exact exporter
removal, Rust-formatted messages, UTF-16 line limits and color helpers. The five
upstream declarations cover errors, objects, custom formatters, levels and
label style. Object inspection and printf/custom placeholders need explicit
Rust dispositions; the adapter cannot recover arguments absent from LogRecord.

## C0 — Freeze the renderer/writer contract

**Files:** this plan, docs/decision.md, docs/limitation.md; read core logger.rs
and tests/logger.rs. Produce the accepted API/design before creating the crate.

- [ ] Specify exact registration, configuration and removal signatures using
  core's exporter registration token; no second filter/buffer API.
- [ ] Choose owned Write + 'static for the initial adapter. State synchronous
  backpressure: a slow writer blocks that local call. Async queues are separate
  scope and must be bounded if later introduced.
- [ ] Preserve Node stdout routing by default. Any stderr split is explicit,
  tested configuration, not an assumed severity convention.
- [ ] Specify plain/ANSI16/ANSI256/auto policy, explicit override precedence,
  terminal detection and environment handling. Never infer truecolor support
  from existing helpers; classify unsupported upstream depth 3 explicitly.
- [ ] Select timestamp zone/format, record-time versus render-time behavior,
  first delta, backward SystemTime handling and update-after-failed-write rule.
  Compare fixed UTC/RFC3339 with upstream local template formatting before
  selecting a dependency; do not hand-roll a calendar formatter.
- [ ] Specify maxLength counting, Unicode label padding, CRLF/multiline
  indentation, trailing newline, flush policy and short/interrupted writes.
- [ ] Define reentrant writer behavior. A user Write implementation may log
  recursively; it must get a visible error or another explicitly chosen bounded
  outcome, never a RefCell panic or unbounded recursive error logging.

**Exit:** approved signatures/defaults and expected byte strings for each
included upstream case; Rust Display/Debug and no browser console are explicit
representations rather than falsely identical output.

## C1 — Deterministic rendering

**Files to create:** crates/re-cordis-logger/Cargo.toml, src/lib.rs, src/render.rs,
tests/render.rs. Modify root Cargo.toml/Cargo.lock only for this accepted crate.

- [ ] Write failing renderer tests using fixed LogRecord timestamps and names:
  every level, empty name/message, multiline, CRLF, astral Unicode, zero/finite
  max length, left/right labels, margin, plain/ANSI modes and elapsed time.
- [ ] Implement pure rendering plus the smallest per-exporter delta state.
  Reuse LogRecord::format and LogRecord::color; do not duplicate error flattening.
- [ ] Compare included TS cases with pinned expected output. Test the upstream
  right-aligned label example explicitly, including continuation indentation.
- [ ] Run `cargo test -p re-cordis-logger --test render --locked`; update API
  docs and commit. No filesystem or terminal is required for renderer tests.

## C2 — Scoped writer/exporter lifecycle

**Files:** src/lib.rs; create src/writer.rs and tests/exporter.rs.
**Consumes:** C1 renderer, core exporter registration and LogError::Exporters.
**Produces:** accepted registration API with owned writer and exact removal.

- [ ] Test two independent exporters, channel/default-level precedence,
  filtered messages, order, failed/partial/interrupted writes and flush failures.
- [ ] Register the adapter on the caller's owner using the existing exporter
  API. Propagate writer errors through the existing callback result. Attempt
  independent exporters even when one fails.
- [ ] Test manual removal, owner restart/disposal, capture release, failed
  admission, logging after removal, and reentrant writer behavior.
- [ ] Test removal during an in-flight exporter snapshot against core's actual
  contract; distinguish future records from an already-selected callback.
- [ ] Run exporter and existing core logger tests, document exact semantics and
  commit. No new core logging architecture is part of this task.

## C3 — Console integration and delivery

**Files:** create examples/console.rs; docs/en/api/console-logger.md and
docs/vi/api/console-logger.md; paired guides/console-logging.md; indexes and
tests/docs/check.py coverage.

- [ ] Run a local example through plugin setup, logging, removal and root drain.
  Capture stdout deterministically; use explicit color in tests, not host TTY state.
- [ ] Cover all five source declarations with tests or reviewed representation
  entries. Name every default that differs from Node/browser behavior.
- [ ] Execute shared acceptance and both language examples; verify core
  buffer/error identity behavior remains unchanged; commit.

**Completion:** a console adapter with scoped cleanup and visible write errors;
no duplicate Logger service, hidden runtime, global logger installation or
unbounded background buffer.
