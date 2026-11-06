# Tạo ứng dụng Rust ngoại tuyến

[English](../../en/guides/create.md) | [Trang tiếng Việt](../README.md)

`re-cordis-create` đã phát hành bản thử nghiệm `0.1.0-alpha.1`. Công cụ sinh
một ứng dụng nhỏ dùng core, console logger và loader từ checkout cục bộ tương thích. S0–S2 ngoại tuyến đã được nghiệm thu độc lập
trên stable/1.85.0; xem [báo cáo nghiệm thu](../../plans/2026-11-02-cordis-completion/evidence/create-acceptance.md).

## Cài CLI đã phát hành

Dùng Rust 1.85 trở lên. Cài CLI và lấy checkout tương thích đầy đủ;
thư mục cha của đích phải tồn tại và đích phải chưa tồn tại hoặc rỗng:

```sh
cargo install re-cordis-create --version '=0.1.0-alpha.1' --locked
git clone https://github.com/antruongnguyen/re-cordis.git ../re-cordis-alpha
git -C ../re-cordis-alpha checkout 84fa9bdf0eb4e631fce792bf163494b0a61aeba8
re-cordis-create my-app --path '../my app' --cordis-path ../re-cordis-alpha
cargo fetch --manifest-path '../my app/Cargo.toml'
cargo test --manifest-path '../my app/Cargo.toml' --offline --locked
cargo run --manifest-path '../my app/Cargo.toml' --offline --locked
```

Cài đặt, clone và khôi phục dependency cần mạng nếu chưa có trong cache.
Generator không truy cập mạng. `cargo fetch` tạo lockfile của ứng dụng;
giữ lockfile và checkout tương thích. Xem trợ giúp CLI đã cài bằng
`re-cordis-create --help`.

## Chạy từ checkout cục bộ

Dùng Rust 1.85 trở lên và checkout tương thích của repository này. Chạy các lệnh sau từ thư mục
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
Ứng dụng được sinh vẫn là `0.0.0` với `publish=false`; dùng dependency path
ngay cả khi CLI được cài từ crates.io.
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

## Phân phối và thông báo giấy phép template

README được sinh giữ nguyên thông báo MIT của mã khởi đầu;
đầu ra vẫn đúng năm tệp. Giữ thông báo đó khi phân phối lại template hoặc phần
đáng kể của nó. Phần bạn bổ sung có thể có thông báo bản quyền riêng.
Cargo.lock không chốt nội dung phụ thuộc path tuyệt đối. Hãy dùng checkout tại
revision đã review; thay đổi checkout có thể đổi lần build ứng dụng tiếp theo.
[Hợp đồng alpha](../../release-readiness.md) đã duyệt giữ công cụ yêu cầu
checkout tường minh này. Cài CLI đã phát hành không kèm checkout framework
hay đổi dependency được sinh sang crates.io. Checkout phát hành tương thích là
`84fa9bdf0eb4e631fce792bf163494b0a61aeba8`.
Kiểm tra version/bố cục không chứng minh tương thích mã nguồn. Chế độ sinh
phụ thuộc registry cần quyết định riêng trong tương lai.

## Kiểm thử archive Cargo

Scaffold test của package Create đã giải nén cần một checkout đầy đủ riêng.
Đặt biến chỉ dành cho test `CORDIS_CREATE_TEST_CHECKOUT` đến checkout đó;
nếu không đặt, test vẫn dùng repository chứa nó như trước. Hành vi CLI giữ nguyên
và vẫn yêu cầu `--cordis-path`. Archive chứa template, không chứa các crate của
framework. Giải nén ngoài workspace nguồn cha để tránh lỗi Cargo dò workspace
không liên quan. [README package](../../../crates/re-cordis-create/README.md)
ghi lệnh chạy; [bằng chứng archive](../../plans/2026-11-02-cordis-completion/evidence/2026-11-05-create-archive.md)
ghi kết quả stable/1.85.0 với checkout tại revision
`03d255029db6a57ccc17c0b1e26facdb491c058b`, byte giải nén giữ nguyên và kiểm tra
ứng dụng năm tệp được tạo bởi CLI build từ archive thật.

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
