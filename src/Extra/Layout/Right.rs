use eframe::egui;

use crate::Extra::App::Application;

pub fn Draw(_application: &mut Application, ctx: &egui::Context) {
    egui::SidePanel::right("右侧").show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.heading("右侧");
        })
    });
}
