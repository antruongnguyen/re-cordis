import { expect, test } from 'bun:test';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkStringify from 'remark-stringify';
import { publicDocumentation, isInternalDocumentation } from './visibility';

const github = 'https://github.com/antruongnguyen/re-cordis';

test('internal documentation never becomes a public link, image or reference definition', () => {
  const processor = unified().use(remarkParse).use(remarkStringify);
  const input = '# Guide\n\n[accepted](../../plans/review.md) and [design][spec].\n\n![private](../../superpowers/layout.png)\n\n[spec]: https://github.com/antruongnguyen/re-cordis/blob/feat/docs/docs/superpowers/spec.md\n\n[API](../api/README.md)\n\n```rust\nfn main() {}\n```\n';
  const tree = processor.parse(input);
  publicDocumentation(tree, 'docs/en/guides/test.md', github);
  const result = processor.stringify(tree);
  expect(result).not.toMatch(/docs\/(plans|superpowers)|\.\.\/(plans|superpowers)/);
  expect(result).toContain('accepted and design.');
  expect(result).toContain('[API](../api/README.md)');
  expect(result).toContain('fn main() {}');
});

test('private boundaries cover relative, encoded and pinned URLs without hiding public sources', () => {
  for (const href of ['../plans/a.md', '../superpowers/a.md', '/docs/plans/a.md', '../%70lans/a.md',
    github + '/tree/main/docs/plans', github + '/blob/feat/docs/docs/superpowers/a.md',
    'https://raw.githubusercontent.com/antruongnguyen/re-cordis/main/docs/plans/a.md']) {
    expect(isInternalDocumentation(href, 'docs/en/README.md', github)).toBe(true);
  }
  for (const href of ['../en/api/README.md', '../../crates/re-cordis-core/src/lib.rs', 'https://example.com/docs/plans/a.md']) {
    expect(isInternalDocumentation(href, 'docs/en/README.md', github)).toBe(false);
  }
});
