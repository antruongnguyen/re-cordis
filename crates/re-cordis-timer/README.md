# re-cordis-timer

Planned release references: [tag-pinned guide](https://github.com/antruongnguyen/re-cordis/blob/v0.1.0-alpha.1/docs/en/guides/timers.md)
and [exact-version API](https://docs.rs/re-cordis-timer/0.1.0-alpha.1/re_cordis_timer/).
Candidate preparation does not establish these destinations' availability.

Experimental `0.1.0-alpha.1`; Rust 2024, minimum Rust 1.85; MIT.
Use exact version pins. Intentional breaking changes require release notes and
a new coordinated alpha version.

Owner-scoped timeout, sleep, interval/stream, throttle and debounce through the sealed `TimerExt` trait on `Context`. No timer service installation is required.

Enable Tokio's time driver and drive the original local Cordis host. Timer wrapper Drop requests cancellation; awaited owner cleanup joins it. `Effect` and `TaskControl` Drop remain inert. Interval streams have one consumer and one coalesced pending tick.

For checkout consumption, use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. Keep all Cordis crates at one reviewed revision.
Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-timer --no-deps --locked
```

Checkout guides: `docs/en/guides/timers.md` and `docs/vi/guides/timers.md`.
Read `docs/release-readiness.md` for the distribution contract and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
