'use client';
import { RootProvider } from 'fumadocs-ui/provider/base';
import { useParams, usePathname, useRouter } from 'next/navigation';
import Link from 'next/link';
import { FrameworkProvider } from 'fumadocs-core/framework';
import { routerUrl } from '@/lib/urls';
import { useEffect, type ComponentProps, type ReactNode } from 'react';
import StaticSearchDialog from './search';
import { providerI18n } from '@/lib/i18n';
import settings from '@/.generated/settings.json';

// Public URLs already contain basePath; avoid applying Next's basePath a second time.
function SiteLink({ href = '', ...props }: ComponentProps<'a'> & { prefetch?: boolean }) {
  const target = routerUrl(href, settings.basePath);
  return <Link href={target || '/'} {...props} />;
}


function useSiteRouter() {
  const router = useRouter();
  return { push: (url: string) => router.push(routerUrl(url, settings.basePath)), refresh: () => router.refresh() };
}

export function Provider({ children }: { children: ReactNode }) {
  const params = useParams();
  const pathname = usePathname();
  const router = useRouter();
  const locale = params.lang === 'vi' ? 'vi' : 'en';
  useEffect(() => { document.documentElement.lang = locale; }, [locale]);
  return (
    <FrameworkProvider Link={SiteLink} useRouter={useSiteRouter} usePathname={usePathname} useParams={useParams}>
    <RootProvider search={{ SearchDialog: StaticSearchDialog }} i18n={{
      ...providerI18n(locale),
      onLocaleChange(next) {
        const route = pathname.replace(/^\/(en|vi)(?=\/|$)/, `/${next}`);
        router.push(route === pathname && !/^\/(en|vi)(?:\/|$)/.test(pathname) ? `/${next}/docs/` : route);
      },
    }}>
      {children}
    </RootProvider>
    </FrameworkProvider>
  );
}
