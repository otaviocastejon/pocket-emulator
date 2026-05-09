use std::path::{Path, PathBuf};

use clap::Parser;
use pocketemulator::cartridge::Header;
use pocketemulator::frontend::{run_window, Controls};
use pocketemulator::regression::run_regression_suite;
use pocketemulator::rom_launch_check;
use pocketemulator::runtime_env;
use pocketemulator::storage::{self, AudioMode, VideoFilter};
use pocketemulator::GameBoy;
mod desktop;
use desktop::cli::Args;
use desktop::logging;
use desktop::package::package_current_platform;
use desktop::panic_hook::install_panic_hook;
use desktop::tauri_api::run_tauri_app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init()?;
    install_panic_hook();
    let args = Args::parse();

    if args.package {
        package_current_platform()?;
        return Ok(());
    }

    if args.regression {
        let dir = args
            .regression_dir
            .unwrap_or_else(|| PathBuf::from("roms/regression"));
        run_regression_suite(dir.as_path())?;
        return Ok(());
    }

    if args.info {
        let rom = args
            .rom
            .as_ref()
            .ok_or("--info requires a ROM path argument")?;
        ensure_supported_rom(rom)?;
        let data = std::fs::read(rom)?;
        match Header::parse(&data) {
            Ok(h) => {
                println!("Title: {}", h.title);
                println!("CGB:   {:?}", h.cgb_support);
                println!("Type:  {}", h.cartridge_type.as_str());
                println!("ROM banks: {}", h.rom_banks);
                println!("RAM: {} bytes", h.ram_size_bytes);
                println!(
                    "Header checksum: {}",
                    if h.header_checksum_ok { "OK" } else { "BAD" }
                );
            }
            Err(e) => {
                eprintln!("Header error: {e}");
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    if args.menu || args.rom.is_none() {
        run_tauri_app()?;
        return Ok(());
    }

    let rom_path = args
        .rom
        .expect("rom path should be present when not using --menu");
    ensure_supported_rom(&rom_path)?;
    let rom_path = storage::import_rom_into_library(&rom_path).unwrap_or(rom_path);
    let profile = storage::load_profile(&rom_path);
    let controls = runtime_env::var_pair(runtime_env::CONTROLS.0, runtime_env::CONTROLS.1)
        .as_deref()
        .and_then(|v| Controls::from_env_string(v))
        .or_else(|| Controls::from_env_string(&profile.controls_env))
        .unwrap_or_default();
    let autosave_enabled = runtime_env::var_pair(runtime_env::AUTOSAVE.0, runtime_env::AUTOSAVE.1)
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(profile.autosave_enabled);
    let video_filter =
        runtime_env::var_pair(runtime_env::VIDEO_FILTER.0, runtime_env::VIDEO_FILTER.1)
            .map(|v| {
                if v.eq_ignore_ascii_case("smooth") {
                    VideoFilter::Smooth
                } else {
                    VideoFilter::Sharp
                }
            })
            .unwrap_or(profile.video_filter);
    let audio_mode = runtime_env::var_pair(runtime_env::AUDIO_MODE.0, runtime_env::AUDIO_MODE.1)
        .map(|v| {
            if v.eq_ignore_ascii_case("low-latency") {
                AudioMode::LowLatency
            } else {
                AudioMode::Balanced
            }
        })
        .unwrap_or(profile.audio_mode);
    let scale = args.scale.unwrap_or(profile.scale).clamp(1, 10);

    run_game_window(
        rom_path,
        scale,
        controls,
        autosave_enabled,
        video_filter,
        audio_mode,
        args.no_autosave,
    )
}

fn ensure_supported_rom(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    rom_launch_check(path).map_err(|e| e.into())
}

fn run_game_window(
    rom_path: PathBuf,
    scale: u32,
    controls: Controls,
    mut autosave_enabled: bool,
    video_filter: VideoFilter,
    audio_mode: AudioMode,
    no_autosave: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if no_autosave {
        autosave_enabled = false;
    }
    let scale = scale.clamp(1, 10);

    let _ = storage::note_recent_rom(&rom_path);
    let _ = storage::save_profile(
        &rom_path,
        storage::GameProfile {
            path: rom_path.clone(),
            scale,
            controls_env: controls.to_env_string(),
            autosave_enabled,
            favorite: storage::is_favorite(&rom_path),
            video_filter,
            audio_mode,
        },
    );
    let _ = storage::ensure_data_dirs();
    let centralized_save = storage::save_path_for_rom(&rom_path);
    let gb = GameBoy::from_rom_file_with_save_path(&rom_path, centralized_save.as_ref())?;
    run_window(
        gb,
        scale,
        controls,
        autosave_enabled,
        video_filter,
        audio_mode,
    )?;
    Ok(())
}
