# Vệ tinh console logger

[English](../../en/api/console-logger.md) | [Mục lục API](README.md)

Mã nguồn: [lib.rs](../../../crates/re-cordis-logger/src/lib.rs).
Hướng dẫn: [ghi log ra console](../guides/console-logging.md).
Ví dụ: [console.rs](../../../crates/re-cordis-logger/examples/console.rs).

`re-cordis-logger` (`0.1.0-alpha.1`, chưa phát hành, Rust 2024, MSRV 1.85) tương ứng
với `logger-console` của TypeScript. Crate không thêm dịch vụ Logger, bộ lọc,
buffer, runtime ẩn hay tác vụ nền. Định dạng lịch dùng `time = 0.3.36`
chỉ với tính năng std/formatting; phụ thuộc của core không thay đổi.

## Đăng ký và render thuần

| Hàm | Chữ ký / kết quả | Hợp đồng |
|---|---|---|
| `register` | `(&Context, W, ConsoleOptions, ExporterOptions) -> Result<ExporterRegistration, AdmissionError>`, `W: Write + 'static` | Writer được sở hữu; chấp nhận giá trị cục bộ; Auto là thuần văn bản. |
| `register_stdout` | `(&Context, ConsoleOptions, ExporterOptions) -> Result<ExporterRegistration, AdmissionError>` | Mọi mức đến stdout; giải quyết Auto một lần lúc đăng ký. |
| `render` | `(&LogRecord, &ConsoleOptions, Option<SystemTime>) -> io::Result<String>` | Render byte thuần có newline cuối; bên gọi cung cấp mốc thời gian. |

Đăng ký cần đúng host `Runtime::run` và chủ sở hữu còn sống. Từ chối giải phóng
writer đã cung cấp mà không export. Token core có Drop không tác động;
`remove()` tiêu thụ token và gỡ đúng lần đăng ký. Cleanup/restart chủ sở hữu
gỡ lần đăng ký và giải phóng capture. Snapshot core cho phép callback đã được
chọn hoàn tất sau khi gỡ. Token của lần đăng ký đã được dispose có thể trả
`false` khi gỡ sau đó.

## `ConsoleOptions`, `LabelStyle`, `Alignment`, `Timestamp`

| Trường | Mặc định | Hành vi |
|---|---|---|
| `colors` | `ColorMode::Auto` | Phát hiện stdout; thuần văn bản với writer tùy chỉnh/render. |
| `max_length` | `10240` | Mỗi dòng thông điệp tính theo UTF-16, không tính `...` thêm vào; zero là giới hạn thật. |
| `timestamp` | `Timestamp::Utc` | Thời gian bản ghi, UTC cố định `yyyy-MM-ddTHH:mm:ss.SSSZ `; `Off` bỏ timestamp. |
| `show_diff` | `false` | Thêm mili giây nguyên, gồm ` +0ms` cho bản ghi đầu tiên hoặc thời gian lùi. |
| `label.width` | `0` | Độ rộng tối thiểu theo UTF-16; giữ tên dài hơn. |
| `label.margin` | `1` | Khoảng trắng giữa prefix, nhãn và thông điệp. |
| `label.align` | `Alignment::Left` | Prefix trước; `Right` đặt nhãn đã đệm trước prefix. |

Prefix mức log là `[E]`, `[W]`, `[I]`, `[D]`. Định dạng core chuẩn hóa
CRLF và không tách scalar Unicode. Dòng tiếp nối khi căn trái thụt bằng độ dài
timestamp + 3 + margin; căn phải thêm độ rộng nhãn danh nghĩa + margin, theo
bố cục bất đối xứng của upstream. Đệm UTF-16 là xác định, không đo độ rộng ô
terminal/grapheme. Nhãn ANSI256 in đậm; nhãn ANSI16 giữ nét thường.
Timestamp xám dùng SGR90 ở ANSI16 và SGR38;5;8 ở ANSI256.
Plain không thêm escape và giữ escape sẵn có trong văn bản người dùng.

UTC dùng thời gian bản ghi thay cho template giờ địa phương lúc render của
upstream. Template/múi giờ tùy chỉnh nằm ngoài phần này. Năm UTC ngoài 0–9999
và bố cục không thể biểu diễn trả `InvalidInput`. Phần nhỏ hơn mili giây bị cắt.
Thời gian chênh lệch luôn dùng mili giây nguyên thay cho định dạng thời lượng
dễ đọc của upstream; ghi thành công với thời gian lùi vẫn trở thành mốc kế tiếp.

## `ColorMode`

`Plain`, `Ansi16` và `Ansi256` là lựa chọn rõ ràng, ưu tiên hơn môi trường.
`Auto` cho stdout chụp `IsTerminal` và giá trị Unicode của biến môi trường một lần:

1. Có `FORCE_COLOR`: `0` thuần, `1` ANSI16, `2`/`3` ANSI256;
   giá trị khác (kể cả rỗng) chọn ANSI16.
2. `NO_COLOR` không rỗng tắt màu.
3. Stdout không phải terminal hoặc `TERM=dumb` tắt màu.
4. `TERM` chứa `256color` chọn ANSI256; terminal khác chọn ANSI16.

Bỏ qua giá trị không phải Unicode. Truecolor mức 3 bị giới hạn ở palette ANSI256
của core; không mô phỏng heuristic console trình duyệt/Windows.
`ColorMode::resolve(terminal, no_color, force_color, term)` áp dụng chính sách
này cho dữ liệu đã chụp, không đọc trạng thái tiến trình, và trả `ColorDepth`
của core. Render thuần và writer tùy chỉnh coi Auto chưa giải quyết là thuần văn bản.

## I/O và gọi lại

Adapter gọi `write_all` đồng bộ rồi một lần `flush`. Không có queue:
writer chậm chặn lời gọi log. `write_all` xử lý ghi ngắn/interrupted tiêu chuẩn;
write-zero, lỗi ghi và flush vẫn là nguồn I/O gốc có thể kiểm tra trong
`LogError::Exporters`. Exporter độc lập vẫn chạy. Không thể hoàn tác byte một
phần hay flush thất bại; adapter không thử lại chúng. Timestamp bản ghi trước
chỉ cập nhật sau khi cả hai thành công.

Export writer đệ quy trả lỗi I/O qua cơ chế lỗi exporter của core. Nó không
lấy mutable borrow lần hai hay tạo chẩn đoán đệ quy tự động. Destructor writer
chạy khi core giải phóng snapshot callback; gỡ đồng bộ thường giải phóng ngay,
nhưng snapshot đang thực thi trì hoãn việc giải phóng đến khi callback đã chọn
hoàn tất.

## Ví dụ

```rust
use std::time::{Duration, UNIX_EPOCH};
use re_cordis_core::{BoxError, Runtime};
use re_cordis_logger::{ColorMode, ConsoleOptions, render};

fn main() -> Result<(), BoxError> {
    let runtime = Runtime::new();
    let mut record = runtime.context().logger(Some("app")).info("hello\nworld")?;
    record.timestamp = UNIX_EPOCH + Duration::from_millis(123);
    let options = ConsoleOptions {
        colors: ColorMode::Plain,
        show_diff: true,
        ..Default::default()
    };
    let text = render(&record, &options, Some(UNIX_EPOCH))?;
    assert_eq!(
        text,
        "1970-01-01T00:00:00.123Z [I] app hello\n                             world +123ms\n"
    );
    Ok(())
}
```
