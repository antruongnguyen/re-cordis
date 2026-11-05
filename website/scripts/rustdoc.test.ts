import { expect, test } from 'bun:test';
import { runInNewContext } from 'node:vm';
import { emptyRegistration } from './rustdoc';

test('empty Rustdoc registration follows immediate and deferred compiler protocol', () => {
  let registered: unknown;
  runInNewContext(emptyRegistration('re_cordis_core'), { window: { register_implementors(value: unknown) { registered = value; } } });
  expect(JSON.stringify(registered)).toBe('{"re_cordis_core":[]}');
  const window: { pending_implementors?: unknown } = {};
  runInNewContext(emptyRegistration('futures_task'), { window });
  expect(JSON.stringify(window.pending_implementors)).toBe('{"futures_task":[]}');
});
