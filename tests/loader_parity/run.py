"""Run two pinned Rust/TypeScript loader causal traces; Python stdlib only."""

import argparse
import difflib
import json
import os
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "tests/loader_parity"
PIN = "35fdc969e4b6c48c48ed44717c7a018324e4dd88"
SCENARIOS = ("recovery", "group")


def run(command, *, cwd=ROOT, env=None):
    subprocess.run(command, cwd=cwd, env=env, check=True, timeout=180, text=True)


def output(command, *, cwd=ROOT):
    return subprocess.check_output(command, cwd=cwd, text=True, timeout=30).strip()


def compare(work):
    for name in SCENARIOS:
        left = (work / "rust" / f"{name}.txt").read_bytes()
        right = (work / "typescript" / f"{name}.txt").read_bytes()
        if left != right:
            raise SystemExit(
                "".join(difflib.unified_diff(
                    left.decode().splitlines(keepends=True),
                    right.decode().splitlines(keepends=True),
                    fromfile=f"Rust/{name}", tofile=f"TypeScript/{name}",
                ))
            )
        print(f"{name}: Rust == TypeScript ({len(left.splitlines())} observations)")


def execute(work, upstream, tools, env):
    rust = work / "rust"
    typescript = work / "typescript"
    rust.mkdir()
    typescript.mkdir()
    config = work / "tsconfig.json"
    config.write_text(json.dumps({
        "extends": str(upstream / "tsconfig.json"),
        "compilerOptions": {
            "composite": False,
            "incremental": False,
            "typeRoots": [
                str(upstream / "node_modules/@types"),
                str(upstream / "node_modules"),
            ],
        },
        "files": [str(upstream / "packages/loader/tests/re-cordis-loader-parity.spec.ts")],
    }))
    destination = upstream / "packages/loader/tests/re-cordis-loader-parity.spec.ts"
    # Exclusive creation prevents overwriting an upstream file or another run.
    with destination.open("x") as target:
        try:
            target.write((FIXTURES / "loader.spec.ts").read_text())
            target.flush()
            run(
                [
                    "corepack", "yarn", "eslint", "--no-ignore", "--max-warnings", "0",
                    "packages/loader/tests/re-cordis-loader-parity.spec.ts",
                ],
                cwd=upstream, env=env,
            )
            run(
                [
                    "corepack", "yarn", "tsc", "--noEmit", "--skipLibCheck",
                    "--target", "es2024", "--module", "nodenext",
                    "--allowImportingTsExtensions", "--strict",
                    "--noImplicitAny", "false", "--noImplicitThis", "false",
                    "--strictFunctionTypes", "false",
                    "--project", str(config),
                ],
                cwd=upstream, env=env,
            )
            env["CORDIS_LOADER_PARITY_OUTPUT"] = str(typescript)
            run(
                ["corepack", "yarn", "test", "loader/re-cordis-loader-parity"],
                cwd=upstream, env=env,
            )
        finally:
            destination.unlink()
    run(["git", "diff", "--exit-code", "HEAD", "--", "."], cwd=upstream)
    env["CORDIS_LOADER_PARITY_OUTPUT"] = str(rust)
    run(
        [tools["cargo"], "test", "-p", "re-cordis-loader", "--test", "loader_parity", "--locked"],
        env=env,
    )
    compare(work)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", type=Path, default=ROOT / "local/cordis")
    parser.add_argument("--toolchain", default="stable")
    parser.add_argument("--output-dir", type=Path, help="retain traces in a new directory")
    args = parser.parse_args()
    upstream = args.upstream.resolve()
    if output(["git", "rev-parse", "HEAD"], cwd=upstream) != PIN:
        raise SystemExit(f"TypeScript checkout must be pinned to {PIN}")
    run(["git", "diff", "--exit-code", "HEAD", "--", "."], cwd=upstream)
    tools = {
        name: output(["rustup", "which", "--toolchain", args.toolchain, name])
        for name in ("rustc", "rustdoc", "cargo")
    }
    env = os.environ.copy()
    env.update(
        PATH=str(Path(tools["rustc"]).parent) + os.pathsep + env["PATH"],
        RUSTC=tools["rustc"],
        RUSTDOC=tools["rustdoc"],
        CARGO_TARGET_DIR=str(ROOT / "target" / args.toolchain),
        CORDIS_LOADER_PARITY_FIXTURES=str(FIXTURES),
    )
    for tool in tools.values():
        run([tool, "--version"], env=env)
    run(["node", "--version"], cwd=upstream, env=env)
    run(["corepack", "yarn", "--version"], cwd=upstream, env=env)
    (ROOT / "target").mkdir(exist_ok=True)
    if args.output_dir:
        work = args.output_dir.resolve()
        work.mkdir(parents=True)
        print(f"Loader traces: {work}", flush=True)
        execute(work, upstream, tools, env)
    else:
        with tempfile.TemporaryDirectory(prefix="loader-parity-", dir=ROOT / "target") as work:
            execute(Path(work), upstream, tools, env)


if __name__ == "__main__":
    main()
