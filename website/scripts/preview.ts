import { readFile } from 'node:fs/promises';
import path from 'node:path';

const { basePath } = JSON.parse(await readFile('out/build-provenance.json', 'utf8'));
const root = path.resolve('out');
const server = Bun.serve({
  hostname: '127.0.0.1',
  port: Number(process.env.PORT ?? 4173),
  async fetch(request) {
    const url = new URL(request.url);
    if (url.pathname === basePath && basePath) return Response.redirect(`${url.origin}${basePath}/`, 308);
    if (!url.pathname.startsWith(`${basePath}/`)) return new Response('Outside documentation project path.', { status: 404 });
    const pathname = decodeURIComponent(url.pathname.slice(basePath.length));
    const file = path.resolve(root, `.${pathname}${pathname.endsWith('/') ? 'index.html' : ''}`);
    if (!file.startsWith(`${root}/`)) return new Response('Invalid path.', { status: 400 });
    const asset = Bun.file(file);
    if (await asset.exists()) return new Response(asset);
    return new Response(Bun.file(path.join(root, '404.html')), { status: 404, headers: { 'Content-Type': 'text/html; charset=utf-8' } });
  },
});
console.log(`Previewing exported files at http://${server.hostname}:${server.port}${basePath}/ (Ctrl-C to stop).`);
