# Effect và dọn dẹp

[English](../../en/api/effects.md) | [Mục lục API](README.md)

Mã nguồn: [effect.rs](../../../crates/re-cordis-core/src/effect.rs),
[setup.rs](../../../crates/re-cordis-core/src/setup.rs).
Hướng dẫn có mã chạy: [effect và dọn dẹp](../guides/effects-and-cleanup.md).

## Giá trị cleanup thuộc quyền sở hữu

| Thao tác | Hợp đồng |
|---|---|
| `Disposer::new` | Callback một lần trả future của `Result<(), E>` |
| `Disposer::sync` | Callback đồng bộ một lần trả `Result<(), E>` |
| `Disposer::compose` | Dọn con tuần tự theo thứ tự ngược; dừng chuỗi khi lỗi |
| `Disposer::from_effect` | Chuyển đăng ký chưa nhận quyền xử lý ra khỏi sổ |
| `Context::on_dispose` | Đăng ký cleanup async của tài nguyên đã có |
| `Context::on_dispose_sync` | Đăng ký cleanup đồng bộ tường minh |

Drop Disposer hoặc token đăng ký không thực hiện dọn dẹp.
Yield/đăng ký giá trị cleanup đã thu được. Effect chuyển đi chỉ có một chủ/quyền
xử lý, không dispose lặp. From-effect có lỗi khi đã bị nhận quyền xử lý hoặc
ghép đệ quy self/tổ tiên.

## Thu nhận tài nguyên

| Phương thức Context | Dạng setup |
|---|---|
| `effect` | `FnOnce() -> Result<Disposer, E>` |
| `effect_iter` | Iterator của `Result<Disposer, E>` |
| `effect_async` | Factory trả future của `Result<Disposer, E>` |
| `effect_stream` | Factory trả stream kết quả disposer |

Mỗi phương thức trả `Result<Effect, SetupError>`. Quyền sở hữu được giữ trước
khi setup chạy. Factory async có phần đầu chạy ngay; thân future được điều
khiển cục bộ. Lỗi giữa chừng rollback các con đã thu được.
Dispose/vô hiệu hóa dừng bước stream sau nhưng giữ lần thu nhận cuối đang
chạy và chờ dọn nó.

Rollback iterator toàn đồng bộ hoàn tất trước khi trả về. Rollback async có
thể tiếp tục trên host; readiness của stream cho phép chờ hoàn tất tường minh.

## Effect và quan sát

| Thao tác | Hợp đồng |
|---|---|
| `Effect::ready().await` | `Result<(), SetupError>` sau setup/rollback; không dispose |
| `with_label(label)` | Đặt nhãn quan sát, không đổi sở hữu |
| `dispose` | Nhận quyền cleanup chính xác ngay; trả `Result<Completion, AdmissionError>` |
| `Context::get_effects` | Trả ảnh chụp giá trị `Vec<EffectMeta>` theo thứ tự đăng ký |

`EffectMeta` có trường công khai `label: String`, `children: Vec<EffectMeta>`.
Metadata không sở hữu tài nguyên. Đăng ký chuyển giữ nhãn và thành con lồng;
disposer thuần không tạo node effect giả.

Dispose thủ công toàn đồng bộ hoàn tất ngay. Phần đầu factory của cleanup
async bắt đầu ngay nhưng thân vẫn async. Nếu thao tác khác đã nhận quyền đăng
ký, dispose thủ công báo không có lần thử; không chờ công việc của thao tác đó.

## Drain và hoàn tất

`Context::drain` nhận quyền ảnh chụp sổ hiện tại và trả Completion.
Callback độc lập tiến triển đồng thời sau khi được gọi theo thứ tự ngược.
Yêu cầu trong một drain chia sẻ lượt đó; đăng ký muộn cần lượt mới.
Gốc có thể tái sử dụng. Dispose vòng đời non-root còn chờ drain owner đang chạy
và tài nguyên được tiếp nhận muộn trước khi nhả provider.

`Completion` triển khai Future với đầu ra
`Result<CleanupReport, CompletionError>`. `CleanupReport::attempted` đếm callback
đã nhận quyền; `failures` mượn mọi lỗi callback giữ lại.
`CleanupFailure::is_panic` phân biệt unwind; `Error::source` giữ nguồn gốc.
Kết quả bên ngoài thành công không có nghĩa báo cáo không có lỗi.

Chờ self/tổ tiên sở hữu bị từ chối. Hủy host cho HostStopped, không phải dọn
dẹp thành công. Giữ completion thủ công đã nhận quyền riêng khi cần.

## Kiểu hỗ trợ và lỗi

`BoxError` là `Box<dyn std::error::Error>`, không có ràng buộc Send/Sync.
`LocalFuture<T>` là future cục bộ `'static` được box, không phải task an toàn
đa luồng hay độc lập executor.

`SetupError` phân biệt lỗi tiếp nhận, callback/rollback, `AlreadyClaimed`,
`RecursiveComposition`, `HostStopped`, `DriverPanicked`.
Bị từ chối tức thời không bao giờ có nghĩa bước thu nhận đã thành công.
Xem [lỗi và hook](errors-hooks.md).
