use crate::interrupts;

#[derive(Debug, Clone)]
pub struct Joypad {
    /// P1/JOYP — bits 4-5 select button/direction rows
    pub p1: u8,
    /// Low active: bit set = not pressed
    pub buttons: u8,
    pub directions: u8,
}

impl Default for Joypad {
    fn default() -> Self {
        Self {
            p1: 0xCF,
            buttons: 0x0F,
            directions: 0x0F,
        }
    }
}

impl Joypad {
    pub fn read(&self) -> u8 {
        // Bits 6-7 always read as 1, bits 0-3 default to 1 (not pressed).
        // Bits 4-5 are the selection lines written by the CPU.
        let mut v = 0xCF | (self.p1 & 0x30);
        if (self.p1 & 0x10) == 0 {
            v &= 0xF0 | (self.directions & 0x0F);
        }
        if (self.p1 & 0x20) == 0 {
            v &= 0xF0 | (self.buttons & 0x0F);
        }
        v
    }

    pub fn write(&mut self, v: u8) {
        // Only bits 4-5 are writable on JOYP.
        self.p1 = v & 0x30;
    }

    pub fn set_button_down(&mut self, mask: u8, if_: &mut u8) {
        let old = self.buttons;
        self.buttons &= !mask;
        if old != self.buttons {
            *if_ |= interrupts::JOYPAD;
        }
    }

    pub fn set_button_up(&mut self, mask: u8) {
        self.buttons |= mask;
    }

    pub fn set_direction_down(&mut self, mask: u8, if_: &mut u8) {
        let old = self.directions;
        self.directions &= !mask;
        if old != self.directions {
            *if_ |= interrupts::JOYPAD;
        }
    }

    pub fn set_direction_up(&mut self, mask: u8) {
        self.directions |= mask;
    }
}

pub const BTN_A: u8 = 1 << 0;
pub const BTN_B: u8 = 1 << 1;
pub const BTN_SELECT: u8 = 1 << 2;
pub const BTN_START: u8 = 1 << 3;

pub const DIR_RIGHT: u8 = 1 << 0;
pub const DIR_LEFT: u8 = 1 << 1;
pub const DIR_UP: u8 = 1 << 2;
pub const DIR_DOWN: u8 = 1 << 3;
