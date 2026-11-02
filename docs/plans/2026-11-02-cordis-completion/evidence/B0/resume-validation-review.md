# B0 integrated resume validation — scoped independent review

Date: 2026-11-02. **Spec verdict: accepted with the preservation boundary
below. Quality verdict: no material finding. The bounded integrated-check
gate is satisfied; no further validation run or artifact repair is required.**
This does not turn the saved report's exit-1 check into a pass.

Reviewed [resume-validation.md](resume-validation.md) against
[its execution handoff](handoffs/resume-verification.md) and
[this review handoff](handoffs/resume-validation-review.md), following the
[prior-decisions-and-evidence-reuse rule](../../README.md#prior-decisions-and-evidence-reuse).
Carried forward the matrix acceptance at `7d1552a`, closed matrix-report
findings at `b87c1b9`, FR-1 correction at `59be2db` and independent acceptance
at `b28a608`. Their unchanged semantic claims were not re-audited.

## Executed results and coverage

The saved report identifies integration HEAD
`f2ff4c3b1a6249f995b7d2d96fdf61741def9d46`, Python 3.15.2, cwd, UTC times,
exact commands, actual exits and compact outputs. Its commands cover the
execution handoff:

| Required check | Actual result and review disposition |
|---|---|
| Integrated fragment structure and scope | Exit 0: five declarations, five variants, 23 obligations, exactly seven routing changes, all other JSON data unchanged, and valid work-package IDs. The changed-files assertion is correctly bounded to the two owned paths in `25329ad..59be2db`; current fragment/report bytes also match `59be2db`. |
| Executable inputs and TS pin | The combined command reached and passed all 71 SHA-256 comparisons against three retained snapshots, the tracked executable-tree comparison and the clean pinned TS checkout check before its later artifact assertion failed. These passing assertions remain evidence despite the command's final exit 1. |
| Original matrix preservation | The strict working-file-to-Git-blob assertion actually failed, exit 1. Separate subsequent exit-0 checks established unchanged committed blobs and the sole CRLF difference described below. |
| Scoped links | Exit 0: all 12 inputs then present, including all nine recovery handoffs, passed the existing `links(path)` helper; 16 local Markdown links were checked. The newly created review handoff was not a missing historical input. |
| Whitespace | `git diff --check` exited 0. The report correctly discloses that this does not check its then-untracked own contents; the independent direct check below confirms no trailing whitespace in the saved report. |

The checker API was independently inspected through CodeGraph. Its
`links(path)` function validates local targets and Markdown heading anchors;
importing the module does not execute its CLI. No Cargo, documentation-example
or parity rerun was justified by the unchanged executable inputs.

## Retained failure and preservation interpretation

The raw-byte failure is real and must remain in
[the saved report](resume-validation.md#executable-inputs-and-original-matrix-preservation):
64 of the 65 original working files equal their accepted Git blobs;
`matrix/commands.tsv` has 54 CRLF line endings while its accepted and current
Git blobs contain 54 LF endings. Its raw length is 145884 bytes versus
145830 committed bytes. The corresponding SHA-256 values are:

- Accepted/current committed blob:
  `b93378360d1f0d4320a4688069442e86556ff9fb3f874a48ef660a54c9485813`.
- Working file:
  `e26746857fb5c683ea657bdf75e7df99d7328d06b41a6429842b8bcc62d921ea`.

All 65 original committed blobs remain exactly equal to `7d1552a`. Replacing
CRLF with LF **in memory for comparison only** reproduces the accepted bytes
for every file, proving there is no other content difference in this TSV.
Independently, `git hash-object --path=<TSV path> <TSV path>` without `-w`
returns `43fc0c6a53403a9a669a60e3a12f79751c22f169`, equal to the accepted
blob ID. This checks Git's current clean conversion without writing an
object, index or artifact. The report's recorded `core.autocrlf=input` is
consistent with this result and with clean Git status despite raw CRLF bytes.
It does not establish when or why those working-file line endings originated.

For this handoff's evidence-preservation gate, exact committed artifact
identity plus an exhaustively isolated line-ending difference is sufficient.
The TSV command records, original logs and executable inputs retain their
accepted content. No missing behavioral check or damaged evidence has been
demonstrated. **Raw working-file identity to the accepted Git blob remains
false**, and this review does not claim it passed, change the report's FAIL
verdict, or authorize normalization of the artifact. No prior review is
superseded by a claim about the bytes in this different working checkout.

## Independent focused verification

Ran a read-only standard-library `python3 -B` check from
`/Users/I756434/Projects/private/cordis-rust` on branch
`feat/cordis-completion`, Python 3.15.2, at integration HEAD
`fb0c194c817bfc267ed7e339405880506df604e8`. Start UTC
`2026-11-02T12:54:15.149802+00:00`; end UTC
`2026-11-02T12:54:16.345329+00:00`; actual exit 0. It independently
confirmed:

- Five declarations / five variants / 23 obligations / the same seven
  routing-only changes; fragment and report match `59be2db`; 35 valid
  work-package IDs.
- All 71 current executable hashes agree with all three retained snapshots;
  no tracked executable-tree delta; TS remains clean at
  `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
- All 65 committed original artifacts are identical; the 64 raw matches and
  sole 54-ending TSV difference, hashes and filtered object ID above hold.
- The existing link helper passes 14 files / 22 local links: the original
  12 inputs plus the saved validation report and new review handoff.
- Direct trailing-whitespace inspection of the saved report and
  `git diff --check` both pass.

The saved report's reviewed SHA-256 is
`275be9d6e6523818b722c2010dcb09990d100d5000bbecbf56f50a5f8a0c49b7`.
These focused checks corroborate the recovered report at the later review
HEAD; they do not retroactively attribute its executed commands to that HEAD.
Only this review file was written. No source, accepted report, artifact,
handoff, status document, Git configuration or index was edited by this review;
no commit, agent dispatch or executable-suite rerun was performed.

## Acceptance boundary

No genuinely missing check remains for the bounded integrated structural,
input, artifact-preservation and link gate. Preserve both the failed raw-byte
assertion and the passing follow-up evidence when recording this acceptance.
The worker's final response disconnected after saving its report; the
recovered, independently checked file supplies the requested validation
record, not evidence of sustained runtime or transport recovery.

B0's 139 remaining declaration records, six type obligations, final ledger
merge/validator/independent review and every downstream implementation gate
remain pending. All seven fragment gaps remain open.

Review output: `docs/plans/2026-11-02-cordis-completion/evidence/B0/resume-validation-review.md`.
