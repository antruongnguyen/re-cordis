# re-cordis-include

Unpublished, unstable `0.0.0`; Rust 2024, minimum Rust 1.85; MIT.

Bounded JSON/restricted YAML configuration files, ordered patches, provenance, explicit refresh, journals and reconciled persistence over loader.

YAML is canonical value output, without comment/style preservation. Synchronous filesystem I/O can block; flush is not crash durability or a multi-file transaction. Join disposal and inspect `final_error` plus Runtime diagnostics. Runtime filesystem evidence covers macOS only.

Use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. There is no supported crates.io installation
or published docs.rs release. Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-include --no-deps --locked
```

Checkout guides: `docs/en/guides/include.md` and `docs/vi/guides/include.md`.
Read `docs/release-readiness.md` for the distribution proposal and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
