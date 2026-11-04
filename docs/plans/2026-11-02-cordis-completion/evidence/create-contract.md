# Create S0: minimal offline contract

Date: 2026-11-04 (Asia/Ho_Chi_Minh). Selected by the user's S0–S2 request.
Baseline: clean `8124fed` on `feat/cordis-completion`; HMR acceptance
`4c8f423`, final HMR executable `55a55d1`. No inherited work was discarded.

## Reused decisions and uncovered delta

Reuse local Tokio/Runtime hosting, typed Plugin/ServiceKey, loader configuration,
logger writer ownership, and explicit joined cleanup. Core M0–M6, timer, logger,
loader/groups, Include and HMR remain accepted prerequisites. B0 and its evidence
gaps are unchanged. The new work is filesystem scaffolding and an actual external
Cargo consumer; it supplies no reason to change prerequisite production code.

The TypeScript semantic source remains
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`:
`packages/create/src/index.ts`, `src/bin.ts`, and `tests/scaffold.spec.ts`.
Its package-name update and user-visible failure purpose carry to Cargo.
Its destructive prompt/force, remote registry/archive and npm/Yarn mechanisms
are not selected. The scaffold fixtures construct archives inline; there is no
bundled template under that package. See [source dispositions](create-source-map.md).

## Selected interface

- Crate and standalone binary: `re-cordis-create`, version 0.0.0,
  publish=false, Rust 2024, rust-version 1.85. No generator dependencies.
- CLI: `re-cordis-create NAME --path DEST --cordis-path CHECKOUT`.
  `--path` defaults to NAME; checkout is required. `--help` is noninteractive.
  Run from the repository with
  `cargo run -p re-cordis-create --offline --locked -- NAME --path DEST --cordis-path CHECKOUT`.
- Library: `create(name: &str, destination: &Path, cordis_path: &Path) -> Result<PathBuf, CreateError>`.
  Success returns the absolute generated directory. Errors distinguish invalid
  input and contextual I/O; owned staging cleanup failure remains observable.
- Exit codes: 0 success/help, 2 argument or validation failure, 1 filesystem I/O.
- Names: 1–64 lowercase ASCII bytes, initial letter, remaining letters/digits,
  hyphens or underscores; reserved Rust/Cargo and Windows device names refused.
  Name and destination are separate: native paths may contain spaces or Unicode.
  Paths embedded in Cargo TOML must be Unicode and properly escaped.
- Validate an explicit checkout root and expected core/logger/loader manifests
  and library sources before staging. Canonical absolute path dependencies remain
  usable while unpublished; no machine path or fictional registry version is baked in.

## Destination policy

The parent must already exist; no ancestor tree is created. Accept an absent
destination or an empty directory; refuse a regular file, symlink, or nonempty
directory. Recheck before commit. Render all fixed files into an exclusively
created sibling staging directory, then use one filesystem rename to commit.
No force option, recursive destination deletion, shell command, or automatic
Git/install/run step is included.

An ordinary failure before rename leaves no partial final project. Remove only
owned staging artifacts; report cleanup failure with its path if cleanup itself
fails. Standard rename refuses a concurrently nonempty directory. Acceptance
does not promise hostile-filesystem confinement against parent swaps, exclusive
ownership of a concurrently empty directory, or power-loss durability.
Replacement of an existing empty directory depends on native rename support;
unsupported replacement returns an I/O error without populating the destination.
Runtime filesystem evidence is macOS only.

## Bundled basic application

Exact generated files:

| File | Purpose |
|---|---|
| `Cargo.toml` | Standalone Cargo workspace, unpublished package, local core/logger/loader dependencies and explicit Tokio runtime feature |
| `src/main.rs` | Current-thread Tokio entry point, local Runtime, visible failures |
| `src/app.rs` | Typed service/plugin, loader configuration, real update and joined shutdown; application tests |
| `README.md` | Commands from the generated directory, dependency and lifecycle constraints |
| `.gitignore` | Local Cargo build output |

The sample must prove a typed service changes on update, cleanup releases owned
resources/captures, and final Registry/effect state returns to baseline.
Application tests exercise failure channels as well as success. Runtime diagnostics
and cleanup reports are checked separately; a successful disposal waiter alone
does not prove failure-free cleanup. Repeat lifecycle use at least 100 times.

The generator is offline and does not install dependencies. Later Cargo commands
need cached dependencies for `--offline`; fresh dependency downloads are separate
user actions. No remote/local arbitrary templates, transport stub, archive/cache,
Include/HMR variants, CI/platform expansion, R0 or R1 are part of this slice.

## Execution and verification plan

1. Scaffold regressions first: invalid/reserved names, argument errors, native
   paths, checkout identity, missing parent, nonempty/repeated destination,
   symlinks, actual write/commit failures, and owned staging cleanup.
2. Implement the fixed renderer and destination commit. Verify exact files,
   manifest parsing through Cargo metadata, path resolution and readable README.
3. Build, test and run generated applications outside the repository workspace
   using explicit rustup stable and 1.85.0. Verify their observable update,
   cleanup/error assertions and repeated cycles; strict Clippy and formatting.
4. Commit the verified S1 milestone with current docs, then obtain fresh
   gpt-6-astra/high independent implementation/application review. Address findings
   in verified commits and re-review precise deltas.
5. Run the existing 34-stage shared acceptance plus Create application checks.
   Resolve all five Rust executables with rustup and matching environment;
   preserve long logs under `target/create-20261104/`.
6. Close paired user guides/API/indexes, source dispositions, progress and
   acceptance evidence; commit S2 on the integration branch without pushing.

File ownership during execution: fresh sol/high implementation owns only
`crates/re-cordis-create/**` and root Cargo manifests; fresh sol/high docs owns
current user/status docs except plans; parent owns checker, plan/progress/evidence
and commits. Review contexts start fresh and write only their assigned report.
