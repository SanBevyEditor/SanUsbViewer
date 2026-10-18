# San USB Viewer

[English](docs/readme/README.en.md) · [README dans les 50 langues](docs/README.md)

**Version actuelle : `0.2.0`** — licence MIT.

![San USB Viewer](assets/screenshots/SanUsbViewer.jpg)

Explorateur USB natif écrit en **Rust**, avec une interface sombre, une topologie
visuelle et des diagnostics lisibles. Première version destinée à Windows 10/11.

## Origine du projet

Vidéo à l’origine du projet :
[https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Le projet s’inspire des usages de **WhatCable** sur Apple et de **USB Device Tree
Viewer** sur Windows : comprendre les périphériques, les hubs et les vitesses de
leurs connexions USB. Il s’agit d’une implémentation indépendante.

## Fonctionnalités

- Détection réelle des périphériques et contrôleurs USB via `nusb`.
- Arborescence par bus et chemin des ports, avec profondeur des hubs.
- Recherche par nom, numéro de série ou VID:PID ; filtres USB 3+ et hubs.
- Vitesse de liaison négociée : 1,5 / 12 / 480 Mb/s, 5 Gb/s,
  ou catégorie SuperSpeedPlus (10 Gb/s ou plus).
- Schéma du trajet : ordinateur, hubs intermédiaires, liaison et périphérique.
- Sous Windows, interrogation complémentaire du hub par IOCTL pour connaître les
  capacités SuperSpeed annoncées par le périphérique. Un appareil compatible
  SuperSpeed connecté en USB 2.0 produit un diagnostic de limitation du trajet.
  Les hubs USB 2 compagnons des hubs USB 3 sont traités séparément pour éviter
  d’attribuer leur branche à 480 Mb/s à un défaut du câble.
- Identifiants VID/PID, descripteur USB, révision, pilote, série et interfaces,
  lorsque le système expose ces informations.
- Actualisation automatique toutes les trois secondes, ou manuelle avec F5.
- Journal de session des 200 derniers changements observés.
- Export du relevé complet en JSON (Ctrl+E) et copie des détails sélectionnés.
- Guide intégré et mode démonstration explicite, jamais utilisé comme secours
  silencieux en cas d’échec de la détection.

## Lancer

Avec Rust **1.95+**, le toolchain MSVC et les outils C++ de Visual Studio sous Windows.
Les exemples utilisent RTK pour filtrer la sortie des commandes ; sans RTK, retirez
simplement le préfixe `rtk`. Le script de distribution accepte les deux cas.

```powershell
rtk cargo run --release
```

Le binaire autonome est produit dans `target/release/san-usb-viewer.exe`.
Le script `build.ps1` crée aussi une distribution dans `dist/SanUsbViewer/`.
Aucune modification de pilote ni installation de WinUSB n’est nécessaire pour
l’inventaire : l’application ne réclame pas les interfaces des périphériques.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Le dossier de destination d’une capture doit exister. La version release est une
application Windows sans console ; préférer `cargo run` en développement pour
l’inventaire en ligne de commande.

## Interpréter les résultats

La **vitesse négociée** est celle de la connexion, et non un benchmark du débit
de copie. Les débits utiles dépendent notamment du protocole, du périphérique et
des autres appareils partageant le bus. Les vitesses de plusieurs périphériques
ne s’additionnent pas pour calculer la bande passante disponible.

Un câble USB passif n’est généralement pas un périphérique énumérable. Cette
version ne lit ni son e-marker, ni la puissance USB Power Delivery, ni son type de
connecteur. Elle ne prétend donc pas certifier sa vitesse maximale. Le diagnostic
se base sur la liaison observée et, si disponibles, sur les indicateurs Windows.
Une liaison lente peut être limitée par le câble, le port, un hub ou l’appareil.
Le numéro de version du descripteur USB n’est jamais utilisé comme vitesse maximale.

La catégorie SuperSpeedPlus ne distingue pas ici les modes 10 et 20 Gb/s.
USB4 / Thunderbolt et leurs tunnels ne sont pas inspectés. Les ports sans
périphérique connecté ne sont pas inventoriés. Un câble de charge sans données
ne crée pas de périphérique visible. Les périphériques internes (Bluetooth,
caméra intégrée, etc.) peuvent apparaître dans l’arborescence USB.

Une information inaccessible est affichée comme indisponible. Le fabricant et
les interfaces ne sont pas toujours exposés par Windows. Les événements qui se
produisent entièrement entre deux analyses peuvent passer inaperçus. Le journal
est en mémoire et commence après le premier inventaire réussi.

Les identifiants de bus et les chemins de ports décrivent la topologie du système.
Un connecteur physique peut avoir deux chemins distincts pour USB 2 et SuperSpeed.

## Architecture et validation

`scanner.rs` effectue l’inventaire ; `windows_probe.rs` encapsule les requêtes
natives de lecture des hubs ; `model.rs` contient les données et diagnostics.
`app.rs` gère les tâches en arrière-plan et la sélection ; `views.rs`,
`inspector.rs`, `connection.rs` et `theme.rs` dessinent l’interface avec egui/eframe.
L’interface reste interactive pendant une analyse et conserve la dernière lecture
réussie si l’analyse suivante échoue.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Les bibliothèques sont multiplateformes et le code Windows est conditionnel.
Les versions macOS/Linux ne sont pas validées dans cette première livraison.

## Confidentialité

L’inventaire fonctionne localement, en lecture seule, sans télémétrie ni service
réseau. Les liens du guide ne s’ouvrent que sur clic. Les rapports JSON peuvent
contenir les numéros de série et les identifiants système : leur partage reste
à votre initiative.

## Références techniques

- [nusb : énumération et informations USB](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft : IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui / eframe](https://github.com/emilk/egui)

Licence MIT. Les dépendances restent soumises à leurs licences respectives.

Le dépôt public contient uniquement les sources, les traductions et les polices
libres nécessaires à la compilation. Les relevés USB, captures de diagnostic,
exports, préférences locales, caches et binaires de développement sont exclus
par `.gitignore`. Pour signaler un problème, utilisez le mode `--demo` ou retirez
les numéros de série et identifiants système de vos relevés avant de les partager.

## Langues — version 0.2.0

L’application inclut **50 langues d’interface**. Cliquez sur le code de langue
(`FR`, `EN`, etc.) en haut à droite pour ouvrir le sélecteur. On peut rechercher
une langue par son nom natif, son nom anglais ou son code. Le changement est
immédiat et le choix est mémorisé dans
`%APPDATA%/SanUsbViewer/preferences.json` sous Windows. Sans préférence, la langue
du système est utilisée ; une langue non prise en charge revient à l’anglais.

La sélection privilégie les langues les plus parlées au total (langue maternelle
et seconde langue), en regroupant les variantes régionales. Ce n’est pas un
classement numérique : les résultats varient selon le regroupement des dialectes.
Les variantes retenues incluent le chinois simplifié, l’arabe standard et les
écritures latines du peul et de l’ouzbek ; le pendjabi utilise le gurmukhi.

Anglais, chinois, hindi, espagnol, français, arabe, bengali, portugais, russe,
ourdou, indonésien, allemand, japonais, marathi, vietnamien, télougou, haoussa,
turc, pendjabi, swahili, filipino, tamoul, persan, coréen, amharique, thaï,
javanais, italien, gujarati, kannada, bhojpuri, birman, polonais, yoruba, lingala,
oromo, peul, malayalam, odia, ukrainien, sindhi, ouzbek, pachto, soundanais,
malais, igbo, népalais, azéri, maithili et somali.

Les menus, diagnostics, textes d’aide, messages d’activité et détails copiés sont
localisés. Les noms et identifiants fournis par le matériel et le système sont
conservés. L’export JSON garde son schéma et ses données d’inventaire, quelle que
soit la langue d’affichage. Les unités techniques restent en **bit/s** (Mbit/s et
Gbit/s) pour éviter toute confusion avec les octets.

Les catalogues français et anglais servent de référence. Les autres langues sont
des **traductions automatiques**, susceptibles de nécessiter des corrections
linguistiques. Les fichiers `locales/*.json` sont intégrés à la compilation et
peuvent être corrigés puis recompilés. Aucune traduction réseau n’est effectuée
par l’application : les 50 langues et les polices Noto sont embarquées, sous
licence SIL Open Font License. Les textes arabes utilisent des segments Unicode
bidirectionnels et des retours à la ligne calculés avant la mise en forme.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` peut désigner un autre fichier de préférences pour les
tests ou une installation portable. Seul le code de langue y est enregistré.
