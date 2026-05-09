use std::time::{SystemTime, UNIX_EPOCH};

use super::{Cartridge, CgbSupport};

#[derive(Debug, Clone)]
pub struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enabled: bool,
    rom_bank: u8,
    ram_bank: u8,
    battery: bool,
    /// `$00` → `$01` writes to `6000-7FFF` arm latch; second step freezes RTC into `rtc_latched`.
    rtc_latch_saw_zero: bool,
    /// Frozen S,M,H,DL,DH after latch (Pan Docs MBC3); reads at `A000-BFFF` use these while mapped to `08-0C`.
    rtc_latched: Option<[u8; 5]>,
}

impl Mbc3 {
    pub fn new(rom: Vec<u8>, ram_size: usize, battery: bool) -> Self {
        Self {
            rom,
            ram: vec![0; ram_size.max(0x2000)],
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
            battery,
            rtc_latch_saw_zero: false,
            rtc_latched: None,
        }
    }

    /// Live RTC registers from wall clock (9-bit day counter, DH carry/halt cleared).
    fn rtc_live_registers(&self) -> [u8; 5] {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let days = ((t / 86400) % 512) as u16;
        let sec_day = t % 86400;
        let h = (sec_day / 3600) as u8;
        let m = ((sec_day % 3600) / 60) as u8;
        let s = (sec_day % 60) as u8;
        let dl = days as u8;
        let dh = ((days >> 8) as u8) & 1;
        [s, m, h, dl, dh]
    }

    fn rom_banks(&self) -> usize {
        (self.rom.len() / 0x4000).max(1)
    }
}

impl Cartridge for Mbc3 {
    fn read_rom(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if addr < 0x4000 {
            self.rom[addr]
        } else {
            let bank = if self.rom_bank == 0 {
                1
            } else {
                self.rom_bank as usize % self.rom_banks()
            };
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
            0x2000..=0x3FFF => {
                self.rom_bank = value & 0x7F;
                if self.rom_bank == 0 {
                    self.rom_bank = 1;
                }
            }
            0x4000..=0x5FFF => {
                self.ram_bank = value;
            }
            // Latch Clock Data — required for MBC3 RTC reads (Pokémon Crystal save, etc.).
            0x6000..=0x7FFF => match value {
                0 => self.rtc_latch_saw_zero = true,
                1 if self.rtc_latch_saw_zero => {
                    self.rtc_latched = Some(self.rtc_live_registers());
                    self.rtc_latch_saw_zero = false;
                }
                _ => self.rtc_latch_saw_zero = false,
            },
            _ => {}
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        if self.ram_bank >= 0x08 && self.ram_bank <= 0x0C {
            let regs = self
                .rtc_latched
                .unwrap_or_else(|| self.rtc_live_registers());
            let idx = (self.ram_bank - 0x08) as usize;
            return regs[idx];
        }
        if self.ram.is_empty() {
            return 0xFF;
        }
        let offset = (self.ram_bank as usize & 0x07) * 0x2000 + ((addr as usize - 0xA000) & 0x1FFF);
        self.ram.get(offset).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }
        if self.ram_bank >= 0x08 && self.ram_bank <= 0x0C {
            return;
        }
        if self.ram.is_empty() {
            return;
        }
        let offset = (self.ram_bank as usize & 0x07) * 0x2000 + ((addr as usize - 0xA000) & 0x1FFF);
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
