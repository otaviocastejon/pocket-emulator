use std::path::PathBuf;

use eframe::egui;
use pocketemulator::storage;

use super::super::app::GameSortMode;
use super::super::LauncherApp;
use super::common::{display_game_name, format_last_played};
use crate::desktop::launcher::components::buttons::{
    action_button, action_button_row, ButtonSize, ButtonVariant,
};
use crate::desktop::launcher::components::cards::section_card;
use crate::desktop::launcher::components::page_headers::section_title_bar;
use crate::desktop::launcher::components::rom_library_list::render_rom_library_table;
use crate::desktop::launcher::components::scroll_fill::fill_vertical_scroll;
use crate::desktop::launcher::components::toasts::ToastTone;
use crate::desktop::launcher::components::tokens::{
    info_text, muted_text, primary_color, text_caption, text_eyebrow, text_h3, warning_text,
};
use crate::desktop::launcher::theme::{space_2, space_4};

pub(crate) fn render_my_games_tab(app: &mut LauncherApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    section_card(ui, |ui| {
        section_title_bar(
            ui,
            text_h3("Recent Games").color(primary_color()),
            "Most recently played first",
            |ui| {
                ui.small(text_caption(format!("{} games", app.my_games.len())));
            },
        );

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(space_2(), space_2());
            ui.label(text_eyebrow("SEARCH"));
            ui.add(
                egui::TextEdit::singleline(&mut app.search)
                    .hint_text("Pokemon, Tetris, ...")
                    .desired_width(260.0),
            );
            ui.checkbox(&mut app.favorites_only, "Favorites only");
            ui.separator();
            ui.label(text_eyebrow("SORT"));
            egui::ComboBox::from_id_source("my_games_sort")
                .selected_text(match app.sort_mode {
                    GameSortMode::Recent => "Recent",
                    GameSortMode::AtoZ => "A-Z",
                    GameSortMode::Favorites => "Favorites first",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.sort_mode, GameSortMode::Recent, "Recent");
                    ui.selectable_value(&mut app.sort_mode, GameSortMode::AtoZ, "A-Z");
                    ui.selectable_value(
                        &mut app.sort_mode,
                        GameSortMode::Favorites,
                        "Favorites first",
                    );
                });
        });
        ui.add_space(space_2());

        fill_vertical_scroll(ui, "my_games_library_scroll", |ui| {
            let viewport_w = ui.available_width();
            ui.set_max_width(viewport_w);

            let mut launch_game: Option<PathBuf> = None;
            let mut listed: Vec<_> = app.my_games.clone();
            match app.sort_mode {
                GameSortMode::Recent => {}
                GameSortMode::AtoZ => {
                    listed.sort_by_key(|g| {
                        g.path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_ascii_lowercase()
                    });
                }
                GameSortMode::Favorites => {
                    listed.sort_by_key(|g| {
                        let fav = storage::is_favorite(&g.path);
                        (
                            !fav,
                            std::cmp::Reverse(g.last_played_unix_secs.unwrap_or(0)),
                        )
                    });
                }
            }
            if app.my_games.is_empty() {
                ui.label("No recent games yet. Use Browse ROM to add one.");
            }
            let mut filtered = Vec::new();
            for game in listed {
                let file_name = game
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("<invalid file name>");
                let haystack = format!("{} {}", display_game_name(&game.path, 128), file_name)
                    .to_ascii_lowercase();
                if !app.search.trim().is_empty()
                    && !haystack.contains(app.search.trim().to_ascii_lowercase().as_str())
                {
                    continue;
                }
                let is_favorite = storage::is_favorite(&game.path);
                if app.favorites_only && !is_favorite {
                    continue;
                }
                filtered.push(game);
            }

            if !filtered.is_empty() {
                let avail = ui.available_width();
                render_rom_library_table(ui, avail, app, &filtered, &mut launch_game);
            } else if !app.my_games.is_empty() {
                ui.label("No games match your search or filters.");
            }
            if let Some(game) = launch_game {
                app.apply_profile_for_rom(&game);
                app.launch_rom_and_close(ctx, game);
            }
        });
    });

    ui.add_space(space_4());
    section_card(ui, |ui| {
        action_button_row(ui, |ui| {
            let can_play = app.selected_rom().is_some();
            let play_slot = ui.add_enabled_ui(can_play, |ui| {
                action_button(ui, "Play", ButtonVariant::Primary, ButtonSize::Lg)
            });
            play_slot
                .response
                .on_disabled_hover_text("Choose a game in the table above first");
            if play_slot.inner.clicked() {
                if let Some(path) = app.selected_rom().cloned() {
                    app.apply_profile_for_rom(&path);
                    app.launch_rom_and_close(ctx, path);
                }
            }
            if action_button(ui, "Import ROM…", ButtonVariant::Secondary, ButtonSize::Lg).clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Game Boy ROM", &["gb", "gbc"])
                    .pick_file()
                {
                    match app.import_rom_into_library(&path) {
                        Ok(imported) => {
                            app.apply_profile_for_rom(&imported);
                        }
                        Err(msg) => {
                            app.error = Some(msg);
                            app.push_toast(
                                "Couldn't import ROM".to_string(),
                                "Please check file format and permissions".to_string(),
                                ToastTone::Destructive,
                                None,
                                4.0,
                            );
                        }
                    }
                }
            }
            if action_button(ui, "Quit", ButtonVariant::Secondary, ButtonSize::Lg).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        ui.add_space(space_2());
        if let Some(selected) = app.selected_rom() {
            let shown = display_game_name(selected.as_path(), 56);
            ui.small(text_caption(format!("Playing next · {shown}")));
            if let Some(health) = storage::save_health_for_rom(selected.as_path()) {
                let line = save_status_one_line(&health);
                let rt = if health.has_save {
                    egui::RichText::new(line).color(muted_text())
                } else if health.has_backup {
                    egui::RichText::new(line).color(warning_text())
                } else {
                    egui::RichText::new(line).color(info_text())
                };
                ui.small(rt);
            }
        } else {
            ui.small(text_caption(
                "Tip · click a row or its title to choose a game (▶ launches immediately). \
                 Same actions as + Add ROM in the header.",
            ));
        }
    });

    if !app.onboarding_checked && app.my_games.is_empty() {
        section_card(ui, |ui| {
            ui.label(
                egui::RichText::new("Quick Start Checklist")
                    .strong()
                    .color(primary_color()),
            );
            ui.label("1) Import your first ROM");
            ui.label("2) Open Settings and confirm controls");
            ui.label("3) Set preferred game window size");
            if ui.button("Got it").clicked() {
                app.onboarding_checked = true;
            }
        });
    }

    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && ctx.memory(|m| m.focused().is_none()) {
        if let Some(path) = app.selected_rom().cloned() {
            app.apply_profile_for_rom(&path);
            app.launch_rom_and_close(ctx, path);
        }
    }
}

fn save_status_one_line(health: &storage::SaveHealth) -> String {
    let last = health
        .last_modified_unix_secs
        .map(|v| format_last_played(Some(v)))
        .unwrap_or_else(|| "never".to_string());
    if health.has_save {
        format!("Save data on disk · last write {last}")
    } else if health.has_backup {
        format!("Backup only (main save missing) · last touched {last}")
    } else {
        "No .sav file yet — new game progress".to_string()
    }
}
