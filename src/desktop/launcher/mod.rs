use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eframe::egui;
use pocketemulator::frontend::Controls;
use pocketemulator::storage::{AudioMode, RecentRom, VideoFilter};
use pocketemulator::ui_icon;

mod app;
mod branding;
mod components;
mod tabs;
mod theme;

pub(super) use app::LauncherApp;

#[derive(Debug, Clone)]
struct Selection {
    rom: PathBuf,
    scale: u32,
    controls: Controls,
    autosave_enabled: bool,
    video_filter: VideoFilter,
    audio_mode: AudioMode,
}

pub fn select_rom_with_ui(
    discovered_roms: Vec<PathBuf>,
    my_games: Vec<RecentRom>,
    default_scale: u32,
) -> Result<(PathBuf, u32, Controls, bool, VideoFilter, AudioMode), Box<dyn std::error::Error>> {
    let shared = Arc::new(Mutex::new(None::<Selection>));
    let shared_out = Arc::clone(&shared);

    theme::init_launcher_theme();

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([980.0, 640.0])
        .with_min_inner_size([760.0, 520.0])
        .with_maximized(true)
        .with_resizable(true);
    if let Some((rgba, w, h)) = ui_icon::load_icon_rgba() {
        viewport = viewport.with_icon(egui::IconData {
            rgba,
            width: w,
            height: h,
        });
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let app_roms = discovered_roms;
    let app_recent = my_games;
    let app_scale = default_scale.clamp(1, 10);

    eframe::run_native(
        "PocketEmulator launcher",
        options,
        Box::new(move |_cc| {
            Box::new(LauncherApp::new(
                app_roms.clone(),
                app_recent.clone(),
                app_scale,
                Arc::clone(&shared_out),
            ))
        }),
    )?;

    let chosen = shared
        .lock()
        .map_err(|_| "Launcher state lock poisoned")?
        .clone()
        .ok_or("Launcher closed without selecting a ROM")?;
    Ok((
        chosen.rom,
        chosen.scale,
        chosen.controls,
        chosen.autosave_enabled,
        chosen.video_filter,
        chosen.audio_mode,
    ))
}
