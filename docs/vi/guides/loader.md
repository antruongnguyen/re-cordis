# Nạp và cấu hình lại plugin

[English](../../en/guides/loader.md) | [Trang tiếng Việt](../README.md)

Crate `re-cordis-loader` chưa phát hành quản lý plugin Rust đã liên kết theo tên
và cây entry có thứ tự. Thêm dependency theo đường dẫn cục bộ cùng với
`re-cordis-core` và Tokio. Loader sở hữu một Fiber riêng dưới Context được
truyền vào, nên cleanup của cha cũng dừng loader.

## Khởi động, phục hồi và dừng

`Catalog::register` giữ `Plugin<C, R>` có kiểu; `Config::new` chứa cấu hình
Rust mà không cần tuần tự hóa. Kiểu nguồn phải khớp catalog. Core vẫn validate
tại mốc readiness, sau khi các dịch vụ bắt buộc đã sẵn sàng.

```rust
use re_cordis_core::{BoxError, Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Group, Loader};

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let settings = ServiceKey::<u32>::new("settings");
            let worker = Plugin::new("worker", |_, value: u32| async move {
                if value == 13 { return Err::<(), BoxError>("example failure".into()); }
                Ok(())
            }).injecting([settings.required()]);
            let mut catalog = Catalog::new();
            catalog.register("worker", worker)?;
            let loader = Loader::new(&root, catalog)?;
            loader.create(EntrySpec::group(Group::new(vec![
                EntrySpec::new("worker", Config::new(1_u32)).id("worker"),
            ])).id("app"))?;
            let first = loader.settle()?.await?;
            assert!(first.failures.is_empty());
            let id = loader.resolve("app:worker")?;
            assert_eq!(loader.snapshot(&id).unwrap().state, EntryState::Pending);
            root.provide(&settings, 7)?;
            assert!(loader.settle()?.await?.failures.is_empty());
            loader.update(&id, EntrySpec::new("worker", Config::new(2_u32)))?.await?;
            assert!(loader.update(&id, EntrySpec::new("worker", Config::new(13_u32)))?.await.is_err());
            loader.update(&id, EntrySpec::new("worker", Config::new(14_u32)))?.await?;
            loader.dispose()?.await?;
            assert!(root.drain()?.await?.failures().is_empty());
            assert!(root.registry().is_empty()?);
            Ok::<_, BoxError>(())
        }).await??;
        Ok::<_, BoxError>(())
    })
}
```

Chạy ứng dụng có output chi tiết hơn từ repository:

```sh
cargo run -p re-cordis-loader --example boot_reconfigure --locked
```

Tiếp nhận và hoàn tất là hai bước: `create` trả identity ngay; `update`,
`remove`, `move_entry` và `dispose` bắt đầu công việc rồi trả waiter.
Thả waiter không hoàn tác thao tác. `settle` chờ công việc đã tiếp nhận,
kể cả hậu duệ cấu trúc, và báo lỗi theo entry.
Settlement thành công vẫn có thể chứa entry Pending; nó không bảo đảm provider
sẽ xuất hiện hay toàn hệ thống luôn sẵn sàng. Chờ cleanup trước khi hủy runtime
và kiểm tra cả lỗi thao tác lẫn báo cáo cleanup.

## Cấu hình và quyền sở hữu nguồn

`EntrySpec` thay thế toàn bộ mô tả; `EntryPatch` dùng
`FieldPatch::Keep`, `Set`, `Clear` cho sửa từng phần.
Snapshot giữ nguồn có kiểu tách khỏi giá trị đã đánh giá gần nhất.
Config mapping chạy trong đúng Context injection của plugin; condition và
intercept động dùng Context injection của entry. Entry bị vô hiệu hóa không
đánh giá callback. Lỗi callback có kiểu là failure hiển thị; update tường minh
có thể phục hồi.

Đăng ký decoder tường minh bằng `Catalog::register_json` trước khi dùng
`decode_json` hoặc `create_json`. Decoder nhận `None` khi thiếu config,
`Some(Null)` khi config là JSON null. Trường entry không biết và biểu thức
disabled không hỗ trợ bị từ chối. Config là dữ liệu do decoder xử lý,
không bao giờ là JavaScript thực thi. Xem trường và giới hạn ở
[API](../api/loader.md).

`SourceDocument::typed` hoặc `SourceDocument::json` cung cấp snapshot chỉ đọc
cho `reconcile`. Các entry nguồn hợp lệ được xử lý độc lập; sửa runtime
validate trước commit. `SourceId` là nhãn do bên gọi quản lý, không phải
đường dẫn để đọc. `with_locations` bổ sung vị trí thực cho đường dẫn chẩn đoán
có cấu trúc. Dữ liệu runtime không được gán tên tệp giả.

Theo dõi `loader.commits()` để nhận sửa nguồn runtime đã chấp nhận.
Reconciliation không phát commit ghi ngược. Bên gọi sở hữu tuần tự hóa và lưu
trữ; loader không đọc hay ghi tệp. `replace_factory` thay declaration đã liên
kết và tạo lại các entry liên quan sau cleanup. Nó không biên dịch mã,
import module hay theo dõi tệp.

Xem [group](groups.md), [API](../api/loader.md) và
[giới hạn](../../limitation.md#loader-and-structural-groups).
