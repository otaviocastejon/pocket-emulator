use std::fs;
use std::path::Path;

use crate::gameboy::GameBoy;

#[derive(Debug, Clone, Copy)]
pub struct CheatPatch {
    pub addr: u16,
    pub value: u8,
}

pub fn load_cheats(rom_path: Option<&Path>) -> Vec<CheatPatch> {
    let Some(rom_path) = rom_path else {
        return Vec::new();
    };
    let cheat_path = rom_path.with_extension("cht");
    let Ok(content) = fs::read_to_string(cheat_path) else {
        return Vec::new();
    };
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (addr_hex, value_hex) = line.split_once(':')?;
            let addr = u16::from_str_radix(addr_hex.trim(), 16).ok()?;
            let value = u8::from_str_radix(value_hex.trim(), 16).ok()?;
            Some(CheatPatch { addr, value })
        })
        .collect()
}

pub fn apply_cheats(gb: &mut GameBoy, cheats: &[CheatPatch]) {
    for cheat in cheats {
        gb.bus.write(cheat.addr, cheat.value);
    }
}
