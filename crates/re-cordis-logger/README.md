# re-cordis-logger

Unpublished, unstable `0.0.0`; Rust 2024, minimum Rust 1.85; MIT.

Scoped synchronous console exporters over core logging: `register`, `register_stdout`, `render` and `ConsoleOptions`. Core retains its separate `Logger` API.

An owned local `Write` receives each record followed by flush; failures are visible and slow I/O blocks the caller. Output uses explicit color policy and UTC record timestamps. Rust formatting replaces JavaScript object inspection. Terminal/runtime evidence covers the tested macOS host.

Use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. There is no supported crates.io installation
or published docs.rs release. Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-logger --no-deps --locked
```

Checkout guides: `docs/en/guides/console-logging.md` and `docs/vi/guides/console-logging.md`.
Read `docs/release-readiness.md` for the distribution proposal and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
