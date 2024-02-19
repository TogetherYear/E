use super::Font::LoadFont;
use eframe::egui;

pub struct Application {
    pub author: String,
}

impl Application {
    pub fn New(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_fonts(LoadFont());
        Self::default()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            author: "测试:TSingleton".to_owned(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("顶部")
            .default_height(20.0)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.heading("顶部");
                })
            });
        egui::TopBottomPanel::bottom("底部")
            .default_height(20.0)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.heading("底部");
                })
            });
        egui::SidePanel::left("左侧")
            .default_width(100.0)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.heading("左侧");
                })
            });
        egui::SidePanel::right("右侧").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("右侧");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading(self.author.as_str());
            })
        });
    }
}
