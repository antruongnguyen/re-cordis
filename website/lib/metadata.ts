import type { Metadata } from 'next';
import settings from '@/.generated/settings.json';

export const siteMetadata: Metadata = {
  title: { default: 're-cordis documentation', template: '%s | re-cordis' },
  description: `Current documentation for published experimental re-cordis ${settings.version}. Exact crates.io pins; Create requires a compatible checkout. Rust 1.85 minimum.`,
  icons: { icon: { url: `${settings.basePath}/icon.svg`, type: 'image/svg+xml' } },
};
