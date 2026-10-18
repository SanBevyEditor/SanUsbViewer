# San USB Viewer

**Filipino** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Mga pagsasalin sa makina; malugod na tinatanggap ang mga pagwawasto.

**Kasalukuyang bersyon: `0.2.0`** — Lisensya ng MIT.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Native USB Explorer na nakasulat sa **Rust**, na may madilim na interface, visual na topology at nababasang diagnostics. Unang bersyon na nilayon para sa Windows 10/11.

## Pinagmulan ng proyekto

Video sa pinagmulan ng proyekto: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Ang proyekto ay inspirasyon ng paggamit ng **WhatCable** sa Apple at **USB Device Tree Viewer** sa Windows: pag-unawa sa mga peripheral, hub at ang bilis ng kanilang mga koneksyon sa USB. Ito ay isang malayang pagpapatupad.

## Mga tampok

- Aktwal na pagtuklas ng mga USB device at controller sa pamamagitan ng `nusb`.
- Istraktura ng puno sa pamamagitan ng bus at port path, na may lalim ng hub.
- Maghanap ayon sa pangalan, serial number o VID:PID; Mga filter at hub ng USB 3+.
- Nakipag-usap na bilis ng link: 1.5 / 12 / 480 Mb/s, 5 Gb/s,
o kategoryang SuperSpeedPlus (10 Gb/s o higit pa).
- Diagram ng paglalakbay: computer, mga intermediate hub, link at device.
- Sa ilalim ng Windows, karagdagang interogasyon ng hub ng IOCTL upang malaman ang
SuperSpeed na mga kakayahan na inihayag ng device. Ang SuperSpeed na katugmang device na konektado sa pamamagitan ng USB 2.0 ay gumagawa ng path limitation diagnostic.   Ang mga USB 2 hubs na kasama ng USB 3 hubs ay ginagamot nang hiwalay upang maiwasang maiugnay ang kanilang 480 Mb/s branch sa isang cable fault.
- Mga pagkakakilanlan ng VID/PID, USB descriptor, rebisyon, driver, serial at mga interface,
kapag inilantad ng system ang impormasyong ito.
- Awtomatikong pag-update bawat tatlong segundo, o manual na may F5.
- Log ng session ng huling 200 naobserbahang pagbabago.
- I-export ang kumpletong pahayag sa JSON (Ctrl+E) at kopyahin ang mga napiling detalye.
- Pinagsamang gabay at tahasang demonstration mode, hindi kailanman ginamit bilang backup
tahimik kung sakaling mabigo ang pagtuklas.

## Ihagis

Gamit ang Rust **1.95+**, ang MSVC toolchain at Visual Studio C++ na mga tool sa Windows. Ang mga halimbawa ay gumagamit ng RTK upang i-filter ang output ng mga utos; nang walang RTK, alisin lang ang prefix na `rtk`. Tinatanggap ng script ng pamamahagi ang parehong mga kaso.

```powershell
rtk cargo run --release
```

Ang standalone na binary ay ginawa sa `target/release/san-usb-viewer.exe`. Ang `build.ps1` script ay lumilikha din ng pamamahagi sa `dist/`. Walang pagbabago sa driver o pag-install ng WinUSB ang kinakailangan para sa imbentaryo: ang application ay hindi nangangailangan ng mga interface ng device.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Dapat na umiiral ang patutunguhang folder para sa pagkuha. Ang bersyon ng release ay isang console-less Windows application; mas gusto ang `cargo run` sa pagbuo para sa imbentaryo ng command line.

## Bigyang-kahulugan ang mga resulta

Ang **negotiated speed** ay sa koneksyon, at hindi isang benchmark ng bilis ng pagkopya. Ang mga kapaki-pakinabang na bilis ay nakasalalay sa partikular sa protocol, ang peripheral at ang iba pang mga device na nagbabahagi ng bus. Ang mga bilis mula sa maraming mga aparato ay hindi nagdaragdag upang makalkula ang magagamit na bandwidth.

Ang isang passive USB cable ay karaniwang hindi isang enumerable device. Hindi binabasa ng bersyong ito ang e-marker nito, o ang kapangyarihan ng USB Power Delivery, o ang uri ng connector nito. Samakatuwid, hindi nito inaangkin na patunayan ang pinakamataas na bilis nito. Ang diagnosis ay batay sa naobserbahang link at, kung magagamit, sa mga tagapagpahiwatig ng Windows. Ang mabagal na link ay maaaring limitado ng cable, port, hub, o device. Ang numero ng bersyon ng USB descriptor ay hindi kailanman ginagamit bilang pinakamataas na bilis.

Ang kategoryang SuperSpeedPlus ay hindi nakikilala sa pagitan ng 10 at 20 Gb/s mode dito. Ang USB4/Thunderbolt at ang kanilang mga tunnel ay hindi siniyasat. Ang mga port na walang naka-attach na device ay hindi iniimbentaryo. Ang charging cable na walang data ay hindi gumagawa ng nakikitang device. Ang mga panloob na device (Bluetooth, built-in na camera, atbp.) ay maaaring lumabas sa USB tree.

Ang hindi naa-access na impormasyon ay ipinapakita bilang hindi magagamit. Ang tagagawa at mga interface ay hindi palaging nakalantad ng Windows. Maaaring hindi napapansin ang mga kaganapang ganap na nagaganap sa pagitan ng mga pag-scan. Ang log ay nasa memorya at magsisimula pagkatapos ng unang matagumpay na imbentaryo.

Inilalarawan ng mga identifier ng bus at port path ang topology ng system. Ang pisikal na connector ay maaaring magkaroon ng dalawang magkahiwalay na landas para sa USB 2 at SuperSpeed.

## Arkitektura at pagpapatunay

Ang `scanner.rs` ay gumaganap ng imbentaryo; Ang `windows_probe.rs` ay nagsa-encapsulate ng mga native read request mula sa mga hub; Ang `model.rs` ay naglalaman ng data at mga diagnostic. Pinangangasiwaan ng `app.rs` ang mga gawain sa background at pagpili; Ang `views.rs`, `inspector.rs`, `connection.rs` at `theme.rs` ay gumuhit ng interface gamit ang egui/eframe. Ang interface ay nananatiling interactive sa panahon ng pag-scan at pinapanatili ang huling matagumpay na pagbabasa kung nabigo ang susunod na pag-scan.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Ang mga aklatan ay cross-platform at ang Windows code ay may kondisyon. Ang mga bersyon ng macOS/Linux ay hindi napatunayan sa unang paghahatid na ito.

## Pagiging kompidensyal

Gumagana ang imbentaryo nang lokal, read-only, nang walang telemetry o mga serbisyo ng network. Ang mga link ng gabay ay bubukas lamang sa pag-click. Maaaring maglaman ang mga ulat ng JSON ng mga serial number at system identifier: nasa iyo ang pagbabahagi ng mga ito.

## Mga teknikal na sanggunian

- [nusb: USB enumeration at impormasyon](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

Lisensya ng MIT. Ang mga outbuildings ay nananatiling napapailalim sa kani-kanilang mga lisensya.

Ang pampublikong imbakan ay naglalaman lamang ng mga mapagkukunan, pagsasalin at libreng mga font na kinakailangan para sa pagsasama-sama. Ang mga pagbabasa ng USB, diagnostic capture, pag-export, lokal na kagustuhan, cache at development binary ay hindi kasama ng `.gitignore`. Upang mag-ulat ng problema, gamitin ang mode na `--demo` o alisin ang mga serial number at system ID sa iyong mga ulat bago ibahagi ang mga ito.

## Mga Wika — bersyon 0.2.0

Kasama sa application ang **50 interface na wika**. I-click ang code ng wika (`FR`, `EN`, atbp.) sa kanang bahagi sa itaas upang buksan ang switcher. Maaari kang maghanap ng isang wika sa pamamagitan ng katutubong pangalan nito, Ingles na pangalan o code nito. Ang pagbabago ay kaagad at ang pagpipilian ay nakaimbak sa `%APPDATA%/SanUsbViewer/preferences.json` sa ilalim ng Windows. Kung walang kagustuhan, ginagamit ang wika ng system; Ang isang hindi sinusuportahang wika ay bumabalik sa Ingles.

Ang pagpili ay pinapaboran ang pinakamaraming sinasalitang wika sa kabuuan (katutubong wika at pangalawang wika), na pinagsasama-sama ang mga variant ng rehiyon. Ito ay hindi isang numerical classification: ang mga resulta ay nag-iiba depende sa pagpapangkat ng mga dialect. Kasama sa mga variant na napili ang Simplified Chinese, Standard Arabic at ang mga Latin na script ng Fulani at Uzbek; Ginagamit ng Punjabi ang Gurmukhi.

English, Chinese, Hindi, Spanish, French, Arabic, Bengali, Portuguese, Russian, Urdu, Indonesian, German, Japanese, Marathi, Vietnamese, Telugu, Hausa, Turkish, Punjabi, Swahili, Filipino, Tamil, Persian, Korean, Amharic, Thai, Javanese, Italian, Gujarati, Kannada, Bhojpuri, Burmese, Polish, Yoruba, Linglaniala, Ukrainian, Fukudia, Ukrainian Pashto, Sundanese, Malay, Igbo, Nepali, Azeri, Maithili at Somali.

Naka-localize ang mga kinopyang menu, diagnostic, text ng tulong, mensahe ng aktibidad at detalye. Pinapanatili ang mga pangalan at identifier na ibinigay ng hardware at system. Pinapanatili ng JSON export ang schema at data ng imbentaryo nito, anuman ang wika ng display. Ang mga teknikal na unit ay nananatili sa **bit/s** (Mbit/s at Gbit/s) upang maiwasan ang pagkalito sa mga byte.

Ang mga katalogo ng Pranses at Ingles ay nagsisilbing sanggunian. Ang iba pang mga wika ay **awtomatikong pagsasalin**, na maaaring mangailangan ng mga linguistic na pagwawasto. Ang mga `locales/*.json` na mga file ay kasama sa compilation at maaaring itama at pagkatapos ay muling i-compile. Walang pagsasalin ng network na isinasagawa ng application: ang 50 wika at mga font ng Noto ay naka-embed, sa ilalim ng SIL Open Font License. Gumagamit ang Arabic text ng mga bidirectional na mga segment ng Unicode at mga bagong linya na kinakalkula bago i-format.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

Maaaring sumangguni ang `SAN_USB_VIEWER_CONFIG` sa isa pang file ng kagustuhan para sa pagsubok o isang portable na pag-install. Tanging ang code ng wika ang naka-save doon.
