# re-cordis-core

Unpublished, unstable `0.0.0`; Rust 2024, minimum Rust 1.85; MIT.

Local plugin lifecycle with typed services, scoped effects, events and diagnostics. Public entry points include `Runtime`, `Context`, `Plugin`, `ServiceKey` and the typed `hooks` module.

Drive local, non-Send handles through `Runtime::run` from Tokio. Keep the host alive through joined cleanup and inspect operation errors, cleanup reports and Runtime diagnostics. This is an explicit Rust API, without transparent JavaScript Proxy/decorator compatibility.

Use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. There is no supported crates.io installation
or published docs.rs release. Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-core --no-deps --locked
```

Checkout guides: `docs/en/getting-started.md` and `docs/vi/getting-started.md`.
Read `docs/release-readiness.md` for the distribution proposal and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
