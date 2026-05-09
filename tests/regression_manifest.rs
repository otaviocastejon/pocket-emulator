use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RegressionCase {
    rom: String,
    max_frames: u64,
    serial_contains: Option<String>,
    framebuffer_hash: Option<u64>,
}

#[test]
fn regression_manifest_cases() {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("regression_manifest.json");
    let manifest = fs::read_to_string(&manifest_path).expect("regression manifest should exist");
    let cases: Vec<RegressionCase> =
        serde_json::from_str(&manifest).expect("manifest must be valid json");
    let rom_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("roms")
        .join("regression");
    for case in cases {
        let path = rom_root.join(&case.rom);
        if !path.exists() {
            continue;
        }
        let mut gb = pocketemulator::GameBoy::from_rom_file(&path).expect("load rom");
        gb.bus.interrupts.ie = 0x1F;
        for _ in 0..case.max_frames {
            gb.run_frame();
        }
        if let Some(needle) = case.serial_contains {
            assert!(
                gb.serial_output().contains(&needle),
                "{} serial output should contain {:?}",
                case.rom,
                needle
            );
        }
        if let Some(expected) = case.framebuffer_hash {
            assert_eq!(
                framebuffer_hash(&gb.bus.ppu.framebuffer),
                expected,
                "{} framebuffer hash mismatch",
                case.rom
            );
        }
    }
}

fn framebuffer_hash(buf: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in buf {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
