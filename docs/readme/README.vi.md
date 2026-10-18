# San USB Viewer

**Tiếng Việt** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Dịch máy; sự điều chỉnh đều được chào đón.

**Phiên bản hiện tại: `0.2.0`** — Giấy phép MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

USB Explorer gốc được viết bằng **Rust**, với giao diện tối, cấu trúc liên kết trực quan và chẩn đoán có thể đọc được. Phiên bản đầu tiên dành cho Windows 10/11.

## Nguồn gốc của dự án

Video tại nguồn gốc của dự án: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Dự án được lấy cảm hứng từ việc sử dụng **WhatCable** trên Apple và **USB Device Tree Viewer** trên Windows: tìm hiểu các thiết bị ngoại vi, hub và tốc độ kết nối USB của chúng. Đây là một thực hiện độc lập.

## Đặc trưng

- Phát hiện thực tế các thiết bị và bộ điều khiển USB thông qua `nusb`.
- Cấu trúc cây theo đường dẫn xe buýt và cổng, với độ sâu trung tâm.
- Tìm kiếm theo tên, số sê-ri hoặc VID:PID; Bộ lọc và hub USB 3+.
- Tốc độ liên kết thỏa thuận: 1,5/12/480 Mb/s, 5 Gb/s,
hoặc danh mục SuperSpeedPlus (10 Gb/s trở lên).
- Sơ đồ hành trình: máy tính, các hub trung gian, liên kết và thiết bị.
- Trong Windows, việc thẩm vấn bổ sung trung tâm của IOCTL để tìm ra
Khả năng của SuperSpeed được thiết bị công bố. Thiết bị tương thích SuperSpeed được kết nối qua USB 2.0 sẽ tạo ra chẩn đoán giới hạn đường dẫn.   Các trung tâm đi kèm với trung tâm USB 2 của trung tâm USB 3 được xử lý riêng biệt để tránh khiến nhánh 480 Mb/s của chúng bị lỗi cáp.
- Mã định danh VID/PID, bộ mô tả USB, bản sửa đổi, trình điều khiển, nối tiếp và giao diện,
khi hệ thống tiết lộ thông tin này.
- Tự động cập nhật ba giây một lần hoặc thủ công với F5.
- Nhật ký phiên của 200 thay đổi được quan sát gần đây nhất.
- Xuất câu lệnh hoàn chỉnh dưới dạng JSON (Ctrl+E) và sao chép các chi tiết đã chọn.
- Hướng dẫn tích hợp và chế độ trình diễn rõ ràng, không bao giờ được sử dụng làm bản sao lưu
im lặng trong trường hợp phát hiện thất bại.

## Ném

Với Rust **1.95+**, chuỗi công cụ MSVC và các công cụ Visual Studio C++ trên Windows. Các ví dụ sử dụng RTK để lọc đầu ra của lệnh; không có RTK, chỉ cần xóa tiền tố `rtk`. Tập lệnh phân phối chấp nhận cả hai trường hợp.

```powershell
rtk cargo run --release
```

Hệ nhị phân độc lập được tạo ra trong `target/release/san-usb-viewer.exe`. Tập lệnh `build.ps1` cũng tạo phân phối trong `dist/SanUsbViewer/`. Không cần sửa đổi trình điều khiển hoặc cài đặt WinUSB cho kho: ứng dụng không yêu cầu giao diện thiết bị.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Thư mục đích để chụp phải tồn tại. Phiên bản phát hành là một ứng dụng Windows không có bảng điều khiển; thích `cargo run` hơn trong quá trình phát triển cho kho lưu trữ dòng lệnh.

## Giải thích kết quả

**Tốc độ thương lượng** là tốc độ của kết nối chứ không phải là điểm chuẩn của tốc độ sao chép. Tốc độ hữu ích đặc biệt phụ thuộc vào giao thức, thiết bị ngoại vi và các thiết bị khác chia sẻ bus. Tốc độ từ nhiều thiết bị không cộng lại để tính toán băng thông sẵn có.

Cáp USB thụ động thường không phải là một thiết bị có thể đếm được. Phiên bản này không đọc được điểm đánh dấu điện tử, nguồn điện USB Power Delivery cũng như loại đầu nối của nó. Do đó, nó không yêu cầu chứng nhận tốc độ tối đa của nó. Chẩn đoán dựa trên liên kết được quan sát và, nếu có, dựa trên các chỉ báo của Windows. Liên kết chậm có thể bị giới hạn bởi cáp, cổng, hub hoặc thiết bị. Số phiên bản mô tả USB không bao giờ được sử dụng làm tốc độ tối đa.

Danh mục SuperSpeedPlus không phân biệt giữa chế độ 10 và 20 Gb/s ở đây. USB4/Thunderbolt và đường hầm của chúng không được kiểm tra. Các cổng không có thiết bị kèm theo sẽ không được kiểm kê. Cáp sạc không có dữ liệu sẽ không tạo ra một thiết bị hiển thị. Các thiết bị bên trong (Bluetooth, camera tích hợp, v.v.) có thể xuất hiện trong cây USB.

Thông tin không thể truy cập được hiển thị là không có sẵn. Nhà sản xuất và giao diện không phải lúc nào cũng được Windows hiển thị. Các sự kiện xảy ra hoàn toàn giữa các lần quét có thể không được chú ý. Nhật ký nằm trong bộ nhớ và bắt đầu sau lần kiểm kê thành công đầu tiên.

Mã định danh bus và đường dẫn cổng mô tả cấu trúc liên kết hệ thống. Đầu nối vật lý có thể có hai đường dẫn riêng cho USB 2 và SuperSpeed.

## Kiến trúc và xác nhận

`scanner.rs` thực hiện kiểm kê; `windows_probe.rs` đóng gói các yêu cầu đọc gốc từ các trung tâm; `model.rs` chứa dữ liệu và chẩn đoán. `app.rs` xử lý các tác vụ và lựa chọn nền; `views.rs`, `inspector.rs`, `connection.rs` và `theme.rs` vẽ giao diện bằng egui/eframe. Giao diện vẫn tương tác trong quá trình quét và giữ lại lần đọc thành công cuối cùng nếu lần quét tiếp theo không thành công.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Các thư viện đa nền tảng và mã Windows có điều kiện. Các phiên bản macOS/Linux không được xác thực trong lần phân phối đầu tiên này.

## Bảo mật

Khoảng không quảng cáo hoạt động cục bộ, chỉ đọc, không cần đo từ xa hoặc dịch vụ mạng. Các liên kết hướng dẫn chỉ mở khi nhấp chuột. Báo cáo JSON có thể chứa số sê-ri và số nhận dạng hệ thống: việc chia sẻ chúng là tùy thuộc vào bạn.

## Tài liệu tham khảo kỹ thuật

- [nusb: liệt kê và thông tin USB](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

Giấy phép MIT. Các công trình phụ vẫn phải tuân theo giấy phép tương ứng của họ.

Kho lưu trữ công cộng chỉ chứa các nguồn, bản dịch và phông chữ miễn phí cần thiết cho việc biên soạn. Việc đọc USB, chụp chẩn đoán, xuất, tùy chọn cục bộ, bộ đệm và tệp nhị phân phát triển bị loại trừ bởi `.gitignore`. Để báo cáo sự cố, hãy sử dụng chế độ `--demo` hoặc xóa số sê-ri và ID hệ thống khỏi báo cáo của bạn trước khi chia sẻ chúng.

## Ngôn ngữ — phiên bản 0.2.0

Ứng dụng này bao gồm **50 ngôn ngữ giao diện**. Nhấp vào mã ngôn ngữ (`FR`, `EN`, v.v.) ở trên cùng bên phải để mở trình chuyển đổi. Bạn có thể tìm kiếm một ngôn ngữ theo tên bản địa, tên tiếng Anh hoặc mã của ngôn ngữ đó. Sự thay đổi diễn ra ngay lập tức và lựa chọn được lưu trữ trong `%APPDATA%/SanUsbViewer/preferences.json` trong Windows. Ngôn ngữ hệ thống sẽ được sử dụng mà không cần ưu tiên; Một ngôn ngữ không được hỗ trợ sẽ trở lại tiếng Anh.

Việc lựa chọn ưu tiên các ngôn ngữ được nói nhiều nhất (tiếng mẹ đẻ và ngôn ngữ thứ hai), nhóm các biến thể khu vực lại với nhau. Đây không phải là cách phân loại bằng số: kết quả khác nhau tùy thuộc vào nhóm phương ngữ. Các biến thể được lựa chọn bao gồm tiếng Trung giản thể, tiếng Ả Rập chuẩn và chữ viết Latinh của Fulani và tiếng Uzbek; Tiếng Punjab sử dụng Gurmukhi.

Tiếng Anh, tiếng Trung, tiếng Hindi, tiếng Tây Ban Nha, tiếng Pháp, tiếng Ả Rập, tiếng Bengal, tiếng Bồ Đào Nha, tiếng Nga, tiếng Urdu, tiếng Indonesia, tiếng Đức, tiếng Nhật, tiếng Marathi, tiếng Việt, tiếng Telugu, tiếng Hausa, tiếng Thổ Nhĩ Kỳ, tiếng Ba Tư, tiếng Swahili, tiếng Philippin, tiếng Tamil, tiếng Ba Tư, tiếng Hàn, tiếng Amharic, tiếng Thái, tiếng Java, tiếng Ý, tiếng Gujarati, tiếng Kannada, tiếng Bhojpuri, tiếng Miến Điện, tiếng Ba Lan, tiếng Yoruba, Lingala, Oromo, Fulani, Malayalam, Odia, tiếng Ukraina, Sindhi, tiếng Uzbek, Tiếng Pashto, tiếng Sundan, tiếng Mã Lai, tiếng Igbo, tiếng Nepal, tiếng Azeri, Maithili và tiếng Somali.

Các menu sao chép, chẩn đoán, văn bản trợ giúp, thông báo hoạt động và chi tiết đều được bản địa hóa. Tên và số nhận dạng do phần cứng và hệ thống cung cấp sẽ được giữ lại. Quá trình xuất JSON giữ lại lược đồ và dữ liệu kiểm kê của nó, bất kể ngôn ngữ hiển thị. Đơn vị kỹ thuật vẫn là **bit/s** (Mbit/s và Gbit/s) để tránh nhầm lẫn với byte.

Các danh mục tiếng Pháp và tiếng Anh dùng làm tài liệu tham khảo. Các ngôn ngữ khác là **bản dịch tự động**, có thể yêu cầu chỉnh sửa ngôn ngữ. Các tệp `locales/*.json` được bao gồm trong quá trình biên dịch và có thể được sửa rồi biên dịch lại. Ứng dụng không thực hiện dịch mạng: 50 ngôn ngữ và phông chữ Noto được nhúng theo Giấy phép Phông chữ Mở SIL. Văn bản tiếng Ả Rập sử dụng các phân đoạn Unicode hai chiều và dòng mới được tính toán trước khi định dạng.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` có thể tham chiếu đến tệp tùy chọn khác để thử nghiệm hoặc cài đặt di động. Chỉ có mã ngôn ngữ được lưu ở đó.
