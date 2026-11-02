#!/usr/bin/env python3
"""Validate the completed B0 declaration and event-type evidence ledger."""

import argparse
from collections import Counter
from copy import deepcopy
from functools import lru_cache
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess


ROOT = Path(__file__).resolve().parents[6]
B0 = Path(__file__).resolve().parents[1]
BASELINE_COMMIT = "8c9a191bff8201265b17eb82e7b9a6e3a59c5ad2"
BASELINE_LEDGER = "docs/plans/2026-11-02-cordis-completion/evidence/B0/ledger/cases.json"
TS_PIN = "35fdc969e4b6c48c48ed44717c7a018324e4dd88"
TYPE_LINES = (13, 14, 17, 19, 21, 22)
TYPE_SOURCE = "local/cordis/packages/core/tests/events.types.ts"
FAMILY_COUNTS = {
    "associate.spec.ts": 6, "decorator.spec.ts": 1, "dispose.spec.ts": 13,
    "events.spec.ts": 19, "fiber.spec.ts": 22, "intercept.spec.ts": 7,
    "invoke.spec.ts": 2, "isolate.spec.ts": 6, "logger.spec.ts": 11,
    "plugin.spec.ts": 10, "reflect.spec.ts": 4,
    "service-dispose.spec.ts": 17, "service.spec.ts": 8,
    "shadow.spec.ts": 8, "update.spec.ts": 10,
}
CLASSES = {
    "executed-comparison", "source-mapped-test", "rust-representation",
    "unsupported", "missing-evidence",
}
STATUSES = {"passed", "failed", "not-executed", "not-applicable"}
GAP_KINDS = {"missing-evidence", "observed-source-gap", "investigation", "confirmed-failure"}
REQUIRED_TOOLCHAINS = {"stable-workspace-test", "1.85.0-workspace-test"}
SCENARIOS = {
    "combined", "coalesced", "removed", "superseded-failure",
    "latest-failure", "explicit-recovery",
}


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def git_output(repo_root, *args):
    result = subprocess.run(
        ["git", "-C", str(repo_root), *args], text=True, capture_output=True
    )
    if result.returncode:
        raise ValueError(result.stderr.strip() or result.stdout.strip())
    return result.stdout


def load_baseline(repo_root):
    raw = git_output(repo_root, "show", f"{BASELINE_COMMIT}:{BASELINE_LEDGER}")
    return json.loads(raw)


def variant_key(parameters):
    return json.dumps(parameters, sort_keys=True, separators=(",", ":"))


def final_text(value):
    if not isinstance(value, str) or not value.strip():
        return False
    lowered = value.casefold()
    return not any(token in lowered for token in (
        "initial inventory", "placeholder", "pending-matrix",
        "in progress", "under review", "tbd", "todo",
    ))


def check_keys(value, expected, label, errors, optional=()):
    actual = set(value) if isinstance(value, dict) else set()
    required, allowed = set(expected), set(expected) | set(optional)
    if missing := required - actual:
        errors.append(f"{label}: missing fields {sorted(missing)}")
    if unknown := actual - allowed:
        errors.append(f"{label}: unknown fields {sorted(unknown)}")


def resolve_reference(path_text, repo_root, source_repo):
    base = source_repo if path_text.startswith("local/cordis/") else repo_root
    return (base / path_text).resolve()


def line_count(path):
    return len(path.read_text(encoding="utf-8").splitlines())


@lru_cache(maxsize=1)
def docs_anchor_helper():
    path = ROOT / "tests/docs/check.py"
    spec = importlib.util.spec_from_file_location("cordis_docs_check", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.anchors


@lru_cache(maxsize=8)
def evidence_context(repo_text, source_text):
    repo_root, source_repo = Path(repo_text), Path(source_text)
    errors = []
    matrix = B0 / "matrix"
    try:
        before = load_json(matrix / "before.json")
        after = load_json(matrix / "after.json")
        results = load_json(matrix / "results.json")
        commands = load_json(matrix / "commands.json")
    except (OSError, ValueError, json.JSONDecodeError) as error:
        return [f"matrix artifacts unreadable: {error}"], {}

    hashes = before.get("executable_input_sha256", {})
    if len(hashes) != 71:
        errors.append(f"matrix must record 71 executable hashes, found {len(hashes)}")
    if hashes != after.get("executable_input_sha256"):
        errors.append("before/after executable hash inventories differ")
    if results.get("input_hash_count") != 71:
        errors.append("matrix results do not report 71 input hashes")
    for relative, expected in hashes.items():
        path = repo_root / relative
        actual = hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
        if actual != expected:
            errors.append(f"matrix input hash mismatch: {relative}")
    for snapshot_name, snapshot in (("before", before), ("after", after)):
        if snapshot.get("ts_head") != TS_PIN:
            errors.append(f"{snapshot_name} matrix TypeScript pin is not {TS_PIN}")
        if snapshot.get("ts_full_status") or snapshot.get("ts_tracked_status"):
            errors.append(f"{snapshot_name} matrix TypeScript checkout was not clean")
    if not results.get("commands_all_exit_zero"):
        errors.append("matrix results do not record all indexed commands as zero-exit")

    command_map = {command.get("id"): command for command in commands}
    logs = {}
    for command_id, command in command_map.items():
        if command.get("exit_code") != 0:
            errors.append(f"matrix command {command_id} did not exit zero")
        log = matrix / command.get("log", "")
        if not log.is_file():
            errors.append(f"matrix command {command_id} has no real log file")
        else:
            logs[command_id] = log.read_text(encoding="utf-8")

    upstream = source_repo / "local/cordis"
    try:
        if git_output(upstream, "rev-parse", "HEAD").strip() != TS_PIN:
            errors.append(f"source checkout is not pinned at {TS_PIN}")
        if git_output(upstream, "status", "--porcelain").strip():
            errors.append("source checkout local/cordis is not clean")
    except ValueError as error:
        errors.append(f"source checkout local/cordis is unavailable: {error}")

    return errors, {
        "results": results, "commands": command_map, "logs": logs,
        "paired": results.get("paired_comparisons", {}),
    }


def check_reference(path_text, lines, label, errors, repo_root, source_repo):
    if not isinstance(path_text, str) or not path_text:
        errors.append(f"{label}: missing file reference")
        return None
    path = resolve_reference(path_text, repo_root, source_repo)
    allowed_root = source_repo.resolve() if path_text.startswith("local/cordis/") else repo_root.resolve()
    if not path.is_relative_to(allowed_root) or not path.is_file():
        errors.append(f"{label}: referenced file does not exist: {path_text}")
        return None
    if not isinstance(lines, list) or not lines or any(type(line) is not int for line in lines):
        errors.append(f"{label}: line reference must be a non-empty integer list")
        return path
    maximum = line_count(path)
    if any(line < 1 or line > maximum for line in lines):
        errors.append(f"{label}: line reference is outside {path_text} (1..{maximum})")
    return path


def validate_restriction(restriction, label, errors, repo_root):
    if not isinstance(restriction, dict):
        errors.append(f"{label}: restriction must be an object")
        return
    check_keys(restriction, ("document", "anchor", "reason"), label + " restriction", errors)
    document = restriction.get("document")
    anchor = restriction.get("anchor")
    reason = restriction.get("reason")
    path = resolve_reference(document, repo_root, repo_root) if isinstance(document, str) else None
    if path is None or not path.is_relative_to(repo_root.resolve()) or not path.is_file():
        errors.append(f"{label}: restriction document does not exist: {document!r}")
    elif not isinstance(anchor, str) or anchor not in docs_anchor_helper()(path.read_text(encoding="utf-8")):
        errors.append(f"{label}: restriction anchor does not exist: {document}#{anchor}")
    if not final_text(reason):
        errors.append(f"{label}: restriction reason is missing or placeholder text")


def validate_gap(gap, label, errors, gap_ids):
    if not isinstance(gap, dict):
        errors.append(f"{label}: gap must be an object")
        return
    check_keys(gap, ("id", "kind", "next_action", "owner", "work_package"), label + " gap", errors)
    gap_id = gap.get("id")
    if not final_text(gap_id):
        errors.append(f"{label}: gap ID is missing or placeholder text")
    elif gap_id in gap_ids:
        errors.append(f"{label}: duplicate gap ID {gap_id}")
    else:
        gap_ids.add(gap_id)
    if gap.get("kind") not in GAP_KINDS:
        errors.append(f"{label}: invalid gap kind {gap.get('kind')!r}")
    for field in ("next_action", "owner", "work_package"):
        if not final_text(gap.get(field)):
            errors.append(f"{label}: gap {field} is missing or placeholder text")
    work_package = gap.get("work_package", "")
    lowered = work_package.casefold() if isinstance(work_package, str) else ""
    if not all(token in lowered for token in ("conditional b2", "depend", "failing public-api reproducer")):
        errors.append(f"{label}: gap does not preserve conditional B2 dependent-gate entry")


def rustdoc_result_matches(row, evidence):
    test = row.get("test", "")
    prefix = f"{evidence.get('file')} - {evidence.get('symbol')} (line "
    if row.get("suite") != "rustdoc" or not test.startswith(prefix):
        return False
    match = re.search(r"\(line (\d+)\)", test)
    cited = [evidence["rustdoc_line"]] if type(evidence.get("rustdoc_line")) is int else evidence.get("lines", [])
    if not match or int(match.group(1)) not in cited:
        return False
    return "compile_fail" not in evidence or row.get("compile_fail") is evidence["compile_fail"]


def validate_rust_evidence(evidence, label, errors, context, repo_root, source_repo, rust_identities):
    if not isinstance(evidence, dict):
        errors.append(f"{label}: Rust evidence must be an object")
        return
    check_keys(
        evidence, ("file", "symbol", "lines", "assertion", "command_ids"),
        label, errors, optional=("rustdoc_line", "compile_fail"),
    )
    file = evidence.get("file")
    symbol = evidence.get("symbol")
    lines = evidence.get("lines")
    path = check_reference(file, lines, label, errors, repo_root, source_repo)
    if not isinstance(file, str) or not file.startswith("crates/re-cordis-core/"):
        errors.append(f"{label}: Rust evidence must cite current core production/tests, not {file!r}")
    if not final_text(symbol) or not final_text(evidence.get("assertion")):
        errors.append(f"{label}: Rust symbol/assertion is missing or placeholder text")
    command_ids = evidence.get("command_ids")
    if not isinstance(command_ids, list) or not REQUIRED_TOOLCHAINS.issubset(command_ids):
        errors.append(f"{label}: Rust evidence must cite both workspace toolchain command IDs")
        return
    if path and "/tests/" in file and isinstance(symbol, str):
        if not re.search(rf"\b(?:async\s+)?fn\s+{re.escape(symbol)}\b", path.read_text(encoding="utf-8")):
            errors.append(f"{label}: Rust test symbol is absent from {file}: {symbol}")
    suite = file.split("crates/re-cordis-core/", 1)[-1] if isinstance(file, str) else ""
    for command_id in sorted(REQUIRED_TOOLCHAINS):
        rows = [
            row for row in context.get("results", {}).get("tests", [])
            if row.get("command_id") == command_id and row.get("outcome") == "ok"
        ]
        if suite.startswith("tests/"):
            matches = [row for row in rows if row.get("suite") == suite and row.get("test") == symbol]
        else:
            matches = [row for row in rows if rustdoc_result_matches(row, evidence)]
        if len(matches) != 1:
            errors.append(f"{label}: no unique matching passing result for {command_id}")
            continue
        row = matches[0]
        expected_log_line = f"test {row['test']} ... ok"
        if expected_log_line not in context.get("logs", {}).get(command_id, ""):
            errors.append(f"{label}: matching result has no actual log line for {command_id}")
        rust_identities.add((file, symbol, row.get("compile_fail", False)))


def validate_paired_evidence(evidence, label, errors, context, repo_root, paired_scenarios):
    if not isinstance(evidence, dict):
        errors.append(f"{label}: paired evidence must be an object")
        return
    check_keys(evidence, ("scenario", "observation", "scope"), label, errors)
    scenario = evidence.get("scenario")
    observation = evidence.get("observation")
    if scenario not in SCENARIOS:
        errors.append(f"{label}: unknown paired scenario {scenario!r}")
        return
    count = context.get("paired", {}).get(scenario)
    if type(observation) is int:
        valid_observation = 1 <= observation <= count
    elif isinstance(observation, list):
        valid_observation = bool(observation) and all(type(item) is int and 1 <= item <= count for item in observation)
    elif isinstance(observation, str):
        trace = repo_root / f"tests/parity/{scenario}.txt"
        valid_observation = bool(observation.strip()) and trace.is_file() and observation in trace.read_text(encoding="utf-8")
    else:
        valid_observation = False
    if not valid_observation:
        errors.append(f"{label}: paired observation does not identify a real {scenario} observation")
    if not final_text(evidence.get("scope")):
        errors.append(f"{label}: paired evidence scope is missing or placeholder text")
    parity_log = context.get("logs", {}).get("parity", "")
    if f"{scenario}: Rust == TypeScript ({count} observations)" not in parity_log:
        errors.append(f"{label}: paired scenario has no matching parity log line")
    paired_scenarios.add(scenario)


def validate_obligation(obligation, label, errors, context, repo_root, source_repo,
                        source_bounds, obligation_ids, gap_ids, rust_identities, paired_scenarios):
    if not isinstance(obligation, dict):
        errors.append(f"{label}: obligation must be an object")
        return
    check_keys(
        obligation, ("id", "behavior", "source_assertion_lines", "class",
                     "rust_evidence", "paired_evidence", "restriction", "gap",
                     "execution_status", "rationale"),
        label, errors, optional=("source_file", "source_line", "source_signature_line"),
    )
    obligation_id = obligation.get("id")
    if not final_text(obligation_id):
        errors.append(f"{label}: obligation ID is missing or placeholder text")
    elif obligation_id in obligation_ids:
        errors.append(f"{label}: duplicate obligation ID {obligation_id}")
    else:
        obligation_ids.add(obligation_id)
    if not final_text(obligation.get("behavior")):
        errors.append(f"{label}: behavior is missing or placeholder text")
    lines = obligation.get("source_assertion_lines")
    if not isinstance(lines, list) or not lines or any(type(line) is not int for line in lines):
        errors.append(f"{label}: source assertion lines must be non-empty integers")
    elif any(line < source_bounds[0] or line > source_bounds[1] for line in lines):
        errors.append(f"{label}: source assertion line falls outside its declaration/expression")
    evidence_class = obligation.get("class")
    if evidence_class not in CLASSES:
        errors.append(f"{label}: invalid evidence class {evidence_class!r}")
    execution_status = obligation.get("execution_status")
    if execution_status == "pending-matrix" or (isinstance(execution_status, str) and "pending" in execution_status):
        errors.append(f"{label}: pending execution join remains")
    elif execution_status not in STATUSES:
        errors.append(f"{label}: invalid execution status {execution_status!r}")
    if not final_text(obligation.get("rationale")):
        errors.append(f"{label}: rationale is missing or placeholder text")

    rust = obligation.get("rust_evidence")
    paired = obligation.get("paired_evidence")
    restriction = obligation.get("restriction")
    gap = obligation.get("gap")
    if not isinstance(rust, list) or not isinstance(paired, list):
        errors.append(f"{label}: Rust and paired evidence fields must be lists")
        rust, paired = [], []
    if evidence_class == "executed-comparison":
        if not paired or execution_status != "passed" or gap is not None:
            errors.append(f"{label}: executed comparison requires passed paired evidence and no gap")
    elif paired:
        errors.append(f"{label}: paired evidence must use executed-comparison class")
    if evidence_class in {"source-mapped-test", "rust-representation"} and (not rust or execution_status != "passed" or gap is not None):
        errors.append(f"{label}: {evidence_class} requires passed Rust evidence and no gap")
    if evidence_class == "rust-representation" and restriction is None:
        errors.append(f"{label}: Rust representation requires a documented restriction")
    if evidence_class == "unsupported" and (restriction is None or gap is not None or execution_status != "not-applicable"):
        errors.append(f"{label}: unsupported requires restriction, not-applicable, and no gap")
    if evidence_class == "missing-evidence" and (gap is None or execution_status not in {"not-executed", "failed"}):
        errors.append(f"{label}: missing evidence requires an owned gap and non-passing status")
    if evidence_class != "missing-evidence" and gap is not None:
        errors.append(f"{label}: only missing-evidence may retain a gap")

    if restriction is not None:
        validate_restriction(restriction, label, errors, repo_root)
    if gap is not None:
        validate_gap(gap, label, errors, gap_ids)
    for index, item in enumerate(rust):
        validate_rust_evidence(
            item, f"{label}.rust_evidence[{index}]", errors, context,
            repo_root, source_repo, rust_identities,
        )
    for index, item in enumerate(paired):
        validate_paired_evidence(
            item, f"{label}.paired_evidence[{index}]", errors, context,
            repo_root, paired_scenarios,
        )


def validate_documents(ledger, event_types, *, repo_root, source_repo, baseline):
    """Return (errors, derived counts) for supplied ledger documents."""
    errors, context = evidence_context(str(repo_root.resolve()), str(source_repo.resolve()))
    errors = list(errors)
    obligation_ids, gap_ids, rust_identities, paired_scenarios = set(), set(), set(), set()
    class_counts = Counter()

    check_keys(
        ledger, ("schema_version", "rust_revision", "typescript_revision",
                 "implementation_revision", "status", "declarations"),
        "ledger", errors,
    )
    if ledger.get("schema_version") != 1:
        errors.append("ledger schema_version must be 1")
    for field in ("rust_revision", "implementation_revision"):
        if ledger.get(field) != baseline.get(field):
            errors.append(f"ledger {field} differs from the pinned baseline control")
    if ledger.get("typescript_revision") != TS_PIN:
        errors.append(f"ledger typescript_revision must be {TS_PIN}")
    if not final_text(ledger.get("status")):
        errors.append("ledger does not have a non-placeholder final status")

    declarations = ledger.get("declarations")
    if not isinstance(declarations, list):
        return errors + ["ledger declarations must be a list"], {}
    actual_by_id = {item.get("id"): item for item in declarations if isinstance(item, dict)}
    expected_by_id = {item["id"]: item for item in baseline["declarations"]}
    for missing in sorted(expected_by_id.keys() - actual_by_id.keys()):
        errors.append(f"missing declaration: {missing}")
    for unknown in sorted(actual_by_id.keys() - expected_by_id.keys(), key=str):
        errors.append(f"unknown declaration: {unknown}")
    if len(actual_by_id) != len(declarations):
        errors.append("declaration IDs are missing or duplicated")
    family_counts = Counter(Path(item.get("source_file", "")).name for item in declarations if isinstance(item, dict))
    if dict(family_counts) != FAMILY_COUNTS:
        errors.append(f"family declaration counts differ: {dict(sorted(family_counts.items()))}")

    for declaration_id in sorted(expected_by_id.keys() & actual_by_id.keys()):
        expected, actual = expected_by_id[declaration_id], actual_by_id[declaration_id]
        label = f"declaration {declaration_id}"
        for field in ("source_file", "source_line", "source_end_line", "title", "suite", "historical_checklist"):
            if actual.get(field) != expected.get(field):
                errors.append(f"{label}: {field} differs from pinned checklist/source provenance")
        source = check_reference(
            actual.get("source_file"), [actual.get("source_line"), actual.get("source_end_line")],
            label, errors, repo_root, source_repo,
        )
        checklist = actual.get("historical_checklist", {})
        if not isinstance(checklist, dict) or checklist.get("checked") is not True:
            errors.append(f"{label}: historical checklist provenance is not checked")
        else:
            check_reference(checklist.get("file"), [1], label + " checklist", errors, repo_root, source_repo)
        expected_variants = {variant_key(item["parameters"]): item for item in expected["variants"]}
        variants = actual.get("variants")
        if not isinstance(variants, list):
            errors.append(f"{label}: variants must be a list")
            continue
        actual_variants = {variant_key(item.get("parameters")): item for item in variants if isinstance(item, dict)}
        if set(actual_variants) != set(expected_variants) or len(actual_variants) != len(variants):
            errors.append(f"{label}: variant set differs from pinned baseline control")
        derived_classes = set()
        for key in sorted(expected_variants.keys() & actual_variants.keys()):
            expected_variant, variant = expected_variants[key], actual_variants[key]
            variant_label = f"{label} variant {key}"
            obligations = variant.get("obligations")
            if not isinstance(obligations, list) or not obligations:
                errors.append(f"{variant_label}: must contain at least one obligation")
                continue
            original = {item["id"]: item["source_assertion_lines"] for item in expected_variant["obligations"]}
            retained = {item.get("id"): item.get("source_assertion_lines") for item in obligations if isinstance(item, dict)}
            for original_id, original_lines in original.items():
                if retained.get(original_id) != original_lines:
                    errors.append(f"{variant_label}: original obligation/lines not retained: {original_id}")
            for index, obligation in enumerate(obligations):
                obligation_label = f"{variant_label}.obligations[{index}]"
                validate_obligation(
                    obligation, obligation_label, errors, context, repo_root, source_repo,
                    (actual.get("source_line", 0), actual.get("source_end_line", 0)),
                    obligation_ids, gap_ids, rust_identities, paired_scenarios,
                )
                if isinstance(obligation, dict) and obligation.get("class") in CLASSES:
                    derived_classes.add(obligation["class"])
                    class_counts[obligation["class"]] += 1
        if actual.get("classes") != sorted(derived_classes):
            errors.append(f"{label}: parent classes do not equal derived obligation classes")

    expected_type_ids = {f"core/events.types.ts:{line}" for line in TYPE_LINES}
    if not isinstance(event_types, dict):
        errors.append("event-types ledger is missing")
        type_obligations = []
    else:
        check_keys(
            event_types, ("schema_version", "rust_revision", "typescript_revision",
                          "status", "obligations"),
            "event-types ledger", errors,
        )
        if event_types.get("schema_version") != 1:
            errors.append("event-types schema_version must be 1")
        if event_types.get("rust_revision") != baseline.get("rust_revision"):
            errors.append("event-types rust_revision differs from pinned baseline")
        if event_types.get("typescript_revision") != TS_PIN:
            errors.append(f"event-types typescript_revision must be {TS_PIN}")
        if not final_text(event_types.get("status")):
            errors.append("event-types ledger does not have a non-placeholder final status")
        type_obligations = event_types.get("obligations")
        if not isinstance(type_obligations, list):
            errors.append("event-types obligations must be a list")
            type_obligations = []
    actual_type_ids = {item.get("id") for item in type_obligations if isinstance(item, dict)}
    if actual_type_ids != expected_type_ids or len(actual_type_ids) != len(type_obligations):
        errors.append("event-type obligation IDs must be exactly lines 13, 14, 17, 19, 21, 22")
    for index, obligation in enumerate(type_obligations):
        label = f"event-types.obligations[{index}]"
        line = obligation.get("source_line") if isinstance(obligation, dict) else None
        expected_id = f"core/events.types.ts:{line}"
        if obligation.get("id") != expected_id or line not in TYPE_LINES:
            errors.append(f"{label}: event-type ID/source line mismatch")
        if obligation.get("source_file") != TYPE_SOURCE:
            errors.append(f"{label}: event-type source file must be {TYPE_SOURCE}")
        if obligation.get("source_signature_line") != 9:
            errors.append(f"{label}: event-type signature line must be 9")
        if obligation.get("source_assertion_lines") != [line]:
            errors.append(f"{label}: event-type assertion lines must equal its source line")
        check_reference(TYPE_SOURCE, [9, line], label, errors, repo_root, source_repo)
        validate_obligation(
            obligation, label, errors, context, repo_root, source_repo, (line or 0, line or 0),
            obligation_ids, gap_ids, rust_identities, paired_scenarios,
        )
        if isinstance(obligation, dict) and obligation.get("class") == "executed-comparison":
            errors.append(f"{label}: separate events.types.ts was not run as an executed comparison")
        if isinstance(obligation, dict) and obligation.get("class") in CLASSES:
            class_counts[obligation["class"]] += 1

    counts = {
        "declarations": len(declarations),
        "variants": sum(len(item.get("variants", [])) for item in declarations if isinstance(item, dict)),
        "obligations": sum(
            len(variant.get("obligations", []))
            for item in declarations if isinstance(item, dict)
            for variant in item.get("variants", []) if isinstance(variant, dict)
        ),
        "type_obligations": len(type_obligations),
        "rust_test_identities": len(rust_identities),
        "paired_scenarios": len(paired_scenarios),
        "classes": dict(sorted(class_counts.items())),
    }
    if counts["declarations"] != 144:
        errors.append(f"derived declaration count is {counts['declarations']}, expected 144")
    if counts["variants"] != 156:
        errors.append(f"derived variant count is {counts['variants']}, expected 156")
    if counts["type_obligations"] != 6:
        errors.append(f"derived event-type obligation count is {counts['type_obligations']}, expected 6")
    return errors, counts


def complete_fixture(baseline):
    fixture = deepcopy(baseline)
    fixture["status"] = "complete evidence reconciliation"
    for declaration in fixture["declarations"]:
        classes = set()
        for variant in declaration["variants"]:
            for obligation in variant["obligations"]:
                obligation.update({
                    "class": "missing-evidence",
                    "rust_evidence": [],
                    "paired_evidence": [],
                    "restriction": None,
                    "gap": {
                        "id": "G-SELF-" + obligation["id"],
                        "kind": "missing-evidence",
                        "next_action": "Add and run a targeted public core regression for this exact assertion boundary.",
                        "owner": "core evidence maintainer",
                        "work_package": (
                            "Core evidence follow-up; a dependent gate and failing public-API "
                            "reproducer are required before conditional B2 entry."
                        ),
                    },
                    "execution_status": "not-executed",
                    "rationale": (
                        "No exact current Rust assertion is available; the boundary remains "
                        "explicitly tracked for a targeted evidence regression."
                    ),
                })
                classes.add(obligation["class"])
        declaration["classes"] = sorted(classes)
    return fixture


def complete_type_fixture(baseline):
    return {
        "schema_version": 1,
        "rust_revision": baseline["rust_revision"],
        "typescript_revision": baseline["typescript_revision"],
        "status": "complete event-type evidence reconciliation",
        "obligations": [
            {
                "id": f"core/events.types.ts:{line}",
                "source_file": TYPE_SOURCE,
                "source_line": line,
                "source_signature_line": 9,
                "source_assertion_lines": [line],
                "behavior": f"Typed event expression at line {line} remains explicitly accounted for.",
                "class": "missing-evidence",
                "rust_evidence": [],
                "paired_evidence": [],
                "restriction": None,
                "gap": {
                    "id": f"G-SELF-events.types.ts:{line}",
                    "kind": "missing-evidence",
                    "next_action": "Add and run a targeted Rustdoc check for this exact type expression.",
                    "owner": "core evidence maintainer",
                    "work_package": (
                        "Core evidence follow-up; a dependent gate and failing public-API "
                        "reproducer are required before conditional B2 entry."
                    ),
                },
                "execution_status": "not-executed",
                "rationale": "The type expression is retained as an explicit, owned evidence gap.",
            }
            for line in TYPE_LINES
        ],
    }


def assert_error(errors, text):
    if not any(text in error for error in errors):
        raise AssertionError(f"expected error containing {text!r}; got {errors!r}")


def self_test(repo_root, source_repo, baseline):
    ledger = complete_fixture(baseline)
    accepted = load_json(B0 / "ledger-inputs/decorator-reflect.json")
    accepted_by_id = {item["id"]: item for item in accepted["declarations"]}
    ledger["declarations"] = [
        deepcopy(accepted_by_id.get(item["id"], item)) for item in ledger["declarations"]
    ]
    event_types = complete_type_fixture(baseline)
    event_types["obligations"][0].update({
        "class": "source-mapped-test",
        "rust_evidence": [{
            "file": "crates/re-cordis-core/src/events.rs",
            "symbol": "events::Event",
            "lines": [62],
            "compile_fail": False,
            "assertion": "Runnable typed Event Rustdoc item passed.",
            "command_ids": ["stable-workspace-test", "1.85.0-workspace-test"],
        }],
        "gap": None,
        "execution_status": "passed",
        "rationale": "Synthetic positive check for the real runnable Rustdoc matrix/log join.",
    })
    errors, positive_counts = validate_documents(
        ledger, event_types, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    if errors:
        raise AssertionError(f"complete synthetic fixture rejected: {errors}")

    errors, _ = validate_documents(
        baseline, None, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    assert_error(errors, "final status")

    changed = deepcopy(ledger)
    changed["declarations"].pop()
    errors, _ = validate_documents(
        changed, event_types, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    assert_error(errors, "missing declaration")

    changed = deepcopy(ledger)
    changed["declarations"][0]["id"] = "core/unknown.spec.ts:1"
    errors, _ = validate_documents(
        changed, event_types, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    assert_error(errors, "unknown declaration")

    changed = deepcopy(ledger)
    parameterized = next(item for item in changed["declarations"] if len(item["variants"]) > 1)
    parameterized["variants"].pop()
    errors, _ = validate_documents(
        changed, event_types, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    assert_error(errors, "variant set")

    changed = deepcopy(ledger)
    obligation = changed["declarations"][0]["variants"][0]["obligations"][0]
    obligation["source_assertion_lines"] = [obligation["source_assertion_lines"][0] + 1]
    errors, _ = validate_documents(
        changed, event_types, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    assert_error(errors, "original obligation/lines not retained")

    changed = deepcopy(ledger)
    obligation = changed["declarations"][0]["variants"][0]["obligations"][0]
    obligation.update({
        "class": "source-mapped-test",
        "rust_evidence": [{
            "file": "crates/re-cordis-core/tests/acceptance.rs",
            "symbol": "not_a_real_test",
            "lines": [1],
            "assertion": "Synthetic unmatched test identity.",
            "command_ids": ["stable-workspace-test", "1.85.0-workspace-test"],
        }],
        "gap": None,
        "execution_status": "passed",
        "rationale": "Synthetic unmatched join.",
    })
    changed["declarations"][0]["classes"] = ["source-mapped-test"]
    errors, _ = validate_documents(
        changed, event_types, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    assert_error(errors, "matching passing result")

    real_evidence = next(
        item
        for declaration in accepted["declarations"]
        for variant in declaration["variants"]
        for obligation in variant["obligations"]
        for item in obligation["rust_evidence"]
        if "/tests/" in item["file"]
    )
    _, context = evidence_context(str(repo_root.resolve()), str(source_repo.resolve()))
    broken_context = dict(context)
    broken_context["logs"] = dict(context["logs"])
    broken_context["logs"]["stable-workspace-test"] = ""
    errors = []
    validate_rust_evidence(
        real_evidence, "synthetic missing log", errors, broken_context, repo_root, source_repo, set()
    )
    assert_error(errors, "no actual log line")

    changed = deepcopy(ledger)
    changed["declarations"][0]["variants"][0]["obligations"][0]["execution_status"] = "pending-matrix"
    errors, _ = validate_documents(
        changed, event_types, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    assert_error(errors, "pending execution")

    changed_types = deepcopy(event_types)
    changed_types["obligations"].pop()
    errors, _ = validate_documents(
        ledger, changed_types, repo_root=repo_root, source_repo=source_repo, baseline=baseline
    )
    assert_error(errors, "event-type obligation IDs")

    print(
        "Self-test passed: accepted real joins, synthetic completion, and nine "
        f"expected negative checks; counts={json.dumps(positive_counts, sort_keys=True)}"
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ledger", type=Path, default=B0 / "ledger/cases.json")
    parser.add_argument("--event-types", type=Path, default=B0 / "ledger/event-types.json")
    parser.add_argument("--source-repo", type=Path, default=ROOT)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    try:
        baseline = load_baseline(ROOT)
    except (ValueError, json.JSONDecodeError) as error:
        parser.error(f"cannot load pinned baseline {BASELINE_COMMIT}: {error}")
    if args.self_test:
        self_test(ROOT, args.source_repo.resolve(), baseline)
        return
    event_types = load_json(args.event_types) if args.event_types.is_file() else None
    errors, counts = validate_documents(
        load_json(args.ledger), event_types, repo_root=ROOT,
        source_repo=args.source_repo.resolve(), baseline=baseline,
    )
    if errors:
        print(f"Validation failed with {len(errors)} error(s).")
        for error in errors[:50]:
            print("ERROR:", error)
        if len(errors) > 50:
            print(f"ERROR: ... {len(errors) - 50} additional error(s) omitted")
        raise SystemExit(1)
    print("B0 ledger validation passed:", json.dumps(counts, sort_keys=True))


if __name__ == "__main__":
    main()
