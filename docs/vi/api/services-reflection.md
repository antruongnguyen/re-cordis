# Dịch vụ, lời gọi bound và adapter reflection

[English](../../en/api/services-reflection.md) | [Mục lục API](README.md)

Mã nguồn: [service.rs](../../../crates/cordis-core/src/service.rs),
[reflect.rs](../../../crates/cordis-core/src/reflect.rs).
Hướng dẫn có mã chạy: [dịch vụ và phạm vi](../guides/services-and-scoping.md).

## Key, yêu cầu và publication

| Thao tác | Hợp đồng |
|---|---|
| `ServiceKey::<T>::new(name)`, `name` | Tên chọn ô; T kiểm tra hợp đồng giá trị |
| `ServiceKey::required` | Tạo `Dependency` bắt buộc |
| `Dependency::with_config(config)` | Gắn cấu hình injection có kiểu |
| `Context::provide(&key, value)` | Đăng ký publication chính xác thuộc Fiber; provider sống trùng gây lỗi |
| `Context::provide_checked(&key, value, check)` | Thêm predicate sẵn sàng đồng bộ `&T -> Result<bool, E>` |
| `Context::set(&key, value)` | Thay giá trị hiện tại; chỉ Fiber cung cấp chính xác được làm |
| `ServiceRegistration::id` | Mượn `PublicationId` của lần công bố này |
| `ServiceRegistration::withdraw` | Nhận quyền rút ngay; trả Completion chờ teardown các phụ thuộc chính xác |
| `ServiceRegistration::into_effect` | Chuyển cleanup qua `Disposer::from_effect` |

Readiness false chặn injection. Lỗi/panic predicate được chẩn đoán cho provider
và chặn injection, không biến thành sẵn sàng thành công.
Publication trong lúc provider Loading chưa sẵn có cho consumer inject.

Set giữ định danh, không thông báo consumer hay kiểm tra lại readiness.
Rút/công bố lại tạo lần mới. Thiếu provider bắt buộc để consumer Pending, không
phải Failed.

## Tra cứu và vòng đời

| Thao tác | Kết quả / hành vi |
|---|---|
| `Context::get` | `Result<Option<Rc<T>>, ServiceError>` theo quyền own/khai báo gần nhất đã commit; root có bypass tường minh |
| `get_direct` | Cùng dạng kết quả; bỏ kiểm tra quyền, yêu cầu provider Active sẵn có |
| `get_unchecked` | Đọc trực tiếp, kể cả provider Loading/Unloading vẫn trong ô |
| `bind` | `Result<BoundService<T>, ServiceError>` với quyền được kiểm tra và nơi định nghĩa chính xác |
| `bind_direct` | Đọc bound bỏ kiểm tra quyền nhưng yêu cầu sẵn có |

Tra cứu có kiểm tra ưu tiên provision của chính owner, rồi góc nhìn đã commit
của khai báo gần nhất, đi theo tổ tiên nơi định nghĩa. Khai báo không hoạt động
là lỗi, không cho phép fallback. Đọc ở root khi thiếu trả None.
Giá trị service `Option<T>` biểu diễn giá trị tùy chọn tường minh, tách biệt
với việc publication có tồn tại hay không.

Con trỏ trả về là ảnh chụp giá trị, không giữ vòng đời tài nguyên.
Injection đã commit giữ provider chính xác qua setup/rollback/cleanup,
kể cả consumer rời Registry. Cleanup cũ vẫn dùng publication đã rút cho đến
khi nhả phụ thuộc đó.

## BoundService và phạm vi

| Thao tác | Hợp đồng |
|---|---|
| `call` | Chạy adapter nhận `(&T, Context)` |
| `context`, `caller` | Xem Context đã truy vết và Context định nghĩa của mã truy cập |
| `call_unshadowed` | Adapter nhận giá trị, Context sử dụng không shadow và caller |
| `rebind` | Truy vết lại tại nơi sử dụng mới trong cùng runtime; không lồng shadow định nghĩa |
| `map` | Mở rộng giá trị/holder, giữ góc nhìn định nghĩa/sử dụng |
| `intercepts::<C>` | Cấu hình injection gần nhất trước, rồi lớp nơi sử dụng từ ngoài vào trong |
| `resolve_intercept` | Truyền lớp có kiểu cho hàm gộp tường minh của bạn |
| `dispatch_options` | Bộ lọc listener theo realm dịch vụ nơi sử dụng |
| `associated(property)` | Bind tên dịch vụ có dấu chấm bằng quyền của mã truy cập gốc |
| `associated_key(&key)` | Bind namespace mở rộng tường minh bằng quyền gốc |
| `read_accessor`, `write_accessor` | Dùng quyền gốc và receiver có kiểu |
| `invoke(args)` | Gọi `ServiceInvoke<A>` qua Context đã truy vết |

`ServiceInvoke<A>` khai báo associated type `Output` và
`invoke(&self, Context, A) -> Result<Self::Output, BoxError>`.
Lời gọi phương thức Rust thông thường không tự rebind.

`isolate`/`isolate_with` của Context chọn ô không fallback cha, có ranh giới
quyền injection kể cả khi dùng lại ID. `intercept` thêm lớp có kiểu.
Provision của chính owner không mượn cấu hình injection.
Kiểu cấu hình không khớp trả lỗi thay vì bỏ lớp im lặng.
`Context::service_filter` tạo bộ lọc realm tương đương; listener global vẫn
bỏ qua bộ lọc.

## Accessor có kiểu

| Thao tác | Hợp đồng |
|---|---|
| `AccessorKey::<R, T>::new(name)`, `name` | Tên thuộc tính toàn cục với hợp đồng receiver/giá trị |
| `Accessor::new(get)` | Adapter chỉ đọc `(Context, Rc<R>) -> Result<T, E>` |
| `with_setter(set)` | Thêm `(Context, Rc<R>, T) -> Result<(), E>` |
| `from_service(key, get)` | Adapter mixin có kiểu nhận `(&S, Context, Rc<R>)` |
| `Context::accessor(&key, accessor)` | Đăng ký thuộc owner, trả Effect của khai báo |
| `read_accessor(&key, Rc<R>)` | Đọc bằng receiver tường minh và quyền mã truy cập |
| `write_accessor(&key, Rc<R>, value)` | Ghi qua setter đã khai báo |

Dùng interior mutability thông thường để sửa receiver. Đây là adapter tường
minh, không phải prototype động hay reflection thuộc tính tùy ý.

## Lỗi

`ServiceError`: lỗi tiếp nhận/hook, provider trùng, sai kiểu, truy cập chưa
khai báo/không hoạt động, chưa provision hoặc sai owner.
`ReflectError`: tiếp nhận, khai báo trùng/thiếu, sai hợp đồng, chỉ đọc hoặc lỗi callback.

Xem [lỗi](errors-hooks.md) về truy nguyên và cách match.
