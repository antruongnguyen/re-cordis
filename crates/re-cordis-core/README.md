# re-cordis-core

Planned release references: [tag-pinned guide](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/getting-started.md)
and [exact-version API](https://docs.rs/re-cordis-core/0.1.0-alpha.1/re_cordis_core/).
Candidate preparation does not establish these destinations' availability.

Experimental `0.1.0-alpha.1`; Rust 2024, minimum Rust 1.85; MIT.
Use exact version pins. Intentional breaking changes require release notes and
a new coordinated alpha version.

Local plugin lifecycle with typed services, scoped effects, events and diagnostics. Public entry points include `Runtime`, `Context`, `Plugin`, `ServiceKey` and the typed `hooks` module.

Drive local, non-Send handles through `Runtime::run` from Tokio. Keep the host alive through joined cleanup and inspect operation errors, cleanup reports and Runtime diagnostics. This is an explicit Rust API, without transparent JavaScript Proxy/decorator compatibility.

For checkout consumption, use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. Keep all Cordis crates at one reviewed revision.
Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-core --no-deps --locked
```

Checkout guides: `docs/en/getting-started.md` and `docs/vi/getting-started.md`.
Read `docs/release-readiness.md` for the distribution contract and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
See [UPSTREAM-NOTICES.md](UPSTREAM-NOTICES.md) for the applicable Cordis/cosmokit
notice for adapted logger material.
