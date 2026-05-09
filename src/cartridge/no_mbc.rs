use super::{Cartridge, CgbSupport};

/// ROM only (32–256 KiB), no banking.
#[derive(Debug, Clone)]
pub struct NoMbc {
    rom: Vec<u8>,
}

impl NoMbc {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }
}

impl Cartridge for NoMbc {
    fn read_rom(&self, addr: u16) -> u8 {
        self.rom.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _addr: u16, _value: u8) {}

    fn read_ram(&self, _addr: u16) -> u8 {
        0xFF
    }

    fn write_ram(&mut self, _addr: u16, _value: u8) {}

    fn battery_backed(&self) -> bool {
        false
    }

    fn external_ram(&self) -> Option<&[u8]> {
        None
    }

    fn external_ram_mut(&mut self) -> Option<&mut [u8]> {
        None
    }

    fn load_ram(&mut self, _data: &[u8]) {}

    fn cgb_support(&self) -> CgbSupport {
        CgbSupport::from_header_byte(self.rom.get(0x143).copied().unwrap_or(0))
    }
}
