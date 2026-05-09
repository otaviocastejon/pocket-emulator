//! Cartridge header parsing (0x0100–0x014F in ROM).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("ROM too small for header (need 0x150 bytes)")]
    TooSmall,
    #[error("invalid header checksum")]
    ChecksumMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Unknown(u8),
}

impl CartridgeType {
    pub fn from_byte(b: u8) -> Self {
        match b {
            0x00 => Self::RomOnly,
            0x01 => Self::Mbc1,
            0x02 => Self::Mbc1Ram,
            0x03 => Self::Mbc1RamBattery,
            0x05 => Self::Mbc2,
            0x06 => Self::Mbc2Battery,
            0x0F => Self::Mbc3TimerBattery,
            0x10 => Self::Mbc3TimerRamBattery,
            0x11 => Self::Mbc3,
            0x12 => Self::Mbc3Ram,
            0x13 => Self::Mbc3RamBattery,
            0x19 => Self::Mbc5,
            0x1A => Self::Mbc5Ram,
            0x1B => Self::Mbc5RamBattery,
            0x1C => Self::Mbc5Rumble,
            0x1D => Self::Mbc5RumbleRam,
            0x1E => Self::Mbc5RumbleRamBattery,
            _ => Self::Unknown(b),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RomOnly => "ROM only (MBC0)",
            Self::Mbc1 => "MBC1",
            Self::Mbc1Ram => "MBC1+RAM",
            Self::Mbc1RamBattery => "MBC1+RAM+BATTERY",
            Self::Mbc2 => "MBC2",
            Self::Mbc2Battery => "MBC2+BATTERY",
            Self::Mbc3 => "MBC3",
            Self::Mbc3Ram => "MBC3+RAM",
            Self::Mbc3RamBattery => "MBC3+RAM+BATTERY",
            Self::Mbc3TimerBattery => "MBC3+TIMER+BATTERY",
            Self::Mbc3TimerRamBattery => "MBC3+TIMER+RAM+BATTERY",
            Self::Mbc5 => "MBC5",
            Self::Mbc5Ram => "MBC5+RAM",
            Self::Mbc5RamBattery => "MBC5+RAM+BATTERY",
            Self::Mbc5Rumble => "MBC5+RUMBLE",
            Self::Mbc5RumbleRam => "MBC5+RUMBLE+RAM",
            Self::Mbc5RumbleRamBattery => "MBC5+RUMBLE+RAM+BATTERY",
            Self::Unknown(_) => "Unknown MBC",
        }
    }

    pub fn has_ram(&self) -> bool {
        matches!(
            self,
            Self::Mbc1Ram
                | Self::Mbc1RamBattery
                | Self::Mbc2
                | Self::Mbc2Battery
                | Self::Mbc3Ram
                | Self::Mbc3RamBattery
                | Self::Mbc3TimerRamBattery
                | Self::Mbc5Ram
                | Self::Mbc5RamBattery
                | Self::Mbc5RumbleRam
                | Self::Mbc5RumbleRamBattery
        )
    }

    pub fn has_battery(&self) -> bool {
        matches!(
            self,
            Self::Mbc1RamBattery
                | Self::Mbc2Battery
                | Self::Mbc3RamBattery
                | Self::Mbc3TimerBattery
                | Self::Mbc3TimerRamBattery
                | Self::Mbc5RamBattery
                | Self::Mbc5RumbleRamBattery
        )
    }
}

/// Cartridge header byte `0x0143` (CGB flag). See Pan Docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgbSupport {
    None,
    Dual,
    GbcOnly,
}

impl CgbSupport {
    pub fn from_header_byte(b: u8) -> Self {
        match b {
            0xC0 => Self::GbcOnly,
            0x80 => Self::Dual,
            _ if (b & 0x80) != 0 => Self::Dual,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub title: String,
    pub cgb_support: CgbSupport,
    pub cartridge_type: CartridgeType,
    pub rom_banks: usize,
    pub ram_size_bytes: usize,
    pub header_checksum_ok: bool,
    pub global_checksum: u16,
}

impl Header {
    pub fn parse(rom: &[u8]) -> Result<Self, HeaderError> {
        if rom.len() < 0x150 {
            return Err(HeaderError::TooSmall);
        }

        let mut title_bytes = Vec::new();
        for &b in &rom[0x0134..=0x0142] {
            if b == 0 {
                break;
            }
            title_bytes.push(b);
        }
        let title = String::from_utf8_lossy(&title_bytes).trim().to_string();

        let cgb_support = CgbSupport::from_header_byte(rom[0x0143]);

        let ctype = CartridgeType::from_byte(rom[0x0147]);

        let rom_banks = match rom[0x0148] {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => 2,
        };

        let ram_size_bytes = match rom[0x0149] {
            0x00 => 0,
            0x01 => 2 * 1024,
            0x02 => 8 * 1024,
            0x03 => 32 * 1024,
            0x04 => 128 * 1024,
            0x05 => 64 * 1024,
            _ => 0,
        };

        let mut sum: u32 = 0;
        for addr in 0x0134..=0x014C {
            sum = sum.wrapping_add(rom[addr as usize] as u32).wrapping_add(1);
        }
        let header_checksum_ok = (sum & 0xFF) == 0;

        let global_checksum = u16::from_be_bytes([rom[0x014E], rom[0x014F]]);

        Ok(Self {
            title,
            cgb_support,
            cartridge_type: ctype,
            rom_banks,
            ram_size_bytes,
            header_checksum_ok,
            global_checksum,
        })
    }
}
