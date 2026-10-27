# Ghi log và chẩn đoán

[English](../../en/guides/logging.md) | [Trang tiếng Việt](../README.md)

## Thêm exporter ra terminal

Core có bản ghi có cấu trúc và bộ đệm giới hạn, không có exporter console ngầm.
Đăng ký exporter trên owner cần điều khiển vòng đời của nó.

```rust
use re_cordis_core::{BoxError, ExporterOptions, Runtime};
use std::convert::Infallible;

fn main() -> Result<(), BoxError> {
    tokio::runtime::Builder::new_current_thread().build()?.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                root.add_exporter(ExporterOptions::default(), |record| {
                    println!("[{}] {}", record.name, record.format(10240));
                    Ok::<_, Infallible>(())
                })?;
                root.logger(Some("app")).info("ready")?;
                assert!(root.log_buffer()?.iter().any(|record| record.message == "ready"));
                Ok::<_, BoxError>(())
            }.await;
            let cleanup = root.drain()?.await?;
            if let Some(failure) = cleanup.failures().first() {
                return Err(failure.clone().into());
            }
            result
        }).await?
    })
}
```

## Tên kênh và cấp độ

`ctx.logger(Some("name"))` được ưu tiên hơn intercept `LoggerConfig` tại nơi sử
dụng. Khi không có tên tường minh, tên intercept được ưu tiên hơn tên Fiber nơi
định nghĩa đã chuẩn hóa. Tên được chốt lúc lấy Logger; lời gọi dịch vụ bound
lồng nhau được quy thuộc nơi định nghĩa tương ứng.

Mức chi tiết tối đa mặc định là Info. `with_level` ghi đè cấp độ của handle.
Mỗi exporter có mức mặc định và mức theo tên chính xác qua `ExporterOptions`.
Cấp độ theo tên của exporter ưu tiên hơn mặc định của nó.
`LogError::Configuration` báo kiểu intercept không tương thích; framework không
âm thầm bỏ qua.

## Bản ghi và lỗi

Bộ đệm giữ 1000 bản ghi được chấp nhận gần nhất theo thứ tự thời gian và tồn
tại qua các lần drain gốc. Bản ghi gồm số thứ tự, thời gian đồng hồ thực, kênh,
cấp độ, thông điệp, định danh Fiber, metadata yếu và lỗi gốc tùy chọn.
`WeakFiber::snapshot` có thể hết hiệu lực; giữ bản ghi không giữ runtime/Fiber sống.

Dùng `format!`/Display của Rust cho thông điệp thường. Không diễn giải định
dạng printf/JSON JavaScript. Dùng `Logger::log_error` với `ErrorValue` khi cần
biểu diễn cause/aggregate tường minh:

- Cause được ghi trước lỗi ngoài và ưu tiên hơn danh sách thành viên.
- Danh sách thành viên không rỗng được trải phẳng đệ quy thay vì ghi aggregate.
- Danh sách rỗng vẫn ghi lỗi gốc.

Đây là adapter tường minh, không tự duyệt mọi đồ thị `Error::source` của Rust.

## Định dạng và lỗi exporter

`LogRecord::format` giới hạn mỗi dòng theo đơn vị UTF-16, chuẩn hóa CRLF, không
cắt đôi Unicode scalar. Zero là giới hạn bằng không thực sự.
`color` hỗ trợ plain, ANSI-16, ANSI-256; exporter quyết định có dùng màu không.

Mọi exporter được chọn đều được thử dù một exporter lỗi. Xử lý `LogError`;
lỗi không có nghĩa các exporter khác không nhận gì. Chẩn đoán framework còn
báo stderr và giữ lỗi mà không đưa đệ quy trở lại exporter chẩn đoán đã hỏng.

`Runtime::take_failures` lấy và xóa lỗi callback framework được giữ lại, không
phải bộ đệm log. Ứng dụng cần kiểm tra/xử lý chúng phù hợp.

Tra cứu: [API ghi log](../api/logging.md).
