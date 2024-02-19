#![allow(non_snake_case)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod Extra;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 260.0])
            .with_min_inner_size([520.0, 260.0])
            .with_decorations(true)
            .with_transparent(false),
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    let icon = Extra::Icon::LoadIcon();
    options.viewport.icon = Some(std::sync::Arc::<egui::IconData>::new(egui::IconData {
        rgba: icon.0,
        width: icon.1,
        height: icon.2,
    }));

    eframe::run_native(
        "工具应用",
        options,
        Box::new(|cc| Box::new(Extra::App::Application::New(cc))),
    )
}
