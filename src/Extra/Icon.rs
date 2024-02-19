pub fn LoadIcon() -> (Vec<u8>, u32, u32) {
    let icon_data = include_bytes!("./Assets/icon.ico");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Ico).unwrap();
    let rgba_data = img.into_rgba8();
    let (w, h) = (rgba_data.width(), rgba_data.height());
    (rgba_data.into_raw(), w, h)
}
