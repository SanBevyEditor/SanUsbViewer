//! Embedded catalogs; the USB model and JSON schema remain language independent.
use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Language {
    pub(crate) code: &'static str,
    pub(crate) name: &'static str,
    pub(crate) english: &'static str,
    pub(crate) rtl: bool,
}

include!(concat!(env!("OUT_DIR"), "/locale_data.rs"));

thread_local! {
    // Rendering is on the UI thread. Worker scans never read this preference.
    static CURRENT: Cell<usize> = const { Cell::new(4) };
}

pub(crate) fn language() -> &'static Language {
    &LANGUAGES[CURRENT.get()]
}

pub(crate) fn set_language(code: &str) {
    CURRENT.set(resolve(code));
}

pub(crate) fn resolve(code: &str) -> usize {
    let normalized = code.to_lowercase().replace('_', "-");
    let primary = normalized.split('-').next().unwrap_or("en");
    let primary = match primary {
        "fil" => "tl",
        "in" => "id",
        _ => primary,
    };
    LANGUAGES
        .iter()
        .position(|l| l.code == primary)
        .unwrap_or(0)
}

/// The source language is French. Unknown strings (system/device names) pass through.
pub(crate) fn tr(source: &str) -> String {
    let catalog = CATALOGS[CURRENT.get()];
    catalog
        .binary_search_by(|(key, _)| key.cmp(&source))
        .map_or_else(|_| source.to_owned(), |index| catalog[index].1.to_owned())
}

pub(crate) fn count(source: &str, number: usize) -> String {
    tr(source).replace("{count}", &number.to_string())
}

/// Localize structured messages after scanning, so a language switch also updates history.
pub(crate) fn message(source: &str) -> String {
    for separator in [" · ", " : "] {
        if let Some((prefix, suffix)) = source.split_once(separator) {
            let translated = tr(prefix);
            if translated != prefix || language().code == "fr" {
                let suffix = if let Some((name, speed)) = suffix.rsplit_once(" · ") {
                    format!("{name} · {}", tr(speed))
                } else {
                    suffix.to_owned()
                };
                return format!("{translated}{separator}{suffix}");
            }
        }
    }
    tr(source)
}

pub(crate) fn device_name(device: &crate::model::UsbDevice, demo: bool) -> String {
    if demo {
        return tr(&device.name);
    }
    if let Some(ids) = device.name.strip_prefix("Périphérique USB ") {
        return format!("{} {ids}", tr("Périphérique USB"));
    }
    device.name.clone()
}

pub(crate) fn route(device: &crate::model::UsbDevice) -> String {
    if device.ports.is_empty() {
        return tr("Hub racine");
    }
    device
        .ports
        .iter()
        .map(|p| format!("{} {p}", tr("Port")))
        .collect::<Vec<_>>()
        .join(" / ")
}

/// egui/harfrust shapes each same-font run but does not reorder mixed bidi runs.
/// Add a separate text section for each Unicode directional run, in visual order.
/// The individual RTL runs stay logical so harfrust can shape Arabic correctly.
pub(crate) fn bidi_job(
    text: &str,
    font: eframe::egui::FontId,
    color: eframe::egui::Color32,
) -> eframe::egui::text::LayoutJob {
    use eframe::egui::{TextFormat, text::LayoutJob};
    let base = language().rtl.then(unicode_bidi::Level::rtl);
    let bidi = unicode_bidi::BidiInfo::new(text, base);
    let mut job = LayoutJob::default();
    for (i, para) in bidi.paragraphs.iter().enumerate() {
        if i > 0 {
            job.append("\n", 0., TextFormat::simple(font.clone(), color));
        }
        let (levels, runs) = bidi.visual_runs(para, para.range.clone());
        for run in runs {
            let mut format = TextFormat::simple(font.clone(), color);
            // Directional sections prevent one face from merging opposite runs.
            // Font families distinguish Arabic from neutral/Latin text.
            if levels[run.start].is_rtl() {
                format.font_id.family = eframe::egui::FontFamily::Name("rtl".into());
            }
            job.append(text[run].trim_end_matches('\n'), 0., format);
        }
    }
    if job.sections.is_empty() {
        job.append(text, 0., TextFormat::simple(font, color));
    }
    job
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn exactly_fifty_complete_catalogs_with_unique_language_codes() {
        assert_eq!(LANGUAGES.len(), 50);
        assert_eq!(CATALOGS.len(), 50);
        assert_eq!(
            LANGUAGES
                .iter()
                .map(|l| l.code)
                .collect::<BTreeSet<_>>()
                .len(),
            50
        );
        let keys: BTreeSet<_> = CATALOGS[0].iter().map(|(key, _)| key).collect();
        for catalog in CATALOGS {
            assert_eq!(
                catalog.iter().map(|(key, _)| key).collect::<BTreeSet<_>>(),
                keys
            );
            assert!(catalog.iter().all(|(_, value)| !value.trim().is_empty()));
            for (key, value) in *catalog {
                assert_eq!(
                    key.matches("{count}").count(),
                    value.matches("{count}").count(),
                    "Invalid parameter: {key}"
                );
            }
        }
    }

    #[test]
    fn regional_locales_and_aliases_resolve_to_the_supported_language() {
        assert_eq!(LANGUAGES[resolve("pt_BR")].code, "pt");
        assert_eq!(LANGUAGES[resolve("zh-Hant-TW")].code, "zh");
        assert_eq!(LANGUAGES[resolve("fil-PH")].code, "tl");
        assert_eq!(LANGUAGES[resolve("xx-ZZ")].code, "en");
    }

    #[test]
    fn language_change_relocalizes_history_without_translating_device_names() {
        set_language("en");
        assert_eq!(
            message("Connecté · Clavier ABC · 5 Gb/s"),
            "Connected · Clavier ABC · 5 Gbit/s"
        );
        set_language("fr");
        assert_eq!(
            message("Connecté · Clavier ABC · 5 Gb/s"),
            "Connecté · Clavier ABC · 5 Gb/s"
        );
    }

    #[test]
    fn mixed_arabic_and_usb_identifiers_keep_the_latin_run_intact() {
        let job = bidi_job(
            "سرعة USB 3.0",
            eframe::egui::FontId::proportional(14.),
            eframe::egui::Color32::WHITE,
        );
        assert!(
            job.sections
                .iter()
                .any(|s| job.text[s.byte_range.start.0..s.byte_range.end.0].contains("USB 3.0"))
        );
        assert!(
            job.sections
                .iter()
                .any(|s| s.format.font_id.family == eframe::egui::FontFamily::Name("rtl".into()))
        );
    }
}
