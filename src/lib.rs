//! DMG Game Boy emulator core.

pub mod apu;
pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod frontend;
pub mod gameboy;
pub mod interrupts;
pub mod joypad;
pub mod platform;
pub mod ppu;
pub mod regression;
pub mod runtime_env;
pub mod serial;
pub mod storage;
pub mod timer;
pub mod ui_icon;

pub use bus::Bus;
pub use cartridge::{from_rom, Cartridge, CartridgeError, CgbSupport, Header};
pub use gameboy::GameBoy;
pub use platform::{classify_rom, rom_launch_check, LaunchCompatibility, RomTarget};
