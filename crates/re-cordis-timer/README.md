# re-cordis-timer

Unpublished, unstable `0.0.0`; Rust 2024, minimum Rust 1.85; MIT.

Owner-scoped timeout, sleep, interval/stream, throttle and debounce through the sealed `TimerExt` trait on `Context`. No timer service installation is required.

Enable Tokio's time driver and drive the original local Cordis host. Timer wrapper Drop requests cancellation; awaited owner cleanup joins it. `Effect` and `TaskControl` Drop remain inert. Interval streams have one consumer and one coalesced pending tick.

Use a complete [re-cordis checkout](https://github.com/antruongnguyen/re-cordis)
with local Cargo path dependencies. There is no supported crates.io installation
or published docs.rs release. Offline builds require cached third-party dependencies.
From the repository root, generate exact signatures with:

```sh
cargo doc -p re-cordis-timer --no-deps --locked
```

Checkout guides: `docs/en/guides/timers.md` and `docs/vi/guides/timers.md`.
Read `docs/release-readiness.md` for the distribution proposal and evidence
boundaries. Those repository guides are not bundled in this crate archive.

See [LICENSE](LICENSE) for the MIT notice.
