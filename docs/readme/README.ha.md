# San USB Viewer

**Hausa** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Fassarar inji; muna maraba da gyara.

** Sigar yanzu: `0.2.0` *** - lasisin MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

USB Explorer na asali wanda aka rubuta a cikin ** Tsatsa ***, tare da dubawa mai duhu, yanayin gani da bincike mai karantawa. Sigar farko da aka yi niyya don Windows 10/11.

## Asalin aikin

Bidiyo a asalin aikin: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

An yi wahayi zuwa aikin ta hanyar amfani da **WhatCable** akan Apple da **USB Device Tree Viewer** akan Windows: fahimtar abubuwan da ke kewaye, cibiyoyi da saurin haɗin kebul ɗin su. Wannan aiwatarwa ne mai zaman kansa.

## Siffofin

- Gano ainihin na'urorin USB da masu sarrafawa ta hanyar `nusb`.
- Tsarin bishiyar ta hanyar bas da hanyar tashar jiragen ruwa, tare da zurfin cibiya.
- Bincika da suna, serial number ko VID:PID; USB 3+ tacewa da cibiyoyi.
- Gudun hanyar haɗin gwiwa: 1.5/12/480 Mb/s, 5Gb/s,
ko nau'in SuperSpeedPlus (10 Gb/s ko fiye).
- Zane na tafiya: kwamfuta, matsakaita cibiya, mahada da na'ura.
- A ƙarƙashin Windows, ƙarin tambayoyin cibiyar ta IOCTL don gano
SuperSpeed damar da na'urar ta sanar. Na'urar da ta dace ta SuperSpeed da aka haɗa ta USB 2.0 tana samar da ƙayyadaddun tantancewar hanya.   Ana kula da abokan haɗin kebul na USB 2 na cibiyoyin USB 3 daban don gujewa danganta reshen su na 480 Mb/s zuwa laifin kebul.
- Masu gano VID/PID, mai siffanta USB, bita, direba, serial da musaya,
lokacin da tsarin ya fallasa wannan bayanin.
- Sabunta atomatik kowane daƙiƙa uku, ko jagora tare da F5.
- Littafin zama na 200 na ƙarshe da aka lura da canje-canje.
- Fitar da cikakkiyar sanarwa a JSON (Ctrl+E) kuma kwafi bayanan da aka zaɓa.
- Haɗin jagora da yanayin nunawa, ba a taɓa amfani da shi azaman madadin ba
shiru idan aka kasa ganowa.

## Jifa

Tare da Tsatsa **1.95+**, kayan aikin MSVC da kayan aikin Kayayyakin Kayayyakin Kayayyakin C++ akan Windows. Misalan suna amfani da RTK don tace fitarwa na umarni; ba tare da RTK ba, kawai cire prefix na `rtk`. Rubutun rarraba yana karɓar lokuta biyu.

```powershell
rtk cargo run --release
```

Ana samar da binary na tsaye a cikin `target/release/san-usb-viewer.exe`. Rubutun `build.ps1` kuma yana ƙirƙirar rarrabawa a cikin `dist/SanUsbViewer/`. Babu gyare-gyaren direba ko shigarwa na WinUSB da ya zama dole don ƙira: aikace-aikacen baya buƙatar mu'amalar na'ura.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Dole ne babban fayil ɗin da aka nufa don ɗauka ya kasance. Sigar sakin aikace-aikacen Windows ne mara ƙarancin na'ura; fi son `cargo run` don haɓakawa don lissafin layin umarni.

## Fassara sakamakon

* Gudun shawarwarin ** shine na haɗin gwiwa, kuma ba ma'auni na saurin kwafi ba. Gudun gudu masu amfani sun dogara musamman akan ƙa'idar, na gefe da sauran na'urorin da ke raba bas ɗin. Gudun gudu daga na'urori da yawa ba sa ƙarawa don ƙididdige yawan bandwidth samuwa.

Kebul na USB mai wucewa gabaɗaya ba na'ura ce mai ƙima ba. Wannan sigar baya karanta alamar e-alamar sa, ko ikon Isar da Wutar USB, ko nau'in haɗin sa. Don haka baya da'awar tabbatar da iyakar saurin sa. Sakamakon ganewar asali ya dogara ne akan hanyar haɗin da aka lura kuma, idan akwai, akan alamun Windows. Ana iya iyakance jinkirin hanyar haɗi ta kebul, tashar jiragen ruwa, cibiya, ko na'ura. Lambar sigar kebul ɗin ba a taɓa amfani da ita azaman matsakaicin gudun ba.

Rukunin SuperSpeedPlus ba ya bambanta tsakanin 10 da 20 Gb/s a nan. USB4/Thunderbolt da ramukan su ba a duba su. Ba a ƙirƙira tashar jiragen ruwa ba tare da na'ura ba. Kebul na caji ba tare da bayanai ba baya ƙirƙirar na'urar bayyane. Na'urorin ciki (Bluetooth, ginanniyar kyamara, da sauransu) na iya bayyana a cikin bishiyar USB.

Ana nuna bayanan da ba za a iya samu ba kamar yadda babu shi. Masu kera da musaya ba koyaushe suke fallasa ta Windows ba. Abubuwan da ke faruwa gaba ɗaya tsakanin dubawa na iya wucewa ba a gane su ba. Log din yana cikin ƙwaƙwalwar ajiya kuma yana farawa bayan ƙididdigar nasara ta farko.

Masu gano bas da hanyoyin tashar jiragen ruwa sun bayyana yanayin tsarin. Mai haɗin jiki zai iya samun hanyoyi daban-daban don USB 2 da SuperSpeed.

## Gine-gine da tabbatarwa

`scanner.rs` yana yin kaya; `windows_probe.rs` yana ɗaukar buƙatun karantawa na asali daga cibiyoyi; `model.rs` ya ƙunshi bayanai da bincike. `app.rs` yana sarrafa ayyuka na baya da zaɓi; `views.rs`, `inspector.rs`, `connection.rs` da `theme.rs` zana mu'amala tare da egui/eframe. Keɓancewar za ta kasance mai mu'amala yayin dubawa kuma tana riƙe karatun nasara na ƙarshe idan binciken na gaba ya gaza.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Laburaren giciye-dandamali ne kuma lambar Windows tana da sharadi. Ba a inganta nau'ikan macOS/Linux a wannan isar da farko ba.

## Asiri

Inventory yana aiki a gida, karantawa kawai, ba tare da sabis na telemetry ko cibiyar sadarwa ba. Hanyoyin haɗin jagora suna buɗewa kawai a danna. Rahoton JSON na iya ƙunsar jerin lambobi da masu gano tsarin: raba su ya rage na ku.

## Bayanan fasaha

- Saukewa: [nusb: kebul na lissafin da bayanai](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- Saukewa: [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- Saukewa: [egui/eframe](https://github.com/emilk/egui)

lasisin MIT. Gine-ginen sun kasance ƙarƙashin lasisin nasu.

Ma'ajiyar jama'a ta ƙunshi tushe kawai, fassarori da maƙallan rubutu masu kyauta waɗanda ake buƙata don haɗawa. `.gitignore` ba a cire karatun USB, kama bincike, fitarwa, abubuwan da ake so na gida, caches da binaries na ci gaba. Don ba da rahoton matsala, yi amfani da yanayin `--demo` ko cire serial lambobi da tsarin ID daga rahotannin ku kafin raba su.

## Harsuna - sigar 0.2.0

Aikace-aikacen ya haɗa da ** 50 harsunan dubawa ***. Danna lambar yare (`FR`, `EN`, da sauransu) a saman dama don buɗe mai sauyawa. Kuna iya nemo harshe ta asalin sunan sa, sunansa na Ingilishi ko lambar sa. Canjin yana nan take kuma ana adana zaɓin a cikin `%APPDATA%/SanUsbViewer/preferences.json` ƙarƙashin Windows. Ba tare da fifiko ba, ana amfani da harshen tsarin; Harshen mara tallafi yana komawa Turanci.

Zaɓin ya fi son yaren da aka fi magana gabaɗaya (harshen uwa da na biyu), tare da haɗa bambance-bambancen yanki. Wannan ba rabe-rabe ba ne na lambobi: sakamakon ya bambanta dangane da yadda ake tara yaruka. Bambance-bambancen da aka zaɓa sun haɗa da Sauƙaƙen Sinanci, Daidaitaccen Larabci da rubutun Latin na Fulani da Uzbek; Punjabi yana amfani da Gurmukhi.

Turanci, Sinanci, Hindi, Mutanen Espanya, Faransanci, Larabci, Bengali, Portuguese, Rashanci, Urdu, Indonesian, Jamusanci, Jafananci, Marathi, Vietnamese, Telugu, Hausa, Baturke, Punjabi, Swahili, Filipino, Tamil, Persian, Korean, Amharic, Thai, Javanese, Italiyanci, Gujarati, Kannada, Bhojpuri, Burmese, Yaren mutanen Poland, Yarbanci, Sinanci, Sinanci, Oroma, Fulani, Oroma, Fulani, Sinanci, Oroma, Fulani, Sinanci, Oroma, Fulani, Yabanci, Sinanci, Oroma, Fulani, Oroma, Fulani, Sinanci, Oroma, Fulani, Yabanci, Sinanci, Oroma, Fulani, Oroma, Fulani, Yoruba, Sinanci, Oromo, Fulani, Oroma, Fulani, Sinanci, Oroma, Oroma, Fulanin, Yoruba, Lingalayalam, Oromo, Oroma, Fulani, Oroma, Fulani, Bafuladiya, Sinanci, Oroma, Bafuladiya. Uzbek, Pashto, Sundanese, Malay, Igbo, Nepali, Azeri, Maithili da Somaliya.

Menu da aka kwafi, bincike, rubutun taimako, saƙonnin ayyuka da cikakkun bayanai suna cikin gida. Sunaye da masu ganowa da kayan aiki da tsarin suka bayar ana riƙe su. Fitarwar JSON tana kiyaye tsarinta da bayanan ƙira, ba tare da la'akari da yaren nuni ba. Raka'o'in fasaha sun kasance a cikin **bit/s** (Mbit/s da Gbit/s) don guje wa rudani da bytes.

Kas ɗin Faransanci da Ingilishi suna aiki azaman abin tunani. Sauran harsuna sune ** fassarorin atomatik ***, waɗanda ƙila suna buƙatar gyare-gyaren harshe. Ana haɗa fayilolin `locales/*.json` a cikin haɗawa kuma ana iya gyara su sannan a sake haɗa su. Babu fassarar hanyar sadarwa da aikace-aikacen ke aiwatarwa: harsuna 50 da Noto fonts an saka su a ƙarƙashin Lasisin Buɗaɗɗen Rubutun SIL. Rubutun Larabci yana amfani da sassan Unicode guda biyu da sabbin layukan da aka lissafta kafin tsarawa.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` na iya komawa zuwa wani fayil ɗin zaɓi don gwaji ko shigarwa mai ɗaukuwa. Lambar yare kawai aka ajiye a wurin.
