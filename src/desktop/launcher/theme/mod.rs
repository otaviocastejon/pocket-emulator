//! Launcher design tokens: loaded from embedded TOML, optionally overridden by
//! `launcher_theme.toml` in the app config directory (`ProjectDirs::config_dir()`).

mod config;

use std::sync::OnceLock;

use eframe::egui;

pub use config::{parse_theme_toml, ThemeFile, EMBEDDED_DEFAULT_THEME};

use config::parse_hex_rgb;

static THEME: OnceLock<ResolvedTheme> = OnceLock::new();

/// Compile-time defaults — must stay aligned with `EMBEDDED_DEFAULT_THEME` / `default_launcher_theme.toml`.
/// Layout that should follow `launcher_theme.toml` must call `space_1()`, `radius_sm()`, etc., not these `const`s.
pub const SPACE_1: f32 = 4.0;
pub const SPACE_2: f32 = 8.0;
pub const SPACE_3: f32 = 12.0;
pub const SPACE_4: f32 = 16.0;
pub const RADIUS_SM: f32 = 8.0;
pub const RADIUS_LG: f32 = 12.0;
pub const RADIUS_XL: f32 = 16.0;

/// All palette and spacing used by the launcher after merge.
#[derive(Clone)]
pub(crate) struct ResolvedTheme {
    pub bg: egui::Color32,
    pub fg: egui::Color32,
    pub surface: egui::Color32,
    pub surface_2: egui::Color32,
    pub surface_3: egui::Color32,
    pub border: egui::Color32,
    pub primary: egui::Color32,
    pub primary_hover: egui::Color32,
    pub accent: egui::Color32,
    pub muted_fg: egui::Color32,
    pub dmg_blue: egui::Color32,
    pub lcd_deep: egui::Color32,
    pub success: egui::Color32,
    pub warning: egui::Color32,
    pub destructive: egui::Color32,
    pub info: egui::Color32,
    pub brand_pocket_word: egui::Color32,
    pub brand_emulator_word: egui::Color32,
    pub success_bg: egui::Color32,
    pub warning_bg: egui::Color32,
    pub destructive_bg: egui::Color32,
    pub info_bg: egui::Color32,
    pub platform_bg: egui::Color32,
    pub platform_text: egui::Color32,
    pub space_1: f32,
    pub space_2: f32,
    pub space_3: f32,
    pub space_4: f32,
    pub radius_sm: f32,
    pub radius_lg: f32,
    pub radius_xl: f32,
    pub shadow_blur: f32,
    pub shadow_offset_y: f32,
    pub shadow_color: egui::Color32,
}

impl ResolvedTheme {
    /// If TOML is missing or invalid keys, keep previous values.
    fn hardcoded_fallback() -> Self {
        Self {
            bg: egui::Color32::from_rgb(18, 18, 20),
            fg: egui::Color32::from_rgb(243, 242, 247),
            surface: egui::Color32::from_rgb(35, 34, 48),
            surface_2: egui::Color32::from_rgb(44, 42, 57),
            surface_3: egui::Color32::from_rgb(56, 54, 70),
            border: egui::Color32::from_rgb(84, 79, 96),
            primary: egui::Color32::from_rgb(255, 64, 129),
            primary_hover: egui::Color32::from_rgb(255, 120, 168),
            accent: egui::Color32::from_rgb(123, 210, 131),
            muted_fg: egui::Color32::from_rgb(162, 155, 178),
            dmg_blue: egui::Color32::from_rgb(45, 126, 214),
            lcd_deep: egui::Color32::from_rgb(74, 170, 84),
            success: egui::Color32::from_rgb(92, 210, 120),
            warning: egui::Color32::from_rgb(240, 184, 52),
            destructive: egui::Color32::from_rgb(231, 96, 85),
            info: egui::Color32::from_rgb(64, 186, 236),
            brand_pocket_word: egui::Color32::from_rgb(255, 228, 236),
            brand_emulator_word: egui::Color32::from_rgb(230, 90, 150),
            success_bg: egui::Color32::from_rgb(42, 74, 54),
            warning_bg: egui::Color32::from_rgb(74, 58, 30),
            destructive_bg: egui::Color32::from_rgb(78, 36, 38),
            info_bg: egui::Color32::from_rgb(26, 56, 78),
            platform_bg: egui::Color32::from_rgb(38, 50, 78),
            platform_text: egui::Color32::from_rgb(146, 174, 255),
            space_1: SPACE_1,
            space_2: SPACE_2,
            space_3: SPACE_3,
            space_4: SPACE_4,
            radius_sm: RADIUS_SM,
            radius_lg: RADIUS_LG,
            radius_xl: RADIUS_XL,
            shadow_blur: 16.0,
            shadow_offset_y: 4.0,
            shadow_color: egui::Color32::from_rgba_unmultiplied(10, 10, 20, 90),
        }
    }

    fn from_merged_layers(mut base: Self, file: ThemeFile) -> Self {
        let c = &file.colors;
        apply_opt_color(&mut base.bg, c.bg.as_deref());
        apply_opt_color(&mut base.fg, c.fg.as_deref());
        apply_opt_color(&mut base.surface, c.surface.as_deref());
        apply_opt_color(&mut base.surface_2, c.surface_2.as_deref());
        apply_opt_color(&mut base.surface_3, c.surface_3.as_deref());
        apply_opt_color(&mut base.border, c.border.as_deref());
        apply_opt_color(&mut base.primary, c.primary.as_deref());
        apply_opt_color(&mut base.primary_hover, c.primary_hover.as_deref());
        apply_opt_color(&mut base.accent, c.accent.as_deref());
        apply_opt_color(&mut base.muted_fg, c.muted_fg.as_deref());
        apply_opt_color(&mut base.dmg_blue, c.dmg_blue.as_deref());
        apply_opt_color(&mut base.lcd_deep, c.lcd_deep.as_deref());
        apply_opt_color(&mut base.success, c.success.as_deref());
        apply_opt_color(&mut base.warning, c.warning.as_deref());
        apply_opt_color(&mut base.destructive, c.destructive.as_deref());
        apply_opt_color(&mut base.info, c.info.as_deref());
        apply_opt_color(&mut base.brand_pocket_word, c.brand_pocket_word.as_deref());
        apply_opt_color(
            &mut base.brand_emulator_word,
            c.brand_emulator_word.as_deref(),
        );
        apply_opt_color(&mut base.success_bg, c.success_bg.as_deref());
        apply_opt_color(&mut base.warning_bg, c.warning_bg.as_deref());
        apply_opt_color(&mut base.destructive_bg, c.destructive_bg.as_deref());
        apply_opt_color(&mut base.info_bg, c.info_bg.as_deref());
        apply_opt_color(&mut base.platform_bg, c.platform_bg.as_deref());
        apply_opt_color(&mut base.platform_text, c.platform_text.as_deref());

        let s = &file.spacing;
        if let Some(v) = s.space_1 {
            base.space_1 = v as f32;
        }
        if let Some(v) = s.space_2 {
            base.space_2 = v as f32;
        }
        if let Some(v) = s.space_3 {
            base.space_3 = v as f32;
        }
        if let Some(v) = s.space_4 {
            base.space_4 = v as f32;
        }

        let r = &file.radius;
        if let Some(v) = r.sm {
            base.radius_sm = v as f32;
        }
        if let Some(v) = r.lg {
            base.radius_lg = v as f32;
        }
        if let Some(v) = r.xl {
            base.radius_xl = v as f32;
        }

        let sh = &file.shadow;
        if let Some(v) = sh.blur {
            base.shadow_blur = v as f32;
        }
        if let Some(v) = sh.offset_y {
            base.shadow_offset_y = v as f32;
        }
        if let (Some(rgb), Some(a)) = (sh.color.as_deref().and_then(parse_hex_rgb), sh.alpha) {
            base.shadow_color = egui::Color32::from_rgba_unmultiplied(rgb.0, rgb.1, rgb.2, a);
        } else if let Some(rgb) = sh.color.as_deref().and_then(parse_hex_rgb) {
            base.shadow_color =
                egui::Color32::from_rgba_unmultiplied(rgb.0, rgb.1, rgb.2, base.shadow_color.a());
        }

        base
    }
}

fn apply_opt_color(slot: &mut egui::Color32, s: Option<&str>) {
    let Some(s) = s else { return };
    if let Some((r, g, b)) = parse_hex_rgb(s) {
        *slot = egui::Color32::from_rgb(r, g, b);
    } else {
        log::warn!("launcher theme: ignored invalid color {s:?}");
    }
}

fn resolved() -> &'static ResolvedTheme {
    THEME.get().expect("init_launcher_theme() before first UI")
}

/// Call once before opening the launcher window (e.g. from `select_rom_with_ui`).
pub fn init_launcher_theme() {
    if THEME.get().is_some() {
        return;
    }
    let embedded = parse_theme_toml(EMBEDDED_DEFAULT_THEME).expect("embedded default theme valid");
    let mut base = ResolvedTheme::from_merged_layers(ResolvedTheme::hardcoded_fallback(), embedded);
    if let Some(path) = user_theme_path() {
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(contents) => match parse_theme_toml(&contents) {
                    Ok(user) => {
                        log::info!("launcher theme: merged user file {}", path.display());
                        base = ResolvedTheme::from_merged_layers(base, user);
                    }
                    Err(e) => log::warn!("launcher theme: invalid {} — {}", path.display(), e),
                },
                Err(e) => log::warn!("launcher theme: could not read {} — {}", path.display(), e),
            }
        }
    }
    let _ = THEME.set(base);
}

fn user_theme_path() -> Option<std::path::PathBuf> {
    pocketemulator::storage::launcher_theme_config_path()
}

#[inline]
pub(crate) fn space_1() -> f32 {
    resolved().space_1
}
#[inline]
pub(crate) fn space_2() -> f32 {
    resolved().space_2
}
#[inline]
pub(crate) fn space_3() -> f32 {
    resolved().space_3
}
#[inline]
pub(crate) fn space_4() -> f32 {
    resolved().space_4
}

#[inline]
pub(crate) fn radius_sm() -> f32 {
    resolved().radius_sm
}
#[inline]
pub(crate) fn radius_lg() -> f32 {
    resolved().radius_lg
}
/// Secondary brand accent (green in default theme) — eyebrows, monospace labels per launcher spec.
pub(crate) fn accent_text() -> egui::Color32 {
    resolved().accent
}

pub(crate) fn fg_text() -> egui::Color32 {
    resolved().fg
}

pub(crate) fn surface_fill() -> egui::Color32 {
    resolved().surface
}

pub(crate) fn surface_2_fill() -> egui::Color32 {
    resolved().surface_2
}

pub(crate) fn surface_3_fill() -> egui::Color32 {
    resolved().surface_3
}

pub(crate) fn border_color() -> egui::Color32 {
    resolved().border
}

pub(crate) fn primary_color() -> egui::Color32 {
    resolved().primary
}

pub(crate) fn primary_hover_color() -> egui::Color32 {
    resolved().primary_hover
}

pub(crate) fn brand_pocket_word() -> egui::Color32 {
    resolved().brand_pocket_word
}

pub(crate) fn brand_emulator_word() -> egui::Color32 {
    resolved().brand_emulator_word
}

pub(crate) fn muted_text() -> egui::Color32 {
    resolved().muted_fg
}

pub(crate) fn success_text() -> egui::Color32 {
    resolved().success
}

pub(crate) fn warning_text() -> egui::Color32 {
    resolved().warning
}

pub(crate) fn destructive_text() -> egui::Color32 {
    resolved().destructive
}

pub(crate) fn info_text() -> egui::Color32 {
    resolved().info
}

pub(crate) fn success_bg() -> egui::Color32 {
    resolved().success_bg
}

pub(crate) fn warning_bg() -> egui::Color32 {
    resolved().warning_bg
}

pub(crate) fn destructive_bg() -> egui::Color32 {
    resolved().destructive_bg
}

pub(crate) fn info_bg() -> egui::Color32 {
    resolved().info_bg
}

pub(crate) fn platform_bg() -> egui::Color32 {
    resolved().platform_bg
}

pub(crate) fn platform_text() -> egui::Color32 {
    resolved().platform_text
}

pub(crate) fn text_display(s: impl Into<String>) -> egui::RichText {
    egui::RichText::new(s.into())
        .size(46.0)
        .strong()
        .color(resolved().fg)
}

pub(crate) fn text_h3(s: impl Into<String>) -> egui::RichText {
    egui::RichText::new(s.into()).size(22.0).strong()
}

pub(crate) fn text_body(s: impl Into<String>) -> egui::RichText {
    egui::RichText::new(s.into()).size(16.0).color(fg_text())
}

pub(crate) fn text_caption(s: impl Into<String>) -> egui::RichText {
    egui::RichText::new(s.into())
        .size(12.0)
        .color(resolved().muted_fg)
}

pub(crate) fn text_eyebrow(s: impl Into<String>) -> egui::RichText {
    egui::RichText::new(s.into())
        .size(11.0)
        .color(accent_text())
        .monospace()
}

pub(crate) fn text_mono(s: impl Into<String>) -> egui::RichText {
    egui::RichText::new(s.into()).size(13.0).monospace()
}

fn shadow_md(t: &ResolvedTheme) -> egui::epaint::Shadow {
    egui::epaint::Shadow {
        offset: egui::vec2(0.0, t.shadow_offset_y),
        blur: t.shadow_blur,
        spread: 0.0,
        color: t.shadow_color,
    }
}

pub(crate) fn section_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let t = resolved();
    egui::Frame::group(ui.style())
        .fill(t.surface_2)
        .rounding(egui::Rounding::same(t.radius_xl))
        .stroke(egui::Stroke::new(1.0, t.border))
        .shadow(shadow_md(t))
        .inner_margin(egui::Margin::same(t.space_3 + 2.0))
        .show(ui, |ui| add_contents(ui));
}

pub(super) fn apply_retro_style(ctx: &egui::Context) {
    let t = resolved();
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(t.space_3, t.space_2 + 2.0);
    style.spacing.button_padding = egui::vec2(t.space_3 + 2.0, t.space_2 + 1.0);
    style.spacing.window_margin = egui::Margin::same(t.space_4);
    style.spacing.menu_margin = egui::Margin::same(t.space_2);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(t.radius_sm);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(t.radius_sm);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(t.radius_sm);
    style.visuals.widgets.active.rounding = egui::Rounding::same(t.radius_sm);
    style.visuals.widgets.open.rounding = egui::Rounding::same(t.radius_sm);
    style.visuals.window_rounding = egui::Rounding::same(t.radius_lg);
    style.text_styles = [
        (
            egui::TextStyle::Heading,
            egui::FontId::new(34.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Monospace,
            egui::FontId::new(14.0, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Small,
            egui::FontId::new(12.0, egui::FontFamily::Proportional),
        ),
    ]
    .into();
    ctx.set_style(style);

    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(t.fg);
    visuals.window_fill = t.bg;
    visuals.panel_fill = t.bg;
    visuals.extreme_bg_color = t.surface;
    visuals.faint_bg_color = t.surface_2;
    visuals.code_bg_color = t.surface_3;
    visuals.window_stroke = egui::Stroke::new(1.0, t.border);
    visuals.widgets.noninteractive.bg_fill = t.surface;
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, t.muted_fg);
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, t.border);
    visuals.widgets.inactive.bg_fill = t.surface_2;
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, t.border);
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, t.fg);
    visuals.widgets.hovered.bg_fill = t.primary_hover;
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, t.primary);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
    visuals.widgets.active.bg_fill = t.primary;
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, t.primary_hover);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
    visuals.widgets.open.bg_fill = t.lcd_deep;
    visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0, t.accent);
    visuals.selection.bg_fill = t.primary;
    visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
    visuals.hyperlink_color = t.dmg_blue;
    visuals.warn_fg_color = t.warning;
    visuals.error_fg_color = t.destructive;
    ctx.set_visuals(visuals);
}
