# API Loader

[English](../../en/api/loader.md) | [Trang tiếng Việt](../README.md)

Sinh chữ ký chính xác bằng `cargo doc --workspace --no-deps --locked`;
mở `target/doc/re_cordis_loader/index.html`. Mọi export cục bộ/non-Send,
chưa ổn định và chưa phát hành. [Hướng dẫn chạy được](../guides/loader.md).

## Catalog, entry và snapshot

| Export | Hợp đồng |
|---|---|
| `Catalog` | Đăng ký Plugin có kiểu theo tên; trùng tên bị từ chối. Decoder JSON và tên dịch vụ có kiểu là tùy chọn. |
| `Config` | Alias của core hooks::Value; nguồn có kiểu chia sẻ bất biến, kiểm tra qua get. Nguồn Rust không cần trait serde. |
| `Loader` | Sở hữu cây entry và catalog dưới một owner riêng của core. |
| `EntryId` | Identity logic ổn định trong loader; sinh entry-N hoặc dùng ID tường minh. |
| `FactoryId` | Identity cấp phát trong catalog, khác EntryId và FiberId của core. |
| `Scope` | Lớp isolation/intercept có kiểu bất biến; isolate mới, isolate_with tường minh, intercept tĩnh và intercept_with được đánh giá. |
| `EntrySpec`, `EntryKind` | Toàn bộ tùy chọn plugin/group; builder injecting, scope, intercept, condition, map_config có kiểu. |
| `EntrySnapshot` | Nguồn/options bất biến, config đã đánh giá, transport, state, cha, con có thứ tự, address và Fiber ID hiện tại. |
| `EntryState` | Pending, Loading, Active, Failed, Unloading, Disabled hoặc Disposed. Thiếu dịch vụ là Pending. |
| `LoaderTask` | Waiter cho sửa đổi đã bắt đầu; thả nó không hủy thao tác. |
| `Settlement`, `EntryFailure` | Snapshot entry và lỗi từng entry; settlement không bảo đảm mọi entry Active. |
| `LoaderError` | Lỗi catalog/identity/config/admission/callback/lifecycle hoặc lỗi có vị trí, giữ nguyên nguyên nhân bên dưới. |
| `CommitRecord`, `Origin` | Snapshot cũ/mới, entry chính xác, nhóm cũ/đích và nguồn Runtime/Source. Đọc nguồn không phát sự kiện ghi ngược. |

`create`/`create_in` tiếp nhận entry đã kiểm tra. `update` thay toàn bộ options;
`remove` gỡ liên kết và bắt đầu teardown. `snapshot`/`entries` chỉ đọc, không
lộ quyền điều khiển Fiber. `resolve` tra địa chỉ cấu trúc hiện tại.
`move_entry` giữ EntryId và đổi địa chỉ; xem [group](groups.md).
`settle` chờ các thế hệ đã tiếp nhận và mọi direct update còn chờ; update mới
hoàn tất trước không giải phóng barrier của update cũ. `remove` chờ update của
cây con bị gỡ; `dispose` và shutdown owner chờ toàn bộ update và cleanup đã tách.
Thả waiter thao tác hoặc settlement không hủy công việc hay xác nhận kết quả.
Loader giữ lỗi direct update chưa được quan sát đầu tiên, thay vì lưu lịch sử
lỗi không giới hạn. Waiter update gốc/remove hoàn tất và settlement giao kết quả
thành công sẽ xác nhận kết quả; dispose có thể báo lại cùng lỗi được giữ.
Sửa đổi phải trong đúng Runtime::run; giữ runtime sống đến khi chờ xong.
Loader giữ lỗi lifecycle-task đã tách đầu tiên và vẫn chờ toàn bộ cleanup.
Lỗi callback cleanup thông thường đi qua chẩn đoán của core: kiểm tra
Runtime::take_failures cùng kết quả thao tác/CleanupReport; Disposed không
đồng nghĩa cleanup không lỗi.

Mỗi factory giữ identity Plugin gốc. Decode cấu hình tách khỏi validation của
core; entry Pending không validate plugin sớm. Update đang chạy bị từ chối
giữ nguồn đã chấp nhận. Entry Failed cần update tường minh để phục hồi.
Condition dùng Context injection của entry; map_config dùng đúng Context
injection của plugin bên trong trước validation.

## Sửa từng phần và nguồn cấu hình

| Export | Hợp đồng |
|---|---|
| `EntryPatch` | Sửa từng phần factory/config/disabled/inject/group. |
| `FieldPatch` | Keep giữ nguyên, Set thay thế, Clear về mặc định; không được xóa factory, config về unit Rust, disabled về false, injection/con về rỗng. |
| `SourceId`, `SourceDocument` | Nguồn typed/JSON chỉ đọc do bên gọi quản lý; reconciliation sở hữu entry của nguồn và xử lý độc lập sibling hợp lệ. |
| `SourceLocation`, `SourceSpan` | Nhãn nguồn tùy chọn, đường dẫn PathSegment, dòng/cột thực bắt đầu từ 1 do locator cung cấp nếu có. |

Trường JSON entry gồm `id`, `name`, `config`, `group`, `disabled`, `inject`,
`isolate`, `intercept`. `group: true` dùng config là mảng con.
Trường không biết bị từ chối. Đăng ký hợp đồng dịch vụ cho tên
inject/isolate/intercept; tên không biết bị từ chối. Realm có tên chia sẻ
isolation ID cục bộ trong loader khi các định nghĩa Scope còn sống. Sau khi
Scope cuối cùng được thả, dùng lại nhãn có thể tạo ID mới; Context đã commit
vẫn giữ góc nhìn dịch vụ cũ. Thiếu config khác JSON null ở biên decoder.
Config do decoder quản lý có thể chứa dữ liệu bất kỳ; không trường nào chạy JavaScript.

| Giới hạn | Giá trị |
|---|---:|
| `MAX_ENTRIES` (entry hiện hữu cộng đang teardown) | 1024 |
| Realm có tên đồng thời sống (cùng giới hạn) | 1024 |
| Direct update còn chờ, toàn loader (cùng giới hạn) | 1024 |
| `MAX_JSON_BYTES` | 1048576 |
| `MAX_JSON_DEPTH` | 32 |
| `MAX_JSON_NODES` | 65536 |
| `MAX_STRING_BYTES` (kể cả khóa object) | 65536 |
| Entry ID tường minh | 1–128 byte UTF-8, không có dấu hai chấm |
| Tên factory | 1–256 byte UTF-8 |
| Độ lồng group Rust | 32 |
| Nhãn isolation | 1–128 byte UTF-8 |

Kích thước payload Config Rust do ứng dụng quản lý. Đây là giới hạn tiếp nhận,
không phải timeout theo thời gian hay cam kết kích thước tuần tự hóa.

`replace_factory` thay declaration có kiểu đã liên kết.
`commits`, `reconcile`, snapshot là điểm tích hợp trong bộ nhớ được adapter
persistence của include sử dụng; theo dõi tệp vẫn chưa triển khai. Loader không
cung cấp filesystem, Node resolution, JS eval hay
nạp lại binary Rust. Xem [giới hạn](../../limitation.md#loader-and-structural-groups).
