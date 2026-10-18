# San USB Viewer

**Bahasa Indonesia** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Terjemahan mesin; koreksi dipersilakan.

**Versi saat ini: `0.2.0`** — lisensi MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

USB Explorer asli ditulis dalam **Rust**, dengan antarmuka gelap, topologi visual, dan diagnostik yang mudah dibaca. Versi pertama ditujukan untuk Windows 10/11.

## Asal usul proyek

Video asal proyek: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Proyek ini terinspirasi oleh penggunaan **WhatCable** di Apple dan **USB Device Tree Viewer** di Windows: memahami periferal, hub, dan kecepatan koneksi USB mereka. Ini adalah implementasi independen.

## Fitur

- Deteksi aktual perangkat dan pengontrol USB melalui `nusb`.
- Struktur pohon berdasarkan jalur bus dan pelabuhan, dengan kedalaman hub.
- Cari berdasarkan nama, nomor seri atau VID:PID; Filter dan hub USB 3+.
- Kecepatan tautan yang dinegosiasikan: 1,5 / 12 / 480 Mb/s, 5 Gb/s,
atau kategori SuperSpeedPlus (10 Gb/s atau lebih).
- Diagram perjalanan: komputer, hub perantara, tautan, dan perangkat.
- Di bawah Windows, interogasi tambahan terhadap hub oleh IOCTL untuk mengetahui
Kemampuan SuperSpeed diumumkan oleh perangkat. Perangkat kompatibel SuperSpeed yang terhubung melalui USB 2.0 menghasilkan diagnostik batasan jalur.   Hub USB 2 yang menyertai hub USB 3 diperlakukan secara terpisah untuk menghindari menghubungkan cabang 480 Mb/snya dengan kesalahan kabel.
- Pengidentifikasi VID/PID, deskriptor USB, revisi, driver, serial dan antarmuka,
ketika sistem memaparkan informasi ini.
- Pembaruan otomatis setiap tiga detik, atau manual dengan F5.
- Log sesi dari 200 perubahan terakhir yang diamati.
- Ekspor pernyataan lengkap dalam JSON (Ctrl+E) dan salin detail yang dipilih.
- Panduan terintegrasi dan mode demonstrasi eksplisit, tidak pernah digunakan sebagai cadangan
diam jika terjadi kegagalan deteksi.

## Melemparkan

Dengan Rust **1.95+**, rantai alat MSVC dan alat Visual Studio C++ di Windows. Contohnya menggunakan RTK untuk memfilter keluaran perintah; tanpa RTK, hapus saja awalan `rtk`. Skrip distribusi menerima kedua kasus tersebut.

```powershell
rtk cargo run --release
```

Biner mandiri diproduksi di `target/release/san-usb-viewer.exe`. Skrip `build.ps1` juga membuat distribusi di `dist/SanUsbViewer/`. Tidak diperlukan modifikasi driver atau instalasi WinUSB untuk inventaris: aplikasi tidak memerlukan antarmuka perangkat.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Folder tujuan pengambilan harus ada. Versi rilisnya adalah aplikasi Windows tanpa konsol; lebih memilih `cargo run` dalam pengembangan untuk inventaris baris perintah.

## Interpretasikan hasilnya

**Kecepatan yang dinegosiasikan** adalah kecepatan koneksi, dan bukan tolok ukur kecepatan penyalinan. Kecepatan yang berguna khususnya bergantung pada protokol, periferal, dan perangkat lain yang berbagi bus. Kecepatan dari beberapa perangkat tidak dihitung untuk menghitung bandwidth yang tersedia.

Kabel USB pasif umumnya bukan perangkat yang dapat dihitung. Versi ini tidak membaca penanda elektroniknya, daya Pengiriman Daya USB, maupun jenis konektornya. Oleh karena itu, ia tidak mengklaim untuk mengesahkan kecepatan maksimumnya. Diagnosis didasarkan pada tautan yang diamati dan, jika tersedia, pada indikator Windows. Tautan lambat mungkin dibatasi oleh kabel, port, hub, atau perangkat. Nomor versi deskriptor USB tidak pernah digunakan sebagai kecepatan maksimum.

Kategori SuperSpeedPlus tidak membedakan antara mode 10 dan 20 Gb/s di sini. USB4/Thunderbolt dan terowongannya tidak diperiksa. Port tanpa perangkat terpasang tidak diinventarisasi. Kabel pengisi daya tanpa data tidak membuat perangkat terlihat. Perangkat internal (Bluetooth, kamera internal, dll.) mungkin muncul di pohon USB.

Informasi yang tidak dapat diakses ditampilkan sebagai tidak tersedia. Pabrikan dan antarmuka tidak selalu diekspos oleh Windows. Peristiwa yang terjadi sepenuhnya di antara pemindaian mungkin luput dari perhatian. Log ada di memori dan dimulai setelah inventarisasi pertama berhasil.

Pengidentifikasi bus dan jalur port menggambarkan topologi sistem. Konektor fisik dapat memiliki dua jalur terpisah untuk USB 2 dan SuperSpeed.

## Arsitektur dan validasi

`scanner.rs` melakukan inventarisasi; `windows_probe.rs` merangkum permintaan baca asli dari hub; `model.rs` berisi data dan diagnostik. `app.rs` menangani tugas dan seleksi latar belakang; `views.rs`, `inspector.rs`, `connection.rs` dan `theme.rs` menggambar antarmuka dengan egui/eframe. Antarmuka tetap interaktif selama pemindaian dan mempertahankan pembacaan terakhir yang berhasil jika pemindaian berikutnya gagal.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Perpustakaan bersifat lintas platform dan kode Windows bersifat kondisional. Versi macOS/Linux tidak divalidasi pada pengiriman pertama ini.

## Kerahasiaan

Inventaris berfungsi secara lokal, hanya baca, tanpa telemetri atau layanan jaringan. Tautan panduan hanya terbuka saat diklik. Laporan JSON dapat berisi nomor seri dan pengidentifikasi sistem: Anda bebas membagikannya.

## Referensi teknis

- [nusb: Pencacahan dan informasi USB](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

lisensi MIT. Bangunan luar tetap tunduk pada izin masing-masing.

Repositori publik hanya berisi sumber, terjemahan, dan font gratis yang diperlukan untuk kompilasi. Pembacaan USB, pengambilan diagnostik, ekspor, preferensi lokal, cache, dan biner pengembangan dikecualikan oleh `.gitignore`. Untuk melaporkan masalah, gunakan mode `--demo` atau hapus nomor seri dan ID sistem dari laporan Anda sebelum membagikannya.

## Bahasa — versi 0.2.0

Aplikasi ini mencakup **50 bahasa antarmuka**. Klik kode bahasa (`FR`, `EN`, dll.) di kanan atas untuk membuka pengalih. Anda dapat mencari suatu bahasa berdasarkan nama aslinya, nama bahasa Inggrisnya, atau kodenya. Perubahannya langsung terjadi dan pilihan disimpan di `%APPDATA%/SanUsbViewer/preferences.json` di Windows. Tanpa preferensi, bahasa sistem digunakan; Bahasa yang tidak didukung akan kembali ke bahasa Inggris.

Pemilihan ini mengutamakan bahasa yang paling banyak digunakan secara total (bahasa ibu dan bahasa kedua), mengelompokkan varian regional. Ini bukan klasifikasi numerik: hasilnya bervariasi tergantung pada pengelompokan dialek. Varian yang dipilih antara lain aksara Cina Sederhana, Arab Standar, dan aksara Latin Fulani dan Uzbek; Punjabi menggunakan Gurmukhi.

Inggris, Tionghoa, Hindi, Spanyol, Prancis, Arab, Bengali, Portugis, Rusia, Urdu, Indonesia, Jerman, Jepang, Marathi, Vietnam, Telugu, Hausa, Turki, Punjabi, Swahili, Filipina, Tamil, Persia, Korea, Amharik, Thai, Jawa, Italia, Gujarati, Kannada, Bhojpuri, Burma, Polandia, Yoruba, Lingala, Oromo, Fulani, Malayalam, Odia, Ukraina, Sindhi, Uzbek, Pashto, Sunda, Melayu, Igbo, Nepal, Azeri, Maithili dan Somalia.

Menu yang disalin, diagnostik, teks bantuan, pesan aktivitas, dan detailnya dilokalkan. Nama dan pengidentifikasi yang diberikan oleh perangkat keras dan sistem dipertahankan. Ekspor JSON menyimpan data skema dan inventarisnya, apa pun bahasa tampilannya. Unit teknis tetap dalam **bit/s** (Mbit/s dan Gbit/s) untuk menghindari kebingungan dengan byte.

Katalog Perancis dan Inggris berfungsi sebagai referensi. Bahasa lainnya adalah **terjemahan otomatis**, yang mungkin memerlukan koreksi linguistik. File `locales/*.json` disertakan dalam kompilasi dan dapat diperbaiki lalu dikompilasi ulang. Tidak ada terjemahan jaringan yang dilakukan oleh aplikasi: 50 bahasa dan font Noto tertanam, di bawah Lisensi Font Terbuka SIL. Teks Arab menggunakan segmen Unicode dua arah dan baris baru yang dihitung sebelum diformat.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` mungkin merujuk ke file preferensi lain untuk pengujian atau instalasi portabel. Hanya kode bahasa yang disimpan di sana.
