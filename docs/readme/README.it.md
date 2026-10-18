# San USB Viewer

**Italiano** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Traduzioni automatiche; sono gradite correzioni.

**Versione corrente: `0.2.0`** — Licenza MIT.

**[Windows 10/11 x64 — v0.2.0 (.zip)](https://github.com/SanBevyEditor/SanUsbViewer/releases/download/v0.2.0/SanUsbViewer-v0.2.0-windows-x86_64.zip)**

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

USB Explorer nativo scritto in **Rust**, con interfaccia scura, topologia visiva e diagnostica leggibile. Prima versione destinata a Windows 10/11.

## Origine del progetto

Video all'origine del progetto: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

Il progetto si ispira agli usi di **WhatCable** su Apple e **USB Device Tree Viewer** su Windows: comprendere le periferiche, gli hub e le velocità delle loro connessioni USB. Questa è un'implementazione indipendente.

## Caratteristiche

- Rilevamento effettivo di dispositivi e controller USB tramite `nusb`.
- Struttura ad albero per percorso bus e porto, con profondità mozzo.
- Ricerca per nome, numero di serie o VID:PID; Filtri e hub USB 3+.
- Velocità di collegamento negoziata: 1,5 / 12 / 480 Mb/s, 5 Gb/s,
o categoria SuperSpeedPlus (10 Gb/s o superiore).
- Diagramma di viaggio: computer, hub intermedi, collegamento e dispositivo.
- In Windows, ulteriore interrogazione dell'hub da parte di IOCTL per scoprire il file
Funzionalità SuperSpeed annunciate dal dispositivo. Un dispositivo compatibile SuperSpeed collegato tramite USB 2.0 produce una diagnostica di limitazione del percorso.   Gli hub USB 2 compagni degli hub USB 3 vengono trattati separatamente per evitare di attribuire la loro diramazione a 480 Mb/s ad un guasto del cavo.
- Identificatori VID/PID, descrittore USB, revisione, driver, seriale e interfacce,
quando il sistema espone queste informazioni.
- Aggiornamento automatico ogni tre secondi o manuale con F5.
- Registro della sessione delle ultime 200 modifiche osservate.
- Esporta l'estratto conto completo in JSON (Ctrl+E) e copia i dettagli selezionati.
- Guida integrata e modalità dimostrativa esplicita, mai utilizzata come backup
silenzioso in caso di errore di rilevamento.

## Gettare

Con Rust **1.95+**, la toolchain MSVC e gli strumenti Visual Studio C++ su Windows. Gli esempi utilizzano RTK per filtrare l'output dei comandi; senza RTK, basta rimuovere il prefisso `rtk`. Lo script di distribuzione accetta entrambi i casi.

```powershell
rtk cargo run --release
```

Il binario autonomo è prodotto in `target/release/san-usb-viewer.exe`. Lo script `build.ps1` crea anche una distribuzione in `dist/`. Per l'inventario non è necessaria alcuna modifica del driver o installazione di WinUSB: l'applicazione non richiede interfacce del dispositivo.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

La cartella di destinazione per un'acquisizione deve esistere. La versione finale è un'applicazione Windows senza console; preferisco `cargo run` in fase di sviluppo per l'inventario della riga di comando.

## Interpretare i risultati

La **velocità negoziata** è quella della connessione e non un parametro di riferimento della velocità di copia. Le velocità utili dipendono in particolare dal protocollo, dalla periferica e dagli altri dispositivi che condividono il bus. Le velocità di più dispositivi non si sommano per calcolare la larghezza di banda disponibile.

Un cavo USB passivo generalmente non è un dispositivo enumerabile. Questa versione non legge il suo e-marker, né la potenza USB Power Delivery, né il tipo di connettore. Non ha quindi la pretesa di certificarne la velocità massima. La diagnosi si basa sul collegamento osservato e, se disponibile, sugli indicatori di Windows. Un collegamento lento può essere limitato dal cavo, dalla porta, dall'hub o dal dispositivo. Il numero di versione del descrittore USB non viene mai utilizzato come velocità massima.

La categoria SuperSpeedPlus in questo caso non distingue tra le modalità 10 e 20 Gb/s. USB4/Thunderbolt e i relativi tunnel non vengono ispezionati. Le porte senza dispositivo collegato non vengono inventariate. Un cavo di ricarica senza dati non crea un dispositivo visibile. I dispositivi interni (Bluetooth, fotocamera integrata, ecc.) potrebbero essere visualizzati nella struttura USB.

Le informazioni inaccessibili vengono visualizzate come non disponibili. Il produttore e le interfacce non sono sempre esposti da Windows. Gli eventi che si verificano interamente tra le scansioni potrebbero passare inosservati. Il registro è in memoria e inizia dopo il primo inventario riuscito.

Gli identificatori del bus e i percorsi delle porte descrivono la topologia del sistema. Un connettore fisico può avere due percorsi separati per USB 2 e SuperSpeed.

## Architettura e validazione

`scanner.rs` esegue l'inventario; `windows_probe.rs` incapsula le richieste di lettura native dagli hub; `model.rs` contiene dati e diagnostica. `app.rs` gestisce le attività e la selezione in background; `views.rs`, `inspector.rs`, `connection.rs` e `theme.rs` disegnano l'interfaccia con egui/eframe. L'interfaccia rimane interattiva durante una scansione e conserva l'ultima lettura riuscita se la scansione successiva fallisce.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

Le librerie sono multipiattaforma e il codice Windows è condizionale. Le versioni macOS/Linux non sono convalidate in questa prima consegna.

## Riservatezza

L'inventario funziona localmente, in sola lettura, senza telemetria o servizi di rete. I collegamenti della guida si aprono solo con un clic. I report JSON possono contenere numeri di serie e identificatori di sistema: la loro condivisione dipende da te.

## Riferimenti tecnici

- [nusb: enumerazione e informazioni USB](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

Licenza del MIT. Gli annessi restano soggetti alle rispettive licenze.

Il repository pubblico contiene solo le fonti, le traduzioni e i font gratuiti necessari per la compilazione. Le letture USB, le acquisizioni diagnostiche, le esportazioni, le preferenze locali, le cache e i file binari di sviluppo sono esclusi da `.gitignore`. Per segnalare un problema, utilizzare la modalità `--demo` o rimuovere i numeri di serie e gli ID di sistema dai report prima di condividerli.

## Lingue — versione 0.2.0

L'applicazione include **50 lingue di interfaccia**. Fare clic sul codice della lingua (`FR`, `EN`, ecc.) in alto a destra per aprire lo switcher. Puoi cercare una lingua in base al nome nativo, al nome inglese o al codice. La modifica è immediata e la scelta viene memorizzata in `%APPDATA%/SanUsbViewer/preferences.json` sotto Windows. Senza preferenza viene utilizzata la lingua del sistema; Una lingua non supportata ritorna all'inglese.

La selezione privilegia le lingue complessivamente più parlate (lingua materna e seconda lingua), raggruppando le varianti regionali. Non si tratta di una classificazione numerica: i risultati variano a seconda del raggruppamento dei dialetti. Le varianti selezionate includono il cinese semplificato, l'arabo standard e le scritture latine di Fulani e uzbeko; Punjabi usa Gurmukhi.

Inglese, cinese, hindi, spagnolo, francese, arabo, bengalese, portoghese, russo, urdu, indonesiano, tedesco, giapponese, marathi, vietnamita, telugu, hausa, turco, punjabi, swahili, filippino, tamil, persiano, coreano, amarico, tailandese, giavanese, italiano, gujarati, kannada, bhojpuri, birmano, polacco, yoruba, lingala, oromo, fulani, malayalam, odia, ucraino, Sindhi, uzbeko, pashto, sundanese, malese, igbo, nepalese, azero, maithili e somalo.

I menu copiati, la diagnostica, i testi di aiuto, i messaggi di attività e i dettagli vengono localizzati. I nomi e gli identificatori forniti dall'hardware e dal sistema vengono conservati. L'esportazione JSON conserva i dati dello schema e dell'inventario, indipendentemente dalla lingua di visualizzazione. Le unità tecniche rimangono in **bit/s** (Mbit/s e Gbit/s) per evitare confusione con i byte.

I cataloghi francese e inglese servono come riferimento. Altre lingue sono **traduzioni automatiche**, che potrebbero richiedere correzioni linguistiche. I file `locales/*.json` sono inclusi nella compilazione e possono essere corretti e quindi ricompilati. Nessuna traduzione in rete viene effettuata dall'applicazione: le 50 lingue e i font Noto sono incorporati, sotto la licenza SIL Open Font. Il testo arabo utilizza segmenti Unicode bidirezionali e ritorni a capo calcolati prima della formattazione.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` può fare riferimento a un altro file delle preferenze per il test o un'installazione portatile. Qui viene salvato solo il codice della lingua.
