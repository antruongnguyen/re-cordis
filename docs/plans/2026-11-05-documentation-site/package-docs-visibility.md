# Package documentation visibility audit

Date: 2026-11-05 (Asia/Ho_Chi_Minh). Audited checkout:
`abfa0f3092fd8b06765d3c004fbf130793b69fe5`.

Scope: prevent repository `docs/plans` and `docs/superpowers` content from
entering crate archives, package READMEs or Rustdoc. Website route, Markdown,
search and exported-file checks are a separate part of this follow-up. This
audit changes no crate README, manifest, source, lockfile or Create template.

## Packaging and docs.rs inputs

The repository Cargo.toml is a virtual workspace, with no root package. Each of
the seven package roots is `crates/re-cordis-<name>/`; `docs/plans` and
`docs/superpowers` are outside every package root. Cargo packages files within
the selected package root. Its `include`/`exclude` patterns are package-relative;
adding exclusions for repository-root directories would not change these
archives. No package has an `include`, `exclude`, custom `readme`, `license-file`
or `build` setting, and there is no package-root build.rs or crate symlink that
pulls in either private documentation tree.

All seven crates have their own README.md and LICENSE. With `package.readme`
unset, Cargo and docs.rs select the package-root README.md; the repository README
is not selected. Those seven READMEs contain no private directory references.
Their existing guide and release-readiness references describe checkout files,
and explicitly state that the repository guides are not bundled in the archive.
They preserve the current unpublished `0.0.0`/`publish=false` state and do not
promise a hosted docs.rs release. No README correction is needed.

Rustdoc takes its prose and examples from Rust source documentation. Crate
headers, documentation attributes and crate-local Markdown contain no reference
to either private tree. No source includes private Markdown. Create's production
`include_str!` calls embed only its five existing files under templates/basic;
that README also contains no private tree reference. Existing parity fixtures
referenced by tests/examples are unrelated to the two private documentation
trees; this audit does not change their archive-testing boundaries.

HMR's existing `package.metadata.docs.rs.features = ["include"]` selects its
optional Include APIs. It does not import repository Markdown, change defaults
or authorize publication. All other crates retain ordinary documentation
defaults. Explicit manifest exclusions are therefore unnecessary for the
current layout. Reassess if a future package README/build script/doc attribute
imports repository documents, or a package is moved to the repository root.

Official references consulted:

- [Cargo package include/exclude rules](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields)
  describe the package-root default, package-relative patterns and
  `cargo package --list` verification.
- [Cargo README rules](https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field)
  describe package-root README selection and registry transfer.
- [docs.rs builds and README selection](https://docs.rs/about/builds)
  describe automatic builds of crates.io releases and the same Cargo README
  rules. A local Rustdoc build is not a hosted docs.rs build.

## Existing artifact inspection

The existing target/site-rustdoc/doc tree was present when inspected. A recursive
scan found no path component named plans or superpowers, and no
`docs/plans`/`docs/superpowers` reference in 3,422 HTML, JavaScript, JSON, text,
Markdown and XML files. The reference scan covered slash-separated,
JSON-escaped-slash, percent-encoded-slash and HTML-entity-slash spellings.
Source/manifest/Markdown inspection found no private directory reference or
private Markdown inclusion. No Cargo command or documentation rebuild was run
by this scoped audit, so this observation alone does not establish a fresh
artifact's checkout provenance.

## Fresh verification gates

The controller runs the package-list and fresh Rustdoc gates on the final tree.
The commands below list package contents without publishing, creating archives
or changing package policy. `--allow-dirty` permits the documentation follow-up
before its local commit. Run each toolchain in sequence with the repository's
existing explicit-tool resolver:

```bash
for toolchain in stable 1.85.0; do
  source .github/scripts/rust-tools.sh "$toolchain"
  lists="$PWD/target/package-docs-visibility/$toolchain"
  mkdir -p "$lists"
  for name in core timer logger loader include hmr create; do
    "$CORDIS_CARGO" package --list --allow-dirty --locked --offline \
      --manifest-path "crates/re-cordis-$name/Cargo.toml" \
      > "$lists/re-cordis-$name.txt"
  done
done
```

Require all 14 list commands to exit zero. Assert the lists contain each crate's
README.md and LICENSE and have no private tree or parent-traversal entry:

```bash
python3 -B - <<'PY'
from pathlib import Path, PurePosixPath

root = Path("target/package-docs-visibility")
for toolchain in ("stable", "1.85.0"):
    for name in ("core", "timer", "logger", "loader", "include", "hmr", "create"):
        file = root / toolchain / f"re-cordis-{name}.txt"
        entries = file.read_text().splitlines()
        assert {"README.md", "LICENSE"} <= set(entries), file
        for entry in entries:
            parts = PurePosixPath(entry.replace("\\", "/")).parts
            assert ".." not in parts, (file, entry)
            assert not any(
                parts[i:i + 2] in (("docs", "plans"), ("docs", "superpowers"))
                for i in range(len(parts) - 1)
            ), (file, entry)
        print(toolchain, name, len(entries), "files: visibility gate passed")
PY
```

Build warnings-denied workspace Rustdoc with all features and the committed
lockfile using the existing documentation workflow. HMR's explicit optional
Include API must remain present. Reuse that fresh output for the following
assertion; point `artifact` at the chosen toolchain's output if different:

```bash
python3 -B - <<'PY'
from html import unescape
from pathlib import Path
from urllib.parse import unquote
import re

artifact = Path("target/site-rustdoc/doc")
assert artifact.is_dir(), artifact
for name in ("core", "timer", "logger", "loader", "include", "hmr", "create"):
    assert (artifact / f"re_cordis_{name}" / "index.html").is_file(), name
private = re.compile(r"docs[/\\]+(?:plans|superpowers)(?:[/\\]|\b)", re.I)
checked = 0
for file in artifact.rglob("*"):
    assert not {"plans", "superpowers"} & set(file.relative_to(artifact).parts), file
    if not file.is_file() or file.suffix not in {
        ".html", ".js", ".json", ".txt", ".md", ".xml"
    }:
        continue
    text = file.read_text(errors="replace").replace("\\/", "/")
    for _ in range(2):
        text = unquote(unescape(text))
    assert not private.search(text), file
    checked += 1
for file, anchor in (
    ("re_cordis_hmr/struct.FileWatcher.html", 'id="method.watch_include"'),
    ("re_cordis_hmr/struct.ReloadPlan.html", 'id="method.replace_include"'),
):
    assert anchor in (artifact / file).read_text(), (file, anchor)
print(checked, "Rustdoc text files: visibility gate passed")
PY
```

The public website additionally needs its own generated Markdown, navigation,
search, LLM index, client/server payload and exported-file assertions; crate
packaging cannot enforce those. Removing these trees from generated
documentation does not remove tracked files from a public Git checkout. The
current audit proves no hosted Pages deployment, crates.io publication or hosted
docs.rs/nightly/Linux execution.
