use eframe::egui;

use crate::Extra::App::Application;

pub fn Draw(_application: &mut Application, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("底部")
        .default_height(20.0)
        .show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("底部");
            })
        });
}
