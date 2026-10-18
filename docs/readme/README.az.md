# San USB Viewer

**Azərbaycanca** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Maşın tərcümələri; düzəlişlər qəbul edilir.

**Mövcud versiya: `0.2.0`** — MIT lisenziyası.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Qaranlıq interfeys, vizual topologiya və oxuna bilən diaqnostika ilə **Rust**-da yazılmış yerli USB Explorer. Windows 10/11 üçün nəzərdə tutulmuş ilk versiya.

## Layihənin mənşəyi

Layihənin başlanğıcındakı video: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Layihə Apple-da **WhatCable** və Windows-da **USB Device Tree Viewer** istifadəsindən ilhamlanıb: periferiya qurğuları, hublar və onların USB qoşulma sürətlərini başa düşmək. Bu müstəqil bir tətbiqdir.

## Xüsusiyyətlər

- `nusb` vasitəsilə USB cihazlarının və nəzarətçilərinin faktiki aşkarlanması.
- Qovşağın dərinliyi ilə avtobus və liman yolu ilə ağac quruluşu.
- Ad, seriya nömrəsi və ya VID:PID üzrə axtarış; USB 3+ filtrləri və hubları.
- Razılaşdırılmış keçid sürəti: 1.5 / 12 / 480 Mb / s, 5 Gb / s,
və ya kateqoriya SuperSpeedPlus (10 Gb/s və ya daha çox).
- Səyahət diaqramı: kompüter, ara mərkəzlər, keçid və cihaz.
- Windows altında, IOCTL tərəfindən mərkəzin əlavə sorğusunu öyrənmək üçün
SuperSpeed imkanları cihaz tərəfindən açıqlandı. USB 2.0 vasitəsilə qoşulmuş SuperSpeed uyğun cihaz yol məhdudiyyəti diaqnostikasını yaradır.   USB 3 qovşaqlarının USB 2 hub yoldaşları, onların 480 Mb/s filialını kabel xətası ilə əlaqələndirməmək üçün ayrıca işlənir.
- VID/PID identifikatorları, USB deskriptoru, reviziya, sürücü, serial və interfeyslər,
sistem bu məlumatı ifşa etdikdə.
- Hər üç saniyədən bir avtomatik yeniləmə və ya F5 ilə əl ilə yeniləmə.
- Son 200 müşahidə dəyişikliyinin sessiya jurnalı.
- Tam bəyanatı JSON-da ixrac edin (Ctrl+E) və seçilmiş detalları kopyalayın.
- İnteqrasiya edilmiş bələdçi və açıq nümayiş rejimi, heç vaxt ehtiyat kimi istifadə edilmir
aşkarlama uğursuzluğu halında səssizdir.

## atmaq

Rust **1.95+**, MSVC alətlər silsiləsi və Windows-da Visual Studio C++ alətləri ilə. Nümunələr əmrlərin çıxışını süzgəcdən keçirmək üçün RTK-dan istifadə edir; RTK olmadan, sadəcə `rtk` prefiksini çıxarın. Dağıtım skripti hər iki halı qəbul edir.

```powershell
rtk cargo run --release
```

Bağımsız binar `target/release/san-usb-viewer.exe`-də istehsal olunur. `build.ps1` skripti `dist/`-də paylama da yaradır. İnventar üçün heç bir sürücü modifikasiyası və ya WinUSB quraşdırılması tələb olunmur: proqram cihaz interfeyslərini tələb etmir.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Çəkiliş üçün təyinat qovluğu mövcud olmalıdır. Buraxılış versiyası konsolsuz Windows proqramıdır; komanda xətti inventarının hazırlanmasında `cargo run`-yə üstünlük verin.

## Nəticələri şərh edin

**Razılaşdırılmış sürət** əlaqə sürətidir və surətin sürəti üçün etalon deyil. Faydalı sürətlər xüsusilə protokoldan, periferiyadan və avtobusu paylaşan digər cihazlardan asılıdır. Mövcud bant genişliyini hesablamaq üçün bir neçə cihazdan gələn sürətlər əlavə olunmur.

Passiv USB kabel ümumiyyətlə sadalanan cihaz deyil. Bu versiya nə onun e-markerini, nə USB Güc Çatdırılma gücünü, nə də konnektor tipini oxumur. Buna görə də maksimum sürətini təsdiqləməyə iddia etmir. Diaqnoz müşahidə edilən linkə və əgər varsa, Windows göstəricilərinə əsaslanır. Yavaş keçid kabel, port, hub və ya cihaz tərəfindən məhdudlaşdırıla bilər. USB təsviri versiya nömrəsi heç vaxt maksimum sürət kimi istifadə edilmir.

SuperSpeedPlus kateqoriyası burada 10 və 20 Gb/s rejimləri arasında fərq qoymur. USB4/Thunderbolt və onların tunelləri yoxlanılmır. Qurğu qoşulmayan portlar inventarlaşdırılmır. Məlumatı olmayan şarj kabeli görünən cihaz yaratmır. USB ağacında daxili qurğular (Bluetooth, daxili kamera və s.) görünə bilər.

Əlçatmaz məlumatlar əlçatmaz kimi göstərilir. İstehsalçı və interfeyslər həmişə Windows tərəfindən açıqlanmır. Tamamilə skanlar arasında baş verən hadisələr diqqətdən kənarda qala bilər. Jurnal yaddaşdadır və ilk uğurlu inventardan sonra başlayır.

Avtobus identifikatorları və port yolları sistem topologiyasını təsvir edir. Fiziki konnektorun USB 2 və SuperSpeed üçün iki ayrı yolu ola bilər.

## Memarlıq və doğrulama

`scanner.rs` inventar aparır; `windows_probe.rs` hublardan yerli oxumaq sorğularını əhatə edir; `model.rs` məlumat və diaqnostikadan ibarətdir. `app.rs` fon tapşırıqlarını və seçimi idarə edir; `views.rs`, `inspector.rs`, `connection.rs` və `theme.rs` interfeysi egui/eframe ilə çəkir. İnterfeys skan zamanı interaktiv olaraq qalır və növbəti skan uğursuz olarsa, sonuncu uğurlu oxunuşu saxlayır.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Kitabxanalar çarpaz platformadır və Windows kodu şərtidir. Bu ilk çatdırılmada macOS/Linux versiyaları təsdiqlənmir.

## Məxfilik

İnventar, telemetriya və ya şəbəkə xidmətləri olmadan yerli, yalnız oxumaq üçün işləyir. Bələdçi bağlantıları yalnız kliklə açılır. JSON hesabatlarında seriya nömrələri və sistem identifikatorları ola bilər: onları paylaşmaq sizə bağlıdır.

## Texniki istinadlar

- [nusb: USB nömrələmə və məlumat](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/frame](https://github.com/emilk/egui)

MIT lisenziyası. Əlavə tikililər öz lisenziyalarına tabe olaraq qalır.

İctimai repozitoriyada yalnız tərtib üçün lazım olan mənbələr, tərcümələr və pulsuz şriftlər var. USB oxunuşları, diaqnostik çəkilişlər, ixraclar, yerli seçimlər, keşlər və inkişaf ikili faylları `.gitignore` tərəfindən xaric edilir. Problemi bildirmək üçün `--demo` rejimindən istifadə edin və ya seriya nömrələrini və sistem identifikatorlarını paylaşmadan əvvəl hesabatlarınızdan silin.

## Dillər - versiya 0.2.0

Tətbiqə **50 interfeys dili** daxildir. Dəyişdiricini açmaq üçün yuxarı sağda dil kodunu (`FR`, `EN` və s.) klikləyin. Siz dili öz doğma adı, ingilis adı və ya kodu ilə axtara bilərsiniz. Dəyişiklik dərhal baş verir və seçim Windows altında `%APPDATA%/SanUsbViewer/preferences.json`-də saxlanılır. Üstünlük verilmədən sistem dili istifadə olunur; Dəstəklənməyən dil ingilis dilinə qayıdır.

Seçim regional variantları qruplaşdıraraq ümumi olaraq ən çox danışılan dillərə (ana dili və ikinci dil) üstünlük verir. Bu ədədi təsnifat deyil: nəticələr dialektlərin qruplaşdırılmasından asılı olaraq dəyişir. Seçilmiş variantlara Sadələşdirilmiş Çin, Standart Ərəb və Fulani və Özbək dillərinin latın yazıları daxildir; Pəncabi Gurmukhi dilindən istifadə edir.

İngilis, Çin, Hind, İspan, Fransız, Ərəb, Benqal, Portuqal, Rus, Urdu, İndoneziya, Alman, Yapon, Marathi, Vyetnam, Teluqu, Hausa, Türk, Pəncab, Suahili, Filippin, Tamil, Fars, Koreya, Amhar, Tay, Yava, İtalyan, Qucarati, Kannada, Yava, İtalyan, Gujarati, Kannada, Linga, Bura, Polish Oromo, Fulani, Malayalam, Odia, Ukrayna, Sindhi, Özbək, Puştu, Sundan, Malay, İqbo, Nepal, Azəri, Maithili və Somali.

Kopyalanan menyular, diaqnostika, yardım mətnləri, fəaliyyət mesajları və təfərrüatlar lokallaşdırılır. Avadanlıq və sistem tərəfindən təmin edilən adlar və identifikatorlar saxlanılır. JSON ixracı ekran dilindən asılı olmayaraq öz sxemini və inventar məlumatlarını saxlayır. Baytlarla qarışıqlığın qarşısını almaq üçün texniki vahidlər **bit/s** (Mbit/s və Gbit/s) ilə qalır.

Fransız və ingilis kataloqları istinad kimi xidmət edir. Digər dillər **avtomatik tərcümələrdir**, linqvistik düzəlişlər tələb edə bilər. `locales/*.json` faylları kompilyasiyaya daxildir və düzəldilə və sonra yenidən tərtib edilə bilər. Tətbiq tərəfindən heç bir şəbəkə tərcüməsi həyata keçirilmir: 50 dil və Noto şriftləri SIL Açıq Font Lisenziyası altında quraşdırılmışdır. Ərəb mətni formatlaşdırmadan əvvəl hesablanmış iki istiqamətli Unicode seqmentlərindən və yeni sətirlərdən istifadə edir.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` sınaq və ya portativ quraşdırma üçün başqa üstünlüklər faylına istinad edə bilər. Orada yalnız dil kodu saxlanılır.
