use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use eframe::egui;
use winit::event::VirtualKeyCode;

/// UI badge label from a lowercased file extension (`gb` / `gbc`).
pub(crate) fn rom_extension_platform_label(ext_lower: &str) -> &'static str {
    match ext_lower {
        "gbc" => "GBC",
        _ => "GB",
    }
}

pub(crate) fn display_game_name(path: &Path, max_chars: usize) -> String {
    let raw = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Game")
        .to_string();
    // Imported library names are normalized as "<name>-<12hex>".
    // Strip hash suffix for a cleaner user-facing title.
    let cleaned = if raw.len() > 13
        && raw.as_bytes().get(raw.len().saturating_sub(13)) == Some(&b'-')
        && raw.chars().rev().take(12).all(|c| c.is_ascii_hexdigit())
    {
        raw[..raw.len() - 13].replace('_', " ")
    } else {
        raw.replace('_', " ")
    };
    if cleaned.chars().count() <= max_chars {
        cleaned
    } else {
        let clipped: String = cleaned.chars().take(max_chars.saturating_sub(1)).collect();
        format!("{clipped}…")
    }
}

pub(crate) fn format_last_played(last_played: Option<u64>) -> String {
    let Some(last_played) = last_played else {
        return "unknown".to_string();
    };
    let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return "unknown".to_string();
    };
    let now = now.as_secs();
    if now <= last_played + 60 {
        return "just now".to_string();
    }
    let delta = now.saturating_sub(last_played);
    if delta < 3600 {
        return format!("{} min ago", delta / 60);
    }
    if delta < 86_400 {
        return format!("{} h ago", delta / 3600);
    }
    if delta < 86_400 * 30 {
        return format!("{} d ago", delta / 86_400);
    }
    "a while ago".to_string()
}

const KEY_CHOICES: &[(&str, VirtualKeyCode)] = &[
    ("Z", VirtualKeyCode::Z),
    ("X", VirtualKeyCode::X),
    ("C", VirtualKeyCode::C),
    ("A", VirtualKeyCode::A),
    ("S", VirtualKeyCode::S),
    ("D", VirtualKeyCode::D),
    ("Q", VirtualKeyCode::Q),
    ("W", VirtualKeyCode::W),
    ("E", VirtualKeyCode::E),
    ("R", VirtualKeyCode::R),
    ("Arrow Up", VirtualKeyCode::Up),
    ("Arrow Down", VirtualKeyCode::Down),
    ("Arrow Left", VirtualKeyCode::Left),
    ("Arrow Right", VirtualKeyCode::Right),
    ("Enter", VirtualKeyCode::Return),
    ("Space", VirtualKeyCode::Space),
    ("Left Shift", VirtualKeyCode::LShift),
    ("Right Shift", VirtualKeyCode::RShift),
    ("Tab", VirtualKeyCode::Tab),
    ("Backspace", VirtualKeyCode::Back),
];

pub(super) fn key_picker(ui: &mut egui::Ui, binding: &mut VirtualKeyCode, id: &str) {
    let selected = key_label(*binding);
    egui::ComboBox::from_id_source(id)
        .selected_text(selected)
        .width(170.0)
        .show_ui(ui, |ui| {
            for (name, key) in KEY_CHOICES {
                ui.selectable_value(binding, *key, *name);
            }
        });
}

fn key_label(key: VirtualKeyCode) -> &'static str {
    for (name, k) in KEY_CHOICES {
        if *k == key {
            return name;
        }
    }
    "Custom"
}
