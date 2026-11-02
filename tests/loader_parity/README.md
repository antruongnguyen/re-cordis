# Bounded paired loader observations

Two scenarios compare nine causal observations from the real pinned TypeScript
loader and the Rust loader public API.

| Scenario | Observations |
|---|---|
| Recovery (4) | Missing dependency leaves the plugin Pending with no apply; provider arrival runs and fails apply; explicit source update recovers; removal releases the successful instance. |
| Structural group (5) | Disabled group stays active with registered disabled child; enable/disable/re-enable controls child applies and disposers; owner shutdown releases the final child. |

Both sides assert the literal checked-in `.txt` expectations and write their
actual traces; the runner compares those files byte for byte. Lines are derived
from actual entry/Fiber states, entry membership, applied config values and
registered cleanup callbacks. No unconditional success-event printing is used.
Rust separately asserts settlement failure reporting and an empty registry after
shutdown. TypeScript uses the pinned upstream MockLoader only to substitute a
static module catalog; entry, dependency, group and disposal machinery are real.

These scenarios characterize existing behavior; dependency-bypass apply,
failure recovery loss, ignored group verdict or missing cleanup changes their
observations. This is bounded evidence, not declaration/assertion parity.
It does not cover entry-level conditions or dynamic JS expressions, mapper
authority, injection changes during cleanup, self-disposal, async import/patch
windows, source reconciliation/adoption, commit payloads, isolation/realm moves,
retained providers, automatic HMR or persistence. Rust owns an outer Fiber even
without entry injection, uses typed plugins/config and explicit Group, and
rebuilds across group/scope moves; these mechanisms are not compared here.

From the repository root with existing upstream tooling:

```sh
python3 -B tests/loader_parity/run.py
python3 -B tests/loader_parity/run.py --toolchain 1.85.0
```

An isolated worktree may point at the existing checkout:

```sh
python3 -B tests/loader_parity/run.py \
  --upstream /absolute/path/to/cordis-rust/local/cordis \
  --output-dir target/loader-parity-evidence
```

The default upstream is `local/cordis`; no dependency installation occurs.
`--output-dir` must be new and retains `rust/*.txt` and `typescript/*.txt`.
Otherwise temporary output is removed. The Python stdlib runner selects exact
Rust executables through `rustup which --toolchain`, uses `cargo test --locked`,
reports tool versions, and runs upstream ESLint, strict TypeScript checking and
Vitest through existing Corepack/Yarn. Each subprocess has a timeout (180 seconds
for checks, 30 seconds for metadata); each behavior scenario has a 10-second
timeout in both runtimes.

The upstream must have HEAD
`35fdc969e4b6c48c48ed44717c7a018324e4dd88` and no tracked changes.
The runner exclusively creates
`packages/loader/tests/re-cordis-loader-parity.spec.ts`, refuses to overwrite
an existing fixture, removes only its own temporary file in `finally`, and
checks tracked cleanliness after successful TypeScript execution. Its temporary
TypeScript config inherits upstream aliases without modifying project config.
Do not run this fixture concurrently against the same upstream checkout.
