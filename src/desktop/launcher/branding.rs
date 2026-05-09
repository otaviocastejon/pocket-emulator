//! Pocket Emulator mark: embedded app icon and header layout for the launcher.

use std::sync::OnceLock;

use eframe::egui;

use super::theme::{brand_emulator_word, brand_pocket_word};

static LOGO_TEXTURE: OnceLock<egui::TextureHandle> = OnceLock::new();

fn load_logo(ctx: &egui::Context) -> egui::TextureHandle {
    let bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/pocket_emulator_icon.png"
    ));
    let image = image::load_from_memory(bytes)
        .expect("embedded Pocket Emulator logo")
        .into_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let rgba = image.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba);
    ctx.load_texture(
        "pocket_emulator_logo",
        color_image,
        egui::TextureOptions::LINEAR,
    )
}

/// Cached texture handle for the square Pocket Emulator icon (shown in the launcher banner).
pub(crate) fn pocket_emulator_logo(ctx: &egui::Context) -> egui::TextureHandle {
    LOGO_TEXTURE.get_or_init(|| load_logo(ctx)).clone()
}

/// Home / My Games hero row: icon + stacked POCKET / EMULATOR wordmark (retro monospace).
pub(crate) fn pocket_emulator_banner(ui: &mut egui::Ui, logo_px: f32) {
    let tex = pocket_emulator_logo(ui.ctx());
    ui.horizontal(|ui| {
        ui.image(egui::ImageSource::Texture(egui::load::SizedTexture::new(
            tex.id(),
            egui::vec2(logo_px, logo_px),
        )));
        ui.add_space(12.0);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(
                egui::RichText::new("POCKET")
                    .size(22.0)
                    .strong()
                    .monospace()
                    .color(brand_pocket_word()),
            );
            ui.label(
                egui::RichText::new("EMULATOR")
                    .size(22.0)
                    .strong()
                    .monospace()
                    .color(brand_emulator_word()),
            );
        });
    });
}
