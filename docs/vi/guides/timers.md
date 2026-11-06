# Timer

[English](../../en/guides/timers.md) | [Trang tiếng Việt](../README.md)

Import `re_cordis_timer::TimerExt`, gọi phương thức trên Context còn sống trong
`Runtime::run` gốc. Sáu thao tác dùng callback cục bộ sở hữu; không cần cài
dịch vụ timer hay ràng buộc Send.

## Công việc một lần và lặp lại

`timeout(delay, callback)` trả Effect có Drop thụ động; dùng
`effect.dispose()?.await?` để hủy và chờ đúng cleanup thủ công đó.
Timeout hoàn tất xóa đăng ký trước khi gọi callback.
`sleep(delay)` tiếp nhận ngay và trả kết quả để await. Poll đầu có thể muộn
mà không đặt lại delay đã khai báo. Chủ sở hữu hủy trả `Disposed`.

`interval(period, callback)` chạy đầu sau một chu kỳ; bỏ tick lỡ mà không đổi
pha. Dispose chủ sở hữu chờ giải phóng capture callback.
Callback có thể đăng ký effect/timer Context lồng nhau.

## Tick stream và hủy

`interval_stream(period)` là Stream một consumer, tối đa một tick chờ gộp.
`control().stop()` kết thúc sạch; `fail(reason)` phát một lỗi rồi None ổn định.
Cleanup chủ sở hữu phát Disposed một lần. Trạng thái kết thúc được ghi nhận đầu
tiên thắng và bỏ tick chờ; yêu cầu lifecycle trước đó chưa tự ghi trạng thái này.

Drop của Sleep, IntervalStream và Scheduled yêu cầu hủy. Hủy task và xóa đăng
ký cần host gốc tiến triển; await barrier dispose/drain chủ sở hữu để chờ hủy.
Drop của Effect lõi vẫn thụ động. Giữ waiter completion đã claim thủ công
riêng khi barrier đó cần thiết.

## Throttle và debounce

`throttle(delay, no_trailing, callback)` gọi đầu ngay; các lần bị chặn tùy chọn
gộp tại ranh giới cửa sổ cố định.
`debounce(delay, callback)` thay deadline cuối tại mỗi lần gọi.
Dùng `scheduled.call()?` và `scheduled.dispose()?`. Lần gọi đang hoạt động
kiểm tra host gốc còn sống. Gọi sau dispose không làm gì; hủy an toàn ngoài host.

Hỗ trợ tự dispose và gọi đệ quy delay dương mà không borrow trạng thái mutable
của framework trong lúc chạy mã người dùng. Bỏ gọi đệ quy delay không.
Panic callback dừng thao tác đó, báo đúng một lần qua chẩn đoán lõi và vẫn cho
phép dùng thao tác khác. Từ chối chờ lifecycle chính mình/tổ tiên.

## Ranh giới đồng hồ và kiểm tra tất định

Kiểm tra deadline tại lúc tiếp nhận. Với delay bằng không, timeout/sleep/debounce chạy ở lượt
host sau; throttle gọi đầu ngay. Interval/stream chu kỳ không và deadline
không biểu diễn được bị từ chối. Chu kỳ dương dưới mili giây dùng độ chính xác
mili giây của Tokio. Bật thời gian Tokio trên executor; thiếu driver trả
TimeDriverUnavailable khi tạo, panic hook đồng bộ thông thường của Tokio vẫn
hiển thị và không có task tách rời.

Dùng `start_paused(true)` và `tokio::time::advance` để kiểm thử tất định.
Deadline đã được đặt khi hàm tạo trả về. Yield sau advance để phát kết quả;
yield sau hủy để xóa đăng ký.
Xem [API timer](../api/timer.md) và
[ví dụ lifecycle plugin dùng cả sáu thao tác](../../../crates/re-cordis-timer/examples/timer_lifecycle.rs).

## Ví dụ hoàn chỉnh

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
                root.sleep(Duration::from_millis(1)).await?;
                let debounced = root.debounce(Duration::ZERO, || println!("quiet"))?;
                debounced.call()?;
                root.sleep(Duration::from_millis(1)).await?;
                debounced.dispose()?;
                root.drain()?.await?;
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}
```
