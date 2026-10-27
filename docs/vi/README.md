# Tài liệu cordis-rust

[English](../en/README.md) | [Trang tài liệu chung](../README.md) | [Mục lục cho AI agent](../llms.txt)

Cordis là framework dành cho ứng dụng có plugin, dịch vụ, cấu hình và tài nguyên
thay đổi theo thời gian. `cordis-rust` biểu diễn ngữ nghĩa của lõi Cordis
TypeScript bằng API Rust tường minh, có kiểu.

Sản phẩm hiện tại là **`cordis-core` chưa phát hành**, phiên bản `0.0.0`, dùng
Rust 2024 và yêu cầu Rust 1.85 trở lên. API chưa ổn định. Các crate vệ tinh timer,
console logger và loader chưa được triển khai. Đây không phải lớp tương thích
trong suốt với Proxy hay decorator của JavaScript.
[Lộ trình mục tiêu tiếp theo](../todo.md#next-implementation-targets-satellites)
xếp timer, console logger, rồi loader; mỗi phần cần rà soát mã nguồn/API/phụ
thuộc và cập nhật tài liệu tương ứng. Crate dự kiến chưa phải API sử dụng được.

## Chọn lộ trình đọc

| Mục tiêu | Bắt đầu tại |
|---|---|
| Xây dựng ứng dụng đầu tiên | [Bắt đầu sử dụng](getting-started.md), sau đó đọc [khái niệm](concepts.md) |
| Tạo thành phần có thể nạp lại | [Plugin và cập nhật](guides/plugins-and-updates.md) |
| Kết nối các thành phần an toàn | [Dịch vụ và phạm vi](guides/services-and-scoping.md) |
| Sở hữu tài nguyên và dừng ứng dụng | [Effect và dọn dẹp](guides/effects-and-cleanup.md) |
| Phát thông báo và chạy middleware | [Sự kiện](guides/events.md) |
| Xuất thông tin chẩn đoán | [Ghi log](guides/logging.md) |
| Tìm nguyên nhân plugin Pending hoặc Failed | [Kiểm thử và xử lý sự cố](guides/testing-and-troubleshooting.md) |
| Xem ứng dụng hoàn chỉnh | [Ví dụ](guides/examples.md) |
| Tra cứu kiểu hoặc thao tác | [Tài liệu API](api/README.md) |
| Tra cứu thuật ngữ | [Bảng thuật ngữ](glossary.md) |

## Những quy tắc quan trọng

Chạy framework bằng `Runtime::run` từ điểm vào của bộ thực thi Tokio.
Context, handle và future của framework mang tính cục bộ, không phải
`Send`/`Sync`. Giữ runtime tồn tại và tiếp tục được thực thi cho đến khi công
việc cần thiết hoàn tất.

Đăng ký dọn tài nguyên với Context sở hữu tài nguyên đó. Thả handle không phải
là dispose; hủy Runtime là hủy thực thi, không phải dọn dẹp bất đồng bộ.
Kiểm tra báo cáo dọn dẹp bên cạnh lỗi của thao tác.

Trạng thái sẵn sàng thuộc từng Fiber, không phải toàn hệ thống. Một lần chờ có
thể kết thúc ở `Pending` khi thiếu phụ thuộc. Chờ bên cung cấp hoàn tất và chờ
consumer sẵn sàng riêng biệt. Consumer Failed cần `update` tường minh để phục hồi.

## Tài liệu tham khảo và bằng chứng

Bộ hướng dẫn mới mô tả API Rust **hiện tại**. Rustdoc được sinh từ mã nguồn cung
cấp chữ ký và ràng buộc kiểu chính xác; xem [mục lục API](api/README.md).
[Các tài liệu cột mốc](../README.md#engineering-references) giữ lại phạm vi lịch
sử và bằng chứng nghiệm thu. Câu “chưa triển khai” trong một cột mốc cũ không
đồng nghĩa với API hiện tại chưa có tính năng đó.

Phạm vi tương thích là lõi TypeScript tại bản chốt và các biểu diễn Rust đã ghi
nhận. Xem [giới hạn M6](../m6.md#remaining-restrictions-and-safety-deviations)
và [đối chiếu vòng đời đã thực thi](../lifecycle-parity.md), không suy diễn thành
tương thích với mọi kiểm thử hay bản upstream mới nhất.
