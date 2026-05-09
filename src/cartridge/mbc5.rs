use super::{Cartridge, CgbSupport};

#[derive(Debug, Clone)]
pub struct Mbc5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enabled: bool,
    rom_bank: u16,
    ram_bank: u8,
    battery: bool,
}

impl Mbc5 {
    pub fn new(rom: Vec<u8>, ram_size: usize, battery: bool) -> Self {
        Self {
            rom,
            ram: vec![0; ram_size.max(0x2000)],
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
            battery,
        }
    }

    fn rom_banks(&self) -> usize {
        (self.rom.len() / 0x4000).max(1)
    }

    fn effective_rom_bank(&self) -> usize {
        (self.rom_bank as usize) % self.rom_banks()
    }
}

impl Cartridge for Mbc5 {
    fn read_rom(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if addr < 0x4000 {
            self.rom[addr]
        } else {
            let bank = self.effective_rom_bank();
            self.rom
                .get(bank * 0x4000 + (addr - 0x4000))
                .copied()
                .unwrap_or(0xFF)
        }
    }

    fn write_rom(&mut self, addr: u16, value: u8) {
        let a = addr as usize;
        match a {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x2000..=0x2FFF => {
                self.rom_bank = (self.rom_bank & 0x100) | (value as u16);
            }
            0x3000..=0x3FFF => {
                self.rom_bank = (self.rom_bank & 0xFF) | (((value & 1) as u16) << 8);
            }
            0x4000..=0x5FFF => {
                self.ram_bank = value & 0x0F;
            }
            _ => {}
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_enabled || self.ram.is_empty() {
            return 0xFF;
        }
        let bank = self.ram_bank as usize;
        let offset = bank * 0x2000 + ((addr as usize - 0xA000) & 0x1FFF);
        self.ram.get(offset).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled || self.ram.is_empty() {
            return;
        }
        let bank = self.ram_bank as usize;
        let offset = bank * 0x2000 + ((addr as usize - 0xA000) & 0x1FFF);
        if let Some(slot) = self.ram.get_mut(offset) {
            *slot = value;
        }
    }

    fn battery_backed(&self) -> bool {
        self.battery
    }

    fn external_ram(&self) -> Option<&[u8]> {
        (!self.ram.is_empty()).then_some(self.ram.as_slice())
    }

    fn external_ram_mut(&mut self) -> Option<&mut [u8]> {
        (!self.ram.is_empty()).then_some(self.ram.as_mut_slice())
    }

    fn load_ram(&mut self, data: &[u8]) {
        if self.ram.is_empty() {
            return;
        }
        let n = data.len().min(self.ram.len());
        self.ram[..n].copy_from_slice(&data[..n]);
    }

    fn cgb_support(&self) -> CgbSupport {
        CgbSupport::from_header_byte(self.rom.get(0x143).copied().unwrap_or(0))
    }
}
