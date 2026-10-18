# San USB Viewer

**Soomaali** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Turjumaada mashiinka; sixitaanka waa la soo dhawaynayaa.

** Nooca hadda: `0.2.0` *** - shatiga MIT.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Dhaladka USB Explorer oo ku qoran **Rust**, oo leh isdhexgal mugdi ah, muuqaal muuqaal ah iyo ogaanshaha la akhriyi karo. Nooca koowaad ee loogu talagalay Windows 10/11.

## Asalka mashruuca

Fiidiyowga asalka ah ee mashruuca: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Mashruucu waxa dhiirigeliyay adeegsiga **WhatCable** ee Apple iyo **USB Device Tree Viewer** ee Daaqadaha: Fahamka meelaha durugsan, xarumaha iyo xawaaraha isku xirka USB-ga. Tani waa hirgalin madax banaan.

## Astaamaha

- Ogaanshaha dhabta ah ee aaladaha USB iyo kontaroolayaasha iyada oo loo sii marayo `nusb`.
- Dhismaha geedka baska iyo dariiqa dekeddu, oo leh qoto dheer.
- Ku raadi magaca, lambarka taxan ama VID:PID; USB 3+ filter iyo hub.
- Xawaaraha isku xirka laga gorgortamay: 1.5/12/480 Mb/s, 5 Gb/s,
ama qaybta SuperSpeedPlus (10 Gb/s ama ka badan).
- Jaantuska safarka: kumbuyuutarka, xarumaha dhexe, isku xirka iyo qalabka.
- Daaqadaha hoostiisa, su'aalo dheeraad ah oo xudunta u ah IOCTL si loo ogaado
Awoodaha SuperSpeed ku dhawaaqay qalabku. Aaladda ku habboon ee SuperSpeed ee ku xidhan USB 2.0 waxay soo saartaa xaddidaadda ogaanshaha dariiqa.   Xubnaha USB 2 ee xarumaha USB 3 si gooni ah ayaa loola dhaqmaa si looga fogaado in loo aaneeyo laantooda 480 Mb/s cillad fiilada.
- Aqoonsayaasha VID/PID, sharaxe USB, dib u eegis, darawal, taxane iyo is dhexgalyo,
marka nidaamku soo bandhigo macluumaadkan.
- Cusbooneysiis toos ah saddexdii ilbiriqsi kasta, ama buug-gacmeed leh F5.
- Diiwaanka fadhiga 200 ee u dambeeyay isbeddellada la arkay.
- Ku Dhoofi bayaanka dhamaystiran ee JSON (Ctrl+E) oo koobiyi tafaasiisha la doortay.
- Tilmaamaha isku dhafan iyo qaabka muujinta cad, oo aan waligood loo isticmaalin kayd ahaan
aamusnaan haddii ay dhacdo in la ogaado.

## Tuur

Rust **1.95+**, Qalabka MSVC iyo Visual Studio C++ ee Windows. Tusaalooyinka waxay isticmaalaan RTK si ay u shaandheeyaan wax soo saarka amarrada; RTK la'aanteed, kaliya ka saar horgalaha `rtk`. Qoraalka qaybinta ayaa aqbalaya labada xaaladood.

```powershell
rtk cargo run --release
```

Binary-ga goonida ah waxaa lagu soo saaray `target/release/san-usb-viewer.exe`. Qoraalka `build.ps1` wuxuu kaloo abuuraa qaybinta `dist/`. Ma jiro wax ka beddelka darawalka ama rakibidda WinUSB lagama maarmaan u ah agabka: codsigu uma baahna interfaces aaladda.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Galka loo socdo ee qabashada waa inuu jiraa. Nooca sii dayntu waa codsi Windows-ka-console-yar; door bidaan `cargo run` horumarinta alaabada khadka taliska.

## Turjumaan natiijooyinka

** Xawaaraha laga wada xaajooday *** waa kan isku xirka, ee ma aha halbeegga xawaaraha koobiga. Xawaaraha waxtarka leh waxay si gaar ah ugu xiran yihiin borotokoolka, durugsan iyo aaladaha kale ee wadaaga baska. Xawaaraha aaladaha kala duwan kuma jiraan xisaabinta xawliga la heli karo.

Xadhkaha USB-ga ee dadban guud ahaan maaha qalab la tirin karo. Noocani ma akhriyo e-markerkiisa, ama awoodda Bixinta Awoodda USB, iyo nooca xidhiidhiyaha. Sidaa darteed ma sheeganayso inay caddayso xawligeeda ugu sarreeya. Cilad-sheegashadu waxay ku salaysan tahay xidhiidhka la arkay iyo, haddii la heli karo, tilmaamayaasha Windows. Xidhiidhiyaha gaabis ah waxa laga yaabaa inuu xaddido fiilada, dekedda, xuddunta, ama qalabka. Lambarka nooca USB-ga looma isticmaalo sida xawaaraha ugu sarreeya.

Qaybta SuperSpeedPlus kama soocayso 10 iyo 20 Gb/s hababka halkan. USB4/Thunderbolt iyo tunneladooda lama baaro. Dekadaha aan qalab ku xidhnayn lama ikhtiraaco. Fiilada dallacaadda oo aan lahayn xog ma abuurto qalab muuqda. Qalabka gudaha (Bluetooth, kamarad ku dhex dhisan, iwm.) ayaa laga yaabaa inay ka soo muuqdaan geedka USB.

Macluumaadka aan la heli karin ayaa loo soo bandhigay sidii aan la heli karin. Soo saaraha iyo is-dhexgalayaashu had iyo jeer ma muujiyaan Windows. Dhacdooyinka ka dhaca gebi ahaanba inta u dhaxaysa baarista ayaa laga yaabaa inaan la ogaan. Logu wuxuu ku jiraa xusuusta wuxuuna bilaabmaa ka dib alaabtii ugu horreysay ee guulaysata.

Aqoonsayaasha baska iyo waddooyinka dekeddu waxay qeexayaan nidaamka topology-ga. Xidhiidhiyaha jireed wuxuu yeelan karaa laba waddo oo kala duwan oo loogu talagalay USB 2 iyo SuperSpeed.

## Dhismaha iyo ansixinta

`scanner.rs` waxay qabataa alaab; `windows_probe.rs` waxay koobaysaa codsiyada akhriska hooyo ee xarumaha; `model.rs` ka kooban xog iyo ogaanshaha. `app.rs` waxay qabataa hawlaha asalka ah iyo xulashada; `views.rs`, `inspector.rs`, `connection.rs` iyo `theme.rs` waxay ku sawiraan interface egui/eframe. Interface-ku waxa uu ahaanayaa mid is dhexgal ah inta lagu jiro iskaanka oo waxa uu hayaa akhrintii ugu dambaysay ee guusha leh haddii sawirka xiga uu guul darraystay.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Maktabadu waa iskutallaab iyo koodhka Windows waa shuruudo. Noocyada macOS/Linux lama ansaxiyay keenintaan kowaad.

## Qarsoodi

Inventory waxay ka shaqeysaa gudaha, akhris-kaliya, iyada oo aan lahayn adeegyo telemetry ama shabakad. Xiriirinta hagaha waxay furan yihiin kaliya guji. Warbixinaha JSON waxa ay ka koobnaan karaan tirooyinka taxanaha ah iyo aqoonsiga nidaamka: wadaagistooda adiga ayay kugu xidhan tahay.

## Tixraacyo farsamo

- [nusb: tirinta USB iyo macluumaadka](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

shatiga MIT. Dhismayaasha dibeddu waxay ahaanayaan kuwo hoos imanaya shatiyadooda.

Kaydka dadweynuhu waxa uu ka kooban yahay kaliya ilaha, tarjumaada iyo xarfaha bilaashka ah ee lagama maarmaanka u ah ururinta Akhriska USB, qabashada ogaanshaha, dhoofinta, dookhyada maxalliga ah, khasnadaha iyo binaries horumarka waxaa ka saaray `.gitignore`. Si aad dhibaatada uga warbixiso, isticmaal qaabka `--demo` ama ka saar tirooyinka taxanaha ah iyo aqoonsiga nidaamka ka hor inta aanad wadaagin.

## Luqadaha - nooca 0.2.0

Codsiga waxaa ku jira ** 50 luqadaha is dhexgalka ***. Guji lambarka luqadda (`FR`, `EN`, iwm.) ee ku yaal midigta sare si aad u furto furaha. Waxaad ku raadin kartaa luqad magaceeda hooyo, magaceeda Ingiriisiga ama koodka. Isbeddelku waa isla markiiba doorashadana waxa lagu kaydiyaa `%APPDATA%/SanUsbViewer/preferences.json` ee hoos yimaada Windows. Dookh la'aan, luqadda nidaamka ayaa la isticmaalaa; Luqada aan la taageerin waxay ku noqotaa Ingiriis

Xulashadu waxay door bidaysaa luqadaha loogu hadlo guud ahaan (af-hooyo iyo luqadda labaad), iyadoo la isu keenayo noocyada kala duwan ee gobolka. Kani maaha kala-soocidda tirooyinka: natiijadu way kala duwan tahay iyadoo ku xidhan kooxaynta lahjadaha. Kala duwanaanshiyaha la doortay waxaa ka mid ah Shiinees la Fududeeyay, Carabiga Caadiga ah iyo farta Laatiinka ee Fulani iyo Uzbek; Punjabi waxay isticmaashaa Gurmukhi.

Ingiriis, Shiine, Hindi, Isbaanish, Faransiis, Carabi, Bengal, Boortaqiis, Ruush, Urdu, Indonesian, Jarmal, Jabbaan, Marathi, Fiyatnaamiis, Telugu, Hausa, Turki, Punjabi, Swahili, Filibiin, Tamil, Faaris, Kuuriyaan, Amxaari, Thai, Javanese, Talyaani, Gujarati, Kannada, Bhojpuri, Burmese, Polish, Yoruba, Lingalam, Oromo, Fulanian Uzbek, Pashto, Sundanese, Malay, Igbo, Nepali, Azeri, Maithili iyo Soomaali.

Menu-yada la guuriyay, ogaanshaha, qoraallada caawinta, fariimaha dhaqdhaqaaqa iyo tafaasiisha waa la meeleeyay. Magacyada iyo aqoonsiga ay bixiyeen qalabka iyo nidaamka waa la hayaa. Dhoofinta JSON waxay haysaa qorshaheeda iyo xogta alaabteeda, iyadoon loo eegin luqadda bandhigga. Unugyada farsamada ayaa ku haray **bit/s** (Mbit/s iyo Gbit/s) si looga fogaado jahawareer ka yimaada bytes.

Buug-gacmeedyada Faransiiska iyo Ingiriiska waxay u adeegaan tixraac ahaan. Luqadaha kale waa ** tarjumaad toos ah ***, taas oo u baahan karta sixid luqadeed. Faylasha `locales/*.json` ayaa lagu soo daray isku dubaridka waana la sixi karaa ka dibna dib ayaa loo ururin karaa. Ma jiro tarjumaad shabakad ah oo lagu fuliyo codsigu: 50-ka luqadood iyo xarfaha Noto ayaa ku dhex jira, oo hoos imaanaya Shatiga Furan ee SIL. Qoraalka Carabiga wuxuu adeegsadaa qaybaha Unicode ee laba jiho iyo khadadka cusub ee la xisaabiyay kahor qaabeynta.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` waxaa laga yaabaa inay tixraacdo faylka kale ee doorbidida ee tijaabinta ama rakibida la qaadi karo. Kaliya koodka luqadda ayaa halkaa ku kaydsan
