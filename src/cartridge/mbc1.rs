use super::{Cartridge, CgbSupport};

#[derive(Debug, Clone)]
pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enabled: bool,
    rom_bank: u8,
    ram_bank: u8,
    banking_mode: u8,
    battery: bool,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>, ram_size: usize, battery: bool) -> Self {
        Self {
            rom,
            ram: vec![0; ram_size.max(0x2000)],
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
            banking_mode: 0,
            battery,
        }
    }

    fn rom_bank_mask(&self) -> u8 {
        let banks = (self.rom.len() / 0x4000).max(1) as u8;
        banks.saturating_sub(1)
    }

    fn rom_bank_lo(&self) -> u8 {
        let mut b = self.rom_bank & 0x1F;
        if b == 0 {
            b = 1;
        }
        b & self.rom_bank_mask()
    }

    fn rom_bank_hi_bits(&self) -> usize {
        (self.ram_bank as usize & 0x03) << 5
    }

    fn effective_rom_bank_n(&self) -> usize {
        let num = (self.rom.len() / 0x4000).max(1);
        let mut bank = self.rom_bank_lo() as usize;
        if self.banking_mode == 1 {
            bank |= self.rom_bank_hi_bits();
        }
        bank % num
    }
}

impl Cartridge for Mbc1 {
    fn read_rom(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if addr < 0x4000 {
            let num = (self.rom.len() / 0x4000).max(1);
            let bank0 = if self.banking_mode == 1 {
                self.rom_bank_hi_bits() % num
            } else {
                0
            };
            self.rom[bank0 * 0x4000 + addr]
        } else {
            let bank = self.effective_rom_bank_n();
            let base = bank * 0x4000;
            self.rom
                .get(base + (addr - 0x4000))
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
            0x2000..=0x3FFF => {
                let mut bank = value & 0x1F;
                if bank == 0 {
                    bank = 1;
                }
                self.rom_bank = bank;
            }
            0x4000..=0x5FFF => {
                self.ram_bank = value & 0x03;
            }
            0x6000..=0x7FFF => {
                self.banking_mode = value & 0x01;
            }
            _ => {}
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_enabled || self.ram.is_empty() {
            return 0xFF;
        }
        let offset = if self.banking_mode == 0 {
            (addr as usize - 0xA000) & (self.ram.len() - 1)
        } else {
            let bank = (self.ram_bank as usize) & 0x03;
            bank * 0x2000 + ((addr as usize - 0xA000) & 0x1FFF)
        };
        self.ram.get(offset).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled || self.ram.is_empty() {
            return;
        }
        let offset = if self.banking_mode == 0 {
            (addr as usize - 0xA000) & (self.ram.len() - 1)
        } else {
            let bank = (self.ram_bank as usize) & 0x03;
            bank * 0x2000 + ((addr as usize - 0xA000) & 0x1FFF)
        };
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
