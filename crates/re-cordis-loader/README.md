# re-cordis-loader

Planned release references: [tag-pinned guide](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/guides/loader.md)
and [exact-version API](https://docs.rs/re-cordis-loader/0.1.0-alpha.1/re_cordis_loader/).
Candidate preparation does not establish these destinations' availability.

Experimental `0.1.0-alpha.1`; Rust 2024, minimum Rust 1.85; MIT.
Use exact version pins. Intentional breaking changes require release notes and
a new coordinated alpha version.

Typed factory catalogs, stable source entries, structural `Group` nodes, bounded JSON transport, explicit reconciliation and joined lifecycle rebuilds.

Loader owns groups; no group facade is needed. Use typed decoders and explicit settlement, inspect per-entry errors and Runtime diagnostics, and keep the local host driven through teardown. This crate performs no filesystem persistence, watching or JavaScript evaluation.

For checkout consumption, use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. Keep all Cordis crates at one reviewed revision.
Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-loader --no-deps --locked
```

Checkout guides: `docs/en/guides/loader.md` and `docs/vi/guides/loader.md`.
Read `docs/release-readiness.md` for the distribution contract and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
