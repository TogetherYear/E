use super::{
    Font::LoadFont,
    Layout::{Bottom, Central, Left, Right, Top},
};
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
        Top::Draw(self, ctx);
        Bottom::Draw(self, ctx);
        Left::Draw(self, ctx);
        Right::Draw(self, ctx);
        Central::Draw(self, ctx);
    }
}
