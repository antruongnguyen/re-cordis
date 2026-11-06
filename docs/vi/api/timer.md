# Vệ tinh Timer

[English](../../en/api/timer.md) | [Mục lục API](README.md)

Mã nguồn: [timer lib.rs](../../../crates/re-cordis-timer/src/lib.rs).
Hướng dẫn: [timer](../guides/timers.md).
Ví dụ chạy được: [timer_lifecycle.rs](../../../crates/re-cordis-timer/examples/timer_lifecycle.rs).

`re-cordis-timer` chưa phát hành (`0.1.0-alpha.1`, Rust 2024, MSRV 1.85).
Import trait kín `TimerExt` trên `Context`; không cần cài dịch vụ.
Task timer dùng host Cordis gốc thông qua
[`Context::task`](effects.md#task-có-host-và-callback-đồng-bộ).
Callback có thể đăng ký timer/effect lồng nhau. Task Tokio tự tạo không thừa
kế quyền tiếp nhận. Dispose chủ sở hữu chờ việc hủy timer hoàn tất.

## TimerExt

| Phương thức | Kết quả | Hợp đồng |
|---|---|---|
| `timeout` | `Result<Effect, TimerError>` | Một callback; xóa đúng đăng ký trước khi gọi |
| `sleep` | `Sleep` | Tiếp nhận ngay; await `Ok(())` hoặc `Disposed` |
| `interval` | `Result<Effect, TimerError>` | Tick đầu sau một chu kỳ; bỏ tick lỡ mà không đổi pha |
| `interval_stream` | `Result<IntervalStream, TimerError>` | Stream một consumer; tối đa một tick chờ được gộp |
| `throttle` | `Result<Scheduled, TimerError>` | Gọi đầu ngay, tùy chọn gọi cuối tại ranh giới cửa sổ |
| `debounce` | `Result<Scheduled, TimerError>` | Lần gọi thay deadline chờ; chạy sau cửa sổ yên lặng |

Callback là `FnOnce` sở hữu (timeout) hoặc `FnMut` (các dạng callback khác),
capture `'static`, không cần `Send`. Hàm tạo kiểm tra host/chủ sở hữu trước
khi dùng thời gian Tokio hay tạo task. Từ chối giải phóng capture.

Deadline được kiểm tra và cố định tại lúc tạo thành công; Sleep được tiếp nhận
ngay cả khi poll đầu muộn. Deadline cuối của Scheduled được cố định tại mỗi lần
gọi thành công. Timeout/sleep/debounce bằng không chạy bất đồng bộ; throttle
bằng không gọi đầu ngay. Interval/stream có chu kỳ không và deadline không
biểu diễn được trả `InvalidDelay`. Chu kỳ dương dưới mili giây được nhận,
với độ chính xác mili giây của Tokio.

## Sleep

Future cho ra `Result<(), TimerError>`. Thành công xóa đăng ký trước khi phát
kết quả; chủ sở hữu hủy cho `Disposed`. Lỗi tạo được lưu và trả tại poll đầu.
Poll sau hoàn tất gây panic. Drop yêu cầu hủy. Hủy task và xóa đăng ký hoàn tất
khi host gốc được điều khiển hoặc barrier của chủ sở hữu được await.

## IntervalStream

Triển khai `futures_util::Stream<Item = Result<(), TimerError>>`.
Giữ tối đa một tick gộp, không lưu lịch sử tick chưa đọc không giới hạn.
Trạng thái kết thúc bỏ tick chờ và ưu tiên trước việc phát tick:

- `control().stop()`: kết thúc sạch `None`.
- `control().fail(reason)`: một `Err(reason)` cuối, rồi `None` ổn định.
- Chủ sở hữu hủy: một `Err(TimerError::Disposed)` cuối, rồi `None` ổn định.

Trạng thái kết thúc được ghi nhận đầu tiên thắng; yêu cầu dispose lifecycle
có trước lúc cleanup chủ sở hữu phát tín hiệu hủy. Drop tương đương stop. Hủy producer không
chờ tick kế tiếp; hoàn tất hủy cần host gốc tiến triển.
Giao thức đa độc giả return-value/throw của JS được bỏ; xem
[giới hạn](../../limitation.md#timer-satellite).

## IntervalControl

Quyền `stop()`/`fail(reason)` nhân bản được từ `control()`.
Yêu cầu kết thúc an toàn ngoài host và không tiếp nhận việc mới.

## Scheduled

Handle sở hữu cho throttle/debounce.

| Phương thức | Hợp đồng |
|---|---|
| `call() -> Result<(), TimerError>` | Lần gọi đang hoạt động cần host gốc còn sống; gọi sau dispose không làm gì |
| `dispose() -> Result<(), TimerError>` | Hủy bằng quyền đã cấp, an toàn ngoài host; gọi về sau không làm gì |

Callback chạy ngoài các borrow mutable của framework. Hỗ trợ tự dispose,
dispose thao tác khác và đăng ký timer lồng nhau. Gọi đệ quy với delay dương
gộp qua cửa sổ cuối thông thường; gọi đệ quy với delay bằng không bị bỏ để chặn
đệ quy và vòng lặp delay không. Panic dừng thao tác bị lỗi. Throttle cạnh đầu
trả lỗi callback đã giữ; lỗi callback bất đồng bộ đi vào chẩn đoán lõi đúng
một lần. Thao tác khác và host vẫn dùng được.

Drop yêu cầu hủy và giải phóng capture callback rảnh đồng bộ; callback đang
chạy giải phóng khi trả về. Barrier chủ sở hữu chờ hủy task và xóa đăng ký.
Điều này không đổi Drop thụ động của Effect lõi. Completion của Effect đã được
claim thủ công riêng vẫn là waiter riêng.

## TimerError

- `Disposed`: chủ sở hữu hủy trước khi phát kết quả.
- `InvalidDelay`: chu kỳ lặp không hoặc deadline đơn điệu không biểu diễn được.
- `TimeDriverUnavailable`: executor không bật time driver của Tokio.
- `Admission(AdmissionError)`: host sai/chưa chạy, chủ sở hữu cũ hoặc từ chối khác.
- `Callback(CleanupFailure)`: panic callback Scheduled gốc đã được báo.

Variant admission/callback giữ nguồn lỗi. Phát hiện thiếu driver bắt panic
tạo đồng bộ của Tokio; panic hook thông thường vẫn chạy. Không tạo task timer
tách rời hay chẩn đoán callback lõi được giữ lại.

## Đồng hồ và kiểm thử

Dùng `tokio::time` đơn điệu thay cho `Date.now()` theo đồng hồ tường của TS.
Hàm tạo đặt deadline trước khi trả; không cần yield trước advance để đặt hạn.
Sau khi dịch đồng hồ tạm dừng, yield để điều khiển phát kết quả/hủy task trước
khi kiểm tra. Guard lõi từ chối callback/task chờ lifecycle của chính mình/tổ tiên.

## Ví dụ

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
