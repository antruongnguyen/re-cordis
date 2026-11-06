import { Provider } from '@/components/provider';
import NotFound from '@/components/not-found';
import { siteMetadata } from '@/lib/metadata';
import './global.css';

export const metadata = { ...siteMetadata, title: 'Page not found · Không tìm thấy trang' };
export default function GlobalNotFound() {
  return <html lang="en" suppressHydrationWarning><body className="flex min-h-screen flex-col"><Provider><NotFound /></Provider></body></html>;
}
