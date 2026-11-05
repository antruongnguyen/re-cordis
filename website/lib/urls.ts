// Fumadocs stores published URLs; Next applies basePath during navigation.
export function routerUrl(url: string, basePath: string): string {
  return basePath && url.startsWith(`${basePath}/`) ? url.slice(basePath.length) : url;
}
