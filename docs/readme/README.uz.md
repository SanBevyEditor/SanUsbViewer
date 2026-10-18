# San USB Viewer

**Oʻzbekcha** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Mashina tarjimalari; tuzatishlar qabul qilinadi.

**Joriy versiya: `0.2.0`** — MIT litsenziyasi.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Native USB Explorer **Rust**-da yozilgan, qorong'u interfeys, vizual topologiya va o'qilishi mumkin bo'lgan diagnostika bilan. Windows 10/11 uchun mo'ljallangan birinchi versiya.

## Loyihaning kelib chiqishi

Loyihaning kelib chiqishidagi video: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Loyiha Apple’da **WhatCable** va Windows’da **USB Device Tree Viewer** qo‘llanilishidan ilhomlangan: tashqi qurilmalar, hublar va ularning USB ulanish tezligini tushunish. Bu mustaqil amalga oshirish.

## Xususiyatlari

- `nusb` orqali USB qurilmalari va kontrollerlarini haqiqiy aniqlash.
- Avtobus va port yo'li bo'yicha daraxt tuzilishi, markaz chuqurligi bilan.
- Ism, seriya raqami yoki VID:PID bo'yicha qidirish; USB 3+ filtrlari va markazlari.
- Kelishilgan ulanish tezligi: 1,5 / 12 / 480 Mb / s, 5 Gb / s,
yoki SuperSpeedPlus toifasi (10 Gb/s yoki undan ortiq).
- Sayohat diagrammasi: kompyuter, oraliq markazlar, havola va qurilma.
- Windows ostida, bilish uchun IOCTL tomonidan markazni qo'shimcha so'roq qilish
Qurilma tomonidan e'lon qilingan SuperSpeed imkoniyatlari. USB 2.0 orqali ulangan SuperSpeed mos keluvchi qurilma yo'l cheklovi diagnostikasini ishlab chiqaradi.   USB 3 hublarining USB 2 hublari, ularning 480 Mb/s tarmog'ini kabel nosozliklari bilan bog'lamaslik uchun alohida ishlanadi.
- VID/PID identifikatorlari, USB deskriptori, reviziyasi, drayveri, seriyali va interfeyslari,
tizim bu ma'lumotni oshkor qilganda.
- Har uch soniyada avtomatik yangilash yoki F5 bilan qo'lda yangilash.
- Oxirgi kuzatilgan 200 ta oʻzgarishlarning sessiya jurnali.
- To'liq bayonotni JSON formatida eksport qiling (Ctrl+E) va tanlangan ma'lumotlarni nusxalash.
- O'rnatilgan qo'llanma va aniq namoyish rejimi, hech qachon zaxira sifatida ishlatilmaydi
aniqlash muvaffaqiyatsiz bo'lsa jim.

## Otish

Rust bilan **1.95+**, MSVC asboblar zanjiri va Windows-dagi Visual Studio C++ vositalari. Misollar buyruqlar chiqishini filtrlash uchun RTK dan foydalanadi; RTK holda, faqat `rtk` prefiksini olib tashlang. Tarqatish skripti ikkala holatni ham qabul qiladi.

```powershell
rtk cargo run --release
```

Mustaqil ikkilik `target/release/san-usb-viewer.exe` da ishlab chiqariladi. `build.ps1` skripti `dist/` da tarqatishni ham yaratadi. Inventarizatsiya uchun drayverlarni o'zgartirish yoki WinUSB-ni o'rnatish shart emas: dastur qurilma interfeyslarini talab qilmaydi.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Suratga olish uchun moʻljallangan papka mavjud boʻlishi kerak. Chiqarish versiyasi konsolsiz Windows ilovasi; buyruq qatori inventarizatsiyasini ishlab chiqishda `cargo run` ni afzal ko'ring.

## Natijalarni izohlang

**kelishilgan tezlik** ulanish tezligi bo'lib, nusxa ko'chirish tezligining benchmarki emas. Foydali tezliklar, xususan, protokolga, periferik va avtobusni baham ko'radigan boshqa qurilmalarga bog'liq. Bir nechta qurilmalarning tezligi mavjud tarmoqli kengligini hisoblash uchun qo'shilmaydi.

Passiv USB kabeli odatda sanab bo'lmaydigan qurilma emas. Ushbu versiya uning elektron markerini, USB quvvat yetkazib berish quvvatini va ulagich turini o'qimaydi. Shuning uchun u maksimal tezligini tasdiqlashga da'vo qilmaydi. Tashxis kuzatilgan havolaga va agar mavjud bo'lsa, Windows ko'rsatkichlariga asoslanadi. Sekin aloqa kabel, port, markaz yoki qurilma bilan cheklanishi mumkin. USB deskriptor versiya raqami hech qachon maksimal tezlik sifatida ishlatilmaydi.

SuperSpeedPlus toifasi bu yerda 10 va 20 Gb/s rejimlarni ajratmaydi. USB4/Thunderbolt va ularning tunnellari tekshirilmaydi. Qurilma ulanmagan portlar inventarizatsiya qilinmaydi. Ma'lumotsiz zaryadlovchi kabel ko'rinadigan qurilmani yaratmaydi. USB daraxtida ichki qurilmalar (Bluetooth, o'rnatilgan kamera va boshqalar) paydo bo'lishi mumkin.

Kirish mumkin bo'lmagan ma'lumotlar mavjud emas sifatida ko'rsatiladi. Ishlab chiqaruvchi va interfeyslar har doim ham Windows tomonidan ta'sirlanmaydi. To'liq skanerlashlar o'rtasida sodir bo'ladigan hodisalar e'tibordan chetda qolishi mumkin. Jurnal xotirada va birinchi muvaffaqiyatli inventarizatsiyadan keyin boshlanadi.

Avtobus identifikatorlari va port yo'llari tizim topologiyasini tavsiflaydi. Jismoniy ulagichda USB 2 va SuperSpeed uchun ikkita alohida yo'l bo'lishi mumkin.

## Arxitektura va tasdiqlash

`scanner.rs` inventarizatsiyani amalga oshiradi; `windows_probe.rs` markazlardan mahalliy o'qish so'rovlarini qamrab oladi; `model.rs` ma'lumotlar va diagnostikani o'z ichiga oladi. `app.rs` fon vazifalari va tanlash bilan shug'ullanadi; `views.rs`, `inspector.rs`, `connection.rs` va `theme.rs` interfeysni egui/eframe bilan chizadi. Interfeys skanerlash vaqtida interaktiv bo'lib qoladi va agar keyingi skanerlash muvaffaqiyatsiz bo'lsa, oxirgi muvaffaqiyatli o'qishni saqlab qoladi.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Kutubxonalar o'zaro platformali va Windows kodi shartli. Ushbu birinchi yetkazib berishda macOS/Linux versiyalari tasdiqlanmagan.

## Maxfiylik

Inventarizatsiya mahalliy, faqat o'qish uchun, telemetriya yoki tarmoq xizmatlarisiz ishlaydi. Qo'llanma havolalari faqat bosish orqali ochiladi. JSON hisobotlarida seriya raqamlari va tizim identifikatorlari bo'lishi mumkin: ularni baham ko'rish sizga bog'liq.

## Texnik ma'lumotnomalar

- [nusb: USB ro'yxati va ma'lumotlari](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/ramka](https://github.com/emilk/egui)

MIT litsenziyasi. Qo'shimcha binolar tegishli litsenziyalarga bo'ysunadi.

Umumiy omborda faqat manbalar, tarjimalar va kompilyatsiya uchun zarur bo'lgan bepul shriftlar mavjud. USB o'qishlari, diagnostik tasvirlar, eksportlar, mahalliy afzalliklar, keshlar va ikkilik fayllarni ishlab chiqish `.gitignore` tomonidan bundan mustasno. Muammo haqida xabar berish uchun `--demo` rejimidan foydalaning yoki seriya raqamlari va tizim identifikatorlarini almashishdan oldin hisobotlaringizdan olib tashlang.

## Tillar - 0.2.0 versiyasi

Ilova **50 ta interfeys tilini o'z ichiga oladi**. O'zgartirgichni ochish uchun yuqori o'ng burchakdagi til kodini (`FR`, `EN` va boshqalar) bosing. Siz tilni ona nomi, inglizcha nomi yoki kodi bo'yicha qidirishingiz mumkin. O'zgartirish darhol amalga oshiriladi va tanlov Windows ostida `%APPDATA%/SanUsbViewer/preferences.json` da saqlanadi. Afzalliksiz tizim tilidan foydalaniladi; Qo'llab-quvvatlanmaydigan til ingliz tiliga qaytadi.

Tanlov mintaqaviy variantlarni birlashtirgan holda jami eng ko'p gapiriladigan tillarni (ona tili va ikkinchi til) ma'qullaydi. Bu raqamli tasnif emas: natijalar dialektlarning guruhlanishiga qarab farqlanadi. Tanlangan variantlar orasida soddalashtirilgan xitoy, standart arab va fulani va oʻzbek lotin yozuvlari; Panjobi Gurmukhi tilidan foydalanadi.

Ingliz, xitoy, hind, ispan, frantsuz, arab, bengal, portugal, rus, urdu, indoneziya, nemis, yapon, marathi, vetnam, telugu, xausa, turk, panjob, suahili, filippin, tamil, fors, koreys, amxar, tay, yava, italyan, gujarati, kannada, yava, italyan, gujarati, kannada, lingapur, burjua, polish, yome Oromo, fulani, malayalam, odiya, ukrain, sindxiy, oʻzbek, pushtu, sundan, malay, igbo, nepal, ozarbayjon, maytili va somali.

Nusxalangan menyular, diagnostika, yordam matnlari, faoliyat xabarlari va tafsilotlari mahalliylashtiriladi. Uskuna va tizim tomonidan taqdim etilgan nomlar va identifikatorlar saqlanadi. JSON eksporti ekran tilidan qatʼiy nazar oʻz sxemasi va inventar maʼlumotlarini saqlaydi. Baytlar bilan chalkashmaslik uchun texnik birliklar **bit/s** (Mbit/s va Gbit/s) da qoladi.

Fransuz va ingliz kataloglari ma'lumotnoma sifatida xizmat qiladi. Boshqa tillar **avtomatik tarjimalar** bo'lib, ular lingvistik tuzatishlarni talab qilishi mumkin. `locales/*.json` fayllari kompilyatsiyaga kiritilgan va ularni tuzatib, keyin qayta kompilyatsiya qilish mumkin. Ilova tomonidan tarmoq tarjimasi amalga oshirilmaydi: 50 ta til va Noto shriftlari SIL Open Shrift Litsenziyasi ostida o'rnatilgan. Arab matni formatlashdan oldin hisoblangan ikki tomonlama Unicode segmentlari va yangi qatorlardan foydalanadi.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` sinov yoki portativ o'rnatish uchun boshqa afzalliklar fayliga murojaat qilishi mumkin. U erda faqat til kodi saqlanadi.
