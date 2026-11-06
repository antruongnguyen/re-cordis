# re-cordis-include

Planned release references: [tag-pinned guide](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/guides/include.md)
and [exact-version API](https://docs.rs/re-cordis-include/0.1.0-alpha.1/re_cordis_include/).
Candidate preparation does not establish these destinations' availability.

Experimental `0.1.0-alpha.1`; Rust 2024, minimum Rust 1.85; MIT.
Use exact version pins. Intentional breaking changes require release notes and
a new coordinated alpha version.

Bounded JSON/restricted YAML configuration files, ordered patches, provenance, explicit refresh, journals and reconciled persistence over loader.

YAML is canonical value output, without comment/style preservation. Synchronous filesystem I/O can block; flush is not crash durability or a multi-file transaction. Join disposal and inspect `final_error` plus Runtime diagnostics. Runtime filesystem evidence covers macOS only.

For checkout consumption, use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. Keep all Cordis crates at one reviewed revision.
Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-include --no-deps --locked
```

Checkout guides: `docs/en/guides/include.md` and `docs/vi/guides/include.md`.
Read `docs/release-readiness.md` for the distribution contract and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
