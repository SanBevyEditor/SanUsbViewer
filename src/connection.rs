use crate::{
    i18n::{self, tr},
    model::{Snapshot, UsbDevice},
    theme::{self, *},
};
use eframe::egui::{self, Color32, FontId, Stroke, Vec2};

pub(crate) fn draw(ui: &mut egui::Ui, snapshot: &Snapshot, device: &UsbDevice) {
    let ancestors = snapshot.ancestors(device);
    let mut nodes: Vec<(String, String)> = vec![("Ordinateur".into(), "Contrôleur USB".into())];
    for hub in ancestors {
        nodes.push((
            if hub.ports.is_empty() {
                "Hub racine".into()
            } else {
                "Hub USB".into()
            },
            hub.speed.label().into(),
        ));
    }
    // Preserve readable spacing even with a deeply nested topology.
    if nodes.len() > 4 {
        let hidden = nodes.len() - 3;
        nodes.drain(1..=hidden);
        nodes.insert(
            1,
            (i18n::count("{count} hubs", hidden), "intermédiaires".into()),
        );
    }
    nodes.push(("Câble / liaison".into(), "Capacité inconnue".into()));
    nodes.push(("Périphérique".into(), device.speed.standard().into()));
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 130.), egui::Sense::hover());
    let painter = ui.painter();
    let count = nodes.len();
    let step = rect.width() / count as f32;
    let y = rect.top() + 43.;
    let color = speed_color(device.speed);
    for (i, (name, subtitle)) in nodes.iter().enumerate() {
        let visual_index = if i18n::language().rtl {
            count - 1 - i
        } else {
            i
        };
        let center = egui::pos2(rect.left() + step * (visual_index as f32 + 0.5), y);
        if i + 1 < count {
            let direction = if i18n::language().rtl { -1. } else { 1. };
            let next = center + Vec2::new(step * direction, 0.);
            painter.line_segment(
                [
                    center + Vec2::new(23. * direction, 0.),
                    next - Vec2::new(23. * direction, 0.),
                ],
                Stroke::new(2., BORDER),
            );
            let mid = center + Vec2::new(step * direction * 0.5, 0.);
            painter.circle_filled(mid, 3., color);
        }
        let circle_color = if i == count - 1 { color } else { MUTED };
        painter.circle_filled(center, 23., Color32::from_rgb(17, 26, 37));
        painter.circle_stroke(
            center,
            23.,
            Stroke::new(1., if i == count - 1 { color } else { BORDER }),
        );
        if i == 0 {
            let screen =
                egui::Rect::from_center_size(center - Vec2::new(0., 2.), Vec2::new(24., 16.));
            painter.rect_stroke(
                screen,
                3,
                Stroke::new(1.5, circle_color),
                egui::StrokeKind::Inside,
            );
            painter.line_segment(
                [center + Vec2::new(0., 6.), center + Vec2::new(0., 11.)],
                Stroke::new(1.5, circle_color),
            );
            painter.line_segment(
                [center + Vec2::new(-7., 11.), center + Vec2::new(7., 11.)],
                Stroke::new(1.5, circle_color),
            );
        } else {
            theme::usb_icon(
                painter,
                egui::Rect::from_center_size(center, Vec2::splat(24.)),
                circle_color,
            );
        }
        for (text, offset, size, color) in [(name, 39., 12., TEXT), (subtitle, 64., 10., MUTED)] {
            let mut job = i18n::bidi_job(&tr(text), FontId::proportional(size), color);
            job.wrap.max_width = (step - 8.).max(40.);
            job.halign = egui::Align::Center;
            let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
            painter.galley(
                center + Vec2::new(-galley.size().x * 0.5, offset - galley.size().y * 0.5),
                galley,
                color,
            );
        }
    }
}
