# API group

[English](../../en/api/groups.md) | [Mục lục API](README.md)

`re_cordis_loader::Group` thực hiện hợp đồng cấu trúc của satellite group
upstream. `Group::new(Vec<EntrySpec>)` nhận `entries` có thứ tự;
`EntrySpec::group` tạo `EntryKind::Group`. Không có registry vòng đời thứ
hai hay package facade group.

Dùng `Loader::create_in/move_entry/update/remove/settle/dispose`.
`EntrySnapshot.children` có thứ tự; `parent` là owner logic hiện tại.
Group lồng nhau vẫn tồn tại khi verdict disabled ngăn hậu duệ.
Đầu vào group runtime được preflight toàn bộ; cập nhật source giữ các entry
con hợp lệ độc lập. Đổi thứ tự cùng group không restart; chuyển group/scope
giữ EntryId nhưng rebuild Fiber outer/inner sau cleanup.

Mỗi generation retire có barrier chia sẻ được giữ lại. Disposal parent/loader
join barrier ngay cả khi đã bỏ waiter và handle Loader.
Con đã được nhận sang group khác chỉ bị group hiện tại gỡ.
ID duy nhất toàn loader và địa chỉ group có thể đổi thay cho ID cục bộ trong
cây nguồn lồng nhau của upstream. Không mô phỏng prototype mutable hay API
reparent của core. Xem [hướng dẫn](../guides/groups.md), [API loader](loader.md)
và [đối chiếu nguồn](../../plans/2026-11-02-cordis-completion/group.md).
