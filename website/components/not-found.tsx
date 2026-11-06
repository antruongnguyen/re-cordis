import Link from 'next/link';

export default function NotFound() {
  return <main className="mx-auto flex max-w-xl flex-1 flex-col justify-center gap-5 p-6">
    <h1 className="text-3xl font-semibold">Page not found · Không tìm thấy trang</h1>
    <p>This development documentation page may have moved. Choose a language, then use the sidebar or search to find the current guide.</p>
    <p lang="vi">Trang tài liệu phát triển có thể đã chuyển. Chọn ngôn ngữ rồi dùng thanh bên hoặc tìm kiếm để tìm hướng dẫn hiện tại.</p>
    <nav className="flex gap-6" aria-label="Return to documentation"><Link className="underline" href="/en/docs/">English</Link><Link className="underline" href="/vi/docs/">Tiếng Việt</Link></nav>
  </main>;
}
