# Timer

[English](../../en/guides/timers.md) | [Trang tiếng Việt](../README.md)

Vệ tinh `re-cordis-timer` thêm các timer thuộc phạm vi chủ sở hữu, xây trên hệ
thống effect của lõi. Mỗi thao tác bị hủy khi context sở hữu nó bị dispose, nên
timer không bao giờ sống lâu hơn thành phần đã tạo ra nó. Truy cập các thao tác
qua trait kín `TimerExt` — import nó và gọi phương thức trên bất kỳ `Context`
nào còn sống. Không cần cài đặt dịch vụ. Xem
[tài liệu API timer](../api/timer.md) để biết toàn bộ bề mặt.

## Callback một lần và lặp lại

`timeout` chạy một lần; `interval` chạy mỗi chu kỳ (tick đầu tại chu kỳ, không
phải tại không). Cả hai đều trả về một `Effect` — dispose nó để hủy. Dispose
chủ sở hữu cũng hủy chúng. Chương trình hoàn chỉnh này chờ (await) một `sleep`
một lần:

```rust
use std::time::Duration;

use re_cordis_core::{BoxError, Runtime};
use re_cordis_timer::TimerExt;

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                // Resolves at the deadline; Err(Disposed) if the owner is
                // disposed first.
                root.sleep(Duration::from_millis(1)).await?;
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}
```

Với callback lặp lại, `root.interval(period, || { ... })?` trả về một `Effect`;
dispose nó để dừng. Tick đầu tại `period`, và các tick bị bỏ lỡ sẽ gộp lại.

## Chờ một khoảng trễ

`sleep` là dạng await của `timeout`. Nó giải quyết `Ok(())` tại thời điểm đến
hạn, hoặc `Err(TimerError::Disposed)` nếu context sở hữu bị dispose trước — nên
một thành phần đang dừng giữa lúc chờ sẽ quan sát được việc hủy thay vì treo.

## Luồng tick

Khi bạn muốn kéo (pull) tick thay vì đẩy (push) tới một callback,
`interval_stream` trả về một `futures_util::Stream` của `Result<(), TimerError>`.
Handle `control()` của nó kết thúc stream với `stop()` (sạch sẽ) hoặc
`fail(reason)` (một lỗi cuối); dispose chủ sở hữu kết thúc nó với
`Err(TimerError::Disposed)`. Khác với async iterator của TypeScript, một stream
có một consumer logic duy nhất — để fan-out, dùng dạng callback `interval`.

## Giới hạn tần suất: throttle và debounce

`throttle` chạy ở cạnh trước và, trừ khi đặt `no_trailing`, thêm một lần nữa tại
cuối cửa sổ. `debounce` chạy một lần khi các lần gọi im lặng suốt trọn `delay`.
Cả hai đều trả về một handle `Scheduled` mà bạn gọi bằng `call()` và hủy bằng
`dispose()`. Các cửa sổ là đơn điệu (`tokio::time`), không phải đồng hồ treo
tường.

## Kiểm thử timer một cách tất định

Dựng runtime với `start_paused(true)` và dịch chuyển đồng hồ một cách tường minh
bằng `tokio::time::advance`. Poll một lần với `tokio::task::yield_now().await`
sau khi tạo hoặc gọi một thao tác, để task của nó đăng ký trên trình điều khiển
timer đang tạm dừng trước khi bạn dịch chuyển — nếu không, lần dịch chuyển sẽ
vượt qua một thời điểm đến hạn mà task chưa kịp đặt. Ví dụ ở trên tuân theo mẫu
này.
