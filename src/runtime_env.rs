//! Child-process env for launcher → game (`POCKETEMU_*`; `MYGAMEBOY_*` aliases).

pub const CONTROLS: (&str, &str) = ("POCKETEMU_CONTROLS", "MYGAMEBOY_CONTROLS");
pub const AUTOSAVE: (&str, &str) = ("POCKETEMU_AUTOSAVE", "MYGAMEBOY_AUTOSAVE");
pub const VIDEO_FILTER: (&str, &str) = ("POCKETEMU_VIDEO_FILTER", "MYGAMEBOY_VIDEO_FILTER");
pub const AUDIO_MODE: (&str, &str) = ("POCKETEMU_AUDIO_MODE", "MYGAMEBOY_AUDIO_MODE");
pub const LINK_BIND: (&str, &str) = ("POCKETEMU_LINK_BIND", "MYGAMEBOY_LINK_BIND");
pub const LINK_PEER: (&str, &str) = ("POCKETEMU_LINK_PEER", "MYGAMEBOY_LINK_PEER");

pub fn var_pair(primary: &str, legacy: &str) -> Option<String> {
    std::env::var(primary)
        .ok()
        .or_else(|| std::env::var(legacy).ok())
}
