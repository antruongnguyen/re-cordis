import { notFound } from 'next/navigation';
import { DocsBody, DocsPage, DocsTitle, MarkdownCopyButton } from 'fumadocs-ui/layouts/docs/page';
import { pages, settings, rustdocCrate } from '@/lib/site';
import { CodeBlock, Pre } from 'fumadocs-ui/components/codeblock';
import { toJsx } from 'fumadocs-core/utils/hast-to-jsx';
import type { Root as HtmlRoot } from 'hast';

const findPage = (lang: string, slug: string[] = []) => pages.find((page) => page.locale === lang && page.url === `${settings.basePath}/${lang}/docs/${slug.length ? `${slug.join('/')}/` : ''}`);
export const dynamicParams = false;
export function generateStaticParams() { return pages.map((page) => ({ lang: page.locale, slug: page.file.replace(/\.md$/, '').split('/').filter((part) => part !== 'README') })); }

export async function generateMetadata({ params }: { params: Promise<{ lang: string; slug?: string[] }> }) {
  const { lang, slug } = await params;
  const page = findPage(lang, slug);
  if (!page) notFound();
  return { title: page.title, alternates: { canonical: settings.origin + page.url, languages: Object.fromEntries(['en', 'vi'].map((locale) => [locale, settings.origin + page.url.replace(`/${lang}/docs/`, `/${locale}/docs/`)])) } };
}

export default async function Page({ params }: { params: Promise<{ lang: string; slug?: string[] }> }) {
  const { lang, slug } = await params;
  const page = findPage(lang, slug);
  if (!page) notFound();
  const vi = lang === 'vi';
  const rustdoc = `${settings.basePath}/rustdoc/${settings.revision}/${rustdocCrate(page)}/index.html`;
  return <DocsPage id="main-content" tabIndex={-1} toc={page.toc}>
    <p className="documentation-status">{vi ? `Tài liệu hiện tại · alpha thử nghiệm ${settings.version} trên crates.io · dùng pin chính xác; Create cần checkout tương thích. Rust 1.85 trở lên; bằng chứng filesystem/watcher trên Apple Silicon macOS.` : `Current documentation · published experimental ${settings.version} on crates.io · use exact pins; Create needs a compatible checkout. Rust 1.85 minimum; filesystem/watcher runtime evidence on Apple Silicon macOS.`}</p>
    <DocsTitle id={page.titleId}>{page.title}</DocsTitle>
    <div className="mb-6 flex flex-wrap items-center gap-4 border-b border-fd-border pb-4 text-sm">
      <MarkdownCopyButton markdownUrl={page.markdownUrl} />
      <a className="underline" href={page.markdownUrl}>{vi ? 'Xem Markdown' : 'View Markdown'}</a>
      <a className="underline" href={page.sourceUrl}>{vi ? 'Nguồn tại revision build' : 'Source at build revision'}</a>
      {page.file.startsWith('api/') && <a className="underline" href={rustdoc}>{vi ? 'Rustdoc chính xác' : 'Exact Rustdoc'}</a>}
    </div>
    <DocsBody>{toJsx(page.body as HtmlRoot, {
      components: {
        pre: ({ ref, ...props }) => {
          void ref;
          return <CodeBlock {...props}><Pre>{props.children}</Pre></CodeBlock>;
        },
      },
    })}</DocsBody>
    {page.file === 'api/README.md' && <nav aria-label={vi ? 'Rustdoc các package' : 'Package Rustdoc'} className="mt-6 flex flex-wrap gap-4 text-sm">{['core', 'timer', 'logger', 'loader', 'include', 'hmr', 'create'].map((crate) => <a key={crate} className="underline" href={`${settings.basePath}/rustdoc/${settings.revision}/re_cordis_${crate}/index.html`}>re-cordis-{crate}</a>)}</nav>}
    <p className="mt-8 break-all text-xs text-fd-muted-foreground">{vi ? 'Revision build' : 'Build revision'}: {settings.revision}</p>
  </DocsPage>;
}
