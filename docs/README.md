# re-cordis documentation

## User documentation / Tài liệu người dùng

| Language / Ngôn ngữ | Guides and API reference / Hướng dẫn và API |
|---|---|
| English | [Start reading](en/README.md) |
| Tiếng Việt | [Bắt đầu đọc](vi/README.md) |
| AI agents | [llms.txt](llms.txt) |

These pages describe the current **unpublished, unstable `re-cordis-core`,
`re-cordis-timer`, `re-cordis-logger`, `re-cordis-loader`, `re-cordis-include`,
`re-cordis-hmr` and selected offline `re-cordis-create`** APIs.
They are organized by user tasks, not implementation milestones. Both language
trees have matching paths, equivalent explanations, and identical executable
code. Each page links to its counterpart.

Các trang này mô tả API **`re-cordis-core`, `re-cordis-timer`, `re-cordis-logger`, `re-cordis-loader`, `re-cordis-include`, `re-cordis-hmr` và `re-cordis-create`
ngoại tuyến đã chọn chưa phát hành,
chưa ổn định** hiện tại.
Nội dung tổ chức theo công việc của người dùng, không theo cột mốc triển khai.
Hai cây ngôn ngữ có đường dẫn tương ứng, diễn giải tương đương và mã chạy giống
nhau. Mỗi trang liên kết đến bản ngôn ngữ còn lại.

## Exact API signatures

Generate Rustdoc locally with `cargo doc --workspace --no-deps --locked`.
Open `target/doc/re_cordis_core/index.html` or
`target/doc/re_cordis_timer/index.html`, `target/doc/re_cordis_logger/index.html`
or `target/doc/re_cordis_loader/index.html`, `target/doc/re_cordis_include/index.html`,
`target/doc/re_cordis_hmr/index.html`, `target/doc/re_cordis_create/index.html`,
adjusted for `CARGO_TARGET_DIR`.
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
| [Release readiness](release-readiness.md) | Current local/Git-only verdict, package audit and proposed experimental release |
| [Limitations](limitation.md) | Known restrictions and unresolved tensions |
| [Decisions](decision.md) | Rationale and conventions |
| [Roadmap](todo.md) | Delivered and deferred scope |
| [Completion plan](plans/2026-11-02-cordis-completion/README.md) | Core/timer repair, all seven satellites, decisions, acceptance and scheduling |
| [Implementation plan](implementation-plan.md) | Pinned references, design gates, milestone sequence |
| [M0 results](m0.md), [case checklist](m0-cases.md) | Research evidence and source-case mapping |

## Maintaining these documents

Update both language versions in the same change. Keep API identifiers and
Rust/TOML/shell blocks identical; translate explanatory prose and use the paired
glossary. A new public export must appear in both API indexes and its chapter.
Add/update the corresponding `llms.txt` entry when reading paths change.
For each delivered satellite, add paired guide/API coverage and extend the
checker's core/timer/logger/loader/include/HMR/Create export and snippet-dependency handling. Keep
planned targets distinct from available APIs; see [the satellite roadmap](todo.md#next-implementation-targets-satellites).

Run the standard-library-only documentation checker (Python 3.9+) from the
repository root:

```sh
python3 -B tests/docs/test_check.py
python3 tests/docs/check.py --toolchain stable
python3 tests/docs/check.py --toolchain 1.85.0
```

It checks local file links/heading anchors, counterpart navigation, identical
code blocks, root-export/hook coverage, and the llms.txt structure. Local-link
coverage includes every Markdown file under docs/plans (including nested
evidence), release-readiness.md, implementation-plan.md and root AGENTS.md.
It validates inline Markdown file/heading links, not bare code-formatted paths
or external URL availability. A regression proves missing nested plan/evidence
targets and anchors fail through the normal structure-check entry point. It explicitly
selects compiler/Cargo/rustdoc paths, builds core, timer, logger, loader, include,
HMR (with its optional Include adapter) and Create with the committed lockfile
offline into `target/stable` or `target/msrv`, then compiles and executes each
unique complete Rust documentation example with warnings denied.
The identical translated examples share that execution evidence.
It also runs the quick start as an external Cargo application using the shown
manifest, adjusting only the local core path. Its temporary lockfile is resolved
offline. Temporary application/example files are removed after the check.
Unchecked Rust attributes and unclosed triple-backtick fences are rejected in
both languages. The regression checks cover these rejection paths and broken
local file/heading links.

Create checks run the actual generator through argument arrays, compare documented
options with live CLI help, verify the five generated files and Cargo metadata,
then build/test/run the external application and check strict Clippy
with each selected toolchain. Formatting uses explicitly resolved stable tools,
following the shared acceptance policy. Dependency resolution is offline. This executes
the selected local scaffolding path, not remote templates or cross-platform I/O.

After those basic-template checks, the same test-owned external project is
extended with explicit local timer/Include/HMR dependencies and the exact shared
R0 example/application/test files. Build/test/run and strict Clippy use both
selected toolchains; formatting remains stable-only. See the
[integration guide](en/guides/system-lifecycle.md) and
[executed R0 evidence](plans/2026-11-02-cordis-completion/evidence/r0-acceptance.md). No production template variant
is added, and no temporary fixture source becomes a public API.

This is an opt-in documentation check; it does not modify the existing CI matrix.
The inspected .github/workflows/ci.yml defines Ubuntu stable/1.85.0 Rust jobs,
stable-only lint/formatting and workspace/M0 tests, but neither Python
documentation regression nor guide execution. Adding those is recommended
with explicit dependency restoration and per-toolchain results; implementation
and wider platform acceptance are deferred by the bounded B1 decision.
Other shell examples are not executed. It does not make authenticated requests, install packages,
or validate external URLs. If dependencies are missing, the offline build fails
explicitly; restore required dependencies deliberately before retrying.
Preserve [the existing acceptance checks](m6.md#verification) for behavior changes.
