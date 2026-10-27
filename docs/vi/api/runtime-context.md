# Runtime, Context và định danh

[English](../../en/api/runtime-context.md) | [Mục lục API](README.md)

Mã nguồn: [runtime.rs](../../../crates/cordis-core/src/runtime.rs),
[context.rs](../../../crates/cordis-core/src/context.rs),
[identity.rs](../../../crates/cordis-core/src/identity.rs).

## Runtime

| Thao tác | Hợp đồng |
|---|---|
| `Runtime::new`, `Default` | Tạo host độc lập và gốc thường trực; không cần executor khi tạo |
| `context` | Trả Context gốc nhẹ, không sở hữu |
| `run(future)` | Điều khiển task host cục bộ đến khi future đầu vào hoàn tất; trả `Result<F::Output, HostError>` |
| `take_failures` | Lấy lỗi callback framework được giữ lại dưới dạng `Vec<CleanupFailure>` |

`Runtime` là chủ sở hữu tường minh, không Clone. Phải được điều khiển từ điểm
vào Tokio, không lồng, không điều khiển đồng thời, không từ task được spawn.
Nếu future đầu vào trả `Result`, `run` thêm lớp lỗi host bên ngoài; xử lý cả hai.

Kết thúc `run` tạm dừng chứ không dọn công việc cục bộ còn lại.
Hủy Runtime sẽ hủy thực thi. Context và Logger không giữ nó sống.

## Định danh và metadata của Context

| Thao tác | Hợp đồng |
|---|---|
| `runtime_id`, `fiber_id` | Mượn định danh opaque của runtime/owner đã chọn |
| `name` | Đọc tên chẩn đoán của Fiber |
| `extend(value)` | Thêm lớp metadata có kiểu, bất biến |
| `metadata::<T>` | Trả metadata cùng kiểu gần nhất dưới dạng `Option<Rc<T>>` |
| `root` | Chọn gốc thường trực, bỏ các lớp metadata/phạm vi dẫn xuất |
| `definition_site` | Góc nhìn quyền truy cập phụ thuộc của lời gọi đã truy vết |
| `use_site` | Góc nhìn nơi sử dụng không shadow cho metadata, cô lập, intercept, effect |
| `caller_site` | Góc nhìn nơi định nghĩa của mã gọi |

Metadata và định danh vẫn đọc được sau khi hủy host, nhưng thao tác cần trạng
thái runtime trả `RuntimeDropped`. Không coi clone Context là sở hữu tài nguyên
hay kéo dài vòng đời host.

## Phạm vi Context

| Thao tác | Hợp đồng |
|---|---|
| `isolate(&key)` | Tạo ranh giới ô dịch vụ mới |
| `isolate_with(&key, &id)` | Tạo ranh giới chọn ô chia sẻ |
| `isolation(&key)` | Tìm `IsolationId` đã chọn; runtime phải tồn tại |
| `intercept(&key, config)` | Thêm cấu hình có kiểu tại nơi sử dụng, không viết lại phương thức động |

Dùng lại ID ô không xóa ranh giới quyền injection kế thừa.
Ô cô lập không fallback ô cha. Xem [dịch vụ/reflection](services-reflection.md)
về quy tắc gộp và truy vết.

## Phương thức mở rộng Context theo nhóm

| Nhóm | Phương thức |
|---|---|
| [Plugin](plugins-registry.md) | `plugin`, `inject`, `inject_method`, `registry`, `on_update`, `on_update_sync` |
| [Tài nguyên](effects.md) | `on_dispose`, `on_dispose_sync`, `effect`, `effect_iter`, `effect_async`, `effect_stream`, `get_effects`, `drain` |
| [Dịch vụ](services-reflection.md) | `provide`, `provide_checked`, `set`, `get`, `get_direct`, `get_unchecked`, `bind`, `bind_direct`, `service_filter`, `accessor`, `read_accessor`, `write_accessor` |
| [Sự kiện](events.md) | `on`, `on_with`, `once`, `emit`, `parallel`, `serial`, `bail`, `waterfall`, `waterfall_async` |
| [Ghi log](logging.md) | `logger`, `log_buffer`, `add_exporter` |

## Định danh và ranh giới luồng

`RuntimeId`, `FiberId`, `PublicationId` do framework cấp.
Tạo định danh cô lập chia sẻ bằng `IsolationId::new`/`Default`.
Clone ID để chia sẻ đúng định danh đó; so sánh không dựa vào chuỗi hiển thị,
kiểu giá trị Rust hay chuỗi con trỏ.

Context/handle/future của framework mang tính cục bộ, không phải API `Send`/`Sync`.
Task do người dùng spawn không kế thừa quyền tiếp nhận.
Xem [khái niệm](../concepts.md) và [lỗi host/tiếp nhận](errors-hooks.md).
