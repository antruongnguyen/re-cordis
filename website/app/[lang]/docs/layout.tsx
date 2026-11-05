import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { FullSearchTrigger } from 'fumadocs-ui/layouts/shared/slots/search-trigger';
import { LanguageSelect, LanguageSelectText } from 'fumadocs-ui/layouts/shared/slots/language-select';
import { ThemeSwitch } from 'fumadocs-ui/layouts/shared/slots/theme-switch';
import { settings, tree } from '@/lib/site';

export default async function Layout({ children, params }: { children: React.ReactNode; params: Promise<{ lang: string }> }) {
  const { lang } = await params;
  const home = `${settings.basePath}/${lang}/docs/`;
  const vi = lang === 'vi';
  return <>
    <a className="skip-link" href="#main-content">{vi ? 'Đến nội dung' : 'Skip to content'}</a>
    <header className="site-header">
      <a href={home} className="site-brand">re-cordis <span>0.0.0</span></a>
      <nav aria-label={vi ? 'Điều hướng chính' : 'Main navigation'} className="site-nav">
        <a href={home}>{vi ? 'Tài liệu' : 'Docs'}</a>
        <a href={home + 'guides/create/'}>Create</a>
        <a href={home + 'api/'}>API</a>
      </nav>
      <div className="site-tools">
        <FullSearchTrigger className="header-search" />
        <LanguageSelect><LanguageSelectText /></LanguageSelect>
        <a href={settings.github}>GitHub</a>
        <ThemeSwitch />
      </div>
    </header>
    <DocsLayout tree={tree(lang)} sidebar={{ collapsible: false }} nav={{ title: 're-cordis · 0.0.0', url: home }} githubUrl={settings.github}>
      {children}
    </DocsLayout>
  </>;
}
