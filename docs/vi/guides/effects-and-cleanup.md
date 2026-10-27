# Effect, sở hữu tài nguyên và dọn dẹp

[English](../../en/guides/effects-and-cleanup.md) | [Trang tiếng Việt](../README.md)

## Đăng ký dọn dẹp với đúng chủ sở hữu

Dùng `on_dispose`/`on_dispose_sync` khi tài nguyên đã được thu nhận.
Dùng `effect`/`effect_async` khi framework cần giữ quyền sở hữu trước setup.
`Disposer` là giá trị dọn dẹp: phải đăng ký hoặc yield nó; thả nó không chạy cleanup.

```rust
use re_cordis_core::{BoxError, Disposer, Runtime};
use std::{cell::RefCell, convert::Infallible, rc::Rc};

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let order = Rc::new(RefCell::new(Vec::new()));
            let result = async {
                let first = order.clone();
                let second = order.clone();
                let resource = root.effect(move || {
                    Ok::<_, Infallible>(Disposer::compose(vec![
                        Disposer::sync(move || {
                            first.borrow_mut().push(1);
                            Ok::<_, Infallible>(())
                        }),
                        Disposer::sync(move || {
                            second.borrow_mut().push(2);
                            Ok::<_, Infallible>(())
                        }),
                    ]))
                })?;
                let report = resource.dispose()?.await?;
                if let Some(failure) = report.failures().first() {
                    return Err(failure.clone().into());
                }
                assert_eq!(*order.borrow(), [2, 1]);
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

## Chọn API thu nhận tài nguyên

| API | Hình thức setup |
|---|---|
| `effect` | Setup đồng bộ trả về một disposer |
| `effect_iter` | Iterator đồng bộ trả kết quả disposer |
| `effect_async` | Factory trả future tạo một disposer |
| `effect_stream` | Factory tạo stream kết quả disposer |

Setup async bắt đầu trên host; chờ `Effect::ready` để quan sát setup và rollback
khi lỗi. Dispose không thả bước thu nhận đang chạy: nó lấy disposer cuối cùng
và chờ dọn dẹp xong trước khi kết thúc. Stream bị vô hiệu hóa sẽ dừng trước khi
chuyển sang bước thu nhận tiếp theo.

Rollback toàn đồng bộ của iterator hoàn tất ngay. Nếu rollback phải chờ cleanup
async, công việc tiếp tục trên host; dùng stream setup và mốc readiness khi cần
chờ tường minh rollback đó.

## Không có một ngăn xếp LIFO toàn cục

Cleanup cấp cao nhất độc lập được gọi theo thứ tự ngược đăng ký và tiến triển
đồng thời. Không dựa vào thứ tự hoàn thành của các cleanup ngang hàng.

`Disposer::compose` chờ từng con tuần tự theo thứ tự ngược.
Lỗi ở một con dừng chuỗi ghép; cleanup độc lập của owner vẫn được thử.
Chỉ dùng composition khi quan hệ phụ thuộc/thứ tự là có chủ ý.

`Disposer::from_effect` chuyển đăng ký chưa nhận quyền xử lý vào composition.
Listener, exporter và đăng ký dịch vụ có `into_effect`.
Việc chuyển loại bỏ quyền xử lý trong sổ cũ để không dọn hai lần.
Đã bị nhận quyền xử lý và chuyển đệ quy self/tổ tiên đều là lỗi tường minh.

## Quy trình dừng ứng dụng

Dừng đầu vào, join worker thuộc sở hữu ứng dụng, rồi chờ dispose Fiber/drain gốc
trong khi runtime vẫn chạy. Effect không tự sở hữu mọi task do người dùng spawn:
effect có thể giữ tín hiệu dừng và chờ join handle trong disposer.

`Completion` cho kết quả `Result<CleanupReport, CompletionError>`.
Kiểm tra `report.failures()`; thành công bên ngoài chỉ có nghĩa đã đến mốc
hoàn tất, không phải mọi callback thành công. Lỗi cleanup vòng đời được chẩn
đoán và giữ lại; xem `Runtime::take_failures` để lấy lỗi không trả qua report.

Drain gốc bao phủ một ảnh chụp có thể tái sử dụng. Đăng ký sau ảnh chụp cần lần
drain khác. Cleanup thủ công đã được nhận quyền riêng có handle hoàn tất riêng;
giữ và chờ nó. Nhận quyền lặp lại với số lần thử bằng không không chờ công việc
của thao tác khác.

Không chờ vòng đời của chính mình hoặc tổ tiên sở hữu mình trong setup/cleanup
đã được quy thuộc. Framework từ chối các lần chờ đó để tránh deadlock.
Điều này không phát hiện mọi chu trình tác vụ do người dùng tạo.

Tra cứu: [API effect](../api/effects.md).
