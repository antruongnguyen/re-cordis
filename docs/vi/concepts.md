# Khái niệm và mô hình thực thi

[English](../en/concepts.md) | [Trang tiếng Việt](README.md)

## Mô hình đối tượng

| Khái niệm | Ý nghĩa |
|---|---|
| `Runtime` | Chủ sở hữu tường minh của host tác vụ cục bộ và trạng thái framework |
| `Context` | Góc nhìn không sở hữu, chọn metadata, phạm vi và chủ tài nguyên |
| `Plugin<C, R>` | Khai báo tái sử dụng: cấu hình nguồn `C`, đầu vào apply đã kiểm tra `R` |
| Fiber / `FiberHandle<C>` | Một phiên bản plugin đã tiếp nhận và bộ điều khiển vòng đời |
| `Registry` | Nhóm Fiber đang lưu trú theo định danh khai báo Plugin |
| `ServiceKey<T>` | Ô dịch vụ có tên và hợp đồng giá trị `T` được kiểm tra |
| Publication | Một lần công bố chính xác, có `PublicationId` |
| Effect | Đăng ký khởi tạo/dọn tài nguyên thuộc một chủ sở hữu |
| Event | Hợp đồng có kiểu cho thông báo, truy vấn hoặc middleware bao quanh |

Clone Plugin giữ nguyên định danh khai báo. Hai Plugin được tạo riêng là khác
nhau dù trùng tên và kiểu closure. Restart Fiber giữ `FiberId`; rút và công bố
lại dịch vụ tạo `PublicationId` mới ngay cả khi giá trị bằng nhau.

## Thực thi và quyền sở hữu là hai việc khác nhau

```text
Bộ thực thi Tokio
  -> Runtime::run điều khiển tác vụ framework cục bộ
       -> Registry giữ các Fiber đang lưu trú
       -> mỗi Fiber sở hữu effect và dọn dẹp plugin con

Context --tham chiếu yếu--> runtime và chủ vòng đời đã chọn
```

Context dẫn xuất không sở hữu runtime. `extend` thêm metadata có kiểu;
`root` chọn gốc thường trực và bỏ metadata dẫn xuất. Quan hệ kế thừa metadata,
sở hữu vòng đời, cô lập dịch vụ và quyền truy cập phụ thuộc là các quan hệ riêng.

Tác vụ Tokio được spawn thông thường không kế thừa quyền tiếp nhận công việc
hay ngữ cảnh quy thuộc callback của framework. Không xử lý lỗi non-`Send`
bằng cách giả định `tokio::spawn` là API hosting thay thế.

## Trạng thái vòng đời

| Trạng thái | Diễn giải |
|---|---|
| `Pending` | Thiếu publication bắt buộc; hoãn kiểm tra cấu hình |
| `Loading` | Đang kiểm tra cấu hình hoặc chạy apply |
| `Active` | Apply hoàn tất thành công |
| `Failed` | Kiểm tra/apply thất bại và effect đã thu được đã rollback |
| `Unloading` | Đang dọn tài nguyên và plugin con |
| `Disposed` | Đã hoàn tất dispose cuối cùng |

Không phải thao tác nào cũng đi qua mọi trạng thái. Dispose từ Pending/Failed
có thể tới thẳng Disposed. `state()` chỉ là ảnh chụp, không phải mốc đồng bộ.

## Thao tác đã bắt đầu và future Rust thông thường

Các phương thức `update`, `restart`, `dispose`, `drain` tiếp nhận/bắt đầu công
việc ngay khi được gọi. Handle trả về dùng để chờ công việc đã bắt đầu.
Thả handle chờ không hủy công việc khi host vẫn tiếp tục được thực thi.

Thân async Rust vẫn chạy lười. Factory của callback có thể chạy phần đầu đồng
bộ, nhưng trả về một khối async không khiến thân khối chạy ngay.
Dùng API đồng bộ tường minh khi thời điểm thực thi đồng bộ là yêu cầu.

`ready` quan sát lượt hiện tại của một Fiber. Nó có thể trả về Pending trước
khi provider xuất hiện và bắt đầu lượt khác. Chờ provider không chờ thay mọi
consumer. Chờ provider liên quan rồi consumer liên quan, và kiểm tra trạng thái
trả về nếu cần Active.

## Vòng đời tài nguyên

Plugin con thuộc chủ tạo ra nó. Effect, listener, exporter và publication thuộc
chủ đăng ký, trừ khi được chuyển quyền tường minh. Drain gốc có thể dùng lại và
chỉ bao phủ ảnh chụp, không phải dừng runtime vĩnh viễn.

Injection đã commit giữ provider chính xác trong suốt setup, rollback và dọn
dẹp cuối của consumer. Chỉ giữ `Rc<T>` hoặc `BoundService<T>` **không** tạo quyền
giữ tài nguyên của provider còn sống.

Đọc [dọn dẹp](guides/effects-and-cleanup.md) và
[dịch vụ](guides/services-and-scoping.md) trước khi chia sẻ giá trị gắn tài nguyên.

## Phạm vi tương thích

Rust dùng lời gọi bound, accessor, khởi tạo và tiêm phương thức tường minh.
Không có Proxy, prototype, decorator, tự khám phá class, native hot loading,
async Drop hay runtime an toàn đa luồng/độc lập bộ thực thi tự động.

Đây là ranh giới API có chủ ý. Xem [đánh giá tương thích](../m6.md) để biết các
biểu diễn và khác biệt an toàn, cùng [bảng thuật ngữ](glossary.md) dùng chung cho
hai ngôn ngữ.
