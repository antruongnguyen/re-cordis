# Các kênh tài liệu

[English](../../en/guides/documentation.md) | [Trang tiếng Việt](../README.md)

Dùng hướng dẫn để hiểu hợp đồng ứng dụng và Rustdoc để tra chữ ký chính xác,
ràng buộc generic, trait implementation và trường công khai. Hai nguồn phải
mô tả cùng revision mã nguồn và cấu hình feature.

Cả bảy crate là bản ứng viên thử nghiệm `0.1.0-alpha.1` chưa phát hành.
Việc chuẩn bị đã được duyệt; phát hành registry vẫn cần cổng riêng.
Site tài liệu phát triển đã công khai trên GitHub Pages, với
hướng dẫn [tiếng Anh](https://antruongnguyen.github.io/re-cordis/en/docs/) và
[tiếng Việt](https://antruongnguyen.github.io/re-cordis/vi/docs/) cùng Rustdoc
cùng revision. Đây không phải bản docs.rs đã phát hành. Tiếp tục
dùng hướng dẫn checkout cục bộ trong [bắt đầu](../getting-started.md)
và [Create](create.md).

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

## Tài liệu phát hành trong tương lai

[docs.rs](https://docs.rs/about/builds) tự động sinh Rustdoc
từ package được phát hành trên crates.io. Site Fumadocs là artifact riêng;
không được tải lên docs.rs. [Bản ứng viên alpha đồng bộ](../../release-readiness.md)
đã được duyệt để chuẩn bị; chính sách manifest chỉ cho crates.io không cho phép tải lên.
Hiện chưa có bản docs.rs của re-cordis để liên kết.

Trước khi phát hành, chuẩn bị snapshot hướng dẫn bất biến từ cùng revision
mã nguồn với bộ package. Chỉ liên kết snapshot đến phiên bản phát hành chính
xác của từng package trên docs.rs sau khi bản dựng được host thành công.
Giữ đường dẫn quay lại tài liệu phát triển và gắn nhãn rõ cả hai kênh.
Trang phát triển thay đổi liên tục hay URL docs.rs `latest` không xác định
đáng tin cậy API của ứng dụng cũ.

[Metadata docs.rs](https://docs.rs/about/metadata) đã chuẩn bị cho HMR chọn
`include` để hiển thị API trong tài liệu, không đổi mặc định ứng dụng.
Kiểm tra Rustdoc stable/MSRV cục bộ xác lập tính tương thích tài liệu cục bộ.
Chúng không xác lập bản dựng docs.rs được host hay thành công trong sandbox
Linux/nightly. Tương tự, dựng website hay biên dịch tài liệu đa nền tảng không
mở rộng bằng chứng runtime filesystem/watcher hiện chỉ có trên macOS.
