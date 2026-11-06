import Link from 'next/link';
import settings from '@/.generated/settings.json';

export default function Home() {
  return <main className="mx-auto flex max-w-xl flex-1 flex-col justify-center gap-6 p-6">
    <h1 className="text-3xl font-semibold">re-cordis</h1>
    <p>Current documentation for experimental re-cordis {settings.version}, published on crates.io. Use exact pins; Rust 1.85 or newer. Create requires a compatible local checkout.</p>
    <p lang="vi">Tài liệu hiện tại cho re-cordis {settings.version} thử nghiệm, đã phát hành trên crates.io. Dùng pin chính xác; Rust 1.85 trở lên. Create cần checkout cục bộ tương thích.</p>
    <nav aria-label="Documentation language" className="flex gap-6"><Link className="underline" href="/en/docs/">English documentation</Link><Link lang="vi" className="underline" href="/vi/docs/">Tài liệu tiếng Việt</Link></nav>
  </main>;
}
