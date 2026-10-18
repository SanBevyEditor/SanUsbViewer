# San USB Viewer

**Igbo** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Ntụgharị asụsụ igwe; anabatakwa ndozi.

** Ụdị dị ugbu a: `0.2.0` *** - ikike MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

USB Explorer nke ala edere na ** Rust ***, nwere interface gbara ọchịchịrị, topology a na-ahụ anya na nchọpụta enwere ike ịgụ. Ezubere ụdị mbụ maka Windows 10/11.

## Mmalite nke oru ngo

Vidiyo na mmalite nke ọrụ a: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Ọrụ a sitere na iji **WhatCable** na Apple na **USB Device Tree Viewer** na Windows: nghọta peripherals, hubs na ọsọ nke njikọ USB ha. Nke a bụ mmejuputa iwu nke nọọrọ onwe ya.

## Atụmatụ

- Nchọpụta ngwaọrụ USB na njikwa n'ezie site na `nusb`.
- Nhazi osisi site na ụgbọ ala na ụzọ ọdụ ụgbọ mmiri, nwere omimi oghere.
- Chọọ n'aha, nọmba serial ma ọ bụ VID:PID; USB 3+ nzacha na hub.
- Ọsọ njikọ mkparịta ụka: 1.5/12/480 Mb/s, 5Gb/s,
ma ọ bụ otu SuperSpeedPlus (10 Gb/s ma ọ bụ karịa).
- Eserese njem: kọmpụta, etiti etiti, njikọ na ngwaọrụ.
- N'okpuru Windows, ajụjụ ndị ọzọ nke hub site IOCTL iji chọpụta
SuperSpeed ike nke ngwaọrụ mara ọkwa. Ngwa dakọtara SuperSpeed ejikọrọ site na USB 2.0 na-arụpụta nchọpụta mmachi ụzọ.   A na-emeso ndị enyi USB 2 hubs nke ọdụ USB 3 iche iche ka ha zere ịkọrọ alaka ha 480 Mb/s maka mperi eriri.
- Ihe nchọpụta VID/PID, onye na-akọwa USB, nyochagharị, ọkwọ ụgbọ ala, serial na interfaces,
mgbe sistemụ na-ekpughe ozi a.
- Mmelite akpaaka kwa sekọnd atọ ọ bụla, ma ọ bụ akwụkwọ ntuziaka na F5.
- Ndekọ nnọkọ nke 200 ikpeazụ hụrụ mgbanwe.
- Bupụ nkwupụta zuru ezu na JSON (Ctrl+E) wee detuo nkọwa ndị ahọpụtara.
- Ntuziaka agbakwunyere na ụdị ngosi doro anya, ejighi ya dị ka ndabere
gbachi nkịtị ma ọ bụrụ na achọpụtaghị ọdịda.

## Tufuo

Site na Rust **1.95+**, ngwa ọrụ MSVC na Visual Studio C++ na Windows. Ihe atụ ndị a na-eji RTK nyochaa mmepụta nke iwu; na-enweghị RTK, wepụ nnọọ `rtk` prefix. Edemede nkesa na-anabata ikpe abụọ ahụ.

```powershell
rtk cargo run --release
```

Emepụtara ọnụọgụ abụọ kwụ ọtọ na `target/release/san-usb-viewer.exe`. Edemede `build.ps1` na-emepụtakwa nkesa na `dist/SanUsbViewer/`. Enweghị mgbanwe ọkwọ ụgbọ ala ma ọ bụ ntinye nke WinUSB dị mkpa maka ngwa ahịa: ngwa ahụ anaghị achọ oghere ngwaọrụ.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Ebe nchekwa ebe a ga-eji weghara ga-adịrịrị. Ụdị ntọhapụ bụ ngwa Windows enweghị njikwa; masịrị `cargo run` na mmepe maka ngwa ngwa ahịrị iwu.

## Kọwaa nsonaazụ ya

Ọsọ nke mkparịta ụka ** bụ nke njikọ, ọ bụghịkwa akara nke ọsọ nnomi. Ọsọ ọsọ bara uru dabere karịsịa na protocol, akụkụ na ngwaọrụ ndị ọzọ na-ekerịta bọs. Ọsọ ọsọ sitere na ngwaọrụ dị iche iche anaghị agbakwunye iji gbakọọ bandwit dị.

Igwe USB na-agafe agafe abụghị ngwaọrụ a na-apụghị ịgụta ọnụ. Ụdị a anaghị agụ akara e-marker ya, ma ọ bụ ike nnyefe ike USB, ma ọ bụ ụdị njikọ ya. Ya mere ọ naghị ekwu na ọ na-akwado oke ọsọ ya. Nchọpụta ahụ dabere na njikọ a hụrụ na, ọ bụrụ na ọ dị, na ihe ngosi Windows. Enwere ike iji eriri, ọdụ ụgbọ mmiri, hub, ma ọ bụ ngwaọrụ machie njikọ dị nwayọ. A naghị eji nọmba ụdị nkọwa USB dị ka oke ọsọ.

Ụdị SuperSpeedPlus anaghị ama ọdịiche dị n'etiti ụdị 10 na 20 Gb/s ebe a. A naghị enyocha USB4/ Thunderbolt na ọwara ha. Ejikọghị ọdụ ụgbọ mmiri na-enweghị ngwaọrụ. Igwe nchaji na-enweghị data anaghị emepụta ngwaọrụ a na-ahụ anya. Ngwa ime (Bluetooth, igwefoto arụnyere, wdg) nwere ike ịpụta na osisi USB.

E gosipụtara ozi enweghị ike ịnweta dị ka adịghị. Windows anaghị ekpughe ndị nrụpụta na interfaces mgbe niile. Ihe omume na-eme kpamkpam n'etiti nyocha nwere ike ghara ịhụta ya. Ndekọ ahụ dị na ebe nchekwa wee malite mgbe ngwa ahịa gara nke ọma nke mbụ gachara.

Ihe nchọpụta ụgbọ ala na ụzọ ọdụ ụgbọ mmiri na-akọwa usoro topology. Njikọ anụ ahụ nwere ike ịnwe ụzọ abụọ dị iche iche maka USB 2 na SuperSpeed.

## Nhazi na nkwado

`scanner.rs` na-eme ngwa ahịa; `windows_probe.rs` na-ekpuchi arịrịọ ọgụgụ obodo sitere na ọdụ; `model.rs` nwere data na nchọpụta nchọpụta. `app.rs` na-ejikwa ọrụ ndabere na nhọrọ; `views.rs`, `inspector.rs`, `connection.rs` na `theme.rs` na-ese interface na egui/eframe. Ihe interface ahụ na-anọgide na-emekọrịta ihe n'oge nyocha ma na-ejigide agụ nke ọma nke ikpeazụ ma ọ bụrụ na nyocha ọzọ daa.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Ọbá akwụkwọ ndị a na-agafe agafe na koodu Windows nwere ọnọdụ. Ụdị macOS/Linux akwadoghị na mbufe mbụ a.

## Nzuzo

Ndepụta ngwaahịa na-arụ ọrụ na mpaghara, naanị ọgụgụ, na-enweghị telemetry ma ọ bụ ọrụ netwọk. Njikọ ntuziaka na-emeghe naanị na pịa. Akụkọ JSON nwere ike ịnwe ọnụọgụ nọmba yana ihe nchọpụta sistemụ: ikesa ha dịịrị gị.

## Nrụtụ aka nka

- [nusb: USB ngụkọ na ozi](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

ikike MIT. Ụlọ ndị ahụ dị n'èzí na-anọgide na-adabere na ikikere nke ha.

Ebe nchekwa ọha nwere naanị isi mmalite, ntụgharị asụsụ na mkpụrụedemede efu dị mkpa maka mkpokọta. `.gitignore` ewepụghị ọgụgụ USB, njide nyocha, mbupụ, mmasị mpaghara, cache na ọnụọgụ mmepe. Iji kọọ nsogbu, jiri ọnọdụ `--demo` ma ọ bụ wepụ nọmba nsonazụ na NJ sistemụ na akụkọ gị tupu ikesa ha.

## Asụsụ - ụdị 0.2.0

Ngwa a gụnyere ** asụsụ interface 50 ***. Pịa koodu asụsụ (`FR`, `EN`, wdg) n'elu aka nri iji mepee switcher. Ị nwere ike ịchọ asụsụ site na aha obodo ya, aha Bekee ma ọ bụ koodu ya. Mgbanwe a bụ ozugbo ma echekwara nhọrọ na `%APPDATA%/SanUsbViewer/preferences.json` n'okpuru Windows. Enweghị mmasị, a na-eji asụsụ usoro; Asụsụ anaghị akwado na-alaghachi na Bekee.

Nhọrọ a na-akwado asụsụ ndị a kacha asụ na mkpokọta (asụsụ nne na asụsụ nke abụọ), na-achịkọta ọnụ ọgụgụ mpaghara. Nke a abụghị nhazi ọnụọgụgụ: nsonaazụ ya dịgasị iche dabere na nchịkọta nke olumba. Ụdị dị iche iche ahọpụtara gụnyere Simplified Chinese, Standard Arabic na Latin script nke Fulani na Uzbek; Punjabi na-eji Gurmukhi.

Bekee, Chinese, Hindi, Spanish, French, Arabic, Bengali, Portuguese, Russian, Urdu, Indonesian, German, Japanese, Marathi, Vietnamese, Telugu, Hausa, Turkish, Punjabi, Swahili, Filipino, Tamil, Persian, Korean, Amharic, Thai, Javanese, Italian, Gujarati, Kannada, Bhojpuri, Burmese, Polish, Yoruba, Lingalam, Oromo, Fulanin, Sinịla, Oromo, Fulani, Oroma, Oroma, Tamil, Persian Uzbek, Pashto, Sundanese, Malay, Igbo, Nepali, Azeri, Maithili na Somali.

A na-ahazi menus, nchọpụta nchọpụta, ederede enyemaka, ozi mmemme na nkọwapụta. Aha na ihe nchọpụta nke ngwaike na sistemu wetara ka edobere. Mbupụ JSON na-edobe atụmatụ ya na data ngwa ahịa ya, n'agbanyeghị asụsụ ngosi. Ngalaba nka na ụzụ na-anọgide na **bit/s** (Mbit/s na Gbit/s) iji zere mgbagwoju anya na bytes.

Katalọgụ French na Bekee na-eje ozi dịka ntụaka. Asụsụ ndị ọzọ bụ ** ntụgharị asụsụ akpaka ***, nke nwere ike ịchọ mmezi asụsụ. Agụnyere faịlụ `locales/*.json` na nchịkọta a nwere ike mezie ya wee chịkọta ya. Enweghị nsụgharị netwọkụ nke ngwa a na-eme: agbakwunyere asụsụ 50 na Noto Font n'okpuru SIL Open Font License. Ederede Arabik na-eji akụkụ Unicode bidirectional na ahịrị ọhụrụ gbakọọ tupu nhazi.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` nwere ike na-ezo aka na faịlụ mmasị ọzọ maka nnwale ma ọ bụ nrụnye mbugharị. Naanị koodu asụsụ ka echekwara ebe ahụ.
