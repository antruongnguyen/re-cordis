# Ví dụ có thể chạy

[English](../../en/guides/examples.md) | [Trang tiếng Việt](../README.md)

Chạy các lệnh từ thư mục gốc repository. Ví dụ dùng cùng production core như
ứng dụng; JSON, HTTP, mật mã và feature Tokio mở rộng trong ví dụ là
**dev-dependencies**, không phải API core.

## Lifecycle: tổng quan framework

```sh
cargo run -p re-cordis-core --example lifecycle --locked
```

[Mã nguồn](../../../crates/re-cordis-core/examples/lifecycle.rs).
Khởi tạo store cô lập, inject consumer, cập nhật cấu hình cả hai và assert lần
đọc cuối của consumer trước khi provider chính xác được dọn.
Đây là ví dụ hoàn chỉnh phù hợp nhất để học ngữ nghĩa framework.

## Digest: xử lý tệp ngoại tuyến

```sh
cargo run -p re-cordis-core --example github_digest --locked -- \
  crates/re-cordis-core/tests/fixtures/github_snapshot.json /tmp/cordis-report.md
```

[Mã nguồn](../../../crates/re-cordis-core/examples/github_digest.rs).
Kiểm tra snapshot giới hạn và ghi Markdown xác định, có escape, bằng thay tệp
nguyên tử cùng thư mục. Đích là một service; processor inject giữ nó khi xử lý.
Cập nhật đầu vào/đích là thao tác lập trình qua handle riêng của ví dụ, không
phải API core công khai.

Fixture là dữ liệu tổng hợp. Báo cáo là chỉ mục hoạt động giới hạn, không phải
lịch sử đầy đủ hay bản diff giữa hai ngày.

## Collector: truy cập chỉ đọc có xác thực, tùy chọn

Collector gọi `gh` đã cài bằng xác thực hiện có. Không chạy nó tự động khi
kiểm tra tài liệu vì lệnh có yêu cầu mạng được xác thực.
Khi đã được cho phép với repository chọn, lệnh là:

```sh
cargo run -p re-cordis-core --example github_collect --locked -- \
  OWNER/REPO /tmp/cordis-snapshot.json
```

[Mã nguồn](../../../crates/re-cordis-core/examples/github_collect.rs).
Hai truy vấn giới hạn công bố một snapshot mới nhất hoàn chỉnh.
Thêm khoảng thời gian để chạy định kỳ; shutdown chờ poll đã tiếp nhận.
Không tích hợp đăng nhập, thay đổi GitHub hay gửi thông báo.

## Webhook: chỉ replay cục bộ có chữ ký

[Mã nguồn](../../../crates/re-cordis-core/examples/github_webhook.rs).
Receiver chỉ loopback sở hữu các task HTTP và writer journal tuần tự.
Xem [hợp đồng consumer và hướng dẫn replay](../../github-activity.md) để biết
cách đặt secret, giới hạn và lệnh chính xác. Không đặt secret thật vào mã nguồn,
ví dụ tài liệu hay lịch sử shell.

HTTP 204 đến sau append/flush, không phải fsync. Delivery trùng vẫn được giữ;
disconnect/deadline có thể xảy ra trước một lần append muộn hơn.
Public ingress và cấu hình webhook GitHub thật nằm ngoài phạm vi ví dụ.

## Ranh giới bằng chứng

Kiểm thử tích hợp ngoại tuyến chạy đúng các triển khai này. Riêng ngày
2026-10-27, hai pilot collector/digest one-shot được cho phép đã quan sát
snapshot rỗng từ `antruongnguyen/cordis-rust` (tên cũ của dự án, nay đã đổi
thành `antruongnguyen/re-cordis`) và snapshot có dữ liệu từ
`cordiverse/cordis` (52 issue, 100 PR). Đã chạm giới hạn PR.

Đó là bằng chứng ứng dụng tại một thời điểm, không phải fixture được commit,
toàn bộ inventory repository, bằng chứng parity core mới hay delivery webhook
thật. Tài liệu không phụ thuộc artifact riêng của phiên làm việc.

## Đầu ra console

[console.rs](../../../crates/re-cordis-logger/examples/console.rs) minh họa stdout
thuộc plugin, định dạng rõ ràng, disposal chủ sở hữu, gỡ thủ công và drain root.
Chạy `cargo run -p re-cordis-logger --example console --locked`.
Xem [ghi log ra console](console-logging.md) để đọc cùng chương trình hoàn chỉnh.

## Khởi động và cấu hình lại loader

Chạy `cargo run -p re-cordis-loader --example boot_reconfigure --locked`.
[Ví dụ](../../../crates/re-cordis-loader/examples/boot_reconfigure.rs) khởi động
group cấu trúc, chờ Pending, nhận provider, cập nhật nguồn, thất bại/phục hồi
tường minh và join shutdown. Không dùng file hay mạng.

## Include: đọc, sửa và lưu cấu hình

Chạy `cargo run -p re-cordis-include --example config_file --locked --offline`.
[Ví dụ](../../../crates/re-cordis-include/examples/config_file.rs) đọc tệp tạm,
sửa cấu hình plugin có kiểu, flush và chờ shutdown.
Xem [tệp cấu hình](include.md) để đọc cùng chương trình thực thi.

## HMR: cấu hình và thay factory đã biên dịch

Chạy `cargo run -p re-cordis-hmr --example reload --features include --locked --offline`.
[Ví dụ](../../../crates/re-cordis-hmr/examples/reload.rs) theo dõi cấu hình tạm qua
Include, thay factory đã biên dịch trên cùng EntryId, xác nhận refresh cấu hình
sau đó, báo RestartRequired cho mã nguồn rồi chờ shutdown.
Xem [HMR](hmr.md#ví-dụ-offline) về các lớp hỗ trợ và kênh lỗi.
