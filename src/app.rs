use crate::{
    i18n, language_picker,
    localized_text::{label, rich},
    preferences,
};
use eframe::egui;
use std::{
    collections::VecDeque,
    sync::mpsc,
    time::{Duration, Instant},
};

use crate::{model::Snapshot, scanner, theme, views};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Page {
    Explorer,
    Activity,
    Guide,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DeviceFilter {
    All,
    SuperSpeed,
    Hubs,
}

#[derive(Clone, Debug)]
pub(crate) struct Activity {
    pub(crate) time: String,
    pub(crate) message: String,
    pub(crate) connected: bool,
}

pub(crate) struct UsbApp {
    pub(crate) language_code: String,
    pub(crate) language_picker: bool,
    pub(crate) language_search: String,
    pub(crate) snapshot: Snapshot,
    pub(crate) selected: Option<String>,
    pub(crate) query: String,
    pub(crate) filter: DeviceFilter,
    pub(crate) page: Page,
    pub(crate) auto_refresh: bool,
    pub(crate) demo: bool,
    pub(crate) scanning: bool,
    pub(crate) error: Option<String>,
    pub(crate) notice: Option<String>,
    pub(crate) activity: VecDeque<Activity>,
    pub(crate) detail_tab: usize,
    receiver: Option<mpsc::Receiver<Result<Snapshot, String>>>,
    last_scan: Instant,
    has_snapshot: bool,
    screenshot: Option<String>,
    screenshot_requested: bool,
    started: Instant,
    export_receiver: Option<mpsc::Receiver<Result<String, String>>>,
}

impl UsbApp {
    pub(crate) fn new(demo: bool, screenshot: Option<String>) -> Self {
        let snapshot = if demo {
            scanner::demo()
        } else {
            Snapshot::default()
        };
        let selected = snapshot
            .devices
            .iter()
            .find(|d| !d.is_hub())
            .map(|d| d.id.clone());
        Self {
            language_code: i18n::language().code.into(),
            language_picker: false,
            language_search: String::new(),
            snapshot,
            selected,
            query: String::new(),
            filter: DeviceFilter::All,
            page: Page::Explorer,
            auto_refresh: true,
            demo,
            scanning: false,
            error: None,
            notice: None,
            activity: VecDeque::new(),
            detail_tab: 0,
            receiver: None,
            last_scan: Instant::now() - Duration::from_secs(10),
            has_snapshot: demo,
            screenshot,
            screenshot_requested: false,
            started: Instant::now(),
            export_receiver: None,
        }
    }

    pub(crate) fn start_scan(&mut self) {
        if self.scanning || self.demo {
            return;
        }
        self.scanning = true;
        self.last_scan = Instant::now();
        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        std::thread::spawn(move || {
            let _ = tx.send(scanner::scan());
        });
    }

    pub(crate) fn set_language(&mut self, code: &str) {
        i18n::set_language(code);
        self.language_code = i18n::language().code.into();
        if let Err(error) = preferences::save(&self.language_code) {
            self.error = Some(format!(
                "{} : {error}",
                i18n::tr("Impossible d’enregistrer la langue")
            ));
        }
    }

    pub(crate) fn switch_demo(&mut self) {
        self.demo = !self.demo;
        // Drop the old receiver so an in-flight real scan cannot replace demo data.
        self.receiver = None;
        self.scanning = false;
        self.has_snapshot = self.demo;
        self.snapshot = if self.demo {
            scanner::demo()
        } else {
            Snapshot::default()
        };
        self.selected = self
            .snapshot
            .devices
            .iter()
            .find(|d| !d.is_hub())
            .map(|d| d.id.clone());
        self.error = None;
        self.notice = None;
        self.activity.clear();
        self.last_scan = Instant::now() - Duration::from_secs(10);
        if !self.demo {
            self.start_scan();
        }
    }

    fn accept_snapshot(&mut self, next: Snapshot) {
        if self.has_snapshot {
            let time = elapsed_clock(self.started.elapsed());
            for d in &next.devices {
                let previous = self.snapshot.device(&d.id);
                if previous.is_none() {
                    self.activity.push_front(Activity {
                        time: time.clone(),
                        message: format!("Connecté · {} · {}", d.name, d.speed.label()),
                        connected: true,
                    });
                } else if previous.is_some_and(|old| old.speed != d.speed) {
                    self.activity.push_front(Activity {
                        time: time.clone(),
                        message: format!("Liaison modifiée · {} · {}", d.name, d.speed.label()),
                        connected: true,
                    });
                }
            }
            for d in &self.snapshot.devices {
                if next.device(&d.id).is_none() {
                    self.activity.push_front(Activity {
                        time: time.clone(),
                        message: format!("Déconnecté · {}", d.name),
                        connected: false,
                    });
                }
            }
            self.activity.truncate(200);
        }
        if self
            .selected
            .as_deref()
            .is_none_or(|id| next.device(id).is_none())
        {
            self.selected = next
                .devices
                .iter()
                .filter(|d| !d.is_hub())
                .max_by(|a, b| {
                    a.speed
                        .mbps()
                        .unwrap_or(0.)
                        .total_cmp(&b.speed.mbps().unwrap_or(0.))
                })
                .or_else(|| next.devices.first())
                .map(|d| d.id.clone());
        }
        self.snapshot = next;
        self.has_snapshot = true;
        self.error = None;
    }

    pub(crate) fn export(&mut self) {
        if self.export_receiver.is_some() {
            return;
        }
        let snapshot = self.snapshot.clone();
        let report_label = i18n::tr("Rapport JSON");
        let (tx, rx) = mpsc::channel();
        self.export_receiver = Some(rx);
        std::thread::spawn(move || {
            let path = rfd::FileDialog::new()
                .add_filter(&report_label, &["json"])
                .set_file_name(if snapshot.demo {
                    "san-usb-demo.json"
                } else {
                    "san-usb-rapport.json"
                })
                .save_file();
            let result = path.map_or_else(
                || Ok("Export annulé".to_owned()),
                |path| {
                    serde_json::to_string_pretty(&snapshot)
                        .map_err(|e| e.to_string())
                        .and_then(|json| std::fs::write(&path, json).map_err(|e| e.to_string()))
                        .map(|()| format!("Rapport enregistré : {}", path.display()))
                },
            );
            let _ = tx.send(result);
        });
    }

    fn tick(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.receiver {
            match rx.try_recv() {
                Ok(result) => {
                    self.receiver = None;
                    self.scanning = false;
                    match result {
                        Ok(snapshot) => self.accept_snapshot(snapshot),
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.receiver = None;
                    self.scanning = false;
                    self.error = Some(
                        "Le service de détection USB s’est arrêté. Vous pouvez relancer l’analyse."
                            .into(),
                    );
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if let Some(rx) = &self.export_receiver
            && let Ok(result) = rx.try_recv()
        {
            self.export_receiver = None;
            match result {
                Ok(message) => self.notice = Some(message),
                Err(e) => self.error = Some(format!("Export impossible : {e}")),
            }
        }
        if !self.demo
            && !self.scanning
            && (!self.has_snapshot
                || self.auto_refresh && self.last_scan.elapsed() > Duration::from_secs(3))
        {
            // Failed initial scans are retried only after the polling interval.
            if self.last_scan.elapsed() > Duration::from_secs(3) {
                self.start_scan();
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F5)) {
            self.start_scan();
        }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::E)) {
            self.export();
        }
        ctx.request_repaint_after(Duration::from_millis(200));
    }

    fn capture(&mut self, ctx: &egui::Context) {
        if self.screenshot.is_none() {
            return;
        }
        if !self.screenshot_requested
            && self.has_snapshot
            && self.started.elapsed() > Duration::from_secs(2)
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
            self.screenshot_requested = true;
        }
        let captured = ctx.input(|i| {
            i.events.iter().find_map(|event| {
                if let egui::Event::Screenshot { image, .. } = event {
                    Some(image.clone())
                } else {
                    None
                }
            })
        });
        if let (Some(image), Some(path)) = (captured, self.screenshot.as_ref()) {
            let rgba: Vec<u8> = image.pixels.iter().flat_map(|p| p.to_array()).collect();
            match image::save_buffer(
                path,
                &rgba,
                image.width() as u32,
                image.height() as u32,
                image::ColorType::Rgba8,
            ) {
                Ok(()) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                Err(e) => {
                    self.error = Some(format!("Capture impossible : {e}"));
                    self.screenshot = None;
                }
            }
        }
    }
}

impl eframe::App for UsbApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        i18n::set_language(&self.language_code);
        let ctx = ui.ctx().clone();
        self.tick(&ctx);
        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(theme::PANEL)
                    .inner_margin(egui::Margin::symmetric(26, 15)),
            )
            .show(ui, |ui| views::header(ui, self));
        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(theme::PANEL)
                    .inner_margin(egui::Margin::symmetric(26, 8)),
            )
            .show(ui, |ui| views::status(ui, self));
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme::BG).inner_margin(26))
            .show(ui, |ui| {
                if let Some(error) = self.error.clone() {
                    ui.horizontal_wrapped(|ui| {
                        label(ui, rich(i18n::message(&error)).color(theme::AMBER));
                        if ui.small_button(rich("Fermer")).clicked() {
                            self.error = None;
                        }
                    });
                    ui.add_space(8.);
                }
                if let Some(notice) = self.notice.clone() {
                    ui.horizontal_wrapped(|ui| {
                        label(ui, rich(i18n::message(&notice)).color(theme::ACCENT));
                        if ui.small_button(rich("Fermer")).clicked() {
                            self.notice = None;
                        }
                    });
                    ui.add_space(8.);
                }
                match self.page {
                    Page::Explorer => views::explorer(ui, self),
                    Page::Activity => views::activity(ui, self),
                    Page::Guide => views::guide(ui, self),
                }
            });
        language_picker::show(&ctx, self);
        self.capture(&ctx);
    }
}

fn elapsed_clock(duration: Duration) -> String {
    let seconds = duration.as_secs();
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
    )
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
