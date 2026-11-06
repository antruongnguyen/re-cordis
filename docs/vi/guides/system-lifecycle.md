# Vòng đời ứng dụng tích hợp

[English](../../en/guides/system-lifecycle.md) | [Trang tiếng Việt](../README.md)

Ứng dụng tham chiếu R0 ghép core cục bộ, console logger, provider có kiểu và
consumer inject, timer theo chủ sở hữu, group cấu trúc của loader, cấu hình/
persistence Include và theo dõi HMR tùy chọn. Ứng dụng dùng API satellite hiện
có. Mô-đun ứng dụng là mã riêng của ví dụ, không phải crate, facade, API công
khai hay template Create phân phối mới.

## Chạy ứng dụng và kiểm thử

Từ checkout:

```sh
cargo run -p re-cordis-hmr --example system_lifecycle --features include --locked --offline
cargo run -p re-cordis-hmr --example system_lifecycle --features include --locked --offline -- --watch
cargo test -p re-cordis-hmr --test system_lifecycle --features include --locked --offline
```

[Điểm vào](../../../crates/re-cordis-hmr/examples/system_lifecycle.rs),
[ứng dụng dùng chung](../../../crates/re-cordis-hmr/examples/system_lifecycle/app.rs)
và [kiểm thử](../../../crates/re-cordis-hmr/tests/system_lifecycle.rs) là tham chiếu
thực thi. Bộ thực thi Tokio current-thread chạy Runtime::run cho đến khi cleanup
đã join hoàn tất. Tệp nằm trong fixture tạm ngoại tuyến; không cần thông tin
đăng nhập, mạng, daemon biên dịch hay process supervisor.

Bộ kiểm thử tích hợp dùng thời gian tạm dừng và gate cleanup tường minh để
assert quan hệ nhân quả. Kiểm thử lặp chạy ít nhất 100 chu kỳ vòng đời.
Polling tệp thật là kiểm tra bổ sung có giới hạn, không điều khiển lịch của
các test xác định.

## Theo dõi cấu hình lại qua cleanup

Include sửa cấu hình tuần tự hóa bằng JsonEdit và chuyển entry bằng move_entry.
Provider chuyển từ group cấu trúc cô lập này sang group khác. Consumer inject
cũ giữ cleanup tại gate và đọc đúng provider đã commit trong khi provider đó
đang bị thu hồi. Sibling không liên quan vẫn có thể cập nhật và timer của nó
vẫn tiến triển trước khi mở gate.

Sau khi giải phóng, provider được dựng lại trong phạm vi đích. Consumer cũ ở
Pending cho đến khi được chuyển sang phạm vi đích; khi đó nó bind provider mới.
EntryId ổn định của loader và identity Fiber/publication mới có vai trò khác
nhau. Chỉ giữ Rc không giữ tài nguyên effect của provider; injection đã commit
mới bảo đảm retention đó.

Xem [group](groups.md), [dịch vụ và phạm vi](services-and-scoping.md) và
[Include](include.md) để biết các hợp đồng nền tảng.

## Kiểm tra từng kênh lỗi

| Quan sát | Ý nghĩa |
|---|---|
| Result của thao tác | Lỗi tiếp nhận, decode, cấu hình lại hoặc thao tác đã join |
| Report failures của Include/thay factory | Entry lỗi độc lập; sibling khỏe vẫn có thể thành công |
| Lịch sử WatchEvent::Failure | Lỗi refresh trả về và Report failures của công việc được theo dõi |
| Include::final_error | Lỗi persistence cuối được giữ lại, kể cả khi thả waiter |
| CleanupReport và Runtime::take_failures | Kết quả cleanup và nguyên nhân callback/cleanup gốc trong host |

Waiter thành công không chứng minh cleanup không có lỗi. Ứng dụng và kiểm thử
đọc các kênh này tại barrier được quy định. Một nguyên nhân được quan sát qua
nhiều kênh vẫn chỉ là một lần lỗi. Update tường minh hoặc candidate đã liên kết
mới phục hồi Failed; provider xuất hiện không tự phục hồi trạng thái đó.

ReloadPlan cấp candidate đã biên dịch cho cùng loader do Include quản lý.
Nó không biên dịch Rust hay thay mã nhị phân native. Xem [HMR](hmr.md) và
[tham chiếu lỗi](../api/errors-hooks.md).

## Shutdown công việc đã tiếp nhận

Application::shutdown theo thứ tự trước hết stop và join watcher khi Include
còn có thể hoàn tất refresh đã tiếp nhận. Sau đó nó dispose/join Include và
chủ sở hữu ứng dụng, gồm timer và writer; lỗi ở từng bước vẫn được quan sát.
Chỉ yêu cầu stop hay thả token không join refresh. Các test kiểm tra riêng việc
hủy ancestor cưỡng bức và đọc nguyên nhân hủy được giữ lại.
Dispose Include flush journal đang chờ trước cleanup con; cần kiểm tra lỗi
persistence và byte thực tế trong tệp. Thả handle còn giữ trước khi assert
capture/weak reference đã giải phóng.

Write đồng bộ và thao tác filesystem thông thường có thể chặn bộ thực thi cục
bộ. Flush/thay tệp không phải fsync, transaction nhiều tệp hay bảo vệ trước race
filesystem thù địch. OS read bị chặn có thể trì hoãn watcher join.
Bằng chứng chạy filesystem/watcher chỉ bao phủ macOS.

## Kiểm tra ứng dụng được sinh thật

```sh
python3 -B tests/docs/test_check.py
python3 -B tests/docs/check.py --toolchain stable
python3 -B tests/docs/check.py --toolchain 1.85.0
```

Checker gọi CLI Create đã giao với checkout tường minh, kiểm tra rồi build/
test/run ứng dụng cơ bản không đổi, sau đó chỉ mở rộng dự án ngoài workspace
tạm đó bằng dependency timer, Include và HMR có đường dẫn cục bộ tường minh.
Nó sao chép đúng điểm vào, ứng dụng dùng chung và test R0 vào dự án rồi build/
test/run bằng toolchain rustup đã chọn. Tham số process là mảng; đường dẫn có
khoảng trắng và ký tự tiếng Việt được thực thi. Cargo resolve ngoại tuyến và
cần dependency có sẵn trong cache.

Template cơ bản production giữ nguyên. Không có biến thể template phân phối
mới. [Create](create.md) hướng dẫn sinh dự án thông thường;
[bằng chứng R0](../../plans/2026-11-02-cordis-completion/evidence/r0-acceptance.md)
ghi ma trận đã chạy, kết quả review và phân loại bằng chứng nguồn. Trace đối chiếu core và loader hiện có giữ
phạm vi bằng chứng riêng; ứng dụng này không phải bộ parity TypeScript mới.
