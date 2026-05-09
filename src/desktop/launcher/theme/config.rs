//! TOML schema and hex parsing for the launcher theme file.
//! See `assets/default_launcher_theme.toml` for the full key set.

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ThemeFile {
    #[serde(default)]
    pub colors: ColorsPartial,
    #[serde(default)]
    pub spacing: SpacingPartial,
    #[serde(default)]
    pub radius: RadiusPartial,
    #[serde(default)]
    pub shadow: ShadowPartial,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ColorsPartial {
    #[serde(default)]
    pub bg: Option<String>,
    #[serde(default)]
    pub fg: Option<String>,
    #[serde(default)]
    pub surface: Option<String>,
    #[serde(default)]
    pub surface_2: Option<String>,
    #[serde(default)]
    pub surface_3: Option<String>,
    #[serde(default)]
    pub border: Option<String>,
    #[serde(default)]
    pub primary: Option<String>,
    #[serde(default)]
    pub primary_hover: Option<String>,
    #[serde(default)]
    pub accent: Option<String>,
    #[serde(default)]
    pub muted_fg: Option<String>,
    #[serde(default)]
    pub dmg_blue: Option<String>,
    #[serde(default)]
    pub lcd_deep: Option<String>,
    #[serde(default)]
    pub success: Option<String>,
    #[serde(default)]
    pub warning: Option<String>,
    #[serde(default)]
    pub destructive: Option<String>,
    #[serde(default)]
    pub info: Option<String>,
    #[serde(default)]
    pub brand_pocket_word: Option<String>,
    #[serde(default)]
    pub brand_emulator_word: Option<String>,
    #[serde(default)]
    pub success_bg: Option<String>,
    #[serde(default)]
    pub warning_bg: Option<String>,
    #[serde(default)]
    pub destructive_bg: Option<String>,
    #[serde(default)]
    pub info_bg: Option<String>,
    #[serde(default)]
    pub platform_bg: Option<String>,
    #[serde(default)]
    pub platform_text: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct SpacingPartial {
    #[serde(default)]
    pub space_1: Option<f64>,
    #[serde(default)]
    pub space_2: Option<f64>,
    #[serde(default)]
    pub space_3: Option<f64>,
    #[serde(default)]
    pub space_4: Option<f64>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct RadiusPartial {
    #[serde(default)]
    pub sm: Option<f64>,
    #[serde(default)]
    pub lg: Option<f64>,
    #[serde(default)]
    pub xl: Option<f64>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ShadowPartial {
    #[serde(default)]
    pub blur: Option<f64>,
    #[serde(default)]
    pub offset_y: Option<f64>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub alpha: Option<u8>,
}

pub fn parse_theme_toml(source: &str) -> Result<ThemeFile, toml::de::Error> {
    toml::from_str(source)
}

/// `"#RRGGBB"` or `"RRGGBB"`.
pub fn parse_hex_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim();
    let s = s.strip_prefix('#').unwrap_or(s);
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some((r, g, b))
}

pub const EMBEDDED_DEFAULT_THEME: &str =
    include_str!("../../../../assets/default_launcher_theme.toml");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_accepts_hash_prefix() {
        assert_eq!(parse_hex_rgb("#ff4081"), Some((255, 64, 129)));
        assert_eq!(parse_hex_rgb("121214"), Some((18, 18, 20)));
    }

    #[test]
    fn embedded_default_theme_parses() {
        parse_theme_toml(EMBEDDED_DEFAULT_THEME).expect("embedded TOML");
    }
}
