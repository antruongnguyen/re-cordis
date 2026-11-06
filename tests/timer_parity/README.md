# Executed timer parity fixture

Four small scenarios compare 16 explicit observations from the real, pinned
TypeScript timer service and the Rust timer API:

| Scenario | Observations |
| --- | --- |
| Timeout | Pending before 10 ms, fires once at 10 ms, remains once at 20 ms |
| Cancellation | Explicit cancellation prevents the pending timeout |
| Sleep | Pending at 5 ms, resolved at 10 ms, then successfully awaited |
| Interval | No initial tick, ticks at 10/20 ms, explicit stop prevents later ticks |

Both sides assert the checked-in `.txt` expectations and write their actual
traces; the Python runner compares those files byte for byte. TypeScript uses
Vitest fake timers starting at zero. Rust uses Tokio's paused current-thread
clock within the core `Runtime` and polls registrations before advancing.
This is ordinary positive-delay behavior. Admission-time anchoring, owner
disposal, invalid/zero durations, missed ticks, bounded stream buffering,
multiple readers, throttle/debounce arguments and wall-clock deviation are
outside these parity claims. Rust-specific repair evidence belongs in unit
tests. Rust `sleep` corresponds to the TypeScript `ctx.timeout(delay)` promise.

From the repository root, with the existing local upstream tooling installed:

```sh
python3 -B tests/timer_parity/run.py
python3 -B tests/timer_parity/run.py --toolchain 1.85.0
```

An isolated worktree can use the existing checkout without copying dependencies:

```sh
python3 -B tests/timer_parity/run.py \
  --upstream /absolute/path/to/cordis-rust/local/cordis \
  --output-dir target/timer-parity-evidence
```

`--output-dir` must be new; it retains `rust/*.txt` and
`typescript/*.txt` for review. Without it, temporary output is removed.
The runner selects Rust executables using `rustup which --toolchain`, uses
`cargo --locked`, reports tool versions, and runs upstream ESLint,
TypeScript checking and Vitest through its existing Corepack/Yarn toolchain.
The temporary output-directory TypeScript config inherits upstream source aliases
and checks only this fixture and its imports; no upstream config is changed.
It performs no dependency installation or network restore.

The upstream must have HEAD
`35fdc969e4b6c48c48ed44717c7a018324e4dd88` and no tracked changes.
The runner exclusively creates
`packages/timer/tests/re-cordis-timer-parity.spec.ts`, removes only that
temporary fixture in `finally`, and checks tracked cleanliness again after
successful TypeScript execution. The pin guard and temporary fixture cleanup
are harness properties, separate from the four timer behavior scenarios.
Do not run this fixture concurrently against the same upstream checkout.
