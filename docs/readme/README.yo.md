# San USB Viewer

**Yorùbá** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Awọn itumọ ẹrọ; awọn atunṣe wa kaabo.

** Ẹya lọwọlọwọ: `0.2.0` *** - iwe-aṣẹ MIT.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Abinibi USB Explorer ti a kọ sinu ** Rust ***, pẹlu wiwo dudu, topology wiwo ati awọn iwadii kika. Ẹya akọkọ ti a pinnu fun Windows 10/11.

## Oti ti ise agbese

Fidio ni ipilẹṣẹ ti iṣẹ akanṣe: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Ise agbese na ni atilẹyin nipasẹ awọn lilo ti ** WhatCable *** lori Apple ati ** USB Device Tree Viewer *** lori Windows: oye awọn agbeegbe, awọn ibudo ati awọn iyara ti awọn asopọ USB wọn. Eleyi jẹ ẹya ominira imuse.

## Awọn ẹya ara ẹrọ

- Wiwa gidi ti awọn ẹrọ USB ati awọn oludari nipasẹ `nusb`.
- Eto igi nipasẹ ọkọ akero ati ọna ibudo, pẹlu ijinle ibudo.
- Ṣewadii pẹlu orukọ, nọmba ni tẹlentẹle tabi VID:PID; USB 3+ Ajọ ati hobu.
- Iyara ọna asopọ idunadura: 1.5/12/480 Mb/s, 5 Gb/s,
tabi ẹka SuperSpeedPlus (10 Gb/s tabi diẹ ẹ sii).
- Aworan irin-ajo: kọnputa, awọn ibudo agbedemeji, ọna asopọ ati ẹrọ.
- Labẹ Windows, afikun ifọrọwanilẹnuwo ti ibudo nipasẹ IOCTL lati wa
Awọn agbara SuperSpeed ti a kede nipasẹ ẹrọ naa. Ẹrọ ibaramu SuperSpeed ti a ti sopọ nipasẹ USB 2.0 ṣe agbejade iwadii aropin ọna.   Awọn ẹlẹgbẹ ibudo USB 2 ti awọn ibudo USB 3 ni a tọju ni lọtọ lati yago fun ikalara ẹka 480 Mb/s wọn si aṣiṣe okun kan.
- VID/PID idamo, USB apejuwe, àtúnyẹwò, awakọ, tẹlentẹle ati awọn atọkun,
nigbati eto ba ṣafihan alaye yii.
- Imudojuiwọn aifọwọyi ni gbogbo iṣẹju-aaya mẹta, tabi afọwọṣe pẹlu F5.
- Iwe akọọlẹ igba ti 200 ti o kẹhin ti ṣe akiyesi awọn ayipada.
- Gbejade alaye pipe ni JSON (Ctrl+E) ki o daakọ awọn alaye ti o yan.
- Itọsọna iṣọpọ ati ipo ifihan gbangba, ko lo bi afẹyinti
ipalọlọ ninu ọran ikuna wiwa.

## Jabọ

Pẹlu Rust ** 1.95+**, ohun elo irinṣẹ MSVC ati awọn irinṣẹ Studio Visual C ++ lori Windows. Awọn apẹẹrẹ lo RTK lati ṣe àlẹmọ iṣẹjade ti awọn aṣẹ; lai RTK, o kan yọ `rtk` ìpele. Iwe afọwọkọ pinpin gba awọn ọran mejeeji.

```powershell
rtk cargo run --release
```

Alakomeji imurasilẹ jẹ iṣelọpọ ni `target/release/san-usb-viewer.exe`. Iwe afọwọkọ `build.ps1` tun ṣẹda pinpin ni `dist/`. Ko si iyipada awakọ tabi fifi sori ẹrọ ti WinUSB jẹ pataki fun akojo oja: ohun elo ko nilo awọn atọkun ẹrọ.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Awọn ibi-ipo folda fun a Yaworan gbọdọ tẹlẹ. Ẹya itusilẹ jẹ ohun elo Windows ti ko ni console; fẹ `cargo run` ni idagbasoke fun akojo laini aṣẹ.

## Tumọ awọn esi

Iyara idunadura ** jẹ ti asopọ, kii ṣe ala ti iyara ẹda. Awọn iyara iwulo gbarale ni pataki lori ilana, agbeegbe ati awọn ẹrọ miiran pinpin ọkọ akero. Awọn iyara lati awọn ẹrọ lọpọlọpọ ko ṣe afikun lati ṣe iṣiro bandiwidi to wa.

Okun USB palolo ni gbogbogbo kii ṣe ẹrọ ti a le loye. Ẹya yii ko ka e-ami rẹ, tabi agbara Ifijiṣẹ Agbara USB, tabi iru asopo rẹ. Nitorinaa ko beere lati jẹrisi iyara ti o pọju. Ayẹwo naa da lori ọna asopọ ti a ṣe akiyesi ati, ti o ba wa, lori awọn afihan Windows. Ọna asopọ lọra le ni opin nipasẹ okun, ibudo, ibudo, tabi ẹrọ. Nọmba ikede USB ti ikede ko ni lo bi iyara to pọ julọ.

Ẹka SuperSpeedPlus ko ṣe iyatọ laarin awọn ipo 10 ati 20 Gb/s nibi. USB4/Thunderbolt ati awọn tunnels wọn ko ṣe ayẹwo. Awọn ebute oko oju omi ti ko ni ẹrọ ti o somọ ko ni inventoried. Okun gbigba agbara laisi data ko ṣẹda ẹrọ ti o han. Awọn ẹrọ inu (Bluetooth, kamẹra ti a ṣe sinu, ati bẹbẹ lọ) le han ninu igi USB.

Alaye ti ko le wọle jẹ afihan bi ko si. Olupese ati awọn atọkun kii ṣe afihan nigbagbogbo nipasẹ Windows. Awọn iṣẹlẹ ti o waye patapata laarin awọn ọlọjẹ le ma ṣe akiyesi. Iwe akọọlẹ wa ni iranti ati bẹrẹ lẹhin atokọ aṣeyọri akọkọ.

Awọn idamo akero ati awọn ọna ibudo ṣe apejuwe topology eto. Asopọmọra ti ara le ni awọn ọna lọtọ meji fun USB 2 ati SuperSpeed.

## Faaji ati afọwọsi

`scanner.rs` ṣe ​​akojo oja; `windows_probe.rs` ṣe ​​akojọpọ awọn ibeere kika abinibi lati awọn ibudo; `model.rs` ni data ninu ati awọn iwadii aisan. `app.rs` mu awọn iṣẹ-ṣiṣe lẹhin ati aṣayan; `views.rs`, `inspector.rs`, `connection.rs` ati `theme.rs` fa wiwo pẹlu egui/eframe. Ni wiwo si maa wa ibanisọrọ nigba kan ọlọjẹ ati ki o da duro awọn ti o kẹhin aseyori kika ti o ba ti nigbamii ti ọlọjẹ kuna.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Awọn ile-ikawe jẹ pẹpẹ-agbelebu ati koodu Windows jẹ ipo. Awọn ẹya macOS/Linux ko ni ifọwọsi ni ifijiṣẹ akọkọ yii.

## Asiri

Oja ṣiṣẹ ni agbegbe, kika-nikan, laisi telemetry tabi awọn iṣẹ nẹtiwọọki. Awọn ọna asopọ itọsọna ṣii nikan ni titẹ. Awọn ijabọ JSON le ni awọn nọmba ni tẹlentẹle ati awọn idamọ eto: pinpin wọn wa fun ọ.

## Awọn itọkasi imọ-ẹrọ

- [nusb: USB enumeration ati alaye](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui / eframe](https://github.com/emilk/egui)

MIT iwe-ašẹ. Awọn ile ita wa labẹ awọn iwe-aṣẹ oniwun wọn.

Ibi ipamọ gbogbo eniyan ni awọn orisun nikan, awọn itumọ ati awọn nkọwe ọfẹ ti o ṣe pataki fun akopọ. Awọn kika USB, awọn igbasilẹ iwadii aisan, awọn okeere, awọn ayanfẹ agbegbe, awọn caches ati awọn alakomeji idagbasoke jẹ imukuro nipasẹ `.gitignore`. Lati jabo iṣoro kan, lo ipo `--demo` tabi yọ awọn nọmba ni tẹlentẹle ati awọn ID eto kuro ninu awọn ijabọ rẹ ṣaaju pinpin wọn.

## Awọn ede - ẹya 0.2.0

Ohun elo naa pẹlu ** awọn ede wiwo 50 ***. Tẹ koodu ede (`FR`, `EN`, ati bẹbẹ lọ) ni apa ọtun oke lati ṣii switcher. O le wa ede kan nipasẹ orukọ abinibi rẹ, orukọ Gẹẹsi rẹ tabi koodu rẹ. Iyipada naa jẹ lẹsẹkẹsẹ ati yiyan ti wa ni ipamọ ni `%APPDATA%/SanUsbViewer/preferences.json` labẹ Windows. Laisi ààyò, ede eto ti lo; Ede ti ko ni atilẹyin pada si Gẹẹsi.

Aṣayan ṣe ojurere awọn ede ti a sọ julọ ni apapọ (ede iya ati ede keji), ṣiṣe akojọpọ awọn iyatọ agbegbe. Eyi kii ṣe isọdi oni nọmba: awọn abajade yatọ si da lori ṣiṣe akojọpọ awọn oriṣi. Awọn iyatọ ti a yan pẹlu Kannada Simplified, Standard Arabic ati awọn iwe afọwọkọ Latin ti Fulani ati Uzbek; Punjabi nlo Gurmukhi.

English, Chinese, Hindi, Spanish, French, Arabic, Bengali, Portuguese, Russian, Urdu, Indonesian, German, Japanese, Marathi, Vietnamese, Telugu, Hausa, Turkish, Punjabi, Swahili, Filipino, Tamil, Persian, Korean, Amharic, Thai, Javanese, Italian, Gujarati, Kannada, Bhojpuri, Burmese, Polish, Yoruba, Maladhikraini, Oromo, Fulanian, Sinini, Oromo, Fulani, Sinini, Oromo, Fulani, Swahili, Sinini, Oromo, Fulani, Swahili, Sinini, Oromo, Fulani, Swahili. Uzbek, Pashto, Sundanese, Malay, Igbo, Nepali, Azeri, Maithili ati Somali.

Awọn akojọ aṣayan ti a daakọ, awọn iwadii aisan, awọn ọrọ iranlọwọ, awọn ifiranṣẹ iṣẹ ṣiṣe ati awọn alaye ti wa ni agbegbe. Awọn orukọ ati awọn idamọ ti a pese nipasẹ ohun elo ati eto ti wa ni idaduro. Ijajajajaja JSON ntọju eto ati data akojo oja rẹ, laibikita ede ifihan. Awọn ẹya imọ-ẹrọ wa ni ** bit/s *** (Mbit/s ati Gbit/s) lati yago fun iporuru pẹlu awọn baiti.

Awọn katalogi Faranse ati Gẹẹsi ṣiṣẹ bi itọkasi kan. Awọn ede miiran jẹ ** awọn itumọ aladaaṣe ***, eyiti o le nilo awọn atunṣe ede. Awọn faili `locales/*.json` wa ninu akopọ ati pe o le ṣe atunṣe lẹhinna tun ṣe akopọ. Ko si itumọ nẹtiwọọki ti a ṣe nipasẹ ohun elo: awọn ede 50 ati awọn akọwe Noto ti wa ni ifibọ, labẹ Iwe-aṣẹ Font Ṣii SIL. Ọrọ Larubawa nlo awọn apa Unicode bidirectional ati awọn laini tuntun ti a ṣe iṣiro ṣaaju kika.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` le tọka si faili awọn ayanfẹ miiran fun idanwo tabi fifi sori ẹrọ to ṣee gbe. koodu ede nikan ni o wa ni ipamọ nibẹ.
