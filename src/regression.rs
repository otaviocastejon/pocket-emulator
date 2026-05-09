use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::GameBoy;

#[derive(Debug, Deserialize)]
struct RegressionCase {
    rom: String,
    max_frames: u64,
    serial_contains: Option<String>,
    framebuffer_hash: Option<u64>,
}

pub fn run_regression_suite(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = PathBuf::from("tests/regression_manifest.json");
    let manifest_str = fs::read_to_string(&manifest_path)?;
    let cases: Vec<RegressionCase> = serde_json::from_str(&manifest_str)?;
    if cases.is_empty() {
        println!("No regression cases defined in {}", manifest_path.display());
        return Ok(());
    }

    let mut failed = 0usize;
    for case in cases {
        let rom_path = dir.join(&case.rom);
        if !rom_path.exists() {
            println!("SKIP {} (missing)", case.rom);
            continue;
        }
        let mut gb = GameBoy::from_rom_file(&rom_path)?;
        gb.bus.interrupts.ie = 0x1F;
        for _ in 0..case.max_frames {
            gb.run_frame();
        }
        let serial_ok = case
            .serial_contains
            .as_ref()
            .map(|needle| gb.serial_output().contains(needle))
            .unwrap_or(true);
        let hash = framebuffer_hash(&gb.bus.ppu.framebuffer);
        let frame_ok = case.framebuffer_hash.map(|v| v == hash).unwrap_or(true);
        if serial_ok && frame_ok {
            println!("PASS {} (hash={hash:016x})", case.rom);
        } else {
            failed += 1;
            println!(
                "FAIL {} (serial_ok={serial_ok}, frame_ok={frame_ok}, hash={hash:016x})",
                case.rom
            );
        }
    }
    if failed > 0 {
        return Err(format!("regression suite failed ({failed} case(s))").into());
    }
    Ok(())
}

fn framebuffer_hash(buf: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in buf {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
