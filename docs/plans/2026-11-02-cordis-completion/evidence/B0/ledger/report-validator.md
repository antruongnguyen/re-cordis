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


## Coordinated final integration — supersedes the unfinished-data gate above

Date: 2026-11-02. Executor: fresh `gpt-6.1-sol/high`; isolated baseline
`a54d0452ef19afe28996e0e9d9ae95b6e702b824` at
`/Users/I756434/.codex/worktrees/b0-final-ledger-integration/cordis-rust`.
The earlier report and its failed normal run / nine-check synthetic self-test
are preserved above as historical evidence. They are not retroactively
complete-data passes. Current assembly and counts are in [report.md](report.md).

The validator now independently controls exactly seven reviewed
non-obligations against the immutable Git inventory and LR1/services
reason/review allowlist. Metadata must retain every field and its exact JSON
type; no class, gap or passing status can be added. Original active guarded
IDs stay behavioral, and original inventory IDs cannot disappear.
Summary and parent classes are regenerated from actual child records.
Runtime classes/statuses, types, original/new IDs, non-obligations and unique
Rust/paired identities are counted separately. Existing matrix result/log,
Rustdoc item/line/kind, source pin, 71-hash and restriction checks are retained.

The positive self-test now loads the real assembled cases/types. The obsolete
synthetic completion helpers were removed. Negative tests retain the original
nine gates and add missing/duplicate behavioral IDs, missing/unknown/duplicate
metadata IDs, altered source/line/context/text/variant/inventory/review/reason
provenance, false-versus-zero JSON typing, active guarded IDs falsely exempted,
metadata inserted into behavioral gaps, classes/gaps/passes on metadata,
inflated obligation/non-obligation/class/pass totals, and wrong Rustdoc kind.

### Actual attempts, including red checks

All invocations below ran from the isolated worktree root.

~~~sh
python3 -B docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/validate.py --self-test --source-repo /Users/I756434/Projects/private/cordis-rust
~~~

First red run, exit **1**: `AssertionError: real complete ledger rejected`.
The eight reported errors were the unknown inventory_non_obligations field
and seven original IDs no longer present in behavioral arrays. This exposed
the old validator's exact missing control before implementing it.
After implementing the control and summary, the command exited **0**, reporting
35 expected negative checks.

A later strict JSON provenance regression set inactive inject=false metadata
to integer zero. It failed at exit **1** with
`AssertionError: expected error containing 'non-obligation'; got []`.
Python dictionary equality treated false and zero alike. Canonical JSON
comparison now preserves the exact reviewed boolean variant. The retry exited
**0**, with actual output:

~~~text
Self-test passed: real complete-data positive, exact reviewed non-obligation control, and 36 expected negative checks; counts={"added_runtime_obligations":15,"classes":{"executed-comparison":24,"missing-evidence":147,"rust-representation":192,"source-mapped-test":180,"unsupported":3},"declarations":144,"inventory_non_obligations":7,"obligations":540,"original_runtime_inventory":532,"paired_scenarios":4,"retained_runtime_inventory":525,"runtime_classes":{"executed-comparison":24,"missing-evidence":147,"rust-representation":186,"source-mapped-test":180,"unsupported":3},"runtime_execution_statuses":{"not-applicable":3,"not-executed":147,"passed":390},"rust_test_identities":94,"type_classes":{"rust-representation":6},"type_obligations":6,"variants":156}
~~~

The normal complete-data invocation is distinct from the self-test:

~~~sh
python3 -B docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/validate.py --source-repo /Users/I756434/Projects/private/cordis-rust
~~~

Exit **0**, actual output:

~~~text
B0 ledger validation passed: {"added_runtime_obligations":15,"classes":{"executed-comparison":24,"missing-evidence":147,"rust-representation":192,"source-mapped-test":180,"unsupported":3},"declarations":144,"inventory_non_obligations":7,"obligations":540,"original_runtime_inventory":532,"paired_scenarios":4,"retained_runtime_inventory":525,"runtime_classes":{"executed-comparison":24,"missing-evidence":147,"rust-representation":186,"source-mapped-test":180,"unsupported":3},"runtime_execution_statuses":{"not-applicable":3,"not-executed":147,"passed":390},"rust_test_identities":94,"type_classes":{"rust-representation":6},"type_obligations":6,"variants":156}
~~~

An intermediate diagnostic invocation of validate_documents exited 0 as a
Python process while returning two errors (missing summary, summary mismatch).
It was a development diagnostic, not a validator pass. The completed normal
command above is the actual complete-data acceptance check.

The requested IDE post-edit recipe resource was unavailable
(`mcp-steroid://ide/verify-after-edit` returned resource-not-found); this was
a documentation lookup, not a check result. Real Python syntax, data, links
and whitespace commands are the verification evidence.

### Exact assembly and applicability check

This additional standard-library command compares every assembled record to
the accepted fragments and independently partitions original/new IDs. Before
claiming result/log joins it invokes the actual validator on the real dataset.

~~~sh
python3 -B - <<'PY'
import hashlib, json, subprocess
from collections import Counter
from pathlib import Path

b0 = Path("docs/plans/2026-11-02-cordis-completion/evidence/B0")
ledger = json.loads((b0 / "ledger/cases.json").read_text())
names = ("decorator-reflect", "events-first-two", "events-rest", "associate",
         "invoke", "intercept", "isolate", "logger", "shadow", "plugin",
         "fiber", "update", "dispose", "service", "service-dispose")
exempt = {r["id"] for r in ledger["inventory_non_obligations"]}
actual = {d["id"]: d for d in ledger["declarations"]}
seen = set()
raw = 0
for name in names:
    fragment = json.loads((b0 / f"ledger-inputs/{name}.json").read_text())
    for declaration in fragment["declarations"]:
        assert declaration["id"] not in seen
        seen.add(declaration["id"])
        raw += sum(len(v["obligations"]) for v in declaration["variants"])
        for variant in declaration["variants"]:
            variant["obligations"] = [o for o in variant["obligations"] if o["id"] not in exempt]
        declaration["classes"] = sorted({
            o["class"] for v in declaration["variants"] for o in v["obligations"]
        })
        assert actual[declaration["id"]] == declaration
assert seen == set(actual) and raw == 543
assert (b0 / "ledger/event-types.json").read_bytes() == (b0 / "ledger-inputs/event-types.json").read_bytes()
original = json.loads(subprocess.check_output([
    "git", "show", "8c9a191:" + str(b0 / "ledger/cases.json")
]))
original_ids = {o["id"] for d in original["declarations"]
                for v in d["variants"] for o in v["obligations"]}
runtime = [o for d in ledger["declarations"] for v in d["variants"] for o in v["obligations"]]
runtime_ids = {o["id"] for o in runtime}
assert original_ids == (runtime_ids & original_ids) | exempt
added = runtime_ids - original_ids
empty = {i for i in added if "/empty-" in i}
assert len(original_ids) == 532 and len(empty) == 4 and len(added - empty) == 11
print("PASS exact accepted assembly: 15 fragments, 144 declarations, 156 variants, raw=543, runtime=540, types=6")
print("PASS immutable original IDs: 532 = 525 behavioral + 7 non-obligations; additions=4 empty paths + 11 splits")
types = json.loads((b0 / "ledger/event-types.json").read_text())["obligations"]
import importlib.util
spec = importlib.util.spec_from_file_location("b0_validator", b0 / "ledger/validate.py")
validator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validator)
errors, counts = validator.validate_documents(
    ledger, json.loads((b0 / "ledger/event-types.json").read_text()),
    repo_root=Path.cwd(), source_repo=Path("/Users/I756434/Projects/private/cordis-rust"),
    baseline=original,
)
assert not errors, errors
refs = [e for o in runtime + types for e in o["rust_evidence"]]
identities = {(e["file"], e["symbol"], e.get("compile_fail", False)) for e in refs}
print(f"PASS preserved evidence: {len(refs)} Rust references, {2 * len(refs)} both-toolchain result/log joins, {len(identities)} identities")
gaps = [o for o in runtime if o["gap"]]
assert all(o["gap"]["owner"] == "core evidence maintainer" and o["gap"]["next_action"] for o in gaps)
assert all(all(t in o["gap"]["work_package"].casefold()
               for t in ("conditional b2", "depend", "failing public-api reproducer")) for o in gaps)
accepted = json.loads((b0 / "ledger-inputs/decorator-reflect.json").read_text())
preserved_gaps = {o["id"]: o for d in accepted["declarations"] for v in d["variants"] for o in v["obligations"] if o["gap"]}
assert len(preserved_gaps) == 7
assert all(next(o for o in runtime if o["id"] == oid) == record for oid, record in preserved_gaps.items())
print(f"PASS gap routing: {len(gaps)} genuine gaps; seven original decorator/reflect gaps unchanged; no B2 admission")
commands = json.loads((b0 / "matrix/commands.json").read_text())
results = json.loads((b0 / "matrix/results.json").read_text())
hashes = json.loads((b0 / "matrix/before.json").read_text())["executable_input_sha256"]
assert len(commands) == 53 and all(c["exit_code"] == 0 and (b0 / "matrix" / c["log"]).is_file() for c in commands)
assert hashes == json.loads((b0 / "matrix/after.json").read_text())["executable_input_sha256"]
assert len(hashes) == 71 and all(hashlib.sha256(Path(p).read_bytes()).hexdigest() == h for p, h in hashes.items())
assert len(results["paired_comparisons"]) == 6 and sum(results["paired_comparisons"].values()) == 73
source = Path("/Users/I756434/Projects/private/cordis-rust/local/cordis")
assert subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip() == ledger["typescript_revision"]
assert not subprocess.check_output(["git", "-C", str(source), "status", "--porcelain"], text=True).strip()
assert not (source / "packages/core/tests/re-cordis-lifecycle-parity.spec.ts").exists()
assert all(p.startswith(str(b0 / "ledger") + "/") for p in subprocess.check_output(["git", "diff", "--name-only", "a54d045"], text=True).splitlines())
print("PASS applicability: 53 zero-exit commands/logs, 71 unchanged hashes, clean TS pin, fixture absent, six/73 retained; ledger-only delta")
PY
~~~

Exit **0**, actual output:

~~~text
PASS exact accepted assembly: 15 fragments, 144 declarations, 156 variants, raw=543, runtime=540, types=6
PASS immutable original IDs: 532 = 525 behavioral + 7 non-obligations; additions=4 empty paths + 11 splits
PASS preserved evidence: 417 Rust references, 834 both-toolchain result/log joins, 94 identities
PASS gap routing: 147 genuine gaps; seven original decorator/reflect gaps unchanged; no B2 admission
PASS applicability: 53 zero-exit commands/logs, 71 unchanged hashes, clean TS pin, fixture absent, six/73 retained; ledger-only delta
~~~

The current reports' links/anchors and Python syntax are checked with:

~~~sh
python3 -B - <<'PY'
import importlib.util
from pathlib import Path
base = Path("docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger")
compile((base / "validate.py").read_text(), "validate.py", "exec")
spec = importlib.util.spec_from_file_location("docs_check", Path("tests/docs/check.py"))
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)
for name in ("report.md", "report-validator.md"):
    check.links(base / name)
print("PASS Python syntax and both report links/anchors")
PY
git diff --check
~~~

Actual results: Python exited **0** with `PASS Python syntax and both report links/anchors`;
git diff --check exited **0** with no whitespace findings.
Final independent Astra acceptance must inspect both validator code and these
real-data/negative results. No source or suite execution was changed, no
subagent was spawned, and no push was made.
