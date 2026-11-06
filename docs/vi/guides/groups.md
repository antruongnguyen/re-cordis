# Group cấu trúc

[English](../../en/guides/groups.md) | [Trang chủ tiếng Việt](../README.md)

Import `Group` từ `re_cordis_loader`. Package group của upstream chỉ tái xuất
hành vi loader; Rust giữ một implementation, không tạo crate facade riêng.
`EntrySpec::group(Group::new(children))` khai báo nguồn con lồng nhau có thứ tự.

Group vẫn là node cấu trúc khi verdict disabled ngăn plugin hậu duệ load.
Nó có thể Active khi con bị vô hiệu hóa; nếu chính group thiếu dependency
hoặc callback lỗi thì vẫn có thể Pending hoặc Failed.
Trạng thái disabled của ancestor áp dụng cho mọi hậu duệ. Cập nhật group
reconcile từng nguồn con độc lập và chờ công việc được thêm; con Pending vẫn
là kết quả bình thường.

EntryId ổn định qua các lần chuyển. `move_entry` thay parent logic đúng một
lần. Đổi thứ tự trong cùng group giữ Fiber identity; chuyển group hoặc thay
scope join generation cũ rồi rebuild Fiber core. Group cũ chỉ gỡ những con
vẫn thuộc mình, không được dispose entry đã chuyển sang group khác.
`address` tổng hợp đổi theo đường group. Đây là tiện ích điều hướng của Rust:
':' upstream xác định cây nguồn lồng nhau; Rust dùng ID duy nhất trên các
source document trong cùng loader.

Dùng các layer `Scope` có kiểu để cô lập mới/dùng chung. Provider chính xác
cũ được giữ đến hết cleanup consumer theo hợp đồng core; chuyển không sửa
Context đã commit của consumer. `loader.dispose()?.await?` join generation
hiện tại và đã retire. Xem [API group](../api/groups.md) và [loader](loader.md).

```rust
use re_cordis_core::{BoxError, Plugin, Runtime};
use re_cordis_loader::{Catalog, Config, EntrySpec, Group, Loader};
use std::convert::Infallible;

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let mut catalog = Catalog::new();
            catalog.register("worker", Plugin::new("worker", |_, (): ()| async {
                Ok::<_, Infallible>(())
            }))?;
            let loader = Loader::new(&root, catalog)?;
            let first = loader.create(EntrySpec::group(Group::new(vec![
                EntrySpec::new("worker", Config::new(())).id("worker"),
            ])).id("first").disabled(true))?;
            let second = loader.create(EntrySpec::group(Group::new(vec![])).id("second"))?;
            loader.settle()?.await?;
            let worker = loader.resolve("first:worker")?;
            loader.move_entry(&worker, Some(&second), 0)?.await?;
            loader.remove(&first)?.await?;
            assert_eq!(loader.resolve("second:worker")?, worker);
            loader.dispose()?.await?;
            Ok::<_, BoxError>(())
        }).await??;
        Ok::<_, BoxError>(())
    })
}
```
