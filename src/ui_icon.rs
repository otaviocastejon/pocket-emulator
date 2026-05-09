const ICON_PNG_BYTES: &[u8] = include_bytes!("../assets/icon.png");

pub fn load_icon_rgba() -> Option<(Vec<u8>, u32, u32)> {
    let img = image::load_from_memory(ICON_PNG_BYTES).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    Some((img.into_raw(), w, h))
}

pub fn icon_png_bytes() -> &'static [u8] {
    ICON_PNG_BYTES
}
