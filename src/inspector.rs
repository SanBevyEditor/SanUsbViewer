use crate::{
    app::UsbApp,
    connection,
    i18n::{self, tr},
    model::{UsbDevice, class_name},
    theme::{self, *},
};
use eframe::egui::{self, Align, Color32, Layout, Stroke, Vec2};

pub(crate) fn device_detail(ui: &mut egui::Ui, app: &mut UsbApp, device: &UsbDevice) {
    label(
        ui,
        muted(format!(
            "{}   /   {}",
            tr("CONNEXION SÉLECTIONNÉE"),
            i18n::route(device)
        ))
        .size(10.5)
        .strong(),
    );
    ui.add_space(4.);
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            label(
                ui,
                crate::localized_text::raw(i18n::device_name(device, app.demo))
                    .size(24.)
                    .strong(),
            );
            label(
                ui,
                muted(format!(
                    "{}   ·   VID {:04X} / PID {:04X}",
                    tr(device.category()),
                    device.vendor_id,
                    device.product_id
                ))
                .size(12.),
            );
        });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            theme::badge(ui, "CONNECTÉ", ACCENT)
        });
    });
    ui.add_space(14.);
    speed_card(ui, device);
    ui.add_space(15.);
    card().show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        label(ui, rich("Le trajet de votre connexion").size(15.).strong());
        connection::draw(ui, &app.snapshot, device);
    });
    ui.add_space(15.);
    ui.horizontal(|ui| {
        for (index, name) in ["Diagnostic", "Identité & interfaces"].iter().enumerate() {
            if ui
                .selectable_label(app.detail_tab == index, rich(*name))
                .clicked()
            {
                app.detail_tab = index;
            }
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.small_button(rich("Copier les détails")).clicked() {
                let (title, message) = device.diagnostic();
                ui.ctx().copy_text(format!(
                    "{}\n{}\nVID:PID {:04X}:{:04X}\n{}\n{} : {}\n{}\n{}",
                    if app.demo {
                        format!("San USB Viewer · {}", tr("DÉMONSTRATION"))
                    } else {
                        "San USB Viewer".into()
                    },
                    device.name,
                    device.vendor_id,
                    device.product_id,
                    i18n::route(device),
                    tr("Liaison"),
                    tr(device.speed.label()),
                    tr(title),
                    tr(message)
                ));
                app.notice = Some("Détails copiés dans le presse-papiers".into());
            }
        });
    });
    ui.add_space(4.);
    if app.detail_tab == 0 {
        diagnostic(ui, device);
    } else {
        identity(ui, device);
    }
    for warning in &app.snapshot.warnings {
        ui.add_space(8.);
        label(ui, rich(i18n::message(warning)).color(AMBER).size(12.));
    }
}

fn speed_card(ui: &mut egui::Ui, device: &UsbDevice) {
    egui::Frame::new()
        .fill(Color32::from_rgb(19, 44, 47))
        .stroke(Stroke::new(1., Color32::from_rgb(41, 85, 82)))
        .corner_radius(14)
        .inner_margin(22)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 3.;
                    label(
                        ui,
                        rich("VITESSE NÉGOCIÉE").size(10.5).strong().color(ACCENT),
                    );
                    label(
                        ui,
                        rich(device.speed.label()).size(43.).strong().color(TEXT),
                    );
                    label(ui, rich(device.speed.standard()).size(13.).color(ACCENT));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(Vec2::new(110., 84.), egui::Sense::hover());
                    draw_speed_meter(ui.painter(), rect, device);
                });
            });
            ui.add_space(10.);
            ui.separator();
            ui.add_space(3.);
            ui.horizontal_wrapped(|ui| {
                label(ui, muted("Signalisation de la liaison").size(11.));
                label(ui, muted("·").size(11.));
                label(
                    ui,
                    muted("Le débit utile dépend du périphérique et du protocole.").size(11.),
                );
            });
        });
}

fn draw_speed_meter(painter: &egui::Painter, rect: egui::Rect, device: &UsbDevice) {
    let center = rect.center() + Vec2::new(0., 15.);
    let radius = 43.;
    let color = speed_color(device.speed);
    let amount = device
        .speed
        .mbps()
        .map_or(0., |v| ((v.log10() + 1.) / 5.).clamp(0.06, 1.));
    for i in 0..30 {
        let angle = std::f32::consts::PI * (1.08 + 0.84 * i as f32 / 29.);
        let direction = Vec2::angled(angle);
        painter.line_segment(
            [
                center + direction * (radius - 8.),
                center + direction * radius,
            ],
            Stroke::new(
                3.,
                if (i as f32 / 30.) < amount {
                    color
                } else {
                    BORDER
                },
            ),
        );
    }
    theme::usb_icon(
        painter,
        egui::Rect::from_center_size(center - Vec2::new(0., 8.), Vec2::splat(32.)),
        color,
    );
}

fn diagnostic(ui: &mut egui::Ui, device: &UsbDevice) {
    card().show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        let (title, message) = device.diagnostic();
        ui.horizontal(|ui| {
            label(ui, rich("●").color(speed_color(device.speed)).size(11.));
            label(ui, rich(title).size(16.).strong());
        });
        label(ui, message);
        ui.add_space(12.);
        ui.separator();
        ui.add_space(6.);
        ui.columns(2, |cols| {
            label(&mut cols[0], muted("CE QUE L’ON OBSERVE").size(10.5).strong());
            label(&mut cols[0], format!("{} : {}", tr("Liaison"), tr(device.speed.label())));
            label(&mut cols[0], format!("{} : {}", tr("Descripteur USB"), device.usb_version));
            if let Some(capabilities) = &device.capabilities {
                label(&mut cols[0], if capabilities.device_super_speed_plus { "Périphérique compatible SuperSpeedPlus" }
                    else if capabilities.device_super_speed { "Périphérique compatible SuperSpeed" }
                    else { "SuperSpeed non signalé par Windows" });
            }
            label(&mut cols[1], muted("CE QUI RESTE INCONNU").size(10.5).strong());
            label(&mut cols[1], "Vitesse maximale du câble seul");
            label(&mut cols[1], "Puissance de charge / e-marker");
        });
        ui.add_space(8.);
        label(ui, muted("La version du descripteur USB décrit le périphérique. Elle n’est pas une mesure de vitesse, ni une certification du câble.").size(11.));
    });
}

fn identity(ui: &mut egui::Ui, device: &UsbDevice) {
    card().show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        egui::Grid::new("device_properties")
            .num_columns(2)
            .spacing([22., 12.])
            .striped(true)
            .show(ui, |ui| {
                for (name, value) in [
                    (
                        "Fabricant",
                        device
                            .manufacturer
                            .clone()
                            .unwrap_or_else(|| "Non fourni par le système".into()),
                    ),
                    (
                        "VID / PID",
                        format!("{:04X} / {:04X}", device.vendor_id, device.product_id),
                    ),
                    (
                        "Numéro de série",
                        device
                            .serial
                            .clone()
                            .unwrap_or_else(|| "Non disponible".into()),
                    ),
                    ("Version USB (descripteur)", device.usb_version.clone()),
                    ("Révision du périphérique", device.device_version.clone()),
                    (
                        "Classe",
                        format!("0x{:02X} · {}", device.class, tr(device.category())),
                    ),
                    ("Adresse sur le bus", device.address.to_string()),
                    (
                        "Pilote",
                        device
                            .driver
                            .clone()
                            .unwrap_or_else(|| "Non disponible".into()),
                    ),
                    ("Chemin des ports", i18n::route(device)),
                ] {
                    label(ui, muted(name));
                    label(ui, value);
                    ui.end_row();
                }
            });
        if let Some(instance) = &device.instance_id {
            ui.add_space(14.);
            egui::CollapsingHeader::new(rich("Identifiant système")).show(ui, |ui| {
                ui.add(egui::Label::new(rich(instance).monospace().size(11.)).wrap());
            });
        }
        ui.add_space(14.);
        label(ui, rich("Interfaces exposées").strong());
        if device.interfaces.is_empty() {
            label(
                ui,
                muted("Aucune interface détaillée exposée par le système pour ce pilote."),
            );
        } else {
            for interface in &device.interfaces {
                label(
                    ui,
                    format!(
                        "#{:02} · {} · {:02X}/{:02X}/{:02X}  {}",
                        interface.number,
                        tr(class_name(interface.class)),
                        interface.class,
                        interface.subclass,
                        interface.protocol,
                        interface.name
                    ),
                );
            }
        }
    });
}
