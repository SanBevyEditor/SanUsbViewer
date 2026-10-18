# San USB Viewer

**Lingála** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Mabongoli ya masini; ba correction ezali bienvenue.

**Version ya lelo: `0.2.0`** — Licence ya MIT.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Native USB Explorer ekomami na **Rust**, na interface ya molili, topologie visuelle mpe ba diagnostics oyo ekoki kotangama. Version ya liboso oyo ebongisami mpo na Windows 10/11.

## Origine ya projet

Video na origine ya projet: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Projet oyo ezali inspiré na ba usages ya **WhatCable** na Apple mpe **USB Device Tree Viewer** na Windows : ko comprendre ba périphériques, ba hubs mpe ba vitesse ya ba connexions USB na bango. Oyo ezali mise en œuvre indépendante.

## Biteni

- Détection réelle ya ba appareils USB na ba contrôleurs via `nusb`.
- Bokeli ya nzete na nzela ya bisi mpe libongo, na bozindo ya hub.
- Luka na nkombo, nimero ya série to VID:PID; Filtres USB 3+ na ba hubs.
- Vitesse ya lien négocié: 1,5 / 12 / 480 Mb/s, 5 Gb/s,
to catégorie SuperSpeedPlus (10 Gb/s to koleka).
- Diagramme ya mobembo : ordinateur, ba hubs intermédiaires, lien na dispositif.
- Na se ya Windows, interrogation supplémentaire ya hub na IOCTL mpo na koyeba ba...
SuperSpeed makoki esakolami na dispositif. Dispositif compatible SuperSpeed oyo ekangami na nzela ya USB 2.0 ebimisaka diagnostic ya limitation ya nzela.   Ba compagnons ya ba hubs USB 2 ya ba hubs USB 3 ba traité yango separatement pona ko éviter ko attribuer branche na bango ya 480 Mb/s na défaut ya câble.
- Ba identifiants VID/PID, descripteur USB, revision, pilote, série na ba interfaces, .
ntango système yango ezali kobimisa makambo yango polele.
- Mikolo oyo automatique chaque trois secondes, to manuel na F5.
- Journal ya session ya ba changements 200 ya suka oyo emonanaki.
- Exporter déclaration mobimba na JSON (Ctrl+E) mpe copier ba détails oyo oponi.
- Guide intégré mpe mode ya démonstration explicite, jamais esalelamaka lokola sauvegarde
silencieux soki détection elongi te.

## Kobwaka

Na Rust **1.95+**, molongo ya bisaleli ya MSVC mpe bisaleli ya Visual Studio C++ na Windows. Bandakisa esalela RTK mpo na ko filtrer sortie ya ba commandes; sans RTK, longola kaka préfixe `rtk`. Script ya distribution endimaka ba cas nionso mibale.

```powershell
rtk cargo run --release
```

Binaire autonome esalemi na `target/release/san-usb-viewer.exe`. Script ya `build.ps1` esali pe bopanzani na `dist/`. Modification ya pilote to installation ya WinUSB ezali na ntina te mpo na inventaire: application esengaka ba interfaces ya dispositif te.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Dossier ya destination pona capture esengeli ezala. Version ya kobimisa ezali application Windows oyo ezangi console; préférer `cargo run` na développement pona inventaire ya ligne ya commande.

## Limbolá matomba oyo ezwami

**Vites négociée** ezali oyo ya connexion, mpe te benchmark ya vitesse ya copie. Ba vitesse ya tina e dépend surtout na protocole, périphérique na ba appareils misusu oyo e partager bus. Vitesse oyo euti na ba appareils ebele ebakisami te mpo na kosala calcul ya bande passante oyo ezali.

Câble USB passif ezalaka mingi mingi dispositif oyo ekoki kotangama te. Version oyo etangi e-marqueur na yango te, to mpe puissance USB Power Delivery, to mpe type ya connecteur na yango te. Yango wana elobi te ete endimisi mbangu na yango ya monene. Diagnostic esalemaka na lien oyo emonanaki mpe, soki ezali, na ba indicateurs ya Windows. Lien ya malembe ekoki kozala na ndelo na câble, port, hub, to appareil. Nimero ya version ya descripteur USB esalelamaka ata moke te lokola vitesse maximale.

Catégorie ya SuperSpeedPlus ekesenisaka te entre ba modes ya 10 na 20 Gb/s awa. USB4/Thunderbolt na ba tunnels na bango e inspecter te. Ba ports oyo ezangi appareil oyo ekangami ezali na inventaire te. Câble ya chargement oyo ezangi ba données esala dispositif oyo emonanaka te. Ba dispositifs ya kati (Bluetooth, caméra intégré, etc.) ekoki kobima na nzete ya USB.

Ba informations oyo ekoki kozuama te elakisami lokola ezali te. Fabricant na ba interfaces ezo exposer toujours te na Windows. Makambo oyo esalemaka mobimba kati na ba scans ekoki kokende kozanga koyeba. Journal ezali na mémoire mpe ebandi sima ya inventaire ya liboso oyo elongi.

Ba identifiants ya bus na ba nzela ya port elimbolaka topologie ya système. Connecteur physique ekoki kozala na ba nzela mibale ekeseni pona USB 2 na SuperSpeed.

## Architecture na validation

`scanner.rs` esalaka inventaire; `windows_probe.rs` e encapsuler ba demandes ya botangi native oyo ewutaka na ba hubs; `model.rs` ezali na ba données na ba diagnostics. `app.rs` esimbaka misala ya sima mpe boponi; `views.rs`, `inspector.rs`, `connection.rs` na `theme.rs` ba benda interface na egui/eframe. Interface etikalaka interactif na tango ya scanner mpe ebatelaka botangi ya suka oyo elongi soki scanner oyo elandi elongi te.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Ba bibliothèques ezali croisé plateforme mpe code ya Windows ezali conditionnel. Ba versions ya macOS/Linux e validé te na livraison oyo ya liboso.

## Nkuku

Inventaire esalaka na esika, kaka na botangi, sans télémétrie to ba services ya réseau. Ba liens ya guide efungwamaka kaka na clique. Ba rapports ya JSON ekoki kozala na ba numéros de série na ba identifiants ya système : kokabola yango ezali na yo.

## Ba références techniques

- [nusb: Botangi ya USB mpe sango](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html) oyo ezali na kati
- [Microsoft: IOCTL_USB_ZUA_MAKANISI_YA_BOKANGI_YA_NODE_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2) oyo ezali na kati
- [egui/eframe oyo ezali na kati](https://github.com/emilk/egui) oyo ezali na kati

Licence ya MIT. Bandako ya libanda etikali na mikanda na yango ya ndingisa.

Bobombi ya bato banso ezali kaka na maziba, mabongoli mpe makomi ya ofele oyo esengeli mpo na bosangisi. Ba lectures USB, ba captures ya diagnostic, ba exportations, ba préférences locales, ba caches na ba binaire ya développement elongolami na `.gitignore`. Mpo na koyebisa mokakatano, salelá mode `--demo` to longola banimero ya série mpe ba ID ya système na balapolo na yo liboso ya kokabola yango.

## Minɔkɔ — version 0.2.0

Application yango ezali na **50 minoko ya interface**. Finá code ya monɔkɔ (`FR`, `EN`, mpe bongo na bongo) na likoló na lobɔkɔ ya mobali mpo na kofungola commutateur. Okoki koluka monoko na nkombo na yango ya mboka, nkombo na yango ya Lingelesi to code na yango. Mbongwana ezali mbala moko mpe boponi ebombami na `%APPDATA%/SanUsbViewer/preferences.json` na nse ya Windows. Kozanga préférence, monoko ya système esalelamaka; Monɔkɔ oyo esungami te ezongaka na Lingelesi.

Pona yango elingi minoko oyo balobaka mingi na mobimba (monɔkɔ ya mboka mpe monɔkɔ ya mibale), kosangisaka ba variantes régionales. Oyo ezali classification numérique te : ba résultats ekeseni selon groupement ya ba dialectes. Ba variantes oyo eponami ezali Chinois Simplifié, Arabe Standard mpe ba écritures latines ya Fulani na Ouzbék; Punjabi esalelaka Gurmukhi.

Lingelesi, Chinois, Hindi, Espagnol, Français, Arabe, Bengali, Portugais, Russe, Urdu, Indonésien, Allemand, Japonais, Marathi, Vietnamien, Telugu, Hausa, Turc, Punjabi, Swahili, Filipino, Tamil, Perse, Coréen, Amharique, Thaïlandais, Javan, Italien, Gujarati, Kannada, Bhojpuri, Birmanis, Polonais, Yoruba, Lingala, Oromo, Fulani, Malayalam, Odia, Ukraine, Sindhi, Ouzbék, Pashto, Sundan, Malais, Igbo, Népal, Azeri, Maithili mpe Somali.

Ba menu copiés, ba diagnostics, ba textes ya aide, ba messages ya activité na ba détails ezali localisé. Ba kombo mpe ba identifiants oyo epesami na matériel mpe système ebombami. Exportation JSON ebombaka ba données ya schéma na inventaire na yango, sans considération ya monoko ya kolakisa. Ba unité techniques etikalaka na **bit/s** (Mbit/s na Gbit/s) pona ko éviter confusion na ba octets.

Ba catalogue ya Lifalanse mpe ya Lingelesi ezali lokola référence. Minɔkɔ mosusu ezali **bobongoli ya automatique**, oyo ekoki kosɛnga kobongisa nkota. Ba fichiers `locales/*.json` ekotisami na compilation mpe ekoki ko corrigé mpe sima ko recompilé. Bobongoli ya réseau moko te esalemi na application : minoko 50 mpe ba fonts ya Noto ekotisami, na se ya Licence ya Font Ouvert ya SIL. Texte arabe esalelaka ba segments ya Unicode bidirectionnels mpe ba lignes ya sika oyo e calculer avant ya formatage.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` ekoki kolobela fisyé mosusu ya ba préférences mpo na komeka to installation portable. Kaka code ya monoko nde ebombami kuna.
