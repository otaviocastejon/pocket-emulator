use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::paths::{data_dir, ensure_data_dirs};
use super::types::{AppState, RecentRom};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LegacyAppState {
    recent_roms: Vec<PathBuf>,
}

fn state_path() -> Option<PathBuf> {
    data_dir().map(|d| d.join("state.json"))
}

pub fn load_state() -> AppState {
    let Some(path) = state_path() else {
        return AppState::default();
    };
    match fs::read_to_string(path) {
        Ok(s) => {
            if let Ok(state) = serde_json::from_str::<AppState>(&s) {
                state
            } else if let Ok(legacy) = serde_json::from_str::<LegacyAppState>(&s) {
                AppState {
                    recent_roms: legacy
                        .recent_roms
                        .into_iter()
                        .map(|path| RecentRom {
                            path,
                            last_played_unix_secs: None,
                        })
                        .collect(),
                    profiles: Vec::new(),
                }
            } else {
                AppState::default()
            }
        }
        Err(_) => AppState::default(),
    }
}

pub fn save_state(state: &AppState) -> io::Result<()> {
    ensure_data_dirs()?;
    let Some(path) = state_path() else {
        return Ok(());
    };
    let json = serde_json::to_string_pretty(state).map_err(io::Error::other)?;
    fs::write(path, json)?;
    Ok(())
}
