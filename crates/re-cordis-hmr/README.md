# re-cordis-hmr

Planned release references: [tag-pinned guide](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/guides/hmr.md)
and [exact-version API](https://docs.rs/re-cordis-hmr/0.1.0-alpha.1/re_cordis_hmr/).
Candidate preparation does not establish these destinations' availability.

Experimental `0.1.0-alpha.1`; Rust 2024, minimum Rust 1.85; MIT.
Use exact version pins. Intentional breaking changes require release notes and
a new coordinated alpha version.

Bounded watching of explicitly registered files and `ReloadPlan` replacement of already-linked typed factories. The opt-in `include` feature refreshes and replaces through the same Include-managed instances; default features are empty.

Stop/Drop requests cancellation; joined watcher disposal waits for admitted work and the polling producer. Keep Include alive until watcher work joins. Rust source changes require an external rebuild/restart; there is no binary hot loading or process supervisor. Runtime filesystem/watcher evidence covers macOS only.

For checkout consumption, use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. Keep all Cordis crates at one reviewed revision.
Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-hmr --no-deps --locked --all-features
```

Checkout guides: `docs/en/guides/hmr.md` and `docs/vi/guides/hmr.md`.
Read `docs/release-readiness.md` for the distribution contract and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
