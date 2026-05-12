use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "PocketEmulator", about = "DMG Game Boy emulator")]
pub struct Args {
    /// Path to a ROM (`.gb` or `.gbc`)
    pub rom: Option<PathBuf>,
    /// Window scale (integer).
    #[arg(long)]
    pub scale: Option<u32>,
    /// Print cartridge header and exit (no window)
    #[arg(long)]
    pub info: bool,
    /// Force launcher menu even when ROM path is provided
    #[arg(long)]
    pub menu: bool,
    /// Optional ROM directory for launcher menu
    #[arg(long)]
    pub rom_dir: Option<PathBuf>,
    /// Disable periodic autosave
    #[arg(long)]
    pub no_autosave: bool,
    /// Build release binary and copy to ./dist
    #[arg(long)]
    pub package: bool,
    /// Run automated compatibility/regression suite
    #[arg(long)]
    pub regression: bool,
    /// ROM directory for regression suite
    #[arg(long)]
    pub regression_dir: Option<PathBuf>,
}
