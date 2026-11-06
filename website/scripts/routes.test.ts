import { expect, test } from 'bun:test';
import { mapLink, markdownUrl, pageUrl, repository } from './routes';

const settings = { basePath: '/re-cordis', github: 'https://github.com/antruongnguyen/re-cordis', revision: 'a'.repeat(40) };
const canonical = new Set(['docs/en/README.md', 'docs/vi/README.md', 'docs/en/api/README.md', 'docs/en/api/effects.md']);

test('infer Pages from GitHub origin rather than the local directory', () => {
  expect(repository('https://github.com/antruongnguyen/re-cordis.git').basePath).toBe('/re-cordis');
  expect(repository('git@github.com:owner/site.git').basePath).toBe('/site');
  expect(repository('https://github.com/owner/owner.github.io.git').basePath).toBe('');
  expect(() => repository('file:///checkout')).toThrow();
});

test('README maps to directory routes and Markdown index files', () => {
  expect(pageUrl('/re-cordis', 'en', 'api/README.md')).toBe('/re-cordis/en/docs/api/');
  expect(markdownUrl('/re-cordis', 'en', 'README.md')).toBe('/re-cordis/markdown/en/index.md');
  expect(markdownUrl('/re-cordis', 'vi', 'api/README.md')).toBe('/re-cordis/markdown/vi/api.md');
});

test('resolve locales, README, anchors, source evidence, spaces and images', () => {
  expect(mapLink('../vi/README.md', 'docs/en/README.md', canonical, settings).url).toBe('/re-cordis/vi/docs/');
  expect(mapLink('README.md#export-map', 'docs/en/api/effects.md', canonical, settings).url).toBe('/re-cordis/en/docs/api/#export-map');
  expect(mapLink('../../../crates/re-cordis-core/src/lib.rs#L10', 'docs/en/api/effects.md', canonical, settings).url).toBe(`${settings.github}/blob/${settings.revision}/crates/re-cordis-core/src/lib.rs#L10`);
  expect(mapLink('../../plans/review%20notes.md#check', 'docs/en/README.md', canonical, settings).url).toBe(`${settings.github}/blob/${settings.revision}/plans/review%20notes.md#check`);
  expect(mapLink('photo.png', 'docs/en/README.md', canonical, settings, true).url).toBe(`https://raw.githubusercontent.com/antruongnguyen/re-cordis/${settings.revision}/docs/en/photo.png`);
  expect(mapLink('#cleanup', 'docs/en/README.md', canonical, settings).url).toBe('#cleanup');
  expect(mapLink('https://example.com/ref', 'docs/en/README.md', canonical, settings).url).toBe('https://example.com/ref');
  expect(() => mapLink('../../../outside.md', 'docs/en/README.md', canonical, settings)).toThrow();
});

test('Next router receives one project prefix for search and sidebar targets', async () => {
  const { routerUrl } = await import('../lib/urls');
  const published = '/re-cordis/en/docs/api/effects/#cleanup';
  const nextTarget = routerUrl(published, '/re-cordis');
  expect(nextTarget).toBe('/en/docs/api/effects/#cleanup');
  expect('/re-cordis' + nextTarget).toBe(published);
  expect(routerUrl('/vi/docs/', '/re-cordis')).toBe('/vi/docs/');
  expect(routerUrl('https://github.com/antruongnguyen/re-cordis', '/re-cordis')).toBe('https://github.com/antruongnguyen/re-cordis');
  expect(routerUrl('/en/docs/', '')).toBe('/en/docs/');
});
