# San USB Viewer

**Türkçe** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Makine çevirileri; düzeltmeler memnuniyetle karşılanır.

**Mevcut sürüm: `0.2.0`** — MIT lisansı.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

**Rust** ile yazılmış, karanlık bir arayüze, görsel topolojiye ve okunabilir tanılamalara sahip yerel USB Explorer. Windows 10/11 için tasarlanan ilk sürüm.

## Projenin kökeni

Projenin kaynağındaki video: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Proje, Apple'daki **WhatCable** ve Windows'taki **USB Device Tree Viewer** kullanımlarından ilham almıştır: çevre birimlerini, hub'ları ve USB bağlantılarının hızlarını anlamak. Bu bağımsız bir uygulamadır.

## Özellikler

- `nusb` aracılığıyla USB cihazlarının ve denetleyicilerinin gerçek tespiti.
- Otobüs ve liman yoluna göre, göbek derinliğine sahip ağaç yapısı.
- Ada, seri numarasına veya VID:PID'ye göre arayın; USB 3+ filtreler ve hub'lar.
- Anlaşmalı bağlantı hızı: 1,5 / 12 / 480 Mb/s, 5 Gb/s,
veya SuperSpeedPlus kategorisi (10 Gb/s veya daha fazla).
- Yolculuk şeması: bilgisayar, ara hub'lar, bağlantı ve cihaz.
- Windows altında, hub'ın IOCTL tarafından ek olarak sorgulanması
Cihaz tarafından duyurulan SuperSpeed yetenekleri. USB 2.0 yoluyla bağlanan SuperSpeed uyumlu bir cihaz, bir yol sınırlama teşhisi üretir.   USB 3 hub'larının USB 2 hub'ları tamamlayıcıları, 480 Mb/s'lik dallarının bir kablo hatasına atfedilmesini önlemek için ayrı olarak ele alınır.
- VID/PID tanımlayıcıları, USB tanımlayıcısı, revizyonu, sürücüsü, seri ve arayüzleri,
sistem bu bilgiyi ortaya çıkardığında.
- Her üç saniyede bir otomatik güncelleme veya F5 ile manuel güncelleme.
- Gözlemlenen son 200 değişikliğin oturum günlüğü.
- İfadenin tamamını JSON'a (Ctrl+E) aktarın ve seçilen ayrıntıları kopyalayın.
- Entegre kılavuz ve açık gösterim modu, hiçbir zaman yedek olarak kullanılmaz
algılama hatası durumunda sessiz.

## Fırlatmak

Rust **1.95+**, MSVC araç zinciri ve Windows'ta Visual Studio C++ araçlarıyla. Örnekler, komutların çıktısını filtrelemek için RTK'yı kullanır; RTK olmadan `rtk` önekini kaldırmanız yeterlidir. Dağıtım betiği her iki durumu da kabul eder.

```powershell
rtk cargo run --release
```

Bağımsız ikili dosya `target/release/san-usb-viewer.exe`'de üretilir. `build.ps1` betiği ayrıca `dist/SanUsbViewer/`'de bir dağıtım oluşturur. Envanter için herhangi bir sürücü değişikliği veya WinUSB kurulumu gerekli değildir: uygulama, cihaz arayüzleri gerektirmez.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Yakalamanın hedef klasörü mevcut olmalıdır. Yayın sürümü, konsolu olmayan bir Windows uygulamasıdır; komut satırı envanteri geliştirme aşamasında `cargo run`'yi tercih edin.

## Sonuçları yorumlayın

**Anlaşılan hız** bağlantı hızıdır ve kopyalama hızının bir ölçütü değildir. Yararlı hızlar özellikle protokole, çevre birimine ve veri yolunu paylaşan diğer cihazlara bağlıdır. Birden fazla cihazın hızları kullanılabilir bant genişliğini hesaplamak için toplanmaz.

Pasif bir USB kablosu genellikle numaralandırılabilir bir cihaz değildir. Bu sürüm e-işaretini, USB Güç Dağıtımı gücünü veya konektör türünü okumaz. Bu nedenle maksimum hızını belgeleme iddiasında değildir. Teşhis, gözlemlenen bağlantıya ve varsa Windows göstergelerine dayanır. Yavaş bir bağlantı kablo, bağlantı noktası, hub veya aygıtla sınırlı olabilir. USB tanımlayıcı sürüm numarası hiçbir zaman maksimum hız olarak kullanılmaz.

SuperSpeedPlus kategorisi burada 10 ve 20 Gb/s modları arasında ayrım yapmamaktadır. USB4/Thunderbolt ve tünelleri incelenmez. Cihaz takılı olmayan bağlantı noktaları envantere alınmaz. Veri içermeyen bir şarj kablosu görünür bir cihaz oluşturmaz. USB ağacında dahili cihazlar (Bluetooth, dahili kamera vb.) görünebilir.

Erişilemeyen bilgiler mevcut değil olarak görüntülenir. Üretici ve arayüzler Windows tarafından her zaman açıklanmaz. Tamamen taramalar arasında meydana gelen olaylar gözden kaçabilir. Günlük bellektedir ve ilk başarılı envanterden sonra başlar.

Veri yolu tanımlayıcıları ve bağlantı noktası yolları sistem topolojisini tanımlar. Fiziksel bir konektörün USB 2 ve SuperSpeed için iki ayrı yolu olabilir.

## Mimari ve doğrulama

`scanner.rs` envanter gerçekleştirir; `windows_probe.rs`, hub'lardan gelen yerel okuma isteklerini kapsüller; `model.rs`, verileri ve teşhisleri içerir. `app.rs` arka plan görevlerini ve seçimini gerçekleştirir; `views.rs`, `inspector.rs`, `connection.rs` ve `theme.rs`, egui/eframe ile arayüzü çizer. Arayüz tarama sırasında etkileşimli kalır ve bir sonraki tarama başarısız olursa son başarılı okumayı korur.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Kitaplıklar platformlar arasıdır ve Windows kodu koşulludur. Bu ilk teslimatta macOS/Linux sürümleri doğrulanmadı.

## Gizlilik

Envanter yerel olarak, salt okunur olarak, telemetri veya ağ hizmetleri olmadan çalışır. Kılavuz bağlantıları yalnızca tıklandığında açılır. JSON raporları seri numaraları ve sistem tanımlayıcıları içerebilir: bunları paylaşmak size kalmıştır.

## Teknik referanslar

- [nusb: USB numaralandırma ve bilgi](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

MİT lisansı. Müştemilatlar ilgili ruhsatlarına tabi olmaya devam etmektedir.

Halka açık depo yalnızca derleme için gerekli olan kaynakları, çevirileri ve ücretsiz yazı tiplerini içerir. USB okumaları, tanılama kayıtları, dışa aktarmalar, yerel tercihler, önbellekler ve geliştirme ikili dosyaları `.gitignore` tarafından hariç tutulur. Bir sorunu bildirmek için `--demo` modunu kullanın veya paylaşmadan önce raporlarınızdan seri numaralarını ve sistem kimliklerini kaldırın.

## Diller — sürüm 0.2.0

Uygulama **50 arayüz dilini** içerir. Değiştiriciyi açmak için sağ üstteki dil koduna (`FR`, `EN`, vb.) tıklayın. Bir dili yerel adına, İngilizce adına veya koduna göre arayabilirsiniz. Değişiklik anında gerçekleşir ve seçim Windows altında `%APPDATA%/SanUsbViewer/preferences.json`'de saklanır. Tercih olmaksızın sistem dili kullanılır; Desteklenmeyen bir dil İngilizceye geri döner.

Seçim, bölgesel değişkenleri bir araya toplayarak toplamda en çok konuşulan dilleri (ana dil ve ikinci dil) tercih ediyor. Bu sayısal bir sınıflandırma değildir: sonuçlar lehçelerin gruplandırılmasına bağlı olarak değişiklik gösterir. Seçilen çeşitler arasında Basitleştirilmiş Çince, Standart Arapça ve Fulani ve Özbekçenin Latin alfabeleri; Punjabi Gurmukhi'yi kullanıyor.

İngilizce, Çince, Hintçe, İspanyolca, Fransızca, Arapça, Bengalce, Portekizce, Rusça, Urduca, Endonezyaca, Almanca, Japonca, Marathi, Vietnamca, Telugu dili, Hausa dili, Türkçe, Pencapça, Svahili dili, Filipince, Tamilce, Farsça, Korece, Amharca, Tayca, Cava Dili, İtalyanca, Gujarati dili, Kannadaca, Bhojpuri, Birmanya dili, Lehçe, Yorubaca, Lingala, Oromo, Fulani, Malayalam, Odia, Ukraynaca, Sindhi, Özbek, Peştuca, Sundan, Malay, İgbo, Nepal, Azeri, Maithili ve Somali.

Kopyalanan menüler, teşhisler, yardım metinleri, etkinlik mesajları ve ayrıntılar yerelleştirilir. Donanım ve sistem tarafından sağlanan adlar ve tanımlayıcılar korunur. JSON dışa aktarımı, görüntüleme dilinden bağımsız olarak şema ve envanter verilerini korur. Baytlarla karışıklığı önlemek için teknik birimler **bit/s** (Mbit/s ve Gbit/s) cinsinden kalır.

Fransızca ve İngilizce kataloglar referans niteliğindedir. Diğer diller **otomatik çevirilerdir** ve dilsel düzeltmeler gerektirebilir. `locales/*.json` dosyaları derlemeye dahil edilmiştir ve düzeltilip yeniden derlenebilir. Uygulama tarafından herhangi bir ağ çevirisi yapılmamaktadır: 50 dil ve Noto yazı tipleri, SIL Açık Yazı Tipi Lisansı kapsamında yerleştirilmiştir. Arapça metin, biçimlendirmeden önce hesaplanan çift yönlü Unicode bölümlerini ve yeni satırları kullanır.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG`, test veya taşınabilir kurulum için başka bir tercihler dosyasına başvurabilir. Burada yalnızca dil kodu kaydedilir.
