import path from 'node:path';

export const locales = ['en', 'vi'] as const;
export type Locale = (typeof locales)[number];

export function slugFor(file: string): string[] {
  return file.replace(/\.md$/, '').split('/').filter((part) => part !== 'README');
}

export function pageUrl(basePath: string, locale: string, file: string): string {
  const slug = slugFor(file);
  return `${basePath}/${locale}/docs/${slug.length ? `${slug.join('/')}/` : ''}`;
}

export function markdownUrl(basePath: string, locale: string, file: string): string {
  const slug = slugFor(file);
  return `${basePath}/markdown/${locale}/${slug.length ? slug.join('/') : 'index'}.md`;
}

export function repository(remote: string): { github: string; origin: string; basePath: string } {
  const match = remote.match(/^(?:https:\/\/github\.com\/|git@github\.com:)([^/]+)\/([^/]+?)(?:\.git)?$/);
  if (!match) throw new Error('A GitHub origin is required to infer the Pages project path.');
  const [, owner, repo] = match;
  return {
    github: `https://github.com/${owner}/${repo}`,
    origin: `https://${owner}.github.io`,
    basePath: repo.toLowerCase() === `${owner}.github.io`.toLowerCase() ? '' : `/${repo}`,
  };
}

export function mapLink(
  href: string,
  source: string,
  canonical: Set<string>,
  settings: { basePath: string; github: string; revision: string },
  image = false,
): { url: string; local?: string } {
  if (/^(?:[a-z][a-z\d+.-]*:|\/\/|#)/i.test(href)) return { url: href };
  const match = href.match(/^([^?#]*)(\?[^#]*)?(#.*)?$/);
  if (!match) throw new Error(`Cannot parse link: ${href}`);
  const [, pathname, query = '', anchor = ''] = match;
  const local = path.posix.normalize(pathname.startsWith('/')
    ? decodeURIComponent(pathname.slice(1))
    : path.posix.join(path.posix.dirname(source), decodeURIComponent(pathname)));
  if (local.startsWith('../')) throw new Error(`Link escapes the checkout: ${source}: ${href}`);
  if (!image && canonical.has(local)) {
    const [, locale, ...parts] = local.split('/');
    return { url: pageUrl(settings.basePath, locale, parts.join('/')) + query + anchor, local };
  }
  const encoded = local.split('/').map(encodeURIComponent).join('/');
  return {
    url: image
      ? `${settings.github.replace('https://github.com/', 'https://raw.githubusercontent.com/')}/${settings.revision}/${encoded}${query}${anchor}`
      : `${settings.github}/blob/${settings.revision}/${encoded}${query}${anchor}`,
    local,
  };
}
