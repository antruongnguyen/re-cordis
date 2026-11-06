import settings from '@/.generated/settings.json';
import pages from '@/.generated/pages.json';
import type { Root, Node } from 'fumadocs-core/page-tree';

export { settings, pages };
export type Page = (typeof pages)[number];

const groups = [
  ['Getting started', 'Bắt đầu', ['README.md', 'getting-started.md', 'concepts.md', 'glossary.md']],
  ['Core', 'Core', ['guides/plugins-and-updates.md', 'guides/services-and-scoping.md', 'guides/events.md', 'guides/effects-and-cleanup.md', 'guides/logging.md']],
  ['Packages', 'Các package', ['guides/timers.md', 'guides/console-logging.md', 'guides/loader.md', 'guides/groups.md', 'guides/include.md', 'guides/hmr.md', 'guides/create.md']],
  ['Practical guides', 'Hướng dẫn thực tế', ['guides/system-lifecycle.md', 'guides/examples.md', 'guides/testing-and-troubleshooting.md', 'guides/documentation.md']],
  ['API contracts', 'Hợp đồng API', ['api/README.md', 'api/runtime-context.md', 'api/plugins-registry.md', 'api/services-reflection.md', 'api/events.md', 'api/effects.md', 'api/logging.md', 'api/errors-hooks.md', 'api/timer.md', 'api/console-logger.md', 'api/loader.md', 'api/groups.md', 'api/include.md', 'api/hmr.md', 'api/create.md']],
] as const;

export function tree(locale: string): Root {
  const localized = pages.filter((page) => page.locale === locale);
  const children: Node[] = groups.map(([en, vi, files]) => ({
    type: 'folder', name: locale === 'vi' ? vi : en,
    defaultOpen: true,
    children: files.flatMap((file) => {
      const page = localized.find((page) => page.file === file);
      return page ? [{ type: 'page' as const, name: page.title, url: page.url }] : [];
    }),
  }));
  const compatibility = [
    ['Current distribution', 'Phân phối hiện tại', 'docs/release-readiness.md'],
    ['Compatibility and platform limits', 'Tương thích và giới hạn nền tảng', 'docs/limitation.md'],
    ['Changelog', 'Lịch sử thay đổi', 'CHANGELOG.md'],
  ];
  children.push({ type: 'folder', name: locale === 'vi' ? 'Tương thích và phát hành' : 'Compatibility and releases', defaultOpen: true, children: compatibility.map(([en, vi, file]) => ({ type: 'page', name: locale === 'vi' ? vi : en, url: `${settings.github}/blob/${settings.revision}/${file}`, external: true })) });
  return { name: 're-cordis', children };
}

export function rustdocCrate(page: Page): string {
  const slug = page.file.replace(/^api\//, '').replace('.md', '');
  const packages: Record<string, string> = { timer: 'timer', 'console-logger': 'logger', loader: 'loader', groups: 'loader', include: 'include', hmr: 'hmr', create: 'create' };
  return `re_cordis_${packages[slug] ?? 'core'}`;
}
