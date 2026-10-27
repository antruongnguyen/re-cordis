# Bắt đầu sử dụng

[English](../en/getting-started.md) | [Trang tiếng Việt](README.md)

## Yêu cầu và khai báo phụ thuộc

Dùng Rust 1.85 trở lên. `cordis-core` chưa được phát hành lên crates.io; không
cài một package khác chỉ vì tên tương tự.

Ví dụ ứng dụng nằm cạnh bản checkout `cordis-rust`:

```text
workspace/
  cordis-rust/
  my-app/
    Cargo.toml
    src/main.rs
```

Khai báo manifest của ứng dụng như sau:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"

[dependencies]
cordis-core = { path = "../cordis-rust/crates/cordis-core" }
tokio = { version = "1.48", default-features = false, features = ["rt", "sync"] }
```

Điều chỉnh đường dẫn theo checkout của bạn. Tên import là `cordis_core`, dùng
dấu gạch dưới. Nếu ứng dụng dùng timer, I/O, signal hoặc macro của Tokio, hãy
bật các feature tương ứng trong phụ thuộc Tokio của **ứng dụng**; bộ feature
tối thiểu của core không tự bật chúng.

## Ứng dụng đầu tiên hoàn chỉnh

Đặt mã sau vào `src/main.rs`, rồi chạy `cargo run` trong `my-app`.

```rust
use cordis_core::{BoxError, Disposer, FiberState, Plugin, Runtime};
use std::convert::Infallible;

fn main() -> Result<(), BoxError> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    executor.block_on(async {
        let runtime = Runtime::new();
        runtime.run(async {
            let root = runtime.context();
            let result = async {
                let worker = Plugin::initialized("Worker", |ctx, name: String| async move {
                    ctx.logger(None).info(format!("hello, {name}"))?;
                    Ok::<_, BoxError>(Disposer::sync(|| Ok::<_, Infallible>(())))
                });
                let fiber = root.plugin(&worker, String::from("Cordis"))?;
                assert_eq!(fiber.ready()?.await?, FiberState::Active);
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

Logger ghi lời chào vào bộ đệm tích hợp; mặc định không in thông điệp ứng dụng
ra terminal. Xem [ghi log](guides/logging.md) để thêm exporter.

## Diễn biến thực thi

`Runtime` sở hữu bộ chủ trì tác vụ cục bộ. Tokio cung cấp bộ thực thi;
`Runtime::run` điều khiển các tác vụ đó trong lúc future được truyền vào còn
chạy. Context gốc là góc nhìn nhẹ, không sở hữu runtime, không phải runtime thứ hai.

`Plugin::initialized` định nghĩa thành phần tái sử dụng có bước khởi tạo trả về
`Disposer`. `root.plugin` tiếp nhận một Fiber ngay và trả về handle.
`ready()?.await?` quan sát kết quả lượt vòng đời hiện tại của Fiber đó.

Ví dụ luôn thử dọn tài nguyên gốc sau cả thành công lẫn lỗi khởi tạo thông
thường. `drain` trả về báo cáo: đến mốc hoàn tất không có nghĩa mọi callback đều
thành công. Nếu công việc và dọn dẹp cùng lỗi, ví dụ trả về lỗi dọn dẹp;
chẩn đoán của core vẫn giữ lỗi callback.

Kết thúc `run` không tự drain. Hủy `Runtime` sẽ hủy các tác vụ còn lại.
Bố trí dọn dẹp tường minh cho mọi nhánh thoát mà ứng dụng kiểm soát; panic,
kết thúc tiến trình và hủy host không phải các nhánh dừng thành công.

## Đọc tiếp

Đọc [khái niệm](concepts.md), sau đó [plugin và cập nhật](guides/plugins-and-updates.md).
Để xem cây provider/consumer hoàn chỉnh, chạy ví dụ sẵn có của repository:

```sh
cargo run -p cordis-core --example lifecycle --locked
```

Chạy lệnh của repository tại thư mục gốc `cordis-rust`, không phải trong `my-app`.
