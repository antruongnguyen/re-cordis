# Create S0–S2 independent implementation and application review

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Reviewed base: `8124fed29e959e0ee7e59772c71d99c457e21bf6`.
Reviewed implementation: `d44620c9c28acfa5fd2e4b93c761af08126580f4`
on `feat/cordis-completion`. The working tree was clean when review began.

**Verdict: changes required.** Two P2 generator defects remain in this revision.
The generated application's selected lifecycle behavior and code quality are
approved on the evidence below. Full offline S0–S2 specification/code-quality
approval is withheld until the two generator corrections receive verified
regressions and a scoped independent re-review. This report is not S2 acceptance.

## Findings

### CR1 — P2: refuse package names that collide with bundled path dependencies

Location: `crates/re-cordis-create/src/lib.rs:227–238`
(the reserved-name set is at lines 147–226).

The validator accepts `re-cordis-core`, `re-cordis-logger`, and
`re-cordis-loader`. For each, the CLI returns 0 and commits the complete project.
The generated package and the corresponding path dependency then have the same
name/version `0.0.0` at different paths. Cargo fails before compilation because
these package identities cannot coexist unambiguously in its lockfile.

Reproduced independently for all three names on both stable and 1.85.0:

```sh
re-cordis-create re-cordis-core --path /existing-parent/new-app --cordis-path /path/to/cordis-rust
cargo check --manifest-path /existing-parent/new-app/Cargo.toml --offline
```

The generator exits 0; Cargo exits 101 with
`package collision in the lockfile: packages re-cordis-core v0.0.0 (...)`
and `only one can be written to lockfile unambiguously`.
Substitute either other dependency name for the same failure.

Reject the three fixed-template dependency names before staging and document the
additional name restriction. Add regressions that preserve absent/empty
destinations and demonstrate the actual Cargo collision that motivates the guard.
No generic dependency discovery or general TOML parser is necessary.

Evidence: `target/create-review-20261104/{stable,1.85.0}.log`;
the six `collision-{toolchain}-{name}` fixture directories retain the exact
generated manifests.

### CR2 — P2: staging must not alias the requested final destination

Location: `crates/re-cordis-create/src/lib.rs:370–377`.

A fresh process accepts an absent destination named
`.re-cordis-create-{process_id}-0`. Its first staging candidate is that same
directory. The generator writes template files at the requested final location
before the commit, then rejects its own now-nonempty destination and deletes it
as staging. A valid request therefore fails and the promised separate-staging
boundary is absent.

The public-library reproduction is:

```rust
let destination = base.join(format!(".re-cordis-create-{}-0", std::process::id()));
let result = re_cordis_create::create("valid-app", &destination, checkout);
```

Run in a new process before any other generator call, with an existing empty
`base` and valid checkout. Observed result:
`Err(Validation("destination is not empty: ..."))`, with destination absent
after cleanup.

The uppercase spelling `.RE-CORDIS-CREATE-{process_id}-0` reproduces the same
failure on this case-insensitive macOS volume. A lexical `Path` equality check
alone does not repair the executed case. Allocate a sibling name that cannot
be filesystem-equivalent to the selected destination before creating/writing it;
retain exclusive creation and bounded collision handling. Cover exact and
case-folded aliases as applicable to the host filesystem.

Evidence: `target/create-review-20261104/staging_alias.rs`,
`staging_alias_uppercase.rs`, `staging-alias.log`, and
`staging-alias-uppercase.log`. Both public-API probes were compiled with the
explicit stable rustc and linked to the independently built generator library.
These are ordinary allowed destination inputs, not hostile ancestor replacement.

## Executed review evidence

All Rust tools were resolved through `rustup which --toolchain stable|1.85.0`:
cargo, rustc, rustdoc, cargo-clippy, and clippy-driver. The runner verified a shared
toolchain bin directory and set matching `RUSTUP_TOOLCHAIN`, `RUSTC`,
`RUSTDOC`, leading `PATH`, and separate
`target/create-review-{stable,1.85.0}` target directories. No Homebrew Rust tool
provided this evidence.

| Tool | Stable | MSRV |
|---|---|---|
| rustc/rustdoc | 1.97.1, 8bab26f4f, 2026-07-14 | 1.85.0, 4d91de4e4, 2025-02-17 |
| Cargo | 1.97.1, c980f4866, 2026-06-30 | 1.85.0, d73d2caf9, 2024-12-31 |
| cargo-clippy/clippy-driver | 0.1.97, 8bab26f4f6 | 0.1.85, 4d91de4e48 |

The saved runner is `target/create-review-20261104/run.py`; complete command
arguments, versions, output and exit codes are in its per-toolchain logs.

| Check | Stable | 1.85.0 |
|---|---:|---:|
| `cargo test -p re-cordis-create --locked --offline` | 17 passed: 4 unit + 13 integration | Same |
| Strict generator all-target Clippy and explicit build | Passed | Passed |
| Unmodified generated external app: exact five files and Cargo workspace metadata | Passed | Passed |
| External `cargo build`, `test`, `run`, strict all-target Clippy, format check | Passed; 5 tests | Passed; 5 tests |
| CR1 real Cargo package collision probes | 3 reproduced failures | 3 reproduced failures |
| Supplemental post-setup application error/release probe | Passed | Passed |

External Cargo commands used `--manifest-path` as a process argument,
`--offline`, and `--locked` after initial dependency resolution.
The original generated applications remain unchanged in
`external-{toolchain} ứng dụng`. They contain their own workspace boundary
inside the repository's ignored target tree; Cargo metadata verified that the
generated project is not a member of the enclosing workspace.
Both executions printed the initial greeting, updated greeting,
`service: updated greeting`, and `shutdown complete`.

The five template tests exercise real typed update, weak old/new service release,
owned writer/capture release, Registry/effect baselines, 100 reuse cycles, original
root cleanup causes, nested Runtime diagnostic causes, and partial setup failure.
Review additionally generated separate `probe-app-{toolchain}` fixtures and
added only a test module. A writer failed specifically during update logging and
during final service logging; both paths returned an error and joined writer,
effect and Registry release. The update callback retained a Runtime diagnostic;
a directly returned root logging error correctly did not duplicate itself in
that cleanup channel. The first probe incorrectly required a diagnostic for both
cases; correcting that review-only assertion yielded 6 passing tests per fixture
(the five original tests plus one two-case probe). Logs preserve both the initial
probe mistake and corrected result in `post-setup*.log`. This was not a
production-code defect or production-code modification.

Also executed: `python3 -B tests/docs/test_check.py` (8 passed), the checker
`check_structure()` (33 matched page pairs, links and export coverage including
2 Create exports), and `git diff --check` for the reviewed revision range
(passed). The parent independently owns full executable documentation checks and
the shared matrix; those are not claimed as this reviewer's executed checks.

## Scope, source and documentation assessment

Reviewed the new generator, CLI, five bundled files, generator tests, checker
delta, paired current Create guide/API pages, and relevant decision/limitation/
status documentation. The stdlib-only fixed renderer, one-pass token expansion,
native argument parsing, path escaping, contextual errors, owned cleanup error
retention and refusal of ordinary nonempty destinations are appropriately small
for the selected scope. CR2 is the exception to separate-stage allocation.
No additional correctness or code-quality finding was identified.

Verified the TypeScript checkout at
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`; inspected Create
`src/index.ts`, `src/bin.ts`, and all 14 `scaffold.spec.ts` declarations.
The source map accurately distinguishes one Cargo-equivalent purpose, one
portable-failure/Cargo representation, ten unselected remote-transport
declarations and two Yarn exclusions. None of those exclusions is a passing
Rust/TypeScript test claim. The implementation offers no downloader, arbitrary
template input, auto Git/install/run step, Include/HMR template or publication.

The docs accurately separate literal checkout/layout validation from general
TOML/source validation, and state offline cache needs, canonical path dependency
portability, native empty-directory rename behavior and macOS-only execution.
They need the CR1 name restriction after repair; final acceptance status remains
the integration owner's subsequent step.

Accepted core M0–M6, timer, logger, loader/groups, Include and HMR contracts were
reused without reopening their implementation. HMR executable `55a55d1` and
acceptance `4c8f423` remain carried evidence. No Linux/Windows runtime result,
hostile-filesystem confinement, crash durability, B1, R0 or R1 is claimed.
