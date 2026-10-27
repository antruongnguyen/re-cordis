# Plugin, Fiber và Registry

[English](../../en/api/plugins-registry.md) | [Mục lục API](README.md)

Mã nguồn: [plugin.rs](../../../crates/cordis-core/src/plugin.rs).
Hướng dẫn có mã chạy: [plugin và cập nhật](../guides/plugins-and-updates.md).

## Khai báo và tiếp nhận Plugin

| Thao tác | Hợp đồng |
|---|---|
| `Plugin::<C>::new(name, apply)` | `C: Clone`; kiểm tra đồng nhất; apply nhận `(Context, C)`, trả future của `Result<(), E>` |
| `Plugin::<C, R>::validated(name, validate, apply)` | Đồng bộ `&C -> Result<R, VE>`; apply nhận `R` đã kiểm tra; nguồn không bắt buộc Clone |
| `Plugin::<C>::initialized(name, initialize)` | Nguồn Clone; chờ future tạo một `Disposer`, sở hữu nó qua effect |
| `Plugin::injecting(dependencies)` | Đặt injection bắt buộc cho góc nhìn khai báo này; giữ định danh nhóm chung |
| `Context::plugin(&plugin, source)` | Tiếp nhận, gắn cleanup cha, trả `Result<FiberHandle<C>, LifecycleError>` trước khi load xong |
| `Context::inject(dependencies, apply)` | Consumer không cấu hình thuộc cha; callback nhận Context |
| `Context::inject_method(dependencies, Rc<T>, method)` | Adapter phương thức tường minh nhận `(Rc<T>, Context)` trong child inject |

Callback/future thuộc quyền sở hữu và `'static`; lỗi chuyển được thành
`BoxError`. Clone khai báo chia sẻ định danh; tạo riêng khai báo khác thì không.
Tên injection lặp giữ vị trí đầu, hợp đồng/cấu hình cuối.
Khai báo bắt buộc có thể mang `Dependency::with_config`.

## FiberHandle

| Thao tác | Hợp đồng |
|---|---|
| `id`, `name` | Định danh phiên bản ổn định và tên chẩn đoán đã chốt |
| `state` | Ảnh chụp `FiberState` hiện tại |
| `config` | Nguồn `Rc<C>` giữ lại, không phải `R` đã kiểm tra |
| `context` | Context không sở hữu của Fiber |
| `ready` | Chờ lượt hiện tại, không tạo công việc |
| `restart` | Buộc apply lại cùng nguồn; không phục hồi Failed |
| `update(source)` | Kiểm tra/commit nguồn và hội tụ; cho phép phục hồi lỗi tường minh |
| `update_with(source, options)` | Cùng thao tác, có lựa chọn thông báo commit |
| `dispose` | Đánh dấu kết thúc/gỡ lưu trú ngay, sau đó chờ teardown thuộc sở hữu |

`ready`, `restart`, `update`, `update_with`, `dispose` trả
`Result<LifecycleTask, LifecycleError>`. Task cho kết quả
`Result<FiberState, LifecycleError>`. Thả task/handle không hủy hay dispose.

`UpdateOptions` có trường công khai `no_save: bool`, mặc định false.
True chỉ bỏ thông báo commit nguồn.
`Context::on_update` cục bộ nhận `(Context, Rc<R>, AsyncNext<bool>)`;
không gọi next nghĩa là chấp nhận mà không restart.
`on_update_sync` nhận `Rc<R>`, trả true để tiếp tục hoặc false để chấp nhận
không restart. Cả hai trả đăng ký listener có phạm vi.

## Trạng thái và hội tụ

Các variant `FiberState`: `Pending`, `Loading`, `Active`, `Failed`, `Unloading`,
`Disposed`. Pending là thiếu yêu cầu; Failed là lỗi kiểm tra/apply sau rollback.
Chỉ Disposed là kết thúc vĩnh viễn.

Kiểm tra update khi loaded diễn ra trước thay nguồn và middleware.
Pending hoãn kiểm tra. Một driver gộp update; apply/cleanup cũ xong trước lượt
load mới nhất có thể chạy. Dispose thắng reload đang xếp hàng. Lỗi load lỗi
thời được chẩn đoán mà không làm hỏng mục tiêu mới nhất.

Handle readiness/update có thể kết thúc Pending. Chờ provider hoàn tất rồi
consumer sẵn sàng riêng; không có handle nào là mốc hội tụ toàn cục.
Consumer Failed bỏ qua refresh phụ thuộc; chỉ update cho phép phục hồi.

## Registry

| Thao tác | Hợp đồng |
|---|---|
| `Context::registry` | Góc nhìn Registry không sở hữu |
| `len`, `is_empty` | Đếm/kiểm tra nhóm khai báo đang lưu trú |
| `contains(&plugin)` | Kiểm tra lưu trú của khai báo này |
| `instances(&plugin)` | Ảnh chụp `Vec<FiberId>` của nhóm hiện tại |
| `remove(&plugin)` | Loại nhóm hiện tại, bắt đầu dispose mọi thành viên; trả task cần chờ |

Phiên bản Failed vẫn lưu trú đến dispose. Spawn mới sau remove tạo nhóm mới;
cleanup cũ không được xóa nhóm mới. Cleanup cha sở hữu con độc lập với vòng đời
handle công khai.

## Lỗi

`LifecycleError` phân biệt `Admission`, `Callback(CleanupFailure)`, lỗi hook,
`HostStopped`, `DriverPanicked`. Có thể truy nguyên lỗi callback gốc.
Lỗi cleanup được chẩn đoán/giữ lại mà không kẹt cleanup độc lập;
kết quả Disposed không chứng minh mọi disposer đều thành công.

Chờ vòng đời self/tổ tiên sở hữu mình trong setup/cleanup đã quy thuộc bị từ chối.
Xem [lỗi và hook](errors-hooks.md), [dọn tài nguyên](effects.md).
