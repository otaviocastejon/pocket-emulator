mod cb;
mod helpers;
mod opcodes_gen;
pub mod registers;

pub use registers::Registers;

use crate::bus::Bus;

pub struct Cpu {
    pub regs: Registers,
    pub ime: bool,
    /// `EI` arms IME to turn on after the **next** full instruction completes.
    pub ei_latch: bool,
    pub halted: bool,
}

impl Cpu {
    /// Registers after boot ROM — matches Nintendo hardware when skipping boot ROM.
    /// **DMG:** `A = 0x01`. **CGB:** `A = 0x11` so games (e.g. GBC-only carts) can detect Color hardware.
    pub fn new_post_boot(cgb_mode: bool) -> Self {
        let mut regs = Registers::default();
        let af = if cgb_mode { 0x11B0 } else { 0x01B0 };
        regs.set_af(af);
        regs.set_bc(0x0013);
        regs.set_de(0x00D8);
        regs.set_hl(0x014D);
        regs.sp = 0xFFFE;
        regs.pc = 0x0100;
        Self {
            regs,
            ime: false,
            ei_latch: false,
            halted: false,
        }
    }

    /// One CPU instruction or interrupt dispatch. Returns **machine cycles**.
    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        if let Some(m) = Self::try_interrupt(self, bus) {
            return m;
        }

        if self.halted {
            if bus.interrupts.ie & bus.interrupts.if_ & 0x1F != 0 {
                self.halted = false;
            } else {
                return 1;
            }
        }

        let op = helpers::fetch_u8(self, bus);
        if op == 0xCB {
            let cb = helpers::fetch_u8(self, bus);
            let extra = cb::execute_cb(self, bus, cb);
            2 + extra
        } else {
            let extra = opcodes_gen::execute(self, bus, op);
            if self.ei_latch {
                self.ime = true;
                self.ei_latch = false;
            } else if op == 0xFB {
                self.ei_latch = true;
            }
            1 + extra
        }
    }

    fn try_interrupt(&mut self, bus: &mut Bus) -> Option<u8> {
        if !self.ime {
            return None;
        }
        let pending = bus.interrupts.ie & bus.interrupts.if_ & 0x1F;
        if pending == 0 {
            return None;
        }

        self.ime = false;
        self.halted = false;
        self.ei_latch = false;

        let (vec, bit) = if pending & 0x01 != 0 {
            (0x40u16, 0x01u8)
        } else if pending & 0x02 != 0 {
            (0x48, 0x02)
        } else if pending & 0x04 != 0 {
            (0x50, 0x04)
        } else if pending & 0x08 != 0 {
            (0x58, 0x08)
        } else if pending & 0x10 != 0 {
            (0x60, 0x10)
        } else {
            return None;
        };

        bus.interrupts.if_ &= !bit;
        let pc = self.regs.pc;
        helpers::push_u16(self, bus, pc);
        self.regs.pc = vec;
        Some(5)
    }
}
