# Thay đổi tệp và thay thế plugin đã liên kết

[English](../../en/guides/hmr.md) | [Trang chủ tiếng Việt](../README.md)

Crate thử nghiệm đã phát hành `re-cordis-hmr` điều phối các tệp đăng ký rõ ràng và
thay thế plugin Rust đã được liên kết vào tiến trình, với tích hợp Include tùy chọn.
H0–H3 đã được nghiệm thu độc lập và qua đầy đủ ma trận chung stable/1.85.0. Xem [API](../api/hmr.md) và
[bản đồ nguồn](../../plans/2026-11-02-cordis-completion/evidence/hmr-source-map.md)
để biết phạm vi và giới hạn bằng chứng.

Chọn thao tác theo dữ liệu đầu vào:

| Đầu vào | Thao tác | Kết quả |
|---|---|---|
| Tệp cấu hình | `FileWatcher::watch` với callback; adapter `watch_include` tùy chọn | Làm mới nguồn đã đăng ký và giữ lỗi để quan sát |
| Plugin mới đã biên dịch vào tiến trình | `ReloadPlan::replace` với các `FactoryReplacement` | Loader dựng lại entry đã chọn sau khi cleanup cũ hoàn tất |
| Tệp mã nguồn Rust thay đổi | `FileWatcher::watch_source` | `WatchEvent::RestartRequired` cho bộ điều phối build/restart của ứng dụng |

Watcher không tự tìm module Rust, duyệt đệ quy cây thư mục, biên dịch mã,
nạp lại binary hay khởi động tiến trình thay thế.
Khai báo từng tệp cần theo dõi và từng phụ thuộc mã cần dùng.

## Theo dõi một tệp và giữ token

Tạo watcher dưới Context Cordis còn hoạt động trên host cục bộ gốc, với
Tokio time được bật. `FileWatcher::new` sở hữu producer polling; `synthetic`
dùng cùng queue hữu hạn, debounce và dispatch cục bộ nhưng không đọc tệp.
Ví dụ dưới đây phát thông báo mã nguồn bằng sự kiện tổng hợp.

```rust
use std::time::Duration;
use re_cordis_core::{BoxError, Runtime};
use re_cordis_hmr::{
    Change, ChangeKind, FileWatcher, ReloadNode, ReloadPlan, WatchEvent, WatchOptions,
};
use re_cordis_timer::TimerExt;

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let plan = ReloadPlan::new(vec![
                ReloadNode {
                    name: "shared".into(),
                    factory: None,
                    dependencies: vec![],
                },
                ReloadNode {
                    name: "worker-code".into(),
                    factory: Some("worker".into()),
                    dependencies: vec!["shared".into()],
                },
            ])?;
            assert_eq!(plan.affected(&["shared".into()])?, vec!["worker"]);

            let mut options = WatchOptions::new(std::env::current_dir()?);
            options.debounce = Duration::ZERO;
            let (watcher, events) = FileWatcher::synthetic(&root, options)?;
            let source = watcher.watch_source("src/main.rs")?;
            events.emit(Change {
                path: source.path().to_path_buf(),
                kind: ChangeKind::Modified,
            })?;
            tokio::time::timeout(Duration::from_secs(2), async {
                loop {
                    if watcher.take_events().iter().any(|event| {
                        matches!(event, WatchEvent::RestartRequired(_))
                    }) {
                        break Ok::<_, BoxError>(());
                    }
                    root.sleep(Duration::from_millis(1)).await?;
                }
            }).await??;

            source.stop();
            watcher.dispose()?.await?;
            root.drain()?.await?;
            Ok::<_, BoxError>(())
        }).await??;
        Ok::<_, BoxError>(())
    })
}
```

`watch(path, callback)` nhận `FnMut(Change)` cục bộ trả về
`LocalFuture<Result<(), BoxError>>`. Capture dữ liệu có ownership; không
yêu cầu Send. Mỗi đăng ký chạy tối đa một future callback và gộp các thay đổi
đến trong lúc đó thành một lượt tiếp theo. Nhiều token trên cùng đường dẫn
giữ các callback riêng.

Giữ WatchToken trong suốt thời gian cần đăng ký. `stop()` và Drop yêu cầu
hủy; chúng không chờ đồng bộ callback đang chạy. Await `watcher.dispose()`
hoặc barrier lifecycle của owner để quan sát callback đã bị hủy và producer
đã hoàn tất. Yêu cầu drain watcher từ callback của chính nó bị từ chối trước
khi watcher dừng.

## Đường dẫn, polling và phục hồi lỗi

Đường dẫn tương đối được giải từ `WatchOptions::base` cố định. Đường dẫn
logic vẫn giữ nguyên khi thay tệp bằng rename hoặc tạo lại tệp/thư mục cha.
Rename thư mục cha không chuyển đăng ký sang vị trí mới. Symlink đang tồn tại
bị từ chối; symlink, thư mục, lỗi đọc hoặc tệp quá lớn phát sinh khi lấy mẫu
tạo lỗi hiển thị và vẫn cho phép phục hồi bằng tệp thường hợp lệ sau đó.

`watch`/`watch_source` ghi đè các thành phần bị bỏ qua và có thể đăng ký
ngoài base. `observe`/`observe_source` lọc tên thành phần native chính xác,
trả `Ok(None)` cho đường dẫn bị bỏ qua. Mặc định là `.git`, `target` và
`node_modules`; không có ngôn ngữ glob hay quét root.

Đăng ký thật nhận Rescan ban đầu, kể cả callback mới trên đường dẫn đã theo
dõi. Polling và debounce dùng hai nhịp riêng. Polling so sánh metadata hữu hạn
và digest streaming không dùng cho mật mã; có thể bỏ lỡ thay đổi hoàn toàn
giữa hai lần lấy mẫu hoặc do va chạm digest. Queue đầy yêu cầu rescan tập
đăng ký hữu hạn.

Dùng `take_events()` để lấy Change, RestartRequired và Failure.
History loại bản ghi cũ nhất khi đầy; `stats()` cho biết overflow và eviction.
Failure giữ nguyên nguồn lỗi khi còn được lưu. `capabilities()` trả snapshot
giá trị và không phát cảnh báo lặp lại.
Nghiệm thu runtime Windows và Linux còn chờ; smoke test macOS không chứng minh
hành vi đường dẫn hay watcher trên hai nền tảng đó.

## Thay thế factory được khai báo rõ ràng

`ReloadNode` đặt tên đơn vị mã của ứng dụng, có thể ánh xạ tới factory trong
catalog, và liệt kê các đơn vị đã khai báo mà nó sử dụng. Node chỉ biểu diễn
phụ thuộc có `factory: None`. `affected(changed)` đi theo phụ thuộc ngược;
phụ thuộc dùng chung không đổi không chọn thêm những consumer khác.

Cung cấp đúng một `FactoryReplacement::new(name, plugin)` cho mỗi factory
bị ảnh hưởng khi gọi `plan.replace(&loader, changed, replacements)`. Await
`LoaderTask<Settlement>` trả về và xem lỗi của từng entry. Đồ thị, tập
candidate, kiểu source và preflight cấu hình thô tùy chọn được kiểm tra trước
bất kỳ teardown nào. Preflight phải thuần; thay đổi cùng loader bị từ chối
trong preflight. Plugin vẫn được validate trên Context thật đã đủ phụ thuộc.

Mỗi entry giữ source config riêng và provider cũ chính xác cho đến khi cleanup
xong. Entry nhanh không liên quan có thể dựng lại trong khi sibling chậm còn
drain. Sửa config, disable/remove và ý định host mới trong cleanup thắng công
việc thay thế cũ. Plugin core lồng nhau dựng lại qua candidate của host được
cung cấp; không tự thay declaration lồng trong host đã biên dịch không đổi.

Apply thất bại sau unload vẫn hiển thị và cần update rõ ràng hoặc candidate
sau để phục hồi. Không thể rollback giao dịch cleanup đã xong hay tác động
bên ngoài. Candidate đã được liên kết vào tiến trình đang chạy; nó không phải
kết quả biên dịch tệp nguồn vừa thay đổi.

## Tích hợp Include tùy chọn

Bật feature `include` của crate HMR rồi gọi
`watch_include(path, include.clone())`. Thao tác đăng ký một đường dẫn kích hoạt
rõ ràng và gọi refresh trên Include được truyền vào, gồm cả đồ thị nguồn của nó.
Đăng ký từng tệp cần kích hoạt refresh; adapter không tự tìm đường dẫn.
Include vẫn độc lập với HMR.

Để thay factory được cùng instance Include đó dùng, gọi
`plan.replace_include(&include, changed, replacements)`. Await
`IncludeTask<Report>` trả về. Nó chuyển đúng completion batch loader và giữ
journal, data, provenance cùng byte tệp của Include; không flush. Đường này
dùng cùng kiểm tra đồ thị và candidate rõ ràng như `replace`.

Adapter giữ IncludeTask đã được nhận ngoài callback có thể bị hủy.
Stop ngăn refresh tiếp theo; cleanup watcher/ancestor chờ completion đã giữ,
không gọi Include mới sau khi admission đóng. Đăng ký đã dừng nhưng còn việc
chưa xong vẫn chiếm quota watch. Lỗi Include trả về và từng nguyên nhân trong
`Report.failures` đi vào history `WatchEvent::Failure` hữu hạn đúng một lần cho
mỗi refresh hoàn tất. Lỗi cleanup core thông thường vẫn dùng kênh
`Runtime::take_failures`; cần kiểm tra cả hai. Lỗi parse giữ nguồn hợp lệ cuối
và cho phép refresh hợp lệ sau đó. Self-write dùng rename của Include hội tụ
mà không apply lại dữ liệu đã chấp nhận không đổi. Xem [Include](include.md)
về giới hạn persistence và durability.

## Ví dụ offline

Chạy [ví dụ reload hoàn chỉnh](../../../crates/re-cordis-hmr/examples/reload.rs)
từ thư mục gốc repository:

```sh
cargo run -p re-cordis-hmr --example reload --features include --locked --offline
```

Ví dụ tự tạo tệp tạm, theo dõi cấu hình bằng một Include, thay compiled-v1 bằng
compiled-v2 và giữ EntryId cùng cấu hình hiện tại, rồi xác nhận refresh cấu hình
sau đó dùng compiled-v2. Nó in snapshot capability một lần, báo
`RestartRequired` cho mã nguồn, chờ shutdown watcher và Include rồi kiểm tra
Registry trở về trạng thái ban đầu. Không cần mạng hay tiến trình build bên ngoài.
