use crate::{
    app::{DeviceFilter, Page, UsbApp},
    i18n::{self, tr},
    inspector,
    model::LinkSpeed,
    theme::{self, *},
};
use eframe::egui::{self, Align, Color32, Layout, Stroke, Vec2};

pub(crate) fn header(ui: &mut egui::Ui, app: &mut UsbApp) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(36.), egui::Sense::hover());
        theme::usb_icon(ui.painter(), rect.shrink(3.), ACCENT);
        ui.vertical(|ui| {
            label(ui, rich("San USB Viewer").size(19.).strong());
            label(ui, muted("Chaque connexion, en clair.").size(11.));
        });
        ui.add_space(16.);
        for (page, label) in [
            (Page::Explorer, "Explorer"),
            (Page::Activity, "Activité"),
            (Page::Guide, "Guide & à propos"),
        ] {
            let active = app.page == page;
            let button = egui::Button::new(rich(label).color(if active { ACCENT } else { MUTED }))
                .fill(if active {
                    Color32::from_rgb(27, 55, 57)
                } else {
                    Color32::TRANSPARENT
                })
                .stroke(Stroke::NONE)
                .corner_radius(8);
            if ui.add(button).clicked() {
                app.page = page;
            }
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .button(crate::localized_text::raw(app.language_code.to_uppercase()).size(12.))
                .on_hover_text(rich("Choisir une langue"))
                .clicked()
            {
                app.language_picker = true;
            }
            if ui
                .add_enabled(
                    !app.snapshot.devices.is_empty(),
                    egui::Button::new(rich("↓  Exporter")).corner_radius(8),
                )
                .on_hover_text(rich("Enregistrer un rapport JSON · Ctrl+E"))
                .clicked()
            {
                app.export();
            }
            theme::badge(
                ui,
                if app.demo { "DÉMONSTRATION" } else { "LOCAL" },
                if app.demo { AMBER } else { ACCENT },
            );
        });
    });
}

pub(crate) fn status(ui: &mut egui::Ui, app: &mut UsbApp) {
    ui.horizontal(|ui| {
        label(
            ui,
            rich("●")
                .color(if app.error.is_some() { AMBER } else { ACCENT })
                .size(10.),
        );
        label(
            ui,
            muted(if app.demo {
                "Données fictives · mode démonstration"
            } else if app.scanning {
                "Analyse des connexions…"
            } else {
                "Données système · lecture seule"
            })
            .size(12.),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            label(
                ui,
                muted(format!("v{}   ·   Rust / egui", env!("CARGO_PKG_VERSION"))).size(11.),
            );
            ui.checkbox(&mut app.auto_refresh, rich("Surveillance · 3 s"));
        });
    });
}

pub(crate) fn explorer(ui: &mut egui::Ui, app: &mut UsbApp) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            heading(ui, "Vos connexions USB");
            label(ui, muted("Explorez le trajet. Comprenez la vitesse."));
        });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .add_enabled(
                    !app.scanning && !app.demo,
                    egui::Button::new(rich(if app.scanning {
                        "Analyse…"
                    } else {
                        "↻  Actualiser"
                    }))
                    .corner_radius(8),
                )
                .on_hover_text(rich("Relancer l’analyse · F5"))
                .clicked()
            {
                app.start_scan();
            }
        });
    });
    ui.add_space(15.);
    let device_count = app.snapshot.devices.iter().filter(|d| !d.is_hub()).count();
    let hub_count = app.snapshot.devices.iter().filter(|d| d.is_hub()).count();
    let super_count = app
        .snapshot
        .devices
        .iter()
        .filter(|d| d.speed.is_super() && !d.is_hub())
        .count();
    ui.columns(4, |cols| {
        stat(
            &mut cols[0],
            "PÉRIPHÉRIQUES",
            &device_count.to_string(),
            "connectés à cet ordinateur",
            TEXT,
        );
        stat(
            &mut cols[1],
            "LIAISONS SUPERSPEED",
            &super_count.to_string(),
            "périphériques à 5 Gb/s ou plus",
            ACCENT,
        );
        stat(
            &mut cols[2],
            "HUBS USB",
            &hub_count.to_string(),
            "dans l’arborescence",
            BLUE,
        );
        stat(
            &mut cols[3],
            "LIAISON LA PLUS RAPIDE",
            app.snapshot.fastest().label(),
            "signalisation, hors débit de copie",
            ACCENT,
        );
    });
    ui.add_space(20.);
    let panel = if i18n::language().rtl {
        egui::Panel::right("devices")
    } else {
        egui::Panel::left("devices")
    };
    panel
        .resizable(true)
        .default_size(320.)
        .size_range(265.0..=440.0)
        .frame(
            egui::Frame::new()
                .fill(PANEL)
                .corner_radius(14)
                .inner_margin(16),
        )
        .show(ui, |ui| tree(ui, app));
    egui::CentralPanel::default()
        .frame(egui::Frame::new().inner_margin(egui::Margin {
            left: 20,
            right: 0,
            top: 0,
            bottom: 0,
        }))
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("detail_scroll")
                .show(ui, |ui| {
                    if let Some(device) = app
                        .selected
                        .as_deref()
                        .and_then(|id| app.snapshot.device(id))
                        .cloned()
                    {
                        inspector::device_detail(ui, app, &device);
                    } else {
                        card().show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.add_space(60.);
                            heading(
                                ui,
                                if app.scanning {
                                    "À la découverte de vos appareils…"
                                } else {
                                    "Aucune connexion à afficher"
                                },
                            );
                            label(
                                ui,
                                muted("Branchez un périphérique USB puis actualisez l’analyse."),
                            );
                            for warning in &app.snapshot.warnings {
                                label(ui, rich(i18n::message(warning)).color(AMBER));
                            }
                            if ui.button(rich("Explorer la démonstration")).clicked() {
                                app.switch_demo();
                            }
                            ui.add_space(60.);
                        });
                    }
                });
        });
}

fn stat(ui: &mut egui::Ui, title: &str, value: &str, subtitle: &str, color: Color32) {
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(12)
        .inner_margin(17)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 3.;
            label(ui, muted(title).size(10.5).strong());
            label(ui, rich(value).size(27.).color(color).strong());
            label(ui, muted(subtitle).size(11.));
        });
}

fn tree(ui: &mut egui::Ui, app: &mut UsbApp) {
    ui.horizontal(|ui| {
        label(ui, rich("Arborescence").strong().size(16.));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            label(
                ui,
                muted(i18n::count("{count} bus", app.snapshot.buses.len())).size(11.),
            );
        });
    });
    ui.add_space(8.);
    ui.add(
        egui::TextEdit::singleline(&mut app.query)
            .hint_text(rich("Rechercher un nom, VID:PID…"))
            .desired_width(f32::INFINITY)
            .margin(Vec2::new(10., 8.)),
    );
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.;
        for (filter, name) in [
            (DeviceFilter::All, "Tous"),
            (DeviceFilter::SuperSpeed, "USB 3+"),
            (DeviceFilter::Hubs, "Hubs"),
        ] {
            if ui
                .selectable_label(app.filter == filter, rich(name))
                .clicked()
            {
                app.filter = filter;
            }
        }
    });
    ui.add_space(8.);
    ui.separator();
    egui::ScrollArea::vertical()
        .id_salt("tree_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let mut visible = 0;
            for (index, bus) in app.snapshot.buses.iter().enumerate() {
                let devices: Vec<_> = app
                    .snapshot
                    .devices
                    .iter()
                    .filter(|d| {
                        d.bus == bus.id
                            && d.matches(&app.query)
                            && match app.filter {
                                DeviceFilter::All => true,
                                DeviceFilter::SuperSpeed => d.speed.is_super(),
                                DeviceFilter::Hubs => d.is_hub(),
                            }
                    })
                    .collect();
                if devices.is_empty() {
                    continue;
                }
                visible += devices.len();
                ui.add_space(12.);
                egui::CollapsingHeader::new(
                    rich(format!(
                        "{} {:02} · {}",
                        tr("BUS"),
                        index + 1,
                        bus.controller
                    ))
                    .color(MUTED)
                    .size(11.)
                    .strong(),
                )
                .id_salt(&bus.id)
                .default_open(true)
                .show_unindented(ui, |ui| {
                    label(ui, muted(&bus.name).size(11.));
                    ui.add_space(4.);
                    for device in devices {
                        let selected = app.selected.as_deref() == Some(&device.id);
                        let (rect, _) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), 68.),
                            egui::Sense::hover(),
                        );
                        let response = ui.interact(
                            rect,
                            egui::Id::new(("usb_row", &device.id)),
                            egui::Sense::click(),
                        );
                        if selected || response.hovered() {
                            ui.painter().rect_filled(
                                rect,
                                8,
                                if selected {
                                    Color32::from_rgb(26, 55, 60)
                                } else {
                                    CARD
                                },
                            );
                        }
                        if selected {
                            ui.painter().rect_filled(
                                egui::Rect::from_min_size(
                                    rect.min + Vec2::new(0., 14.),
                                    Vec2::new(3., 40.),
                                ),
                                2,
                                ACCENT,
                            );
                        }
                        let indent = device.ports.len().saturating_sub(1).min(4) as f32 * 12.;
                        let icon_rect = egui::Rect::from_min_size(
                            rect.min + Vec2::new(12. + indent, 17.),
                            Vec2::splat(27.),
                        );
                        theme::usb_icon(ui.painter(), icon_rect, speed_color(device.speed));
                        let text_rect = egui::Rect::from_min_max(
                            rect.min + Vec2::new(50. + indent, 10.),
                            rect.max - Vec2::new(8., 5.),
                        );
                        let mut job = i18n::bidi_job(
                            &i18n::device_name(device, app.demo),
                            egui::FontId::proportional(13.),
                            if selected {
                                TEXT
                            } else {
                                Color32::from_rgb(192, 207, 220)
                            },
                        );
                        job.wrap.max_width = text_rect.width();
                        job.wrap.max_rows = 1;
                        let galley = ui.painter().layout_job(job);
                        ui.painter().galley(text_rect.min, galley, TEXT);
                        ui.painter().text(
                            text_rect.min + Vec2::new(0., 24.),
                            egui::Align2::LEFT_TOP,
                            format!(
                                "{}   ·   {}",
                                device.ports.last().map_or_else(
                                    || tr("Racine"),
                                    |p| format!("{} {p}", tr("Port"))
                                ),
                                tr(device.speed.label())
                            ),
                            egui::FontId::proportional(11.),
                            speed_color(device.speed),
                        );
                        if response
                            .on_hover_text(format!(
                                "{}\n{}\n{:04X}:{:04X}",
                                device.name,
                                i18n::route(device),
                                device.vendor_id,
                                device.product_id
                            ))
                            .clicked()
                        {
                            app.selected = Some(device.id.clone());
                            app.detail_tab = 0;
                        }
                    }
                });
            }
            if visible == 0 {
                ui.add_space(25.);
                label(
                    ui,
                    muted(if app.scanning {
                        "Lecture des connexions…"
                    } else {
                        "Aucun périphérique correspondant."
                    }),
                );
            }
        });
}

pub(crate) fn activity(ui: &mut egui::Ui, app: &mut UsbApp) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            heading(ui, "Le fil des connexions");
            label(
                ui,
                muted("Branchements et débranchements observés pendant cette session."),
            );
        });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.button(rich("Effacer le journal")).clicked() {
                app.activity.clear();
            }
        });
    });
    ui.add_space(22.);
    card().show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        if app.activity.is_empty() {
            ui.add_space(30.);
            label(ui, rich("À l’écoute de vos connexions").size(20.));
            label(ui, muted("Activez la surveillance, puis branchez ou débranchez un appareil."));
            label(ui, muted("Le journal conserve les 200 derniers changements. Les changements entre deux analyses peuvent passer inaperçus."));
            ui.add_space(30.);
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for item in &app.activity {
                    ui.horizontal(|ui| {
                        label(ui, rich(&item.time).monospace().color(MUTED));
                        label(ui, rich(if item.connected { "+" } else { "−" }).color(if item.connected { ACCENT } else { AMBER }));
                        label(ui, i18n::message(&item.message));
                    });
                    ui.separator();
                }
            });
        }
    });
}

pub(crate) fn guide(ui: &mut egui::Ui, app: &mut UsbApp) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        heading(ui, "Comprendre votre connexion");
        label(ui, muted("Quelques repères pour choisir le bon câble et le bon port."));
        ui.add_space(22.);
        card().show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            label(ui, rich("Une vitesse, toute une chaîne").size(21.).strong());
            label(ui, "Le contrôleur, le port, les hubs, le câble et le périphérique participent à la négociation. Remplacer un seul élément peut changer la vitesse obtenue.");
            ui.add_space(8.);
            for speed in [LinkSpeed::Low, LinkSpeed::Full, LinkSpeed::High, LinkSpeed::Super, LinkSpeed::SuperPlus] {
                ui.horizontal(|ui| {
                    ui.add_sized([145., 24.], egui::Label::new(rich(speed.label()).color(speed_color(speed))));
                    label(ui, speed.standard());
                });
            }
            ui.add_space(10.);
            label(ui, muted("SuperSpeedPlus est une catégorie : 10 Gb/s ou plus. Cette version ne différencie pas les modes 10 et 20 Gb/s, ni les liaisons USB4 / Thunderbolt."));
        });
        ui.add_space(16.);
        card().show(ui, |ui| {
            label(ui, rich("Tester un câble par comparaison").size(20.).strong());
            for text in [
                "1. Branchez un périphérique USB 3 connu directement sur un port USB 3 connu.",
                "2. Notez la vitesse négociée et le numéro du port.",
                "3. Remplacez uniquement le câble, puis attendez la nouvelle analyse.",
                "4. Si la liaison redescend à 480 Mb/s, vérifiez le câble et ses connecteurs.",
            ] { label(ui, text); }
            ui.add_space(8.);
            label(ui, muted("Un câble de charge sans données ne crée pas de périphérique détectable. La forme USB-C ne garantit aucune vitesse. La puissance USB Power Delivery et l’e-marker du câble ne sont pas lus par cette version."));
        });
        ui.add_space(16.);
        card().show(ui, |ui| {
            label(ui, rich("À propos de San USB Viewer").size(20.).strong());
            label(ui, "Application native en Rust, inspirée de WhatCable et de USB Device Tree Viewer.");
            ui.hyperlink_to(rich("Origine du projet · vidéo YouTube ↗"), SOURCE_URL);
            ui.hyperlink_to(rich("Données USB · bibliothèque nusb ↗"), "https://github.com/kevinmehall/nusb");
            label(ui, muted("Analyse locale en lecture seule, sans test de copie ni changement de pilote. Les exports JSON peuvent inclure les numéros de série et les identifiants système."));
            ui.add_space(8.);
            label(ui, muted("Traductions automatiques, à améliorer avec vos corrections.").size(12.));
            if ui.button(rich(if app.demo { "Revenir à mes périphériques" } else { "Explorer avec des données de démonstration" })).clicked() { app.switch_demo(); }
        });
    });
}
