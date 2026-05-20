use crate::apu::Apu;
use crate::cartridge::{Cartridge, CgbSupport};
use crate::interrupts::Interrupts;
use crate::joypad::Joypad;
use crate::ppu::Ppu;
use crate::serial::Serial;
use crate::timer::Timer;

pub struct Bus {
    pub cartridge: Box<dyn Cartridge>,
    /// DMG: flat 8 KiB via `0xC000–0xDFFF` (mirrored echo). CGB: 32 KiB; `0xC000–0xCFFF` bank 0,
    /// `0xD000–0xDFFF` bank from SVBK (`FF70`, 0 ⇒ bank 1).
    pub wram: [u8; 0x8000],
    pub hram: [u8; 0x7F],
    pub ppu: Ppu,
    pub timer: Timer,
    pub joypad: Joypad,
    pub interrupts: Interrupts,
    pub serial: Serial,
    pub apu: Apu,
    /// Cartridge requests CGB features (header `0x0143`).
    pub cgb_mode: bool,
    /// Speed switch pending (KEY1 bit 0); completed by `STOP` on CGB.
    prepare_speed_switch: bool,
    /// KEY1 bit 7 — affects KEY1 reads; timing still 1× in this core.
    pub double_speed: bool,
    /// FF70 lower 3 bits; 0 selects WRAM bank 1 at `0xD000`.
    svbk: u8,
    /// CGB VRAM DMA source/dest (FF51–FF54).
    hdma1: u8,
    hdma2: u8,
    hdma3: u8,
    hdma4: u8,
    /// HBlank DMA (FF55 bit 7 set): 16 bytes per visible scanline HBlank, not VBlank.
    hdma: Option<HdmaState>,
    /// T-cycles to stall the CPU during an in-progress general-purpose VRAM DMA.
    gdma_stall_t_cycles: u32,
}

/// Active CGB HBlank DMA (see Pan Docs FF55 bit 7).
struct HdmaState {
    src: u16,
    dst: u16,
    /// 16-byte blocks left to transfer (decremented after each HBlank chunk).
    blocks_remaining: u8,
    /// VBK (FF4F) latched when the transfer started.
    vram_bank: u8,
}

impl Bus {
    pub fn new(cartridge: Box<dyn Cartridge>) -> Self {
        let cgb_mode = matches!(
            cartridge.cgb_support(),
            CgbSupport::Dual | CgbSupport::GbcOnly
        );
        let mut ppu = Ppu::default();
        ppu.cgb_mode = cgb_mode;
        let mut out = Self {
            cartridge,
            wram: [0; 0x8000],
            hram: [0; 0x7F],
            ppu,
            timer: Timer::default(),
            joypad: Joypad::default(),
            interrupts: Interrupts::default(),
            serial: Serial::default(),
            apu: Apu::default(),
            cgb_mode,
            prepare_speed_switch: false,
            double_speed: false,
            svbk: 0,
            hdma1: 0,
            hdma2: 0,
            hdma3: 0,
            hdma4: 0,
            hdma: None,
            gdma_stall_t_cycles: 0,
        };
        out.serial.configure_link_from_env();
        out
    }

    pub fn gdma_stall_active(&self) -> bool {
        self.gdma_stall_t_cycles > 0
    }

    pub fn consume_gdma_stall(&mut self, t_cycles: u32) -> u32 {
        let take = t_cycles.min(self.gdma_stall_t_cycles);
        self.gdma_stall_t_cycles -= take;
        take
    }

    #[inline]
    fn svbk_bank_index(&self) -> usize {
        let b = self.svbk & 0x07;
        if b == 0 {
            1
        } else {
            b as usize
        }
    }

    /// Normalize WRAM echo `0xE000–0xFDFF` → `0xC000–0xDDFF`.
    #[inline]
    fn wram_addr_phys(addr: u16) -> u16 {
        if (0xE000..=0xFDFF).contains(&addr) {
            addr - 0x2000
        } else {
            addr
        }
    }

    fn read_wram(&self, addr: u16) -> u8 {
        let a = Self::wram_addr_phys(addr);
        if !self.cgb_mode {
            return self.wram[((a - 0xC000) & 0x1FFF) as usize];
        }
        match a {
            0xC000..=0xCFFF => self.wram[(a - 0xC000) as usize],
            0xD000..=0xDFFF => {
                let bank = self.svbk_bank_index();
                self.wram[bank * 0x1000 + (a - 0xD000) as usize]
            }
            _ => 0xFF,
        }
    }

    fn write_wram(&mut self, addr: u16, v: u8) {
        let a = Self::wram_addr_phys(addr);
        if !self.cgb_mode {
            let i = ((a - 0xC000) & 0x1FFF) as usize;
            self.wram[i] = v;
            return;
        }
        match a {
            0xC000..=0xCFFF => {
                let i = (a - 0xC000) as usize;
                self.wram[i] = v;
            }
            0xD000..=0xDFFF => {
                let bank = self.svbk_bank_index();
                let i = bank * 0x1000 + (a - 0xD000) as usize;
                self.wram[i] = v;
            }
            _ => {}
        }
    }

    fn read_key1(&self) -> u8 {
        let mut v = 0x7Eu8;
        if self.double_speed {
            v |= 0x80;
        }
        if self.prepare_speed_switch {
            v |= 0x01;
        }
        v
    }

    /// CGB `STOP` after writing prepare to KEY1: toggle speed and clear prepare.
    pub fn cgb_stop_speed_switch(&mut self) {
        if !self.cgb_mode || !self.prepare_speed_switch {
            return;
        }
        self.double_speed = !self.double_speed;
        self.prepare_speed_switch = false;
    }

    /// General-purpose VRAM DMA (FF55 bit 7 clear): copies `(ctrl & 0x7F + 1) × 16` bytes at once.
    /// Overflow past `$9FFF` drops bytes (no wrap into low VRAM).
    fn run_vram_dma(&mut self, ctrl: u8) {
        let blocks = (ctrl & 0x7F) as u32 + 1;
        let len = blocks * 0x10;
        let src_base = (((self.hdma1 as u16) << 8) | (self.hdma2 as u16)) & 0xFFF0;
        let dst_base = 0x8000u16 | ((((self.hdma3 as u16) << 8) | (self.hdma4 as u16)) & 0x1FF0);
        let bank = self.ppu.vram_bank & 1;
        for i in 0..len {
            let dst = dst_base.wrapping_add(i as u16);
            if !(0x8000..=0x9FFF).contains(&dst) {
                break;
            }
            let b = self.read_dma_source(src_base.wrapping_add(i as u16));
            self.ppu.write_vram_dma(dst, b, bank);
        }
        // Pan Docs: ~8 M-cycles per 16-byte block while the CPU is halted.
        self.gdma_stall_t_cycles = blocks * 8 * 4;
    }

    /// One HBlank HDMA step: copy 16 bytes. Called when PPU signals start of HBlank on LY 0–143.
    pub fn hdma_hblank_step(&mut self) {
        let Some(mut h) = self.hdma.take() else {
            return;
        };
        let src0 = h.src;
        let dst0 = h.dst;
        for i in 0..16u16 {
            let dst = dst0.wrapping_add(i);
            if !(0x8000..=0x9FFF).contains(&dst) {
                // Destination overflow: transfer stops (Pan Docs).
                return;
            }
            let b = self.read_dma_source(src0.wrapping_add(i));
            self.ppu.write_vram_dma(dst, b, h.vram_bank);
        }
        h.src = h.src.wrapping_add(16);
        h.dst = h.dst.wrapping_add(16);
        h.blocks_remaining = h.blocks_remaining.saturating_sub(1);

        self.hdma1 = (h.src >> 8) as u8;
        self.hdma2 = (h.src & 0xFF) as u8;
        self.hdma3 = (h.dst >> 8) as u8;
        self.hdma4 = (h.dst & 0xFF) as u8;

        if h.blocks_remaining > 0 {
            self.hdma = Some(h);
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.read_inner(addr)
    }

    fn read_inner(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read_rom(addr),
            0x8000..=0x9FFF => {
                if self.ppu.cpu_can_access_vram() {
                    self.ppu.read_vram(addr)
                } else {
                    0xFF
                }
            }
            0xA000..=0xBFFF => self.cartridge.read_ram(addr),
            0xC000..=0xFDFF => self.read_wram(addr),
            0xFE00..=0xFE9F => {
                if self.ppu.cpu_can_access_oam() {
                    self.ppu.read_oam(addr)
                } else {
                    0xFF
                }
            }
            0xFEA0..=0xFEFF => 0,
            0xFF00 => self.joypad.read(),
            0xFF01 => self.serial.sb,
            0xFF02 => self.serial.sc | 0x7E,
            0xFF04 => self.timer.read_div(),
            0xFF05 => self.timer.read_tima(),
            0xFF06 => self.timer.read_tma(),
            0xFF07 => self.timer.read_tac(),
            0xFF0F => self.interrupts.if_ | 0xE0,
            0xFF10..=0xFF3F => self.apu.read(addr),
            0xFF40 => self.ppu.lcdc,
            0xFF41 => self.ppu.read_stat(),
            0xFF42 => self.ppu.scy,
            0xFF43 => self.ppu.scx,
            0xFF44 => self.ppu.ly,
            0xFF45 => self.ppu.lyc,
            0xFF46 => self.ppu.dma,
            0xFF47 => self.ppu.bgp,
            0xFF48 => self.ppu.obp0,
            0xFF49 => self.ppu.obp1,
            0xFF4A => self.ppu.wy,
            0xFF4B => self.ppu.wx,
            0xFF4D => {
                if self.cgb_mode {
                    self.read_key1()
                } else {
                    0xFF
                }
            }
            0xFF4F => {
                if self.cgb_mode {
                    self.ppu.vram_bank | 0xFE
                } else {
                    0xFF
                }
            }
            0xFF50 => 0xFF,
            0xFF70 => {
                if self.cgb_mode {
                    self.svbk | 0xF8
                } else {
                    0xFF
                }
            }
            0xFF51 => {
                if self.cgb_mode {
                    self.hdma1
                } else {
                    0xFF
                }
            }
            0xFF52 => {
                if self.cgb_mode {
                    self.hdma2
                } else {
                    0xFF
                }
            }
            0xFF53 => {
                if self.cgb_mode {
                    self.hdma3
                } else {
                    0xFF
                }
            }
            0xFF54 => {
                if self.cgb_mode {
                    self.hdma4
                } else {
                    0xFF
                }
            }
            0xFF55 => {
                if self.cgb_mode {
                    match &self.hdma {
                        None => 0xFF,
                        Some(h) => {
                            let rem = h.blocks_remaining.saturating_sub(1) & 0x7F;
                            // Bit 7 clear while HDMA active; bit 7 set when idle ($FF).
                            rem
                        }
                    }
                } else {
                    0xFF
                }
            }
            0xFF68 => {
                if self.cgb_mode && self.ppu.cpu_can_access_cgb_palette() {
                    self.ppu.read_bcps()
                } else {
                    0xFF
                }
            }
            0xFF69 => {
                if self.cgb_mode && self.ppu.cpu_can_access_cgb_palette() {
                    self.ppu.read_bcpd()
                } else {
                    0xFF
                }
            }
            0xFF6A => {
                if self.cgb_mode && self.ppu.cpu_can_access_cgb_palette() {
                    self.ppu.read_ocps()
                } else {
                    0xFF
                }
            }
            0xFF6B => {
                if self.cgb_mode && self.ppu.cpu_can_access_cgb_palette() {
                    self.ppu.read_ocpd()
                } else {
                    0xFF
                }
            }
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupts.ie,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        self.write_inner(addr, v);
    }

    fn write_inner(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x7FFF => self.cartridge.write_rom(addr, v),
            0x8000..=0x9FFF => {
                if self.ppu.cpu_can_access_vram() {
                    self.ppu.write_vram(addr, v);
                }
            }
            0xA000..=0xBFFF => self.cartridge.write_ram(addr, v),
            0xC000..=0xFDFF => self.write_wram(addr, v),
            0xFE00..=0xFE9F => {
                if self.ppu.cpu_can_access_oam() {
                    self.ppu.write_oam(addr, v);
                }
            }
            0xFEA0..=0xFEFF => {}
            0xFF00 => self.joypad.write(v),
            0xFF01 => self.serial.write_sb(v),
            0xFF02 => self.serial.write_sc(v, &mut self.interrupts.if_),
            0xFF04 => self.timer.write_div(),
            0xFF05 => self.timer.write_tima(v),
            0xFF06 => self.timer.write_tma(v),
            0xFF07 => self.timer.write_tac(v),
            0xFF0F => self.interrupts.if_ = v & 0x1F,
            0xFF10..=0xFF3F => self.apu.write(addr, v),
            0xFF40 => self.ppu.lcdc = v,
            0xFF41 => self.ppu.write_stat(v),
            0xFF42 => self.ppu.scy = v,
            0xFF43 => self.ppu.scx = v,
            0xFF44 => {} // LY read only
            0xFF45 => self.ppu.lyc = v,
            0xFF46 => {
                self.ppu.dma = v;
                self.run_oam_dma(v);
            }
            0xFF47 => self.ppu.bgp = v,
            0xFF48 => self.ppu.obp0 = v,
            0xFF49 => self.ppu.obp1 = v,
            0xFF4A => self.ppu.wy = v,
            0xFF4B => self.ppu.wx = v,
            0xFF4D => {
                if self.cgb_mode {
                    self.prepare_speed_switch = (v & 0x01) != 0;
                }
            }
            0xFF4F => {
                if self.cgb_mode {
                    self.ppu.vram_bank = v & 0x01;
                }
            }
            0xFF70 => {
                if self.cgb_mode {
                    self.svbk = v & 0x07;
                }
            }
            0xFF51 => {
                if self.cgb_mode {
                    self.hdma1 = v;
                }
            }
            0xFF52 => {
                if self.cgb_mode {
                    self.hdma2 = v;
                }
            }
            0xFF53 => {
                if self.cgb_mode {
                    self.hdma3 = v;
                }
            }
            0xFF54 => {
                if self.cgb_mode {
                    self.hdma4 = v;
                }
            }
            0xFF55 => {
                if !self.cgb_mode {
                    return;
                }
                // `$00` = cancel HBlank DMA only (Pan Docs).
                if v == 0 {
                    self.hdma = None;
                    return;
                }
                // Bit 7 set: HBlank DMA — 16 bytes per visible-line HBlank (not instant full copy).
                if (v & 0x80) != 0 {
                    let src_base = (((self.hdma1 as u16) << 8) | (self.hdma2 as u16)) & 0xFFF0;
                    let dst_base =
                        0x8000u16 | ((((self.hdma3 as u16) << 8) | (self.hdma4 as u16)) & 0x1FF0);
                    let blocks = (v & 0x7F) + 1;
                    self.hdma = Some(HdmaState {
                        src: src_base,
                        dst: dst_base,
                        blocks_remaining: blocks,
                        vram_bank: self.ppu.vram_bank & 1,
                    });
                    return;
                }
                // General-purpose DMA (instant); terminates any active HBlank DMA.
                self.hdma = None;
                self.run_vram_dma(v);
            }
            0xFF68 => {
                if self.cgb_mode && self.ppu.cpu_can_access_cgb_palette() {
                    self.ppu.write_bcps(v);
                }
            }
            0xFF69 => {
                if self.cgb_mode && self.ppu.cpu_can_access_cgb_palette() {
                    self.ppu.write_bcpd(v);
                }
            }
            0xFF6A => {
                if self.cgb_mode && self.ppu.cpu_can_access_cgb_palette() {
                    self.ppu.write_ocps(v);
                }
            }
            0xFF6B => {
                if self.cgb_mode && self.ppu.cpu_can_access_cgb_palette() {
                    self.ppu.write_ocpd(v);
                }
            }
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = v,
            0xFFFF => self.interrupts.ie = v & 0x1F,
            _ => {}
        }
    }

    fn run_oam_dma(&mut self, v: u8) {
        let base = (v as u16) << 8;
        for i in 0..0xA0u16 {
            let b = self.read_dma_source(base + i);
            self.ppu.write_oam(0xFE00 + i, b);
        }
    }

    fn read_dma_source(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read_rom(addr),
            0x8000..=0x9FFF => self.ppu.read_vram(addr),
            0xA000..=0xBFFF => self.cartridge.read_ram(addr),
            0xC000..=0xFDFF => self.read_wram(addr),
            _ => 0xFF,
        }
    }
}
