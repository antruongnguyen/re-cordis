# Lỗi, kiểm tra cấu hình và hook nội bộ

[English](../../en/api/errors-hooks.md) | [Mục lục API](README.md)

Mã nguồn: [error.rs](../../../crates/cordis-core/src/error.rs),
[hooks.rs](../../../crates/cordis-core/src/hooks.rs).

## Nhóm lỗi

| Kiểu | Ý nghĩa / variant chính |
|---|---|
| `HostError` | MissingExecutor, NestedHost, AlreadyRunning, SpawnedTask |
| `AdmissionError` | RuntimeDropped, HostNotRunning, WrongHost, ReentrantDrain, IdentityExhausted, InactiveContext |
| `CompletionError` | HostStopped, DriverPanicked |
| `LifecycleError` | Admission, Callback, Hook, HostStopped, DriverPanicked |
| `SetupError` | Admission, Callback, AlreadyClaimed, RecursiveComposition, HostStopped, DriverPanicked |
| `ServiceError` | Admission/Hook, DuplicateProvider, TypeMismatch, Undeclared, Inactive, NotProvided, NotOwner |
| `ReflectError` | Admission, Duplicate, Missing, Contract, ReadOnly, Callback |
| `EventError` | Admission, ContractMismatch, UnsupportedMode, Callbacks, HostStopped, DriverPanicked |
| `LogError` | RuntimeDropped, Formatting, Configuration, Exporters, SequenceExhausted |
| `CleanupFailure` | Lỗi callback gốc hoặc unwind đã bắt được giữ lại |

Phần lớn enum lỗi trên là non-exhaustive; xem Rustdoc trước khi match đầy đủ.
`ReflectError` hiện exhaustive nhưng API vẫn chưa ổn định.
Dùng `Error::source` để truy nguyên, không phân tích chuỗi Display.
Với aggregate lỗi event/exporter, xem mọi thành viên chứ không chỉ nguồn đầu.
`CleanupFailure::is_panic` phân biệt unwind đã bắt.

Từ chối tức thời và lỗi callback sau đó là hai giai đoạn khác nhau.
HostStopped/DriverPanicked không bao giờ là cleanup thành công.
Lỗi tiếp nhận không chạy setup; tài nguyên thu trước đăng ký bị từ chối vẫn
thuộc trách nhiệm ứng dụng.

## Kiểm tra cấu hình có cấu trúc

`ValidationError::new(Vec<ValidationIssue>)` giữ trường `issues` công khai.
Mỗi `ValidationIssue` có `message: String`, `path: Vec<PathSegment>`.
`PathSegment::Key(String)` và `Index(usize)` giữ vị trí trường/mảng;
đường dẫn rỗng là toàn cấu hình.

Trả lỗi này từ validator đồng bộ khi cần thông tin có cấu trúc.
Chấp nhận mọi lỗi validator chuyển được thành BoxError.
Serde, kiểm tra async và khám phá Standard Schema không phải API tự động.

## Payload hook có kiểu

`hooks::Value` bọc dữ liệu dị thể được kiểm tra. `new`, `from_rc`, `is::<T>`,
`get::<T>` giữ/kiểm tra hợp đồng Rust; `get` trả `Rc<T>` hoặc ServiceError.
Dùng `is` để phân biệt hợp đồng không liên quan mà không coi mọi sai khác là
lỗi vận hành.

| Hàm hook | Payload / trường công khai | Mode / mục đích |
|---|---|---|
| `hooks::plugin()` | `FiberInfo { id, name, state, live }` | Quan sát tiếp nhận/loại bỏ |
| `hooks::status()` | `Status { fiber, previous }` | Quan sát chuyển trạng thái |
| `hooks::commit()` | `Commit { fiber, source }` | Commit nguồn; no_save bỏ thông báo |
| `hooks::config()` | `Config { fiber, source }` | Around-transform đồng bộ, kết quả Value |
| `hooks::update()` | `Update { fiber, resolved }` | Around-update async, kết quả bool |
| `hooks::service()` | `ServiceChange { name, slot, value }` | Quan sát sẵn có theo realm chính xác |
| `hooks::get()` | `Get { context, name }` | Around-get đồng bộ, kết quả Option<Value> |
| `hooks::set()` | `Set { context, name, value }` | Around-set đồng bộ, kết quả () |
| `hooks::listener()` | `ListenerRequest` | Responder đồng bộ; Break cung cấp đăng ký thay thế |
| `hooks::dispatch()` | `Dispatch { mode, event, arguments, context }` | Quan sát dispatch không nội bộ |

Hook quan sát dùng responder đồng bộ trả Continue.
`FiberInfo::live` mô tả tiếp nhận, không phải hoàn tất cleanup.
Các trường là ảnh chụp giá trị; giữ chúng không giữ Fiber.
Giá trị nguồn và giá trị update đã kiểm tra thuộc các giai đoạn khác nhau.
Kiểu `C` và `R` có thể khác nhau hoặc giống nhau.

`ListenerRequest` có các phương thức `context`, `event`, `listener`, `options`.
`replace`/`replace_with` thay Listener đúng hợp đồng, giữ sở hữu/once gốc và
tránh interception đệ quy. Khôi phục Event/Listener có kiểu qua `Value::get`.

Ưu tiên `Context::on_update`/`on_update_sync` cho logic update từng Fiber.
Middleware update cục bộ trước hook global thông thường; hook global prepend
trước router cục bộ. Hook vẫn là đăng ký có phạm vi; phần triển khai giao thức
tích hợp tồn tại qua các lần drain gốc.

## Ranh giới an toàn

Lỗi observer được chẩn đoán mà không làm kẹt cleanup; lỗi middleware biến
đổi/update vẫn đến caller của thao tác. Không tự sửa panic invariant framework,
abort, destructor panic hay chu trình task/Rc do người dùng tạo.

Không coi payload hook là bản ghi runtime có thể sửa, hoặc dùng truy cập trực
tiếp để bỏ qua yêu cầu vòng đời phụ thuộc. Xem
[giới hạn tương thích](../../m6.md#remaining-restrictions-and-safety-deviations).
