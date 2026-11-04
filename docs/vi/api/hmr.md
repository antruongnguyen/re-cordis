# Satellite HMR

[English](../../en/api/hmr.md) | [Mục lục API](README.md)

Nguồn: [lib.rs](../../../crates/re-cordis-hmr/src/lib.rs),
[watch.rs](../../../crates/re-cordis-hmr/src/watch.rs) và
[reload.rs](../../../crates/re-cordis-hmr/src/reload.rs).
Hướng dẫn: [thay đổi tệp và thay thế plugin](../guides/hmr.md).

`re-cordis-hmr` chưa phát hành (`0.0.0`, Rust 2024, MSRV 1.85).
Trang này mô tả API Rust đã triển khai. H2 đã được nghiệm thu độc lập;
review watcher/tích hợp và nghiệm thu chung còn chờ.
Theo dõi tệp, thay thế plugin đã liên kết và thông báo cần restart là ba khả
năng riêng. Không có duyệt root đệ quy, tìm module, import/cache Node,
nạp binary động hay bộ điều phối tiến trình.

## WatchOptions và capability

Tạo option bằng `WatchOptions::new(base)`. Các field và mặc định:

| Field | Mặc định / hợp đồng |
|---|---|
| `base: PathBuf` | Base native do caller cung cấp, giải một lần; hỗ trợ đường dẫn không phải UTF-8 |
| `poll_interval: Duration` | 100 ms, phải dương |
| `debounce: Duration` | 50 ms; hỗ trợ zero |
| `max_watches: usize` | 256 đăng ký active và đã dừng nhưng chưa hoàn tất |
| `queue_capacity: usize` | 128 message đầu vào |
| `event_capacity: usize` | 128 event lưu lại, loại event cũ nhất khi đầy |
| `max_file_bytes: u64` | 4 MiB; mỗi mẫu đọc tối đa bound + 1 theo chunk |
| `ignored: Vec<OsString>` | Tên thành phần chính xác `.git`, `target`, `node_modules` |

Capacity dương và timer admission có kiểm tra phải hợp lệ trước khi producer
khởi động. max_file_bytes phải nhỏ hơn u64::MAX; đường dẫn tối đa 4096 byte
native. ignored nhận tối đa 256 tên, mỗi tên tối đa 4096 byte native. Polling dùng metadata và digest streaming hữu hạn; có thể bỏ lỡ
thay đổi giữa hai mẫu hoặc do va chạm digest. Đây là theo dõi tệp thông thường,
không phải confinement trước filesystem thù địch. Nghiệm thu runtime
Windows/Linux còn chờ.

`WatchCapabilities` cho biết `backend: WatchBackend` (`OwnedPolling` hoặc
`Synthetic`), `file_callbacks: bool` và `source_restart_required: bool`.
Đọc snapshot không nhận việc mới và không phát cảnh báo.

## FileWatcher

FileWatcher sở hữu Fiber core riêng, giữ cục bộ và không có Send/Sync.
Tạo/đăng ký trên host Cordis gốc còn sống với Tokio time được bật.

| Method | Kết quả | Hợp đồng |
|---|---|---|
| `new(owner, options)` | `Result<FileWatcher, WatchError>` | Producer polling có ownership và shutdown được chờ |
| `synthetic(owner, options)` | `Result<(FileWatcher, SyntheticEvents), WatchError>` | Cùng dispatch hữu hạn; không có producer filesystem |
| `watch(path, callback)` | `Result<WatchToken, WatchError>` | Tệp thường đăng ký rõ ràng; ghi đè ignored |
| `observe(path, callback)` | `Result<Option<WatchToken>, WatchError>` | Thành phần ignored trả None trước khi nhận việc |
| `watch_source(path)` | `Result<WatchToken, WatchError>` | Đường dẫn rõ ràng phát RestartRequired |
| `observe_source(path)` | `Result<Option<WatchToken>, WatchError>` | Đăng ký source có lọc |
| `capabilities()` | `WatchCapabilities` | Snapshot giá trị |
| `stats()` | `WatchStats` | Việc còn giữ/active và counter queue/history hữu hạn |
| `take_events()` | `Vec<WatchEvent>` | Lấy hết history hiện tại; dùng được ngoài host |
| `dispose()` | `Result<LifecycleTask, WatchError>` | Đóng admission/dispatch rồi chờ task và producer |

Callback là `FnMut(Change) -> LocalFuture<Result<(), BoxError>> + 'static`.
Invocation và poll future dùng hosting core đã được chấp nhận. Mỗi đăng ký
có một callback active và một lượt tiếp theo được gộp; đăng ký không liên
quan tiến triển độc lập. Lỗi trả về giữ nguyên nguồn; containment panic giữ
các event sau còn dùng được và giữ diagnostic core.

Đường dẫn dùng identity của parent gần nhất đang tồn tại cộng suffix logic,
không dùng inode cuối. Đường dẫn tương đối giải từ base; cho phép thiếu
tệp/parent. Symlink cuối hoặc ancestor bị từ chối khi đăng ký; symlink/thư
mục phát sinh khi lấy mẫu báo lỗi và cho phép phục hồi bằng tệp thường.
Đăng ký thật nhận Rescan ban đầu. Dừng token cuối thay generation đường dẫn,
nên event cũ đang queue không gọi token mới trên cùng đường dẫn.

Drain lifecycle self/ancestor từ callback được host bị từ chối trước tác
động stop. Dispose chờ sau khi lệnh đọc OS đã bị block trả về; không hứa
deadline shutdown cứng. Drop yêu cầu stop; barrier owner quan sát completion.
Drop LifecycleTask không thu hồi cleanup đã nhận.

## Token, đầu vào synthetic và quan sát

`WatchToken` là đăng ký cục bộ chính xác, không clone, có
`id() -> WatchId`, `path() -> &Path` và `stop()` idempotent.
Stop/Drop yêu cầu hủy, dùng được ngoài host; completion cần host tiến triển
hoặc dispose owner/watcher. Dừng một token không bỏ callback sibling.

`SyntheticEvents` clone được và có Send + Sync. `emit(Change)` trả
`Result<SyntheticDelivery, SyntheticError>`; `rescan()` trả
`Result<(), SyntheticError>`. Dùng đường dẫn token tuyệt đối.
Bỏ qua đường dẫn chưa biết. Queue đầy trả `RescanRequested`, phục hồi bằng
tập đăng ký hữu hạn; `Queued` là nhận message, không phải callback đã xong.
SyntheticError gồm `Closed` và `InvalidPath`.

`Change { path: PathBuf, kind: ChangeKind }` dùng Created, Modified, Removed
hoặc Rescan. `WatchEvent` gồm Change, RestartRequired và Failure.
`WatchFailure` có watch/path tùy chọn, `WatchFailureStage`
Read/Callback/Producer và nguồn lỗi gốc. Eviction history có thể loại lỗi.

WatchStats có `watches` (active cộng retirement chưa xong), `active`,
`paths`, `queued`, `pending`, `callbacks_running`, `events`,
`queue_overflows`, `events_evicted` và `producer_running`.
Counter saturate. Việc pending bị chặn bởi số đăng ký, không bởi độ dài burst.

WatchError gồm Admission, Lifecycle, Timer, Io (path/source),
InvalidOptions, InvalidPath (path/reason), LimitExceeded và Disposed.
Variant có source giữ nguyên nguyên nhân.

## Adapter Include tùy chọn

Với feature `include`, gọi
`watch_include(path, include) -> Result<WatchToken, WatchError>`.
Thao tác đăng ký một đường dẫn kích hoạt rõ ràng và refresh Include được truyền
vào, gồm đồ thị nguồn của nó. Đường dẫn không được tự tìm; đăng ký tường minh
vượt qua ignored. Include không phụ thuộc HMR.

Adapter giữ IncludeTask đã nhận độc lập với cancellation callback,
chạy tối đa một refresh và một lượt tiếp theo được gộp cho mỗi đăng ký,
và chờ completion đã giữ khi dispose watcher/ancestor.
Cleanup không nhận việc Include mới. Lỗi Include trả về và từng lỗi Report đi
vào history watcher hữu hạn đúng một lần cho mỗi refresh hoàn tất, giữ nguyên
nguyên nhân và vị trí. Lỗi cleanup core thông thường dùng diagnostic Runtime;
cần kiểm tra thêm `Runtime::take_failures`. Retirement chưa xong tính vào
max_watches; adapter không dispose Include được truyền vào.

## ReloadNode, ReloadPlan và FactoryReplacement

`ReloadNode { name: String, factory: Option<String>, dependencies: Vec<String> }`
là khai báo rõ ràng của ứng dụng. None đánh dấu node chỉ biểu diễn phụ thuộc.
Tên node/factory duy nhất, dài 1–256 byte UTF-8. Mỗi edge trỏ tới node tồn tại.
Giới hạn là 256 node, 1024 edge và 256 changed unit/candidate.
Chu kỳ hội tụ bằng fixed point hữu hạn.

| Method | Hợp đồng |
|---|---|
| `ReloadPlan::new(nodes) -> Result<ReloadPlan, LoaderError>` | Kiểm tra tên, mapping, edge và bound |
| `affected(changed) -> Result<Vec<String>, LoaderError>` | Closure phụ thuộc ngược, factory theo thứ tự node; unit không biết báo lỗi |
| `replace(loader, changed, replacements) -> Result<LoaderTask<Settlement>, LoaderError>` | Tập candidate chính xác và một batch loader |
| `replace_include(include, changed, replacements) -> Result<IncludeTask<Report>, IncludeError>` | Feature include tùy chọn: cùng kiểm tra chọn candidate và completion Include/loader chính xác |

Method tùy chọn `replace_include` chuyển qua `Include::replace_factories`
cho entry được cùng instance Include quản lý. Nó giữ journal, data,
provenance và byte tệp, không flush, rồi trả đúng report batch loader qua
IncludeTask. Include vẫn không phụ thuộc HMR.

Plan không sở hữu tài nguyên hay worker. Tập thay đổi rỗng không chọn factory;
candidate thừa vẫn báo lỗi. Phụ thuộc dùng chung không đổi không chọn consumer
không liên quan. Tên tệp hay metadata Cargo không tự tạo đồ thị.

HMR re-export `FactoryReplacement` của loader. Dùng `new(name, Plugin<C,R>)`,
`name()` và `with_preflight(Fn(&Config) -> Result<(), E>)` tùy chọn.
Loader kiểm tra toàn batch trước khi công bố catalog hoặc unload:
sự tồn tại, duy nhất, kiểu source/config được giữ, bound và preflight
cấu hình thô được host. Preflight phải thuần; admission thay đổi cùng loader
và dispose có guard. Nó không thay validation Plugin core sau khi đủ phụ
thuộc hay biến tác động bên ngoài thành giao dịch.

Loader giữ config riêng từng entry và provider cũ chính xác qua cleanup,
rồi dùng ý định entry/parent sống mới nhất. Sibling tiến triển độc lập.
Candidate host được cung cấp dựng lại plugin core lồng nhau; không tự thay
declaration lồng trong host không đổi.
Settlement báo lỗi sau unload. Candidate/update rõ ràng sau đó phục hồi;
teardown đã xong không rollback.
