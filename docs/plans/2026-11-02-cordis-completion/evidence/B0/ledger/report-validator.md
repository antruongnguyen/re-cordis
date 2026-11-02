# B0 ledger validator report

Date: 2026-11-02. Executor: `gpt-5.6-sol/high`, the explicitly authorized
fallback. Worktree baseline: `8c9a191bff8201265b17eb82e7b9a6e3a59c5ad2`.
This task owns only `ledger/validate.py` and this report. It did not edit the
ledger inventory, family fragments, matrix evidence, source, tests, manifests
or progress, and it did not rerun Cargo, documentation examples or parity.

## Implemented gate

The standard-library validator uses the committed `8c9a191` inventory as its
immutable identity, provenance, variant and original-obligation control. It
does not use the mutable final ledger as its own oracle. It enforces:

- 144 exact declaration identities and the 15 exact family counts; the eight
  pinned parameter sites and 156 exact variants; retained original obligation
  IDs and assertion lines while permitting additional uniquely named splits;
- exact source/checklist provenance, real source and assertion line bounds,
  derived parent classes, valid final statuses and class-specific evidence,
  restriction or owned-gap fields;
- the exact `event-types.json` shape and the six ordinary evidence records at
  `events.types.ts` lines 13, 14, 17, 19, 21 and 22, each linked to signature
  line 9 and its own source assertion line;
- real restriction anchors through the existing `tests/docs/check.py`
  `anchors()` helper; real current-core Rust test/Rustdoc files, symbols and
  line bounds; matching passing `results.json` identities and literal `... ok`
  log lines for both `stable-workspace-test` and `1.85.0-workspace-test`;
- exact paired scenario/observation/scope joins only for the six recorded
  comparison scenarios, with their actual parity log summaries;
- all 71 unchanged executable-input hashes, zero-exit matrix metadata, the
  fixed TypeScript pin `35fdc969e4b6c48c48ed44717c7a018324e4dd88`,
  and a clean ignored `local/cordis` checkout supplied via `--source-repo`; and
- no pending execution join, no inventory placeholder disposition, and an
  owned concrete next action for every gap. Every gap preserves the rule that
  a dependent gate plus a failing public-API reproducer is required before
  conditional B2 entry.

The validator reports declaration, variant, runtime obligation, event-type
obligation, unique Rust test/Rustdoc identity, paired-scenario and per-class
totals separately. It does not calculate a parity percentage.

## Bounded verification

From the isolated worktree root:

~~~text
$ python3 -B docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/validate.py --self-test --source-repo /Users/I756434/Projects/private/cordis-rust
exit 0
Self-test passed: accepted real joins, synthetic completion, and nine expected negative checks; counts={"classes": {"missing-evidence": 521, "rust-representation": 4, "source-mapped-test": 10, "unsupported": 3}, "declarations": 144, "obligations": 532, "paired_scenarios": 0, "rust_test_identities": 6, "type_obligations": 6, "variants": 156}
~~~

The positive fixture retains all pinned identities and overlays the accepted
five decorator/reflect dispositions, exercises a real passing Rustdoc identity
on both toolchains, and supplies explicit owned gaps for the otherwise
unfinished records. Negative checks reject the current incomplete inventory, a
missing declaration, an unknown declaration, a missing parameter variant, a
changed original obligation line, an unmatched Rust test result, a matched
result without an actual log line, a pending execution status and a missing
event-type record. Expected negative failures are asserted as failures; an
unexpected validator error fails the self-test.

The current ledger is intentionally not complete and there is no final
`ledger/event-types.json`, so the normal command is an expected negative:

~~~text
$ python3 -B docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/validate.py --source-repo /Users/I756434/Projects/private/cordis-rust
exit 1
Validation failed with 1106 error(s).
ERROR: ledger does not have a non-placeholder final status
...
~~~

This is not a failed acceptance run and is not evidence that the eventual full
ledger passes. It demonstrates rejection of the known inventory checkpoint.
No executable suite was rerun.

Final source-only checks are recorded after the report is written:

~~~text
$ python3 -B -c 'compile(open("docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/validate.py").read(), "validate.py", "exec")'
exit 0
$ PYTHONPATH=tests/docs python3 -B -c 'from pathlib import Path; import check; check.links(Path("docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/report-validator.md")); print("report link/anchor check passed")'
exit 0
report link/anchor check passed
$ git diff --check --cached
exit 0
~~~

## Remaining integration step

The integration owner must assemble the completed family records plus the
accepted decorator/reflect records into `ledger/cases.json`, place the six
completed type records at `ledger/event-types.json`, and then run the normal
validator command above with the integration checkout as `--source-repo`. Only
that future exit-zero result may be copied into `ledger/report.md` and offered
to the independent B0 ledger review. Any tracked gap continues to block only a
slice shown to depend on it; it does not open B2 without the conditional
dependent-gate reproducer.
