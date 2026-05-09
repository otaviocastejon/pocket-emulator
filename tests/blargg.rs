//! Integration tests when ROMs are present under `roms/tests/` (gitignored).

use std::path::PathBuf;

fn rom(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("roms")
        .join("tests")
        .join(name)
}

fn run_until_serial_substring(rom_name: &str, needle: &str, max_frames: u64) -> bool {
    let path = rom(rom_name);
    if !path.exists() {
        return false;
    }
    let mut gb = pocketemulator::GameBoy::from_rom_file(&path).expect("load rom");
    gb.bus.interrupts.ie = 0x1F;
    for _ in 0..max_frames {
        gb.run_frame();
        if gb.serial_output().contains(needle) {
            return true;
        }
    }
    false
}

#[test]
fn blargg_cpu_instrs() {
    let p = rom("cpu_instrs.gb");
    if !p.exists() {
        return;
    }
    assert!(
        run_until_serial_substring("cpu_instrs.gb", "Passed", 500_000),
        "cpu_instrs.gb should print Passed"
    );
}

#[test]
fn blargg_instr_timing() {
    let p = rom("instr_timing.gb");
    if !p.exists() {
        return;
    }
    assert!(
        run_until_serial_substring("instr_timing.gb", "Passed", 500_000),
        "instr_timing.gb should print Passed"
    );
}

#[test]
fn dmg_acid2_runs() {
    let p = rom("dmg-acid2.gb");
    if !p.exists() {
        return;
    }
    let mut gb = pocketemulator::GameBoy::from_rom_file(&p).expect("load rom");
    gb.bus.interrupts.ie = 0x1F;
    for _ in 0..300 {
        gb.run_frame();
    }
    assert!(gb.bus.ppu.framebuffer.iter().any(|&b| b != 0));
}
