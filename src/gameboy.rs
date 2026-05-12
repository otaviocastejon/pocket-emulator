use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::bus::Bus;
use crate::cartridge::{self, CartridgeError};
use crate::cpu::Cpu;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Cartridge(#[from] CartridgeError),
}

pub struct GameBoy {
    pub cpu: Cpu,
    pub bus: Bus,
    save_path: Option<std::path::PathBuf>,
    rom_path: Option<std::path::PathBuf>,
}

impl Drop for GameBoy {
    fn drop(&mut self) {
        let _ = self.persist_save();
    }
}

impl GameBoy {
    pub fn new(bus: Bus) -> Self {
        let cgb = bus.cgb_mode;
        Self {
            cpu: Cpu::new_post_boot(cgb),
            bus,
            save_path: None,
            rom_path: None,
        }
    }

    /// Load ROM from disk; if battery-backed, tries to load `.sav` next to the ROM.
    pub fn from_rom_file(path: impl AsRef<Path>) -> Result<Self, LoadError> {
        Self::from_rom_file_with_save_path(path.as_ref(), None::<&Path>)
    }

    /// Load ROM and optionally override `.sav` path location.
    pub fn from_rom_file_with_save_path(
        path: impl AsRef<Path>,
        save_path_override: Option<impl AsRef<Path>>,
    ) -> Result<Self, LoadError> {
        let path = path.as_ref();
        let bytes = fs::read(path)?;
        let mut cart = cartridge::from_rom(bytes)?;
        let save_path = if cart.battery_backed() {
            save_path_override
                .as_ref()
                .map(|p| p.as_ref().to_path_buf())
                .or_else(|| Some(path.with_extension("sav")))
        } else {
            None
        };
        if let Some(ref sp) = save_path {
            if let Ok(data) = fs::read(sp) {
                cart.load_ram(&data);
            }
        }
        let bus = Bus::new(cart);
        let cgb = bus.cgb_mode;
        Ok(Self {
            cpu: Cpu::new_post_boot(cgb),
            bus,
            save_path,
            rom_path: Some(path.to_path_buf()),
        })
    }

    pub fn persist_save(&self) -> std::io::Result<()> {
        let Some(ref path) = self.save_path else {
            return Ok(());
        };
        if !self.bus.cartridge.battery_backed() {
            return Ok(());
        }
        if let Some(ram) = self.bus.cartridge.external_ram() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            if path.exists() {
                let _ = fs::copy(path, path.with_extension("sav.bak"));
            }
            let tmp = path.with_extension("sav.tmp");
            fs::write(&tmp, ram)?;
            fs::rename(tmp, path)?;
        }
        Ok(())
    }

    /// Reload cartridge RAM from `.sav` file (battery-backed carts only).
    pub fn reload_save(&mut self) -> std::io::Result<()> {
        let Some(ref path) = self.save_path else {
            return Ok(());
        };
        if !self.bus.cartridge.battery_backed() {
            return Ok(());
        }
        let data = fs::read(path).or_else(|_| fs::read(path.with_extension("sav.bak")))?;
        self.bus.cartridge.load_ram(&data);
        Ok(())
    }

    pub fn serial_output(&self) -> String {
        self.bus.serial.buffer.clone()
    }

    pub fn take_serial(&mut self) -> String {
        self.bus.serial.take_output()
    }

    pub fn save_path(&self) -> Option<&Path> {
        self.save_path.as_deref()
    }

    pub fn rom_path(&self) -> Option<&Path> {
        self.rom_path.as_deref()
    }

    pub fn save_dir(&self) -> Option<&Path> {
        self.save_path.as_ref().and_then(|p| p.parent())
    }

    pub fn cartridge_ram_snapshot(&self) -> Option<Vec<u8>> {
        self.bus.cartridge.external_ram().map(|r| r.to_vec())
    }

    pub fn load_cartridge_ram_snapshot(&mut self, data: &[u8]) {
        self.bus.cartridge.load_ram(data);
    }

    /// Advance by `t_cycles` **T-states** (4.19 MHz ticks).
    pub fn run_t_cycles(&mut self, mut cycles: u32) -> u32 {
        let target = cycles;
        while cycles > 0 {
            let m = self.cpu.step(&mut self.bus) as u32;
            let t = m * 4;
            self.bus.timer.step(t, &mut self.bus.interrupts.if_);
            self.bus.ppu.step(t, &mut self.bus.interrupts.if_);
            for _ in 0..self.bus.ppu.take_hblank_dma_edges() {
                self.bus.hdma_hblank_step();
            }
            self.bus.apu.step(t);
            cycles = cycles.saturating_sub(t);
        }
        target
    }

    /// Run one CPU instruction worth of system time.
    pub fn step_instruction(&mut self) -> u32 {
        let m = self.cpu.step(&mut self.bus) as u32;
        let t = m * 4;
        self.bus.timer.step(t, &mut self.bus.interrupts.if_);
        self.bus.ppu.step(t, &mut self.bus.interrupts.if_);
        for _ in 0..self.bus.ppu.take_hblank_dma_edges() {
            self.bus.hdma_hblank_step();
        }
        self.bus.apu.step(t);
        t
    }

    /// Advance ~one frame (70224 T-states).
    pub fn run_frame(&mut self) {
        self.run_t_cycles(70224);
    }
}
