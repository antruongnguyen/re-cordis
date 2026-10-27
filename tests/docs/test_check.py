"""Regression checks for the documentation validator; standard library only."""

from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import check


class DocumentationChecks(unittest.TestCase):
    def test_complete_fences(self):
        self.assertEqual(
            check.blocks("```rust\nfn main() {}\n```\n\n```toml\nx = 1\n```\n"),
            [("rust", "fn main() {}\n"), ("toml", "x = 1\n")],
        )
        self.assertEqual(check.blocks("Plain text.\n"), [])

    def test_unclosed_or_malformed_fences(self):
        for text in ("```rust\nfn main() {}\n", "```\n",
                     "```rust\nfn main() {}\n```extra\n"):
            with self.subTest(text=text):
                with self.assertRaisesRegex(ValueError, "Unclosed or malformed"):
                    check.blocks(text)

    def test_unchecked_rust_attributes(self):
        for attributes in ("ignore", "no_run", "compile_fail"):
            with self.subTest(attributes=attributes):
                with self.assertRaisesRegex(ValueError, "complete runnable Rust blocks"):
                    check.blocks(f"```rust,{attributes}\nfn main() {{}}\n```\n")

    def test_both_languages_reject_unchecked_or_unclosed_blocks(self):
        for language in ("en", "vi"):
            for extra, error in (
                ("\n```rust,ignore\nfn main() { unknown(); }\n```\n",
                 "complete runnable Rust blocks"),
                ("\n```rust\nfn main() { unknown(); }\n", "Unclosed or malformed"),
            ):
                with self.subTest(language=language, extra=extra):
                    with tempfile.TemporaryDirectory(prefix="cordis-docs-test-") as directory:
                        docs = Path(directory).resolve()
                        for current, other in (("en", "vi"), ("vi", "en")):
                            folder = docs / current
                            folder.mkdir()
                            text = (f"[Other](../{other}/README.md)\n\n"
                                    "```rust\nfn main() {}\n```\n")
                            if current == language:
                                text += extra
                            (folder / "README.md").write_text(text, encoding="utf-8")
                        with patch.object(check, "DOCS", docs):
                            with self.assertRaisesRegex(ValueError, error):
                                check.check_structure()

    def test_local_file_and_heading_links(self):
        with tempfile.TemporaryDirectory(prefix="cordis-docs-links-") as directory:
            root = Path(directory).resolve()
            target = root / "target.md"
            target.write_text("# Heading\n\n## Heading\n", encoding="utf-8")
            source = root / "source.md"
            with patch.object(check, "ROOT", root):
                source.write_text("[Good](target.md#heading-1)\n", encoding="utf-8")
                check.links(source)
                for link, error in (
                    ("missing.md", "missing link target"),
                    ("target.md#absent", "missing heading"),
                    ("../outside.md", "link leaves checkout"),
                ):
                    with self.subTest(link=link):
                        source.write_text(f"[Bad]({link})\n", encoding="utf-8")
                        with self.assertRaisesRegex(ValueError, error):
                            check.links(source)


if __name__ == "__main__":
    unittest.main()
