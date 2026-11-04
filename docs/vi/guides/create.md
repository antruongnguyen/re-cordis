# Tạo ứng dụng Rust ngoại tuyến

[English](../../en/guides/create.md) | [Trang tiếng Việt](../README.md)

`re-cordis-create` sinh một ứng dụng nhỏ dùng core, console logger và loader
chưa phát hành từ checkout cục bộ. S0–S2 ngoại tuyến đã được nghiệm thu độc lập
trên stable/1.85.0; xem [báo cáo nghiệm thu](../../plans/2026-11-02-cordis-completion/evidence/create-acceptance.md).

## Sinh và chạy ứng dụng

Dùng Rust 1.85 trở lên và checkout repository này. Chạy các lệnh sau từ thư mục
gốc; thư mục cha của đích phải tồn tại, còn `../my app` phải chưa tồn tại hoặc
là thư mục thông thường rỗng.

```sh
cargo run -p re-cordis-create --offline --locked -- \
  my-app --path '../my app' --cordis-path .
cargo test --manifest-path '../my app/Cargo.toml' --offline
cargo run --manifest-path '../my app/Cargo.toml' --offline --locked
```

Bản thân generator chỉ dùng thư viện chuẩn và không khởi chạy subprocess.
Các lệnh Cargo tường minh này cần dependency có sẵn trong cache cục bộ.
Nếu thiếu dependency, Cargo ngoại tuyến báo lỗi; generator không tải về.
Lệnh kiểm thử ứng dụng đầu tiên tạo lockfile của ứng dụng.

Xem trợ giúp lệnh:

```sh
cargo run -p re-cordis-create --offline --locked -- --help
```

| Đối số | Ý nghĩa |
|---|---|
| `NAME` | Tên package Cargo; bắt đầu bằng chữ ASCII thường, sau đó là chữ thường, chữ số, `_` hoặc `-`, tối đa 64 byte. Từ dành riêng Rust/Cargo, tên thiết bị Windows, và `re-cordis-core`, `re-cordis-logger`, `re-cordis-loader` bị từ chối. |
| `--path PATH` | Đích; mặc định là `NAME` tương đối với thư mục làm việc. Đường dẫn native cho phép dấu cách và Unicode. |
| `--cordis-path PATH` | Checkout repository cục bộ bắt buộc, cung cấp core, logger và loader. |
| `-h`, `--help` | In cách dùng và thoát thành công, không sinh tệp. |

Mã thoát là `0` khi thành công/trợ giúp, `2` khi lỗi cách dùng hoặc validation,
và `1` khi lỗi I/O hoặc dọn staging. Lỗi nêu bước kiểm tra hoặc thao tác tệp bị thất bại.
Xem [API Create](../api/create.md) để gọi từ thư viện.

## Nội dung ứng dụng

Bộ tệp cố định gồm `Cargo.toml`, `src/main.rs`, `src/app.rs`, `README.md`
và `.gitignore`. Đọc README được sinh và `src/app.rs` để tùy chỉnh.
[Mã nguồn generator](../../../crates/re-cordis-create/src/lib.rs) ghi
[ứng dụng đi kèm](../../../crates/re-cordis-create/templates/basic/app.rs).

Manifest có `[workspace]` riêng với resolver 3 để ứng dụng độc lập với workspace bao quanh.
Ứng dụng dùng Rust 2024/MSRV 1.85, chưa phát hành, và lưu dependency theo đường
dẫn tuyệt đối canonical đến ba crate Cordis cục bộ.
Không tham chiếu bản phát hành `re-cordis` giả định trên crates.io.
Checkout phải giữ cách viết manifest/layout của repository này; validation
không phải parser Cargo TOML tổng quát hay kiểm tra mã nguồn. Đường dẫn dependency
canonical phải là Unicode để ghi vào Cargo TOML. Nếu chuyển checkout, cần sửa chúng.

Ứng dụng chạy `Runtime::run` cục bộ từ Tokio current-thread, đăng ký writer
console thuộc phạm vi chủ sở hữu, công bố và đọc service có kiểu, khởi động
entry loader bằng cấu hình lập trình, cập nhật cấu hình và chờ shutdown
tường minh. Lỗi thao tác, báo cáo cleanup và lỗi runtime đều được kiểm tra.
Kiểm thử của ứng dụng xác nhận giải phóng tài nguyên và báo lỗi. Mở rộng chương
trình đó theo hướng dẫn [loader](loader.md), [service](services-and-scoping.md)
và [cleanup](effects-and-cleanup.md).

## Đích và phạm vi

Generator từ chối tệp, symlink hoặc thư mục không rỗng đã tồn tại, và đích
kết thúc bằng `.` hoặc `..`.
Nó ghi đủ năm tệp trong thư mục staging cùng cha trước một lần rename,
rồi kiểm tra đích lại trước khi commit. Lỗi dọn staging được báo rõ.
Lỗi ghi nội dung không đưa tệp template vào đích cuối; rename thư mục rỗng vẫn
có thể lỗi trên một nền tảng/filesystem cụ thể.

Generator không khởi tạo Git, cài dependency hay chạy ứng dụng.
Không có template từ xa, downloader, archive, cache, biến thể Include/HMR
hay tùy chọn package manager. Thao tác filesystem portable không bảo đảm
confinement khi bên đối nghịch thay thư mục cha đồng thời, hay độ bền khi mất
điện. Bằng chứng filesystem thực thi giới hạn ở máy macOS đã kiểm thử; xem
[giới hạn](../../limitation.md#create-offline-scaffolding).
