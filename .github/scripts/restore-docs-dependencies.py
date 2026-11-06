#!/usr/bin/env python3
"""Restore the external guide applications' dependency features before offline checks.

Workspace fetch alone omits futures-util's default async-await-macro feature,
used by tests/docs/check.py's generated R0 application. This disposable manifest
matches that application's dependency superset and restores the published
quick-start core package before the offline checker. It changes neither the
distributed Create template nor either repository lockfile.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def main():
    cargo = os.environ["CORDIS_CARGO"]
    if not Path(cargo).is_absolute():
        raise ValueError("Source .github/scripts/rust-tools.sh first")
    with tempfile.TemporaryDirectory(prefix="cordis-doc-dependencies-") as directory:
        folder = Path(directory)
        dependencies = []
        for name in ("core", "timer", "logger", "loader", "include", "hmr"):
            path = json.dumps(str(ROOT / "crates" / f"re-cordis-{name}"))
            feature = ', features = ["include"]' if name == "hmr" else ""
            dependencies.append(f"re-cordis-{name} = {{ path = {path}{feature} }}")
        dependencies.extend([
            'registry-core = { package = "re-cordis-core", version = "=0.1.0-alpha.1" }',
            'futures-util = "0.3"',
            'serde_json = "1"',
            'tokio = { version = "1.48", default-features = false, features = ["rt", "time", "sync", "macros", "test-util"] }',
        ])
        manifest = ('[package]\nname = "cordis-doc-dependencies"\nversion = "0.0.0"\n'
                    'edition = "2024"\nrust-version = "1.85"\npublish = false\n'
                    '[workspace]\nresolver = "3"\n[dependencies]\n'
                    + "\n".join(dependencies) + "\n")
        (folder / "Cargo.toml").write_text(manifest)
        (folder / "src").mkdir()
        (folder / "src/lib.rs").write_text("")
        subprocess.run([cargo, "fetch", "--manifest-path", str(folder / "Cargo.toml")],
                       check=True, cwd=ROOT)


if __name__ == "__main__":
    main()
