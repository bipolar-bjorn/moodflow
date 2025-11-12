mod app;
mod database;
mod models;
mod mood;
mod security;
mod settings;
mod ui;

fn main() -> eframe::Result<()> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([350.0, 600.0])
        .with_min_inner_size([350.0, 600.0])
        .with_max_inner_size([350.0, 600.0])
        .with_decorations(true)
        .with_transparent(true)
        .with_close_button(true)
        .with_minimize_button(true)
        .with_maximize_button(false)
        .with_title("MoodFlow");

    let options = eframe::NativeOptions {
        viewport: viewport,
        ..Default::default()
    };

    eframe::run_native(
        "MoodFlow",
        options,
        Box::new(|_cc| Ok(Box::new(app::MoodFlowApp::default()))),
    )
}
