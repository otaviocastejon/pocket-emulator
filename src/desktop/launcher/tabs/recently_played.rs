use std::path::PathBuf;

use eframe::egui;
use pocketemulator::storage;

use super::super::LauncherApp;
use crate::desktop::launcher::components::badges::{status_badge, BadgeTone};
use crate::desktop::launcher::components::buttons::{action_button, ButtonSize, ButtonVariant};
use crate::desktop::launcher::components::cards::section_card;
use crate::desktop::launcher::components::rom_library_list::render_rom_library_table;
use crate::desktop::launcher::components::scroll_fill::fill_vertical_scroll;
use crate::desktop::launcher::components::toasts::ToastTone;
use crate::desktop::launcher::components::tokens::{
    info_text, primary_color, text_caption, text_h3,
};
use crate::desktop::launcher::theme::{space_2, space_4};

pub(crate) fn render_recently_played_tab(
    app: &mut LauncherApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
) {
    section_card(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(text_h3("Play History").color(primary_color()));
            ui.separator();
            ui.small(text_caption("Most recent sessions"));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.small(text_caption("Top 12 entries"));
            });
        });
        ui.add_space(space_2());

        let mut launch_game: Option<PathBuf> = None;
        fill_vertical_scroll(ui, "recent_play_history_scroll", |ui| {
            if app.my_games.is_empty() {
                ui.label("No play history yet. Launch a game to build history.");
            }
            let recent: Vec<_> = app.my_games.iter().take(12).cloned().collect();
            if !recent.is_empty() {
                let avail = ui.available_width();
                render_rom_library_table(ui, avail, app, &recent, &mut launch_game);
            }
            if let Some(path) = launch_game {
                app.apply_profile_for_rom(&path);
                app.launch_rom_and_close(ctx, path);
            }
        });
    });
    ui.add_space(space_4());
    section_card(ui, |ui| {
        ui.horizontal(|ui| {
            let can_play = app.selected_rom().is_some();
            if ui
                .add_enabled_ui(can_play, |ui| {
                    action_button(ui, "Play Selected", ButtonVariant::Primary, ButtonSize::Lg)
                })
                .inner
                .clicked()
            {
                if let Some(path) = app.selected_rom().cloned() {
                    app.apply_profile_for_rom(&path);
                    app.launch_rom_and_close(ctx, path);
                }
            }
            if action_button(
                ui,
                "Import ROM...",
                ButtonVariant::Secondary,
                ButtonSize::Lg,
            )
            .clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Game Boy ROM", &["gb", "gbc"])
                    .pick_file()
                {
                    match app.import_rom_into_library(&path) {
                        Ok(imported) => app.apply_profile_for_rom(&imported),
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
        });
        if let Some(selected) = app.selected_rom() {
            ui.add_space(space_2());
            let fav = storage::is_favorite(selected.as_path());
            ui.small(text_caption(format!(
                "Selected: {}{}",
                selected
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown"),
                if fav { "  |  Favorited" } else { "" }
            )));
            ui.horizontal(|ui| {
                status_badge(ui, "Tip", BadgeTone::Info);
                ui.small(
                    egui::RichText::new("Press Play Selected to resume quickly").color(info_text()),
                );
            });
        }
    });
}
