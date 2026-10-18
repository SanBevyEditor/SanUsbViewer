# San USB Viewer

**Deutsch** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Maschinelle Übersetzungen; Korrekturen sind willkommen.

**Aktuelle Version: `0.2.0`** – MIT-Lizenz.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Nativer USB-Explorer, geschrieben in **Rust**, mit dunkler Oberfläche, visueller Topologie und lesbaren Diagnosen. Erste Version für Windows 10/11.

## Ursprung des Projekts

Video zum Ursprung des Projekts: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Das Projekt ist von der Verwendung von **WhatCable** bei Apple und **USB Device Tree Viewer** unter Windows inspiriert: Verständnis von Peripheriegeräten, Hubs und der Geschwindigkeit ihrer USB-Verbindungen. Dies ist eine unabhängige Implementierung.

## Merkmale

- Tatsächliche Erkennung von USB-Geräten und Controllern über `nusb`.
- Baumstruktur nach Bus- und Portpfad, mit Hubtiefe.
- Suche nach Name, Seriennummer oder VID:PID; USB 3+ Filter und Hubs.
- Ausgehandelte Verbindungsgeschwindigkeit: 1,5 / 12 / 480 Mbit/s, 5 Gbit/s,
oder Kategorie SuperSpeedPlus (10 Gbit/s oder mehr).
- Reisediagramm: Computer, Zwischenknoten, Link und Gerät.
- Unter Windows ist eine zusätzliche Abfrage des Hubs durch IOCTL erforderlich, um das herauszufinden
Vom Gerät angekündigte SuperSpeed-Funktionen. Ein über USB 2.0 angeschlossenes SuperSpeed-kompatibles Gerät erzeugt eine Pfadbegrenzungsdiagnose.   Die USB-2-Hubs als Begleiter der USB-3-Hubs werden separat behandelt, um zu vermeiden, dass ihr 480-Mbit/s-Zweig auf einen Kabelfehler zurückzuführen ist.
- VID/PID-Kennungen, USB-Deskriptor, Revision, Treiber, Seriell und Schnittstellen,
wenn das System diese Informationen offenlegt.
- Automatische Aktualisierung alle drei Sekunden oder manuell mit F5.
- Sitzungsprotokoll der letzten 200 beobachteten Änderungen.
- Exportieren Sie die vollständige Anweisung in JSON (Strg+E) und kopieren Sie die ausgewählten Details.
- Integrierte Anleitung und expliziter Demonstrationsmodus, wird nie als Backup verwendet
stumm, wenn die Erkennung fehlschlägt.

## Werfen

Mit Rust **1.95+**, der MSVC-Toolchain und Visual Studio C++-Tools unter Windows. Die Beispiele verwenden RTK, um die Ausgabe von Befehlen zu filtern; Ohne RTK entfernen Sie einfach das Präfix `rtk`. Das Verteilungsskript akzeptiert beide Fälle.

```powershell
rtk cargo run --release
```

Die eigenständige Binärdatei wird in `target/release/san-usb-viewer.exe` erstellt. Das `build.ps1`-Skript erstellt auch eine Verteilung in `dist/`. Für die Inventarisierung ist keine Treiberänderung oder Installation von WinUSB erforderlich: Die Anwendung benötigt keine Geräteschnittstellen.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

Der Zielordner für eine Aufnahme muss vorhanden sein. Die Release-Version ist eine konsolenlose Windows-Anwendung; Bevorzugen Sie `cargo run` in der Entwicklung für die Befehlszeileninventur.

## Interpretieren Sie die Ergebnisse

Die **ausgehandelte Geschwindigkeit** ist die der Verbindung und kein Maßstab für die Kopiergeschwindigkeit. Die nutzbaren Geschwindigkeiten hängen insbesondere vom Protokoll, der Peripherie und den anderen Geräten ab, die sich den Bus teilen. Die Geschwindigkeiten mehrerer Geräte addieren sich nicht zur Berechnung der verfügbaren Bandbreite.

Ein passives USB-Kabel ist im Allgemeinen kein aufzählbares Gerät. Diese Version liest weder den E-Marker noch die USB-Power-Delivery-Stromversorgung noch den Steckertyp. Es erhebt daher nicht den Anspruch, seine Höchstgeschwindigkeit zu zertifizieren. Die Diagnose basiert auf dem beobachteten Link und, falls verfügbar, auf Windows-Indikatoren. Eine langsame Verbindung kann durch das Kabel, den Port, den Hub oder das Gerät eingeschränkt sein. Die Versionsnummer des USB-Deskriptors wird niemals als maximale Geschwindigkeit verwendet.

Die Kategorie SuperSpeedPlus unterscheidet hier nicht zwischen 10- und 20-Gbit/s-Modi. USB4/Thunderbolt und deren Tunnel werden nicht überprüft. Ports ohne angeschlossenes Gerät werden nicht inventarisiert. Ein Ladekabel ohne Daten ergibt kein sichtbares Gerät. Interne Geräte (Bluetooth, integrierte Kamera usw.) werden möglicherweise im USB-Baum angezeigt.

Unzugängliche Informationen werden als nicht verfügbar angezeigt. Der Hersteller und die Schnittstellen werden von Windows nicht immer offengelegt. Ereignisse, die vollständig zwischen Scans auftreten, können unbemerkt bleiben. Das Protokoll befindet sich im Speicher und beginnt nach der ersten erfolgreichen Inventur.

Buskennungen und Portpfade beschreiben die Systemtopologie. Ein physischer Anschluss kann zwei separate Pfade für USB 2 und SuperSpeed haben.

## Architektur und Validierung

`scanner.rs` führt eine Inventur durch; `windows_probe.rs` kapselt native Leseanforderungen von Hubs; `model.rs` enthält Daten und Diagnosen. `app.rs` übernimmt Hintergrundaufgaben und Auswahl; `views.rs`, `inspector.rs`, `connection.rs` und `theme.rs` zeichnen die Schnittstelle mit egui/eframe. Die Schnittstelle bleibt während eines Scans interaktiv und behält den letzten erfolgreichen Lesevorgang bei, wenn der nächste Scan fehlschlägt.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Die Bibliotheken sind plattformübergreifend und der Windows-Code ist bedingt. Die macOS/Linux-Versionen werden in dieser ersten Lieferung nicht validiert.

## Vertraulichkeit

Die Bestandsaufnahme funktioniert lokal, schreibgeschützt, ohne Telemetrie oder Netzwerkdienste. Die Guide-Links öffnen sich nur per Klick. JSON-Berichte können Seriennummern und Systemkennungen enthalten: Die Weitergabe liegt bei Ihnen.

## Technische Referenzen

- [nusb: USB-Aufzählung und Informationen](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

MIT-Lizenz. Für die Nebengebäude gelten weiterhin die jeweiligen Lizenzen.

Das öffentliche Repository enthält nur die für die Zusammenstellung notwendigen Quellen, Übersetzungen und kostenlosen Schriftarten. USB-Messwerte, Diagnoseerfassungen, Exporte, lokale Einstellungen, Caches und Entwicklungsbinärdateien werden von `.gitignore` ausgeschlossen. Um ein Problem zu melden, verwenden Sie den Modus `--demo` oder entfernen Sie Seriennummern und System-IDs aus Ihren Berichten, bevor Sie sie weitergeben.

## Sprachen – Version 0.2.0

Die Anwendung umfasst **50 Schnittstellensprachen**. Klicken Sie oben rechts auf den Sprachcode (`FR`, `EN` usw.), um den Umschalter zu öffnen. Sie können nach einer Sprache anhand ihres nativen Namens, ihres englischen Namens oder ihres Codes suchen. Die Änderung erfolgt sofort und die Auswahl wird in `%APPDATA%/SanUsbViewer/preferences.json` unter Windows gespeichert. Ohne Präferenz wird die Systemsprache verwendet; Eine nicht unterstützte Sprache wird auf Englisch zurückgesetzt.

Die Auswahl bevorzugt die insgesamt am häufigsten gesprochenen Sprachen (Muttersprache und Zweitsprache) und gruppiert regionale Varianten. Hierbei handelt es sich nicht um eine numerische Klassifizierung: Die Ergebnisse variieren je nach Gruppierung der Dialekte. Zu den ausgewählten Varianten gehören vereinfachtes Chinesisch, Standardarabisch und die lateinischen Schriften von Fulani und Usbekisch; Punjabi verwendet Gurmukhi.

Englisch, Chinesisch, Hindi, Spanisch, Französisch, Arabisch, Bengali, Portugiesisch, Russisch, Urdu, Indonesisch, Deutsch, Japanisch, Marathi, Vietnamesisch, Telugu, Hausa, Türkisch, Punjabi, Suaheli, Filipino, Tamil, Persisch, Koreanisch, Amharisch, Thailändisch, Javanisch, Italienisch, Gujarati, Kannada, Bhojpuri, Burmesisch, Polnisch, Yoruba, Lingala, Oromo, Fulani, Malayalam, Odia, Ukrainisch, Sindhi, Usbekisch, Paschtu, Sundanesisch, Malaiisch, Igbo, Nepali, Aserbaidschanisch, Maithili und Somali.

Kopierte Menüs, Diagnosen, Hilfetexte, Aktivitätsmeldungen und Details werden lokalisiert. Von der Hardware und dem System bereitgestellte Namen und Kennungen bleiben erhalten. Der JSON-Export behält seine Schema- und Inventardaten unabhängig von der Anzeigesprache. Technische Einheiten bleiben in **Bit/s** (Mbit/s und Gbit/s), um Verwechslungen mit Bytes zu vermeiden.

Als Referenz dienen die französischen und englischen Kataloge. Bei anderen Sprachen handelt es sich um **automatische Übersetzungen**, die möglicherweise sprachliche Korrekturen erfordern. `locales/*.json`-Dateien werden in die Kompilierung einbezogen und können korrigiert und anschließend neu kompiliert werden. Die Anwendung führt keine Netzwerkübersetzung durch: Die 50 Sprachen und Noto-Schriftarten sind unter der SIL Open Font License eingebettet. Arabischer Text verwendet bidirektionale Unicode-Segmente und Zeilenumbrüche, die vor der Formatierung berechnet werden.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` verweist möglicherweise auf eine andere Einstellungsdatei zum Testen oder auf eine tragbare Installation. Dort wird nur der Sprachcode gespeichert.
