# San USB Viewer

**Kiswahili** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Tafsiri za mashine; masahihisho yanakaribishwa.

**Toleo la sasa: `0.2.0`** — Leseni ya MIT.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Native USB Explorer iliyoandikwa kwa **Rust**, yenye kiolesura cheusi, topolojia ya kuona na uchunguzi unaoweza kusomeka. Toleo la kwanza linalokusudiwa Windows 10/11.

## Asili ya mradi

Video katika asili ya mradi: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Mradi huu umechochewa na matumizi ya **WhatCable** kwenye Apple na **USB Device Tree Viewer** kwenye Windows: kuelewa vifaa vya pembeni, vitovu na kasi ya miunganisho yao ya USB. Huu ni utekelezaji wa kujitegemea.

## Vipengele

- Ugunduzi halisi wa vifaa vya USB na vidhibiti kupitia `nusb`.
- Muundo wa miti kwa njia ya basi na bandari, yenye kina cha kitovu.
- Tafuta kwa jina, nambari ya serial au VID:PID; Vichungi vya USB 3+ na vitovu.
- Kasi ya kiungo iliyojadiliwa: 1.5 / 12 / 480 Mb/s, 5 Gb/s,
au kategoria SuperSpeedPlus (Gb 10/s au zaidi).
- Mchoro wa safari: kompyuta, vibanda vya kati, kiungo na kifaa.
- Chini ya Windows, mahojiano ya ziada ya kitovu na IOCTL ili kujua
Uwezo wa SuperSpeed uliotangazwa na kifaa. Kifaa kinachooana cha SuperSpeed kilichounganishwa kupitia USB 2.0 hutoa uchunguzi wa kizuizi cha njia.   Vitovu vya USB 2 viandamani vya vitovu vya USB 3 vinashughulikiwa kando ili kuepuka kuhusisha tawi lao la 480 Mb/s na hitilafu ya kebo.
- Vitambulisho vya VID/PID, kifafanuzi cha USB, marekebisho, kiendeshi, mfululizo na violesura,
wakati mfumo unafichua habari hii.
- Sasisha kiotomatiki kila sekunde tatu, au mwongozo na F5.
- Kumbukumbu ya kipindi cha mabadiliko 200 ya mwisho yaliyozingatiwa.
- Hamisha taarifa kamili katika JSON (Ctrl+E) na unakili maelezo yaliyochaguliwa.
- Mwongozo uliojumuishwa na hali ya onyesho dhahiri, haijawahi kutumika kama nakala rudufu
kimya katika kesi ya kushindwa kutambua.

## Tupa

Na Rust **1.95+**, zana za MSVC na Visual Studio C++ kwenye Windows. Mifano hutumia RTK kuchuja matokeo ya amri; bila RTK, ondoa tu kiambishi awali cha `rtk`. Hati ya usambazaji inakubali visa vyote viwili.

```powershell
rtk cargo run --release
```

Nambari inayojitegemea inatolewa katika `target/release/san-usb-viewer.exe`. Hati ya `build.ps1` pia inaunda usambazaji katika `dist/SanUsbViewer/`. Hakuna marekebisho ya kiendeshi au usakinishaji wa WinUSB ni muhimu kwa hesabu: programu hauhitaji miingiliano ya kifaa.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Folda lengwa la kunasa lazima liwepo. Toleo la kutolewa ni programu ya Windows isiyo na console; pendelea `cargo run` katika ukuzaji wa hesabu ya safu ya amri.

## Tafsiri matokeo

**Kasi ya mazungumzo** ni ile ya muunganisho, na si alama ya kasi ya kunakili. Kasi muhimu inategemea hasa itifaki, vifaa vya pembeni na vifaa vingine vinavyoshiriki basi. Kasi kutoka kwa vifaa vingi haijumuishi ili kukokotoa kipimo data kinachopatikana.

Kebo ya USB tulivu kwa ujumla si kifaa kinachoweza kuhesabika. Toleo hili halisomi kialama chake cha kielektroni, wala nguvu ya Utoaji Nishati ya USB, wala aina yake ya kiunganishi. Kwa hiyo haidai kuthibitisha kasi yake ya juu. Utambuzi unategemea kiungo kilichozingatiwa na, ikiwa kinapatikana, kwenye viashiria vya Windows. Kiungo cha polepole kinaweza kuzuiwa na kebo, mlango, kitovu au kifaa. Nambari ya toleo la kifafanuzi la USB haitumiki kamwe kama kasi ya juu zaidi.

Kitengo cha SuperSpeedPlus hakitofautishi kati ya modi 10 na 20 za Gb/s hapa. USB4/Thunderbolt na vichuguu vyake hazijakaguliwa. Bandari zisizo na kifaa kilichounganishwa hazijaorodheshwa. Kebo ya kuchaji bila data haifanyi kifaa kinachoonekana. Vifaa vya ndani (Bluetooth, kamera iliyojengewa ndani, n.k.) vinaweza kuonekana kwenye mti wa USB.

Taarifa zisizoweza kufikiwa zinaonyeshwa kama hazipatikani. Mtengenezaji na miingiliano sio wazi kila wakati na Windows. Matukio ambayo hutokea kabisa kati ya utafutaji yanaweza yasitambuliwe. Logi iko kwenye kumbukumbu na huanza baada ya hesabu ya kwanza iliyofanikiwa.

Vitambulisho vya mabasi na njia za bandari huelezea topolojia ya mfumo. Kiunganishi cha kimwili kinaweza kuwa na njia mbili tofauti za USB 2 na SuperSpeed.

## Usanifu na uthibitishaji

`scanner.rs` hufanya hesabu; `windows_probe.rs` hujumuisha maombi asilia ya usomaji kutoka kwa vitovu; `model.rs` ina data na uchunguzi. `app.rs` inashughulikia kazi za nyuma na uteuzi; `views.rs`, `inspector.rs`, `connection.rs` na `theme.rs` huchora kiolesura kwa egui/eframe. Kiolesura husalia kikiingiliana wakati wa kuchanganua na huhifadhi usomaji wa mwisho uliofaulu ikiwa utaftaji unaofuata utashindwa.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Maktaba ni za jukwaa na msimbo wa Windows ni wa masharti. Matoleo ya macOS/Linux hayajathibitishwa katika uwasilishaji huu wa kwanza.

## Usiri

Malipo hufanya kazi ndani ya nchi, kusoma tu, bila telemetry au huduma za mtandao. Viungo vya mwongozo hufungua tu kwa kubofya. Ripoti za JSON zinaweza kuwa na nambari za ufuatiliaji na vitambulisho vya mfumo: kuzishiriki ni juu yako.

## Marejeleo ya kiufundi

- [nusb: hesabu ya USB na habari](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

Leseni ya MIT. Jengo la nje linabaki chini ya leseni zao.

Hazina ya umma ina vyanzo tu, tafsiri na fonti zisizolipishwa zinazohitajika kwa ujumuishaji. Usomaji wa USB, picha za uchunguzi, mauzo ya nje, mapendeleo ya ndani, kache na jozi za usanidi hazijajumuishwa na `.gitignore`. Ili kuripoti tatizo, tumia hali ya `--demo` au uondoe nambari za mfululizo na vitambulisho vya mfumo kwenye ripoti zako kabla ya kuzishiriki.

## Lugha - toleo la 0.2.0

Programu inajumuisha **lugha 50 za kiolesura**. Bofya msimbo wa lugha (`FR`, `EN`, n.k.) iliyo upande wa juu kulia ili kufungua kibadilishaji. Unaweza kutafuta lugha kwa jina lake la asili, jina lake la Kiingereza au msimbo wake. Mabadiliko ni ya haraka na chaguo limehifadhiwa katika `%APPDATA%/SanUsbViewer/preferences.json` chini ya Windows. Bila upendeleo, lugha ya mfumo hutumiwa; Lugha isiyotumika inarudi kwa Kiingereza.

Uteuzi huo unapendelea lugha zinazozungumzwa zaidi kwa jumla (lugha ya mama na lugha ya pili), ikiweka pamoja anuwai za kikanda. Huu sio uainishaji wa nambari: matokeo hutofautiana kulingana na mpangilio wa lahaja. Vibadala vilivyochaguliwa ni pamoja na Kichina Kilichorahisishwa, Kiarabu Sanifu na hati za Kilatini za Kifulani na Kiuzbeki; Kipunjabi hutumia Gurmukhi.

Kiingereza, Kichina, Kihindi, Kihispania, Kifaransa, Kiarabu, Kibengali, Kireno, Kirusi, Kiurdu, Kiindonesia, Kijerumani, Kijapani, Marathi, Kivietinamu, Kitelugu, Kihausa, Kituruki, Kipunjabi, Kiswahili, Kifilipino, Kitamil, Kiajemi, Kikorea, Kiamhari, Kithai, Kijava, Kiitaliano, Kigujarati, Kikannada, Bhojpuri, Kipolandi, Kiyoruba, Kimalayalam, Kimalayalam Odia, Kiukreni, Kisindhi, Kiuzbeki, Kipashto, Kisunda, Kimalei, Kiigbo, Kinepali, Kiazeri, Maithili na Kisomali.

Menyu zilizonakiliwa, uchunguzi, maandishi ya usaidizi, ujumbe wa shughuli na maelezo yamejanibishwa. Majina na vitambulisho vilivyotolewa na maunzi na mfumo huhifadhiwa. Uhamishaji wa JSON huhifadhi utaratibu wake na data ya orodha, bila kujali lugha ya kuonyesha. Vitengo vya kiufundi vinasalia katika **bit/s** (Mbit/s na Gbit/s) ili kuepuka kuchanganyikiwa na baiti.

Katalogi za Kifaransa na Kiingereza hutumika kama marejeleo. Lugha zingine ni **tafsiri za kiotomatiki**, ambazo zinaweza kuhitaji marekebisho ya lugha. Faili za `locales/*.json` zimejumuishwa kwenye mkusanyiko na zinaweza kusahihishwa na kisha kukusanywa tena. Hakuna tafsiri ya mtandao inayofanywa na programu: lugha 50 na fonti za Noto zimepachikwa, chini ya Leseni ya SIL Open Font. Maandishi ya Kiarabu hutumia sehemu za Unicode zinazoelekezwa pande mbili na laini mpya zinazokokotolewa kabla ya umbizo.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` inaweza kurejelea faili nyingine ya mapendeleo kwa majaribio au usakinishaji wa kubebeka. Msimbo wa lugha pekee ndio umehifadhiwa hapo.
