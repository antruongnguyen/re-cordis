# Group API

[Tiếng Việt](../../vi/api/groups.md) | [API index](README.md)

`re_cordis_loader::Group` owns the upstream group satellite's structural
contract. `Group::new(Vec<EntrySpec>)` supplies ordered `entries`;
`EntrySpec::group` creates `EntryKind::Group`. There is no second lifecycle
registry or group facade package.

Use `Loader::create_in/move_entry/update/remove/settle/dispose`.
`EntrySnapshot.children` is ordered and `parent` is the current logical
owner. Nested groups remain present while their verdict disables descendants.
Whole runtime group input is preflighted; source updates preserve independently
valid child entries. Groups with condition/intercept callbacks reevaluate through an outer lifecycle
rebuild, so descendant Fiber IDs may change; logical EntryIds stay stable.
Same-group reorder is inert; cross-group/scope moves
preserve EntryId while rebuilding outer/inner Fiber allocations after cleanup.

Each retired generation has a retained shared barrier. Parent/loader disposal
joins it even when operation waiters and Loader handles have been dropped.
An adopted child is removed only by its current group.
Globally unique entry IDs and mutable group addresses deliberately replace
upstream nested-source-local IDs. No mutable prototype or core reparent API
is emulated. See [guide](../guides/groups.md), [loader API](loader.md) and
[source correspondence](../../plans/2026-11-02-cordis-completion/group.md).
