# Ghi log ra console

[English](../../en/guides/console-logging.md) | [Trang tiếng Việt](../README.md)

`re-cordis-logger` là crate vệ tinh console thử nghiệm đã phát hành, dùng hệ thống log
sẵn có của core. Đăng ký stdout khi thiết lập plugin hoặc trong thao tác root
đang được host. Context sở hữu gỡ exporter khi dọn dẹp; core vẫn chọn mức log,
lưu bản ghi và giữ danh tính lỗi. Xem [API console](../api/console-logger.md)
để biết các mặc định và hành vi chính xác.

## Cài đặt và gỡ đầu ra

Thêm phụ thuộc đường dẫn cục bộ đến `crates/re-cordis-logger` cùng với
`re-cordis-core` và Tokio. Chương trình hoàn chỉnh này chọn đầu ra thuần văn bản,
không timestamp để stdout có thể tái lập. Khi dùng terminal thông thường,
`ConsoleOptions::default()` bật phát hiện màu tự động và timestamp UTC từ bản ghi.

```rust
use re_cordis_core::{BoxError, ExporterOptions, Plugin, Runtime};
use re_cordis_logger::{ColorMode, ConsoleOptions, Timestamp, register_stdout};

fn options() -> ConsoleOptions {
    ConsoleOptions { colors: ColorMode::Plain, timestamp: Timestamp::Off, ..Default::default() }
}

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let plugin = Plugin::new("console", |ctx, ()| async move {
                register_stdout(&ctx, options(), ExporterOptions::default())?;
                ctx.logger(Some("app")).info("plugin ready")?;
                Ok::<_, BoxError>(())
            });
            let fiber = root.plugin(&plugin, ())?;
            fiber.ready()?.await?;
            root.logger(Some("app")).warn("owned output")?;
            fiber.dispose()?.await?;
            let output = register_stdout(&root, options(), ExporterOptions::default())?;
            root.logger(Some("app")).info("manual output")?;
            output.remove()?;
            root.logger(Some("app")).info("buffer only")?;
            let report = root.drain()?.await?;
            assert!(report.failures().is_empty());
            assert!(root.get_effects()?.is_empty());
            Ok::<_, BoxError>(())
        }).await??;
        Ok::<_, BoxError>(())
    })
}
```

Đầu ra:

```text
[I] app plugin ready
[W] app owned output
[I] app manual output
```

Chạy ứng dụng này bằng
`cargo run -p re-cordis-logger --example console --locked` từ checkout.
Mọi mức log đều dùng stdout. Thả token đăng ký không gỡ exporter;
`remove()` giải phóng đúng lần đăng ký đó đồng bộ. Disposal, restart plugin
và drain root giải phóng các đăng ký chúng sở hữu. Snapshot exporter đã được
core chọn có thể hoàn tất sau khi gỡ; các bản ghi tương lai không gọi nó.

## Writer tùy chỉnh và lỗi

`register(&ctx, writer, options, filtering)` nhận giá trị `Write + 'static`
được sở hữu, kể cả writer kiểm thử dùng `Rc` cục bộ. Auto là thuần văn bản
đối với writer tùy chỉnh và render thuần; chọn `Ansi16`/`Ansi256` rõ ràng
hoặc dùng `ColorMode::resolve` với thông tin terminal đã thu thập. Plain nghĩa
là adapter không thêm escape ANSI; escape do người dùng cung cấp vẫn được giữ.

Mỗi bản ghi được chọn dùng `write_all` và một lần `flush`. Ghi ngắn và lỗi
interrupted theo hợp đồng thư viện chuẩn. I/O chậm chặn lời gọi log cục bộ đó.
Lỗi ghi/flush xuất hiện trong `LogError::Exporters`; core vẫn thử các exporter
độc lập. Không thể hoàn tác byte đã ghi một phần. Mốc tính thời gian chỉ cập
nhật sau khi ghi và flush thành công. Nếu writer ghi log đệ quy, lần export
lồng nhận lỗi I/O rõ ràng; adapter không panic do borrow và không tự ghi lỗi đó
lần nữa.

Dùng `Display`, `format!` và `Debug` của Rust để định dạng thông điệp.
Placeholder printf JavaScript, inspect object và cách console trình duyệt
hiển thị đối số không được tái tạo từ thông điệp đã định dạng của core.
Xử lý cause/aggregate của lỗi vẫn dùng API `ErrorValue` hiện có.
