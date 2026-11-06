import { expect, test } from 'bun:test';
import { runInNewContext } from 'node:vm';
import { emptyRegistration, dependencyProseLinksFor } from './rustdoc';

test('empty Rustdoc registration follows immediate and deferred compiler protocol', () => {
  let registered: unknown;
  runInNewContext(emptyRegistration('re_cordis_core'), { window: { register_implementors(value: unknown) { registered = value; } } });
  expect(JSON.stringify(registered)).toBe('{"re_cordis_core":[]}');
  const window: { pending_implementors?: unknown } = {};
  runInNewContext(emptyRegistration('futures_task'), { window });
  expect(JSON.stringify(window.pending_implementors)).toBe('{"futures_task":[]}');
});

// Exact parse_terminated code-header text from the accepted 1.97.1 and hosted 1.99.1 artifacts.
const tokenLink = '<a class="associatedtype" href="trait.Peek.html#associatedtype.Token" title="type syn::parse::Peek::Token">Token</a>';
const unlinkedSignature = `<section id="method.parse_terminated"><h4 class="code-header">pub fn parse_terminated&lt;T, P&gt;(
    &amp;'a self,
    parser: fn(ParseStream&lt;'a&gt;) -&gt; Result&lt;T&gt;,
    separator: P,
) -&gt; Result&lt;Punctuated&lt;T, P::Token&gt;&gt;<div class="where">where
    P: Peek,
    P::Token: Parse,</div></h4></section>`;
const linkedSignature = unlinkedSignature.replaceAll('P::Token', 'P::' + tokenLink);

function tokenCount(html: string) {
  return dependencyProseLinksFor(html).find((entry) => entry.file === 'syn/parse/struct.ParseBuffer.html' && entry.target === 'trait.Peek.html#associatedtype.Token')?.count;
}

test('recognize only the two plain hidden-token uses in the compiler signature', () => {
  expect(tokenCount(linkedSignature)).toBe(2);
  expect(tokenCount(unlinkedSignature)).toBeUndefined();
  expect(dependencyProseLinksFor(unlinkedSignature)).toHaveLength(96);
  expect(dependencyProseLinksFor(linkedSignature)).toHaveLength(97);
  // An omitted method, token use, partial conversion, or wrong signature retains the strict expectation.
  for (const html of ['', unlinkedSignature.replace('P::Token', ''), unlinkedSignature.replace('P::Token', 'P::' + tokenLink), unlinkedSignature.replace('parse_terminated&lt;', 'other&lt;')]) {
    expect(tokenCount(html)).toBe(2);
  }
});
