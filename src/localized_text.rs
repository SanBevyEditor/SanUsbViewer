//! Styled localized text with Unicode bidi runs and width-aware RTL line breaks.
use crate::i18n;
use eframe::egui::{self, Color32, FontFamily, FontId, RichText, WidgetText};

pub(crate) struct LocalText {
    text: String,
    size: f32,
    color: Color32,
    strong: bool,
    monospace: bool,
}

pub(crate) fn rich(text: impl Into<String>) -> LocalText {
    raw(i18n::tr(&text.into()))
}

pub(crate) fn raw(text: impl Into<String>) -> LocalText {
    LocalText {
        text: text.into(),
        size: 14.,
        color: Color32::PLACEHOLDER,
        strong: false,
        monospace: false,
    }
}

impl LocalText {
    pub(crate) fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub(crate) fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }
    pub(crate) fn strong(mut self) -> Self {
        self.strong = true;
        self
    }
    pub(crate) fn monospace(mut self) -> Self {
        self.monospace = true;
        self
    }

    fn is_rtl(&self) -> bool {
        unicode_bidi::BidiInfo::new(&self.text, None).has_rtl()
    }

    fn font(&self) -> FontId {
        FontId::new(
            self.size,
            if self.monospace {
                FontFamily::Monospace
            } else {
                FontFamily::Proportional
            },
        )
    }

    fn wrapped(&self, ui: &egui::Ui) -> WidgetText {
        if !self.is_rtl() {
            return self.to_widget();
        }
        let max_width = ui.available_width().max(80.);
        let font = self.font();
        let mut lines = Vec::new();
        for paragraph in self.text.split('\n') {
            let mut line = String::new();
            for word in paragraph.split_whitespace() {
                let candidate = if line.is_empty() {
                    word.into()
                } else {
                    format!("{line} {word}")
                };
                let job = i18n::bidi_job(&candidate, font.clone(), self.color);
                let width = ui.fonts_mut(|fonts| fonts.layout_job(job).size().x);
                if width > max_width && !line.is_empty() {
                    lines.push(std::mem::take(&mut line));
                    line.push_str(word);
                } else {
                    line = candidate;
                }
            }
            lines.push(line);
        }
        i18n::bidi_job(&lines.join("\n"), font, self.color).into()
    }

    fn to_widget(&self) -> WidgetText {
        if self.is_rtl() {
            return i18n::bidi_job(&self.text, self.font(), self.color).into();
        }
        let mut text = RichText::new(&self.text).size(self.size).color(self.color);
        if self.strong {
            text = text.strong();
        }
        if self.monospace {
            text = text.monospace();
        }
        text.into()
    }
}

impl From<LocalText> for WidgetText {
    fn from(text: LocalText) -> Self {
        text.to_widget()
    }
}
impl From<&str> for LocalText {
    fn from(text: &str) -> Self {
        rich(text)
    }
}
impl From<String> for LocalText {
    fn from(text: String) -> Self {
        rich(text)
    }
}
impl From<&String> for LocalText {
    fn from(text: &String) -> Self {
        rich(text)
    }
}

pub(crate) fn label(ui: &mut egui::Ui, text: impl Into<LocalText>) -> egui::Response {
    let text = text.into();
    ui.label(text.wrapped(ui))
}

pub(crate) fn heading(ui: &mut egui::Ui, text: impl Into<LocalText>) -> egui::Response {
    label(ui, text.into().size(25.).strong())
}
