# re-cordis-logger

Planned release references: [tag-pinned guide](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/guides/console-logging.md)
and [exact-version API](https://docs.rs/re-cordis-logger/0.1.0-alpha.1/re_cordis_logger/).
Candidate preparation does not establish these destinations' availability.

Experimental `0.1.0-alpha.1`; Rust 2024, minimum Rust 1.85; MIT.
Use exact version pins. Intentional breaking changes require release notes and
a new coordinated alpha version.

Scoped synchronous console exporters over core logging: `register`, `register_stdout`, `render` and `ConsoleOptions`. Core retains its separate `Logger` API.

An owned local `Write` receives each record followed by flush; failures are visible and slow I/O blocks the caller. Output uses explicit color policy and UTC record timestamps. Rust formatting replaces JavaScript object inspection. Terminal/runtime evidence covers the tested macOS host.

For checkout consumption, use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. Keep all Cordis crates at one reviewed revision.
Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-logger --no-deps --locked
```

Checkout guides: `docs/en/guides/console-logging.md` and `docs/vi/guides/console-logging.md`.
Read `docs/release-readiness.md` for the distribution contract and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
