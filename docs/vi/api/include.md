# API include

[English](../../en/api/include.md) | [Trang tiếng Việt](../README.md)

Sinh chữ ký chính xác bằng `cargo doc --workspace --no-deps --locked`;
mở `target/doc/re_cordis_include/index.html`. API cục bộ, chưa ổn định
và chưa phát hành. [Hướng dẫn tệp](../guides/include.md).

| Export | Hợp đồng |
|---|---|
| `Include` | Owner riêng và loader nội bộ; thao tác nguồn tường minh, snapshot bất biến và disposal có chờ hoàn tất. |
| `IncludeOptions` | Đường dẫn gốc bản địa và JSON ban đầu tùy chọn chỉ dùng khi thiếu tệp. |
| `IncludeTask` | Waiter cho công việc loader đã tiếp nhận; giữ host gốc đến lúc hoàn tất. |
| `Report` | Cờ nguồn thay đổi, lỗi độc lập và xung đột tệp thắng; kiểm tra cả hai danh sách sau await. |
| `JsonEdit` | Đặt/bỏ trường JSON cấp entry; builder phân biệt null tường minh với bỏ trường. |
| `Conflict` | ID entry, khóa xung đột tùy chọn và lý do; xung đột cấu trúc không có khóa. |
| `IncludeError` | Thông báo, vị trí nguồn tùy chọn dùng chung, nguyên nhân gốc và xung đột được giữ khi ghi thất bại. |
| `Location` | Đường dẫn tệp đã chuẩn hóa, đường dẫn thô có cấu trúc và SourceSpan tùy chọn. |
| `Owner`, `OwnerKind`, `Ownership` | Nơi sở hữu entry trong file/patch/embedded và ghi đè từng khóa; khóa không có owner riêng kế thừa owner của entry. |
| `ParsedDocument`, `parse_document` | Parse JSON nghiêm ngặt hoặc YAML có giới hạn kèm vị trí node thật, chọn theo đuôi .json/.yaml/.yml. |

`new` tiếp nhận quyền sở hữu trước I/O. `refresh` đọc và bắt đầu reconcile
nguồn; `entries`, `data` và `ownership` cho quan sát bất biến.
`edit/create/remove/move_entry` tiếp nhận thay đổi runtime có thể lưu;
`pending` đếm bản ghi journal và `settle` chờ công việc loader đã tiếp nhận.
`flush` chạy I/O đồng bộ rồi trả waiter cho công việc loader sau hòa giải.
`dispose` dừng tiếp nhận và chờ teardown. `final_error` giữ lỗi ghi cuối dù waiter bị bỏ.
Kiểm tra report/chẩn đoán dọn dẹp của core bên cạnh kết quả include.

| Giới hạn được export | Giá trị |
|---|---:|
| `MAX_BYTES` — byte nguồn/JSON hiệu lực/journal | 1048576 |
| `MAX_DEPTH` — độ sâu giá trị/include/group | 32 |
| `MAX_NODES` — số node giá trị đã parse | 65536 |
| `MAX_STRING_BYTES` — chuỗi và khóa | 65536 |
| `MAX_FILES` — nguồn chuẩn hóa | 64 |
| `MAX_ENTRIES` — entry hiệu lực/bản ghi chờ | 1024 |
| `MAX_PATCHES` — patch mỗi tệp | 1024 |
| `MAX_RETRIES` — số lần thử thay thế | 3 |

Cột của node đếm Unicode scalar; cột lỗi cú pháp JSON đếm byte.
Đầu vào runtime không có span tệp. Đường dẫn bản địa theo hệ điều hành;
cú pháp Windows không phải đường dẫn tuyệt đối trên Unix. Danh tính nguồn
chuẩn hóa đi theo symlink.

YAML nhận giá trị tương thích JSON và từ chối alias, anchor, tag, khóa trùng
cùng nhiều document. Đầu ra chuẩn hóa là pretty JSON; không giữ comment hoặc
style. Giới hạn tệp không hứa thời gian hoàn tất hay độ bền khi mất điện.
Xem [giới hạn](../../limitation.md#include-files-and-persistence).
