use crate::interrupts;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const FRAMEBUFFER_LEN: usize = LCD_WIDTH * LCD_HEIGHT * 4;

const MODE2_END: u32 = 80;
const MODE3_END: u32 = 80 + 172;
const LINE_CYCLES: u32 = 456;
const VBLANK_START: u8 = 144;

#[derive(Debug, Clone)]
pub struct Ppu {
    pub lcdc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub dma: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub wy: u8,
    pub wx: u8,
    /// Dot counter within current scanline (0–455)
    pub dot: u32,
    pub mode: u8,
    /// CGB: 8 KiB × 2 banks (VBK @ FF4F). DMG uses bank 0 only.
    pub vram: [u8; 0x4000],
    pub vram_bank: u8,
    pub cgb_mode: bool,
    /// FF68 — BG palette index (bits 0–5), bit 7 auto-increment after BCPD access.
    pub bcps: u8,
    /// FF6A — OBJ palette index (bits 0–5), bit 7 auto-increment after OCPD access.
    pub ocps: u8,
    /// 64 bytes: 8 palettes × 4 colors × 2 bytes (RGB555 little-endian).
    pub bg_palette_ram: [u8; 64],
    pub obj_palette_ram: [u8; 64],
    pub oam: [u8; 0xA0],
    pub framebuffer: [u8; FRAMEBUFFER_LEN],
    /// Previous composite STAT interrupt line (edge detect)
    prev_stat_line: bool,
    /// If LCD was off last step (for rising edge LY reset)
    lcd_was_off: bool,
    /// HBlank starts seen this `step` on visible lines (LY 0..143): one count per scanline crossed.
    hblank_dma_edges: u32,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            lcdc: 0x91,
            stat: 0x85,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            dot: 0,
            mode: 0,
            vram: [0; 0x4000],
            vram_bank: 0,
            cgb_mode: false,
            bcps: 0,
            ocps: 0,
            bg_palette_ram: [0; 64],
            obj_palette_ram: [0; 64],
            oam: [0; 0xA0],
            framebuffer: [0; FRAMEBUFFER_LEN],
            prev_stat_line: false,
            lcd_was_off: true,
            hblank_dma_edges: 0,
        }
    }
}

impl Ppu {
    pub fn lcd_on(&self) -> bool {
        (self.lcdc & 0x80) != 0
    }

    fn vram_offset(&self, addr: u16) -> usize {
        let bank = if self.cgb_mode {
            (self.vram_bank & 1) as usize
        } else {
            0usize
        };
        let rel = ((addr.wrapping_sub(0x8000)) as usize) & 0x1FFF;
        bank * 0x2000 + rel
    }

    /// Tile VRAM offset within one 8 KiB bank (mirrors $8000–$9FFF).
    #[inline]
    fn vram_tile_rel(tile_addr: u16) -> usize {
        (tile_addr.wrapping_sub(0x8000) as usize) & 0x1FFF
    }

    /// Two consecutive tile bytes (pattern row); indices clamped so we never read past VRAM.
    #[inline]
    fn vram_tile_row_pair(&self, tile_bank: usize, tile_addr: u16) -> (u8, u8) {
        let b = tile_bank & 1;
        let base = b * 0x2000 + Self::vram_tile_rel(tile_addr);
        let max = self.vram.len() - 1;
        let i0 = base.min(max);
        let i1 = (base + 1).min(max);
        (self.vram[i0], self.vram[i1])
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[self.vram_offset(addr)]
    }

    pub fn write_vram(&mut self, addr: u16, v: u8) {
        let o = self.vram_offset(addr);
        self.vram[o] = v;
    }

    /// VRAM DMA (FF51–FF55): writes use the latched bank, not a mid-transfer VBK change.
    pub fn write_vram_dma(&mut self, addr: u16, v: u8, bank: u8) {
        let rel = ((addr.wrapping_sub(0x8000)) as usize) & 0x1FFF;
        let o = ((bank as usize) & 1) * 0x2000 + rel;
        self.vram[o] = v;
    }

    /// Pan Docs: CPU reads/writes to VRAM are ignored during Mode 3 (LCD draw).
    pub fn cpu_can_access_vram(&self) -> bool {
        !self.lcd_on() || self.mode != 3
    }

    /// OAM is only CPU-accessible during H-Blank and V-Blank.
    pub fn cpu_can_access_oam(&self) -> bool {
        !self.lcd_on() || self.mode == 0 || self.mode == 1
    }

    /// CGB palette ports (FF68–FF6B) are blocked during Mode 3.
    pub fn cpu_can_access_cgb_palette(&self) -> bool {
        !self.cgb_mode || !self.lcd_on() || self.mode != 3
    }

    pub fn read_bcps(&self) -> u8 {
        self.bcps | 0x40
    }

    pub fn write_bcps(&mut self, v: u8) {
        self.bcps = (v & 0xBF) | (v & 0x80);
    }

    pub fn read_bcpd(&mut self) -> u8 {
        let i = (self.bcps & 0x3F) as usize;
        let v = self.bg_palette_ram[i];
        if (self.bcps & 0x80) != 0 {
            let ni = ((self.bcps & 0x3F) + 1) & 0x3F;
            self.bcps = (self.bcps & 0x80) | ni;
        }
        v
    }

    pub fn write_bcpd(&mut self, v: u8) {
        let i = (self.bcps & 0x3F) as usize;
        self.bg_palette_ram[i] = v;
        if (self.bcps & 0x80) != 0 {
            let ni = ((self.bcps & 0x3F) + 1) & 0x3F;
            self.bcps = (self.bcps & 0x80) | ni;
        }
    }

    pub fn read_ocps(&self) -> u8 {
        self.ocps | 0x40
    }

    pub fn write_ocps(&mut self, v: u8) {
        self.ocps = (v & 0xBF) | (v & 0x80);
    }

    pub fn read_ocpd(&mut self) -> u8 {
        let i = (self.ocps & 0x3F) as usize;
        let v = self.obj_palette_ram[i];
        if (self.ocps & 0x80) != 0 {
            let ni = ((self.ocps & 0x3F) + 1) & 0x3F;
            self.ocps = (self.ocps & 0x80) | ni;
        }
        v
    }

    pub fn write_ocpd(&mut self, v: u8) {
        let i = (self.ocps & 0x3F) as usize;
        self.obj_palette_ram[i] = v;
        if (self.ocps & 0x80) != 0 {
            let ni = ((self.ocps & 0x3F) + 1) & 0x3F;
            self.ocps = (self.ocps & 0x80) | ni;
        }
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        self.oam[(addr - 0xFE00) as usize]
    }

    pub fn write_oam(&mut self, addr: u16, v: u8) {
        self.oam[(addr - 0xFE00) as usize] = v;
    }

    fn stat_mode_bits(&self) -> u8 {
        self.mode & 0x03
    }

    pub fn read_stat(&self) -> u8 {
        let mut s = self.stat & 0xF8;
        s |= self.stat_mode_bits();
        if self.ly == self.lyc {
            s |= 0x04;
        }
        s | 0x80
    }

    pub fn write_stat(&mut self, v: u8) {
        self.stat = (v & 0xF8) | (self.stat & 0x07);
    }

    /// Advance PPU by `t_cycles` (CPU clock / T-states).
    pub fn step(&mut self, t_cycles: u32, if_: &mut u8) {
        if !self.lcd_on() {
            if !self.lcd_was_off {
                // turning off
            }
            self.lcd_was_off = true;
            self.ly = 0;
            self.dot = 0;
            self.mode = 0;
            self.prev_stat_line = false;
            return;
        }
        self.lcd_was_off = false;

        let mut remaining = t_cycles;
        while remaining > 0 {
            let dot_before = self.dot;
            let take = remaining.min(LINE_CYCLES - self.dot);
            self.dot += take;
            remaining -= take;

            // CGB HDMA: one chunk per scanline at the start of HBlank (first dot of mode 0).
            if self.lcd_on()
                && self.ly < VBLANK_START
                && dot_before < MODE3_END
                && self.dot >= MODE3_END
            {
                self.hblank_dma_edges = self.hblank_dma_edges.saturating_add(1);
            }

            self.update_mode_for_dot();
            self.check_stat_irq(if_);

            if self.dot >= LINE_CYCLES {
                self.dot = 0;
                self.end_of_scanline(if_);
            }
        }
    }

    /// Consume number of visible-scanline HBlank entries that occurred this `step` (for HDMA chunks).
    pub fn take_hblank_dma_edges(&mut self) -> u32 {
        let n = self.hblank_dma_edges;
        self.hblank_dma_edges = 0;
        n
    }

    fn update_mode_for_dot(&mut self) {
        if self.ly >= VBLANK_START {
            self.mode = 1;
        } else if self.dot < MODE2_END {
            self.mode = 2;
        } else if self.dot < MODE3_END {
            self.mode = 3;
        } else {
            self.mode = 0;
        }
    }

    fn stat_irq_line(&self) -> bool {
        let lyc = (self.stat & 0x40) != 0 && self.ly == self.lyc;
        let m2 = (self.stat & 0x20) != 0 && self.mode == 2 && self.ly < VBLANK_START;
        let m0 = (self.stat & 0x08) != 0 && self.mode == 0 && self.ly < VBLANK_START;
        let m1 = (self.stat & 0x10) != 0 && self.mode == 1;
        lyc || m2 || m0 || m1
    }

    fn check_stat_irq(&mut self, if_: &mut u8) {
        let line = self.stat_irq_line();
        if line && !self.prev_stat_line {
            *if_ |= interrupts::LCD_STAT;
        }
        self.prev_stat_line = line;
    }

    fn end_of_scanline(&mut self, if_: &mut u8) {
        if self.ly < VBLANK_START && self.lcd_on() {
            self.render_scanline();
        }
        let prev = self.ly;
        self.ly = ((self.ly as u16 + 1) % 154) as u8;
        if self.ly == VBLANK_START && prev == VBLANK_START - 1 {
            *if_ |= interrupts::VBLANK;
        }
    }

    fn dmg_palette(&self, pal: u8, color_index: u8) -> [u8; 4] {
        let idx = (pal >> (color_index * 2)) & 0x03;
        match idx {
            0 => [0xE0, 0xF8, 0xD0, 0xFF],
            1 => [0x88, 0xC0, 0x70, 0xFF],
            2 => [0x34, 0x68, 0x56, 0xFF],
            _ => [0x08, 0x18, 0x10, 0xFF],
        }
    }

    fn rgb555(lo: u8, hi: u8) -> [u8; 4] {
        let w = u16::from_le_bytes([lo, hi]);
        let r = (w & 0x1F) as u32;
        let g = ((w >> 5) & 0x1F) as u32;
        let b = ((w >> 10) & 0x1F) as u32;
        [
            ((r * 255 + 15) / 31) as u8,
            ((g * 255 + 15) / 31) as u8,
            ((b * 255 + 15) / 31) as u8,
            0xFF,
        ]
    }

    #[inline]
    fn cgb_bg_px(&self, pal: u8, ci: u8) -> [u8; 4] {
        let i = (pal as usize & 7) * 8 + (ci as usize & 3) * 2;
        let lo = self.bg_palette_ram[i];
        let hi = self.bg_palette_ram[i + 1];
        Self::rgb555(lo, hi)
    }

    #[inline]
    fn cgb_obj_px(&self, pal: u8, ci: u8) -> [u8; 4] {
        let i = (pal as usize & 7) * 8 + (ci as usize & 3) * 2;
        let lo = self.obj_palette_ram[i];
        let hi = self.obj_palette_ram[i + 1];
        Self::rgb555(lo, hi)
    }

    fn render_scanline(&mut self) {
        let y = self.ly as usize;
        if y >= LCD_HEIGHT {
            return;
        }

        if !self.cgb_mode && (self.lcdc & 0x01) == 0 && (self.lcdc & 0x21) == 0 {
            for x in 0..LCD_WIDTH {
                let i = (y * LCD_WIDTH + x) * 4;
                self.framebuffer[i..i + 4].copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            return;
        }

        let bg_pixels_enabled = self.cgb_mode || (self.lcdc & 0x01) != 0;

        let mut line_bg: [[u8; 4]; LCD_WIDTH] = [[0xFF; 4]; LCD_WIDTH];
        let mut line_bg_idx: [u8; LCD_WIDTH] = [0; LCD_WIDTH];
        let mut line_bg_prio = [false; LCD_WIDTH];

        if bg_pixels_enabled {
            let tile_map_base = if (self.lcdc & 0x08) != 0 {
                0x1C00
            } else {
                0x1800
            };
            let unsigned = (self.lcdc & 0x10) != 0;
            let py = self.ly.wrapping_add(self.scy);
            if self.cgb_mode {
                for x in 0..LCD_WIDTH {
                    let px = (x as u8).wrapping_add(self.scx);
                    let tile_x = (px as u16 / 8) % 32;
                    let tile_y = (py as u16 / 8) % 32;
                    let fine_x = px % 8;
                    let fine_y = py % 8;
                    let map_idx = tile_y * 32 + tile_x;
                    let map_off = (tile_map_base + map_idx) as usize;
                    let tile_id = self.vram[map_off];
                    let attr = self.vram[0x2000 + map_off];
                    let tile_bank = (attr >> 3) & 1;
                    let bg_pal = attr & 0x07;
                    line_bg_prio[x] = (attr & 0x80) != 0;
                    let fy = if (attr & 0x40) != 0 {
                        7 - fine_y
                    } else {
                        fine_y
                    };
                    let fx = if (attr & 0x20) != 0 {
                        fine_x
                    } else {
                        7 - fine_x
                    };
                    let tile_addr = if unsigned {
                        0x8000u16.saturating_add(tile_id as u16 * 16 + fy as u16 * 2)
                    } else {
                        let signed_id = tile_id as i8 as i32;
                        let off = signed_id * 16 + fy as i32 * 2;
                        (0x9000i32 + off).clamp(0x8000, 0x9FFF) as u16
                    };
                    let (a, b) = self.vram_tile_row_pair(tile_bank as usize, tile_addr);
                    let c0 = ((a >> fx) & 1) | (((b >> fx) & 1) << 1);
                    line_bg[x] = self.cgb_bg_px(bg_pal, c0);
                    line_bg_idx[x] = c0;
                }
            } else {
                for x in 0..LCD_WIDTH {
                    let px = (x as u8).wrapping_add(self.scx);
                    let tile_x = (px as u16 / 8) % 32;
                    let tile_y = (py as u16 / 8) % 32;
                    let fine_x = px % 8;
                    let fine_y = py % 8;
                    let map_idx = tile_y * 32 + tile_x;
                    let tile_id = self.vram[(tile_map_base + map_idx) as usize];
                    let tile_addr = if unsigned {
                        0x8000u16.saturating_add(tile_id as u16 * 16 + fine_y as u16 * 2)
                    } else {
                        let signed_id = tile_id as i8 as i32;
                        let off = signed_id * 16 + fine_y as i32 * 2;
                        (0x9000i32 + off).clamp(0x8000, 0x9FFF) as u16
                    };
                    let (a, b) = self.vram_tile_row_pair(0, tile_addr);
                    let bit = 7 - fine_x;
                    let c0 = ((a >> bit) & 1) | (((b >> bit) & 1) << 1);
                    line_bg[x] = self.dmg_palette(self.bgp, c0);
                    line_bg_idx[x] = c0;
                }
            }
        } else {
            for x in 0..LCD_WIDTH {
                line_bg[x] = [0xFF, 0xFF, 0xFF, 0xFF];
                line_bg_idx[x] = 0;
            }
        }

        let window_line_active = (self.lcdc & 0x20) != 0
            && self.ly >= self.wy
            && (self.cgb_mode || (self.lcdc & 0x01) != 0);
        if window_line_active {
            let tile_map_base = if (self.lcdc & 0x40) != 0 {
                0x1C00
            } else {
                0x1800
            };
            let unsigned = (self.lcdc & 0x10) != 0;
            let win_y = self.ly.wrapping_sub(self.wy);
            if self.cgb_mode {
                for x in 0..LCD_WIDTH {
                    let wx0 = self.wx.saturating_sub(7);
                    if (x as u8) < wx0 {
                        continue;
                    }
                    let px = (x as u8).wrapping_sub(wx0);
                    let tile_x = (px as u16 / 8) % 32;
                    let tile_y = (win_y as u16 / 8) % 32;
                    let fine_x = px % 8;
                    let fine_y = win_y % 8;
                    let map_idx = tile_y * 32 + tile_x;
                    let map_off = (tile_map_base + map_idx) as usize;
                    let tile_id = self.vram[map_off];
                    let attr = self.vram[0x2000 + map_off];
                    let tile_bank = (attr >> 3) & 1;
                    let bg_pal = attr & 0x07;
                    line_bg_prio[x] = (attr & 0x80) != 0;
                    let fy = if (attr & 0x40) != 0 {
                        7 - fine_y
                    } else {
                        fine_y
                    };
                    let fx = if (attr & 0x20) != 0 {
                        fine_x
                    } else {
                        7 - fine_x
                    };
                    let tile_addr = if unsigned {
                        0x8000u16.saturating_add(tile_id as u16 * 16 + fy as u16 * 2)
                    } else {
                        let signed_id = tile_id as i8 as i32;
                        let off = signed_id * 16 + fy as i32 * 2;
                        (0x9000i32 + off).clamp(0x8000, 0x9FFF) as u16
                    };
                    let (a, b) = self.vram_tile_row_pair(tile_bank as usize, tile_addr);
                    let c0 = ((a >> fx) & 1) | (((b >> fx) & 1) << 1);
                    line_bg[x] = self.cgb_bg_px(bg_pal, c0);
                    line_bg_idx[x] = c0;
                }
            } else {
                for x in 0..LCD_WIDTH {
                    let wx0 = self.wx.saturating_sub(7);
                    if (x as u8) < wx0 {
                        continue;
                    }
                    let px = (x as u8).wrapping_sub(wx0);
                    let tile_x = (px as u16 / 8) % 32;
                    let tile_y = (win_y as u16 / 8) % 32;
                    let fine_x = px % 8;
                    let fine_y = win_y % 8;
                    let map_idx = tile_y * 32 + tile_x;
                    let tile_id = self.vram[(tile_map_base + map_idx) as usize];
                    let tile_addr = if unsigned {
                        0x8000u16.saturating_add(tile_id as u16 * 16 + fine_y as u16 * 2)
                    } else {
                        let signed_id = tile_id as i8 as i32;
                        let off = signed_id * 16 + fine_y as i32 * 2;
                        (0x9000i32 + off).clamp(0x8000, 0x9FFF) as u16
                    };
                    let (a, b) = self.vram_tile_row_pair(0, tile_addr);
                    let bit = 7 - fine_x;
                    let c0 = ((a >> bit) & 1) | (((b >> bit) & 1) << 1);
                    line_bg[x] = self.dmg_palette(self.bgp, c0);
                    line_bg_idx[x] = c0;
                }
            }
        }

        if (self.lcdc & 0x02) != 0 {
            let h = if (self.lcdc & 0x04) != 0 { 16u8 } else { 8u8 };
            let mut sprites: Vec<usize> = Vec::new();
            for i in (0..0xA0).step_by(4) {
                let sy = self.oam[i];
                let ly = self.ly;
                if ly + 16 < sy || ly + 16 >= sy.wrapping_add(h) {
                    continue;
                }
                sprites.push(i);
                if sprites.len() >= 10 {
                    break;
                }
            }

            for &i in sprites.iter().rev() {
                let y = self.oam[i];
                let x = self.oam[i + 1];
                let tile = self.oam[i + 2];
                let flags = self.oam[i + 3];
                let xpos = x.wrapping_sub(8) as i16;
                let x_flip = (flags & 0x20) != 0;
                let y_flip = (flags & 0x40) != 0;
                let priority = (flags & 0x80) != 0;
                let mut tile_id = tile;
                let mut py = self.ly.wrapping_add(16).wrapping_sub(y);
                if h == 16 {
                    tile_id &= 0xFE;
                    if py >= 8 {
                        tile_id = tile_id.wrapping_add(1);
                        py -= 8;
                    }
                }
                if y_flip {
                    py = h - 1 - py;
                }
                let tile_addr = 0x8000u16.saturating_add(tile_id as u16 * 16 + py as u16 * 2);

                let (tile_bank, dmg_pal) = if self.cgb_mode {
                    let bank = (flags >> 3) & 1;
                    (bank as usize, flags & 0x07)
                } else {
                    (0usize, {
                        let p = if (flags & 0x10) != 0 {
                            self.obp1
                        } else {
                            self.obp0
                        };
                        p
                    })
                };

                let (a, b) = self.vram_tile_row_pair(tile_bank, tile_addr);
                for fx in 0u8..8 {
                    let sx = xpos + fx as i16;
                    if sx < 0 || sx >= LCD_WIDTH as i16 {
                        continue;
                    }
                    let bit = if x_flip { fx } else { 7 - fx };
                    let c0 = ((a >> bit) & 1) | (((b >> bit) & 1) << 1);
                    if c0 == 0 {
                        continue;
                    }
                    let sx = sx as usize;
                    let bg_ci = line_bg_idx[sx];
                    if self.cgb_mode {
                        if (self.lcdc & 0x01) != 0 {
                            if priority && bg_ci != 0 {
                                continue;
                            }
                            if line_bg_prio[sx] && bg_ci != 0 {
                                continue;
                            }
                        }
                        line_bg[sx] = self.cgb_obj_px(dmg_pal, c0);
                    } else {
                        if priority && bg_ci != 0 {
                            continue;
                        }
                        line_bg[sx] = self.dmg_palette(dmg_pal, c0);
                    }
                }
            }
        }

        for x in 0..LCD_WIDTH {
            let idx = (y * LCD_WIDTH + x) * 4;
            self.framebuffer[idx..idx + 4].copy_from_slice(&line_bg[x]);
        }
    }
}
