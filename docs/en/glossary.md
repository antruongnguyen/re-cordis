# Glossary

[Tiếng Việt](../vi/glossary.md) | [English home](README.md)

API identifiers remain unchanged in both languages.

| English | Vietnamese term | Meaning |
|---|---|---|
| Context | ngữ cảnh (`Context`) | A view into one runtime and selected owner/scope |
| Runtime / host | runtime / bộ chủ trì thực thi | Explicit owner and driver of local framework tasks |
| Plugin declaration | khai báo plugin | Reusable typed component definition |
| Fiber | phiên bản plugin (`Fiber`) | One live/resident plugin instance, not an OS thread |
| Registry | sổ đăng ký (`Registry`) | Plugin-identity grouping of resident instances |
| Service | dịch vụ | Typed capability published under a name |
| Service slot | ô dịch vụ | Name/isolation-selected publication location |
| Publication | lần công bố dịch vụ | One exact provider occurrence |
| Dependency injection | tiêm phụ thuộc | Declared, committed access to required services |
| Effect | hiệu ứng tài nguyên (`effect`) | Owned setup/cleanup registration |
| Disposer | bộ dọn dẹp (`Disposer`) | One owned cleanup or a composed cleanup chain |
| Drain | rút và dọn tài nguyên (`drain`) | Claim and await a journal snapshot |
| Readiness | trạng thái sẵn sàng | Current-pass settlement; not always Active |
| Settlement barrier | mốc chờ hoàn tất | Wait point for a particular operation/pass |
| Source / resolved config | cấu hình nguồn / đã kiểm tra | Retained input versus validated apply input |
| Isolation | cô lập | Explicit service-slot and authority boundary |
| Intercept | lớp cấu hình chặn (`intercept`) | Use-site configuration layer, not arbitrary method interception |
| Definition / use site | nơi định nghĩa / nơi sử dụng | Dependency authority versus effect/configuration attribution |
| Around-middleware | middleware bao quanh | A chain with a one-use continuation |
| Exporter | bộ xuất log | Scoped callback receiving selected log records |
| Cancellation | hủy thực thi | Stopping outstanding work, not successful async cleanup |
| Lease | quyền giữ tài nguyên còn sống | A lifetime guarantee that plain `Rc<T>` reads do not provide |
