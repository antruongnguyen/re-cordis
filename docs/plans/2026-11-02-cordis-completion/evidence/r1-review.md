# R1 independent readiness review

Date: 2026-11-05 (Asia/Ho_Chi_Minh).
Baseline: e3f0f656aabfecb0088c5d63e1306231bfbe0188.
Reviewed audit/correction/proposal: b45a11cc2e9b95d64c213f47c2961ce50d6f15ad,
on feat/cordis-completion.

**Verdict: the assessment and proposed scope are sound; two focused
documentation corrections are required before local R1 acceptance.**
Neither finding requests runtime/API redesign, version/dependency changes or
publication. The current local/Git-checkout-only recommendation is supported.

## Findings

### R1-R1 — P2: do not imply the release proposal is already approved

Location: CHANGELOG.md:37–38.

The new entry calls this a “concrete, separately approved release proposal”.
The reviewed task authorizes assessment and a proposal; the release decision
is still outstanding. This wording contradicts the explicit decision boundary
in the readiness report and can mislead a reader using only the changelog.

Correction: say “a concrete proposal requiring a separate release decision”
(or equivalent). The controller independently flagged the same wording during
review; this is one finding, not two.

### R1-R2 — P2: name the demonstrated Create archive-test limitation

Locations: docs/release-readiness.md:143 and 223–226; the corresponding
verification/remaining-gates sections in r1-acceptance.md.
Underlying existing test helper: crates/re-cordis-create/tests/scaffold.rs:27–33.

The report correctly distinguishes successful Cargo archive verification builds
from unexecuted archive test suites. Independent review went one stage further:
the unchanged stable Create archive compiles its tests, but six scaffold tests
fail because checkout() assumes that CARGO_MANIFEST_DIR/../.. contains the
complete repository. At an extracted package location it does not contain
crates/re-cordis-core/Cargo.toml. This is now a demonstrated package-test fixture
limitation, rather than only missing archive-test evidence.

Fresh stable result: five library unit tests pass; of 17 scaffold tests,
11 pass and six fail (exit 101). The failed tests are:

- complete_file_set_and_repeat_refusal — source assertion at line 54;
- empty_directory_and_native_unicode_spaces_destination — line 108;
- cli_success_uses_argument_arrays_and_explicit_checkout — line 198;
- native_nonunicode_destination_obeys_host_filesystem_and_cleans_failure — line 339;
- stage_shaped_and_long_destinations_generate_complete_projects — parent
  assertion at line 429 after the child reaches line 447;
- other_cordis_crate_names_remain_available_for_projects — line 466.

The primary cause is “checkout is missing …/stable/crates/re-cordis-core/Cargo.toml”.
The non-Unicode case consequently observes validation exit 2 before reaching
its expected macOS filesystem exit 1. This is not a generator failure when
given the required valid checkout: the archive-built CLI and both generated
applications pass the separate checks below.

Correction: add this precise observed limit to readiness/acceptance evidence
and the subsequent release package-test gate. State that release preparation
must supply a documented compatible fixture or make those tests independent
of repository location before claiming an independently passing archive suite.
Do not drop tests, fabricate a full checkout inside the archive, or claim that
the successful package verification build executed them. No R1 production/test
redesign is required. No MSRV archive-suite execution is claimed.

## Fresh independent verification

Review began at a clean b45a11c. The exact diff contains 37 documentation/notice
files; no Rust source, test/checker, manifest or lockfile change. Comparison
against the retained R0 final-inputs.json confirms 164 of 166 inputs unchanged;
only the Create crate README and basic template README differ. Six new package
READMEs and seven new LICENSE files are outside that original fingerprint set.

All tools were independently resolved through rustup for stable and 1.85.0:
cargo, rustc, rustdoc, cargo-clippy and clippy-driver have the same selected
toolchain bin; matching RUSTUP_TOOLCHAIN/RUSTC/RUSTDOC and leading PATH were set.
Stable cargo-fmt/rustfmt were resolved but no new formatting run is claimed.
Versions match the audit: stable Cargo/rustc 1.97.1 and MSRV 1.85.0.
No toolchain installation or Homebrew compiler was used.

| Fresh check | Observed result |
|---|---|
| All-feature locked/offline Cargo metadata, both toolchains | 74 packages; identical resolved graph; seven 0.0.0/publish=false/MIT/Rust-2024/MSRV-1.85 package policies |
| Seven clean package lists per toolchain | All pass; core 48, timer 23, logger 12, loader 27, Include 25, HMR 20, Create 14 files |
| Clean core/Create package verification, both toolchains | All four pass without --allow-dirty; each archive records exact b45a11c VCS hash without dirty flag |
| Archive/package notice and metadata inspection | Seven LICENSE files equal root bytes; all READMEs resolve; normalized core/Create metadata preserves policy and README; Create embeds the exact template notice |
| Stable HMR package --no-verify | Expected 101 for versionless core path dependency; no constructed satellite archive claimed |
| Stable seven publish --dry-run attempts | All refuse at publish=false with 101; no registry validation/upload |
| Stable extracted Create tests | Five unit and 11 scaffold tests pass; six scaffold tests fail as R1-R2 describes |
| Actual archive-built Create CLI, both toolchains | Generates exact five-file app; MIT notice present; four non-README files match original actual generated output byte-for-byte |
| Each actual generated basic application | Offline build, five tests including 100 lifecycle cycles, run and strict all-target Clippy pass |
| Documentation checker regressions | Eight pass |
| Structure/export checks and changed Markdown links | 34 EN/VI pairs, all current indexed exports, 13 discovered snippets, 29 changed Markdown pages pass; llms.txt is 8157 bytes |

The snippet count in the last row is discovery, not a fresh reviewer execution.
Both full documentation checkers, including the external R0 program, are
inspected controller evidence below. Repeated/generated tests are not additional
unique tests.

Raw reviewer artifacts are under ignored target/r1-review: tools.json,
commands.json, content-checks.json, generated-checks.json,
documentation-checks.json and per-stage stdout/stderr. These are supporting
local artifacts, not distributed files. The fresh commands use argument arrays;
core/Create packaging is cargo package -p PACKAGE --locked --offline.
Generated app test/run/Clippy commands add --locked after their initial offline
build resolves the application lockfile.

For the R1-R2 reproduction, the stable Create archive SHA-256 is
5413dafe6cd29b2da8c6e137ccd8d982a182e0f221de9fbaf6345933ac0d0bd8.
Cargo first refused direct nested archive tests because of repository workspace
discovery; two setup-attempt logs remain preserved. A scratch parent workspace
at target/r1-review/stable/Cargo.toml declares only the unchanged extracted
package as a member, with resolver 3 and no added dependencies. This admits:

    cargo test --manifest-path target/r1-review/stable/package/re-cordis-create-0.0.0/Cargo.toml --offline

The selected absolute stable Cargo path and full output are recorded as
stable-create-archive-tests-member in commands.json and matching log files.
All files from each extracted Create archive were compared byte-for-byte with
its .crate after CLI execution. No production or extracted package manifest,
source, fixture or template was modified. Archive verification is still a pass;
the later test-stage failure remains visible and distinct.

## Inspected prior evidence and readiness judgment

I inspected the package auditor's 107 command records and outputs: 71 successful
commands and 36 expected dependency/publication refusals, with no mismatched
exit. Its final lists/notices match the independent results. The seven retained
registry HTTP 404 records are point-in-time absence evidence only; no fresh
reviewer registry query, ownership/reservation claim or publication occurred.

Controller logs retain the initial llms.txt size failures and successful
corrected stable/MSRV documentation runs: 34 page pairs, 13 executed snippets,
quick start, actual basic Create apps (five tests each) and actual generated
R0 apps (eight tests each, 100 cycles, default/watch runs and strict Clippy).
Create's 22 checkout-based tests pass on both controller toolchains. These
controller executions do not conflict with the newly observed relocated
archive-test fixture assumption.

R0's 40 stages and 517 workspace checks per toolchain remain carried evidence,
supported by the 166-input comparison. They were not rerun by this reviewer,
nor combined with package probes. Accepted B0/core/satellite evidence is reused;
this review adds no whole-program parity, platform or stability promise.

The manifests, inspected public roots and dependency graph support the seven
package proposal. Core's Tokio rt/sync production graph remains separate from
dev feature unification; TimerExt is sealed; logger adapts core logging; Group
belongs to loader; HMR re-exports loader's replacement type and defaults to no
Include feature. Create is dependency-free as a package while generated apps
require core/logger/loader and a compatible retained checkout.

The proposed coordinated 0.1.0-alpha.1 and exact internal version requirements
are appropriately future work. The normal/optional DAG permits core, then
timer/logger/loader, then Include, then HMR; offline Create can build independently
and be validated last. HMR's versionless dev logger normalization concern is
correctly identified as an example/test closure issue, separate from production
edges. No facade or separate Group package is justified by current consumers.

Current package/runtime usefulness does not establish publication readiness.
The report correctly keeps ownership, real registry prerequisites/normalized
archives, provenance/notices, reachable release documentation and compatible
Create checkout as release gates. Local non-Send hosting, joined cleanup,
bounded parity, synchronous I/O, polling/YAML limits and tested Apple Silicon
macOS-only evidence are visible experimental-scope limits. Remote/registry
Create, platform expansion, stable API, broad B1 and core redesign remain
outside R1.

No further actionable issue was found in the reviewed scope. Re-review only
the two documentation corrections and final acceptance/status consistency.
Cargo/docs execution ownership was explicitly returned to the controller after
the fresh checks. This reviewer changed only this report and ignored review
artifacts, and made no commit, push, tag or publication.
