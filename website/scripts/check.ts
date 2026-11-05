import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { load } from 'cheerio';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkRehype from 'remark-rehype';
import rehypeSlug from 'rehype-slug';
import { visit } from 'unist-util-visit';
import { staticClient } from 'fumadocs-core/search/client/orama-static';
import type { Root as MdRoot } from 'mdast';
import type { Root as HtmlRoot } from 'hast';
import dependencyProseLinks from './rustdoc-prose-links.json';
import { locales, mapLink, pageUrl, markdownUrl } from './routes';

const root = path.resolve('..');
const output = path.resolve('out');
const provenance = JSON.parse(await readFile('out/build-provenance.json', 'utf8'));
const pages: { locale: string; file: string; source: string; title: string; titleId: string; url: string; markdownUrl: string; sourceUrl: string; toc: { url: string }[] }[] = JSON.parse(await readFile('.generated/pages.json', 'utf8'));
const { basePath, origin, github, revision } = provenance;
assert(!Object.hasOwn(provenance, 'rustdocInput'), 'Private Rustdoc input path must not be published.');
for await (const file of new Bun.Glob('**/*').scan('out')) {
  if (file.startsWith('rustdoc/')) continue; // Exact Rustdoc retains compiler paths; Pages rebuilds on its hosted macOS runner.
  if (!/\.(?:html|js|json|css|md|txt)$/.test(file)) continue;
  const content = await readFile(path.join(output, file), 'utf8');
  assert(!process.env.HOME || !content.includes(process.env.HOME), 'Workstation home path leaked into public artifact: ' + file);
}
const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
assert.equal(revision, head, 'Export revision is stale; rebuild after commit.');
assert.match(revision, /^[a-f0-9]{40}$/);
assert.equal(basePath, '/re-cordis', 'This repository publishes at its actual project subpath.');
assert.equal(provenance.status, 'development-unpublished');
for (const [source, expected] of Object.entries(provenance.sources)) {
  assert.equal(createHash('sha256').update(await readFile(path.join(root, source))).digest('hex'), expected, `Source changed after build: ${source}`);
}
assert.equal(provenance.rustdocFileKeyFormat, 'sha256-relative-path');
let rustdocFileCount = 0;
for await (const file of new Bun.Glob('**/*').scan({ cwd: 'out/rustdoc/' + revision, dot: true })) {
  const key = createHash('sha256').update(file).digest('hex');
  const actual: string = createHash('sha256').update(await readFile(path.join(output, 'rustdoc', revision, file))).digest('hex');
  assert.equal(actual, provenance.rustdocFiles[key], 'Bundled Rustdoc changed: ' + file);
  rustdocFileCount++;
}
assert.equal(rustdocFileCount, Object.keys(provenance.rustdocFiles).length, 'Rustdoc file manifest differs from output.');
for (const [file, expected] of Object.entries(provenance.emittedRustdocFiles)) {
      assert.equal(provenance.rustdocFiles[file], expected, 'Compiler-emitted Rustdoc bytes must stay exact: ' + file);
    }
    const canonical = new Set<string>();
for (const locale of locales) for await (const file of new Bun.Glob('**/*.md').scan(path.join(root, 'docs', locale))) canonical.add(`docs/${locale}/${file}`);
assert.deepEqual([...canonical].sort(), pages.map((page) => page.source).sort(), 'Canonical page discovery/export mismatch.');
assert.equal(new Set(pages.map((page) => page.url)).size, pages.length, 'Route collision.');
const anchorIds = new Map<string, Set<string>>();
const redirects = new Map<string, string>();
const rustdocErrors: { source: string; target: string; message: string }[] = [];
assert(dependencyProseLinks.every((entry) => !entry.file.startsWith('re_cordis_')), 'Framework entries are forbidden in the dependency prose boundary.');
const unresolvedCounts = new Map<string, number>();
let checkedLinks = 0;
let checkedAssets = 0;
async function document(file: string) {
  const html = load(await readFile(file, 'utf8'));
  if (!anchorIds.has(file)) anchorIds.set(file, new Set(html('[id]').toArray().map((el) => html(el).attr('id')!)));
  const refresh = html('meta[http-equiv="refresh"]').attr('content');
  if (refresh?.startsWith('0;URL=')) redirects.set(file, refresh.slice(6));
  return html;
}
function exportedFile(url: URL) {
  assert(url.pathname.startsWith(`${basePath}/`), `Missing project subpath: ${url.href}`);
  const relative = decodeURIComponent(url.pathname.slice(basePath.length + 1));
  return path.join(output, relative, url.pathname.endsWith('/') ? 'index.html' : '');
}
async function checkUrl(href: string, current: string, seenRedirects = new Set<string>(), allowDependencyProse = true) {
  if (!href || /^(?:mailto:|tel:|data:|javascript:)/.test(href)) return;
  const rustdocPrefix = basePath + '/rustdoc/' + revision + '/';
  const dependencyFile = current.startsWith(rustdocPrefix) ? current.slice(rustdocPrefix.length) : undefined;
  const known = allowDependencyProse && dependencyFile && !dependencyFile.startsWith('re_cordis_')
    ? dependencyProseLinks.find((entry) => entry.file === dependencyFile && entry.target === href)
    : undefined;
  if (known) {
    const key = known.file + '\u0000' + href;
    unresolvedCounts.set(key, (unresolvedCounts.get(key) ?? 0) + 1);
    return;
  }
  assert(!/^[A-Za-z_]\w*::/.test(href), 'Unexpected unresolved Rust-path href: ' + current + ' → ' + href);
  const url = new URL(href, `${origin}${current}`);
  if (url.origin === origin) {
    const file = exportedFile(url);
    await stat(file).catch(() => { throw new Error(`Missing exported target: ${current} → ${href}`); });
    if (url.hash && file.endsWith('.html')) {
      if (!anchorIds.has(file)) await document(file);
      const redirect = redirects.get(file);
          if (redirect) {
            assert(!seenRedirects.has(file), 'Rustdoc redirect cycle: ' + file);
            seenRedirects.add(file);
            const destination = new URL(redirect, url);
            destination.search = url.search;
            destination.hash = url.hash;
            await checkUrl(destination.href, current, seenRedirects, allowDependencyProse);
            return;
          }
          const anchors = [url.hash.slice(1), decodeURIComponent(url.hash.slice(1))];
      const ids = anchorIds.get(file)!;
          const range = url.hash.match(/^#(\d+)-(\d+)$/);
          const found = range && url.pathname.includes('/src/')
            ? ids.has(range[1]) && ids.has(range[2])
            : anchors.some((anchor) => ids.has(anchor));
          assert(found, 'Missing exported anchor: ' + current + ' → ' + href);
    }
    if (/\.(?:css|js|png|svg|woff2?)(?:$|\?)/.test(href)) checkedAssets++;
    checkedLinks++;
  } else if (url.href.startsWith(`${github}/blob/`)) {
    const prefix = `${github}/blob/${revision}/`;
    assert(url.href.startsWith(prefix), `Own-source URL must pin build revision: ${href}`);
    const source = decodeURIComponent(url.pathname.slice(new URL(prefix).pathname.length));
    const file = path.join(root, source);
    await stat(file);
    if (url.hash && file.endsWith('.md')) {
      const tree = await unified().use(remarkParse).use(remarkGfm).use(remarkRehype).use(rehypeSlug).run(unified().use(remarkParse).use(remarkGfm).parse(await readFile(file, 'utf8'))) as HtmlRoot;
      const ids: string[] = [];
      visit(tree, 'element', (node) => { if (node.properties.id) ids.push(String(node.properties.id)); });
      assert(ids.includes(decodeURIComponent(url.hash.slice(1))), `Missing source Markdown anchor: ${current} → ${href}`);
    } else if (/^#L\d+(?:-L\d+)?$/.test(url.hash)) {
      const lines = (await readFile(file, 'utf8')).split('\n').length;
      assert(Number(url.hash.match(/\d+/)![0]) <= lines, `Missing source line: ${href}`);
    }
    checkedLinks++;
  }
}
async function checkHtml(file: string, url: string, collect = false) {
  const html = await document(file);
  for (const element of html('[href], [src]').toArray()) {
    const target = html(element).attr('href') ?? html(element).attr('src') ?? '';
        try { await checkUrl(target, url, new Set<string>(), html(element).is('a')); }
        catch (error) {
          if (!collect) throw error;
          rustdocErrors.push({ source: url, target, message: error instanceof Error ? error.message : String(error) });
        }
  }
  for (const element of html('[srcset]').toArray()) {
    for (const candidate of html(element).attr('srcset')!.split(',')) await checkUrl(candidate.trim().split(/\s+/)[0], url);
  }
  const refresh = html('meta[http-equiv="refresh"]').attr('content');
  if (refresh?.startsWith('0;URL=')) redirects.set(file, refresh.slice(6));
  return html;
}
const markdownParser = unified().use(remarkParse).use(remarkGfm);
function codeBlocks(tree: MdRoot) {
  const code: { lang?: string | null; meta?: string | null; value: string }[] = [];
  visit(tree, 'code', (node) => { code.push({ lang: node.lang, meta: node.meta, value: node.value }); });
  return code;
}
for (const page of pages) {
  assert.equal(page.url, pageUrl(basePath, page.locale, page.file));
  assert.equal(page.markdownUrl, markdownUrl(basePath, page.locale, page.file));
  const html = await checkHtml(exportedFile(new URL(origin + page.url)), page.url);
  assert.equal(html('h1').text(), page.title);
  assert.equal(html('html').attr('lang'), page.locale, 'Missing static document language.');
  assert(html('[id]').toArray().some((el) => html(el).attr('id') === page.titleId));
  assert(html('#main-content').length, 'Missing keyboard skip destination.');
  assert(html('body').text().includes('0.0.0'));
  assert(html('a').toArray().some((el) => html(el).attr('href') === page.sourceUrl));
  for (const heading of page.toc) await checkUrl(heading.url, page.url);
  const markdown = await readFile(exportedFile(new URL(origin + page.markdownUrl)), 'utf8');
  assert(markdown.includes(`Build revision ${revision}`));
  const actual = markdownParser.parse(markdown) as MdRoot;
  const raw = markdownParser.parse(await readFile(path.join(root, page.source), 'utf8')) as MdRoot;
  assert.deepEqual(codeBlocks(actual), codeBlocks(raw), `Executable Markdown code changed: ${page.source}`);
  const urls: string[] = [];
  visit(actual, (node) => { if (node.type === 'link' || node.type === 'image' || node.type === 'definition') urls.push(node.url); });
  for (const href of urls) await checkUrl(href, page.url);
  visit(raw, (node) => {
    if (node.type === 'link' || node.type === 'image' || node.type === 'definition') {
      assert(urls.includes(mapLink(node.url, page.source, canonical, provenance, node.type === 'image').url), `Missing mapped Markdown link: ${page.source}: ${node.url}`);
    }
  });
}
for await (const file of new Bun.Glob('**/*.html').scan(`out/rustdoc/${revision}`)) {
  await checkHtml(path.join(output, 'rustdoc', revision, file), `${basePath}/rustdoc/${revision}/${file}`, true);
}
await writeFile('.generated/rustdoc-link-errors.json', JSON.stringify(rustdocErrors, null, 2));
assert.equal(rustdocErrors.length, 0, 'Rustdoc hyperlink/anchor failures: ' + rustdocErrors.length + '; exact diagnostics in .generated/rustdoc-link-errors.json');
for await (const file of new Bun.Glob('**/*.css').scan('out')) {
  const css = await readFile(path.join(output, file), 'utf8');
  for (const [, href] of css.matchAll(/url\(["']?([^"')]+)["']?\)/g)) await checkUrl(href, `${basePath}/${file}`);
}
await checkHtml(path.join(output, 'index.html'), `${basePath}/`);
const missing = await checkHtml(path.join(output, '404.html'), `${basePath}/404.html`);
assert(missing('h1').text().includes('Không tìm thấy trang'));
await stat(path.join(output, '.nojekyll'));
const llms = await readFile('out/llms.txt', 'utf8');
assert(llms.includes('local/Git checkout'));
assert(llms.includes('no published') || llms.includes('No crates.io or hosted docs.rs'));
assert(llms.includes('Essential contracts:'));
const localLlms = await readFile(path.join(root, 'docs/llms.txt'), 'utf8');
const essential = localLlms.slice(localLlms.indexOf('Package map:'), localLlms.indexOf('For exact signatures,')).trim();
assert(llms.includes(essential), 'Public index must preserve reviewed package/essential contracts verbatim.');
for (const page of pages) assert(llms.includes(`](${origin}${page.markdownUrl})`), `Public LLM page missing: ${page.url}`);
for (const [, href] of llms.matchAll(/\]\(([^)]+)\)/g)) await checkUrl(href, `${basePath}/llms.txt`);
const originalFetch = globalThis.fetch;
try {
  // Exercise the same static search client against actual exported bytes, without a server.
  globalThis.fetch = Object.assign(async (input: RequestInfo | URL) => new Response(await readFile(exportedFile(new URL(String(input), origin)), 'utf8'), { headers: { 'Content-Type': 'application/json' } }), { preconnect: originalFetch.preconnect });
  for (const [locale, query] of [['en', 'cleanup'], ['vi', 'dọn dẹp']]) {
    const results = await staticClient({ from: `${basePath}/search-index/${locale}.json` }).search(query);
    assert(results.length > 0, `Static search returned no ${locale} results.`);
    for (const result of results) {
      assert(result.url.startsWith(`${basePath}/${locale}/docs/`), `Wrong search locale/subpath: ${result.url}`);
      assert(!result.url.includes('/plans/'));
      await checkUrl(result.url, `${basePath}/${locale}/docs/`);
    }
    console.log(`Static ${locale} search '${query}': ${results.length} results with correct routes.`);
  }
} finally { globalThis.fetch = originalFetch; }
assert.equal(unresolvedCounts.size, dependencyProseLinks.length, 'Dependency prose link set changed.');
    const unresolvedDependencyProse = [];
    for (const entry of dependencyProseLinks) {
      assert.equal(unresolvedCounts.get(entry.file + '\u0000' + entry.target), entry.count, 'Dependency prose reference count changed: ' + entry.file + ' → ' + entry.target);
      const sourceSha256 = createHash('sha256').update(await readFile(path.join(output, 'rustdoc', revision, entry.file))).digest('hex');
      unresolvedDependencyProse.push({ ...entry, sourceSha256 });
    }
    await writeFile('.generated/static-check.json', JSON.stringify({ revision, pages: pages.length, checkedLinks, checkedAssets, unresolvedDependencyProse }, null, 2));
    console.log('KNOWN BOUNDARY: 112 unresolved dependency prose links (54 Rust-path, 35 inherited-anchor, 23 inactive/incorrect targets; 76 targets, 32 files); ZERO framework/site exceptions. Exact entries/hashes: .generated/static-check.json.');
    console.log(`PASS: ${pages.length} canonical routes/Markdown files, ${checkedLinks} links/anchors, ${checkedAssets} asset references, complete exact Rustdoc and public llms.txt.`);
