# re-cordis documentation

## User documentation / Tài liệu người dùng

| Language / Ngôn ngữ | Guides and API reference / Hướng dẫn và API |
|---|---|
| English | [Start reading](en/README.md) |
| Tiếng Việt | [Bắt đầu đọc](vi/README.md) |
| AI agents | [llms.txt](llms.txt) |

These pages describe the current **unpublished, unstable `re-cordis-core`** API.
They are organized by user tasks, not implementation milestones. Both language
trees have matching paths, equivalent explanations, and identical executable
code. Each page links to its counterpart.

Các trang này mô tả API **`re-cordis-core` chưa phát hành, chưa ổn định** hiện tại.
Nội dung tổ chức theo công việc của người dùng, không theo cột mốc triển khai.
Hai cây ngôn ngữ có đường dẫn tương ứng, diễn giải tương đương và mã chạy giống
nhau. Mỗi trang liên kết đến bản ngôn ngữ còn lại.

## Exact API signatures

Generate Rustdoc locally with `cargo doc -p re-cordis-core --no-deps --locked`.
Open `target/doc/re_cordis_core/index.html`, adjusted for `CARGO_TARGET_DIR`.
The bilingual reference explains contracts and maps public exports; Rustdoc
supplies exact signatures and bounds. There is no published docs.rs release.

## Using the AI-agent index

Give your coding agent [docs/llms.txt](llms.txt) together with access to this
checkout. Ask it to follow the relevant English or Vietnamese guide and API
links before proposing application code. Relative links resolve from `docs/`;
the index is not self-contained API documentation.

The file is a retrieval aid following the [llms.txt proposal](https://llmstxt.org/),
not an authorization mechanism or replacement for repository instructions.
It does not grant network, credential, deployment, or mutation permissions.
Remote links to newly added pages are not assumed available before publication.

## Engineering references

The following existing English documents retain design history and acceptance
evidence. Historical “later work” statements must be read in their milestone
context; current capabilities are described by the new guides and current source.

| Documents | Purpose |
|---|---|
| [M1](m1.md), [M2](m2.md), [M3](m3.md), [M4](m4.md), [M5](m5.md), [M6](m6.md) | Hosting, effects/events, plugins, services, scoping/updates, final compatibility audit |
| [Lifecycle parity](lifecycle-parity.md) | Six executed paired scenarios and evidence boundaries |
| [GitHub activity](github-activity.md) | Application contracts, limits, and executable commands |
| [Limitations](limitation.md) | Known restrictions and unresolved tensions |
| [Decisions](decision.md) | Rationale and conventions |
| [Roadmap](todo.md) | Delivered and deferred scope |
| [Implementation plan](implementation-plan.md) | Pinned references, design gates, milestone sequence |
| [M0 results](m0.md), [case checklist](m0-cases.md) | Research evidence and source-case mapping |

## Maintaining these documents

Update both language versions in the same change. Keep API identifiers and
Rust/TOML/shell blocks identical; translate explanatory prose and use the paired
glossary. A new public export must appear in both API indexes and its chapter.
Add/update the corresponding `llms.txt` entry when reading paths change.
For each delivered satellite, add paired guide/API coverage and extend the
checker's currently core-only export and snippet-dependency handling. Keep
planned targets distinct from available APIs; see [the satellite roadmap](todo.md#next-implementation-targets-satellites).

Run the standard-library-only documentation checker (Python 3.9+) from the
repository root:

```sh
python3 -B tests/docs/test_check.py
python3 tests/docs/check.py --toolchain stable
python3 tests/docs/check.py --toolchain 1.85.0
```

It checks local file links/heading anchors, counterpart navigation, identical
code blocks, root-export/hook coverage, and the llms.txt structure. It explicitly
selects compiler/Cargo/rustdoc paths, builds core with the committed lockfile
offline into `target/stable` or `target/msrv`, then compiles and executes each
unique complete Rust documentation example with warnings denied.
The identical translated examples share that execution evidence.
It also runs the quick start as an external Cargo application using the shown
manifest, adjusting only the local core path. Its temporary lockfile is resolved
offline. Temporary application/example files are removed after the check.
Unchecked Rust attributes and unclosed triple-backtick fences are rejected in
both languages. The regression checks cover these rejection paths and broken
local file/heading links.

This is an opt-in documentation check; it does not modify the existing CI matrix.
It does not run shell examples, make authenticated requests, install packages,
or validate external URLs. If dependencies are missing, the offline build fails
explicitly; restore required dependencies deliberately before retrying.
Preserve [the existing acceptance checks](m6.md#verification) for behavior changes.
