import { load } from 'cheerio';
import dependencyProseLinks from './rustdoc-prose-links.json';

// Rustdoc 1.97.1 references these scripts but emits no out-of-crate records.
// This finite set was checked against a complete workspace + dependency build.
export const emptyImplementorScripts = [
  { file: 'trait.impl/futures_task/arc_wake/trait.ArcWake.js', owner: 'futures_task', sources: ['futures_task/trait.ArcWake.html', 'futures_util/task/trait.ArcWake.html'] },
  { file: 'trait.impl/yaml_rust2/parser/trait.EventReceiver.js', owner: 'yaml_rust2', sources: ['yaml_rust2/parser/trait.EventReceiver.html'] },
  { file: 'trait.impl/re_cordis_core/service/trait.ServiceInvoke.js', owner: 're_cordis_core', sources: ['re_cordis_core/trait.ServiceInvoke.html'] },
  { file: 'trait.impl/serde_core/de/trait.VariantAccess.js', owner: 'serde_core', sources: ['serde_core/de/trait.VariantAccess.html'] },
];

export function emptyRegistration(owner: string): string {
  return `(function() {
    const implementors = Object.fromEntries(${JSON.stringify([[owner, []]])});
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()\n`;
}

// Rustdoc 1.97.1 links two hidden Peek::Token uses; 1.99.1 renders them as plain text.
// Recognize that exact signature only; every other omission/count change remains an error.
export function dependencyProseLinksFor(parseBufferHtml: string) {
  const html = load(parseBufferHtml);
  const header = html('[id="method.parse_terminated"] h4.code-header');
  const target = 'trait.Peek.html#associatedtype.Token';
  const signature = "pub fn parse_terminated<T, P>( &'a self, parser: fn(ParseStream<'a>) -> Result<T>, separator: P, ) -> Result<Punctuated<T, P::Token>>where P: Peek, P::Token: Parse,";
  const unlinkedTokens = header.length === 1
    && header.text().replace(/\s+/g, ' ').trim() === signature
    && (header.html()?.match(/P::Token/g) ?? []).length === 2
    && html('a').toArray().every((element) => html(element).attr('href') !== target);
  return unlinkedTokens
    ? dependencyProseLinks.filter((entry) => !(entry.file === 'syn/parse/struct.ParseBuffer.html' && entry.target === target))
    : dependencyProseLinks;
}
