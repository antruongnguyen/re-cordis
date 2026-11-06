# Bảng thuật ngữ

[English](../en/glossary.md) | [Trang tiếng Việt](README.md)

Giữ nguyên định danh API trong cả hai ngôn ngữ.

| Tiếng Anh | Thuật ngữ tiếng Việt | Ý nghĩa |
|---|---|---|
| Context | ngữ cảnh (`Context`) | Góc nhìn vào một runtime và chủ/phạm vi đã chọn |
| Runtime / host | runtime / bộ chủ trì thực thi | Chủ sở hữu và bộ điều khiển tác vụ framework cục bộ |
| Plugin declaration | khai báo plugin | Định nghĩa thành phần có kiểu, tái sử dụng được |
| Fiber | phiên bản plugin (`Fiber`) | Một phiên bản plugin sống/đang lưu trú, không phải luồng hệ điều hành |
| Registry | sổ đăng ký (`Registry`) | Nhóm phiên bản theo định danh Plugin |
| Service | dịch vụ | Khả năng có kiểu được công bố dưới một tên |
| Service slot | ô dịch vụ | Vị trí publication được chọn theo tên/phạm vi cô lập |
| Publication | lần công bố dịch vụ | Một lần cung cấp dịch vụ chính xác |
| Dependency injection | tiêm phụ thuộc | Quyền truy cập dịch vụ bắt buộc đã khai báo và commit |
| Effect | hiệu ứng tài nguyên (`effect`) | Đăng ký setup/cleanup có chủ sở hữu |
| Disposer | bộ dọn dẹp (`Disposer`) | Một hàm dọn dẹp thuộc quyền sở hữu hoặc chuỗi dọn dẹp ghép |
| Drain | rút và dọn tài nguyên (`drain`) | Nhận quyền xử lý và chờ một ảnh chụp sổ tài nguyên |
| Readiness | trạng thái sẵn sàng | Kết quả lượt hiện tại; không phải lúc nào cũng Active |
| Settlement barrier | mốc chờ hoàn tất | Điểm chờ một thao tác/lượt cụ thể |
| Source / resolved config | cấu hình nguồn / đã kiểm tra | Đầu vào được giữ lại so với đầu vào apply đã kiểm tra |
| Isolation | cô lập | Ranh giới ô dịch vụ và quyền truy cập tường minh |
| Intercept | lớp cấu hình chặn (`intercept`) | Lớp cấu hình nơi sử dụng, không chặn phương thức tùy ý |
| Definition / use site | nơi định nghĩa / nơi sử dụng | Quyền truy cập phụ thuộc so với nơi quy thuộc effect/cấu hình |
| Around-middleware | middleware bao quanh | Chuỗi có continuation dùng một lần |
| Exporter | bộ xuất log | Callback có phạm vi nhận bản ghi log đã chọn |
| Cancellation | hủy thực thi | Dừng công việc còn lại, không phải dọn dẹp async thành công |
| Lease | quyền giữ tài nguyên còn sống | Bảo đảm vòng đời mà việc đọc `Rc<T>` đơn thuần không cung cấp |
| Loader entry | entry của loader | Identity logic ổn định và options nguồn, khác Fiber có thể thay thế |
| Structural group | nhóm cấu trúc | Cha có danh sách con có thứ tự, vẫn tồn tại khi con bị vô hiệu hóa |
| Source reconciliation | đồng bộ nguồn cấu hình | Áp dụng snapshot nguồn của bên gọi mà không phát commit ghi ngược |
| Console exporter | bộ xuất log ra console | Writer native theo phạm vi chủ sở hữu, render các bản ghi core được chọn |
| Include source | nguồn include | Tệp hữu hạn cung cấp entry và patch cho loader có owner |
| Patch ownership | quyền sở hữu của patch | Vị trí file/patch thực sự cung cấp entry hoặc một khóa |
| Pending journal | nhật ký thay đổi chờ ghi | Thay đổi runtime đã gộp, có giới hạn và đang chờ lưu |
| Three-way reconciliation | hòa giải ba phía | So sánh nguồn trước đó, tệp hiện tại và thay đổi runtime; báo xung đột |
| Scaffolding | sinh khung dự án | Sinh ứng dụng cố định có thể chỉnh sửa từ checkout cục bộ |
| Staging directory | thư mục staging | Thư mục cùng cha thuộc generator, ghi đủ trước rename cuối |
