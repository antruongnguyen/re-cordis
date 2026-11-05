# Documentation delivery execution plan

Date: 2026-11-05 (Asia/Ho_Chi_Minh).
Spec: [approved session prompt](../2026-11-05-documentation-site-session.md).
Baseline: 94907cb on feat/cordis-completion; clean working tree.
Remote: https://github.com/antruongnguyen/re-cordis.git; project path /re-cordis.

## Constraints and ownership

Preserve seven 0.0.0/publish=false crates, production APIs/features/dependencies,
Cargo locks and five-file Create template. Reuse accepted B1 runtime evidence;
this work does not reopen the 35 accepted work packages. Keep the reviewed local
docs/llms.txt with checkout-relative links and its existing constraints.

- Website implementer: website/** only, canonical EN/VI compilation and checks.
- Package implementer: minimal docs.rs metadata, bilingual documentation-channel
  guide and package assessment; no Cargo execution.
- Controller: workflows, shared documentation, all Cargo/full guide execution,
  browser verification, integration and local commits.
- Fresh independent reviewer: completed artifact, workflows, package preparation
  and evidence; substantive corrections receive fresh scoped re-review.

## Gates

1. Consult current official Fumadocs/Next/Pages/docs.rs documentation. Keep
   docs/en and docs/vi authoritative. Build the standard Fumadocs layout with
   explicit locale routes, static browser search, README/Markdown/source link
   mapping, per-page Markdown, public llms index and same-revision Rustdoc.
   Verify frozen Bun install, type/lint/build and all exported links/assets.
2. Prepare read-only website/docs CI with explicit Cargo/Bun restoration. Keep
   runtime guide checks on macOS, stable and 1.85.0; preserve existing Rust CI.
   Separate manual Pages deployment behind an owner opt-in and protected
   github-pages environment, using official configure/upload/deploy actions.
3. Run nine existing Python regressions and full docs checks on both resolved
   toolchains. Run warnings-denied workspace all-feature Rustdoc and focused
   HMR optional-feature/no-default Rustdoc checks. Validate a cold Cargo cache.
4. Preview only exported files under /re-cordis. Exercise EN/VI search,
   language switching, reload, source/API links, 404, mobile and keyboard use.
   Verify public Markdown/LLM output against actual artifacts and claims.
5. Update current docs and save exact commands, versions, input fingerprints,
   failures/corrections and boundaries. Commit each verified milestone locally;
   independently review, correct/re-review, then commit final evidence.

No push, deployment, repository settings, crate publication, tags, remote
Create or framework expansion. A local build proves neither hosted Pages nor
hosted docs.rs success; Linux compilation does not establish runtime support.
