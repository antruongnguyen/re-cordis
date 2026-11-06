# Tài liệu API

[English](../../en/api/README.md) | [Trang tiếng Việt](../README.md)

Tài liệu bao phủ export công khai của `re-cordis-core` và vệ tinh
`re-cordis-timer`, `re-cordis-logger`, `re-cordis-loader`, `re-cordis-include`, `re-cordis-hmr` và `re-cordis-create` ngoại tuyến đã chọn, không phải bộ lưu trữ nội bộ hay handle riêng của ví dụ. Nội dung mô tả cách dùng và hợp đồng hành vi;
Rustdoc được sinh từ mã nguồn là tham chiếu chính xác cho chữ ký, ràng buộc
generic, trait implementation và trường công khai.

[Hướng dẫn tích hợp](../guides/system-lifecycle.md) ghép các API này.
Các kiểu ứng dụng thuộc riêng ví dụ và không thêm export công khai.

[Website phát triển công khai](https://antruongnguyen.github.io/re-cordis/vi/docs/api/)
cũng kèm Rustdoc chính xác từ revision của bản dựng đã triển khai.
Xem [các kênh tài liệu](../guides/documentation.md) để biết đầu ra công khai và cục bộ,
API Include tùy chọn của HMR và chính sách docs.rs theo phiên bản chính xác.

## Sinh tài liệu API chính xác

Tại thư mục gốc repository:

```sh
cargo doc -p re-cordis-core --no-deps --locked
```

Mở `target/doc/re_cordis_core/index.html`, hoặc đường dẫn tương ứng trong
`CARGO_TARGET_DIR`. Lệnh không phát hành gì lên docs.rs.
## Rustdoc đã phát hành

Cả bảy package thử nghiệm `0.1.0-alpha.1` đã phát hành trên crates.io.
Dùng các trang đã kiểm chứng theo phiên bản chính xác để tra chữ ký đã phát hành:

- [re-cordis-core](https://docs.rs/re-cordis-core/0.1.0-alpha.1/re_cordis_core/)
- [re-cordis-timer](https://docs.rs/re-cordis-timer/0.1.0-alpha.1/re_cordis_timer/)
- [re-cordis-logger](https://docs.rs/re-cordis-logger/0.1.0-alpha.1/re_cordis_logger/)
- [re-cordis-loader](https://docs.rs/re-cordis-loader/0.1.0-alpha.1/re_cordis_loader/)
- [re-cordis-include](https://docs.rs/re-cordis-include/0.1.0-alpha.1/re_cordis_include/)
- [re-cordis-hmr](https://docs.rs/re-cordis-hmr/0.1.0-alpha.1/re_cordis_hmr/)
- [re-cordis-create](https://docs.rs/re-cordis-create/0.1.0-alpha.1/re_cordis_create/)

Rustdoc được host của HMR gồm API Include tùy chọn. Các bản dựng này không
mở rộng nghiệm thu nền tảng runtime. Rustdoc site hiện tại theo revision bản
dựng; mã nguồn phát hành là `84fa9bdf0eb4e631fce792bf163494b0a61aeba8`.

## Bản đồ export công khai

Mỗi export gốc từ [lib.rs](../../../crates/re-cordis-core/src/lib.rs) thuộc một
chương bên dưới, và export của vệ tinh `re-cordis-timer` được bao phủ bởi
[chương timer](timer.md); export console thuộc
[console logger](console-logger.md). Phương thức mở rộng Context được phân theo chức năng.

| Chương | Kiểu / module công khai |
|---|---|
| [Runtime và Context](runtime-context.md) | `Runtime`, `Context`, `RuntimeId`, `FiberId`, `PublicationId`, `IsolationId` |
| [Plugin và Registry](plugins-registry.md) | `Plugin`, `FiberHandle`, `FiberState`, `LifecycleTask`, `UpdateOptions`, `Registry`, `LifecycleError` |
| [Dịch vụ và reflection](services-reflection.md) | `ServiceKey`, `Dependency`, `ServiceRegistration`, `BoundService`, `ServiceInvoke`, `AccessorKey`, `Accessor`, `ServiceError`, `ReflectError` |
| [Effect và dọn dẹp](effects.md) | `Disposer`, `Effect`, `EffectMeta`, `Completion`, `CleanupReport`, `CleanupFailure`, `LocalFuture`, `BoxError`, `SetupError`, `TaskControl`, `CallbackError` |
| [Sự kiện](events.md) | `Event`, `Listener`, `ListenerOptions`, `DispatchOptions`, `ListenerRegistration`, `Next`, `AsyncNext`, `EventTask`, `EventError` |
| [Ghi log](logging.md) | `Logger`, `LoggerConfig`, `Level`, `LogRecord`, `WeakFiber`, `ErrorValue`, `ColorDepth`, `ExporterOptions`, `ExporterRegistration`, `LogError` |
| [Lỗi và hook](errors-hooks.md) | `HostError`, `AdmissionError`, `CompletionError`, `ValidationError`, `ValidationIssue`, `PathSegment`, module công khai `hooks` |
| [Vệ tinh Timer](timer.md) | `TimerExt`, `Sleep`, `IntervalStream`, `IntervalControl`, `Scheduled`, `TimerError` |
| [Vệ tinh Console](console-logger.md) | `ColorMode`, `Alignment`, `LabelStyle`, `Timestamp`, `ConsoleOptions`, `render`, `register`, `register_stdout` |
| [Loader](loader.md) | `Catalog`, `Config`, `EntrySpec`, `EntryKind`, `EntryId`, `FactoryId`, `FactoryReplacement`, `Loader`, `LoaderTask`, `LoaderError`, `EntrySnapshot`, `EntryState`, `EntryFailure`, `Settlement`, `Scope`, `Origin`, `CommitRecord`, `SourceId`, `SourceDocument`, `SourceLocation`, `SourceSpan`, `FieldPatch`, `EntryPatch`, `MAX_ENTRIES`, `MAX_JSON_BYTES`, `MAX_JSON_DEPTH`, `MAX_JSON_NODES`, `MAX_STRING_BYTES` |
| [Groups](groups.md) | `Group` |
| [Include](include.md) | `Include`, `IncludeOptions`, `IncludeTask`, `Report`, `JsonEdit`, `Conflict`, `IncludeError`, `Location`, `Owner`, `OwnerKind`, `Ownership`, `ParsedDocument`, `parse_document`, `MAX_BYTES`, `MAX_DEPTH`, `MAX_NODES`, `MAX_STRING_BYTES`, `MAX_FILES`, `MAX_ENTRIES`, `MAX_PATCHES`, `MAX_RETRIES` |
| [HMR](hmr.md) | `Change`, `ChangeKind`, `FactoryReplacement`, `FileWatcher`, `ReloadNode`, `ReloadPlan`, `SyntheticDelivery`, `SyntheticError`, `SyntheticEvents`, `WatchBackend`, `WatchCapabilities`, `WatchError`, `WatchEvent`, `WatchFailure`, `WatchFailureStage`, `WatchId`, `WatchOptions`, `WatchStats`, `WatchToken` |
| [Create](create.md) | `create`, `CreateError` |

## Đọc kết quả phương thức theo hai giai đoạn

Thao tác như `fiber.update(source)?` có thể lỗi tức thời trước tiếp nhận;
task trả về có thể lỗi sau đó khi được chờ. `Completion`, `LifecycleTask`,
`EventTask` là handle chỉ chờ, không phải lệnh bắt đầu khi poll lần đầu.
Giữ host tồn tại và được điều khiển đến lúc chúng hoàn tất.

`Effect::ready` là chờ setup async, không phải lệnh cleanup.
Ghi log và đọc thuần có yêu cầu tiếp nhận khác với đăng ký công việc mới;
xem từng chương thay vì cho rằng mọi phương thức đều cần `run` hoặc đều dùng
được bên ngoài nó.

## Tính ổn định và tương thích

R1 đã đánh giá API mà không thêm cam kết semver.
[Hợp đồng alpha](../../release-readiness.md) đã duyệt dùng bản ứng viên đồng bộ
`0.1.0-alpha.1` thử nghiệm đã phát hành. Dùng pin chính xác; thay đổi phá vỡ tương thích
cần ghi chú phát hành và phiên bản alpha đồng bộ mới. Trường công khai,
trait implementation và re-export phụ thuộc cũng ảnh hưởng tính tương thích.


Mọi API hiện chưa ổn định. Nhiều lỗi có `#[non_exhaustive]`; match ở ứng dụng
phải cho phép variant tương lai. ID là định danh opaque, so sánh/hash được;
không có hợp đồng số hóa, serialization hay thứ tự.

Tham chiếu ngữ nghĩa là Cordis TypeScript tại bản chốt, cùng các biểu diễn Rust
và khác biệt an toàn tường minh. Tài liệu cột mốc lịch sử là bằng chứng, không
thay thế việc kiểm tra chữ ký Rust hiện tại.
