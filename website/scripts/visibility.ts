import path from 'node:path';
import type { Root, RootContent } from 'mdast';
import type { Parent } from 'unist';
import { toString } from 'mdast-util-to-string';
import { visit } from 'unist-util-visit';

export const internalDocumentationPath = /\bdocs[\\/](?:plans|superpowers)(?:[\\/]|\b)/;

export function isInternalDocumentation(href: string, source: string, github: string): boolean {
  if (/^(?:https?:)?\/\//.test(href)) {
    const url = new URL(href, github);
    const repository = new URL(github).pathname;
    const ownSource = (url.hostname === 'github.com' || url.hostname === 'raw.githubusercontent.com')
      && url.pathname.startsWith(repository + '/');
    return ownSource && internalDocumentationPath.test(decodeURIComponent(url.pathname));
  }
  if (/^(?:[a-z][a-z\d+.-]*:|#)/i.test(href)) return false;
  const pathname = decodeURIComponent(href.split(/[?#]/)[0]);
  const local = path.posix.normalize(pathname.startsWith('/') ? pathname.slice(1) : path.posix.join(path.posix.dirname(source), pathname));
  return /^(?:docs\/(?:plans|superpowers))(?:\/|$)/.test(local);
}

// Remove publication-only links without changing canonical prose or executable code.
export function publicDocumentation(tree: Root, source: string, github: string): void {
  const definitions = new Set<string>();
  visit(tree, 'definition', (node) => {
    if (isInternalDocumentation(node.url, source, github)) definitions.add(node.identifier);
  });
  function filter(parent: Parent) {
    parent.children = parent.children.flatMap((child) => {
      const node = child as RootContent;
      if ('children' in node) filter(node);
      const hidden = (('url' in node) && isInternalDocumentation(node.url, source, github))
        || ((node.type === 'linkReference' || node.type === 'imageReference') && definitions.has(node.identifier));
      if (hidden) {
        if ((node.type === 'link' || node.type === 'linkReference') && !internalDocumentationPath.test(toString(node))) return node.children;
        return [];
      }
      return [node];
    });
  }
  filter(tree);
}
