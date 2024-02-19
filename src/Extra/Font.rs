use std::fs;

use eframe::egui;

pub fn LoadFont() -> egui::FontDefinitions {
    let mut font = egui::FontDefinitions::default();

    font.font_data.insert(
        "Font".to_owned(),
        egui::FontData::from_owned(fs::read("C:\\Windows\\Fonts\\msyh.ttc").unwrap()),
    );
    font.families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "Font".to_owned());
    font.families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("Font".to_owned());
    font
}
