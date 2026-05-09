mod header;
mod mbc1;
mod mbc3;
mod mbc5;
mod no_mbc;

pub use header::{CartridgeType, CgbSupport, Header, HeaderError};

use header::CartridgeType as CT;
use mbc1::Mbc1;
use mbc3::Mbc3;
use mbc5::Mbc5;
use no_mbc::NoMbc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CartridgeError {
    #[error(transparent)]
    Header(#[from] HeaderError),
    #[error("unsupported cartridge type: {0:?}")]
    UnsupportedType(CT),
}

pub trait Cartridge: Send {
    fn read_rom(&self, addr: u16) -> u8;
    fn write_rom(&mut self, addr: u16, value: u8);
    fn read_ram(&self, addr: u16) -> u8;
    fn write_ram(&mut self, addr: u16, value: u8);
    fn battery_backed(&self) -> bool;
    fn external_ram(&self) -> Option<&[u8]>;
    fn external_ram_mut(&mut self) -> Option<&mut [u8]>;
    fn load_ram(&mut self, data: &[u8]);
    /// Cartridge header byte `0x0143` (CGB compatibility).
    fn cgb_support(&self) -> CgbSupport;
}

pub fn from_rom(rom: Vec<u8>) -> Result<Box<dyn Cartridge>, CartridgeError> {
    let header = Header::parse(&rom)?;
    let ram_size = if header.cartridge_type.has_ram() {
        if matches!(header.cartridge_type, CT::Mbc2 | CT::Mbc2Battery) {
            512
        } else {
            header.ram_size_bytes.max(0x2000)
        }
    } else {
        0
    };

    let battery = header.cartridge_type.has_battery();

    let cart: Box<dyn Cartridge> = match header.cartridge_type {
        CT::RomOnly => Box::new(NoMbc::new(rom)),
        CT::Mbc1 | CT::Mbc1Ram | CT::Mbc1RamBattery => Box::new(Mbc1::new(rom, ram_size, battery)),
        CT::Mbc2 | CT::Mbc2Battery => Box::new(Mbc1::new(rom, 512, battery)), // 512 nibbles mapped as bytes
        CT::Mbc3
        | CT::Mbc3Ram
        | CT::Mbc3RamBattery
        | CT::Mbc3TimerBattery
        | CT::Mbc3TimerRamBattery => Box::new(Mbc3::new(rom, ram_size, battery)),
        CT::Mbc5
        | CT::Mbc5Ram
        | CT::Mbc5RamBattery
        | CT::Mbc5Rumble
        | CT::Mbc5RumbleRam
        | CT::Mbc5RumbleRamBattery => Box::new(Mbc5::new(rom, ram_size, battery)),
        CT::Unknown(_) => return Err(CartridgeError::UnsupportedType(header.cartridge_type)),
    };

    Ok(cart)
}
