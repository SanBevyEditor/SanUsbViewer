use crate::{
    app::UsbApp,
    i18n::{self, LANGUAGES},
    localized_text::{label, raw, rich},
};
use eframe::egui;

pub(crate) fn show(ctx: &egui::Context, app: &mut UsbApp) {
    if !app.language_picker {
        return;
    }
    let mut open = true;
    let mut chosen = None;
    egui::Window::new(rich("Choisir une langue").size(20.))
        .id(egui::Id::new("language_picker"))
        .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
        .open(&mut open)
        .default_width(420.)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut app.language_search)
                    .hint_text(rich("Rechercher une langue…"))
                    .desired_width(f32::INFINITY),
            );
            label(
                ui,
                rich("La langue est enregistrée sur cet ordinateur.").size(12.),
            );
            if ui.button(rich("Langue du système")).clicked() {
                let code = sys_locale::get_locale().unwrap_or_else(|| "en".into());
                chosen = Some(LANGUAGES[i18n::resolve(&code)].code);
            }
            ui.separator();
            egui::ScrollArea::vertical()
                .max_height(400.)
                .show(ui, |ui| {
                    let query = app.language_search.trim().to_lowercase();
                    for language in LANGUAGES {
                        if !format!("{} {} {}", language.name, language.english, language.code)
                            .to_lowercase()
                            .contains(&query)
                        {
                            continue;
                        }
                        ui.push_id(language.code, |ui| {
                            ui.horizontal(|ui| {
                                if ui
                                    .selectable_label(
                                        app.language_code == language.code,
                                        raw(language.name),
                                    )
                                    .clicked()
                                {
                                    chosen = Some(language.code);
                                }
                                label(
                                    ui,
                                    raw(format!(
                                        "{} · {}",
                                        language.english,
                                        language.code.to_uppercase()
                                    ))
                                    .size(11.)
                                    .color(crate::theme::MUTED),
                                );
                            });
                        });
                    }
                });
        });
    app.language_picker = open;
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.language_picker = false;
    }
    if let Some(code) = chosen {
        app.set_language(code);
        app.language_picker = false;
        app.language_search.clear();
    }
}
