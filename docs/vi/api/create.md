# Create: sinh dự án ngoại tuyến

[English](../../en/api/create.md) | [Mục lục API](README.md)

Mã nguồn: [lib.rs](../../../crates/re-cordis-create/src/lib.rs).
CLI: [main.rs](../../../crates/re-cordis-create/src/main.rs).
Hướng dẫn: [tạo ứng dụng](../guides/create.md).

`re-cordis-create` (`0.0.0`, chưa phát hành, Rust 2024, MSRV 1.85) cung cấp
một generator đồng bộ chỉ dùng thư viện chuẩn và binary cùng tên.
Không cần Context, Runtime hay host framework. Triển khai và nghiệm thu S0–S2
được ghi trong [gói công việc](../../plans/2026-11-02-cordis-completion/create.md).
[Hợp đồng Rust](../../plans/2026-11-02-cordis-completion/evidence/create-contract.md)
ghi các ranh giới đã chọn.

## `create`

Chữ ký: `create(name: &str, destination: &Path, cordis_path: &Path) -> Result<PathBuf, CreateError>`.

Tên bắt đầu bằng chữ ASCII thường, chỉ chứa chữ thường/chữ số/`_`/`-`,
tối đa 64 byte. Từ dành riêng Rust/Cargo và tên thiết bị Windows bị từ chối.
Đích riêng biệt giữ nguyên đối số đường dẫn native, gồm dấu cách và Unicode;
filesystem có thể từ chối chuỗi byte khác bằng lỗi I/O.
Thư mục cha phải tồn tại; đích phải chưa tồn tại hoặc là thư mục thông thường
rỗng. Tệp, symlink, thư mục không rỗng và đích cuối `.`/`..` bị từ chối.

Checkout được kiểm tra manifest package core, logger, loader và các tệp
`src/lib.rs` tương ứng. Nó yêu cầu cách viết `[workspace]`, `[package]`/tên
package hiện tại; đây là kiểm tra layout, không phải parser Cargo TOML tổng quát
hay kiểm tra mã nguồn. Đường dẫn dependency là đường dẫn Unicode tuyệt đối
canonical, được escape thành chuỗi TOML. Ứng dụng có `[workspace]` riêng với resolver 3, Rust 2024/MSRV 1.85,
chưa phát hành. Template cố định ghi đúng `Cargo.toml`, `src/main.rs`,
`src/app.rs`, `README.md` và `.gitignore`.

Nội dung được ghi đủ trong thư mục staging cùng cha trước một lần rename.
Đích được kiểm tra trước khi ghi và trước commit; rename chuẩn từ chối đích
trở thành không rỗng do thay đổi đồng thời. Lỗi dọn staging được báo rõ.
Đường dẫn tuyệt đối trả về dùng thư mục cha canonical, chỉ thư mục ứng dụng hoàn tất.
Thư mục rỗng đã tồn tại vẫn có thể gây lỗi rename tùy nền tảng.

## `CreateError`

| Variant | Trường / hợp đồng |
|---|---|
| `Validation` | Thông báo `String` cho tên, đích/layout checkout không hợp lệ và đầu vào thiếu. |
| `Io` | `operation: &'static str`, `path: PathBuf`, `source: std::io::Error`; giữ thao tác filesystem lỗi. |
| `Cleanup` | `original: Box<CreateError>`, `staging: PathBuf`, `source: std::io::Error`; giữ lỗi gốc và lỗi dọn dẹp, nêu staging cần xóa thủ công. |

`exit_code(&self) -> u8` trả `2` cho validation và `1` cho I/O/dọn dẹp.
Display cung cấp thông báo CLI. `Error::source` không có cho validation,
là lỗi I/O gốc cho `Io`, và CreateError gốc cho `Cleanup`; lỗi I/O dọn dẹp
có trong trường `source`. Thiếu thư mục cha/checkout/manifest và sai identity
là lỗi validation; lỗi đọc/canonicalize khác giữ nguồn I/O.
Sinh Rustdoc chính xác cục bộ:

```sh
cargo doc -p re-cordis-create --no-deps --offline --locked
```

Mở `target/doc/re_cordis_create/index.html` (điều chỉnh theo `CARGO_TARGET_DIR`).

## CLI

`re-cordis-create NAME --cordis-path PATH [--path PATH]` dùng cùng generator.
`--path` mặc định là `NAME`; `--cordis-path` bắt buộc.
`-h`/`--help` in cách dùng, không sinh tệp và thoát `0`.
Option không biết hoặc đối số thiếu/trùng thoát `2`; sinh thành công thoát
`0`, validation thoát `2`, I/O hoặc dọn dẹp thoát `1`.

Binary đọc đối số đường dẫn native, không yêu cầu chuyển sang Unicode.
Nó không chạy subprocess Git/cài đặt/khởi động và không thao tác mạng.
Không có template từ xa, option archive/cache, biến thể Include/HMR
hay tự giải dependency. Các lệnh Cargo ứng dụng tường minh nằm trong
[hướng dẫn](../guides/create.md).

Staging/rename portable không phải confinement chống thay thư mục cha đối
nghịch hay giao thức bảo đảm bền vững khi mất điện. Bằng chứng filesystem
thực thi chỉ có macOS; xem [giới hạn](../../limitation.md#create-offline-scaffolding).
