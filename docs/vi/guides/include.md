# Tệp cấu hình

[English](../../en/guides/include.md) | [Trang tiếng Việt](../README.md)

Vệ tinh `re-cordis-include` chưa phát hành đọc JSON và YAML có giới hạn vào
loader có kiểu đã được nghiệm thu. Đăng ký plugin đã liên kết cùng bộ giải mã
JSON trong `Catalog`, rồi tạo `Include::new` bên trong `Runtime::run`.
`IncludeOptions::new(path)` chọn tệp gốc. Constructor tiếp nhận quyền sở hữu;
`refresh()?.await?` đọc nguồn và chờ áp dụng xong.

Nguồn có thể là mảng hoặc object chứa `entries` và `patches`.
Entry thông thường theo [schema loader](../api/loader.md). Ví dụ:

```json
[
  {"id": "app", "name": "application", "config": {"port": 3000}},
  {"id": "extra", "name": "include", "config": {"path": "extra.yaml"}}
]
```

Đường dẫn lồng nhau được tính từ tệp chứa chúng. Đường dẫn bản địa đã chuẩn hóa
xác định danh tính tệp, bao gồm đích symlink; mount trùng và chu trình bị từ chối.
Include lồng nhau hiện thành group cấu trúc trong snapshot của loader.
Entry ẩn danh nhận ID ổn định trong bộ nhớ mà không buộc ghi tệp.
Dùng ID tường minh khi cần giữ danh tính qua việc sắp xếp lại cấu trúc nguồn.

Chỉ dùng `IncludeOptions::initial(value)` khi tệp gốc chưa tồn tại. Nó không
thay thế tệp hiện có nhưng không đọc được hoặc sai cú pháp. Lỗi đọc/parse giữ
nguyên trạng thái được chấp nhận gần nhất. Sau khi sửa tệp, gọi `refresh` lại.
Entry riêng lẻ không hợp lệ theo chính sách nguồn độc lập của loader:
các entry hợp lệ vẫn chạy, report chỉ rõ entry bị từ chối.
Kiểm tra `Report::failures`; waiter hoàn tất không có nghĩa mọi plugin đều
Active. Nội dung nguồn giống hệt sẽ không kích hoạt reconcile lặp lại.

JSON dùng cú pháp nghiêm ngặt và từ chối khóa trùng. YAML nhận giá trị tương
thích JSON, số hữu hạn và khóa chuỗi. Anchor, alias, tag tường minh, số không
hữu hạn và nhiều document đều bị từ chối. Ghi chuẩn hóa tạo pretty JSON, cũng
hợp lệ với YAML 1.2; comment, cách đặt dấu nháy và style không được giữ lại.
Đây là cấu hình theo giá trị, không thực thi JavaScript.

`entries` trả snapshot bất biến của loader; `data` trả JSON hiệu lực.
`Location` chỉ rõ tệp sở hữu thực tế, đường dẫn có cấu trúc và vị trí bắt đầu
từ 1 khi có dữ liệu. Vị trí node JSON/YAML đếm Unicode scalar; lỗi cú pháp
JSON nghiêm ngặt giữ cột theo byte của serde_json. Cách đếm này khác UTF-16
của upstream. Đầu vào runtime không được gán vị trí tệp giả.

I/O tệp chạy đồng bộ và có thể chặn khi lưu trữ chậm. Giữ runtime gốc được
chạy cho đến khi waiter vòng đời hoàn tất. Chờ `dispose` tường minh;
kiểm tra kết quả, `final_error` và chẩn đoán dọn dẹp của core. Bỏ handle
không hoàn tất việc dọn dẹp bất đồng bộ.

Xem [API include](../api/include.md), [quyết định](../../decision.md#include-files-and-persistence)
và [giới hạn](../../limitation.md#include-files-and-persistence).

## Patch có thứ tự và quyền sở hữu

Nguồn dạng object có thể thêm mảng `patches` theo thứ tự:

```json
{
  "entries": [{"id": "app", "name": "application", "config": {"port": 3000}}],
  "patches": [
    {"id": "app", "name": "application", "config": {"port": 4000}},
    {"insert": [{"id": "extra", "name": "application"}]},
    {"id": "extra", "disabled": true}
  ]
}
```

`name` của patch là điều kiện khớp, không đổi tên đối tượng đích.
Nếu không có `insert` thì phải có `id`; nếu có `insert`, bỏ `id` nghĩa là
thêm vào cuối gốc nguồn, còn `id` chọn một group cấu trúc.
Đích không tồn tại, tên không khớp hoặc chèn vào entry không phải group đều được
báo lỗi và bỏ qua. Patch sau nhìn thấy entry do patch trước chèn.
Thay config của group sẽ bỏ quyền sở hữu của các con cũ trước khi đăng ký
entry thay thế.

`ownership(id)` tách owner của entry khỏi owner của từng khóa bị ghi đè.
Entry có thể nằm trong tệp con nhưng config thuộc patch trong tệp cha.
Bỏ patch sẽ khôi phục giá trị gốc khi refresh.
Danh tính ẩn danh được ghép theo tệp chuẩn hóa/vị trí thô; cache hiện hành được
dọn và có giới hạn. ID sinh ra tránh ID tường minh trong cả nguồn lẫn patch.
