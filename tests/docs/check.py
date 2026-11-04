#!/usr/bin/env python3
"""Check bilingual documentation and run complete Rust snippets without downloads."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[2]
DOCS = ROOT / "docs"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def blocks(text):
    found = re.findall(r"^```([^\n]*)\n(.*?)^```[ \t]*$", text, re.M | re.S)
    fences = re.findall(r"^```[^\n]*$", text, re.M)
    require(len(fences) == 2 * len(found), "Unclosed or malformed fenced code block")
    for kind, _ in found:
        require(not kind.startswith("rust") or kind == "rust",
                "Use only complete runnable Rust blocks, without unchecked attributes")
    return found


def prose(text):
    return re.sub(r"^```[^\n]*\n.*?^```[ \t]*$", "", text, flags=re.M | re.S)


def anchors(text):
    found = set()
    counts = {}
    for heading in re.findall(r"^#{1,6}\s+(.+?)\s*#*\s*$", prose(text), re.M):
        slug = re.sub(r"[^\w -]", "", heading.lower()).replace(" ", "-")
        count = counts.get(slug, 0)
        counts[slug] = count + 1
        found.add(f"{slug}-{count}" if count else slug)
    return found


def links(path):
    text = path.read_text(encoding="utf-8")
    blocks(text)
    text = prose(text)
    for target in re.findall(r"\[[^\]\n]+\]\(([^)\s]+)\)", text):
        url = urlsplit(target)
        if url.scheme or url.netloc:
            continue
        resolved = (path.parent / unquote(url.path)).resolve() if url.path else path
        require(resolved.is_relative_to(ROOT), f"{path}: link leaves checkout: {target}")
        require(resolved.exists(), f"{path}: missing link target: {target}")
        if url.fragment and resolved.suffix == ".md":
            require(
                unquote(url.fragment) in anchors(resolved.read_text(encoding="utf-8")),
                f"{path}: missing heading: {target}",
            )


def direct_exports(source):
    """Discover direct public satellite types and free functions."""
    return set(re.findall(r"^pub (?:trait|struct|enum|fn) (\w+)", source, re.M))


def exported_names(source):
    """Discover direct types/functions and explicit root reexports."""
    names = direct_exports(source)
    names.update(re.findall(r"^pub (?:const|type) (\w+)", source, re.M))
    for group, single, alias in re.findall(
            r"^pub use [\w:]+::(?:\{([^}]+)\}|(\w+)(?: as (\w+))?);", source, re.M | re.S):
        names.update(re.findall(r"\b\w+\b", group) if group else [alias or single])
    return names


def check_create_help(help_text, documentation):
    """Keep the actual bundled CLI flags visible in the user reference."""
    options = set(re.findall(r"--[a-z][a-z-]*", help_text))
    require(options, "No Create CLI options discovered")
    for option in sorted(options):
        require(option in documentation, f"Create CLI option not documented: {option}")


def check_structure():
    english = {path.relative_to(DOCS / "en") for path in (DOCS / "en").rglob("*.md")}
    vietnamese = {path.relative_to(DOCS / "vi") for path in (DOCS / "vi").rglob("*.md")}
    require(english and english == vietnamese, "English/Vietnamese page paths differ")
    snippets = {}
    for relative in sorted(english):
        en, vi = (DOCS / language / relative for language in ("en", "vi"))
        en_text, vi_text = en.read_text(encoding="utf-8"), vi.read_text(encoding="utf-8")
        for path, other in ((en, vi), (vi, en)):
            counterpart = Path(os.path.relpath(other, path.parent)).as_posix()
            require(f"]({counterpart})" in path.read_text(encoding="utf-8"),
                    f"{path}: missing language counterpart")
        en_blocks = [(kind, code) for kind, code in blocks(en_text)
                     if kind in ("rust", "toml", "sh", "bash")]
        vi_blocks = [(kind, code) for kind, code in blocks(vi_text)
                     if kind in ("rust", "toml", "sh", "bash")]
        require(en_blocks == vi_blocks, f"{relative}: translated code differs")
        for kind, code in blocks(en_text):
            if kind == "rust":
                snippets.setdefault(code, f"{relative} (both languages)")

    pages = [DOCS / language / relative for language in ("en", "vi") for relative in english]
    pages += [DOCS / "README.md", DOCS / "llms.txt", ROOT / "README.md",
              ROOT / "CHANGELOG.md", DOCS / "todo.md", DOCS / "decision.md",
              DOCS / "limitation.md"]
    for path in pages:
        links(path)

    source = (ROOT / "crates/re-cordis-core/src/lib.rs").read_text(encoding="utf-8")
    exports = set()
    for group, single in re.findall(r"pub use \w+::(?:\{([^}]+)\}|(\w+));", source, re.S):
        exports.update(re.findall(r"\b\w+\b", group or single))
    require(exports, "No public exports discovered")
    # Timer satellite exports: public items declared directly in its lib.rs
    # (traits, types, enums). Each must be indexed and covered in a chapter.
    timer_source = (ROOT / "crates/re-cordis-timer/src/lib.rs").read_text(encoding="utf-8")
    timer_exports = direct_exports(timer_source)
    require(timer_exports, "No public timer exports discovered")
    logger_source = (ROOT / "crates/re-cordis-logger/src/lib.rs").read_text(encoding="utf-8")
    logger_exports = direct_exports(logger_source)
    require(logger_exports, "No public logger exports discovered")
    loader_source = (ROOT / "crates/re-cordis-loader/src/lib.rs").read_text(encoding="utf-8")
    loader_exports = exported_names(loader_source)
    require(loader_exports, "No public loader exports discovered")
    include_source = (ROOT / "crates/re-cordis-include/src/lib.rs").read_text(encoding="utf-8")
    include_exports = exported_names(include_source)
    require(include_exports, "No public include exports discovered")
    hmr_source = (ROOT / "crates/re-cordis-hmr/src/lib.rs").read_text(encoding="utf-8")
    hmr_exports = exported_names(hmr_source)
    require(hmr_exports, "No public HMR exports discovered")
    create_source = (ROOT / "crates/re-cordis-create/src/lib.rs").read_text(encoding="utf-8")
    create_exports = exported_names(create_source)
    require(create_exports, "No public Create exports discovered")
    hooks = (ROOT / "crates/re-cordis-core/src/hooks.rs").read_text(encoding="utf-8")
    payloads = re.findall(r"^pub struct (\w+)", hooks, re.M)
    functions = re.findall(r"^pub fn (\w+)", hooks, re.M)
    for language in ("en", "vi"):
        index = (DOCS / language / "api/README.md").read_text(encoding="utf-8")
        chapters = "\n".join(path.read_text(encoding="utf-8")
                             for path in (DOCS / language / "api").glob("*.md")
                             if path.name != "README.md")
        for name in exports | timer_exports | logger_exports | loader_exports | include_exports | hmr_exports | create_exports:
            require(f"`{name}`" in index, f"{language}: export not indexed: {name}")
            require(re.search(rf"\b{re.escape(name)}\b", chapters),
                    f"{language}: export not covered in a chapter: {name}")
        hook_doc = (DOCS / language / "api/errors-hooks.md").read_text(encoding="utf-8")
        for name in payloads:
            require(re.search(rf"\b{name}\b", hook_doc), f"{language}: missing hooks::{name}")
        for name in functions:
            require(f"hooks::{name}()" in hook_doc, f"{language}: missing hooks::{name}()")

    llms = (DOCS / "llms.txt").read_text(encoding="utf-8")
    require(llms.startswith("# re-cordis\n\n> "), "llms.txt needs project heading and summary")
    require(len(re.findall(r"^# ", llms, re.M)) == 1, "llms.txt needs exactly one H1")
    require(len(llms.encode("utf-8")) < 8192, "Keep llms.txt below 8 KiB")
    require("## Optional\n" in llms, "llms.txt needs optional evidence links")
    for section in re.split(r"^## .+\n", llms, flags=re.M)[1:]:
        require(all(not line.strip() or re.fullmatch(r"- \[.+\]\([^)]+\): .+", line)
                    for line in section.splitlines()), "llms.txt H2 sections must be file lists")
    print(f"Checked {len(english)} page pairs, {len(exports)} root exports, "
          f"{len(timer_exports)} timer exports, {len(logger_exports)} logger exports, "
          f"{len(loader_exports)} loader exports, {len(include_exports)} include exports, "
          f"{len(hmr_exports)} HMR exports, {len(create_exports)} Create exports, "
          f"{len(payloads)} hook payloads, {len(functions)} hooks, local links and llms.txt.")
    require(snippets, "No runnable Rust documentation examples found")
    return snippets


def run(command, env, timeout=180):
    result = subprocess.run(command, cwd=ROOT, env=env, text=True,
                            capture_output=True, timeout=timeout)
    if result.returncode:
        raise RuntimeError(f"Command failed: {command}\n{result.stdout}\n{result.stderr}")
    return result.stdout


def run_examples(toolchain, snippets):
    env = os.environ.copy()
    tools = {}
    for name in ("cargo", "rustc", "rustdoc", "cargo-clippy", "clippy-driver"):
        tools[name] = run(["rustup", "which", "--toolchain", toolchain, name], env, 30).strip()
    env["RUSTUP_TOOLCHAIN"] = toolchain
    env["RUSTC"], env["RUSTDOC"] = tools["rustc"], tools["rustdoc"]
    env["PATH"] = str(Path(tools["rustc"]).parent) + os.pathsep + env["PATH"]
    env["CARGO_TARGET_DIR"] = str(ROOT / "target" / ("msrv" if toolchain == "1.85.0" else "stable"))
    for name in ("cargo", "rustc", "rustdoc"):
        print(run([tools[name], "--version"], env, 30).strip())
    # Build delivered satellites and collect their explicit snippet libraries.
    # Cargo supplies their transitive dependencies through the dependency path.
    wanted = {"re_cordis_core", "re_cordis_timer", "re_cordis_logger", "re_cordis_loader",
              "re_cordis_include", "re_cordis_hmr", "re_cordis_create", "serde_json", "tokio", "futures_util"}
    output = run([tools["cargo"], "build", "-p", "re-cordis-timer",
                  "-p", "re-cordis-logger", "-p", "re-cordis-loader",
                  "-p", "re-cordis-include", "-p", "re-cordis-hmr", "-p", "re-cordis-create",
                  "--features", "re-cordis-hmr/include", "--lib", "--offline", "--locked",
                  "--message-format=json"], env)
    libraries = {}
    for line in output.splitlines():
        message = json.loads(line)
        if message.get("reason") == "compiler-artifact":
            name = message["target"]["name"]
            if name in wanted:
                for filename in message["filenames"]:
                    if filename.endswith(".rlib"):
                        libraries[name] = Path(filename)
    require(set(libraries) == wanted, "Missing compiler library artifacts")
    with tempfile.TemporaryDirectory(prefix="cordis-docs-") as directory:
        folder = Path(directory)
        for index, (code, label) in enumerate(snippets.items()):
            source, executable = folder / f"example_{index}.rs", folder / f"example_{index}"
            source.write_text(code, encoding="utf-8")
            command = [tools["rustc"], "--edition=2024", "-D", "warnings", str(source),
                       "-o", str(executable)]
            for directory in sorted({library.parent for library in libraries.values()}):
                command += ["-L", f"dependency={directory}"]
            for name, library in libraries.items():
                command += ["--extern", f"{name}={library}"]
            run(command, env)
            run([str(executable)], env, 30)
            print(f"Passed: {label}")
    print(f"Executed {len(snippets)} unique Rust examples on {toolchain}.")

    quickstart = dict(blocks((DOCS / "en/getting-started.md").read_text(encoding="utf-8")))
    dependency = ROOT / "crates/re-cordis-core"
    manifest = quickstart["toml"].replace(
        '"../re-cordis/crates/re-cordis-core"', json.dumps(dependency.as_posix())
    )
    with tempfile.TemporaryDirectory(prefix="cordis-quickstart-") as directory:
        folder = Path(directory)
        (folder / "Cargo.toml").write_text(manifest, encoding="utf-8")
        (folder / "src").mkdir()
        (folder / "src/main.rs").write_text(quickstart["rust"], encoding="utf-8")
        run([tools["cargo"], "run", "--manifest-path", str(folder / "Cargo.toml"),
             "--offline", "--quiet"], env)
    print(f"Passed actual quick-start dependency manifest on {toolchain}.")

    create_command = [tools["cargo"], "run", "-p", "re-cordis-create", "--offline", "--locked", "--"]
    help_text = run([*create_command, "--help"], env)
    for language in ("en", "vi"):
        documentation = "\n".join(
            (DOCS / language / f"{part}/create.md").read_text(encoding="utf-8")
            for part in ("guides", "api")
        )
        check_create_help(help_text, documentation)
    with tempfile.TemporaryDirectory(prefix="cordis-create-docs-") as directory:
        destination = Path(directory) / "ứng dụng có khoảng trắng"
        run([*create_command, "cordis-created-app", "--path", str(destination),
             "--cordis-path", str(ROOT)], env)
        expected = {"Cargo.toml", "src/main.rs", "src/app.rs", "README.md", ".gitignore"}
        actual = {p.relative_to(destination).as_posix()
                  for p in destination.rglob("*") if p.is_file()}
        require(actual == expected, f"Unexpected Create file set: {actual}")
        manifest_path = str(destination / "Cargo.toml")
        metadata = json.loads(run([tools["cargo"], "metadata", "--manifest-path", manifest_path,
                                   "--no-deps", "--format-version=1", "--offline"], env))
        require(Path(metadata["workspace_root"]).resolve() == destination.resolve(),
                "Generated application inherited a different workspace")
        package = next(p for p in metadata["packages"] if p["name"] == "cordis-created-app")
        require(package["version"] == "0.0.0" and package["edition"] == "2024" and package["rust_version"] == "1.85"
                and package["publish"] == [], "Generated application package policy drifted")
        for name in ("core", "logger", "loader"):
            dependency = next(d for d in package["dependencies"] if d["name"] == f"re-cordis-{name}")
            require(Path(dependency["path"]).resolve() == ROOT / f"crates/re-cordis-{name}",
                    f"Incorrect generated path dependency: {name}")
        for command in ("build", "test", "run"):
            # First build resolves an external lockfile using cached dependencies.
            args = [tools["cargo"], command, "--manifest-path", manifest_path, "--offline"]
            if command != "build":
                args.append("--locked")
            output = run(args, env)
            print(f"Generated application {command} on {toolchain}:\n{output.strip()}")
        run([tools["cargo"], "clippy", "--manifest-path", manifest_path,
             "--all-targets", "--offline", "--locked", "--", "-D", "warnings"], env)
        if toolchain == "stable":
            for name in ("cargo-fmt", "rustfmt"):
                formatter = run(["rustup", "which", "--toolchain", toolchain, name], env, 30).strip()
                require(Path(formatter).parent == Path(tools["rustc"]).parent,
                        f"Formatter is outside the selected toolchain: {name}")
            run([tools["cargo"], "fmt", "--manifest-path", manifest_path, "--check"], env)
    print(f"Passed Create CLI, exact files, Cargo metadata and external application on {toolchain}.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--toolchain", choices=("stable", "1.85.0"), default="stable")
    args = parser.parse_args()
    run_examples(args.toolchain, check_structure())
