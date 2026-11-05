import { notFound } from 'next/navigation';
import { Provider } from '@/components/provider';
import { siteMetadata } from '@/lib/metadata';
import '../global.css';

export const metadata = siteMetadata;
export const dynamicParams = false;
export function generateStaticParams() { return [{ lang: 'en' }, { lang: 'vi' }]; }

export default async function LanguageLayout({ children, params }: { children: React.ReactNode; params: Promise<{ lang: string }> }) {
  const { lang } = await params;
  if (lang !== 'en' && lang !== 'vi') notFound();
  return <html lang={lang} suppressHydrationWarning><body className="flex min-h-screen flex-col"><Provider>{children}</Provider></body></html>;
}
