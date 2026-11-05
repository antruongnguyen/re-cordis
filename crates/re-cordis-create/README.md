# re-cordis-create

Unpublished offline scaffold command for Rust 1.85+:

```sh
cargo run -p re-cordis-create --offline --locked -- my-app --path '../my app' --cordis-path .
```

Run that command from a Cordis checkout. `--cordis-path` is required; `--path`
defaults to the project name. The command writes exactly Cargo.toml, src/main.rs,
src/app.rs, README.md and .gitignore. The template uses core, console logger and
typed loader configuration with update and joined shutdown.

The library exposes `create(name, destination, cordis_path)` and `CreateError`.
Success/help exit 0, validation exits 2, filesystem/owned-staging cleanup errors
exit 1. Names are 1–64 lowercase ASCII characters, start with a letter and exclude
Rust/Cargo/Windows reserved names.

Only absent or empty directory destinations are admitted. Files, symlinks and
nonempty targets are refused twice: before staging and before a single sibling
rename. Only owned staging is removed on failure. Cleanup failure names its path
and retains both original errors. Native OS path validity, empty-directory rename
support, malicious ancestor swaps, concurrent empty-target races and power-loss
durability are separate filesystem limits.

There is one bundled template, no remote transport, no Git/init/install/run
options and no automatic processes. Generated absolute path dependencies refer
to the supplied checkout. Inspect the generated README before testing or running
the application. The fixed layout check is not a general Cargo TOML parser or a
source authenticity audit.

Scaffold tests use the containing repository by default. When testing an
extracted Cargo archive outside that repository, explicitly supply a compatible
complete checkout through the test-only `CORDIS_CREATE_TEST_CHECKOUT` variable:

```sh
CORDIS_CREATE_TEST_CHECKOUT='/absolute/path/to/pinned/cordis-rust' cargo test --manifest-path '/absolute/path/to/extracted/re-cordis-create-0.0.0/Cargo.toml' --locked --offline
```

The fixture was verified with the complete checkout pinned to Git revision
`03d255029db6a57ccc17c0b1e26facdb491c058b` on stable and Rust 1.85.0. Keep all
Cordis crates at that same revision when reproducing this check. The variable
preserves native OS paths and applies only to tests; CLI use still requires
`--cordis-path`. Extract the archive outside any containing source workspace;
Cargo workspace discovery is separate from checkout fixture validation. These
tests remain dependent on an external checkout; the archive does not bundle the
runtime crates. Offline generated-app checks also require cached dependencies.

Version remains `0.0.0` with `publish=false`; there is no supported crates.io
installation or published docs.rs release. The checkout must remain available
at the generated absolute paths; moving it requires manifest edits, and its
source changes affect subsequent builds without a package-version change.
Runtime filesystem evidence covers macOS only. The checkout's
`docs/release-readiness.md` records the release proposal; it is not bundled in
this crate archive. See [LICENSE](LICENSE) for the MIT notice.
