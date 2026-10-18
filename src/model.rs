use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub(crate) enum LinkSpeed {
    Low,
    Full,
    High,
    Super,
    SuperPlus,
    #[default]
    Unknown,
}

impl LinkSpeed {
    pub(crate) fn from_usb(speed: Option<nusb::Speed>) -> Self {
        match speed {
            Some(nusb::Speed::Low) => Self::Low,
            Some(nusb::Speed::Full) => Self::Full,
            Some(nusb::Speed::High) => Self::High,
            Some(nusb::Speed::Super) => Self::Super,
            Some(nusb::Speed::SuperPlus) => Self::SuperPlus,
            _ => Self::Unknown,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Low => "1,5 Mb/s",
            Self::Full => "12 Mb/s",
            Self::High => "480 Mb/s",
            Self::Super => "5 Gb/s",
            Self::SuperPlus => "10 Gb/s ou +",
            Self::Unknown => "Non disponible",
        }
    }

    pub(crate) fn standard(self) -> &'static str {
        match self {
            Self::Low => "Low Speed",
            Self::Full => "Full Speed",
            Self::High => "High Speed",
            Self::Super => "SuperSpeed",
            Self::SuperPlus => "SuperSpeedPlus",
            Self::Unknown => "Vitesse inconnue",
        }
    }

    pub(crate) fn mbps(self) -> Option<f32> {
        match self {
            Self::Low => Some(1.5),
            Self::Full => Some(12.0),
            Self::High => Some(480.0),
            Self::Super => Some(5000.0),
            Self::SuperPlus => Some(10000.0),
            Self::Unknown => None,
        }
    }

    pub(crate) fn is_super(self) -> bool {
        matches!(self, Self::Super | Self::SuperPlus)
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct UsbInterface {
    pub(crate) number: u8,
    pub(crate) class: u8,
    pub(crate) subclass: u8,
    pub(crate) protocol: u8,
    pub(crate) name: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct UsbDevice {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) manufacturer: Option<String>,
    pub(crate) serial: Option<String>,
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,
    pub(crate) usb_version: String,
    pub(crate) device_version: String,
    pub(crate) class: u8,
    pub(crate) bus: String,
    pub(crate) ports: Vec<u8>,
    pub(crate) address: u8,
    pub(crate) speed: LinkSpeed,
    pub(crate) driver: Option<String>,
    pub(crate) instance_id: Option<String>,
    pub(crate) interfaces: Vec<UsbInterface>,
    pub(crate) capabilities: Option<UsbCapabilities>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct UsbCapabilities {
    pub(crate) device_super_speed: bool,
    pub(crate) device_super_speed_plus: bool,
    pub(crate) port_usb3_protocol: bool,
}

impl UsbDevice {
    pub(crate) fn category(&self) -> &'static str {
        if self.class == 0
            && let Some(interface) = self.interfaces.first()
        {
            return class_name(interface.class);
        }
        class_name(self.class)
    }

    pub(crate) fn is_hub(&self) -> bool {
        self.class == 9
    }

    pub(crate) fn matches(&self, query: &str) -> bool {
        let text = format!(
            "{} {} {:04x}:{:04x} {} {}",
            self.name,
            self.manufacturer.as_deref().unwrap_or(""),
            self.vendor_id,
            self.product_id,
            self.serial.as_deref().unwrap_or(""),
            self.category()
        );
        text.to_lowercase().contains(&query.to_lowercase())
    }

    pub(crate) fn diagnostic(&self) -> (&'static str, &'static str) {
        if self.is_hub() && self.speed == LinkSpeed::High {
            return (
                "Un hub sur la branche USB 2.0",
                "Un hub USB 3 peut exposer une branche USB 2.0 et une branche SuperSpeed distinctes. Cette entrée à 480 Mb/s ne suffit donc pas à conclure que le câble limite le hub. Vérifiez aussi les autres branches dans l’arborescence.",
            );
        }
        if !self.speed.is_super()
            && self.speed != LinkSpeed::Unknown
            && self
                .capabilities
                .as_ref()
                .is_some_and(|c| c.device_super_speed)
        {
            return (
                "Le périphérique peut aller plus vite",
                "Windows signale un périphérique compatible SuperSpeed, mais la liaison négociée reste en USB 2.0. Testez un port USB 3 direct, puis un autre câble. Le câble, le port ou un hub peut limiter la connexion.",
            );
        }
        match self.speed {
            LinkSpeed::Unknown => (
                "Vitesse indisponible",
                "Le système ne fournit pas la vitesse de cette connexion. Essayez un autre port et actualisez l’analyse.",
            ),
            LinkSpeed::Low | LinkSpeed::Full => (
                "Connexion basse vitesse",
                "Cette vitesse est courante pour un clavier, une souris ou un périphérique audio. Elle ne permet pas d’évaluer les capacités du câble.",
            ),
            LinkSpeed::High => (
                "Une liaison USB 2.0",
                "La connexion utilise High Speed. Si votre périphérique est compatible USB 3, comparez avec un câble et un port USB 3 connus, sans hub intermédiaire.",
            ),
            LinkSpeed::Super => (
                "SuperSpeed est actif",
                "La liaison fonctionne à 5 Gb/s. Le trajet prend donc en charge SuperSpeed. La vitesse maximale du câble seul reste inconnue.",
            ),
            LinkSpeed::SuperPlus => (
                "SuperSpeedPlus est actif",
                "Le système signale une liaison à 10 Gb/s ou plus. Cette information ne distingue pas les modes 10 et 20 Gb/s et ne mesure pas le débit de copie.",
            ),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct UsbBus {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) controller: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct Snapshot {
    pub(crate) timestamp: u64,
    pub(crate) demo: bool,
    pub(crate) buses: Vec<UsbBus>,
    pub(crate) devices: Vec<UsbDevice>,
    pub(crate) warnings: Vec<String>,
}

impl Snapshot {
    pub(crate) fn device(&self, id: &str) -> Option<&UsbDevice> {
        self.devices.iter().find(|d| d.id == id)
    }

    pub(crate) fn fastest(&self) -> LinkSpeed {
        self.devices
            .iter()
            .map(|d| d.speed)
            .max_by(|a, b| a.mbps().unwrap_or(0.0).total_cmp(&b.mbps().unwrap_or(0.0)))
            .unwrap_or_default()
    }

    pub(crate) fn ancestors(&self, device: &UsbDevice) -> Vec<&UsbDevice> {
        let mut hubs: Vec<_> = self
            .devices
            .iter()
            .filter(|d| {
                d.bus == device.bus
                    && d.is_hub()
                    && d.ports.len() < device.ports.len()
                    && device.ports.starts_with(&d.ports)
            })
            .collect();
        hubs.sort_by_key(|d| d.ports.len());
        hubs
    }
}

pub(crate) fn class_name(class: u8) -> &'static str {
    match class {
        0 => "Composite",
        1 => "Audio",
        2 | 10 => "Communication",
        3 => "Clavier / souris",
        6 => "Image",
        7 => "Imprimante",
        8 => "Stockage",
        9 => "Hub USB",
        14 => "Vidéo",
        224 => "Sans fil",
        239 => "Composite",
        255 => "Spécifique fabricant",
        _ => "Périphérique USB",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ancestry_excludes_siblings_and_other_controllers() {
        let hub = |bus: &str, ports: &[u8]| UsbDevice {
            bus: bus.into(),
            ports: ports.into(),
            class: 9,
            ..Default::default()
        };
        let snapshot = Snapshot {
            devices: vec![
                hub("A", &[1]),
                hub("A", &[2]),
                hub("B", &[1]),
                hub("A", &[]),
            ],
            ..Default::default()
        };
        let target = hub("A", &[1, 3]);
        let paths: Vec<_> = snapshot
            .ancestors(&target)
            .iter()
            .map(|h| h.ports.clone())
            .collect();
        assert_eq!(paths, vec![vec![], vec![1]]);
    }

    #[test]
    fn unknown_speed_does_not_become_zero_speed() {
        assert_eq!(LinkSpeed::Unknown.mbps(), None);
    }

    #[test]
    fn search_accepts_hex_ids_and_case_insensitive_names() {
        let device = UsbDevice {
            name: "Portable SSD".into(),
            vendor_id: 0x0781,
            product_id: 0x558c,
            ..Default::default()
        };
        assert!(device.matches("ssd") && device.matches("0781:558C"));
    }

    #[test]
    fn usb2_companion_hub_does_not_produce_a_cable_bottleneck_warning() {
        let hub = UsbDevice {
            class: 9,
            speed: LinkSpeed::High,
            capabilities: Some(UsbCapabilities {
                device_super_speed: true,
                device_super_speed_plus: false,
                port_usb3_protocol: false,
            }),
            ..Default::default()
        };
        assert_eq!(hub.diagnostic().0, "Un hub sur la branche USB 2.0");
    }

    #[test]
    fn superspeed_device_negotiated_at_high_speed_reports_a_path_limitation() {
        let device = UsbDevice {
            class: 8,
            speed: LinkSpeed::High,
            capabilities: Some(UsbCapabilities {
                device_super_speed: true,
                device_super_speed_plus: false,
                port_usb3_protocol: false,
            }),
            ..Default::default()
        };
        assert_eq!(
            device.diagnostic().0,
            "Le périphérique peut aller plus vite"
        );
    }
}
