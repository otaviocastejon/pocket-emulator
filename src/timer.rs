/// DIV / TIMA / TMA / TAC
#[derive(Debug, Clone)]
pub struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    /// Internal counter for TIMA (hidden hardware)
    tima_counter: i32,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            div: 0xABCC,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_counter: 0,
        }
    }
}

impl Timer {
    /// Advance by `t_cycles` (CPU clock cycles, multiple of 4 typical).
    pub fn step(&mut self, t_cycles: u32, if_: &mut u8) {
        let old_div = self.div;
        self.div = self.div.wrapping_add(t_cycles as u16);

        // DIV increment bit: bit 13 of internal counter for 16384 Hz when tac off?
        // Hardware: DIV is upper byte of 16-bit counter that increments every t-cycle.
        // So DIV register (FF04) is bits 8-15 of div counter, increments when bit 8-15 change?
        // Actually: DIV maps to high byte of sys counter. Each t-cycle increments internal 16-bit.
        // For emulator: div is 16-bit counter; read returns (div >> 8) as u8.

        if !self.tac_enabled() {
            return;
        }

        let freq = self.tima_frequency_t_cycles();
        if freq == 0 {
            return;
        }

        self.tima_counter += t_cycles as i32;
        while self.tima_counter >= freq {
            self.tima_counter -= freq;
            let (new, ov) = self.tima.overflowing_add(1);
            if ov {
                self.tima = self.tma;
                *if_ |= 0x04; // timer interrupt
            } else {
                self.tima = new;
            }
        }

        // Suppress spurious: only process tima when tac enabled - done above
        let _ = old_div;
    }

    fn tac_enabled(&self) -> bool {
        (self.tac & 0x04) != 0
    }

    /// T-cycles between TIMA increments for current TAC low 2 bits.
    fn tima_frequency_t_cycles(&self) -> i32 {
        match self.tac & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => 1024,
        }
    }

    pub fn read_div(&self) -> u8 {
        (self.div >> 8) as u8
    }

    pub fn write_div(&mut self) {
        self.div = 0;
        self.tima_counter = 0;
    }

    pub fn read_tima(&self) -> u8 {
        self.tima
    }

    pub fn write_tima(&mut self, v: u8) {
        self.tima = v;
    }

    pub fn read_tma(&self) -> u8 {
        self.tma
    }

    pub fn write_tma(&mut self, v: u8) {
        self.tma = v;
    }

    pub fn read_tac(&self) -> u8 {
        self.tac | 0xF8
    }

    pub fn write_tac(&mut self, v: u8) {
        let old_enable = self.tac_enabled();
        self.tac = v;
        if old_enable != self.tac_enabled() {
            self.tima_counter = 0;
        }
    }
}
