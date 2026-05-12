//! ROM target classification from cartridge header / extension.

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomTarget {
    DmgCompatible,
    GbcRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchCompatibility {
    Supported,
    Unsupported { reason: &'static str },
}

impl RomTarget {
    pub fn launch_compatibility(self) -> LaunchCompatibility {
        match self {
            RomTarget::DmgCompatible | RomTarget::GbcRequired => LaunchCompatibility::Supported,
        }
    }
}

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

pub fn classify_rom(_path: &Path, rom_prefix: &[u8]) -> RomTarget {
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
