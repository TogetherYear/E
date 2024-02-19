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
            author: "测试：TSingleton".to_owned(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.author.as_str());
        });
    }
}
