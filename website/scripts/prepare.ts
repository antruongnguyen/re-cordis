import { TOML } from 'bun';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFile, writeFile, mkdir, rm, stat } from 'node:fs/promises';
import path from 'node:path';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkRehype from 'remark-rehype';
import remarkStringify from 'remark-stringify';
import rehypeSlug from 'rehype-slug';
import { rehypeCode } from 'fumadocs-core/mdx-plugins/rehype-code';
import { createSearchAPI } from 'fumadocs-core/search/server';
import { toString } from 'mdast-util-to-string';
import { toText } from 'hast-util-to-text';
import { visit } from 'unist-util-visit';
import type { Root as MdRoot } from 'mdast';
import type { Root as HtmlRoot } from 'hast';
import { locales, mapLink, markdownUrl, pageUrl, repository } from './routes';
import { publicDocumentation } from './visibility';

const root = path.resolve('..');
const git = (...args: string[]) => execFileSync('git', args, { cwd: root, encoding: 'utf8' }).trim();
const revision = git('rev-parse', 'HEAD');
if (process.env.CORDIS_REVISION && process.env.CORDIS_REVISION !== revision) {
  throw new Error('CORDIS_REVISION must equal the current full HEAD; mixed revision builds are refused.');
}
const manifest = await readFile(path.join(root, 'Cargo.toml'), 'utf8');
const version = (TOML.parse(manifest) as { workspace: { package: { version: string } } }).workspace.package.version;
if (version !== '0.1.0-alpha.1') throw new Error('Verify and update published release facts before documenting another version.');
const settings = { ...repository(git('remote', 'get-url', 'origin')), revision, version, status: 'experimental-alpha-published', releaseRevision: '84fa9bdf0eb4e631fce792bf163494b0a61aeba8' };
const rustdocDir = process.env.CORDIS_RUSTDOC_DIR;
if (!rustdocDir) throw new Error('Set CORDIS_RUSTDOC_DIR to freshly generated workspace --all-features Rustdoc including dependencies. See README.md.');
const crates = ['core', 'timer', 'logger', 'loader', 'include', 'hmr', 'create'];
for (const crate of crates) {
  await stat(path.resolve(rustdocDir, `re_cordis_${crate}/index.html`));
}
const hmr = await readFile(path.resolve(rustdocDir, 're_cordis_hmr/struct.FileWatcher.html'), 'utf8');
if (!hmr.includes('id="method.watch_include"')) throw new Error('Rustdoc must include the optional HMR Include API (--all-features).');

await rm('.generated', { recursive: true, force: true });
await rm('public', { recursive: true, force: true });
await mkdir('.generated', { recursive: true });
await mkdir('public/search-index', { recursive: true });
const canonical = new Set<string>();
for (const locale of locales) {
  for await (const file of new Bun.Glob('**/*.md').scan(path.join(root, 'docs', locale))) canonical.add(`docs/${locale}/${file}`);
}
const english = [...canonical].filter((file) => file.startsWith('docs/en/')).map((file) => file.slice(8)).sort();
const vietnamese = [...canonical].filter((file) => file.startsWith('docs/vi/')).map((file) => file.slice(8)).sort();
if (JSON.stringify(english) !== JSON.stringify(vietnamese)) throw new Error('EN/VI documentation files differ.');
const pages = [];
const sources: Record<string, string> = { 'Cargo.toml': createHash('sha256').update(manifest).digest('hex') };
const linkSources = new Set<string>();
const markdownProcessor = unified().use(remarkParse).use(remarkGfm).use(remarkStringify, { fences: true });
const htmlProcessor = unified().use(remarkRehype).use(rehypeSlug).use(rehypeCode, {
  themes: { light: 'github-light', dark: 'github-dark' },
});
for (const source of [...canonical].sort()) {
  const locale = source.split('/')[1];
  const file = source.slice(8);
  const raw = await readFile(path.join(root, source), 'utf8');
  sources[source] = createHash('sha256').update(raw).digest('hex');
  const md = markdownProcessor.parse(raw) as MdRoot;
  const heading = md.children.find((node) => node.type === 'heading' && node.depth === 1);
  if (!heading) throw new Error(`Missing page title: ${source}`);
  const title = toString(heading);
  publicDocumentation(md, source, settings.github);
  visit(md, (node) => {
    if (node.type !== 'link' && node.type !== 'image' && node.type !== 'definition') return;
    const mapped = mapLink(node.url, source, canonical, settings, node.type === 'image');
    node.url = mapped.url;
    if (mapped.local) linkSources.add(mapped.local);
  });
  const markdown = markdownProcessor.stringify(md);
  const htmlTree = await htmlProcessor.run(md) as HtmlRoot;
  const toc: { title: string; url: string; depth: number }[] = [];
  let titleId = '';
  visit(htmlTree, 'element', (node) => {
    if (!/^h[1-6]$/.test(node.tagName)) return;
    const depth = Number(node.tagName[1]);
    if (depth === 1 && !titleId) titleId = String(node.properties.id);
    else toc.push({ title: toText(node), url: `#${node.properties.id}`, depth });
  });
  htmlTree.children = htmlTree.children.filter((node) => !(node.type === 'element' && node.tagName === 'h1'));
  const url = pageUrl(settings.basePath, locale, file);
  const mdUrl = markdownUrl(settings.basePath, locale, file);
  const output = path.join('public', mdUrl.slice(settings.basePath.length + 1));
  await mkdir(path.dirname(output), { recursive: true });
  await writeFile(output, `<!-- Current documentation; published experimental ${version}. Release source ${settings.releaseRevision}. Build revision ${revision}. -->\n<!-- Page: ${settings.origin}${url} -->\n\n${markdown}`);
  pages.push({ locale, file, source, title, titleId, body: htmlTree, toc, url, markdownUrl: mdUrl, content: toText(htmlTree), sourceUrl: `${settings.github}/blob/${revision}/${source}` });
}
for (const source of linkSources) {
  await stat(path.join(root, source));
  sources[source] ??= createHash('sha256').update(await readFile(path.join(root, source))).digest('hex');
}
for (const locale of locales) {
  const search = createSearchAPI('simple', { indexes: pages.filter((page) => page.locale === locale).map((page) => ({ title: page.title, content: page.content, url: page.url })) });
  await writeFile(`public/search-index/${locale}.json`, JSON.stringify(await search.export()));
}
const constraints = await readFile(path.join(root, 'docs/llms.txt'), 'utf8');
const preamble = constraints.slice(constraints.indexOf('TypeScript at'), constraints.indexOf('All crates are')).trim();
const start = constraints.indexOf('Package map:');
    const end = constraints.indexOf('For exact signatures,');
    const constraintText = start >= 0 && end > start ? constraints.slice(start, end).trim() : '';
    if (!constraintText) throw new Error('Local docs/llms.txt must expose its essential contracts.');
    const publicConstraints = (preamble + '\n\n' + constraintText).replace(/\[([^\]]+)\]\(([^)]+)\)/g, (_, label: string, href: string) => `[${label}](${mapLink(href, 'docs/llms.txt', canonical, settings).url})`);
let llms = `# re-cordis\n\n> Current documentation for published experimental ${version} packages on crates.io. Use exact pins. Rust 1.85 minimum; runtime filesystem/watcher evidence on tested Apple Silicon macOS only. Create requires a compatible complete checkout.\n\nRelease source: ${settings.releaseRevision}. Current guides include later documentation updates; frozen preparation statements are historical. Exact-version Rustdoc: https://docs.rs/re-cordis-core/${version}/re_cordis_core/\n\nBuild revision: ${revision}. Exact local Rustdoc: ${settings.origin}${settings.basePath}/rustdoc/${revision}/re_cordis_core/index.html\n\n${publicConstraints}\n`;
for (const locale of locales) {
  llms += `\n## ${locale === 'en' ? 'English' : 'Tiếng Việt'}\n\n`;
  for (const page of pages.filter((page) => page.locale === locale)) llms += `- [${page.title}](${settings.origin}${page.markdownUrl}): ${settings.origin}${page.url}\n`;
}
await writeFile('public/llms.txt', llms);
await writeFile('public/.nojekyll', '');
await writeFile('.generated/settings.json', JSON.stringify(settings, null, 2));
await writeFile('.generated/pages.json', JSON.stringify(pages));
await writeFile('.generated/provenance.json', JSON.stringify({ ...settings, dirty: Boolean(git('status', '--porcelain')), sources, rustdocInput: path.resolve(rustdocDir), crates }, null, 2));
console.log(`Prepared ${pages.length} canonical pages (${english.length} pairs), two static search indexes, Markdown and llms.txt at ${settings.basePath}; revision ${revision}.`);
