import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { settings, tree } from '@/lib/site';

export default async function Layout({ children, params }: { children: React.ReactNode; params: Promise<{ lang: string }> }) {
  const { lang } = await params;
  return <>
    <a className="skip-link" href="#main-content">{lang === 'vi' ? 'Đến nội dung' : 'Skip to content'}</a>
    <DocsLayout tree={tree(lang)} nav={{ title: 're-cordis · 0.0.0', url: `${settings.basePath}/${lang}/docs/` }} githubUrl={settings.github}>
      {children}
    </DocsLayout>
  </>;
}
