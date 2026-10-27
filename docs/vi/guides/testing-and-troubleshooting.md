# Kiểm thử và xử lý sự cố

[English](../../en/guides/testing-and-troubleshooting.md) | [Trang tiếng Việt](../README.md)

## Kiểm thử các mốc quan sát được

Chạy mỗi kiểm thử từ điểm vào Tokio và điều khiển một phạm vi `Runtime::run`.
Dùng channel/gate để kiểm soát setup và cleanup async, không dùng sleep.
Assert trạng thái vòng đời trả về khi yêu cầu Active hoặc Disposed.

Các điều kiện hữu ích gồm số lần apply/cleanup chính xác, lần đọc cuối từ
provider cũ diễn ra trước khi provider đóng, không có chẩn đoán trên nhánh
thành công, Registry/effect trở về mốc ban đầu sau teardown.
`Registry::len` đếm nhóm khai báo, còn `instances` trả định danh của một khai báo.

Với trường hợp phức tạp, tham khảo mẫu trong
[updates](../../../crates/re-cordis-core/tests/updates.rs),
[service disposal](../../../crates/re-cordis-core/tests/service_dispose.rs) và
[lifecycle parity](../../../crates/re-cordis-core/tests/lifecycle_parity.rs).
Kiểm thử có thể dùng feature Tokio chỉ bật ở dev-dependencies; phụ thuộc ứng
dụng phải bật feature thực sự sử dụng.

## Triệu chứng thường gặp

| Triệu chứng | Kiểm tra / xử lý |
|---|---|
| `HostNotRunning` | Tiếp nhận công việc trong `run` của runtime sở hữu |
| `WrongHost` | Không trộn Context, bound service hay thao tác từ runtime độc lập |
| `MissingExecutor` / `SpawnedTask` | Điều khiển `run` từ Tokio block_on/main/test, không từ task được spawn |
| `NestedHost` / `AlreadyRunning` | Không lồng host Cordis hay điều khiển đồng thời |
| Fiber luôn Pending | Kiểm tra tên chính xác, ô cô lập, predicate readiness và provider Active |
| `ready` trả về nhưng consumer chưa Active | Pending là kết quả hợp lệ; chờ provider rồi consumer riêng |
| Fiber vẫn Failed sau refresh provider | `update` Fiber Failed tường minh; `restart` không xóa lỗi |
| Update không hợp lệ không thay gì | Kiểm tra khi loaded chủ ý giữ nguồn hợp lệ trước đó |
| `Undeclared` / `Inactive` | Khai báo injection; không bypass quyền để che lỗi vòng đời |
| `set` không reload consumer | Set thay giá trị, không thay định danh publication; dùng thay thế vòng đời provider |
| Tài nguyên đóng dù giữ `Rc<T>` | Giá trị trả về không giữ tài nguyên provider; dùng injection đã commit |
| Event `UnsupportedMode` | Khớp responder/around và mode sync/async |
| Không có log ở terminal | Đăng ký exporter; sink mặc định là bộ đệm trong bộ nhớ |
| `ReentrantDrain` | Không chờ vòng đời self/tổ tiên sở hữu mình trong setup/cleanup đã quy thuộc |
| `InactiveContext` | Dừng tạo công việc bằng Context của owner đã kết thúc |
| `RuntimeDropped` / `HostStopped` | Giữ owner sống và tiếp tục thực thi đến mốc hoàn tất |
| Cleanup hoàn tất nhưng đóng tài nguyên lỗi | Kiểm tra `CleanupReport::failures` và lỗi được runtime giữ lại |

## Điều tra lỗi

Truyền lỗi tiếp nhận tức thời lên và chờ kết quả thao tác đã bắt đầu.
Kiểm tra variant có cấu trúc và `std::error::Error::source`, không chỉ chuỗi lỗi.
Nhiều enum lỗi công khai là non-exhaustive: có nhánh dự phòng khi match từ
ứng dụng bên ngoài.

Core bắt unwind callback ở các ranh giới đã mô tả; không bảo đảm sửa panic
invariant, abort, destructor panic hay chu trình `Rc` tùy ý.
Không dùng catch quá rộng hay fallback im lặng để biến teardown lỗi thành thành công.

## Các lệnh kiểm tra repository

Chạy từ thư mục gốc:

```sh
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo doc -p re-cordis-core --no-deps --locked
```

Về chọn MSRV/stable tường minh trong môi trường này, xem
[xác minh M6](../../m6.md#verification). Đối chiếu TS đầy đủ là lệnh opt-in riêng,
cần checkout cục bộ đúng bản chốt và phụ thuộc được chuẩn bị;
xem [cách chạy parity](../../lifecycle-parity.md#reproduce).

Các kiểm tra tài liệu nằm ở [trang tài liệu chung](../../README.md#maintaining-these-documents).
Chúng không yêu cầu truy cập GitHub trực tiếp.
