# Public Documentation and Experimental Alpha Delivery Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to execute this plan task-by-task. The user requested execution on 2026-11-06. D1 and D2 are complete; preserve the subsequent release decision gates.

**Goal:** Deliver the accepted framework through verified public documentation, then a separately approved seven-package experimental alpha.

**Architecture:** Reuse the existing Rust and Documentation workflows, manual Pages deployment, seven crates and offline Create template. Integrate the accepted branch before preparing a separate release change. Preserve existing runtime contracts; prove registry consumption separately from local workspace success.

**Tech Stack:** Rust 2024, stable and Rust 1.85.0, Cargo, GitHub Actions/Pages, Bun 1.4.4, Node.js 22, Fumadocs/Next.js, crates.io and docs.rs.

**Spec:** [Accepted release proposal](../../release-readiness.md), [documentation delivery](../../plans/2026-11-05-documentation-site/delivery.md), [deployment contract](../../../website/README.md#verification-and-deployment).

## Global constraints

- User choice on 2026-11-06: public docs first, then experimental alpha. This plan does not itself authorize push, merge, deployment, version changes or publication.
- Keep all seven packages at `0.0.0`/`publish=false` until the explicit release decision in D3.
- Proposed alpha: `0.1.0-alpha.1`, coordinated across the seven crates; exact internal requirements `=0.1.0-alpha.1`.
- Preserve Rust 2024, MSRV 1.85 verified with 1.85.0, empty HMR default features and its optional `include` adapter.
- Initial runtime support statement remains the tested Apple Silicon macOS environment. Ubuntu CI and Linux documentation builds do not establish full Linux/Windows filesystem or watcher acceptance.
- Preserve local/non-Send hosting, typed APIs, explicit joined teardown, bounded I/O and the offline Create checkout requirement.
- No facade, extra satellite/template, registry-backed Create mode, binary hot loading or broad platform expansion in this delivery.
- Preserve notices and the five-file generated application. Keep `docs/plans` and `docs/superpowers` out of published pages, search, Markdown/LLM output, crate READMEs and Rustdoc.
- Reuse accepted evidence where executable inputs are unchanged. The 147 B0 comparison gaps are not a required feature backlog.
- Serialize Cargo/site operations. Commit each verified milestone on its working branch; no Co-authored-by trailer. Push only within explicit execution authorization.

## Baseline and evidence

Planning baseline: `3f70922`, clean `feat/cordis-completion`, remote repository `antruongnguyen/re-cordis`.
At inspection, the branch is five commits ahead of its remote (`94907cb`) and 122 commits ahead of `main` (`808c9f5`), with no divergence or open PR. Recheck after fetching at D1; these are observations, not permanent assumptions.

The [35 accepted work packages](../../plans/2026-11-02-cordis-completion/progress.md#work-packages) remain closed.
Fresh local audit at the baseline: all-feature workspace tests pass on stable 1.97.1 and 1.85.0, 517 checks per toolchain (492 unit/integration and 25 Rustdoc), plus nine documentation regressions covering 35 language pairs. Logs: `target/progress-audit-20261106/tests-{stable,1.85.0}.log`.
This was not a repeat of the full release/parity/site matrix. The latest successful hosted Rust run was at `94907cb`; current documentation workflows have no established hosted acceptance.

## Sequence and decision points

| Stage | Depends on | Deliverable and exit criterion |
|---|---|---|
| D1 — Integrate accepted delivery | Authorization to push, open PR and merge | Reviewed branch integrated; Rust and Documentation checks pass at the exact merged revision |
| D2 — Publish documentation | D1 and authorization for Pages configuration/deployment | Public EN/VI site, search and exact-revision Rustdoc verified on the hosted artifact |
| D3 — Confirm alpha contract | D2 | Recorded decision covering seven packages, version, compatibility, publisher and Create checkout policy |
| D4 — Prepare and review candidate | D3 | Committed versioned candidate with full local verification, package inventory and release notes |
| D5 — Publish and prove consumption | D4 and authorization to publish the exact candidate | All seven packages available; actual registry and installed-CLI consumers pass; docs.rs checked |
| D6 — Close delivery | D5 | Release tag/notes and docs agree with actual artifacts; concise next consumer backlog |

Stopping after D2 is a useful completed delivery if the alpha decision is deferred. D3–D5 are proposed release work, not an obligation to publish regardless of findings. A later execution instruction may authorize several stages together; do not ask repeatedly for an action already authorized.

## D1 — Integrate the existing branch

**Execution:** Complete at merged main revision eb5d689; see [integration evidence](../../plans/2026-11-06-public-delivery/integration.md).

**Files:** Read `.github/workflows/{ci,documentation,pages}.yml`, `.github/scripts/{rust-tools.sh,restore-docs-dependencies.py}` and the accepted documentation evidence. Change only reproduced workflow/docs defects.
**Produces:** Reviewed main revision and run URLs. Record them in `docs/plans/2026-11-06-public-delivery/integration.md` during execution.

- [x] Fetch and reconcile the live branch, remote and default branch. Preserve unrelated work; do not reset or force-push.
- [x] Review the full branch against main, using existing slice acceptance to focus on integration risk. The five unpushed commits are not the full PR scope.
- [x] Prepare a PR title/body describing the completed framework and documentation delivery, carried evidence, current unpublished status and fresh checks. Use a body file with actual newlines; open a draft PR only after push/PR authorization and attach it to the Codex chat.
- [x] Push the reviewed branch and observe both Rust and Documentation workflows for its exact head SHA. Do not substitute the older green run at `94907cb`.
- [x] Inspect hosted failures before editing. Fix their cause, add a focused regression only for changed executable behavior, commit, then repeat affected checks. Keep strict link/provenance checks and supported runner/action versions.
- [x] Merge after review and required checks. Prefer a merge commit so acceptance/source commits remain in ancestry; if repository policy requires another method, verify referenced commits remain reachable rather than silently rewriting evidence.
- [x] Verify Rust and Documentation again at the resulting main SHA and record the PR, merge SHA, run URLs, tool versions and actual results.

Initial read-only commands, from the repository root:

```sh
git fetch origin
git status --short --branch
git rev-list --left-right --count origin/main...HEAD
git diff --stat origin/main...HEAD
gh pr list --head feat/cordis-completion --state open
gh run list --branch feat/cordis-completion --limit 10
```

After push authorization, use `git push -u origin feat/cordis-completion`. For a known run ID, use `gh run view` to confirm its head SHA and jobs before `gh run watch --exit-status`. Do not bypass failed required checks or use admin merge as a workaround.

**Exit:** The complete accepted branch is integrated and both workflows are green on main. No crates published and no Pages deployment inferred from a preview artifact.

## D2 — Deploy and verify public documentation

**Execution:** Complete; final public revision 638ae03 and its live checks are recorded in [documentation evidence](../../plans/2026-11-06-public-delivery/documentation.md).

**Files:** Existing `pages.yml`/`documentation.yml`, `website/**`; afterwards update actual availability claims in `README.md`, `docs/{README,release-readiness,limitation,todo}.md`, `AGENTS.md` and matching `docs/{en,vi}/guides/documentation.md` only where needed.
**Produces:** Actual site URL, deployment/run SHA and smoke-test evidence in `docs/plans/2026-11-06-public-delivery/documentation.md`.

- [x] Confirm `pages.yml` exists on main. Inspect current Pages source, `github-pages` environment and repository variable before changing settings.
- [x] With deployment authorization, select GitHub Actions as the Pages source, configure appropriate allowed branches/reviewers, and set `PAGES_DEPLOY_ENABLED=true`. Preserve existing stronger protection.
- [x] Dispatch the existing workflow on reviewed main: `gh workflow run pages.yml --ref main -f deploy=true`.
- [x] Record the SHA GitHub actually ran and require its documentation verification jobs and deployment job to succeed. Use the hosted macOS-built Rustdoc artifact; never upload the workstation export.
- [x] Open the URL returned by the deployment. Expected project base is `https://antruongnguyen.github.io/re-cordis/`; verify it rather than assuming it.
- [x] Exercise EN/VI entry pages, deep-link reload, language preservation, EN “cleanup” and VI “dọn dẹp” search, keyboard selection/Escape, mobile navigation, copy controls and a real 404 recovery.
- [x] Open HMR's optional Include API in Rustdoc. Confirm revision directories and public provenance identify the deployed source; check source links resolve on GitHub. Inspect public Markdown and llms.txt. Require the static checker to preserve internal-tree exclusion across all public outputs.
- [x] Confirm the known dependency-owned Rustdoc link boundary is unchanged or specifically reviewed; do not widen exceptions to silence a hosted failure.
- [x] Update availability/status documentation only after observing the live result. Run documentation regressions and both full guide checks for changed guides. Commit/review/integrate those edits, then deploy the final documentation revision and record its result.

If deployment fails before publication, retain the last successful site and report the failing job. If a deployed revision is defective, deploy a previously verified source revision through the same checks, or a reviewed corrective revision; do not force-move source history.

**Exit:** Users can read verified public documentation with working source/Rustdoc links. Registry availability is still described separately.

## D3 — Record the release decision

**Execution:** Candidate preparation approved by the maintainer; [decision and publisher designation](../../plans/2026-11-06-public-delivery/release-decision.md). Publication remains separately gated.

**Files:** `docs/release-readiness.md`; create `docs/plans/2026-11-06-public-delivery/release-decision.md` when the decision is made.
**Produces:** A concrete approved or deferred release contract consumed by D4.

- [x] Present the existing seven-package proposal, with no expanded API/platform/template scope.
- [x] Confirm coordinated `0.1.0-alpha.1`, exact consumer pins and an alpha policy permitting documented breaking changes only in a new coordinated alpha version.
- [x] Confirm publisher account/authority and recheck exact normalized crate names at release time. Inspect an existing package's ownership if present; absence is not a reservation.
- [x] Confirm the support statement, actual documentation URL, proposed tag `v0.1.0-alpha.1` and the compatible complete checkout for offline Create. Record the final candidate SHA as the intended target of that tag in release evidence before publication.
- [x] State clearly that the published Create CLI still requires `--cordis-path`; publishing it does not produce a registry-backed template.
- [x] Record approval to prepare version/dependency/publication-policy changes, or record deferral and stop at the completed D2 outcome.

**Exit:** One explicit release contract. Choosing this planning sequence is not that release approval.

## D4 — Prepare one reviewable alpha candidate

**Files:** Root `Cargo.toml`/`Cargo.lock`; all seven `crates/*/Cargo.toml` and package READMEs; `CHANGELOG.md`, current status/decision/limitation docs, and matching EN/VI installation/Create/documentation guidance. Preserve the template's manifest/source and notices unless a reproduced compatibility failure requires a separately reviewed correction.
**Produces:** A reviewed candidate SHA, package inventory, local verification report and draft release notes in `docs/plans/2026-11-06-public-delivery/release-candidate.md`.

- [ ] Start `feat/alpha-release` from integrated main after checking for an existing suitable branch/worktree. Inspect branch protection and preserve source/acceptance history.
- [ ] Set the shared package version to `0.1.0-alpha.1`. For the approved seven crates, remove the publication prohibition or explicitly restrict publication to crates.io.
- [ ] Add `version = "=0.1.0-alpha.1"` alongside every internal dependency path, including optional HMR Include and HMR's dev-only logger edge. Re-resolve and review Cargo.lock; preserve third-party features/pins and HMR defaults.
- [ ] Update package/readme/release text for the actual alpha contract. Use immutable release-tag guide links and exact-version docs.rs links for release references; keep the development site distinguishable from the release snapshot. Draft links do not count as reachable documentation.
- [ ] Re-run the full shared acceptance matrix below on the versioned candidate, not just the prior 517-test audit. Check each crate's minimal normal dependency graph as well as unified workspace features.
- [ ] Inspect all intended Cargo file lists and normalized manifests where archives can actually be built. Confirm package LICENSE/README, core upstream notices, examples/tests and the embedded Create template; exclude repository-only evidence.
- [ ] Verify actual core/Create archives and their tests on both toolchains. Extract outside any Cargo workspace; Create tests use a separate candidate checkout via `CORDIS_CREATE_TEST_CHECKOUT`. Archive contents must remain unchanged by verification.
- [ ] Record which satellite archive/dry-run checks await real registry prerequisites. Do not claim a seven-package registry preflight from local path builds, no-verify archives or patch overrides.
- [ ] Review changes and results, resolve material findings, commit/integrate the candidate and pass hosted checks. Record the final immutable SHA outside the commit it identifies to avoid self-referential metadata.
- [ ] Present that candidate, package list, results, remaining progressive checks, release notes and partial-publication recovery rule for publication authorization. No automatic publication workflow is needed for this one-off alpha.

**Exit:** A clean, reviewed versioned candidate with honest local-vs-registry evidence and an exact publication scope.

### Shared release verification

Use [B1's complete shared matrix](../../plans/2026-11-02-cordis-completion/evidence/2026-11-05-b1-closure.md#complete-fresh-shared-verification) and its tracked [R0 runner reference](../../plans/2026-11-02-cordis-completion/evidence/r0-run-matrix.py.txt). Copy/adapt a temporary runner only for the current revision and output paths; do not depend on an old ignored script or create a new CI framework.

For each of stable and 1.85.0, source `.github/scripts/rust-tools.sh` from the repository root, select a distinct `CARGO_TARGET_DIR`, deliberately restore dependencies, then execute:

```sh
"$CORDIS_CARGO" check --workspace --all-features --all-targets --locked --offline
"$CORDIS_CARGO" test --workspace --all-features --locked --offline
"$CORDIS_CARGO" clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
"$CORDIS_CARGO" doc --workspace --all-features --no-deps --locked --offline
"$CORDIS_CARGO" test -p re-cordis-hmr --no-default-features --locked --offline
"$CORDIS_CARGO" test --manifest-path spikes/m0/Cargo.toml --locked --offline
"$CORDIS_CARGO" clippy --manifest-path spikes/m0/Cargo.toml --all-targets --locked --offline -- -D warnings
```

Run the existing lifecycle, timer_lifecycle, console, boot_reconfigure, config_file, reload and system_lifecycle examples, plus system_lifecycle's real-watch mode and the offline GitHub digest, with B1's exact arguments. Run stable formatting, nine docs regressions, full docs checkers and loader comparisons on both toolchains, and the core six-scenario comparison in the explicit stable environment. Restore the pinned TypeScript prerequisites described by the existing parity guides; do not substitute another reference pin.

```sh
python3 -B tests/docs/test_check.py
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
python3 -B tests/loader_parity/run.py --toolchain stable
python3 -B tests/loader_parity/run.py --toolchain 1.85.0
python3 -B tests/parity/run.py
```

Full guide checks include real generated basic/R0 applications and their 100 lifecycle cycles. Retain six core scenarios/73 observations and two loader scenarios/nine observations as bounded evidence. Use actual result counts, not a hardcoded historical total.

For site checks, follow [website build instructions](../../../website/README.md#build-and-preview): fresh same-revision dependency-inclusive Rustdoc, frozen Bun install, build, typecheck, lint and check. Cargo and site assembly remain serialized. Hosted Documentation additionally covers both HMR Rustdoc configurations.

## D5 — Publish sequentially and test real distribution

**Entry:** Explicit authorization for the exact D4 candidate, seven packages and registry. Configure authentication using the normal credential flow without printing or committing tokens.
**Produces:** Registry versions/checksums, package/CLI consumer results and docs.rs URLs.

Publish in this order, one package at a time:

1. `re-cordis-core`
2. `re-cordis-timer`
3. `re-cordis-logger`
4. `re-cordis-loader`
5. `re-cordis-include`
6. `re-cordis-hmr`
7. `re-cordis-create`

- [ ] Before each package, confirm a clean unchanged candidate and exact expected package/version. Wait for predecessors to resolve from the real crates.io index; check HMR's optional/dev edges too.
- [ ] Build/inspect the actual normalized archive, test its packaged examples/tests and run `cargo publish --dry-run` against those real prerequisites. Select explicit stable/1.85.0 tools for archive verification. Preserve Create's separate checkout fixture and no-default/include checks for HMR.
- [ ] With that package's progressive checks passing, publish from the authorized candidate using normal verification. Do not add `--no-verify` or `--allow-dirty` to bypass failures.
- [ ] Confirm the exact registry version, checksum and ownership before continuing. If the tool reports an ambiguous result, query the registry before retrying publication.
- [ ] After all six libraries exist, build an external application using exact registry versions for all six, with HMR's `include` feature. Reuse the accepted R0 application/tests in a disposable fixture; only its six dependency sources change. Cargo metadata must show registry sources and no Cordis path/git/`[patch]` overrides. Pass build, tests, strict Clippy, normal run and real-watch run on stable/1.85.0.
- [ ] Install the actual published Create version into a temporary root with `cargo install re-cordis-create --version '=0.1.0-alpha.1' --locked --root "$cordis_install_root"`. Define that task-specific temporary directory first. Use the installed binary and a separately obtained checkout at the recorded compatible SHA. Verify its five generated files and MIT notice, then the generated application's build, five tests/100 lifecycle cycles, run and strict Clippy on both toolchains.
- [ ] Check actual docs.rs builds for all seven exact versions and HMR's optional Include methods. The [docs.rs assessment](../../plans/2026-11-05-documentation-site/docs-rs-assessment.md) is preparation, not hosted success; investigate failures without claiming Linux runtime support.
- [ ] Record results in `docs/plans/2026-11-06-public-delivery/publication.md`. Keep candidate SHA, archive/checksum, toolchain, run/registry URL and observed result distinct.

**Failure/recovery:** Stop on a real package, registry-consumer or docs.rs failure. Report exactly which versions are already published. Published bytes are immutable; do not automatically yank or claim rollback. Transient index/network failure may be retried against the same candidate after checking state. Any source/dependency correction requires review and an explicit version/partial-release decision before continuing.

**Exit:** All seven registry packages and exact-version docs.rs pages are verified, the installed CLI works with its documented checkout, and the external registry consumer passes. Partial publication is not completion.

## D6 — Close delivery and choose the next consumer improvement

**Files:** `CHANGELOG.md`, current README/status/readiness/decision/limitation docs and paired user guides; new `docs/plans/2026-11-06-public-delivery/closure.md`.
**Produces:** Coherent published release and a small evidence-driven next backlog.

- [ ] Create the authorized annotated release tag `v0.1.0-alpha.1` at the exact published source SHA; verify no conflicting tag exists and never overwrite one. Publish release notes with package scope, install paths, compatibility limits and known limitations.
- [ ] Verify tag-pinned EN/VI guide links, source links, package links and docs.rs URLs. Preserve immutable release references even as development docs move.
- [ ] Update claims to observed results, integrate documentation corrections with their required checks, and deploy the final public docs revision. Record the release source SHA and later documentation SHA separately.
- [ ] Mark D1–D6 done only with their evidence links. Keep the 35 implementation packages closed and historical evidence intact.
- [ ] Use one actual consumer to identify the next priority: onboarding friction, a reproduced lifecycle issue, or a specifically required platform. Reuse the existing GitHub activity application if useful; no live collection, public webhook or notifications are implied by this plan.
- [ ] Create only a short ranked backlog with a concrete reproduction or consumer requirement for each item. Broader platform acceptance and registry-backed Create remain optional follow-ups.

## Plan review and execution handoff

This plan reuses existing delivery machinery and release policy. It introduces no automation, runtime code or new package in the planning change. Alternatives considered: stopping at public docs is a valid D2 outcome; platform-first delivery would add a separately scoped acceptance project. The user selected docs followed by alpha.

Execution should begin with D1. Read this plan and its linked specs, recheck Git/live hosting state, prepare the concrete PR and verification scope, and obtain only missing authorization for the requested external actions. Keep independent review proportional to each change; if subagents are selected during execution, follow the repository's documented routing and isolation rules.

Planning validation: check Markdown links/structure and the roadmap pointer, inspect the diff, then commit the plan. Runtime/site matrices belong to their execution stages; no deployment or release check is marked passed merely by writing this plan.
