use eframe::egui;

use crate::Extra::App::Application;

pub fn Draw(application: &mut Application, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.heading(application.author.as_str());
        })
    });
}
