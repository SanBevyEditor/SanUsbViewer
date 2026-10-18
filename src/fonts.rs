//! OFL-licensed Noto fonts are embedded so every supported script works offline.
use eframe::egui::{self, FontFamily};

pub(crate) fn install(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    macro_rules! add {
        ($name:literal, $file:literal) => {
            fonts.font_data.insert(
                $name.into(),
                egui::FontData::from_static(include_bytes!(concat!("../assets/fonts/", $file)))
                    .into(),
            );
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .push($name.into());
            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .push($name.into());
        };
    }
    add!("Noto Sans", "NotoSans-Regular.ttf");
    add!("Arabic", "NotoSansArabic-Regular.ttf");
    add!("Devanagari", "NotoSansDevanagari-Regular.ttf");
    add!("Bengali", "NotoSansBengali-Regular.ttf");
    add!("Gurmukhi", "NotoSansGurmukhi-Regular.ttf");
    add!("Gujarati", "NotoSansGujarati-Regular.ttf");
    add!("Tamil", "NotoSansTamil-Regular.ttf");
    add!("Telugu", "NotoSansTelugu-Regular.ttf");
    add!("Kannada", "NotoSansKannada-Regular.ttf");
    add!("Malayalam", "NotoSansMalayalam-Regular.ttf");
    add!("Odia", "NotoSansOriya-Regular.ttf");
    add!("Thai", "NotoSansThai-Regular.ttf");
    add!("Myanmar", "NotoSansMyanmar-Regular.ttf");
    add!("Ethiopic", "NotoSansEthiopic-Regular.ttf");
    add!("CJK", "NotoSansCJKsc-Regular.otf");
    // Use bundled Latin first, before egui's symbol/Latin fallback fonts.
    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.retain(|name| name != "Noto Sans");
        family.insert(0, "Noto Sans".into());
    }
    let mut rtl = fonts.families[&FontFamily::Proportional].clone();
    rtl.retain(|name| name != "Arabic");
    rtl.insert(0, "Arabic".into());
    fonts.families.insert(FontFamily::Name("rtl".into()), rtl);
    ctx.set_fonts(fonts);
}
