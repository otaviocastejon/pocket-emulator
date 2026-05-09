pub(crate) mod common;
mod download_roms;
mod my_games;
mod recently_played;
mod settings;

pub(crate) use download_roms::{poll_download_folder_imports, render_get_roms_tab};
pub(crate) use my_games::render_my_games_tab;
pub(crate) use recently_played::render_recently_played_tab;
pub(crate) use settings::render_settings_tab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LauncherTab {
    MyGames,
    GetRoms,
    RecentlyPlayed,
    Settings,
}

pub(super) fn header_copy(active_tab: LauncherTab) -> (&'static str, &'static str) {
    match active_tab {
        LauncherTab::MyGames => ("My Games", "Your library, ready to play"),
        LauncherTab::GetRoms => (
            "Get ROMs",
            "Find games in your browser — imports appear in My Games",
        ),
        LauncherTab::RecentlyPlayed => ("Recent Sessions", "Jump back into your latest games"),
        LauncherTab::Settings => ("Settings", "Controls, display, and runtime behavior"),
    }
}
