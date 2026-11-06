import Link from 'next/link';
import settings from '@/.generated/settings.json';

export default function Home() {
  return <main className="mx-auto flex max-w-xl flex-1 flex-col justify-center gap-6 p-6">
    <h1 className="text-3xl font-semibold">re-cordis</h1>
    <p>Development documentation for the unpublished re-cordis {settings.version} candidate. Use a reviewed local checkout; Rust 1.85 or newer.</p>
    <p lang="vi">Tài liệu phát triển cho bản ứng viên re-cordis {settings.version}, hiện chưa phát hành. Dùng checkout đã review; Rust 1.85 trở lên.</p>
    <nav aria-label="Documentation language" className="flex gap-6"><Link className="underline" href="/en/docs/">English documentation</Link><Link lang="vi" className="underline" href="/vi/docs/">Tài liệu tiếng Việt</Link></nav>
  </main>;
}
