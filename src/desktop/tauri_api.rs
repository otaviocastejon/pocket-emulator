use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::UNIX_EPOCH;
use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use pocketemulator::frontend::Controls;
use pocketemulator::runtime_env;
use pocketemulator::storage::{self, AudioMode, GameProfile, VideoFilter};

use super::roms::discover_roms;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RomSummary {
    path: String,
    name: String,
    extension: String,
    favorite: bool,
    last_played_unix_secs: Option<u64>,
    profile: ProfileSummary,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSummary {
    scale: u32,
    controls_env: String,
    autosave_enabled: bool,
    video_filter: VideoFilter,
    audio_mode: AudioMode,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveProfileRequest {
    path: String,
    scale: u32,
    controls_env: String,
    autosave_enabled: bool,
    video_filter: VideoFilter,
    audio_mode: AudioMode,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchRomRequest {
    path: String,
    scale: Option<u32>,
    controls_env: Option<String>,
    autosave_enabled: Option<bool>,
    video_filter: Option<VideoFilter>,
    audio_mode: Option<AudioMode>,
    no_autosave: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveFileSummary {
    rom_path: String,
    rom_name: String,
    save_path: String,
    kind: String,
    size_bytes: u64,
    modified_unix_secs: Option<u64>,
}

#[tauri::command]
pub fn list_roms(rom_dir: Option<String>) -> Result<Vec<RomSummary>, String> {
    let rom_dir_path = rom_dir.map(PathBuf::from);
    let mut discovered = discover_roms(rom_dir_path.as_ref()).map_err(|e| e.to_string())?;
    discovered.sort();
    discovered.dedup();

    let recent = storage::recent_games_existing();
    let mut out = Vec::with_capacity(discovered.len());
    for path in discovered {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        let profile = storage::load_profile(&canonical);
        let last_played = recent
            .iter()
            .find(|r| r.path == canonical)
            .and_then(|r| r.last_played_unix_secs);
        out.push(rom_to_summary(&canonical, profile, last_played));
    }
    Ok(out)
}

#[tauri::command]
pub fn list_save_files(rom_dir: Option<String>) -> Result<Vec<SaveFileSummary>, String> {
    let rom_dir_path = rom_dir.map(PathBuf::from);
    let mut discovered = discover_roms(rom_dir_path.as_ref()).map_err(|e| e.to_string())?;
    discovered.sort();
    discovered.dedup();

    let mut out = Vec::new();
    let mut known: HashMap<PathBuf, (String, String, String)> = HashMap::new();
    for path in discovered {
        let canonical = path.canonicalize().unwrap_or(path);
        let rom_name = canonical
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Game")
            .replace('_', " ");
        let Some(primary) = storage::save_path_for_rom(&canonical) else {
            continue;
        };
        let backup = primary.with_extension("sav.bak");

        known.insert(
            primary.clone(),
            (
                canonical.to_string_lossy().to_string(),
                rom_name.clone(),
                "save".to_string(),
            ),
        );
        known.insert(
            backup,
            (
                canonical.to_string_lossy().to_string(),
                rom_name,
                "backup".to_string(),
            ),
        );
    }

    if let Some(data_dir) = storage::data_dir() {
        let saves_dir = data_dir.join("saves");
        if let Ok(entries) = fs::read_dir(&saves_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let path_text = path.to_string_lossy();
                if !path_text.ends_with(".sav") && !path_text.ends_with(".sav.bak") {
                    continue;
                }
                let meta = fs::metadata(&path).ok();
                let modified = meta
                    .as_ref()
                    .and_then(|m| m.modified().ok())
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map(|d| d.as_secs());
                let (rom_path, rom_name, kind) = known.get(&path).cloned().unwrap_or_else(|| {
                    let file_name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    let inferred_kind = if path_text.ends_with(".sav.bak") {
                        "backup".to_string()
                    } else {
                        "save".to_string()
                    };
                    ("".to_string(), file_name, inferred_kind)
                });
                out.push(SaveFileSummary {
                    rom_path,
                    rom_name,
                    save_path: path.to_string_lossy().to_string(),
                    kind,
                    size_bytes: meta.as_ref().map_or(0, |m| m.len()),
                    modified_unix_secs: modified,
                });
            }
        }
    }
    out.sort_by(|a, b| b.modified_unix_secs.cmp(&a.modified_unix_secs));
    Ok(out)
}

#[tauri::command]
pub fn export_save_file(save_path: String) -> Result<(), String> {
    let src = PathBuf::from(&save_path);
    if !src.exists() {
        return Err("Save file not found".to_string());
    }
    let default_name = src
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("save.sav")
        .to_string();
    let dest = rfd::FileDialog::new()
        .set_file_name(&default_name)
        .save_file();
    let Some(dest_path) = dest else {
        return Ok(());
    };
    std::fs::copy(&src, &dest_path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_save_file(save_path: String) -> Result<(), String> {
    let path = PathBuf::from(&save_path);
    if !path.exists() {
        return Ok(());
    }
    std::fs::remove_file(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_saves_for_rom(rom_path: String) -> Result<u32, String> {
    let rom = PathBuf::from(&rom_path);
    let Some(primary) = storage::save_path_for_rom(&rom) else {
        return Ok(0);
    };
    let backup = primary.with_extension("sav.bak");
    let mut deleted = 0;
    for candidate in [primary, backup] {
        if candidate.exists() {
            std::fs::remove_file(&candidate).map_err(|e| e.to_string())?;
            deleted += 1;
        }
    }
    Ok(deleted)
}

#[tauri::command]
pub fn pick_and_import_rom() -> Result<Option<RomSummary>, String> {
    let picked = rfd::FileDialog::new()
        .add_filter("Game Boy ROM", &["gb", "gbc"])
        .pick_file();
    let Some(path) = picked else {
        return Ok(None);
    };
    let imported = storage::import_rom_into_library(&path).map_err(|e| e.to_string())?;
    storage::note_recent_rom(&imported).map_err(|e| e.to_string())?;
    let profile = storage::load_profile(&imported);
    let last_played = storage::recent_games_existing()
        .into_iter()
        .find(|r| r.path == imported)
        .and_then(|r| r.last_played_unix_secs);
    Ok(Some(rom_to_summary(&imported, profile, last_played)))
}

#[tauri::command]
pub fn import_rom_from_path(path: String) -> Result<RomSummary, String> {
    let imported = storage::import_rom_into_library(Path::new(&path)).map_err(|e| e.to_string())?;
    storage::note_recent_rom(&imported).map_err(|e| e.to_string())?;
    let profile = storage::load_profile(&imported);
    let last_played = storage::recent_games_existing()
        .into_iter()
        .find(|r| r.path == imported)
        .and_then(|r| r.last_played_unix_secs);
    Ok(rom_to_summary(&imported, profile, last_played))
}

#[tauri::command]
pub fn load_profile(path: String) -> Result<ProfileSummary, String> {
    let profile = storage::load_profile(Path::new(&path));
    Ok(profile_summary(profile))
}

#[tauri::command]
pub fn save_profile(request: SaveProfileRequest) -> Result<(), String> {
    let rom = PathBuf::from(&request.path);
    let profile = GameProfile {
        path: rom.clone(),
        scale: request.scale.clamp(1, 10),
        controls_env: request.controls_env,
        autosave_enabled: request.autosave_enabled,
        favorite: storage::is_favorite(&rom),
        video_filter: request.video_filter,
        audio_mode: request.audio_mode,
    };
    storage::save_profile(&rom, profile).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_favorite(path: String) -> Result<bool, String> {
    storage::toggle_favorite(Path::new(&path)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_rom_catalog() -> Result<(), String> {
    webbrowser::open("https://www.romsgames.net/roms/gameboy-color/")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn launch_rom(app: tauri::AppHandle, request: LaunchRomRequest) -> Result<(), String> {
    let path = PathBuf::from(&request.path);
    if !path.exists() {
        return Err("ROM file not found".to_string());
    }
    let profile = storage::load_profile(&path);
    let controls = request
        .controls_env
        .as_deref()
        .and_then(Controls::from_env_string)
        .unwrap_or_else(|| {
            Controls::from_env_string(&profile.controls_env).unwrap_or_else(Controls::default)
        });
    let autosave_enabled = request.autosave_enabled.unwrap_or(profile.autosave_enabled);
    let video_filter = request.video_filter.unwrap_or(profile.video_filter);
    let audio_mode = request.audio_mode.unwrap_or(profile.audio_mode);
    let scale = request.scale.unwrap_or(profile.scale).clamp(1, 10);
    let no_autosave = request.no_autosave.unwrap_or(false);

    let rom_path = storage::import_rom_into_library(&path).unwrap_or(path);
    storage::note_recent_rom(&rom_path).map_err(|e| e.to_string())?;
    storage::save_profile(
        &rom_path,
        GameProfile {
            path: rom_path.clone(),
            scale,
            controls_env: controls.to_env_string(),
            autosave_enabled,
            favorite: storage::is_favorite(&rom_path),
            video_filter,
            audio_mode,
        },
    )
    .map_err(|e| e.to_string())?;

    let exe = resolve_executable_for_spawn(&app)?;
    let vf = match video_filter {
        VideoFilter::Sharp => "sharp",
        VideoFilter::Smooth => "smooth",
    };
    let am = match audio_mode {
        AudioMode::Balanced => "balanced",
        AudioMode::LowLatency => "low-latency",
    };

    let rom_abs = rom_path
        .canonicalize()
        .unwrap_or_else(|_| rom_path.to_path_buf());
    let mut cmd = Command::new(exe);
    cmd.arg(&rom_abs)
        .arg("--scale")
        .arg(scale.to_string())
        .env(runtime_env::CONTROLS.0, controls.to_env_string())
        .env(runtime_env::CONTROLS.1, controls.to_env_string())
        .env(
            runtime_env::AUTOSAVE.0,
            if autosave_enabled { "1" } else { "0" },
        )
        .env(
            runtime_env::AUTOSAVE.1,
            if autosave_enabled { "1" } else { "0" },
        )
        .env(runtime_env::VIDEO_FILTER.0, vf)
        .env(runtime_env::VIDEO_FILTER.1, vf)
        .env(runtime_env::AUDIO_MODE.0, am)
        .env(runtime_env::AUDIO_MODE.1, am);
    if no_autosave {
        cmd.arg("--no-autosave");
    }
    cmd.spawn().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn run_tauri_app() -> tauri::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_roms,
            list_save_files,
            pick_and_import_rom,
            import_rom_from_path,
            load_profile,
            save_profile,
            toggle_favorite,
            open_rom_catalog,
            export_save_file,
            delete_save_file,
            delete_saves_for_rom,
            launch_rom
        ])
        .run(tauri::generate_context!())
}

fn rom_to_summary(path: &Path, profile: GameProfile, last_played_unix_secs: Option<u64>) -> RomSummary {
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Game")
        .replace('_', " ");
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    RomSummary {
        path: path.to_string_lossy().to_string(),
        name,
        extension,
        favorite: profile.favorite,
        last_played_unix_secs,
        profile: profile_summary(profile),
    }
}

fn profile_summary(profile: GameProfile) -> ProfileSummary {
    ProfileSummary {
        scale: profile.scale,
        controls_env: profile.controls_env,
        autosave_enabled: profile.autosave_enabled,
        video_filter: profile.video_filter,
        audio_mode: profile.audio_mode,
    }
}

fn resolve_executable_for_spawn(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    if let Some(argv0) = std::env::args_os().next() {
        let p = PathBuf::from(&argv0);
        if p.exists() {
            return Ok(p);
        }
        if let Ok(cwd) = std::env::current_dir() {
            let joined = cwd.join(&p);
            if joined.exists() {
                return Ok(joined);
            }
        }
    }
    let _ = app;
    std::env::current_exe().map_err(|e| e.to_string())
}
