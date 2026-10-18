pub(crate) use crate::localized_text::{heading, label, rich};
use eframe::egui::{self, Color32, FontId, Stroke, TextStyle, Vec2};

pub(crate) const BG: Color32 = Color32::from_rgb(13, 19, 27);
pub(crate) const PANEL: Color32 = Color32::from_rgb(18, 26, 36);
pub(crate) const CARD: Color32 = Color32::from_rgb(23, 33, 45);
pub(crate) const BORDER: Color32 = Color32::from_rgb(40, 55, 70);
pub(crate) const TEXT: Color32 = Color32::from_rgb(229, 238, 245);
pub(crate) const MUTED: Color32 = Color32::from_rgb(139, 158, 177);
pub(crate) const ACCENT: Color32 = Color32::from_rgb(75, 224, 193);
pub(crate) const BLUE: Color32 = Color32::from_rgb(113, 174, 255);
pub(crate) const AMBER: Color32 = Color32::from_rgb(244, 193, 112);
pub(crate) const SOURCE_URL: &str = "https://www.youtube.com/watch?v=xk6GxQEmkus";

pub(crate) fn configure(ctx: &egui::Context) {
    crate::fonts::install(ctx);
    ctx.set_theme(egui::Theme::Dark);
    let mut style = (*ctx.style_of(egui::Theme::Dark)).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.override_text_color = Some(TEXT);
    style.visuals.panel_fill = BG;
    style.visuals.window_fill = PANEL;
    style.visuals.extreme_bg_color = BG;
    style.visuals.faint_bg_color = CARD;
    style.visuals.hyperlink_color = ACCENT;
    style.visuals.selection.bg_fill = Color32::from_rgb(29, 79, 76);
    style.visuals.selection.stroke = Stroke::new(1.0, ACCENT);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    style.visuals.widgets.inactive.weak_bg_fill = CARD;
    style.visuals.widgets.inactive.bg_fill = CARD;
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(33, 53, 64);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(28, 76, 72);
    style.spacing.item_spacing = Vec2::new(12.0, 7.0);
    style.spacing.button_padding = Vec2::new(14.0, 9.0);
    style.spacing.interact_size.y = 34.0;
    style
        .text_styles
        .insert(TextStyle::Body, FontId::proportional(14.0));
    style
        .text_styles
        .insert(TextStyle::Button, FontId::proportional(14.0));
    style
        .text_styles
        .insert(TextStyle::Small, FontId::proportional(12.0));
    style
        .text_styles
        .insert(TextStyle::Heading, FontId::proportional(25.0));
    ctx.set_style_of(egui::Theme::Dark, style);
}

pub(crate) fn muted(text: impl Into<String>) -> crate::localized_text::LocalText {
    rich(text).color(MUTED)
}

pub(crate) fn card() -> egui::Frame {
    egui::Frame::new()
        .fill(CARD)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(14)
        .inner_margin(20)
}

pub(crate) fn badge(ui: &mut egui::Ui, text: &str, color: Color32) {
    let font = FontId::proportional(11.);
    let job = crate::i18n::bidi_job(&crate::i18n::tr(text), font, color);
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(galley.size().x + 18., 25.), egui::Sense::hover());
    ui.painter()
        .rect_filled(rect, 6, color.gamma_multiply(0.12));
    ui.painter()
        .galley(rect.center() - galley.size() * 0.5, galley, color);
}

pub(crate) fn speed_color(speed: crate::model::LinkSpeed) -> Color32 {
    use crate::model::LinkSpeed;
    match speed {
        LinkSpeed::Super | LinkSpeed::SuperPlus => ACCENT,
        LinkSpeed::High => BLUE,
        LinkSpeed::Unknown => MUTED,
        LinkSpeed::Low | LinkSpeed::Full => AMBER,
    }
}

/// Drawn vectors keep device symbols sharp on high DPI displays.
pub(crate) fn usb_icon(painter: &egui::Painter, rect: egui::Rect, color: Color32) {
    let c = rect.center();
    let s = rect.width().min(rect.height()) / 32.0;
    let point = |x, y| c + Vec2::new(x * s, y * s);
    let stroke = Stroke::new(1.8 * s, color);
    painter.line_segment([point(0., 10.), point(0., -11.)], stroke);
    painter.line_segment([point(0., 4.), point(-8., -2.)], stroke);
    painter.line_segment([point(-8., -2.), point(-8., -6.)], stroke);
    painter.line_segment([point(0., 0.), point(8., -6.)], stroke);
    painter.line_segment([point(8., -6.), point(8., -9.)], stroke);
    painter.circle_filled(point(0., 11.), 3. * s, color);
    painter.circle_filled(point(-8., -8.), 2.3 * s, color);
    painter.rect_filled(
        egui::Rect::from_center_size(point(8., -11.), Vec2::splat(4.5 * s)),
        0,
        color,
    );
    painter.add(egui::Shape::convex_polygon(
        vec![point(0., -15.), point(-3.5, -10.), point(3.5, -10.)],
        color,
        Stroke::NONE,
    ));
}

pub(crate) fn app_icon() -> egui::IconData {
    let size = 64;
    let mut rgba = vec![0; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let index = (y * size + x) * 4;
            let mark = (29..35).contains(&x) && (12..51).contains(&y)
                || (17..47).contains(&x) && (25..31).contains(&y)
                || (14..20).contains(&x) && (17..31).contains(&y)
                || (44..50).contains(&x) && (15..31).contains(&y);
            let c = if mark { ACCENT } else { PANEL };
            rgba[index..index + 4].copy_from_slice(&c.to_array());
        }
    }
    egui::IconData {
        rgba,
        width: size as u32,
        height: size as u32,
    }
}
