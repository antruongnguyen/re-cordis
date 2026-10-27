# Dịch vụ, tiêm phụ thuộc và phạm vi

[English](../../en/guides/services-and-scoping.md) | [Trang tiếng Việt](../README.md)

## Công bố khả năng, khai báo consumer

`ServiceKey<T>` đặt tên ô và kiểm tra kiểu giá trị. Tên và phạm vi cô lập chọn
định danh; riêng `T` không đủ. Hai hợp đồng không tương thích cho cùng ô là lỗi,
không phải hai dịch vụ độc lập.

Dùng injection thay vì đọc trực tiếp không hạn chế nếu consumer cần giữ tài
nguyên provider trong suốt setup và teardown.

```rust
use re_cordis_core::{BoxError, FiberState, Runtime, ServiceKey};
use std::io;

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                let key = ServiceKey::<u32>::new("count");
                let consumer = root.inject([key.required()], {
                    let key = key.clone();
                    move |ctx| {
                        let key = key.clone();
                        async move {
                            let value = ctx.get(&key)?.ok_or_else(|| {
                                io::Error::new(io::ErrorKind::NotFound, "count unavailable")
                            })?;
                            assert_eq!(*value, 7);
                            Ok::<_, BoxError>(())
                        }
                    }
                })?;
                assert_eq!(consumer.ready()?.await?, FiberState::Pending);
                let publication = root.provide(&key, 7)?;
                assert_eq!(consumer.ready()?.await?, FiberState::Active);
                let bound = root.bind(&key)?;
                assert_eq!(bound.call(|value, _ctx| *value), 7);
                publication.withdraw()?.await?;
                assert_eq!(consumer.ready()?.await?, FiberState::Pending);
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

Provision thuộc gốc tiện cho ứng dụng nhỏ. Với tài nguyên khởi tạo async,
công bố từ `Plugin::initialized`: consumer dùng injection chỉ chuyển sang Active
sau khi khởi tạo kết thúc thành công. Đọc trực tiếp không kiểm tra vẫn có thể
thấy publication trong Loading; thao tác đó không phải mốc chờ sẵn sàng.

## Chọn cách tra cứu có chủ ý

| Thao tác | Quyền / tính sẵn có |
|---|---|
| `get` | Provision của chính owner hoặc quyền từ injection đã commit/tổ tiên; gốc có bypass tường minh |
| `get_direct` | Bỏ kiểm tra injection nhưng yêu cầu provider Active và sẵn có |
| `get_unchecked` | Đọc trực tiếp, kể cả provider Loading/Unloading vẫn nằm trong ô |
| `bind` | Tra cứu có kiểm tra và truy vết nơi định nghĩa/sử dụng tường minh |
| `bind_direct` | Đọc bound từ provider sẵn có, bỏ kiểm tra quyền injection |

Truy cập chưa khai báo ở non-root là lỗi. Khai báo gần nhất không hoạt động
không được rơi về provider khác. Dùng Context được truyền vào adapter bound;
lời gọi phương thức trên `Rc<T>` thông thường không tự truy vết mã Rust.

## Thay giá trị không phải thay provider

Chỉ Fiber cung cấp mới được `set` publication của nó. Set giữ định danh và
không reload consumer hay kiểm tra lại predicate sẵn sàng.
`Rc<T>` đã trả về vẫn là ảnh chụp giá trị cũ.

Rút/công bố lại tạo publication mới. Cleanup của injection cũ giữ đúng lần
công bố cũ đến khi dùng xong, kể cả consumer đã rời Registry. `Rc<T>` hoặc
handle bound đơn thuần không tự giữ tài nguyên provider.

## Cô lập và intercept

`ctx.isolate(&key)` chọn ô mới. Chia sẻ `IsolationId` qua `isolate_with` nếu
hai Context cần cùng ô cô lập. Ô cô lập không có fallback tổ tiên. Kể cả chọn
lại cùng nhãn cũng tạo ranh giới quyền injection kế thừa.

`Dependency::with_config` cung cấp cấu hình injection có kiểu.
`ctx.intercept(&key, config)` thêm lớp tại nơi sử dụng.
`intercepts::<C>()` của dịch vụ bound đọc cấu hình injection gần nhất có quyền
trước, sau đó các lớp Context từ ngoài vào trong. `resolve_intercept` nhận phép
gộp do bạn cung cấp; framework không đoán cách gộp JSON/shallow.

Lời gọi bound dùng quyền phụ thuộc của nơi định nghĩa provider, nhưng cấu hình
và sở hữu effect từ nơi sử dụng của caller. Gọi từ root không trao quyền root
cho dịch vụ định nghĩa trong plugin.

## Adapter nâng cao

Dùng `AccessorKey`/`Accessor` cho thuộc tính có kiểu tường minh,
`Accessor::from_service` cho mixin, `ServiceInvoke` cho dịch vụ có thể gọi.
`map`, `rebind`, `associated`, `associated_key` của bound giữ truy vết tường
minh, không tạo prototype động. `service_filter`/`dispatch_options` chọn listener
theo realm dịch vụ; listener global bỏ qua bộ lọc.

Tra cứu: [API dịch vụ và reflection](../api/services-reflection.md).
Ví dụ tài nguyên hoàn chỉnh: [lifecycle.rs](../../../crates/re-cordis-core/examples/lifecycle.rs).
