use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use eframe::egui;
use pocketemulator::frontend::Controls;
use pocketemulator::rom_launch_check;
use pocketemulator::storage::{self, AudioMode, RecentRom, VideoFilter};

use super::branding::pocket_emulator_banner;
use super::components::buttons::{action_button, ButtonSize, ButtonVariant};
use super::components::cards::section_card;
use super::components::tab_bar::{library_folder_row, tab_button, TabIcon};
use super::components::toasts::{show_toasts, ToastMessage, ToastTone};
use super::components::tokens::{
    destructive_text, muted_text, text_caption, text_display, text_eyebrow,
};
use super::tabs::{
    poll_download_folder_imports, render_get_roms_tab, render_my_games_tab,
    render_recently_played_tab, render_settings_tab, LauncherTab,
};
use super::theme::{apply_retro_style, space_2, space_3, surface_fill};
use super::Selection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GameSortMode {
    Recent,
    AtoZ,
    Favorites,
}

pub(crate) struct LauncherApp {
    pub(in crate::desktop::launcher) roms: Vec<PathBuf>,
    pub(in crate::desktop::launcher) my_games: Vec<RecentRom>,
    pub(in crate::desktop::launcher) selected_idx: usize,
    pub(in crate::desktop::launcher) selected_rom_path: Option<PathBuf>,
    pub(in crate::desktop::launcher) scale: u32,
    pub(in crate::desktop::launcher) controls: Controls,
    pub(in crate::desktop::launcher) autosave_enabled: bool,
    out: Arc<Mutex<Option<Selection>>>,
    pub(in crate::desktop::launcher) error: Option<String>,
    styled: bool,
    pub(in crate::desktop::launcher) active_tab: LauncherTab,
    pub(in crate::desktop::launcher) search: String,
    pub(in crate::desktop::launcher) favorites_only: bool,
    pub(in crate::desktop::launcher) sort_mode: GameSortMode,
    pub(in crate::desktop::launcher) selected_video_filter: VideoFilter,
    pub(in crate::desktop::launcher) selected_audio_mode: AudioMode,
    pub(in crate::desktop::launcher) onboarding_checked: bool,
    next_toast_id: u64,
    toasts: Vec<ToastMessage>,
    /// [`poll_download_folder_imports`] throttling (egui time, seconds).
    pub(in crate::desktop::launcher) last_download_poll_time: f64,
    /// Previous (size, mtime) per Downloads path — two identical polls ⇒ file stable.
    pub(in crate::desktop::launcher) download_prev_snapshot: HashMap<PathBuf, (u64, u128)>,
    /// Canonical Downloads paths we already imported into the library.
    pub(in crate::desktop::launcher) download_imported_sources: HashSet<PathBuf>,
}

impl LauncherApp {
    pub(super) fn new(
        mut roms: Vec<PathBuf>,
        mut my_games: Vec<RecentRom>,
        default_scale: u32,
        out: Arc<Mutex<Option<Selection>>>,
    ) -> Self {
        my_games.retain(|entry| entry.path.exists());
        my_games.dedup_by(|a, b| a.path == b.path);
        my_games.sort_by_key(|r| std::cmp::Reverse(r.last_played_unix_secs.unwrap_or(0)));
        for game in &my_games {
            if !roms.iter().any(|p| p == &game.path) {
                roms.push(game.path.clone());
            }
        }
        roms.sort();
        roms.dedup();

        let error = if roms.is_empty() {
            Some("No ROMs found in ROMS/ or roms/. Use Browse ROM to pick one.".to_string())
        } else {
            None
        };
        Self {
            roms,
            my_games,
            selected_idx: 0,
            selected_rom_path: None,
            scale: default_scale,
            controls: Controls::default(),
            autosave_enabled: true,
            out,
            error,
            styled: false,
            active_tab: LauncherTab::MyGames,
            search: String::new(),
            favorites_only: false,
            sort_mode: GameSortMode::Recent,
            selected_video_filter: VideoFilter::Sharp,
            selected_audio_mode: AudioMode::Balanced,
            onboarding_checked: false,
            next_toast_id: 1,
            toasts: Vec::new(),
            last_download_poll_time: -1_000.0,
            download_prev_snapshot: HashMap::new(),
            download_imported_sources: HashSet::new(),
        }
    }

    /// ROM the user explicitly chose in the launcher (`Browse`, table row, import, …).
    /// No silent fallback to `roms[selected_idx]` — that made Play / Settings look “selected”
    /// when nothing was picked.
    pub(in crate::desktop::launcher) fn selected_rom(&self) -> Option<&PathBuf> {
        self.selected_rom_path.as_ref()
    }

    pub(in crate::desktop::launcher) fn apply_profile_for_rom(&mut self, rom: &Path) {
        let profile = storage::load_profile(rom);
        self.scale = profile.scale.clamp(1, 10);
        self.autosave_enabled = profile.autosave_enabled;
        self.controls = Controls::from_env_string(&profile.controls_env).unwrap_or_default();
        self.selected_video_filter = profile.video_filter;
        self.selected_audio_mode = profile.audio_mode;
    }

    fn ensure_selected(&mut self, rom: PathBuf) {
        if let Some(existing) = self.roms.iter().position(|p| p == &rom) {
            self.selected_idx = existing;
        } else {
            self.roms.push(rom.clone());
            self.selected_idx = self.roms.len().saturating_sub(1);
        }
        self.selected_rom_path = Some(rom);
        self.error = None;
    }

    pub(in crate::desktop::launcher) fn launch_rom_and_close(
        &mut self,
        ctx: &egui::Context,
        rom: PathBuf,
    ) {
        // Resolve library path before checks/save so profiles match `main`'s post-import ROM path
        // (the child loads settings by canonical library path).
        let rom = storage::import_rom_into_library(&rom).unwrap_or(rom);

        if let Err(msg) = rom_launch_check(&rom) {
            self.error = Some(msg.clone());
            self.push_toast(
                "Can't launch this ROM".to_string(),
                msg,
                ToastTone::Destructive,
                None,
                8.0,
            );
            return;
        }

        self.ensure_selected(rom.clone());
        let _ = storage::save_profile(
            &rom,
            storage::GameProfile {
                path: rom.clone(),
                scale: self.scale,
                controls_env: self.controls.to_env_string(),
                autosave_enabled: self.autosave_enabled,
                favorite: storage::is_favorite(&rom),
                video_filter: self.selected_video_filter,
                audio_mode: self.selected_audio_mode,
            },
        );
        if let Ok(mut slot) = self.out.lock() {
            *slot = Some(Selection {
                rom,
                scale: self.scale,
                controls: self.controls.clone(),
                autosave_enabled: self.autosave_enabled,
                video_filter: self.selected_video_filter,
                audio_mode: self.selected_audio_mode,
            });
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    pub(in crate::desktop::launcher) fn import_rom_into_library(
        &mut self,
        source_path: &Path,
    ) -> Result<PathBuf, String> {
        let imported = storage::import_rom_into_library(source_path).map_err(|_| {
            "Could not import ROM into library. Please try another file.".to_string()
        })?;
        if let Err(e) = storage::note_recent_rom(&imported) {
            log::warn!("failed to note imported ROM in recent list: {e}");
        }
        if !self.roms.iter().any(|p| p == &imported) {
            self.roms.push(imported.clone());
            self.roms.sort();
            self.roms.dedup();
        }
        if let Some(i) = self.roms.iter().position(|p| p == &imported) {
            self.selected_idx = i;
        }
        self.my_games = storage::recent_games_existing();
        self.selected_rom_path = Some(imported.clone());
        self.error = None;
        self.push_toast(
            "ROM imported".to_string(),
            format!(
                "{} added to your library",
                imported
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Game")
            ),
            ToastTone::Success,
            None,
            3.0,
        );
        Ok(imported)
    }

    pub(in crate::desktop::launcher) fn push_toast(
        &mut self,
        title: String,
        detail: String,
        tone: ToastTone,
        action_label: Option<String>,
        ttl_seconds: f64,
    ) {
        let id = self.next_toast_id;
        self.next_toast_id = self.next_toast_id.saturating_add(1);
        // Keep one actionable toast max.
        if action_label.is_some() {
            self.toasts.retain(|t| t.action_label.is_none());
        }
        self.toasts.push(ToastMessage {
            id,
            title,
            detail,
            tone,
            action_label,
            ttl_pending: Some(ttl_seconds.max(0.25)),
            expires_at: 0.0,
        });
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.styled {
            apply_retro_style(ctx);
            self.styled = true;
        }
        let mut nav_frame = egui::Frame::side_top_panel(ctx.style().as_ref());
        nav_frame.fill = surface_fill();
        egui::SidePanel::left("launcher_nav")
            .frame(nav_frame)
            .resizable(true)
            .default_width(230.0)
            .min_width(200.0)
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("LAUNCHER").small().color(muted_text()));
                ui.add_space(space_2());
                if tab_button(
                    ui,
                    self.active_tab == LauncherTab::MyGames,
                    TabIcon::Gamepad,
                    "My Games",
                )
                .clicked()
                {
                    self.active_tab = LauncherTab::MyGames;
                }
                if tab_button(
                    ui,
                    self.active_tab == LauncherTab::GetRoms,
                    TabIcon::Globe,
                    "Get ROMs",
                )
                .clicked()
                {
                    self.active_tab = LauncherTab::GetRoms;
                }
                ui.add_enabled_ui(false, |ui| {
                    tab_button(ui, false, TabIcon::Clock, "Favorites")
                        .on_hover_text("Not available yet");
                    tab_button(ui, false, TabIcon::Gamepad, "Saves")
                        .on_hover_text("Not available yet");
                });
                if tab_button(
                    ui,
                    self.active_tab == LauncherTab::RecentlyPlayed,
                    TabIcon::Clock,
                    "Recent",
                )
                .clicked()
                {
                    self.active_tab = LauncherTab::RecentlyPlayed;
                }
                if tab_button(
                    ui,
                    self.active_tab == LauncherTab::Settings,
                    TabIcon::Gear,
                    "Settings",
                )
                .clicked()
                {
                    self.active_tab = LauncherTab::Settings;
                }
                ui.add_space(space_3() - 2.0);
                ui.separator();
                ui.add_space(space_2());
                ui.label(text_caption("LIBRARY"));
                library_folder_row(ui, "All ROMs");
                library_folder_row(ui, "Game Boy");
                library_folder_row(ui, "Game Boy Color");
                ui.add_space(space_2());
                ui.separator();
                ui.add_space(space_2());
                ui.small(text_caption(format!("{} ROMs detected", self.roms.len())));
                ui.small(text_caption(format!(
                    "{} recently played",
                    self.my_games.len()
                )));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            section_card(ui, |ui| {
                ui.horizontal(|ui| {
                    let (title, subtitle) = super::tabs::header_copy(self.active_tab);
                    ui.vertical(|ui| {
                        ui.label(text_eyebrow(format!(
                            "LAUNCHER · {}",
                            match self.active_tab {
                                LauncherTab::MyGames => "MY GAMES",
                                LauncherTab::GetRoms => "GET ROMS",
                                LauncherTab::RecentlyPlayed => "RECENT",
                                LauncherTab::Settings => "SETTINGS",
                            }
                        )));
                        if self.active_tab == LauncherTab::MyGames {
                            ui.add_space(space_2());
                            pocket_emulator_banner(ui, 56.0);
                            ui.add_space(space_2());
                            ui.label(text_display(title));
                        } else {
                            ui.label(text_display(title));
                        }
                        let copy = match self.active_tab {
                            LauncherTab::MyGames => format!("{} ROMs in library", self.roms.len()),
                            _ => subtitle.to_string(),
                        };
                        ui.label(text_caption(copy));
                    });
                    if self.active_tab == LauncherTab::MyGames {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if action_button(ui, "+  Add ROM", ButtonVariant::Primary, ButtonSize::Lg)
                                .clicked()
                            {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("Game Boy ROM", &["gb", "gbc"])
                                    .pick_file()
                                {
                                    if self.import_rom_into_library(&path).is_err() {
                                        self.error = Some(
                                            "Could not import ROM into library. Please try another file."
                                                .to_string(),
                                        );
                                    }
                                }
                            }
                        });
                    }
                });
            });
            ui.add_space(space_3());
            match self.active_tab {
                LauncherTab::MyGames => render_my_games_tab(self, ui, ctx),
                LauncherTab::GetRoms => render_get_roms_tab(self, ui),
                LauncherTab::RecentlyPlayed => render_recently_played_tab(self, ui, ctx),
                LauncherTab::Settings => render_settings_tab(self, ui),
            }

            if let Some(err) = &self.error {
                ui.add_space(space_2());
                ui.colored_label(destructive_text(), err);
            }
        });
        poll_download_folder_imports(self, ctx);

        show_toasts(ctx, &mut self.toasts, |_id| {});
    }
}
