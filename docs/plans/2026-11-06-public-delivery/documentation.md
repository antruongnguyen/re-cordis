# D2 public documentation delivery

Date: 2026-11-06 (Asia/Ho_Chi_Minh).
Status: **initial deployment and live smoke checks passed; documentation refresh in progress**.

## Published revision and gates

The site is available at [https://antruongnguyen.github.io/re-cordis/](https://antruongnguyen.github.io/re-cordis/).

Initial deployed source: `3718217aac6f8f11f1f11c221bf2730cbe169bd6`.
The [manual Pages run](https://github.com/antruongnguyen/re-cordis/actions/runs/42204868321)
passed both macOS guide/API jobs, the Linux static-site check and deployment.
The returned live `build-provenance.json` identifies that exact revision.

Pages uses GitHub Actions (`build_type=workflow`). Its `github-pages`
environment permits only the `main` branch through a custom branch policy.
The repository variable `PAGES_DEPLOY_ENABLED=true` enables the existing
manual workflow, which still requires `deploy=true`. No automatic deployment
on push/PR was introduced. Hosted Rustdoc is rebuilt by the macOS job.

The [CI trigger follow-up PR #2](https://github.com/antruongnguyen/re-cordis/pull/2)
merged at this revision. Its final PR head `fdf0035` had exactly two successful
runs, both from `pull_request`: [Rust](https://github.com/antruongnguyen/re-cordis/actions/runs/42203738866)
and [Documentation](https://github.com/antruongnguyen/re-cordis/actions/runs/42203738830).
The merged source then passed one [Rust push run](https://github.com/antruongnguyen/re-cordis/actions/runs/42204299211)
and one [Documentation push run](https://github.com/antruongnguyen/re-cordis/actions/runs/42204299299).
Feature-branch push duplicates are removed; jobs, matrices and manual/reusable
entry points remain unchanged.

## Hosted artifact verification

The initial Pages run checked 70 canonical routes/Markdown files,
473541 link/anchor references and 15875 asset references.
It reported 110 known dependency-prose references across 96 exact entries,
75 targets and 31 files, with zero framework/site exceptions.
The [compiler-rendering repair](hosted-link-fix.md) preserves strict inventory
and byte-fidelity checks. The complete hosted check ran before publication.

HTTP 200 was observed for the site root, EN/VI documentation homes, English
API index, public llms.txt, Vietnamese effects-guide Markdown and provenance.
The exact-revision GitHub source link for `docs/en/README.md` returned HTTP 200.
The live public agent index includes both languages and no internal planning
routes or removed reference-project names.

## Live browser smoke checks

Chrome DevTools MCP used an isolated `re-cordis-delivery` context.
Desktop viewport: 1440×1000. Mobile/touch viewport: 390×844.
The flow was documentation entry → search/language/navigation → matching page,
with explicit cleanup/API documentation and recovery checks.

| Check | Observed result |
|---|---|
| Identity and content | Correct titles, populated article and matching EN/VI HTML language |
| English search | “cleanup” returns 10 results; Enter opens the English effects/cleanup guide |
| Vietnamese search | “dọn dẹp” returns 10 Vietnamese results; Enter opens the Vietnamese effects API |
| Language switch and reload | English effects guide switches to the same Vietnamese guide; direct reload preserves it |
| Empty results | Unmatched query displays “Không tìm thấy kết quả” |
| Escape/focus | Open search dialog closes; focus returns to the search trigger |
| Skip link | Tab selects “Đến nội dung”; Enter focuses `ARTICLE#main-content` |
| Mobile sidebar | Opens; choosing Vietnamese HMR API navigates and closes it (`aria-expanded=false`) |
| Layout | No document overflow on the tested desktop guide and mobile home/effects/HMR pages |
| Code copy control | Vietnamese button acknowledges “Đã sao chép mã”; OS clipboard bytes were not read |
| Exact Rustdoc | HMR index resolves; `FileWatcher.html#method.watch_include` exists with the expected optional-Include signature |
| Real 404 | Missing path returns HTTP 404 and bilingual recovery links; each returns to its matching language home |
| Console | No Fumadocs application errors/warnings in the observed flows |

Native Rustdoc emits seven compiler-owned font-preload credential-mode warnings.
The method page renders and the documented API anchor works; generated compiler
HTML is not rewritten to silence these warnings.

Desktop and mobile screenshots were visually inspected inline. The browser tool
denied filesystem screenshot export, so no saved screenshot file is claimed.
A new-tab navigation initially exceeded the tool's ten-second limit; reuse of
the existing tab with a thirty-second navigation bound loaded Rustdoc successfully.
This is a Chrome desktop/mobile smoke check, not all-browser or platform acceptance.

## Documentation refresh

Current availability text and root README links are updated separately from
the already published initial artifact. The [public delivery plan](../../superpowers/plans/2026-11-06-public-delivery.md)
still requires the reviewed status update to reach main, its hosted checks to
pass, and the manual Pages workflow to publish that documentation revision.
Final provenance/HTTP checks and the refresh run belong in the closing record.

The update preserves seven `0.0.0`/`publish=false` packages, local-checkout
consumption, all notices, Rust contracts and the existing platform limits.
Crates.io/docs.rs publication and the alpha decision remain separate.
No registry version or release tag was created.

## Local evidence

Ignored records are under `target/public-delivery-20261106/`:
`pages-settings.json`, `run-42204868321.json`,
`pages-initial-site-check.log`, `deployed-provenance-initial.json`,
`live-http-initial.json`, `live-source-link.json` and
`live-browser-initial.json`. The status-update guide logs are retained in
`public-docs-status/`. These paths are local evidence, not published assets.
