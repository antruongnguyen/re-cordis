# re-cordis-loader

Unpublished, unstable `0.0.0`; Rust 2024, minimum Rust 1.85; MIT.

Typed factory catalogs, stable source entries, structural `Group` nodes, bounded JSON transport, explicit reconciliation and joined lifecycle rebuilds.

Loader owns groups; no group facade is needed. Use typed decoders and explicit settlement, inspect per-entry errors and Runtime diagnostics, and keep the local host driven through teardown. This crate performs no filesystem persistence, watching or JavaScript evaluation.

Use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. There is no supported crates.io installation
or published docs.rs release. Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-loader --no-deps --locked
```

Checkout guides: `docs/en/guides/loader.md` and `docs/vi/guides/loader.md`.
Read `docs/release-readiness.md` for the distribution proposal and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
