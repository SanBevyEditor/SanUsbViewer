# San USB Viewer

**Fulfulde** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Firooji masiŋaaji; peeje ina njaɓɓaa.

**Njaru jooni: `0.2.0`** — yamiroore MIT.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Explorer USB asliijo binndaaɗo e **Rust**, jogiiɗo jokkorgal niɓɓiɗngal, topolooji yiyteede e ƴeewndorɗe janngeteeɗe. Jaaynde adannde ndee ko ngam Windows 10/11.

## Iwdi eɓɓaande ndee

Wideyoo to fuɗɗoode eɓɓoore ndee: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Eɓɓaande ndee ina ƴellitoo e kuutoragol **WhatCable** e Apple e **USB Device Tree Viewer** e Windows : faamde periferik, hubs e njaaweeki jokkorɗe mum en USB. Ɗum ko gollal keeringal.

## Tagle

- Yiytude goonga e kaɓirɗe USB e jokkondire e `nusb`.
- Sifaa leɗɗe e laawol otooji e porto, e luggiɗde hub.
- Yiylo e innde, limoore walla VID:PID; Filteruuji e huɓeere USB 3+.
- Njaaweeki jokkorde kaaldigal: 1,5 / 12 / 480 Mb/s, 5 Gb/s,
walla cate SuperSpeedPlus (10 Gb/s walla ko ɓuri ɗum).
- Natal yahdu : ordinateer, huɓeere hakkundeere, jokkorde e kaɓirgal.
- Les Windows, ɓeydaare naamne hub e IOCTL ngam anndude...
SuperSpeed baawɗe ɗe kaɓirgal ngal habri. Kaɓirgal SuperSpeed jokkondirngal e USB 2.0 ina waɗa ƴeewndo keerol laawol.   Hubs USB 2 sehilaaɓe hubs USB 3 ina njuɓɓinee ceertuɗe ngam waasde jogaade caltal mum en 480 Mb/s e ŋakkeende kaɓirgal.
- Anndinooje VID/PID, sifaa USB, ƴeewndo, difoore, serial e jokkondire,
nde system oo feeñnini ndeeɗoo humpito.
- Hesɗitinde otomatik kala leƴƴanɗe tati, walla manuel e F5.
- Log session e 200 mbayliigaaji cakkitiiɗi teskaaɗi ɗii.
- Yaltin konngol timmungol ngol e JSON (Ctrl+E) e ƴettude defte cuɓaaɗe ɗee.
- Ardorde kuuɓtidinnde e mbaadi hollirde laaɓtunde, meeɗaa huutoreede no backup nii
silmi so tawii detekteer ina ŋakki.

## Sakinugo

E Rust **1.95+**, kuutorɗe MSVC e kuutorɗe Visual Studio C++ e Windows. Yeruuji ɗii kuutortoo ko RTK ngam filtinde yaltoore jamirooje ; tawa RTK alaa, ittu tan jokkol `rtk` ngol. Binndol renndinde ngol ina jaɓa ɗeeɗoo geɗe ɗiɗi kala.

```powershell
rtk cargo run --release
```

Binnditagol ngol ina feewnee e `target/release/san-usb-viewer.exe`. Binndol `build.ps1` ngol kadi ina sosa renndo e nder `dist/`. Alaa waylo-waylo difoore walla udditgol WinUSB ina haani wonande innde ndee : kuutorgal ngal ɗaɓɓaani jokkondire kaɓirɗe.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Dowlaaji jokkondire ngam nanngude ina poti woodde. Jaaynde yaltinnde ndee ko kuutorgal Windows ngal alaa konsol ; ɓuri yiɗde `cargo run` e ƴellitaare ngam limto doggol yamiroore.

## Firo ko heɓaa heen koo

**Njaaweeki kaaldigal** ko oon jokkorde, wonaa teskuyaaji njaaweeki koppi. Jaawle nafooje ɗee fawii ko e porotokol, periferik e kaɓirɗe goɗɗe rennduɗe oto oo. Jaawle ummoraade e kaɓirɗe keewɗe ɓeydataa ngam hiisaade bandwidth keɓaaɗo.

Kaɓirgal USB passif ko ɓuri heewde wonaa kaɓirgal limtotoongal. Ooɗoo mbaydi janngaani e-marker mum, walla doole USB Power Delivery, walla sifaa jokkorde mum. Ko ɗum waɗi nde wiytaani ina seedtoo njaaweeki mayre ɓurki mawnude. Ñawu nguu tuugii ko e jokkorde teskaande ndee, so ina woodi, e kollitooje Windows. Jokkorde leelnde ina waawi ustaade e kaɓirgal, porto, hub, walla kaɓirgal. Limoore mbaydi sifotoondi USB meeɗaa huutoreede no njaaweeki ɓurki mawnude nii.

Kategori SuperSpeedPlus seerndaani ɗoo hakkunde 10 e 20 Gb/s. USB4/Thunderbolt e tunwuɗi mum en ƴeewtaaka. Portooji ɗi ngalaa kaɓirgal jokkondirngal, limtaaka. Kaɓirgal charging ngal alaa data, sosataa kaɓirgal yiyteteengal. Kaɓirɗe nder (Bluetooth, kamera mahaaɗo ekn) ina mbaawi feeñde e lekki USB.

Kabaaruuji ɗi keɓaaka kollirtee ko ɗi keɓaaka. Pewnitoowo oo e jokkorɗe ɗee ngonaa ko Windows feeñnini ɗum en sahaa kala. Kewuuji baɗeteeɗi hakkunde skanndeeji ɗii fof ina mbaawi waasde teskaade. Log oo ina woni e ciimtol, fuɗɗotoo ko caggal limto adanngo jaɓngo.

Anndinooje otooji e laabi porto ina cifa topolooji sisteem. Jokkorde ɓaleere ina waawi jogaade laabi ɗiɗi ceertuɗi wonande USB 2 e SuperSpeed.

## Arsitektuur e tabitinde

`scanner.rs` ina waɗa limto; `windows_probe.rs` ina moofta ɗaɓɓaaɗe janngeteeɗe native ummoraade e hubs; `model.rs` ina waɗi dokke e ƴeewndorɗe. `app.rs` ina jokkondiri e golle caggal e suɓagol; `views.rs`, `inspector.rs`, `connection.rs` e `theme.rs` ina njiyloo jokkondire e egui/eframe. Koolaaɗo kuuɓal oo ina heddii e jokkondirde e sahaa skannde, ina jogoo janngugol cakkitiingol jaɓngol so skannde aroore ndee ŋakki.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Defte ɗee ko cross-platform e kode Windows ko sarɗiiji. Jaayɗe macOS/Linux ɗee ndartinaaka e ndeeɗoo jokkondire gadane.

## Sirlu

Innde ina gollira e nokku, janngetee tan, tawa alaa telemetri walla sarwisaaji lowre. Jokkorɗe njiylawu ɗee uddittee tan ko e ɓuuɓri. Ciimtol JSON ina waawi jogaade limooje jokkondire e anndinooje sisteem : renndinde ɗe ko aan tan waawi ɗum.

## Tuugnorgal karallaagal

- [nusb: Limto USB e humpito](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Mikrosoft: IOCTL_USB_HEƁ_JOKKOL_NODE_HUMPITOL_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/kaɓirgal](https://github.com/emilk/egui)

Laylaytol MIT. Mahdiiji ɗii ina keddii e jogaade yamiroore mum en.

Depo renndo ngoo ina waɗi tan ko iwdiiji, firooji e binndi ɗi ngalaa njoɓdi potɗi huutoreede ngam mooftude. Janngugol USB, nanngugol diagnostic, yaltugol, ɓure nokkuuje, kaɓirɗe e binndi ƴellitaare ina njaltinaa e `.gitignore`. Ngam humpitaade caɗeele, huutoro mbaadi `--demo` walla ittu limooje serie e ID-ji system e ciimtol maa hade maa renndinde ɗum.

## Ɗemɗe — mbaydi 0.2.0

Kuutorgal ngal ina waɗi **50 ɗemngal jokkorgal**. Ɓoɗɗu kodde ɗemngal (`FR`, `EN`, ekn) to dow to bannge ñaamo ngam udditaade jokkorgal ngal. Aɗa waawi yiylaade ɗemngal e innde mum, innde mum e ɗemngal Engele walla kod mum. Waylo ngoo ko ɗoon e ɗoon kadi cuɓagol ngol ina mooftaa e `%APPDATA%/SanUsbViewer/preferences.json` les Windows. So ɓuraani, ɗemngal system ngal huutortee ; Ɗemngal ngal wallittaaka ina hoota e Engele.

Suɓngo ngoo ɓuri yiɗde ko ɗemɗe ɓurɗe haaleede e nder keewal (ɗemngal neeniwal e ɗemngal ɗiɗaɓal), renndinde mbaydiiji diiwaanuuji. Ɗuum wonaa senngo limoore : njeñtudi ndii ina seerti fawaade e renndinde ɗemɗe ɗee. Feereeji cuɓaaɗi ɗii ko Sinuwaa koolaaɗo, Arab kuuɓtodinɗo e binndi Latin Fulani e Usbek ; Punjabi ina huutoroo ɗemngal Gurmuki.

Engele, Sinuwaa, Hindi, Españool, Farayse, Arab, Bengali, Purtugeec, Riisi, Urdu, Indoneesi, Almaañ, Japon, Marathi, Wiyetnaam, Telugu, Hawsa, Turki, Punjabi, Suwahili, Filipiin, Tamil, Perse, Kore, Amhari, Taylande, Java, Itaali, Gujarati, Poli Yoruba, Lingala, Oromo, Fulani, Malayalam, Odia, Ukrani, Sindhi, Usbek, Pasto, Sundaan, Malay, Igbo, Nepali, Azeri, Maithili e Somali.

Menuuji koppiraaɗi, diagnostics, binndanɗe ballal, mesasuuji golle e detaayuuji ina njuɓɓinee e nokkuuji. Inɗe e anndinooje ɗe hardware e system rokki ina keɓee. Eksport JSON ina jogoo dokke mum skeema e innde, tawa ɗemngal hollito ngal ina fawii heen. Yuɓɓooji karallaagal ina keddii e **bit/s** (Mbit/s e Gbit/s) ngam waasde jiiɓrude e bayte.

Catal Farayse e Engele ina waɗi heen teskuyaaji. Ɗemɗe goɗɗe ko **firooji otomaatik**, ina mbaawi ɗaɓɓude peeje ɗemɗe. Fiilde `locales/*.json` ina tawee e mooftirgel ngel, ina waawi feewnireede, caggal ɗuum mooftireede. Alaa firo lowre nde kuutorgal ngal waɗata : ɗemɗe 50 ɗee e binndi Noto ina njuɓɓinee, e les njiimaandi SIL Open Font License. Binndol aarabeere huutortoo ko taƴe Unicode ɗiɗi e diidi kesi hiisaaɗi hade mum en waɗeede.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` ina waawi tuugnaade e fiilde ɓurannde woɗnde ngam jarribaade walla jokkorgal portaabe. Ko kodde ɗemngal tan hisnaa toon.
