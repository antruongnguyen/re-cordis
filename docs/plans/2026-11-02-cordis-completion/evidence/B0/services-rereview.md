# B0 services corrections re-review

**Verdict: accepted with tracked evidence gaps.** SR-1–SR-5 are resolved by
accurate, bounded mappings and explicit follow-up gaps. No actionable unresolved
correction finding or correction-induced breakage was found. This accepts the
services inputs/report, not final shared-ledger assembly or B0 acceptance.

Reviewed at `b515447db380502be0e6168b50dbcea0a3c728a1` in
`/Users/I756434/.codex/worktrees/b0-services-rereview/cordis-rust`.
The three reviewed files are byte-identical to correction `e84cbbc` and its
integration `a1c6ef7`, based on `c462dfdf732bf2ed056eefb7832654efd9629e3a`.
The TypeScript specification remains
`35fdc969e4b6c48c48ed44717c7a018324e4dd88`.

Scope follows the [reuse rule](../../README.md#prior-decisions-and-evidence-reuse),
[progress](../../progress.md), [B0 Tasks 2–3](plan.md), and
[initial services review](services-review.md). Only the changed
[service input](ledger-inputs/service.json),
[service-dispose input](ledger-inputs/service-dispose.json), and
[report](ledger-inputs/report-services.md) were reassessed. The unchanged
semantic review, accepted matrix/report, FR-1, first-two events, six type
representations and integrated validation retain their prior scope and verdicts.

## Resolution of the five findings

- **SR-1 accepted.** Pinned `service.spec.ts:48–49,86–87` delegates to the
  reversible acquisition/cleanup in `utils.ts:62–66`. All increases use
  `root.foo`; the middle getter is consumer `ctx.foo.value` at 66 and root
  `root.foo.value` at 101. The corrected records state these distinct sites.
  [acceptance.rs](../../../../../crates/re-cordis-core/tests/acceptance.rs):652–667
  supplies three direct Cell mutations, final count 3 and no failures;
  [scoping.rs](../../../../../crates/re-cordis-core/tests/scoping.rs):188–200
  separately proves a bound effect survives its definition relay's disposal
  and belongs to root. Final-count and checked-access representations retain
  these limits. The four intermediate getter/effect obligations
  `:36/a61,a66` and `:76/a97,a101` now name the exact remaining regressions.
- **SR-2 accepted.** TS `:238/false` selects provider disposal at 261 and maps
  to Rust `owner_shutdown=true` at
  [service_dispose.rs](../../../../../crates/re-cordis-core/tests/service_dispose.rs):798.
  The gate/open/read-before-close assertions at 808–812 support that mapping;
  `gated_consumer` reads after its cleanup gate and `Probe.read` asserts open.
  Rust false/manual-effect disposal remains supplementary. TS true disposes
  a non-root outer owner containing the composed provider/consumer siblings;
  its `a266,a267` are now explicit gaps. Root-drain/inherited-child evidence is
  not promoted to that unexecuted arrangement.
- **SR-3 accepted.** At Rust 476–536, distinct versions and captured consumer
  reads support `:355/a380,a382`; `a381` now tracks the missing provider/current
  read before replacement setup is released. At Rust 545–610, both bindings
  use the same Rc(3), IDs differ, and the timeline uses local load ordinals.
  The corrected text retains withdrawal-before-final-disposal evidence for
  `:418/a439`, without treating that timeline as a value-selection oracle.
  `a440,a441` separately track post-convergence consumer value and actual
  setup/rollback/cleanup values under in-flight initialization. The Active-old-
  load scenario is not substituted for this initialization scenario.
- **SR-4 accepted.** Rust service cleanup at 451–467 establishes retained
  provider access, final listener delivery and one failure.
  [foundations.rs](../../../../../crates/re-cordis-core/tests/foundations.rs):251–258
  separately preserves a typed UserError field. Neither proves the diagnostic
  contains the service cleanup's original allocation. `:184/a195` now retains
  both partial witnesses and names an exact source/diagnostic identity
  regression. No explicit ErrorValue or apply-failure paired evidence is
  recast as this composition.
- **SR-5 accepted.** The replacement command actually compares variant
  parameters, original obligation IDs and assertion lines with the inventory,
  validates all used restriction anchors, invokes obligation schema checks,
  and joins exact test identities to results and log lines. Its printed
  outcomes match those checks. Source semantics were independently checked
  above; mechanical validation is not presented as semantic equivalence.

## Accounting and verification

The corrected inputs retain **25 declarations / 35 variants / 81 obligations**:
**55 source mappings, 11 Rust representations, 15 evidence gaps**. There are
**20 distinct Rust test identities / 40 passing result-and-log joins**, three
restriction anchors and zero paired claims. These are existing matrix joins,
not new Rust or TypeScript executions.

Twenty obligation records changed; the other 61 are object-identical to the
pre-correction inputs. All declaration metadata, parameters and retained IDs
are unchanged. The five original gaps (`service.spec.ts:7/a33`,
`:128/a161,a162,a163`, `:203/a214`) are object-identical, including their actions
and ownership. Ten new original-ID gaps retain partial witnesses while the
whole obligation remains `not-executed`. All 15 have owner/action/dependency
triage; B2 still requires a failing public-API reproducer and dependent gate.

The same four independently confirmed `:95/{provider,root,provider first,consumer
first}/a125` non-obligations remain omitted from behavioral totals. Their
guard/provenance disposition from the initial review is carried forward;
`restart/a125` remains behavioral. Final `not-applicable` metadata and
validator/control assembly belong to the later integration step.

The report's recorded command was run from this review worktree after proving
all three inputs equal the correction and integration commits. Only its two
ownership/whitespace diff endpoints were fixed to `e84cbbc`, because this
worktree also contains later unrelated integrations. No schema or evidence
assertion was relaxed. The reproducible invocation is:

```sh
python3 -B - <<'PY'
import re, subprocess
from pathlib import Path
base = Path('docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger-inputs')
for name in ('service.json', 'service-dispose.json', 'report-services.md'):
    path = base / name
    for revision in ('e84cbbc', 'a1c6ef7'):
        assert path.read_bytes() == subprocess.check_output(
            ['git', 'show', revision + ':' + str(path)])
block = re.search(r'~~~sh\n(.*?)\n~~~',
                  (base / 'report-services.md').read_text(), re.S).group(1)
for before, after in (
    ("git('diff', '--name-only', base_revision)",
     "git('diff', '--name-only', base_revision, 'e84cbbc')"),
    ('git diff c462dfdf732bf2ed056eefb7832654efd9629e3a --check',
     'git diff c462dfdf732bf2ed056eefb7832654efd9629e3a e84cbbc --check'),
):
    assert block.count(before) == 1
    block = block.replace(before, after)
subprocess.run(['zsh', '-c', block], check=True)
PY
```

Result: exit 0; all five report PASS lines reproduced, including exact inventory
provenance, 15 owned gaps, 20/40 joins, clean pinned TS, absent temporary fixture,
report links/whitespace, exactly three correction files and no executable delta.
The independent delta check also passed the 20/61 unchanged-record split and
five original-gap comparisons. Its first attempt rejected the intentionally
updated input status prose; excluding only that prose from the immutable
envelope comparison corrected the review helper, with no input edit.

After writing this review, its links and trailing whitespace and the staged
diff were checked. Only this review file is committed. No Cargo, documentation
example, TS or parity suite, full validator, or matrix re-audit was run.
