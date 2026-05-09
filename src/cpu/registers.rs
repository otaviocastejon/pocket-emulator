#[derive(Debug, Clone, Default)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f & 0xF0])
    }

    pub fn set_af(&mut self, v: u16) {
        let [a, f] = v.to_be_bytes();
        self.a = a;
        self.f = f & 0xF0;
    }

    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn set_bc(&mut self, v: u16) {
        let [b, c] = v.to_be_bytes();
        self.b = b;
        self.c = c;
    }

    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn set_de(&mut self, v: u16) {
        let [d, e] = v.to_be_bytes();
        self.d = d;
        self.e = e;
    }

    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn set_hl(&mut self, v: u16) {
        let [h, l] = v.to_be_bytes();
        self.h = h;
        self.l = l;
    }

    pub fn flag_z(&self) -> bool {
        (self.f & 0x80) != 0
    }

    pub fn flag_n(&self) -> bool {
        (self.f & 0x40) != 0
    }

    pub fn flag_h(&self) -> bool {
        (self.f & 0x20) != 0
    }

    pub fn flag_c(&self) -> bool {
        (self.f & 0x10) != 0
    }

    pub fn set_z(&mut self, v: bool) {
        if v {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
    }

    pub fn set_n(&mut self, v: bool) {
        if v {
            self.f |= 0x40;
        } else {
            self.f &= !0x40;
        }
    }

    pub fn set_h(&mut self, v: bool) {
        if v {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
    }

    pub fn set_c(&mut self, v: bool) {
        if v {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }
    }
}
