use eframe::egui;

use crate::Extra::App::Application;

pub fn Draw(_application: &mut Application, ctx: &egui::Context) {
    egui::TopBottomPanel::top("顶部").show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.heading("顶部");
        })
    });
}
