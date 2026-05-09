use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::state::{load_state, save_state};
use super::types::{GameProfile, RecentRom};

pub fn note_recent_rom(path: &Path) -> io::Result<()> {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs());
    let mut state = load_state();
    state.recent_roms.retain(|entry| entry.path != canonical);
    state.recent_roms.insert(
        0,
        RecentRom {
            path: canonical.clone(),
            last_played_unix_secs: now,
        },
    );
    if state.recent_roms.len() > 20 {
        state.recent_roms.truncate(20);
    }
    if !state.profiles.iter().any(|p| p.path == canonical) {
        let profile = GameProfile {
            path: canonical.clone(),
            ..GameProfile::default()
        };
        state.profiles.push(profile);
    }
    save_state(&state)
}

pub fn recent_roms_existing() -> Vec<PathBuf> {
    let state = load_state();
    state
        .recent_roms
        .into_iter()
        .map(|r| r.path)
        .filter(|p| p.exists())
        .collect::<Vec<_>>()
}

pub fn recent_games_existing() -> Vec<RecentRom> {
    let state = load_state();
    state
        .recent_roms
        .into_iter()
        .filter(|entry| entry.path.exists())
        .collect::<Vec<_>>()
}

pub fn load_profile(path: &Path) -> GameProfile {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let state = load_state();
    state
        .profiles
        .into_iter()
        .find(|p| p.path == canonical)
        .unwrap_or(GameProfile {
            path: canonical,
            ..GameProfile::default()
        })
}

pub fn save_profile(path: &Path, profile: GameProfile) -> io::Result<()> {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let mut state = load_state();
    let mut normalized = profile;
    normalized.path = canonical.clone();
    if let Some(existing) = state.profiles.iter_mut().find(|p| p.path == canonical) {
        *existing = normalized;
    } else {
        state.profiles.push(normalized);
    }
    save_state(&state)
}

pub fn is_favorite(path: &Path) -> bool {
    load_profile(path).favorite
}

pub fn toggle_favorite(path: &Path) -> io::Result<bool> {
    let mut profile = load_profile(path);
    profile.favorite = !profile.favorite;
    let favorite = profile.favorite;
    save_profile(path, profile)?;
    Ok(favorite)
}
