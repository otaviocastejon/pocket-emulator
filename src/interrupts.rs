#[derive(Debug, Clone, Default)]
pub struct Interrupts {
    pub ie: u8,
    pub if_: u8,
}

pub const VBLANK: u8 = 1 << 0;
pub const LCD_STAT: u8 = 1 << 1;
pub const TIMER: u8 = 1 << 2;
pub const SERIAL: u8 = 1 << 3;
pub const JOYPAD: u8 = 1 << 4;

impl Interrupts {
    pub fn request(&mut self, bit: u8) {
        self.if_ |= bit;
    }

    pub fn clear(&mut self, bit: u8) {
        self.if_ &= !bit;
    }
}
