# San USB Viewer

**Bahasa Melayu** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Terjemahan mesin; pembetulan dialu-alukan.

**Versi semasa: `0.2.0`** — lesen MIT.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Penjelajah USB asli yang ditulis dalam **Rust**, dengan antara muka gelap, topologi visual dan diagnostik yang boleh dibaca. Versi pertama ditujukan untuk Windows 10/11.

## Asal projek

Video pada asal projek: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Projek ini diilhamkan oleh penggunaan **WhatCable** pada Apple dan **USB Device Tree Viewer** pada Windows: memahami persisian, hab dan kelajuan sambungan USB mereka. Ini adalah pelaksanaan bebas.

## Ciri-ciri

- Pengesanan sebenar peranti dan pengawal USB melalui `nusb`.
- Struktur pokok melalui laluan bas dan pelabuhan, dengan kedalaman hab.
- Cari mengikut nama, nombor siri atau VID:PID; Penapis dan hab USB 3+.
- Kelajuan pautan yang dirundingkan: 1.5 / 12 / 480 Mb/s, 5 Gb/s,
atau kategori SuperSpeedPlus (10 Gb/s atau lebih).
- Gambar rajah perjalanan: komputer, hab perantaraan, pautan dan peranti.
- Di bawah Windows, soal siasat tambahan hab oleh IOCTL untuk mengetahui
Keupayaan SuperSpeed diumumkan oleh peranti. Peranti serasi SuperSpeed yang disambungkan melalui USB 2.0 menghasilkan diagnostik pengehadan laluan.   Rakan hab USB 2 hab USB 3 dirawat secara berasingan untuk mengelakkan mengaitkan cawangan 480 Mb/s mereka kepada kerosakan kabel.
- Pengecam VID/PID, deskriptor USB, semakan, pemacu, siri dan antara muka,
apabila sistem mendedahkan maklumat ini.
- Kemas kini automatik setiap tiga saat, atau manual dengan F5.
- Log sesi 200 perubahan terakhir yang diperhatikan.
- Eksport pernyataan lengkap dalam JSON (Ctrl+E) dan salin butiran yang dipilih.
- Panduan bersepadu dan mod demonstrasi eksplisit, tidak pernah digunakan sebagai sandaran
senyap sekiranya berlaku kegagalan pengesanan.

## Baling

Dengan Rust **1.95+**, rantai alat MSVC dan alatan Visual Studio C++ pada Windows. Contoh menggunakan RTK untuk menapis output arahan; tanpa RTK, alih keluar awalan `rtk`. Skrip pengedaran menerima kedua-dua kes.

```powershell
rtk cargo run --release
```

Perduaan kendiri dihasilkan dalam `target/release/san-usb-viewer.exe`. Skrip `build.ps1` juga mencipta pengedaran dalam `dist/`. Tiada pengubahsuaian pemacu atau pemasangan WinUSB diperlukan untuk inventori: aplikasi tidak memerlukan antara muka peranti.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Folder destinasi untuk tangkapan mesti wujud. Versi keluaran ialah aplikasi Windows tanpa konsol; lebih suka `cargo run` dalam pembangunan untuk inventori baris arahan.

## Mentafsir keputusan

**Kelajuan yang dirundingkan** ialah sambungan, dan bukan penanda aras kelajuan salinan. Kelajuan berguna bergantung khususnya pada protokol, peranti dan peranti lain yang berkongsi bas. Kelajuan daripada berbilang peranti tidak ditambah untuk mengira lebar jalur yang tersedia.

Kabel USB pasif biasanya bukan peranti yang boleh dikira. Versi ini tidak membaca penanda e-nya, mahupun kuasa Penghantaran Kuasa USB, mahupun jenis penyambungnya. Oleh itu, ia tidak mendakwa untuk mengesahkan kelajuan maksimumnya. Diagnosis adalah berdasarkan pautan yang diperhatikan dan, jika tersedia, pada penunjuk Windows. Pautan perlahan mungkin dihadkan oleh kabel, port, hab atau peranti. Nombor versi deskriptor USB tidak pernah digunakan sebagai kelajuan maksimum.

Kategori SuperSpeedPlus tidak membezakan antara mod 10 dan 20 Gb/s di sini. USB4/Thunderbolt dan terowongnya tidak diperiksa. Port tanpa peranti yang dilampirkan tidak diinventori. Kabel pengecasan tanpa data tidak mencipta peranti yang boleh dilihat. Peranti dalaman (Bluetooth, kamera terbina dalam, dll.) mungkin muncul dalam pepohon USB.

Maklumat yang tidak boleh diakses dipaparkan sebagai tidak tersedia. Pengilang dan antara muka tidak selalu didedahkan oleh Windows. Peristiwa yang berlaku sepenuhnya antara imbasan mungkin tidak disedari. Log berada dalam ingatan dan bermula selepas inventori pertama yang berjaya.

Pengecam bas dan laluan port menerangkan topologi sistem. Penyambung fizikal boleh mempunyai dua laluan berasingan untuk USB 2 dan SuperSpeed.

## Seni bina dan pengesahan

`scanner.rs` melaksanakan inventori; `windows_probe.rs` merangkum permintaan baca asli daripada hab; `model.rs` mengandungi data dan diagnostik. `app.rs` mengendalikan tugasan latar belakang dan pemilihan; `views.rs`, `inspector.rs`, `connection.rs` dan `theme.rs` melukis antara muka dengan egui/eframe. Antara muka kekal interaktif semasa imbasan dan mengekalkan bacaan terakhir yang berjaya jika imbasan seterusnya gagal.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Perpustakaan adalah platform silang dan kod Windows adalah bersyarat. Versi macOS/Linux tidak disahkan dalam penghantaran pertama ini.

## Kerahsiaan

Inventori berfungsi secara tempatan, baca sahaja, tanpa telemetri atau perkhidmatan rangkaian. Pautan panduan hanya dibuka pada klik. Laporan JSON boleh mengandungi nombor siri dan pengecam sistem: berkongsinya terpulang kepada anda.

## Rujukan teknikal

- [nusb: penghitungan dan maklumat USB](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

lesen MIT. Bangunan luar kekal tertakluk kepada lesen masing-masing.

Repositori awam hanya mengandungi sumber, terjemahan dan fon percuma yang diperlukan untuk penyusunan. Bacaan USB, tangkapan diagnostik, eksport, keutamaan tempatan, cache dan binari pembangunan dikecualikan oleh `.gitignore`. Untuk melaporkan masalah, gunakan mod `--demo` atau alih keluar nombor siri dan ID sistem daripada laporan anda sebelum berkongsinya.

## Bahasa — versi 0.2.0

Aplikasi ini termasuk **50 bahasa antara muka**. Klik kod bahasa (`FR`, `EN`, dll.) di bahagian atas sebelah kanan untuk membuka penukar. Anda boleh mencari bahasa mengikut nama asalnya, nama Inggerisnya atau kodnya. Perubahan adalah serta-merta dan pilihan disimpan dalam `%APPDATA%/SanUsbViewer/preferences.json` di bawah Windows. Tanpa keutamaan, bahasa sistem digunakan; Bahasa yang tidak disokong kembali kepada bahasa Inggeris.

Pemilihan ini mengutamakan bahasa yang paling banyak dituturkan secara keseluruhan (bahasa ibunda dan bahasa kedua), mengumpulkan varian serantau. Ini bukan klasifikasi berangka: keputusan berbeza bergantung pada pengelompokan dialek. Varian yang dipilih termasuk Bahasa Cina Ringkas, Bahasa Arab Standard dan skrip Latin Fulani dan Uzbekistan; Punjabi menggunakan Gurmukhi.

Inggeris, Cina, Hindi, Sepanyol, Perancis, Arab, Benggali, Portugis, Rusia, Urdu, Indonesia, Jerman, Jepun, Marathi, Vietnam, Telugu, Hausa, Turki, Punjabi, Swahili, Filipina, Tamil, Parsi, Korea, Amharik, Thai, Jawa, Itali, Gujerat, Kannada, Bhojpuri, Burma, Poland, Yoruba, Linglaniala, Ukrainian, Ukrainian, Fukushima, Ukrainian, Ukrainian, Fukudia Pashto, Sunda, Melayu, Igbo, Nepal, Azeri, Maithili dan Somalia.

Menu yang disalin, diagnostik, teks bantuan, mesej aktiviti dan butiran disetempatkan. Nama dan pengecam yang disediakan oleh perkakasan dan sistem dikekalkan. Eksport JSON menyimpan data skema dan inventorinya, tanpa mengira bahasa paparan. Unit teknikal kekal dalam **bit/s** (Mbit/s dan Gbit/s) untuk mengelakkan kekeliruan dengan bait.

Katalog Perancis dan Inggeris berfungsi sebagai rujukan. Bahasa lain ialah **terjemahan automatik**, yang mungkin memerlukan pembetulan linguistik. Fail `locales/*.json` disertakan dalam kompilasi dan boleh dibetulkan dan kemudian disusun semula. Tiada terjemahan rangkaian dijalankan oleh aplikasi: 50 bahasa dan fon Noto dibenamkan, di bawah Lesen Font Terbuka SIL. Teks Arab menggunakan segmen Unicode dwiarah dan baris baharu yang dikira sebelum memformat.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` boleh merujuk kepada fail keutamaan lain untuk ujian atau pemasangan mudah alih. Hanya kod bahasa yang disimpan di sana.
