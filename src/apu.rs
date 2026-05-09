//! DMG APU: four channels, frame sequencer, stereo mix. Advances on every T-state (`step`).

/// CPU T-states per second (4.194304 MHz).
pub const T_CYCLES_PER_SEC: u64 = 4_194_304;
/// Frame sequencer ticks at 512 Hz → once every 8192 T-cycles.
const FS_PERIOD: u64 = T_CYCLES_PER_SEC / 512;
/// Default output sample rate before matching the audio device.
pub const OUTPUT_SAMPLE_RATE: u32 = 48_000;

/// Duty cycle patterns (bit 7 = first step).
const DUTY: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(Debug, Clone)]
pub struct Apu {
    enabled: bool,
    /// NR50
    nr50: u8,
    /// NR51 panning
    nr51: u8,
    /// NR52 (only bit 7 stored; low bits computed)
    nr52: u8,

    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,

    wave_ram: [u8; 16],

    fs_cycles: u64,
    fs_step: u8,

    /// Playback sample rate (Hz), must match the output device.
    playback_hz: u32,
    /// Fractional sample timing: `accum += t * playback_hz`; emit when `>= T_CYCLES_PER_SEC`.
    sample_accum: u64,
    pending: Vec<f32>,

    /// Raw register shadow for reads / power-off masking
    regs: [u8; 0x30],
}

impl Default for Apu {
    fn default() -> Self {
        Self {
            enabled: false,
            nr50: 0,
            nr51: 0,
            nr52: 0,
            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
            ch3: WaveChannel::default(),
            ch4: NoiseChannel::default(),
            wave_ram: [0; 16],
            fs_cycles: 0,
            fs_step: 0,
            playback_hz: OUTPUT_SAMPLE_RATE,
            sample_accum: 0,
            pending: Vec::with_capacity(4096),
            regs: [0; 0x30],
        }
    }
}

impl Apu {
    /// Advance audio circuitry by `t_cycles` T-states (same clock as timer/PPU).
    pub fn step(&mut self, t_cycles: u32) {
        let t = t_cycles as u64;
        let hz = self.playback_hz as u64;
        if !self.enabled {
            self.sample_accum = self.sample_accum.saturating_add(t.saturating_mul(hz));
            while self.sample_accum >= T_CYCLES_PER_SEC {
                self.sample_accum -= T_CYCLES_PER_SEC;
                self.pending.push(0.0);
                self.pending.push(0.0);
            }
            return;
        }

        self.fs_cycles += t;
        while self.fs_cycles >= FS_PERIOD {
            self.fs_cycles -= FS_PERIOD;
            self.tick_frame_sequencer();
        }

        self.ch1.step_timer(t);
        self.ch2.step_timer(t);
        self.ch3.step_timer(t);
        self.ch4.step_timer(t);

        self.sample_accum = self.sample_accum.saturating_add(t.saturating_mul(hz));
        while self.sample_accum >= T_CYCLES_PER_SEC {
            self.sample_accum -= T_CYCLES_PER_SEC;
            let (l, r) = self.mix();
            self.pending.push(l);
            self.pending.push(r);
        }
    }

    /// Match emulation audio pacing to the opened output device (Hz).
    pub fn set_playback_sample_rate(&mut self, hz: u32) {
        let hz = hz.max(4000);
        // Preserve fractional phase when possible so rate tweaks don't burst samples.
        let old = self.playback_hz as u64;
        let new = hz as u64;
        if old != 0 && old != new {
            self.sample_accum = self.sample_accum.saturating_mul(new).saturating_div(old);
        }
        self.playback_hz = hz;
    }

    /// Drain synthesized stereo samples (interleaved L,R,…). Call once per frame (or whenever).
    pub fn take_pending_samples(&mut self) -> Vec<f32> {
        std::mem::take(&mut self.pending)
    }

    fn tick_frame_sequencer(&mut self) {
        let s = self.fs_step;
        self.fs_step = (self.fs_step + 1) & 7;

        let length = s == 0 || s == 2 || s == 4 || s == 6;
        let sweep = s == 2 || s == 6;
        let env = s == 7;

        if length {
            self.ch1.clock_length();
            self.ch2.clock_length();
            self.ch3.clock_length();
            self.ch4.clock_length();
        }
        if sweep {
            self.ch1.clock_sweep();
        }
        if env {
            self.ch1.clock_envelope();
            self.ch2.clock_envelope();
            self.ch4.clock_envelope();
        }
    }

    fn mix(&mut self) -> (f32, f32) {
        let s1 = self.ch1.output();
        let s2 = self.ch2.output();
        let s3 = self.ch3.output(&self.wave_ram);
        let s4 = self.ch4.output();

        let mut left = 0.0f32;
        let mut right = 0.0f32;
        let pan = self.nr51;

        if pan & 0x10 != 0 {
            left += s1;
        }
        if pan & 0x20 != 0 {
            left += s2;
        }
        if pan & 0x40 != 0 {
            left += s3;
        }
        if pan & 0x80 != 0 {
            left += s4;
        }

        if pan & 0x01 != 0 {
            right += s1;
        }
        if pan & 0x02 != 0 {
            right += s2;
        }
        if pan & 0x04 != 0 {
            right += s3;
        }
        if pan & 0x08 != 0 {
            right += s4;
        }

        let vol_l = ((self.nr50 >> 4) & 7) as f32 + 1.0;
        let vol_r = (self.nr50 & 7) as f32 + 1.0;

        left *= vol_l / 8.0;
        right *= vol_r / 8.0;

        // Normalize ~4 channels max 0.25 each → ~1.0 peak; soft clip
        const GAIN: f32 = 0.35;
        (tanh_soft(left * GAIN), tanh_soft(right * GAIN))
    }

    pub fn read(&self, addr: u16) -> u8 {
        let i = (addr - 0xFF10) as usize;
        if addr >= 0xFF30 && addr <= 0xFF3F {
            let o = (addr - 0xFF30) as usize;
            if !self.enabled {
                return 0xFF;
            }
            return self.wave_ram[o];
        }
        if !self.enabled {
            return match addr {
                0xFF26 => self.nr52 & 0x70,
                _ => 0xFF,
            };
        }

        match addr {
            0xFF10 => self.ch1.sweep_read() | 0x80,
            0xFF11 => self.ch1.len_duty_read(),
            0xFF12 => self.ch1.env.read_reg(),
            0xFF13 => 0xFF,
            0xFF14 => self.ch1.control_read(),
            0xFF16 => self.ch2.len_duty_read(),
            0xFF17 => self.ch2.env.read_reg(),
            0xFF18 => 0xFF,
            0xFF19 => self.ch2.control_read(),
            0xFF1A => self.ch3.read_dac(),
            0xFF1B => self.ch3.read_len(),
            0xFF1C => self.ch3.read_vol(),
            0xFF1D => 0xFF,
            0xFF1E => self.ch3.read_ctrl(),
            0xFF20 => self.ch4.read_len(),
            0xFF21 => self.ch4.read_env(),
            0xFF22 => self.ch4.read_poly(),
            0xFF23 => self.ch4.read_ctrl(),
            0xFF24 => self.nr50 | 0x88,
            0xFF25 => self.nr51,
            0xFF26 => {
                self.nr52 & 0xF0
                    | (if self.ch1.on() { 1 } else { 0 })
                    | (if self.ch2.on() { 2 } else { 0 })
                    | (if self.ch3.on() { 4 } else { 0 })
                    | (if self.ch4.on() { 8 } else { 0 })
            }
            _ => self.regs.get(i).copied().unwrap_or(0xFF),
        }
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        let i = (addr - 0xFF10) as usize;
        if let Some(slot) = self.regs.get_mut(i) {
            *slot = v;
        }

        if addr >= 0xFF30 && addr <= 0xFF3F {
            let o = (addr - 0xFF30) as usize;
            self.wave_ram[o] = v;
            return;
        }

        if !self.enabled && addr != 0xFF26 {
            return;
        }

        match addr {
            0xFF12 => self.ch1.env.write_reg(v),
            0xFF17 => self.ch2.env.write_reg(v),
            0xFF21 => self.ch4.write_env(v),
            _ => {}
        }

        match addr {
            0xFF10 => self.ch1.write_sweep(v),
            0xFF11 => self.ch1.write_len_duty(v),
            0xFF12 => {}
            0xFF13 => self.ch1.write_freq_low(v),
            0xFF14 => self.ch1.write_freq_high(v, &mut self.enabled),
            0xFF16 => self.ch2.write_len_duty(v),
            0xFF17 => {}
            0xFF18 => self.ch2.write_freq_low(v),
            0xFF19 => self.ch2.write_freq_high(v, &mut self.enabled),
            0xFF1A => self.ch3.write_dac(v),
            0xFF1B => self.ch3.write_len(v),
            0xFF1C => self.ch3.write_vol_code(v),
            0xFF1D => self.ch3.write_freq_low(v),
            0xFF1E => self.ch3.write_freq_high(v, &mut self.enabled),
            0xFF20 => self.ch4.write_len(v),
            0xFF22 => self.ch4.write_poly(v),
            0xFF23 => self.ch4.write_ctrl(v, &mut self.enabled),
            0xFF24 => self.nr50 = v,
            0xFF25 => self.nr51 = v,
            0xFF26 => {
                let on = v & 0x80 != 0;
                if on && !self.enabled {
                    self.fs_cycles = 0;
                    self.fs_step = 0;
                }
                if !on && self.enabled {
                    self.clear_all_regs();
                }
                self.enabled = on;
                self.nr52 = (self.nr52 & 0xF0) | (v & 0x80);
            }
            _ => {}
        }
    }

    fn clear_all_regs(&mut self) {
        self.nr50 = 0;
        self.nr51 = 0;
        self.ch1 = SquareChannel::new(true);
        self.ch2 = SquareChannel::new(false);
        self.ch3 = WaveChannel::default();
        self.ch4 = NoiseChannel::default();
        self.wave_ram.fill(0);
        self.regs.fill(0);
    }
}

fn tanh_soft(x: f32) -> f32 {
    if x > 1.0 {
        1.0
    } else if x < -1.0 {
        -1.0
    } else {
        x
    }
}

#[derive(Debug, Clone)]
struct SquareChannel {
    has_sweep: bool,
    sweep: u8,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,
    sweep_timer: u8,
    sweep_shadow: u16,
    sweep_enabled: bool,

    duty_code: usize,
    duty_step: u8,

    length: u8,
    length_enable: bool,

    env: Envelope,

    freq: u16,
    timer: u32,

    triggered: bool,
}

impl SquareChannel {
    fn new(has_sweep: bool) -> Self {
        Self {
            has_sweep,
            sweep: 0,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            sweep_timer: 0,
            sweep_shadow: 0,
            sweep_enabled: false,
            duty_code: 0,
            duty_step: 0,
            length: 0,
            length_enable: false,
            env: Envelope::default(),
            freq: 0,
            timer: 0,
            triggered: false,
        }
    }

    fn on(&self) -> bool {
        self.triggered
    }

    fn len_duty_read(&self) -> u8 {
        (self.duty_code as u8) << 6 | self.length
    }

    fn control_read(&self) -> u8 {
        let mut r = 0xBF;
        if self.length_enable {
            r |= 0x40;
        }
        r
    }

    fn sweep_read(&self) -> u8 {
        self.sweep
    }

    fn write_sweep(&mut self, v: u8) {
        self.sweep = v;
        self.sweep_period = (v >> 4) & 7;
        self.sweep_negate = v & 0x08 != 0;
        self.sweep_shift = v & 7;
        self.sweep_enabled = self.has_sweep && self.sweep_shift != 0 && self.sweep_period != 0;
    }

    fn write_len_duty(&mut self, v: u8) {
        self.duty_code = ((v >> 6) & 3) as usize;
        self.length = 64 - (v & 0x3F);
    }

    fn write_freq_low(&mut self, v: u8) {
        self.freq = (self.freq & 0x700) | v as u16;
    }

    fn write_freq_high(&mut self, v: u8, master_on: &mut bool) {
        self.freq = (self.freq & 0xFF) | (((v & 7) as u16) << 8);
        self.length_enable = v & 0x40 != 0;
        if v & 0x80 != 0 {
            self.trigger(master_on);
        }
    }

    fn trigger(&mut self, _master_on: &mut bool) {
        self.triggered = true;
        self.env.reload();
        self.timer = period_from_freq(self.freq);
        self.duty_step = 0;
        if self.length == 0 {
            self.length = 64;
        }
        if self.has_sweep {
            self.sweep_shadow = self.freq;
            self.sweep_timer = if self.sweep_period != 0 {
                self.sweep_period
            } else {
                8
            };
            if self.sweep_shift != 0
                && sweep_calc(self.freq, self.sweep_shift, self.sweep_negate) > 2047
            {
                self.triggered = false;
            }
        }
    }

    fn step_timer(&mut self, t: u64) {
        if !self.triggered {
            return;
        }
        let mut left = t;
        while left > 0 {
            let period = self.timer.max(1);
            let step = left.min(period as u64);
            self.timer -= step as u32;
            left -= step;
            if self.timer == 0 {
                self.duty_step = (self.duty_step + 1) & 7;
                self.timer = period_from_freq(self.freq).max(4);
            }
        }
    }

    fn clock_length(&mut self) {
        if self.length_enable && self.length > 0 {
            self.length -= 1;
            if self.length == 0 {
                self.triggered = false;
            }
        }
    }

    fn clock_sweep(&mut self) {
        if !self.has_sweep || !self.sweep_enabled || !self.triggered {
            return;
        }
        let dec = self.sweep_period != 0;
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }
        if self.sweep_timer == 0 {
            self.sweep_timer = if self.sweep_period != 0 {
                self.sweep_period
            } else {
                8
            };
            if dec && self.sweep_shift != 0 {
                let new_freq = sweep_calc(self.sweep_shadow, self.sweep_shift, self.sweep_negate);
                if new_freq > 2047 {
                    self.triggered = false;
                } else {
                    self.freq = new_freq;
                    self.sweep_shadow = new_freq;
                }
            }
        }
    }

    fn clock_envelope(&mut self) {
        self.env.clock();
    }

    fn output(&self) -> f32 {
        if !self.triggered {
            return 0.0;
        }
        let amp = self.env.current_volume() as f32 / 15.0;
        let bit = DUTY[self.duty_code][self.duty_step as usize];
        if bit == 0 {
            0.0
        } else {
            amp * 0.25
        }
    }
}

fn period_from_freq(freq: u16) -> u32 {
    let f = freq & 0x7FF;
    ((2048u32 - f as u32) * 4).max(4)
}

fn sweep_calc(shadow: u16, shift: u8, negate: bool) -> u16 {
    let delta = shadow >> shift;
    if negate {
        shadow.wrapping_sub(delta)
    } else {
        shadow.wrapping_add(delta)
    }
}

#[derive(Debug, Clone, Default)]
struct Envelope {
    initial_volume: u8,
    direction_up: bool,
    period: u8,
    volume: u8,
    timer: u8,
}

impl Envelope {
    fn read_reg(&self) -> u8 {
        self.initial_volume << 4 | if self.direction_up { 0x08 } else { 0 } | self.period
    }

    fn write_reg(&mut self, v: u8) {
        self.initial_volume = v >> 4;
        self.direction_up = v & 0x08 != 0;
        self.period = v & 7;
    }

    fn reload(&mut self) {
        self.volume = self.initial_volume;
        self.timer = if self.period != 0 { self.period } else { 8 };
    }

    fn clock(&mut self) {
        if self.period == 0 {
            return;
        }
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.timer = self.period;
            if self.direction_up && self.volume < 15 {
                self.volume += 1;
            } else if !self.direction_up && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

    fn current_volume(&self) -> u8 {
        self.volume
    }
}

#[derive(Debug, Clone)]
struct WaveChannel {
    len: u16,
    len_enable: bool,
    dac_on: bool,
    vol_code: u8,
    freq: u16,
    timer: u32,
    pos: usize,
    triggered: bool,
}

impl Default for WaveChannel {
    fn default() -> Self {
        Self {
            len: 0,
            len_enable: false,
            dac_on: false,
            vol_code: 0,
            freq: 0,
            timer: 0,
            pos: 0,
            triggered: false,
        }
    }
}

impl WaveChannel {
    fn on(&self) -> bool {
        self.triggered && self.dac_on
    }

    fn read_dac(&self) -> u8 {
        if self.dac_on {
            0x80 | 0x7F
        } else {
            0x7F
        }
    }

    fn read_len(&self) -> u8 {
        self.len as u8
    }

    fn read_vol(&self) -> u8 {
        self.vol_code << 5
    }

    fn read_ctrl(&self) -> u8 {
        let mut r = 0xBF;
        if self.len_enable {
            r |= 0x40;
        }
        r
    }

    fn write_dac(&mut self, v: u8) {
        self.dac_on = v & 0x80 != 0;
        if !self.dac_on {
            self.triggered = false;
        }
    }

    fn write_len(&mut self, v: u8) {
        self.len = 256 - v as u16;
    }

    fn write_vol_code(&mut self, v: u8) {
        self.vol_code = (v >> 5) & 3;
    }

    fn write_freq_low(&mut self, v: u8) {
        self.freq = (self.freq & 0x700) | v as u16;
    }

    fn write_freq_high(&mut self, v: u8, _master: &mut bool) {
        self.freq = (self.freq & 0xFF) | (((v & 7) as u16) << 8);
        self.len_enable = v & 0x40 != 0;
        if v & 0x80 != 0 {
            self.trigger();
        }
    }

    fn trigger(&mut self) {
        if !self.dac_on {
            return;
        }
        self.triggered = true;
        self.timer = wave_period(self.freq);
        if self.len == 0 {
            self.len = 256;
        }
        self.pos = 0;
    }

    fn step_timer(&mut self, t: u64) {
        if !self.triggered || !self.dac_on {
            return;
        }
        let mut left = t;
        while left > 0 {
            if self.timer == 0 {
                self.timer = wave_period(self.freq);
            }
            let step = left.min(self.timer as u64);
            self.timer -= step as u32;
            left -= step;
            if self.timer == 0 {
                self.pos = (self.pos + 1) & 31;
                self.timer = wave_period(self.freq);
            }
        }
    }

    fn clock_length(&mut self) {
        if self.len_enable && self.len > 0 {
            self.len -= 1;
            if self.len == 0 {
                self.triggered = false;
            }
        }
    }

    fn output(&self, wave_ram: &[u8; 16]) -> f32 {
        if !self.on() {
            return 0.0;
        }
        let b = wave_ram[self.pos / 2];
        let nibble = if self.pos & 1 == 0 { b >> 4 } else { b & 0xF };
        let amp = match self.vol_code {
            0 => 0,
            1 => nibble,
            2 => nibble >> 1,
            _ => nibble >> 2,
        };
        amp as f32 / 16.0 * 0.25
    }
}

fn wave_period(freq: u16) -> u32 {
    let f = freq & 0x7FF;
    ((2048u32 - f as u32) * 2).max(2)
}

#[derive(Debug, Clone)]
struct NoiseChannel {
    len: u8,
    len_enable: bool,
    env: Envelope,
    clock_shift: u8,
    width_mode: bool,
    divisor_code: u8,
    timer: u32,
    lfsr: u16,
    triggered: bool,
}

impl Default for NoiseChannel {
    fn default() -> Self {
        Self {
            len: 0,
            len_enable: false,
            env: Envelope::default(),
            clock_shift: 0,
            width_mode: false,
            divisor_code: 0,
            timer: 0,
            lfsr: 0x7FFF,
            triggered: false,
        }
    }
}

impl NoiseChannel {
    fn on(&self) -> bool {
        self.triggered
    }

    fn read_len(&self) -> u8 {
        self.len
    }

    fn read_env(&self) -> u8 {
        self.env.read_reg()
    }

    fn read_poly(&self) -> u8 {
        self.clock_shift << 4 | self.divisor_code | if self.width_mode { 0x08 } else { 0 }
    }

    fn read_ctrl(&self) -> u8 {
        let mut r = 0xBF;
        if self.len_enable {
            r |= 0x40;
        }
        r
    }

    fn write_len(&mut self, v: u8) {
        self.len = 64 - (v & 0x3F);
    }

    fn write_env(&mut self, v: u8) {
        self.env.write_reg(v);
    }

    fn write_poly(&mut self, v: u8) {
        self.clock_shift = v >> 4;
        self.width_mode = v & 0x08 != 0;
        self.divisor_code = v & 7;
    }

    fn write_ctrl(&mut self, v: u8, _master: &mut bool) {
        self.len_enable = v & 0x40 != 0;
        if v & 0x80 != 0 {
            self.trigger();
        }
    }

    fn trigger(&mut self) {
        self.triggered = true;
        self.env.reload();
        self.timer = noise_period(self.divisor_code, self.clock_shift);
        self.lfsr = if self.width_mode { 0x7F } else { 0x7FFF };
        if self.len == 0 {
            self.len = 64;
        }
    }

    fn step_timer(&mut self, t: u64) {
        if !self.triggered {
            return;
        }
        let mut left = t;
        while left > 0 {
            let period = self.timer.max(1);
            let step = left.min(period as u64);
            self.timer -= step as u32;
            left -= step;
            if self.timer == 0 {
                let xor_bit = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
                self.lfsr >>= 1;
                if xor_bit != 0 {
                    self.lfsr |= 0x4000;
                }
                if self.width_mode {
                    self.lfsr &= 0x7F;
                }
                self.timer = noise_period(self.divisor_code, self.clock_shift).max(2);
            }
        }
    }

    fn clock_length(&mut self) {
        if self.len_enable && self.len > 0 {
            self.len -= 1;
            if self.len == 0 {
                self.triggered = false;
            }
        }
    }

    fn clock_envelope(&mut self) {
        self.env.clock();
    }

    fn output(&self) -> f32 {
        if !self.triggered {
            return 0.0;
        }
        let amp = self.env.current_volume() as f32 / 15.0;
        if (self.lfsr & 1) != 0 {
            0.0
        } else {
            amp * 0.25
        }
    }
}

fn noise_period(divisor_code: u8, shift: u8) -> u32 {
    let base = match divisor_code & 7 {
        0 => 8u32,
        1 => 16,
        2 => 32,
        3 => 48,
        4 => 64,
        5 => 80,
        6 => 96,
        _ => 112,
    };
    base << shift as u32
}
