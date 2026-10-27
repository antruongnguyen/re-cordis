# Ghi log

[English](../../en/api/logging.md) | [Mục lục API](README.md)

Mã nguồn: [logger.rs](../../../crates/re-cordis-core/src/logger.rs).
Hướng dẫn có mã chạy: [ghi log](../guides/logging.md).

## Logger và cấu hình

| API | Hợp đồng |
|---|---|
| `Context::logger(Option<&str>)` | Lấy kênh không sở hữu; tên/thiết lập được chốt |
| `Logger::with_level` | Clone handle với mức chi tiết ghi đè tường minh |
| `log`, `error`, `warn`, `info`, `debug` | Định dạng Display đồng bộ; trả `Result<LogRecord, LogError>` |
| `log_error(level, ErrorValue)` | Ghi cây lỗi đã thích nghi; trả `Result<Vec<LogRecord>, LogError>` |
| `Context::log_buffer` | Ảnh chụp 1000 bản ghi được chấp nhận gần nhất theo thời gian |

Ghi log cần runtime sống, không cần phạm vi run đang hoạt động.
Đăng ký/gỡ exporter cần tiếp nhận từ host.
Thứ tự mức là `Level::Error`, `Warn`, `Info`, `Debug`; mức chi tiết tối đa
chấp nhận mọi cấp đến và bao gồm cấp đó.

`LoggerConfig` có trường công khai tùy chọn `name`, `level`. Trường thiếu kế
thừa lớp ngoài. Tên khi lấy tường minh ưu tiên hơn tên intercept, rồi tên Fiber
nơi định nghĩa đã chuẩn hóa. Mức mặc định Info.
Dùng intercept logger trên `ServiceKey::<Logger>::new("logger")` qua
`Context::intercept`; không cần công bố logger tùy chỉnh để ghi đè.

## Bản ghi và định dạng

`LogRecord` công khai:

| Trường | Ý nghĩa |
|---|---|
| `sequence: u64`, `timestamp: SystemTime` | Số thứ tự trong runtime bắt đầu từ một; thời gian đồng hồ thực |
| `name`, `level`, `message` | Kênh, cấp độ, chuỗi đã định dạng Rust |
| `fiber: FiberId`, `fiber_ref: WeakFiber` | Quy thuộc không giữ vòng đời |
| `failure: Option<CleanupFailure>` | Nguồn lỗi tường minh/tự động gốc |

`WeakFiber::snapshot` trả `Option<hooks::FiberInfo>` hiện tại; handle Disposed
còn giữ có thể vẫn cho metadata đến khi allocation được giải phóng.
Metadata gốc thuộc root thường trực.

`LogRecord::format(max_length)` giới hạn mỗi dòng theo UTF-16, thêm `...` khi
cắt ngắn, chuẩn hóa CRLF và giữ Unicode scalar. Dùng 10240 cho mặc định upstream.
`color(value, depth)` tô màu kênh tùy chọn với `ColorDepth::None`, `Ansi16`,
`Ansi256`. Không tự chèn ANSI vào bộ đệm.

## ErrorValue

| Builder | Ý nghĩa |
|---|---|
| `new(error)` | Sở hữu lỗi Rust |
| `from_rc(Rc<dyn Error>)` | Giữ chính xác định danh lỗi chia sẻ |
| `with_cause(value)` | Ghi cause trước lỗi ngoài; ưu tiên hơn thành viên |
| `with_members(Vec<ErrorValue>)` | Trải phẳng đệ quy thành viên aggregate không rỗng |

Danh sách thành viên rỗng vẫn ghi lỗi gốc. Ghi Display không tự khám phá đồ
thị cause/aggregate tùy ý hay diễn giải chỉ thị định dạng JavaScript.

## Exporter

`Context::add_exporter(options, callback)` đăng ký callback có phạm vi owner
`&LogRecord -> Result<(), E>`, trả `ExporterRegistration`.
`remove` gỡ đúng exporter đó đồng bộ; `into_effect` chuyển cleanup vào composition.
Drop không có tác dụng dọn dẹp.

`ExporterOptions` có `default_level: Option<Level>`,
`levels: BTreeMap<String, Level>`. Mức theo tên chính xác ưu tiên hơn mặc định
exporter, rồi mức kênh. Định tuyến độc lập theo từng exporter;
chấp nhận vào bộ đệm và chọn exporter không phải cùng thao tác.

Mọi exporter được chọn đều được thử. `LogError::Exporters` giữ mọi nguồn
callback lỗi. Các variant khác là RuntimeDropped, unwind khi định dạng,
cấu hình không tương thích và hết số thứ tự.
Lỗi không có nghĩa mọi đích đều không nhận gì.
