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


`unset` của patch bỏ trường hiệu lực sau các giá trị ghi đè. Ví dụ,
`{"id":"app","unset":["config","disabled"]}` che các trường đó ngay cả khi
lớp bên dưới cung cấp giá trị. `config: null` vẫn là null tường minh;
nó khác `unset: ["config"]`. Không thể unset khóa điều khiển `id`, `name`,
`insert` hoặc `unset`. `JsonEdit::remove` ghi sự vắng mặt này tại owner của
patch; `JsonEdit::set` về sau xóa marker tương ứng.

## Thay đổi runtime và flush

Dùng `edit(id, JsonEdit::new().set(key, value))` để thay một trường cấp entry
hoàn chỉnh. `JsonEdit::remove(key)` bỏ trường; đặt config thành JSON null
giữ giá trị null tường minh. `create(value, parent, position)`,
`remove(id)` và `move_entry(id, parent, position)` sửa cấu trúc.
Parent là ID entry tùy chọn; None chọn gốc. Các thao tác trả waiter,
còn `settle` chờ công việc loader đã tiếp nhận.

Thay đổi vào journal hữu hạn trong bộ nhớ trước khi ghi tệp. Chỉnh sửa lặp
được gộp theo entry; tạo rồi xóa không để lại yêu cầu ghi.
`pending` trả số bản ghi entry đang chờ. Vượt giới hạn bị từ chối trước
khi đổi runtime. Adapter không lộ loader có thể sửa trực tiếp và không
tự serialize Config Rust bất kỳ.

Chờ `flush()?.await?` để đọc lại, hòa giải, ghi và chờ áp dụng trạng thái đã
hòa giải. `refresh` dùng cùng chính sách. Tệp thắng khi hai bên đổi cùng
khóa cấp entry thành các giá trị khác nhau; `config` là một khóa nguyên khối,
không merge đệ quy các trường cấu hình ứng dụng. Thay đổi giống nhau không
xung đột. Xóa trong tệp thắng chỉnh sửa runtime; sửa entry trong tệp xung đột
với xóa ở runtime. Đổi parent không tương thích được báo là xung đột cấu trúc.
Kiểm tra `Report::conflicts`.

Ghi ngược theo owner của từng entry/khóa, gồm tệp lồng nhau, giá trị ghi đè của
patch và entry nằm trong phần insert hoặc config group thay thế.
Reconcile nguồn không tự tạo journal ghi ngược.
Không thể dùng sửa runtime tổng quát để thay group/config được sinh của
include descriptor; hãy đổi đường dẫn trong nguồn rồi refresh.

Khi ghi thất bại, thao tác trả lỗi và giữ thay đổi runtime đang chờ trong bộ
nhớ. Sửa nguyên nhân bên ngoài rồi thử refresh/flush lại, hoặc chờ lần thử cuối
khi dispose. Không thay thế tệp hiện tại không parse được để ép lưu thành công.

Mỗi tệp được ghi qua tệp tạm cùng thư mục với số lần thử hữu hạn.
Mỗi lần thử đọc lại và hòa giải nội dung hiện tại. Thay thế diễn ra từng tệp;
không có transaction nhiều tệp, bảo đảm fsync khi mất điện hay giao thức đầy đủ
chống người ghi đồng thời. Người ghi không phối hợp vẫn có thể chen vào giữa
bước kiểm tra nội dung cuối và rename.

Flush nhiều tệp thành công một phần vẫn giữ snapshot đầy đủ có giới hạn để
lần thử lại khôi phục được cả các trường không đổi của entry đã chuyển, bất kể
thứ tự ghi. Tệp gốc còn thiếu được tạo độc quyền bằng hard link tới tệp tạm
đã ghi đủ; filesystem phải hỗ trợ hard link. Thay tệp hiện có giữ permission
bits. Chuyển cây con chứa include sẽ đổi descriptor sang đường dẫn tuyệt đối
đã chuẩn hóa. Xung đột vẫn có trong lỗi ghi và report kế tiếp.

## Theo dõi tệp và thay factory tùy chọn

Bật feature `include` của `re-cordis-hmr` để đăng ký tệp kích hoạt tường minh
bằng `watch_include` và thay factory đã biên dịch qua `replace_include`
trên cùng Include. Refresh gồm đồ thị nguồn của nó; watcher không tự tìm tệp.
Xem [HMR](hmr.md#tích-hợp-include-tùy-chọn) về completion được giữ,
kênh lỗi và ví dụ reload offline.

## Ví dụ offline

Chương trình hoàn chỉnh này đọc tệp thật, áp dụng thay đổi runtime có kiểu, flush rồi chờ shutdown. Nó tự tạo và dọn thư mục tạm; không cần mạng hay cấu hình bên ngoài.

```sh
cargo run -p re-cordis-include --example config_file --locked --offline
```

```rust
//! Offline file read, JSON edit, flush and joined shutdown.
use re_cordis_core::{BoxError, Plugin, Runtime};
use re_cordis_include::{Include, IncludeOptions, JsonEdit};
use re_cordis_loader::Catalog;
use serde_json::json;
use std::{
    convert::Infallible,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() -> Result<(), BoxError> {
    let directory = std::env::temp_dir().join(format!(
        "re-cordis-include-example-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    ));
    std::fs::create_dir(&directory)?;
    let result = (|| -> Result<(), BoxError> {
        let path = directory.join("cordis.json");
        std::fs::write(
            &path,
            serde_json::to_vec_pretty(&json!([
                {"id":"worker","name":"worker","config":1}
            ]))?,
        )?;
        let executor = tokio::runtime::Builder::new_current_thread().build()?;
        executor.block_on(async {
            let runtime = Runtime::new();
            runtime
                .run(async {
                    let root = runtime.context();
                    let mut catalog = Catalog::new();
                    catalog.register_json(
                        "worker",
                        Plugin::new("worker", |_, value: u32| async move {
                            println!("worker loaded with config {value}");
                            Ok::<_, Infallible>(())
                        }),
                        |value| serde_json::from_value::<u32>(value.cloned().unwrap_or(json!(0))),
                    )?;
                    let include = Include::new(&root, catalog, IncludeOptions::new(path.clone()))?;
                    let read = include.refresh()?.await?;
                    assert!(read.failures.is_empty());
                    let changed = include
                        .edit("worker", JsonEdit::new().set("config", json!(2)))?
                        .await?;
                    assert!(changed.failures.is_empty() && changed.conflicts.is_empty());
                    let flush = include.flush()?.await?;
                    assert!(flush.failures.is_empty() && flush.conflicts.is_empty());
                    let written: serde_json::Value =
                        serde_json::from_slice(&std::fs::read(&path)?)?;
                    assert_eq!(written[0]["config"], 2);
                    include.dispose()?.await?;
                    assert!(root.registry().is_empty()?);
                    assert!(root.drain()?.await?.failures().is_empty());
                    assert!(runtime.take_failures().is_empty());
                    println!("config 2 persisted; joined shutdown restored the Registry");
                    Ok::<_, BoxError>(())
                })
                .await??;
            Ok::<_, BoxError>(())
        })
    })();
    let cleanup = std::fs::remove_dir_all(&directory);
    result?;
    cleanup?;
    Ok(())
}
```
