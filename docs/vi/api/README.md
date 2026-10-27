# Tài liệu API

[English](../../en/api/README.md) | [Trang tiếng Việt](../README.md)

Tài liệu bao phủ export công khai của `re-cordis-core`, không phải bộ lưu trữ nội bộ
hay handle riêng của ví dụ. Nội dung mô tả cách dùng và hợp đồng hành vi;
Rustdoc được sinh từ mã nguồn là tham chiếu chính xác cho chữ ký, ràng buộc
generic, trait implementation và trường công khai.

## Sinh tài liệu API chính xác

Tại thư mục gốc repository:

```sh
cargo doc -p re-cordis-core --no-deps --locked
```

Mở `target/doc/re_cordis_core/index.html`, hoặc đường dẫn tương ứng trong
`CARGO_TARGET_DIR`. Lệnh không phát hành gì lên docs.rs.
Package `0.0.0` chưa phát hành không phải đích cài đặt crates.io được hỗ trợ.

## Bản đồ export công khai

Mỗi export gốc từ [lib.rs](../../../crates/re-cordis-core/src/lib.rs) thuộc một
chương bên dưới. Phương thức mở rộng Context được phân theo chức năng.

| Chương | Kiểu / module công khai |
|---|---|
| [Runtime và Context](runtime-context.md) | `Runtime`, `Context`, `RuntimeId`, `FiberId`, `PublicationId`, `IsolationId` |
| [Plugin và Registry](plugins-registry.md) | `Plugin`, `FiberHandle`, `FiberState`, `LifecycleTask`, `UpdateOptions`, `Registry`, `LifecycleError` |
| [Dịch vụ và reflection](services-reflection.md) | `ServiceKey`, `Dependency`, `ServiceRegistration`, `BoundService`, `ServiceInvoke`, `AccessorKey`, `Accessor`, `ServiceError`, `ReflectError` |
| [Effect và dọn dẹp](effects.md) | `Disposer`, `Effect`, `EffectMeta`, `Completion`, `CleanupReport`, `CleanupFailure`, `LocalFuture`, `BoxError`, `SetupError` |
| [Sự kiện](events.md) | `Event`, `Listener`, `ListenerOptions`, `DispatchOptions`, `ListenerRegistration`, `Next`, `AsyncNext`, `EventTask`, `EventError` |
| [Ghi log](logging.md) | `Logger`, `LoggerConfig`, `Level`, `LogRecord`, `WeakFiber`, `ErrorValue`, `ColorDepth`, `ExporterOptions`, `ExporterRegistration`, `LogError` |
| [Lỗi và hook](errors-hooks.md) | `HostError`, `AdmissionError`, `CompletionError`, `ValidationError`, `ValidationIssue`, `PathSegment`, module công khai `hooks` |

## Đọc kết quả phương thức theo hai giai đoạn

Thao tác như `fiber.update(source)?` có thể lỗi tức thời trước tiếp nhận;
task trả về có thể lỗi sau đó khi được chờ. `Completion`, `LifecycleTask`,
`EventTask` là handle chỉ chờ, không phải lệnh bắt đầu khi poll lần đầu.
Giữ host tồn tại và được điều khiển đến lúc chúng hoàn tất.

`Effect::ready` là chờ setup async, không phải lệnh cleanup.
Ghi log và đọc thuần có yêu cầu tiếp nhận khác với đăng ký công việc mới;
xem từng chương thay vì cho rằng mọi phương thức đều cần `run` hoặc đều dùng
được bên ngoài nó.

## Tính ổn định và tương thích

Mọi API hiện chưa ổn định. Nhiều lỗi có `#[non_exhaustive]`; match ở ứng dụng
phải cho phép variant tương lai. ID là định danh opaque, so sánh/hash được;
không có hợp đồng số hóa, serialization hay thứ tự.

Tham chiếu ngữ nghĩa là Cordis TypeScript tại bản chốt, cùng các biểu diễn Rust
và khác biệt an toàn tường minh. Tài liệu cột mốc lịch sử là bằng chứng, không
thay thế việc kiểm tra chữ ký Rust hiện tại.
