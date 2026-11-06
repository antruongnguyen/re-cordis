# Các kênh tài liệu

[English](../../en/guides/documentation.md) | [Trang tiếng Việt](../README.md)

Dùng hướng dẫn để hiểu hợp đồng ứng dụng và Rustdoc để tra chữ ký chính xác,
ràng buộc generic, trait implementation và trường công khai. Hai nguồn phải
mô tả cùng revision mã nguồn và cấu hình feature.

Cả bảy package thử nghiệm `0.1.0-alpha.1` đã phát hành trên crates.io;
Rustdoc docs.rs theo phiên bản chính xác đã được kiểm chứng. Mã nguồn phát hành
và checkout tương thích của Create là `84fa9bdf0eb4e631fce792bf163494b0a61aeba8`.
Site tài liệu phát triển đã công khai trên GitHub Pages, với
hướng dẫn [tiếng Anh](https://antruongnguyen.github.io/re-cordis/en/docs/) và
[tiếng Việt](https://antruongnguyen.github.io/re-cordis/vi/docs/) cùng Rustdoc
cùng revision. Revision bản dựng đã triển khai có thể khác mã nguồn phát hành
đã chốt. Dùng hướng dẫn registry trong [bắt đầu](../getting-started.md)
hoặc CLI đã cài vẫn yêu cầu checkout tường minh trong [Create](create.md).

## Hướng dẫn phát triển

`docs/en/` và `docs/vi/` là nguồn Markdown chính. Website dựng trực tiếp từ
các tệp này, giữ các trang tương ứng và ví dụ thực thi giống nhau.
Kế hoạch kỹ thuật và hồ sơ thiết kế nội bộ chỉ nằm trong checkout. Trang công
khai, tìm kiếm và Markdown tải xuống bỏ liên kết đến các hồ sơ đó; package crate
và Rustdoc không nhập chúng. Website dùng khung tài liệu toàn chiều rộng,
sidebar cố định và mục lục trên desktop.

Hướng dẫn phát triển có thể thay đổi mà không đổi phiên bản package. Site công
khai đi theo revision đã triển khai, được ghi trong
[provenance bản dựng](https://antruongnguyen.github.io/re-cordis/build-provenance.json).
Kiểm tra revision đó trước khi so sánh hướng dẫn đã lưu với mã checkout mới.

Site tĩnh của dự án dùng `/re-cordis/en/docs/` và
`/re-cordis/vi/docs/`. Mỗi trang hiển thị liên kết mã nguồn và Markdown.
Mục lục `/re-cordis/llms.txt` và các tệp `/re-cordis/markdown/` được export dùng
đường dẫn site. [Mục lục AI trong checkout](../../llms.txt) riêng dùng đường dẫn
tương đối từ `docs/` và cần checkout để giải các đường dẫn đó.

Xem [hướng dẫn website](../../../website/README.md) để dựng,
xem trước và cập nhật GitHub Pages. CI Rust và tài liệu kiểm tra pull request
và push vào main; lần triển khai Pages thủ công đầu tiên đã thành công.
Chuẩn bị hay xem trước bản export không triển khai site. Những lần cập nhật
tài liệu tiếp theo cần triển khai thủ công riêng từ revision main đã review.

## Rustdoc chính xác từ checkout

Tại thư mục gốc repository, giải công cụ Rust stable qua rustup và sinh
tài liệu cho cả bảy package và dependency của chúng, gồm adapter Include tùy chọn
của HMR:

```sh
CORDIS_DOC_TOOLCHAIN=stable
CORDIS_DOC_CARGO="$(rustup which --toolchain "$CORDIS_DOC_TOOLCHAIN" cargo)"
CORDIS_DOC_RUSTC="$(rustup which --toolchain "$CORDIS_DOC_TOOLCHAIN" rustc)"
CORDIS_DOC_RUSTDOC="$(rustup which --toolchain "$CORDIS_DOC_TOOLCHAIN" rustdoc)"
CARGO_TARGET_DIR=target/site-rustdoc \
RUSTUP_TOOLCHAIN="$CORDIS_DOC_TOOLCHAIN" \
RUSTC="$CORDIS_DOC_RUSTC" RUSTDOC="$CORDIS_DOC_RUSTDOC" \
PATH="$(dirname "$CORDIS_DOC_CARGO"):$PATH" \
"$CORDIS_DOC_CARGO" doc --workspace --all-features --locked
```

Mở `target/site-rustdoc/doc/re_cordis_core/index.html`; thư mục các crate khác
cũng dùng tên với dấu gạch dưới. Thay `stable` bằng `1.85.0` để sinh tài liệu
với compiler tối thiểu được hỗ trợ. Phải có các dependency; chỉ thêm `--offline`
sau khi khôi phục dependency theo lockfile ở máy cục bộ. `cargo doc` sinh tệp
và không phát hành gì.

Kèm tài liệu dependency trong bundle site này để liên kết Rustdoc đến trait
và chữ ký của dependency có đích cục bộ. Bản dựng tài liệu này không thêm bằng
chứng nghiệm thu runtime cho các dependency hay nền tảng đó.

Website nhận thư mục Rustdoc này qua `CORDIS_RUSTDOC_DIR`.
Các [trang API](../api/README.md) liên kết đến crate tương ứng tại
`/re-cordis/rustdoc/{build_revision}/{crate}/index.html`, trong đó `build_revision`
là Git revision đầy đủ được ghi lại và `crate` là tên thư mục dùng dấu gạch dưới
như `re_cordis_hmr`. Dựng hướng dẫn và Rustdoc từ cùng checkout đã commit;
không dùng lại Rustdoc được sinh từ revision khác.

Bản dựng tài liệu đầy đủ bật feature `include` của HMR. Ứng dụng vẫn phải
bật feature này rõ ràng để dùng `watch_include` và `replace_include`;
feature mặc định của HMR vẫn rỗng. Xem [hợp đồng API HMR](../api/hmr.md)
cho các phương thức phụ thuộc feature.

## Tài liệu bản phát hành

[docs.rs](https://docs.rs/about/builds) sinh Rustdoc từ package phát hành
trên crates.io. Cả bảy trang chính xác `0.1.0-alpha.1` đã được kiểm chứng;
dùng [liên kết package](../api/README.md#rustdoc-đã-phát-hành) để tra chữ ký đã phát hành.
[Metadata docs.rs](https://docs.rs/about/metadata) của HMR hiển thị `watch_include`
và `replace_include`, không đổi feature mặc định rỗng của ứng dụng.
Tài liệu được host không mở rộng phạm vi runtime Apple Silicon macOS đã kiểm thử.

[Snapshot mã nguồn đã chốt](https://github.com/antruongnguyen/re-cordis/tree/84fa9bdf0eb4e631fce792bf163494b0a61aeba8)
xác định bộ package đã phát hành và checkout tương thích của Create. Hướng dẫn
và README package trong snapshot vẫn giữ nội dung chuẩn bị ứng viên trước khi
tải lên; các câu đó mô tả trạng thái lịch sử. Hướng dẫn main hiện tại bổ sung
cài đặt registry và tình trạng sẵn có sau mã nguồn đã chốt, không đổi package
đã phát hành. Dùng phiên bản docs.rs chính xác thay vì `latest` khi tra API
của ứng dụng cũ.

Site Fumadocs là artifact riêng, gồm hướng dẫn và Rustdoc cùng revision bản
dựng được ghi lại. Commit tài liệu, dựng website hay tải package lên registry
không triển khai Pages. Đọc provenance công khai để xác định snapshot đang
hoạt động; các lần cập nhật tiếp theo giữ cổng triển khai thủ công.
