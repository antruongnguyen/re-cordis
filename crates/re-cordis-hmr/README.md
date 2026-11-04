# re-cordis-hmr

Unpublished, unstable `0.0.0`; Rust 2024, minimum Rust 1.85; MIT.

Bounded watching of explicitly registered files and `ReloadPlan` replacement of already-linked typed factories. The opt-in `include` feature refreshes and replaces through the same Include-managed instances; default features are empty.

Stop/Drop requests cancellation; joined watcher disposal waits for admitted work and the polling producer. Keep Include alive until watcher work joins. Rust source changes require an external rebuild/restart; there is no binary hot loading or process supervisor. Runtime filesystem/watcher evidence covers macOS only.

Use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. There is no supported crates.io installation
or published docs.rs release. Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-hmr --no-deps --locked --all-features
```

Checkout guides: `docs/en/guides/hmr.md` and `docs/vi/guides/hmr.md`.
Read `docs/release-readiness.md` for the distribution proposal and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
