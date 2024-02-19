use eframe::egui;

use crate::Extra::App::Application;

pub fn Draw(_application: &mut Application, ctx: &egui::Context) {
    egui::SidePanel::left("左侧")
        .default_width(100.0)
        .show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("左侧");
            })
        });
}
