use std::time::{SystemTime, UNIX_EPOCH};

use nusb::MaybeFuture;

use crate::model::{LinkSpeed, Snapshot, UsbBus, UsbDevice, UsbInterface};

pub(crate) fn scan() -> Result<Snapshot, String> {
    let devices = nusb::list_devices()
        .wait()
        .map_err(|e| format!("Impossible de lire les périphériques USB : {e}"))?;
    let mut snapshot = Snapshot {
        timestamp: now(),
        ..Default::default()
    };
    match nusb::list_buses().wait() {
        Ok(buses) => {
            snapshot.buses = buses
                .map(|b| UsbBus {
                    id: b.bus_id().to_owned(),
                    name: b.system_name().unwrap_or("Contrôleur USB").to_owned(),
                    controller: b
                        .controller_type()
                        .map_or_else(|| "USB".into(), |t| format!("{t:?}")),
                })
                .collect()
        }
        Err(e) => snapshot
            .warnings
            .push(format!("Contrôleurs partiellement disponibles : {e}")),
    }
    snapshot.devices = devices
        .map(|d| {
            let name = d
                .product_string()
                .filter(|s| !s.trim().is_empty())
                .map_or_else(
                    || {
                        format!(
                            "Périphérique USB {:04X}:{:04X}",
                            d.vendor_id(),
                            d.product_id()
                        )
                    },
                    str::to_owned,
                );
            #[cfg(target_os = "windows")]
            let (driver, instance_id) = (
                d.driver().map(str::to_owned),
                Some(d.instance_id().to_string_lossy().into_owned()),
            );
            #[cfg(not(target_os = "windows"))]
            let (driver, instance_id) = (None, None);
            #[cfg(windows)]
            let capabilities =
                crate::windows_probe::capabilities(d.parent_instance_id(), d.port_number());
            #[cfg(not(windows))]
            let capabilities = None;
            UsbDevice {
                id: format!(
                    "{}:{:?}:{:04x}:{:04x}",
                    d.bus_id(),
                    d.port_chain(),
                    d.vendor_id(),
                    d.product_id()
                ),
                name,
                manufacturer: d.manufacturer_string().map(str::to_owned),
                serial: d.serial_number().map(str::to_owned),
                vendor_id: d.vendor_id(),
                product_id: d.product_id(),
                usb_version: format_bcd(d.usb_version()),
                device_version: format_bcd(d.device_version()),
                class: d.class(),
                bus: d.bus_id().to_owned(),
                ports: d.port_chain().to_vec(),
                address: d.device_address(),
                speed: LinkSpeed::from_usb(d.speed()),
                driver,
                instance_id,
                capabilities,
                interfaces: d
                    .interfaces()
                    .map(|i| UsbInterface {
                        number: i.interface_number(),
                        class: i.class(),
                        subclass: i.subclass(),
                        protocol: i.protocol(),
                        name: i.interface_string().unwrap_or("").to_owned(),
                    })
                    .collect(),
            }
        })
        .collect();
    snapshot
        .devices
        .sort_by(|a, b| (&a.bus, &a.ports).cmp(&(&b.bus, &b.ports)));
    if snapshot.devices.is_empty() && !snapshot.buses.is_empty() {
        snapshot.warnings.push("Des contrôleurs USB sont visibles, mais aucun périphérique n’est lisible. Si des appareils sont branchés, vérifiez les permissions d’accès aux hubs (un bac à sable peut les bloquer).".into());
    }
    for d in &snapshot.devices {
        if !snapshot.buses.iter().any(|b| b.id == d.bus) {
            snapshot.buses.push(UsbBus {
                id: d.bus.clone(),
                name: "Contrôleur USB".into(),
                controller: "USB".into(),
            });
        }
    }
    Ok(snapshot)
}

fn format_bcd(version: u16) -> String {
    format!("{:X}.{:02X}", version >> 8, version & 0xff)
}

pub(crate) fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Sample data is strictly opt-in and always marked in the interface and export.
pub(crate) fn demo() -> Snapshot {
    let device = |name: &str, ports: &[u8], class, speed, vid, pid| UsbDevice {
        id: format!("demo:{ports:?}"),
        name: name.into(),
        bus: "demo-bus".into(),
        ports: ports.into(),
        class,
        speed,
        vendor_id: vid,
        product_id: pid,
        usb_version: "3.20".into(),
        device_version: "1.00".into(),
        driver: Some("Données de démonstration".into()),
        ..Default::default()
    };
    Snapshot {
        timestamp: now(),
        demo: true,
        buses: vec![UsbBus {
            id: "demo-bus".into(),
            name: "Contrôleur hôte USB xHCI".into(),
            controller: "XHCI".into(),
        }],
        devices: vec![
            device("Hub USB 3.0", &[3], 9, LinkSpeed::Super, 0x2109, 0x0817),
            device("SSD portable", &[3, 1], 8, LinkSpeed::Super, 0x0781, 0x558c),
            device(
                "Clavier mécanique",
                &[3, 2],
                3,
                LinkSpeed::Full,
                0x046d,
                0xc343,
            ),
            device("Caméra USB", &[5], 14, LinkSpeed::High, 0x046d, 0x082d),
            device("Interface audio", &[7], 1, LinkSpeed::High, 0x1235, 0x8210),
        ],
        warnings: vec![],
    }
}
