# Sự kiện và listener

[English](../../en/api/events.md) | [Mục lục API](README.md)

Mã nguồn: [events.rs](../../../crates/re-cordis-core/src/events.rs).
Hướng dẫn có mã chạy: [sự kiện](../guides/events.md).

## Hợp đồng và đăng ký

| API | Hợp đồng |
|---|---|
| `Event::<A, R>::named(name)` | Định danh chuỗi chia sẻ, kiểm tra kiểu đối số/kết quả đang tồn tại |
| `Event::symbol(description)` | Định danh opaque mới; clone để chia sẻ |
| `Listener::sync` | `(Context, A) -> Result<ControlFlow<R>, E>` |
| `Listener::asynchronous` | Factory nhận `(Context, A)`, trả responder async |
| `Listener::around` | `(Context, A, Next<R>) -> Result<R, E>` |
| `Listener::around_async` | Factory nhận `(Context, A, AsyncNext<R>)`, trả kết quả async |
| `Context::on` | Đăng ký với tùy chọn mặc định |
| `Context::on_with` | Đăng ký với `ListenerOptions` tường minh |
| `Context::once` | Đăng ký hành vi gỡ trước khi gọi |

`R` mặc định `()`. Dispatch cần đối số Clone; hợp đồng listener và future thuộc
quyền sở hữu/`'static`. Callback nhận Context đăng ký đã chọn, không phải `this`
JavaScript ngầm.

`ListenerOptions` có bool công khai `prepend`, `global`, mặc định false.
`DispatchOptions::default` chọn listener không lọc;
`DispatchOptions::filtered` kiểm tra Context đăng ký. Global bỏ qua bộ lọc.

`ListenerRegistration::remove` nhận quyền gỡ đăng ký đồng bộ, trả
`Result<bool, AdmissionError>`. `into_effect` chuyển việc gỡ có phạm vi vào
composition. Thả token không gỡ đăng ký.

## Kết quả dispatch

Mọi phương thức nhận `(&event, args, DispatchOptions)`; các dạng waterfall
còn nhận callback kết thúc thuộc quyền sở hữu, không có đối số.

| Phương thức Context | Kết quả thành công bên ngoài | Listener được chấp nhận |
|---|---|---|
| `emit` | `EventTask<()>` | sync / asynchronous |
| `parallel` | `EventTask<()>` | sync / asynchronous |
| `serial` | `EventTask<ControlFlow<R>>` | sync / asynchronous |
| `bail` | `ControlFlow<R>` | sync |
| `waterfall` | `R` | around |
| `waterfall_async` | `EventTask<R>` | around_async |

Lỗi bên ngoài là `EventError`. `EventTask<T>` chờ
`Result<T, EventError>` của công việc đã bắt đầu.

Emit gọi callback sync/factory ngay, bỏ câu trả lời Break và dừng khi lỗi đồng
bộ. Continuation đã bắt đầu vẫn sống nếu có lỗi tức thời sau đó.
Parallel gọi mọi listener và chờ mọi kết quả; lỗi giữ thứ tự snapshot.
Serial/bail dừng ở Break đầu tiên, kể cả giá trị zero/rỗng.
Waterfall là middleware bao quanh, không phải reduce.

`Next::call(self)` trả `Result<R, EventError>`.
`AsyncNext::call(self)` trả future cục bộ của kết quả đó.
Cả hai tiêu thụ chính mình nên gọi next lặp là lỗi kiểu. Future continuation
async vẫn cần await; gọi adapter không khiến thân future Rust tùy ý chạy ngay.

## Snapshot và lỗi

Snapshot các listener đã đăng ký được chụp trước khi chạy bộ lọc dispatch.
Bộ lọc chọn từ snapshot đó trước khi các callback sự kiện được chọn chạy.
Gỡ callback đã chọn không thu hồi snapshot đang chạy.
Once gỡ trước khi gọi nhưng không thêm cổng nhận quyền lần hai cho các snapshot
đã chọn. Gỡ listener cuối giải phóng bucket kiểu sống của event có tên.

Tổ hợp listener/mode không hỗ trợ gây lỗi trước khi gọi snapshot.
Lỗi async emit luôn được chẩn đoán. Lỗi callback của
serial/parallel/async-waterfall trả cho handle chờ được giữ hoặc được chẩn đoán
khi handle bị bỏ. Thả EventTask không hủy worker đã bắt đầu.

`EventError` phân biệt tiếp nhận, sai hợp đồng, mode không hỗ trợ, lỗi callback,
HostStopped, DriverPanicked. `Callbacks` chứa nhãn event và
`Vec<CleanupFailure>`; Error source là lỗi đầu tiên, không thay thế việc xem
tất cả thành viên aggregate.

Với interception nội bộ, `hooks::ListenerRequest::replace` và `replace_with`
giữ sở hữu đăng ký mà không gọi đệ quy giao thức interception.
Xem [hook](errors-hooks.md).
