# San USB Viewer

**Basa Jawa** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Terjemahan mesin; koreksi olèh.

**Versi saiki: `0.2.0`** — lisensi MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Native USB Explorer ditulis ing ** Rust **, karo antarmuka peteng, topologi visual lan diagnosa diwaca. Versi pisanan kanggo Windows 10/11.

## Asal saka proyek

Video ing asal saka project: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Proyek kasebut diilhami dening panggunaan ** WhatCable ** ing Apple lan ** USB Device Tree Viewer ** ing Windows: ngerti peripheral, hub lan kecepatan sambungan USB. Iki minangka implementasine independen.

## Fitur

- Deteksi nyata piranti lan pengontrol USB liwat `nusb`.
- Struktur wit kanthi jalur bis lan pelabuhan, kanthi ambane hub.
- Telusuri miturut jeneng, nomer seri utawa VID:PID; USB 3+ saringan lan hub.
- Kacepetan pranala rembugan: 1.5 / 12 / 480 Mb/s, 5 Gb/s,
utawa kategori SuperSpeedPlus (10 Gb / s utawa luwih).
- Diagram perjalanan: komputer, hub penengah, link lan piranti.
- Ing Windows, interogasi tambahan hub dening IOCTL kanggo mangerteni ing
Kapabilitas SuperSpeed sing diumumake piranti kasebut. Piranti sing kompatibel karo SuperSpeed sing disambungake liwat USB 2.0 ngasilake diagnostik watesan jalur.   USB 2 hub kanca saka USB 3 hub dianggep kapisah supaya ora ngubungake cabang 480 Mb / s kanggo fault kabel.
- Pengenal VID/PID, deskriptor USB, revisi, driver, serial lan antarmuka,
nalika sistem mbukak informasi iki.
- Nganyari otomatis saben telung detik, utawa manual nganggo F5.
- Log sesi saka 200 owah-owahan sing diamati pungkasan.
- Ekspor statement lengkap ing JSON (Ctrl + E) lan nyalin rincian sing dipilih.
- Pandhuan terpadu lan mode demonstrasi eksplisit, ora tau digunakake minangka serep
meneng yen gagal deteksi.

## uncalan

Kanthi Rust **1.95+**, toolchain MSVC lan alat Visual Studio C++ ing Windows. Conto nggunakake RTK kanggo nyaring output printah; tanpa RTK, mung mbusak awalan `rtk`. Skrip distribusi nampa loro kasus kasebut.

```powershell
rtk cargo run --release
```

Binar mandiri diprodhuksi ing `target/release/san-usb-viewer.exe`. Skrip `build.ps1` uga nggawe distribusi ing `dist/SanUsbViewer/`. Ora ana modifikasi driver utawa instalasi WinUSB sing dibutuhake kanggo inventaris: aplikasi kasebut ora mbutuhake antarmuka piranti.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Folder tujuan kanggo njupuk kudu ana. Versi rilis minangka aplikasi Windows tanpa konsol; luwih seneng `cargo run` ing pembangunan kanggo persediaan baris printah.

## Interpretasi asil

** kacepetan rembugan ** iku sambungan, lan ora pathokan kacepetan salinan. Kacepetan sing migunani gumantung utamane ing protokol, periferal lan piranti liyane sing nuduhake bis. Kacepetan saka macem-macem piranti ora nambah kanggo ngetung bandwidth kasedhiya.

Kabel USB pasif umume dudu piranti sing bisa dianggep. Versi iki ora maca e-marker, utawa daya Pangiriman Daya USB, utawa jinis konektore. Mulane ora ngaku nandhakake kacepetan maksimal. Diagnosis adhedhasar link sing diamati lan, yen kasedhiya, ing indikator Windows. Link alon bisa diwatesi dening kabel, port, hub, utawa piranti. Nomer versi deskriptor USB ora tau digunakake minangka kacepetan maksimum.

Kategori SuperSpeedPlus ora mbedakake antarane mode 10 lan 20 Gb/s ing kene. USB4/Thunderbolt lan terowongane ora dipriksa. Port tanpa piranti sing dipasang ora diinventarisasi. Kabel pangisi daya tanpa data ora nggawe piranti sing katon. Piranti internal (Bluetooth, kamera sing dibangun, lsp.) bisa uga katon ing wit USB.

Informasi sing ora bisa diakses ditampilake minangka ora kasedhiya. Produsèn lan antarmuka ora tansah kapapar dening Windows. Acara sing kedadeyan kabeh antarane pindai bisa uga ora dingerteni. Log ana ing memori lan diwiwiti sawise persediaan sukses pisanan.

Pengenal bus lan jalur port nggambarake topologi sistem. Konektor fisik bisa duwe rong jalur kapisah kanggo USB 2 lan SuperSpeed.

## Arsitektur lan validasi

`scanner.rs` nindakake persediaan; `windows_probe.rs` encapsulates native diwaca panjalukan saka hub; `model.rs` ngemot data lan diagnostik. `app.rs` nangani tugas latar mburi lan pilihan; `views.rs`, `inspector.rs`, `connection.rs` lan `theme.rs` tarik antarmuka karo egui / eframe. Antarmuka tetep interaktif sajrone pindai lan nahan maca sukses pungkasan yen pindai sabanjure gagal.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Pustaka kasebut minangka lintas platform lan kode Windows iku kondisional. Versi macOS/Linux ora divalidasi ing kiriman pisanan iki.

## Rahasia

Inventaris bisa digunakake sacara lokal, mung diwaca, tanpa layanan telemetri utawa jaringan. Link guide mung mbukak ing klik. Laporan JSON bisa ngemot nomer seri lan pengenal sistem: nuduhake sampeyan terserah.

## Referensi teknis

- [nusb: enumerasi USB lan informasi](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui / eframe](https://github.com/emilk/egui)

lisensi MIT. Bangunan njaba tetep tundhuk karo lisensi masing-masing.

Repositori umum mung ngemot sumber, terjemahan lan font gratis sing dibutuhake kanggo kompilasi. Wacan USB, tangkapan diagnostik, ekspor, preferensi lokal, cache lan binari pangembangan ora kalebu `.gitignore`. Kanggo nglaporake masalah, gunakake mode `--demo` utawa copot nomer seri lan ID sistem saka laporan sadurunge nuduhake.

## Basa - versi 0.2.0

Aplikasi kalebu **50 basa antarmuka**. Klik kode basa (`FR`, `EN`, etc.) ing sisih tengen ndhuwur kanggo mbukak switcher. Sampeyan bisa nggoleki basa kanthi jeneng asli, jeneng Inggris utawa kode. Owah-owahan langsung lan pilihan disimpen ing `%APPDATA%/SanUsbViewer/preferences.json` ing Windows. Tanpa preferensi, basa sistem digunakake; Basa sing ora didhukung bali menyang Inggris.

Pilihan kasebut milih basa sing paling akeh diucapake (basa ibu lan basa kapindho), nglumpukake varian regional. Iki dudu klasifikasi numerik: asil beda-beda gumantung saka klompok dialek. Varian sing dipilih kalebu Cina Sederhana, Arab Standar lan aksara Latin Fulani lan Uzbek; Punjabi nggunakake Gurmukhi.

Inggris, Tionghoa, Hindi, Spanyol, Prancis, Arab, Bengali, Portugis, Rusia, Urdu, Indonesia, Jerman, Jepang, Marathi, Vietnam, Telugu, Hausa, Turki, Punjabi, Swahili, Filipino, Tamil, Persia, Korea, Amharik, Thai, Jawa, Italia, Gujarati, Kannada, Bhojpuri, Burma, Polandia, Yoruba, Linglaniala, Ukrainian, Undhia, Ukrainian, Fukushima, Ukrainian, Fukushima Pashto, Sunda, Malay, Igbo, Nepali, Azeri, Maithili lan Somali.

Menu sing disalin, diagnostik, teks bantuan, pesen kegiatan lan rincian dilokalisasi. Jeneng lan pengenal sing diwenehake dening hardware lan sistem disimpen. Ekspor JSON nyimpen data skema lan inventaris, tanpa dipikirake basa tampilan. Unit teknis tetep ing ** bit / s ** (Mbit / s lan Gbit / s) supaya ora bingung karo bita.

Katalog Prancis lan Inggris minangka referensi. Basa liya yaiku **terjemahan otomatis**, sing mbutuhake koreksi linguistik. File `locales/*.json` kalebu ing kompilasi lan bisa didandani banjur dikompilasi maneh. Ora ana terjemahan jaringan sing ditindakake dening aplikasi kasebut: 50 basa lan font Noto dipasang, miturut Lisensi SIL Open Font. Teks Arab nggunakake segmen Unicode bidirectional lan baris anyar sing diwilang sadurunge format.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` bisa uga ngrujuk menyang file pilihan liyane kanggo tes utawa instalasi portabel. Mung kode basa sing disimpen ing kono.
