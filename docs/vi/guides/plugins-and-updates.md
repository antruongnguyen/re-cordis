# Plugin, cấu hình và phục hồi

[English](../../en/guides/plugins-and-updates.md) | [Trang tiếng Việt](../README.md)

## Chọn kiểu khai báo

Dùng `Plugin::new` khi nguồn và đầu vào apply cùng một kiểu có thể clone.
Dùng `Plugin::validated` cho phép biến đổi đồng bộ `&C -> Result<R, E>`;
kiểu nguồn không bắt buộc Clone. Dùng `Plugin::initialized` khi khởi tạo async
tự nhiên tạo ra một `Disposer`.

Callback apply đăng ký tài nguyên qua Context rồi trả về `Result<(), E>`.
Framework không diễn giải nó thành constructor, generator JavaScript hay hàm
trả về disposer tùy ý. Kiểm tra cấu hình là đồng bộ; không tự khám phá thư viện
schema hoặc serde.

## Ví dụ cập nhật có kiểm tra cấu hình

Ví dụ chỉ cần các phụ thuộc trong [bắt đầu sử dụng](../getting-started.md).

```rust
use re_cordis_core::{BoxError, FiberState, Plugin, Runtime};
use std::io;

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                let plugin = Plugin::validated(
                    "BatchWorker",
                    |source: &u32| {
                        if *source == 0 {
                            Err(io::Error::new(io::ErrorKind::InvalidInput, "batch must be positive"))
                        } else {
                            Ok(*source)
                        }
                    },
                    |ctx, batch: u32| async move {
                        ctx.logger(None).info(format!("batch={batch}"))?;
                        Ok::<_, BoxError>(())
                    },
                );
                let fiber = root.plugin(&plugin, 1)?;
                assert_eq!(fiber.ready()?.await?, FiberState::Active);
                assert!(fiber.update(0).is_err());
                assert_eq!(*fiber.config(), 1);
                assert_eq!(fiber.update(2)?.await?, FiberState::Active);
                assert_eq!(*fiber.config(), 2);
                assert_eq!(fiber.dispose()?.await?, FiberState::Disposed);
                Ok::<_, BoxError>(())
            }.await;
            let cleanup = root.drain()?.await?;
            if let Some(failure) = cleanup.failures().first() {
                return Err(failure.clone().into());
            }
            result
        }).await?
    })
}
```

## Tiếp nhận không đồng nghĩa với sẵn sàng

`Context::plugin` commit quyền lưu trú trong Registry và sở hữu của cha ngay,
rồi trả handle; apply chạy trên host. Lỗi kiểm tra ban đầu để lại một phiên
bản Failed vẫn có thể truy cập. Chờ `ready` để quan sát, sau đó dispose hoặc
update tường minh.

Mỗi Fiber giữ nguồn `C`; apply nhận `R` đã kiểm tra. Cấu hình đề xuất khi đã
loaded được kiểm tra trước commit nguồn hay middleware cập nhật. Giá trị không
hợp lệ giữ nguyên nguồn/tài nguyên đang chạy. Fiber Pending giữ nguồn mới nhất
và hoãn kiểm tra cho đến khi dịch vụ bắt buộc sẵn có.

## Restart, update và middleware

`restart` buộc apply lại cùng nguồn nhưng **không** xóa Failed.
`update` nhận nguồn mới và cho phép phục hồi tường minh, ngay cả khi giá trị
nguồn không đổi. Chỉ rút/công bố lại provider không thử lại consumer Failed.

`on_update` nhận đầu vào đã kiểm tra và `AsyncNext<bool>` thuộc quyền sở hữu.
Gọi next bắt đầu nhánh restart. Không gọi next nghĩa là chấp nhận nguồn mà
không restart. `on_update_sync` trả true để tiếp tục hoặc false để chấp nhận mà
không restart. Các listener này có phạm vi và bị gỡ khi reload.

`UpdateOptions { no_save: true }` chỉ bỏ thông báo commit.
Nó không ngăn cập nhật nguồn được giữ lại hay áp dụng cấu hình.

## Đồng thời và dừng ứng dụng

Một driver vòng đời hội tụ tới cấu hình và phụ thuộc chính xác mới nhất.
Apply đang chạy không bị cưỡng bức hủy. Dọn dẹp hoàn tất trước khi mục tiêu
mới nhất có thể chạy được nạp; một số cập nhật trung gian có thể bị bỏ qua.
Dispose cuối cùng được ưu tiên hơn reload đang xếp hàng.

Handle chờ update có thể kết thúc ở Pending khi provider chưa có. Chờ provider
hoàn tất rồi chờ consumer sẵn sàng riêng. Không handle nào là mốc hội tụ toàn runtime.

Dùng `Registry::remove` để loại nhóm hiện tại của một khai báo; chờ từng task
trả về. Đăng ký mới của cùng khai báo tạo nhóm mới.
`Registry::len` đếm nhóm, không đếm phiên bản.

Tra cứu: [API plugin và Registry](../api/plugins-registry.md).
Bằng chứng: [kiểm thử cập nhật](../../../crates/re-cordis-core/tests/updates.rs) và
[các kịch bản vòng đời đối chiếu](../../lifecycle-parity.md).
