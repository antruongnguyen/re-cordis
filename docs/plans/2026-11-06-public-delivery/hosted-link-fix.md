# Hosted dependency link repair — 2026-11-06

## Scope and root cause

Base: `c373a814a366751d5e48978522b58dd825ab6ca4`.
The isolated native Codex worktree is
`/Users/I756434/.codex/worktrees/docs-hosted-links/cordis-rust`.
The integration checkout was read only; no push, merge or deployment occurred.

Documentation [push run 42196744104](https://github.com/antruongnguyen/re-cordis/actions/runs/42196744104)
and [PR run 42196762710](https://github.com/antruongnguyen/re-cordis/actions/runs/42196762710)
passed macOS stable/1.85.0 guide/API jobs and site build/typecheck/lint/unit
tests, then failed the exact dependency inventory assertion: `96 !== 97`.

The real `rustdoc-for-website` artifact was downloaded from the push run.
Its stable guide job `127821840376` records
`rustdoc 1.99.1 (58cf8f9c3 2026-10-13)` on aarch64 Apple Darwin.
The accepted local artifact uses Rustdoc 1.97.1, also visible in its
versioned standard-library hyperlinks. Both contain Syn 3.0.9 and 3443
compiler-emitted files.

Exactly one inventory entry differs:

| File | Target | Local 1.97.1 | Hosted 1.99.1 |
| --- | --- | --- | --- |
| `syn/parse/struct.ParseBuffer.html` | `trait.Peek.html#associatedtype.Token` | 2 links | 0 links |

The two uses occur in `parse_terminated`'s return type and where clause
(source `src/syn/parse.rs.html`, lines 735–746).
`Peek::Token` is explicitly `#[doc(hidden)]` in
`src/syn/lookahead.rs.html`, lines 176–180. Neither compiler's
`syn/parse/trait.Peek.html` contains an `associatedtype.Token` anchor.
Rustdoc 1.97.1 incorrectly links both uses; 1.99.1 renders the same two
`P::Token` uses as plain text. This is a compiler rendering improvement,
not a dependency, source, platform or artifact-transfer change.

The visible source text, obtained by removing HTML tags and decoding entities,
has identical SHA-256 across both compiler artifacts:

- `parse.rs.html`: `185d5b2c6d8f0890ba16e22a17ce8b5403a9e56da424565ff84067b33c85b85d`.
- `lookahead.rs.html`: `fbdd8ceee88b34b5788f41f744da89309fd366f319b783916483fd002e2d3827`.

Raw `ParseBuffer.html` SHA-256 differs, as expected for distinct compilers:

- Local: `46a7ab3888614d19c1f5fe25f1afbe7ccb8cda7b839f99563bda3eed34b3f6dc`.
- Hosted: `1fd224865d798de620c8b78366de194b1f2c3af56e9470a019140b5cf17c8634`.

All other 96 file/target/count entries match exactly.

## Correction and regression

Keep the original 97-entry JSON inventory. Select its precise 96-entry variant
only when the complete `parse_terminated` signature matches the observed
signature, both `P::Token` uses are plain text, and the old target is absent
throughout that dependency page. Do not accept arbitrary subsets or optional
counts. The existing strict set/count assertions remain unchanged.

The regression first failed on the hosted plain-text rendering
(`expected undefined; received 2`), then passed with the classifier.
It covers both observed signatures and retains the original expectation for
an omitted method, omitted token use, partial link conversion and wrong
signature. Full exported-artifact mutations additionally prove unrelated
missing/count-changed entries still fail at the original assertions.

## Executed verification

Commands run from this worktree; site commands run from `website/`.
Logs are retained under the ignored worktree `target/`.

| Command | Result |
| --- | --- |
| `gh run download 42196744104 --name rustdoc-for-website --dir target/hosted-rustdoc` | Exit 0; actual hosted artifact |
| `gh run view 42196744104 --log --job 127821840376` | Exit 0; compiler provenance |
| `bun install --frozen-lockfile` | Exit 0; unchanged lockfile |
| `CORDIS_RUSTDOC_DIR=../target/hosted-rustdoc bun run build` before repair | Exit 0 |
| `bun run check` before repair | Exit 1; reproduced `96 !== 97` |
| `bun test scripts/rustdoc.test.ts` with baseline classifier | Exit 1; 1 pass/1 expected failure |
| `bun test scripts/rustdoc.test.ts` after repair | Exit 0; 2 pass/0 fail |
| `CORDIS_RUSTDOC_DIR=../target/hosted-rustdoc bun run build` after repair/current docs | Exit 0 |
| `bun run typecheck` | Exit 0 |
| `bun run lint` | Exit 0 |
| `bun run check` on real hosted input | Exit 0; 8 tests; 70 pages; 473541 links/anchors; 15875 assets; 110 references/96 entries, 75 targets/31 files |
| `CORDIS_RUSTDOC_DIR=/Users/I756434/Projects/private/cordis-rust/target/site-rustdoc/doc bun run build` | Exit 0 |
| `bun run check` on real accepted local input | Exit 0; 8 tests; 70 pages; 471159 links/anchors; 15875 assets; 112 references/97 entries, 76 targets/32 files |
| `python3 -B tests/docs/test_check.py` | Exit 0 |
| `git diff --check` | Exit 0 |

For each full-check negative, copy the hosted input into
`target/mutated-rustdoc`, mutate only that copy, then execute these separately
and in order:

```sh
CORDIS_RUSTDOC_DIR=../target/mutated-rustdoc bun scripts/prepare.ts
bun scripts/finalize.ts
bun scripts/check.ts
```

Preparation/finalization exit 0. The checker deliberately exits 1:

1. Replace the single `href="struct.Slab.html#with_capacity"` in
   `slab/index.html` with `href=""`: exact set assertion fails `95 !== 96`.
2. Restore Slab; replace one of two `href="crate::select"` attributes in
   `futures_util/future/trait.TryFutureExt.html` with `href=""`:
   exact reference-count assertion fails `1 !== 2`.

Key logs: `hosted-check-red.log`, `regression-red.log`,
`regression-green.log`, `hosted-build-green.log`, `hosted-check-green.log`,
`omission-check.log`, `count-check.log`, `local-build-green.log`,
`local-check-green.log`, `typecheck.log`, `lint.log`, `docs-test-check.log`.

## Preserved boundaries

Both real-input checks verify the complete source/Rustdoc manifests and exact
compiler-emitted bytes, actual HTML links/anchors/assets, private-document
exclusion, Markdown code, search and public LLM output. Both report zero
framework/site exceptions. The four bounded empty implementor registrations
and generated workspace landing remain unchanged.

No Rust implementation, Cargo manifests, dependencies, canonical EN/VI guides,
toolchains or workflows changed. Retain macOS Rustdoc → Linux site transfer,
major action tags, explicit stable/1.85.0 checks, manual Pages gates and all seven
`0.0.0`/`publish=false` crates. No full Cargo matrix was repeated for this
site-checker correction. No runtime/platform expansion or publication occurs.

The final real-input site checks used the base HEAD with this scoped worktree
change marked dirty, reusing the two existing compiler artifacts because Rust
source was unchanged. This establishes the local correction only.
Fresh hosted CI at the integrated commit and independent scoped review remain
required before D1 acceptance; Pages and registry/docs.rs gates remain separate.
