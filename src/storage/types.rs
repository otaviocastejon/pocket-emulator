use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::frontend::Controls;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppState {
    #[serde(default)]
    pub recent_roms: Vec<RecentRom>,
    #[serde(default)]
    pub profiles: Vec<GameProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentRom {
    pub path: PathBuf,
    pub last_played_unix_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProfile {
    pub path: PathBuf,
    pub scale: u32,
    pub controls_env: String,
    pub autosave_enabled: bool,
    pub favorite: bool,
    pub video_filter: VideoFilter,
    pub audio_mode: AudioMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoFilter {
    Sharp,
    Smooth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioMode {
    Balanced,
    LowLatency,
}

impl Default for GameProfile {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            scale: 4,
            controls_env: Controls::default().to_env_string(),
            autosave_enabled: true,
            favorite: false,
            video_filter: VideoFilter::Sharp,
            audio_mode: AudioMode::Balanced,
        }
    }
}
