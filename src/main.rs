#![windows_subsystem = "windows"] // do not display a console window on startup on Windows
mod app;
mod character;
mod ui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("dsa5-analyzer"),
        ..Default::default()
    };

    eframe::run_native(
        "DSA5 character analyzer",
        options,
        Box::new(|_cc| Ok(Box::new(app::App::new()))),
    )
}
