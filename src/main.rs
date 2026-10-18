#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod connection;
mod fonts;
mod i18n;
mod inspector;
mod language_picker;
mod localized_text;
mod model;
mod preferences;
mod scanner;
mod theme;
mod views;
#[cfg(windows)]
mod windows_probe;

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--scan") {
        match scanner::scan() {
            Ok(snapshot) => match serde_json::to_string_pretty(&snapshot) {
                Ok(json) => println!("{json}"),
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            },
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        return Ok(());
    }
    let demo = args.iter().any(|a| a == "--demo");
    let saved = preferences::load();
    let locale = args
        .windows(2)
        .find(|a| a[0] == "--lang")
        .map(|a| a[1].clone())
        .or(saved.language)
        .or_else(sys_locale::get_locale)
        .unwrap_or_else(|| "en".into());
    i18n::set_language(&locale);
    let initial_page = args
        .windows(2)
        .find(|a| a[0] == "--page")
        .map(|a| a[1].as_str());
    let initial_page = match initial_page {
        Some("guide") => app::Page::Guide,
        Some("activity") => app::Page::Activity,
        _ => app::Page::Explorer,
    };
    let show_languages = args.iter().any(|a| a == "--language-picker");
    let screenshot = args
        .windows(2)
        .find(|a| a[0] == "--screenshot")
        .map(|a| a[1].clone());
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("San USB Viewer")
            .with_inner_size([1380.0, 900.0])
            .with_min_inner_size([1000.0, 700.0])
            .with_icon(theme::app_icon()),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "San USB Viewer",
        options,
        Box::new(move |cc| {
            theme::configure(&cc.egui_ctx);
            let mut application = app::UsbApp::new(demo, screenshot);
            application.page = initial_page;
            application.language_picker = show_languages;
            Ok(Box::new(application))
        }),
    )
}
