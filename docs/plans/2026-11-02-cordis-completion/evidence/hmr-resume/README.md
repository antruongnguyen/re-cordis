# Tiếp tục HMR H0–H3

Tạm dừng theo yêu cầu người dùng ngày 2026-11-03 (Asia/Ho_Chi_Minh).
Repository: `/Users/I756434/Projects/private/cordis-rust`.
Nhánh: `feat/cordis-completion`. Checkpoint trước phiên: `47042d9`.
Commit chứa handoff này là checkpoint tạm dừng. HMR **chưa được nghiệm thu**.

## Bắt đầu lại

Đọc AGENTS.md, kiểm tra branch/HEAD/status và giữ mọi thay đổi mới hơn checkpoint.
Đọc [progress](../../progress.md), [hợp đồng](../hmr-contract.md),
[H0 review](../hmr-h0-review.md), [H2 review](../hmr-reload-review.md)
và các evidence được liên kết dưới đây. Giao tiếp với người dùng bằng tiếng Việt.
Không tiếp tục create S0–S2 hoặc whole-program R0/R1; không push/publish.

Các worker của phiên đã được yêu cầu dừng. Phiên sau dùng context mới:
`fork_turns=none`, `gpt-6.1-sol/high` cho implementation,
`gpt-6-astra/high` cho review độc lập. Phân quyền file rõ ràng và tuần tự hóa
việc ghi file chung; không dựa vào mailbox hay lịch sử worker cũ.

## Trạng thái và việc còn lại

| Phần | Đã có | Cần tiếp tục |
|---|---|---|
| H0 | Hợp đồng và dependency gate được review; commit `5065adc`. notify 8.2.0/8.2.1 qua MSRV nhưng không có portable PollWatcher join; chọn stdlib polling có JoinHandle. | Giữ các quyết định; không mở lại audit B0/prerequisite. |
| H1 | Crate watcher, exact token, queue/overflow/debounce, typed events/restart-required, local hosted callbacks, joined producer/task cleanup; 27 test + 1 Rustdoc qua stable/1.85.0. | Review độc lập watcher cùng adapter H3; adapter Include chưa viết. |
| H2 | Batch loader, explicit ReloadPlan, giữ config/provider, stale-intent guards, nested/recovery tests; loader checkpoint `bbef48b` và correction hiện tại. | **Sửa H2-R1** rồi re-review đúng delta; chưa chấp thuận H2. |
| H3 Include factory seam | `Include::replace_factories` và `ReloadPlan::replace_include` đã triển khai, giữ journal/data/provenance/file bytes; 2 test mới qua cả hai toolchain. | Giữ scope hẹp, review trong H2 delta. |
| H3 watch adapter | Thiết kế đã lưu; 9 test và ví dụ offline đã soạn nhưng chưa chạy. | Thêm `watch_include`, giữ/join IncludeTask đã admit; không gọi Include mới lúc cleanup; quota tính active+retiring. |
| H3 docs/evidence | 31 cặp trang, 19 HMR exports, 13 snippet chạy trên cả hai toolchain; map đủ 53 TS declarations. | Cập nhật API/status/source-map theo code cuối, gắn test thực tế và acceptance. |
| Nghiệm thu | Kiểm tra focused đã chạy; runner matrix đã chuẩn bị. | Chạy toàn bộ shared matrix, review watcher/adapter độc lập, sửa findings, commit từng batch và acceptance cuối. |

## Finding mở H2-R1 — ưu tiên sửa trước

Reviewer tái hiện trên stable và 1.85.0: cùng batch có entries a/b, commit
observer trả `observer-a`/`observer-b`, replacement apply cũng lỗi
`apply-1`/`apply-2`. Exact batch chỉ trả `a:observer-a` và `b:apply-2`;
`observer-b` biến mất cả ở settlement sau và Runtime diagnostics.
Nguyên nhân: dedup scheduling failures theo EntryId đã nhầm hai lỗi khác nhau
của cùng entry là một. Cần giữ cả lỗi apply và scheduling khác nhau, chỉ bỏ
đúng occurrence đã có. Không làm mất chính sách bounded first-unobserved slot.

Reproducer đã được lưu bền vững:
[probe source](reload-review-probe.rs.txt), [manifest](reload-review-probe.toml.txt).
Xem [review đang dở](../hmr-reload-review.md) để biết phạm vi còn chưa review.
Chưa sửa finding này vì người dùng yêu cầu tạm dừng.

## Drafts đã bảo toàn

- [9 integration tests](include_integration.rs.txt) → sau khi nhận ownership,
  khôi phục thành `crates/re-cordis-hmr/tests/include_integration.rs`.
- [Ví dụ reload offline](reload.rs.txt) → `crates/re-cordis-hmr/examples/reload.rs`;
  thêm `required-features = ["include"]` và dev-only serde_json vào manifest.
- [Runner shared matrix](run_matrix.py.txt) → khôi phục về
  `target/hmr-20261103/run_matrix.py`. Resolve lại tool paths cho máy hiện tại;
  [tools.json](tools.json) chỉ ghi paths/versions của phiên này.
- [H1 frozen hashes](h1-freeze-sha256.txt).
- [Thiết kế adapter và kế hoạch kiểm thử](../hmr-include-implementation.md).

Bản gốc còn trong ignored `target/hmr-h3/`, `target/hmr-reload-review/probe/`
và `target/hmr-20261103/`. Các bản `.txt` ở đây cố ý không thuộc Cargo targets:
chúng là drafts/reproducer, không phải tests đã green.

## Bằng chứng đã chạy

- H1 worker: 27 H1 tests + 1 Rustdoc, all-target check, strict Clippy/Rustdoc,
  stable formatting đều qua trên explicit stable 1.97.1 và Rust 1.85.0.
  [Evidence](../hmr-watch-implementation.md), logs `target/hmr-h1/`.
- H2 worker: loader 85 checks, HMR reload/nested 17, Include 79, strict
  Clippy/Rustdoc trên hai toolchain. [Evidence](../hmr-reload-implementation.md),
  logs `target/hmr-h2/`.
- Parent chạy lại checkpoint hiện tại: HMR **45** checks (27 watch + 17 reload/
  nested + 1 Rustdoc), loader batch **9**, Include linked **2**, cả hai toolchain.
  Logs `target/hmr-20261103/{hmr,loader,include}-checkpoint-{stable,1.85.0}.log`.
- Parent docs: **31** cặp trang, **19** HMR exports, **13** executable snippets
  và external quick start qua cả hai toolchain; bảy checker regressions qua.
  Logs `docs-h1-{stable,1.85.0}.log` trong cùng thư mục.
- Reviewer: 28 focused tests qua trên mỗi toolchain nhưng probe H2-R1 **đỏ**.
  Không suy diễn focused green thành acceptance.
- **Chưa chạy full shared matrix cho HMR.** Không tái sử dụng include final
  412 workspace checks làm chứng cứ cho mã HMR mới.

Mọi Cargo/rustc/rustdoc/cargo-clippy/clippy-driver phải resolve bằng
`rustup which --toolchain stable|1.85.0`, đặt matching RUSTUP_TOOLCHAIN,
RUSTC, RUSTDOC và bin đầu PATH. Không để Homebrew Rust lọt vào evidence.

## Quyết định cần giữ

- Chỉ file đăng ký tường minh; không recursive scan hay Rust module inference.
- Real watch có Rescan ban đầu; base canonical một lần, reject symlink bên dưới.
- WatchToken stop/Drop chỉ yêu cầu hủy; watcher/ancestor barrier mới join.
- Include adapter giữ shared barrier đã admit bên ngoài callback bị hủy,
  giữ original errors và tính quota cho công việc đang nghỉ chờ hoàn tất.
- Same-loader mutation bị chặn trong optional raw-C preflight. Core Plugin
  validation vẫn dependency-ready, không đổi core.
- Changed Rust source chỉ báo restart-required; không supervisor, Node caches,
  JS eval hay binary hot swap. Chưa có runtime acceptance Windows/Linux.

Các bản draft/probe .txt đã chuẩn hóa whitespace cuối dòng khi lưu Git; bản gốc
trong target vẫn giữ nguyên. [Fingerprint mã/fixture](frozen-inputs.json) ghi
125 input thực thi tại thời điểm tạm dừng; kiểm tra delta trước khi tiếp tục.
