//! Which console a ROM targets, and what PocketEmulator can run today.
//!
//! ## Reality check (roadmap)
//!
//! - **DMG (“Game Boy”)** — current core is DMG-first; most `.gb` / dual-mode `.gbc` titles that
//!   still run on DMG are OK.
//! - **GBC-only** — ROMs with header flag requiring Color hardware (e.g. Pokémon Crystal) need a
//!   **CGB mode**: extra VRAM/palettes, speed switch, HDMA, etc. That is **weeks–months** of work on
//!   top of this codebase, not a toggle.

use std::path::Path;

/// High-level target encoded in the ROM or filename.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomTarget {
    /// Original Game Boy (and dual-mode carts when run in DMG compatibility).
    DmgCompatible,
    /// Needs Game Boy Color hardware (CGB flag 0xC0 or equivalent).
    GbcRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchCompatibility {
    /// Can run with the current DMG-focused core (maybe without accurate Color graphics).
    Supported,
    /// Recognized but not emulated yet (clear error instead of a broken in-game screen).
    Unsupported { reason: &'static str },
}

impl RomTarget {
    pub fn launch_compatibility(self) -> LaunchCompatibility {
        match self {
            RomTarget::DmgCompatible | RomTarget::GbcRequired => LaunchCompatibility::Supported,
        }
    }
}

/// Whether [`GameBoy`](crate::gameboy::GameBoy) can run this path (extension + header when needed).
///
/// Returns `Err` with a short message for unsupported targets (bad extension, etc.).
pub fn rom_launch_check(path: &Path) -> Result<(), String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext != "gb" && ext != "gbc" {
        return Err(format!(
            "Unsupported ROM format: .{} (supported extensions: .gb, .gbc)",
            if ext.is_empty() { "<none>" } else { &ext }
        ));
    }

    let data = std::fs::read(path).map_err(|e| e.to_string())?;
    let take = data.len().min(0x150);
    let prefix = data[..take].to_vec();

    match classify_rom(path, &prefix).launch_compatibility() {
        LaunchCompatibility::Supported => Ok(()),
        LaunchCompatibility::Unsupported { reason } => Err(reason.to_string()),
    }
}

/// Classify using file extension and, when possible, GB cartridge header byte `0x0143`.
pub fn classify_rom(_path: &Path, rom_prefix: &[u8]) -> RomTarget {
    // Heuristic: very large “ROM” that isn’t a GB header — don’t try to parse as GB.
    if rom_prefix.len() >= 0x150 {
        if matches!(
            crate::cartridge::CgbSupport::from_header_byte(rom_prefix[0x0143]),
            crate::cartridge::CgbSupport::GbcOnly
        ) {
            return RomTarget::GbcRequired;
        }
    }

    RomTarget::DmgCompatible
}
