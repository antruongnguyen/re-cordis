# R1 readiness assessment and acceptance

Date: 2026-11-05 (Asia/Ho_Chi_Minh).
**Verdict: R1 completed and independently accepted within its assessment scope.**
Current readiness remains **local/Git checkout consumption only**; the proposed
experimental release is not approved for publication.
R1 concerns readiness assessment and a concrete release proposal, not publication.

## Baseline and scope

Started from clean `e3f0f656aabfecb0088c5d63e1306231bfbe0188` on
`feat/cordis-completion`; there was no existing R1 work or newer work to discard.
Carry R0 executable `35556d5108eaf13104a5fb287c83b28ffb5d1576`,
generated-project checker `4742efe44a55dd1464eb851f9d9eb11d66fde82b`,
Create acceptance `e8b5eb5`/executable `8373fa3`, HMR acceptance
`4c8f423`/executable `55a55d1` and the accepted earlier satellites/core/B0.

Read AGENTS.md, progress, shared acceptance/master plan, R0 acceptance/review/
re-review/source map, decision/limitation and documentation maintenance rules.
[The R1 plan](r1-plan.md) records ownership and measurable gates.
The [user-facing report](../../../release-readiness.md) is the primary proposal;
[package audit](r1-package-audit.md) records actual distribution inspection.

All Rust implementation, tests, checker scripts, manifests, lockfiles and feature
definitions remain unchanged. Two existing files in R0's 166-file fingerprint
set change: Create's crate README and basic template README. The other 164 match
the retained R0 hashes. Six package READMEs and seven package LICENSE files are
new outside that set. This is documentation/notice correction, not a template
variant or dependency/feature change. The generated application still contains
five files and identical Rust/manifest inputs.

Accordingly R0's 40-stage/517-workspace-check result per toolchain is reused as
**carried** runtime evidence, not a fresh R1 full matrix. The 12 GitHub application
tests, six paired core scenarios/73 observations, two loader scenarios/nine
observations, timer's prior comparison and accepted B0 accounting are unchanged.
None is added to R1's package command/test totals or silently reclassified.
Current filesystem/watcher runtime evidence remains macOS only.

## Findings and delivered corrections

| Finding | Reproduced evidence | Focused correction |
|---|---|---|
| Missing MIT text in distributed source archives | All seven baseline file lists and actual core/Create archives omit root LICENSE | Seven package-local byte-identical MIT notices |
| Missing package onboarding | Six package READMEs absent; Cargo readme metadata null | Six package-specific READMEs; Create distribution clarification |
| Missing notice in generated template output | Actual five-file app declares MIT but lacks permission/copyright text | Exact root notice appended to existing generated README; no sixth file |
| Documentation size regression during R1 | Initial stable/MSRV checker exits 1 at llms.txt <8192 assertion; file was 8510 bytes | Remove duplicated prose, retain readiness reading path; 8157 bytes, checker unchanged |

Source provenance remains qualified: our root MIT notice does not certify
upstream attribution or replace applicable Shigma/third-party notices. Actual
release preparation must close distribution-specific provenance/notice work.

## Executed verification

Package executor: fresh gpt-6.1-sol/high, fork_turns=none, writing only its
audit report and ignored target/r1-audit. Controller owns shared docs/corrections.
Cargo ownership was explicitly transferred before controller docs execution.
No concurrent Cargo/docs/parity operation touched shared fixtures.

| Fresh R1 check | Result |
|---|---|
| Actual Cargo metadata/all-feature graph, both toolchains | Same 74 packages; normal/build/dev and target edges inspected separately |
| Seven package file lists on stable and 1.85.0 | All pass; final counts core 48, timer 23, logger 12, loader 27, Include 25, HMR 20, Create 14 |
| Core/Create archive construction and verification builds | Pass on stable and 1.85.0; actual archive MIT bytes verified |
| Five satellite archive construction attempts | Expected exit 101: versionless core path dependency; no archive/build success claimed |
| All publish dry-runs | Expected exit 101 at publish=false; registry dependency verification was not reached |
| Isolated versionless dev-dependency normalization probe | Confirms Cargo drops that edge; does not construct a fake HMR release |
| Exact crates.io API requests | Seven HTTP 404 responses on audit date; no ownership/availability promise |
| python3 -B tests/docs/test_check.py | Eight regressions pass |
| python3 -B tests/docs/check.py --toolchain stable | Pass after llms correction: 34 page pairs, 13 unique snippets, external quick start, actual Create and generated R0 apps |
| Same docs checker with --toolchain 1.85.0 | Pass with equivalent coverage |
| Generated basic app, both docs checker runs | Five tests including 100 cycles; build/run/strict Clippy pass |
| Generated R0 app, both docs checker runs | Eight tests including 100 complete cycles; build, default and real-watch run, strict Clippy pass |
| cargo test -p re-cordis-create --locked --offline, each explicit toolchain | 22 package tests pass on each |
| Additional real stable Create output notice probe | Exactly five files; generated README contains root MIT bytes; other four files match the earlier actual generated app byte-for-byte |
| Changed Markdown links/anchors via existing checker links function | Final 32-page sweep passes; earlier 28/29/30-page sweeps are not additional unique pages |
| git diff --check | Pass for inspected work |

The fresh basic/R0 external suites and repeated package checks are not new
unique tests. Package verification builds do not execute archive all-target
test suites. Full shared workspace/M0/Clippy/Rustdoc/parity execution was not
repeated because executable/dependency inputs did not change; no such fresh
pass is claimed.

## Commands, tools and retained logs

All Cargo/compiler/doc/lint tools resolve through rustup on each selected
toolchain: cargo, rustc, rustdoc, cargo-clippy and clippy-driver. Matching
RUSTUP_TOOLCHAIN, absolute RUSTC/RUSTDOC and the toolchain bin first in PATH
prevent Homebrew fallback. Stable cargo-fmt and rustfmt resolve explicitly;
formatting is stable-only. No component installation occurred.

- Stable Cargo/rustc/rustdoc 1.97.1, Clippy 0.1.97, rustfmt 1.9.0-stable.
- MSRV Cargo/rustc/rustdoc 1.85.0, Clippy 0.1.85.
- Host independently queried during R1: arm64 macOS 26.7.1, build 25G241;
  Python 3.15.2 and Git 2.56.0. These runs add no Linux/Windows or alternate
  macOS architecture/version evidence.

Reproduce from the repository root with the explicit rustup environment in
[shared acceptance](../acceptance.md#toolchain-matrix), using the package audit's
stage commands and:

```sh
python3 -B tests/docs/test_check.py
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
cargo test -p re-cordis-create --locked --offline
```

Run the Cargo command once per explicitly selected toolchain. The docs checker
selects matching tools itself; the controller additionally validates their
common bin directory. Fresh package commands used --allow-dirty only for
authorized doc additions; release packages must be rebuilt from the final
clean release revision.

Ignored raw evidence:
- target/r1-audit/commands.jsonl, tools.json, final-summary.json, metadata,
  initial/final .crate archives, package lists, registry-checks.json and logs.
- target/r1-final/tools.json, checks.json and initial docs logs retain the
  llms failure; checks-corrected.json and docs-corrected-{toolchain}.log retain
  successful final guide/application execution.
- target/r1-final/focused-checks.json and create-tests-{toolchain}.log record
  22-test package runs and real generated-notice evidence.
- target/r1-final/input-reuse.json records the 166-input comparison;
  changed-links.json records the initial link sweep.

Ignored logs are supporting local artifacts, not promised distribution files.
This report and the audit retain the commands, counts, expected failures and
scope needed to reproduce them.

## Concrete recommendation and remaining decisions

**Current verdict: ready only for local/Git checkout consumption.**
Propose 0.1.0-alpha.1 for six runtime crates and the offline explicit-checkout
Create tool, all coordinated; no facade. Runtime order is core, then
timer/logger/loader, then Include, then HMR. Create has no Cargo prerequisite,
but comes last for workflow validation. HMR's dev logger edge needs a tested
version to preserve packaged example/test usability.

Actual publication remains gated on a separate scope/version/MSRV/platform
decision; exact internal version requirements; normalized package and registry
resolution/build/test verification; name/owner authority; provenance/notices;
reachable release docs and the compatible Create checkout. macOS-only evidence,
local/non-Send hosting, bounded parity, synchronous I/O, polling gaps, value YAML
and offline paths are nonblocking limitations only within that proposed scope.
Remote/registry templates, extra templates, platform expansion, process
supervision, stable API and standalone B1 remain deferred.

## Review and revision closure

Audit/package-notice milestone: `b45a11cc2e9b95d64c213f47c2961ce50d6f15ad`.
A fresh gpt-6-astra/high reviewer inspected exactly baseline..b45a11c, reproduced
clean package evidence and exercised the actual packaged Create CLI. See
[the independent review](r1-review.md) for commands, failures and findings.

Two focused documentation corrections follow that review (R1-R1 and R1-R2): CHANGELOG now says
the proposal requires a separate release decision, and the primary report
explicitly records six stable extracted-archive scaffold test failures caused
by its repository-relative checkout fixture. Five library and 11 scaffold tests
pass in that run; the MSRV archive suite was not run. A prior parent-workspace
inference error belongs to the review harness and was isolated in ignored
scratch configuration without changing archive bytes. Neither failure is
relabeled as a pass.

The unchanged packaged CLI with explicit real checkout passes fresh generated
application build/five tests/run/strict Clippy on both toolchains. Its exact
five-file output includes MIT; four non-README files match initial output.
The release decision must choose an explicit checkout-aware archive fixture or
retain those tests as checkout-only with packaged-CLI distribution coverage.
No generator/test redesign is introduced in this R1 documentation correction.

Correction revision: `82d8f70e939e7886db835fcfb55e8d630c745e87`.
A second fresh gpt-6-astra/high context reviewed the exact b45a11c..82d8f70
delta and retained artifacts. [Scoped re-review](r1-rereview.md) marks both
R1-R1 and R1-R2 ADDRESSED with no new actionable finding and authorizes local
assessment closure. Fresh archive-byte, policy, R0-fingerprint, link/anchor
and exact-diff checks pass; no redundant Cargo/full-matrix execution is claimed.

The final acceptance documentation revision is the containing closing commit
on feat/cordis-completion. Its only changes after 82d8f70 are the scoped review
report and final status/evidence links. Audited package contents remain at
b45a11c; the final reviewed proposal correction is 82d8f70. No implementation,
template, dependency or feature input changes after package verification.
Identify the closing commit at this checkpoint with:

```sh
git log -1 --format=%H -- docs/plans/2026-11-02-cordis-completion/evidence/r1-acceptance.md
```

Final closure passed the existing 34-pair structure/export checks, a 32-page
changed-Markdown link/anchor sweep, publication/version/notice assertions and
git diff --check. Package inputs are unchanged since b45a11c and guide/API
inputs since the reviewed correction. The exact results are recorded in
target/r1-final/closure-checks.json. R1's completed
evidence is separate from the recurring checklist in shared acceptance.
Publication/version changes remain excluded; no next package, push, tag or
background continuation is started.
