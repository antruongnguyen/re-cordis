# Include API

[Tiếng Việt](../../vi/api/include.md) | [English home](../README.md)

Generate exact signatures with `cargo doc --workspace --no-deps --locked`;
open `target/doc/re_cordis_include/index.html`. The API is local, unstable
and unpublished. [File guide](../guides/include.md).

| Export | Contract |
|---|---|
| `Include` | Dedicated owner and private loader; explicit source operations, immutable snapshots and joined disposal. |
| `IncludeOptions` | Native root path and optional missing-only initial JSON value. |
| `IncludeTask` | Waiter for admitted loader work; keep the original host alive through completion. |
| `Report` | Changed-source flag and independent failures; inspect failures after awaiting. |
| `IncludeError` | Message, optional real source location and original error cause. |
| `Location` | Canonical file path, structured raw path and optional SourceSpan. |
| `Owner`, `OwnerKind`, `Ownership` | Actual file/patch/embedded entry owner and per-key overrides; absent key ownership inherits the entry owner. |
| `ParsedDocument`, `parse_document` | Bounded strict JSON or restricted YAML value parsing with actual node locations, selected by .json/.yaml/.yml extension. |

`new` admits ownership before I/O. `refresh` reads and starts source
reconciliation; `entries`, `data` and `ownership` expose immutable
observations. `flush` writes admitted pending content synchronously;
`dispose` stops admission and joins teardown. `final_error` retains a final
write failure independently of a dropped waiter. Inspect core cleanup
reports/diagnostics in addition to include results.

| Exported bound | Value |
|---|---:|
| `MAX_BYTES` — source/journal bytes | 1048576 |
| `MAX_DEPTH` — value/include/group nesting | 32 |
| `MAX_NODES` — parsed value nodes | 65536 |
| `MAX_STRING_BYTES` — strings and keys | 65536 |
| `MAX_FILES` — canonical sources | 64 |
| `MAX_ENTRIES` — effective entries/pending records | 1024 |
| `MAX_PATCHES` — patches per file | 1024 |
| `MAX_RETRIES` — replacement attempts | 3 |

Node columns count Unicode scalars; JSON syntax-error columns count bytes.
Runtime input has no file span. Native paths follow the host OS; Windows path
syntax is not a Unix absolute path. Canonical source identity follows symlinks.

YAML supports JSON-compatible values with explicit rejection of aliases,
anchors, tags, duplicate keys and multiple documents. Canonical output is
pretty JSON; neither comments nor style are retained. Filesystem bounds do
not promise wall-clock deadlines or crash durability. See
[limitations](../../limitation.md#include-files-and-persistence).
