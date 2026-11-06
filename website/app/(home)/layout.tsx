import { Provider } from '@/components/provider';
import { siteMetadata } from '@/lib/metadata';
import '../global.css';

export const metadata = siteMetadata;
export default function HomeLayout({ children }: { children: React.ReactNode }) {
  return <html lang="en" suppressHydrationWarning><body className="flex min-h-screen flex-col"><Provider>{children}</Provider></body></html>;
}
