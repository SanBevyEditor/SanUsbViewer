# San USB Viewer

**English** · [English](README.en.md) · [Français](../../README.md) · [50 languages](../README.md)

> Machine translations; corrections are welcome.

**Current version: `0.2.0`** — MIT license.

![San USB Viewer](../../assets/screenshots/SanUsbViewer.jpg)

Native USB Explorer written in **Rust**, with a dark interface, visual topology and readable diagnostics. First version intended for Windows 10/11.

## Origin of the project

Video at the origin of the project: [https://www.youtube.com/watch?v=xk6GxQEmkus](https://www.youtube.com/watch?v=xk6GxQEmkus).

The project is inspired by the uses of **WhatCable** on Apple and **USB Device Tree Viewer** on Windows: understanding peripherals, hubs and the speeds of their USB connections. This is an independent implementation.

## Features

- Actual detection of USB devices and controllers via `nusb`.
- Tree structure by bus and port path, with hub depth.
- Search by name, serial number or VID:PID; USB 3+ filters and hubs.
- Negotiated link speed: 1.5 / 12 / 480 Mb/s, 5 Gb/s,
or category SuperSpeedPlus (10 Gb/s or more).
- Connection path diagram: computer, intermediate hubs, link and device.
- Under Windows, additional interrogation of the hub by IOCTL to find out the
SuperSpeed capabilities reported by the device. A SuperSpeed compatible device connected via USB 2.0 produces a path limitation diagnostic. USB 2 companion hubs of USB 3 hubs are treated separately to avoid attributing their 480 Mb/s branch to a cable fault.
- VID/PID identifiers, USB descriptor, revision, driver, serial and interfaces,
when the system exposes this information.
- Automatic update every three seconds, or manual with F5.
- Session log of the last 200 observed changes.
- Export the complete inventory in JSON (Ctrl+E) and copy the selected details.
- Integrated guide and explicit demonstration mode. Detection failures are reported without silently switching to demonstration data.

## Build and run

With Rust **1.95+**, the MSVC toolchain and Visual Studio C++ tools on Windows. The examples use RTK to filter the output of commands; without RTK, just remove the `rtk` prefix. The distribution script accepts both cases.

```powershell
rtk cargo run --release
```

The standalone binary is produced in `target/release/san-usb-viewer.exe`. The `build.ps1` script also creates a distribution in `dist/SanUsbViewer/`. No driver modification or installation of WinUSB is necessary for the inventory: the application does not claim device interfaces.

```powershell
# Démonstration avec données fictives, identifiées dans l’interface et l’export
rtk cargo run -- --demo

# Inventaire JSON sur la sortie standard (utiliser le binaire de développement)
rtk cargo run -- --scan

# Capture du rendu réel de l’application, puis fermeture automatique
rtk cargo run -- --screenshot artifacts/live.png
```

The destination folder for a capture must exist. The release version is a console-less Windows application; prefer `cargo run` in development for command line inventory.

## Interpret the results

The **negotiated speed** is that of the connection, and not a benchmark of copy speed. The useful speeds depend in particular on the protocol, the peripheral and the other devices sharing the bus. Speeds from multiple devices do not add up to calculate available bandwidth.

A passive USB cable is generally not an enumerable device. This version does not read its e-marker, nor the USB Power Delivery power, nor its connector type. It therefore does not claim to certify its maximum speed. The diagnosis is based on the observed link and, if available, on Windows indicators. A slow link may be limited by the cable, port, hub, or device. The USB descriptor version number is never used as the maximum speed.

The SuperSpeedPlus category does not distinguish between 10 and 20 Gb/s modes here. USB4/Thunderbolt and their tunnels are not inspected. Ports without a device attached are not inventoried. A charging cable without data does not create a visible device. Internal devices (Bluetooth, built-in camera, etc.) may appear in the USB tree.

Inaccessible information is displayed as unavailable. The manufacturer and interfaces are not always exposed by Windows. Events that occur entirely between scans may go unnoticed. The log is in memory and begins after the first successful inventory.

Bus identifiers and port paths describe the system topology. A physical connector can have two separate paths for USB 2 and SuperSpeed.

## Architecture and validation

`scanner.rs` performs inventory; `windows_probe.rs` encapsulates native read requests from hubs; `model.rs` contains data and diagnostics. `app.rs` handles background tasks and selection; `views.rs`, `inspector.rs`, `connection.rs` and `theme.rs` draw the interface with egui/eframe. The interface remains interactive during a scan and retains the last successful read if the next scan fails.

```powershell
rtk cargo fmt --check
rtk cargo clippy --all-targets -- -D warnings
rtk cargo test --workspace
```

The libraries are cross-platform and the Windows code is conditional. The macOS/Linux versions are not validated in this first delivery.

## Confidentiality

Inventory works locally, read-only, without telemetry or network services. The guide links only open on click. JSON reports can contain serial numbers and system identifiers: sharing them is up to you.

## Technical references

- [nusb: USB enumeration and information](https://docs.rs/nusb/latest/nusb/struct.DeviceInfo.html)
- [Microsoft: IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbioctl/ni-usbioctl-ioctl_usb_get_node_connection_information_ex_v2)
- [egui/eframe](https://github.com/emilk/egui)

MIT license. Dependencies remain subject to their respective licenses.

The public repository contains only the sources, translations and free fonts necessary for compilation. USB readings, diagnostic captures, exports, local preferences, caches and development binaries are excluded by `.gitignore`. To report a problem, use mode `--demo` or remove serial numbers and system IDs from your reports before sharing them.

## Languages ​​— version 0.2.0

The application includes **50 interface languages**. Click the language code (`FR`, `EN`, etc.) at the top right to open the switcher. You can search for a language by its native name, its English name or its code. The change is immediate and the choice is stored in `%APPDATA%/SanUsbViewer/preferences.json` under Windows. Without preference, the system language is used; An unsupported language reverts to English.

The selection favors the most spoken languages ​​in total (mother tongue and second language), grouping together regional variants. This is not a numerical classification: the results vary depending on the grouping of dialects. The variants selected include Simplified Chinese, Standard Arabic and the Latin scripts of Fulani and Uzbek; Punjabi uses Gurmukhi.

English, Chinese, Hindi, Spanish, French, Arabic, Bengali, Portuguese, Russian, Urdu, Indonesian, German, Japanese, Marathi, Vietnamese, Telugu, Hausa, Turkish, Punjabi, Swahili, Filipino, Tamil, Persian, Korean, Amharic, Thai, Javanese, Italian, Gujarati, Kannada, Bhojpuri, Burmese, Polish, Yoruba, Lingala, Oromo, Fulani, Malayalam, Odia, Ukrainian, Sindhi, Uzbek, Pashto, Sundanese, Malay, Igbo, Nepali, Azeri, Maithili and Somali.

Menus, diagnostics, help texts, activity messages and copied details are localized. Names and identifiers provided by the hardware and system are retained. The JSON export keeps its schema and inventory data, regardless of the display language. Technical units remain in **bit/s** (Mbit/s and Gbit/s) to avoid confusion with bytes.

The French and English catalogs serve as a reference. Other languages ​​are **automatic translations**, which may require linguistic corrections. `locales/*.json` files are included in the compilation and can be corrected and then recompiled. No network translation is carried out by the application: the 50 languages ​​and Noto fonts are embedded, under the SIL Open Font License. Arabic text uses bidirectional Unicode segments and newlines calculated before formatting.

```powershell
# Forcer une langue pour ce lancement, sans remplacer le choix enregistré
rtk cargo run -- --lang ja

# Vérifier les catalogues, les paramètres et les unités de vitesse
rtk python tools/validate_locales.py

# Capture du guide en arabe
rtk cargo run -- --demo --lang ar --page guide --screenshot artifacts/guide-ar.png
```

`SAN_USB_VIEWER_CONFIG` may refer to another preferences file for testing or a portable installation. Only the language code is saved there.
