# San USB Viewer

**Afaan Oromoo** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Hiikkaa maashinii; sirreeffamni ni simatama.

**Vorshinii ammaa: `0.2.0`** — Hayyama MIT.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Native USB Explorer **Rust** tiin barreeffame, interfeesii dukkanaa'aa, toopoloojii mul'ataa fi adda baasuu dubbifamuu danda'u qaba. Vershinii jalqabaa Windows 10/11f yaadame.

## Ka’umsa pirojektichaa

Viidiyoo ka'umsa pirojektichaa: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Pirojektiin kun itti fayyadama **WhatCable** Apple irratti fi **USB Device Tree Viewer** Windows irratti: hubannoo peripherals, hubs fi saffisa walqabsiisa USB isaanii irraa kan ka'edha. Kun hojiirra oolmaa of danda’eedha.

## Amaloota

- Meeshaalee USB fi too'attoota karaa `nusb` qabatamaan adda baasuu.
- Caasaa mukaa daandii otobusii fi buufata dooniitiin, gadi fageenya hub waliin.
- Maqaa, lakkoofsa tartiiba ykn VID:PID tiin barbaadi; Filter fi hub USB 3+.
- Saffisa walitti hidhamiinsa marii: 1.5 / 12 / 480 Mb/s, 5 Gb/s, .
ykn gosa SuperSpeedPlus (10 Gb/s fi isaa ol).
- Diyaagirama imala: kompiitara, wiirtuuwwan gidduugaleessaa, hidhaa fi meeshaa.
- Windows jalatti, gaaffii fi deebii dabalataa hub IOCTLn gochuun baruuf
Dandeettiiwwan SuperSpeed meeshaa kanaan ifoomsan. Meeshaan SuperSpeed wajjin walsimu karaa USB 2.0 walqabsiifame adda baasuu daangeffama daandii uuma.   Hiriyoonni USB 2 hubs USB 3 hubs damee isaanii 480 Mb/s dogoggora keebilii irraa akka hin walqabsiifneef addatti ilaalamu.
- Adda baastoota VID/PID, ibsituu USB, fooyya'iinsa, konkolaachisaa, tartiiba fi interfeesii, .
yeroo sirnichi odeeffannoo kana saaxilu.
- Sekondii sadii sadiitti ofumaan haaromsuu, ykn F5 waliin harkaan.
- Galmee turtii jijjiirama 200 darban ilaalaman.
- Hima guutuu JSON (Ctrl+E) keessatti ergi fi ibsa filatame waraabi.
- Qajeelfama walitti makamaa fi haala agarsiisa ifa ta'e, akka duwwaatti gonkumaa hin fayyadamne
yoo detection kufe callisee.

## Darbachuu

Rust **1.95+** waliin, meeshaalee MSVC fi meeshaalee Visual Studio C++ Windows irratti. Fakkeenyonni firii ajajoota calaluuf RTK fayyadamu; RTK malee, durtii `rtk` qofa haqi. Iskiriiptiin raabsaa haala lamaan fudhata.

```powershell
rtk cargo run --release
```

Baayinariin of danda'e `target/release/san-usb-viewer.exe` keessatti oomishama. Iskiriiptiin `build.ps1` akkasumas raabsa `dist/` keessatti uuma. Fooyya'iinsi konkolaachisaa ykn fe'iinsi WinUSB kamiyyuu inventariif hin barbaachisu: aplikeeshiniin walqunnamsiistota meeshaa hin barbaadu.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Galfata galma qabachuuf jiraachuu qaba. Vershiniin gadhiifame kun application Windows kan konsoolii hin qabneedha; inventarii sarara ajajaaf `cargo run` misooma keessatti filadhu.

## Bu’aa isaa hiikuu

**Saffisni mari'atame** kan walitti hidhamiinsaati, malee ulaagaa saffisa waraabbii miti. Saffisni faayidaa qabu keessumaa pirootokoolii, naannoo fi meeshaalee biroo baasii qooddatan irratti hundaa'a. Saffisni meeshaalee hedduu irraa dhufu baandii jiru shallaguuf hin ida'amu.

Keebiliin USB dabarsoo akka waliigalaatti meeshaa lakkaa'amuu danda'u miti. Version kun e-marker isaa, humna USB Power Delivery, gosa connector isaa hin dubbisu. Kanaaf saffisa isaa isa guddaa mirkaneessa jedhee hin dubbatu. Qorannoon kun hidhaa ilaalame irratti hundaa'ee yoo jiraate agarsiistota Windows irratti hundaa'a. Hidhiin suuta jedhu keebilii, buufata, hub ykn meeshaatiin daangeffamuu danda'a. Lakkoofsi version ibsituu USB gonkumaa akka saffisa guddaatti hin fayyadamu.

Ramaddiin SuperSpeedPlus asitti haalawwan 10 fi 20 Gb/s addaan hin baasu. USB4/Thunderbolt fi tunnel isaanii hin sakatta'aman. Buufatni meeshaa hin maxxanne inventarii hin taasifamu. Keebiliin chaarjii daataa hin qabne meeshaa mul’atu hin uumu. Meeshaaleen keessaa (Bluetooth, built-in camera fi kkf) muka USB keessatti mul'achuu danda'u.

Odeeffannoon dhaqqabamaa hin taane akka hin jirretti agarsiifama. Oomishtoonni fi interfeesii yeroo hunda Windowsiin hin saaxilaman. Taateewwan guutummaatti iskaan gidduutti uumaman osoo hin hubatamiin hafuu danda’u. Galmeen yaadannoo keessa jira, inventarii milkaa'aa jalqabaa booda jalqaba.

Adda baastonni baasii fi daandii buufataa toopoloojii sirnichaa ibsu. Koneektariin fiizikaalaa USB 2 fi SuperSpeed tiif daandii adda addaa lama qabaachuu danda'a.

## Arkiteekcharii fi mirkaneessuu

`scanner.rs` inventarii raawwata; `windows_probe.rs` gaaffiiwwan dubbisa dhalootaa hubs irraa ni qabata; `model.rs` daataa fi adda baasuu of keessaa qaba. `app.rs` hojiiwwan duubbee fi filannoo ni qabata; `views.rs`, `inspector.rs`, `connection.rs` fi `theme.rs` egui/eframe waliin walqunnamtii kaasu. Walqunnamtiin yeroo skaaniin walqunnamtii ta'ee hafa, yoo skaaniin itti aanu kufe dubbisa milkaa'aa isa dhumaa qabata.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Manneen kitaabaa qaxxaamuraa-platform yoo ta'an koodii Windows haal-dureedha. Gosoonni macOS/Linux geejjibaa jalqabaa kana keessatti hin mirkanaa'an.

## Iccitii eeguu

Inventariin naannootti, dubbisa qofaaf, tajaajila telemetry ykn network malee hojjeta. Hidhamtoonni qajeelfamaa cuqaasuu qofa irratti banamu. Gabaasaaleen JSON lakkoofsa tartiiba fi adda baastoota sirnaa qabaachuu danda'u: isaan qooduun sirratti hundaa'a.

## Wabiiwwan teeknikaa

- [nusb: Lakkoofsa USB fi odeeffannoo](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html) jedhamuun beekama
- [Microsoft: IOCTL_USB_ODEEFFANNOO_WALQUNNAMTII_NODE_GET_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2) jedhamuun beekama
- [egui/eframe jedhamuun beekama](https://github.com/emilk/egui) jedhamuun beekama

Hayyama MIT. Gamoonni ala jiran hayyama isaanii jalatti hafaniiru.

Kuusaan ummataa maddoota, hiikkaa fi qubee bilisaa qindeessuuf barbaachisan qofa of keessaa qaba. Dubbifamni USB, qabannaa adda baasuu, al-ergii, filannoowwan naannoo, kaachota fi baayinarii misoomaa `.gitignore` tiin ala ta'u. Rakkoo gabaasuuf, haalata `--demo` fayyadami ykn lakkoofsa tartiiba fi ID sirnaa gabaasa kee irraa osoo hin qoodin haqi.

## Afaanota — version 0.2.0

Appilikeeshiniin kun **afaan walqunnamtii 50** of keessaa qaba. Jijjiiramaa banuuf koodii afaanii (`FR`, `EN`, fi kkf) gubbaa mirgaa cuqaasi. Afaan tokko maqaa dhaloota isaatiin, maqaa Ingiliffaa isaatiin ykn koodii isaatiin barbaaduu dandeessa. Jijjiiramni battalumatti kan raawwatamu yoo ta'u filannoo `%APPDATA%/SanUsbViewer/preferences.json` Windows jalatti kuufameera. Filannoo malee afaan sirnichaa fayyadama; Afaan hin deeggaramne tokko gara Ingiliffaatti deebi'a.

Filannoon kun afaanota baay’ee dubbataman walumaa galatti (afaan dhalootaa fi afaan lammaffaa) kan fayyadu yoo ta’u, garaagarummaa naannoo walitti gurmaa’a. Kun ramaddii lakkoofsaa miti: bu’aan isaa garee loqoda irratti hundaa’uun garaagarummaa qaba. Garaagarummaan filataman Chaayinaa salphaa, Arabaa Istaandardii fi qubee Laatiin Fuulaanii fi Uzbeek; Punjaabiin Gurmukhi fayyadama.

Afaan Ingiliffaa, Chaayinaa, Hindii, Ispeen, Faransaayi, Arabaa, Beengaalii, Poortugaal, Raashiyaa, Urduu, Indooneezhiyaa, Jarmanii, Jaappaan, Maraatii, Veetnaam, Telugu, Hawusa, Turkii, Punjaabi, Swahili, Filiippiinsii, Taamill, Faarsi, Kooriyaa, Amaaraa, Taayilaand, Jaavaan, Xaaliyaanii, Gujarati, Kannada, Bhojpuri, Burmase, Poolaandi, Yoruba, Afaan Liingaalaa, Afaan Oromoo, Fuulaanii, Malayaalam, Oodiyaa, Yukireen, Siindii, Uzbeek, Paashtoo, Sundaanii, Maalaay, Igboo, Neepaal, Azeri, Maayitii fi Somaalee.

Menuuwwan waraabaman, adda baasuu, barruuwwan gargaarsaa, ergaawwan sochii fi ibsitoonni naannootti ta'u. Maqaa fi adda baastonni haardwaarii fi sirnichi kennaman ni qabamu. Al-ergiin JSON deetaa iskiimaa fi inventarii isaa eega, afaan agarsiisaa osoo hin ilaalin. Yuunitiiwwan teeknikaa baayitii wajjin akka hin burjaajofneef **bit/s** (Mbit/s fi Gbit/s) keessatti hafu.

Kataloogoonni Faransaayii fi Ingiliffaa akka wabii tajaajilu. Afaanonni biroo **hiikkaa ofumaan** yoo ta'an, kunis sirreeffama afaanii barbaadu danda'a. Faayiloonni `locales/*.json` qindeessaa keessatti hammatamaniiru, sirreeffamuu fi sana booda irra deebi'amee qindaa'uu danda'u. Hiikkaan neetworkii kamiyyuu aplikeeshiniin hin raawwatamu: afaanonni 50 fi qubeewwan Noto kan ukkaamfamanidha, Hayyama Qubee Banaa SIL jalatti. Barruun Arabaa kutaalee Yuunikoodii kallattii lamaa fi sararoota haaraa foormaatii gochuu dura shallagaman fayyadama.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` faayilii filannoo biraa qorannoof ykn fe'iinsa socho'aa agarsiisuu danda'a. Koodii afaanii qofatu achitti kuufamee jira.
