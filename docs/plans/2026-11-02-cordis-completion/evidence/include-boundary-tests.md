# Include boundary acceptance tests (2026-11-03)

Independent acceptance additions against baseline 61b7f97. Production ownership
remains with the include implementation worker; accepted loader/core contracts
are unchanged. These files use only public Include/core APIs and existing
temporary-directory helpers.

## Frozen tests

- `crates/re-cordis-include/tests/read_acceptance.rs`: six tests.
- `crates/re-cordis-include/tests/lifecycle_acceptance.rs`: five tests.

Read coverage: missing root without initial, malformed/non-array/unknown document
shape retention, scalar/unknown-factory failures with valid siblings, removal and
recovery of an invalid existing ID, actual factory rename and retirement,
canonical native absolute/relative paths and Unix symlink target directory,
Unicode scalar source positions, canonical YAML-compatible JSON value roundtrip,
unsupported tags/anchors/aliases/duplicates/documents/nonfinite numbers, unknown
extension, exact byte/depth/node/UTF-8 string/key/file/entry/patch limits.

Lifecycle coverage: absent/wrong host and inactive owner admission before I/O;
decoder reentry refusal; file replacement during real apply followed by refresh;
ten nested source remove/restore loops; gated slow teardown with a dropped
disposal waiter and weak capture release; 100 create/use/dispose cycles proving
Registry, root effects, service visibility, event callback and weak capture
baselines return after each joined disposal. No wall-clock sleeps, chmod oracle,
private access or test-only production seam is used.

## Executed evidence

Tools were resolved with explicit `rustup which --toolchain stable/1.85.0`
for Cargo, RUSTC and RUSTDOC. PATH is the selected toolchain bin followed only
by /usr/bin:/bin:/usr/sbin:/sbin. Commands/environment records and full logs are
under ignored `target/include-20261103/`.

- `boundary-stable-attempt2.log`: all five lifecycle tests and five of six
  initial read tests passed; scalar-entry source mapping failed.
- `boundary-msrv-attempt1.log`: the same 10/11 result on Rust 1.85.0.
- `boundary-scalar-location-reproducer.log`: targeted stable reproduction:
  null source entry gets LoaderError path [Index(0), Key("")], but exposed
  IncludeError.location is None.
- `boundary-yaml-nonfinite-reproducer.log`: targeted stable reproduction after
  completing YAML core sign/case fixtures: `a: -.Inf` returns String("-.Inf")
  instead of the contracted unsupported-nonfinite rejection.
- Explicit stable rustfmt --edition 2024 --check on both frozen files: exit 0.

Both regressions were sent to the parent and production owner with concrete
reproducers. After the production owner's corrections, boundary-stable-frozen3.log
and boundary-msrv-frozen2.log each record all eleven tests passing (exit 0).
The graph-limit fixture restores its valid final source before disposal so it
does not overlap the persistence worker's final-I/O-error tests.
The parent owns full I4 acceptance and the complete stable/MSRV matrices.

Frozen SHA-256:
`read_acceptance.rs`
`337895ab8093bcb1913db2ab37367ce86bedb46547fac174bdab6b37448d1da6`

`lifecycle_acceptance.rs`
`04cbdb931e459fb2152beddb0e3e1645bcc4c8cbd43ffaecbefc3d5706c041a8`

Integrated I3 verification replaces the test's eager format!/expect diagnostic
with an equivalent lazy panic closure to satisfy strict Clippy. The complete
48-test package and documentation checks pass on both explicit toolchains after
that change. Final integrated read_acceptance.rs SHA-256:
`fb1e574e19fc245ddf133d99651889fd1b1c83dae81c458190bd27404c00b43a`.
The lifecycle_acceptance.rs fingerprint remains unchanged.
