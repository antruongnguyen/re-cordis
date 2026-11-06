import path from 'node:path';
import settings from './.generated/settings.json' with { type: 'json' };

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  outputFileTracingRoot: path.resolve(import.meta.dirname, '..'),
  trailingSlash: true,
  basePath: settings.basePath,
  images: { unoptimized: true },
  reactStrictMode: true,
  experimental: { cpus: 2, globalNotFound: true },
};
export default nextConfig;
