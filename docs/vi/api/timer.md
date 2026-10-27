# Vệ tinh Timer

[English](../../en/api/timer.md) | [Mục lục API](README.md)

Mã nguồn: [re-cordis-timer/src/lib.rs](../../../crates/re-cordis-timer/src/lib.rs).
Hướng dẫn có mã chạy: [timer](../guides/timers.md).
Ví dụ có thể chạy:
[timer_lifecycle.rs](../../../crates/re-cordis-timer/examples/timer_lifecycle.rs).

`re-cordis-timer` là crate vệ tinh chưa phát hành (`0.0.0`, Rust 2024, MSRV
1.85) cung cấp các timer thuộc phạm vi chủ sở hữu, xây trên hệ thống effect sẵn
có của lõi. Nó không thêm một lõi định thời: mỗi thao tác lên lịch một task
`tokio::time` trên local task set của runtime và đăng ký một cleanup
`Context::on_dispose_sync`, nên dispose context sở hữu sẽ hủy thao tác và không
callback nào chạy sau đó.

Các thao tác được truy cập qua trait kín [`TimerExt`](#timerext), chỉ được
triển khai cho `Context`. Không cần đăng ký dịch vụ hay bước cài đặt nào —
import trait là đủ.

## `TimerExt`

Trait mở rộng kín trên `Context`. Import nó để gọi các phương thức.

| Phương thức | Kết quả | Hành vi |
|---|---|---|
| `timeout` | `Result<Effect, TimerError>` | Chạy `callback` một lần sau `delay`, rồi tự dispose. Dispose `Effect` để hủy sớm. Tối đa một lần gọi. |
| `sleep` | [`Sleep`](#sleep) | Dạng await: giải quyết `Ok(())` một lần tại thời điểm đến hạn, hoặc `Err(TimerError::Disposed)` nếu chủ sở hữu bị dispose trước. |
| `interval` | `Result<Effect, TimerError>` | Chạy `callback` mỗi `delay`; tick đầu tại `delay`. Dispose `Effect` để dừng. Tick bị bỏ lỡ sẽ gộp (bỏ qua). |
| `interval_stream` | `Result<IntervalStream, TimerError>` | Một [`Stream`](#intervalstream) các tick; xem hợp đồng kết thúc của nó. |
| `throttle` | `Result<Scheduled, TimerError>` | Giới hạn tần suất theo cạnh trước với một lần gọi cuối tùy chọn; cửa sổ đơn điệu. |
| `debounce` | `Result<Scheduled, TimerError>` | Gộp theo cạnh sau; mỗi lần gọi khởi động lại cửa sổ. |

Dạng callback nhận `FnOnce` (`timeout`) hoặc `FnMut` (`interval`,
`throttle`, `debounce`) với ràng buộc `'static`. `delay` bằng không được chấp
nhận và chạy ở lượt runtime kế tiếp.

## `Sleep`

Future trả về bởi `TimerExt::sleep`. Cho ra `Result<(), TimerError>`: `Ok(())`
đúng một lần tại thời điểm đến hạn, hoặc `Err(TimerError::Disposed)` nếu context
sở hữu bị dispose trước thời điểm đó. Nó giữ đăng ký dispose của mình sống suốt
vòng đời, nên hệ thống dây nối hủy vẫn kết nối cho đến khi cuộc đua kết thúc.

## `IntervalStream`

Triển khai `futures_util::Stream<Item = Result<(), TimerError>>`. Mỗi tick cho
ra `Ok(())`. Kết thúc:

- [`control().stop()`](#intervalcontrol) — kết thúc stream sạch sẽ (`None`),
  không có item cuối.
- `control().fail(reason)` — phát một `Err(reason)` cuối, rồi `None`.
- Dispose chủ sở hữu — phát một `Err(TimerError::Disposed)` cuối, rồi `None`.

Một `Stream` có một consumer logic duy nhất. Giao thức
`return(value)`/`throw(reason)` đa độc giả đồng thời của async iterator
TypeScript **không** được tái tạo; dùng `interval` với callback khi cần fan-out.
Xem [giới hạn](../../limitation.md#timer-satellite).

## `IntervalControl`

Handle nhân bản được từ `IntervalStream::control()` với `stop()` và
`fail(reason)`, như mô tả ở trên.

## `Scheduled`

Handle gọi được và hủy được từ `throttle` và `debounce`.

| Phương thức | Hành vi |
|---|---|
| `call()` | Gọi thao tác. Throttle chạy ở cạnh trước và (trừ khi `no_trailing`) một lần tại cuối cửa sổ; debounce chạy một lần khi các lần gọi im lặng suốt trọn `delay`. Handle đã dispose không làm gì. |
| `dispose()` | Hủy: bỏ mọi lần chạy cuối đang chờ và bỏ qua các lần gọi về sau. Dispose chủ sở hữu cũng vậy. |

## `TimerError`

Lỗi `#[derive(Debug)]` với các variant:

- `Disposed` — context sở hữu bị dispose trước khi thao tác hoàn tất.
- `InvalidDelay` — delay yêu cầu vượt quá phạm vi hỗ trợ.
- `Admission(AdmissionError)` — context không thể tiếp nhận đăng ký.

## Đồng hồ và kiểm thử

Timer dùng đồng hồ đơn điệu `tokio::time` mà runtime cục bộ điều khiển, không
phải thời gian đồng hồ treo tường. Điều này khác với cửa sổ throttle/debounce
dựa trên `Date.now()` của TypeScript; xem [quyết định](../../decision.md#timer-satellite).
Chạy kiểm thử tất định với runtime tạm dừng (`start_paused`) và
`tokio::time::advance`, poll một lần (`tokio::task::yield_now().await`) sau khi
tạo hoặc gọi một thao tác để task của nó đăng ký trước khi đồng hồ dịch chuyển.

## Ví dụ

Một chương trình hoàn chỉnh chờ (await) một khoảng trễ thực ngắn qua `sleep`:

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
                Ok::<_, BoxError>(())
            })
            .await??;
        Ok::<_, BoxError>(())
    })
}
```

Để kiểm thử tất định, dùng runtime tạm dừng (`start_paused(true)`, vốn cần
feature `test-util` của tokio) và `tokio::time::advance`; xem
[hướng dẫn timer](../guides/timers.md).
