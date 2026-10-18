use super::*;
use crate::model::LinkSpeed;
use eframe::App;

#[test]
fn unplug_selected_device_reconciles_selection_and_records_event() {
    let mut app = UsbApp::new(true, None);
    let removed_id = app.selected.clone();
    let mut next = app.snapshot.clone();
    next.devices.retain(|d| Some(&d.id) != removed_id.as_ref());
    app.accept_snapshot(next);
    assert!(
        app.selected != removed_id && app.activity.front().is_some_and(|event| !event.connected)
    );
}

#[test]
fn initial_scan_does_not_report_every_device_as_newly_connected() {
    let mut app = UsbApp::new(false, None);
    app.accept_snapshot(scanner::demo());
    assert!(app.activity.is_empty());
}

#[test]
fn speed_renegotiation_is_recorded_without_losing_selection() {
    let mut app = UsbApp::new(true, None);
    let selected = app.selected.clone();
    let mut next = app.snapshot.clone();
    for device in &mut next.devices {
        if Some(&device.id) == selected.as_ref() {
            device.speed = LinkSpeed::High;
        }
    }
    app.accept_snapshot(next);
    assert!(
        app.selected == selected
            && app
                .activity
                .front()
                .is_some_and(|event| event.message.contains("Liaison modifiée"))
    );
}

#[test]
fn demo_switch_discards_an_in_flight_hardware_response() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = UsbApp::new(false, None);
    let (tx, rx) = mpsc::channel();
    app.receiver = Some(rx);
    app.scanning = true;
    tx.send(Ok(Snapshot::default()))?;
    app.switch_demo();
    app.tick(&egui::Context::default());
    assert!(app.snapshot.demo && app.snapshot.devices.len() == 5);
    Ok(())
}

#[test]
fn ui_clicks_filter_select_device_and_navigate() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = egui::Context::default();
    theme::configure(&ctx);
    let mut app = UsbApp::new(true, None);
    let output = render(&ctx, &mut app, vec![]);
    click_label(&ctx, &mut app, &output, "USB 3+")?;
    let filtered = render(&ctx, &mut app, vec![]);
    assert_eq!(app.filter, DeviceFilter::SuperSpeed);
    assert!(text_position(&filtered, "Clavier mécanique").is_none());
    click_label(&ctx, &mut app, &filtered, "Tous")?;
    let all = render(&ctx, &mut app, vec![]);
    click_label(&ctx, &mut app, &all, "Clavier mécanique")?;
    assert!(
        app.selected
            .as_deref()
            .and_then(|id| app.snapshot.device(id))
            .is_some_and(|d| d.class == 3)
    );
    let selected = render(&ctx, &mut app, vec![]);
    click_label(&ctx, &mut app, &selected, "Activité")?;
    assert_eq!(app.page, Page::Activity);
    let activity = render(&ctx, &mut app, vec![]);
    click_label(&ctx, &mut app, &activity, "Guide & à propos")?;
    assert_eq!(app.page, Page::Guide);
    Ok(())
}

#[test]
fn ui_renders_details_and_guide_at_minimum_window_size() {
    let ctx = egui::Context::default();
    theme::configure(&ctx);
    let mut app = UsbApp::new(true, None);
    app.detail_tab = 1;
    let mut frame = eframe::Frame::_new_kittest();
    for page in [Page::Explorer, Page::Guide, Page::Activity] {
        app.page = page;
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000., 700.),
                )),
                ..Default::default()
            },
            |ui| app.ui(ui, &mut frame),
        );
        output.textures_delta.clear();
        assert!(!output.shapes.is_empty());
    }
}

#[test]
fn all_fifty_locales_render_every_page_and_the_language_picker() {
    let ctx = egui::Context::default();
    theme::configure(&ctx);
    let mut app = UsbApp::new(true, None);
    for language in i18n::LANGUAGES {
        app.language_code = language.code.into();
        for page in [Page::Explorer, Page::Guide, Page::Activity] {
            app.page = page;
            let output = render(&ctx, &mut app, vec![]);
            assert!(!output.shapes.is_empty(), "Empty UI: {}", language.code);
        }
    }
    app.language_picker = true;
    render(&ctx, &mut app, vec![]);
    i18n::set_language("fr");
}

#[test]
fn english_ui_has_translated_navigation_and_demo_device_labels() {
    let ctx = egui::Context::default();
    theme::configure(&ctx);
    let mut app = UsbApp::new(true, None);
    app.language_code = "en".into();
    let output = render(&ctx, &mut app, vec![]);
    assert!(text_position(&output, "Your USB connections").is_some());
    assert!(text_position(&output, "Portable SSD").is_some());
    assert!(text_position(&output, "Vos connexions USB").is_none());
    i18n::set_language("fr");
}

#[test]
fn language_picker_click_updates_ui_and_persists_choice() -> Result<(), Box<dyn std::error::Error>>
{
    // A child test process injects the portable configuration path without
    // mutating the environment of concurrently running tests (or user settings).
    if std::env::var("SAN_USB_LANGUAGE_TEST_CHILD").as_deref() != Ok("1") {
        let path = std::env::temp_dir().join(format!("san-usb-picker-{}.json", std::process::id()));
        let output = std::process::Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "app::tests::language_picker_click_updates_ui_and_persists_choice",
                "--nocapture",
            ])
            .env("SAN_USB_LANGUAGE_TEST_CHILD", "1")
            .env("SAN_USB_VIEWER_CONFIG", &path)
            .output()?;
        assert!(
            output.status.success(),
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let saved: preferences::Preferences =
            serde_json::from_str(&std::fs::read_to_string(&path)?)?;
        std::fs::remove_file(path)?;
        assert_eq!(saved.language.as_deref(), Some("en"));
        return Ok(());
    }
    let ctx = egui::Context::default();
    theme::configure(&ctx);
    let mut app = UsbApp::new(true, None);
    let output = render(&ctx, &mut app, vec![]);
    click_label(&ctx, &mut app, &output, "FR")?;
    assert!(app.language_picker);
    render(&ctx, &mut app, vec![]);
    let picker = render(&ctx, &mut app, vec![]);
    click_label(&ctx, &mut app, &picker, "English")?;
    assert_eq!(app.language_code, "en");
    assert!(!app.language_picker);
    let output = render(&ctx, &mut app, vec![]);
    assert!(text_position(&output, "Your USB connections").is_some());
    assert_eq!(preferences::load().language.as_deref(), Some("en"));
    Ok(())
}

fn render(ctx: &egui::Context, app: &mut UsbApp, events: Vec<egui::Event>) -> egui::FullOutput {
    let mut frame = eframe::Frame::_new_kittest();
    let mut output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1380., 900.),
            )),
            events,
            ..Default::default()
        },
        |ui| app.ui(ui, &mut frame),
    );
    output.textures_delta.clear();
    output
}

fn text_position(output: &egui::FullOutput, text: &str) -> Option<egui::Pos2> {
    output.shapes.iter().find_map(|shape| {
        if let egui::Shape::Text(t) = &shape.shape
            && t.galley.text() == text
        {
            Some(t.pos + t.galley.size() * 0.5)
        } else {
            None
        }
    })
}

fn click_label(
    ctx: &egui::Context,
    app: &mut UsbApp,
    output: &egui::FullOutput,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let pos =
        text_position(output, text).ok_or_else(|| format!("Visible control not found: {text}"))?;
    let button = |pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    render(ctx, app, vec![egui::Event::PointerMoved(pos), button(true)]);
    render(ctx, app, vec![button(false)]);
    Ok(())
}
