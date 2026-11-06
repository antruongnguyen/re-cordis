import type { Metadata } from 'next';
import settings from '@/.generated/settings.json';

export const siteMetadata: Metadata = {
  title: { default: 're-cordis documentation', template: '%s | re-cordis' },
  description: 'Development documentation for the unpublished Rust Cordis framework. Local checkout use; Rust 1.85 minimum.',
  icons: { icon: { url: `${settings.basePath}/icon.svg`, type: 'image/svg+xml' } },
};
