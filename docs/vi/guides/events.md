# Sự kiện và middleware bao quanh

[English](../../en/guides/events.md) | [Trang tiếng Việt](../README.md)

## Định nghĩa hợp đồng

`Event<A, R>` có đối số thuộc quyền sở hữu kiểu `A` và câu trả lời kiểu `R`
(mặc định `()`). Event có tên chia sẻ định danh theo tên và kiểm tra hợp đồng
kiểu đang tồn tại. Event symbol tạo định danh riêng; clone chia sẻ nó, lặp lại
mô tả không tạo cùng định danh.

```rust
use cordis_core::{BoxError, DispatchOptions, Event, Listener, Runtime};
use std::{convert::Infallible, ops::ControlFlow};

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                let query = Event::<u32, u32>::symbol("double");
                root.on(&query, Listener::sync(|_, value| {
                    Ok::<_, Infallible>(ControlFlow::Break(value * 2))
                }))?;
                assert_eq!(
                    root.serial(&query, 3, DispatchOptions::default())?.await?,
                    ControlFlow::Break(6),
                );
                let chain = Event::<(), u32>::symbol("around");
                root.on(&chain, Listener::around(|_, (), next| {
                    Ok::<_, BoxError>(next.call()? + 1)
                }))?;
                let answer = root.waterfall(&chain, (), DispatchOptions::default(), || {
                    Ok::<_, Infallible>(10_u32)
                })?;
                assert_eq!(answer, 11);
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

## Chọn mode và kiểu listener tương ứng

| Mode | Kiểu listener | Kết quả |
|---|---|---|
| `emit` | `sync`, `asynchronous` | Gọi ngay; handle chờ kết quả phần trì hoãn |
| `parallel` | `sync`, `asynchronous` | Chờ mọi kết quả, giữ mọi lỗi |
| `serial` | `sync`, `asynchronous` | `ControlFlow::Break` đầu tiên theo thứ tự |
| `bail` | chỉ `sync` | Break đầu tiên, đồng bộ |
| `waterfall` | `around` | Kết quả chuỗi bao quanh đồng bộ |
| `waterfall_async` | `around_async` | Kết quả chuỗi bao quanh async |

Hai dòng cuối là các dạng Rust tường minh của mode waterfall của Cordis, không
phải vòng lặp reduce giá trị. Middleware gọi `next.call()` để tiếp tục hoặc
trả về không gọi next để chặn phần còn lại. Next thuộc quyền sở hữu và dùng một
lần; gọi hai lần gây lỗi biên dịch. Không tự trộn around-listener sync và async.

Continue nghĩa là không trả lời. `Break(0)` và `Break(String::new())` vẫn là câu
trả lời. Emit/parallel không dùng Break để dừng sớm. Snapshot không tương thích
mode bị từ chối trước khi callback chạy.

## Thời điểm chạy async

`Listener::asynchronous` gọi factory trong lúc dispatch. Phần nằm ngoài khối
async trả về là phần đầu chạy ngay; phần trong khối chạy khi được điều khiển.
Emit dừng khi responder đồng bộ lỗi, nhưng continuation async đã bắt đầu vẫn
được lên lịch.

Chờ event task trả về khi cần bảo đảm hoàn thành. Gỡ listener hoặc drain owner
không hủy dispatch đã chọn listener đó. Hủy host sẽ hủy tác vụ framework còn lại.

## Chọn listener và vòng đời

`on`, `on_with`, `once` đăng ký trên owner đã chọn. `ListenerOptions` điều khiển
prepend/global. `DispatchOptions::filtered` kiểm tra Context đăng ký của
listener; không có định tuyến chỉ-tổ-tiên tự động. Listener global bỏ qua bộ
lọc. Với sự kiện từ dịch vụ, chọn bộ lọc realm dịch vụ tường minh.

Once gỡ trước khi gọi để bảo vệ dispatch đệ quy. Nó không thu hồi callback đã
được snapshot khác chọn trước đó. `remove` thủ công gỡ đúng một đăng ký đồng
bộ; thả token không làm gì.

Lỗi async từ emit được chẩn đoán kể cả còn handle chờ. Với
serial/parallel/async-waterfall, lỗi callback thuộc caller đang chờ hoặc được
chẩn đoán nếu handle bị bỏ. Truyền lỗi lên thay vì bỏ kết quả.

Tra cứu: [API sự kiện](../api/events.md).
