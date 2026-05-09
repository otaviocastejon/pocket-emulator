//! ROM library table: shared by My Games and Recent tabs.

use std::path::PathBuf;

use eframe::egui;
use pocketemulator::storage::{self, RecentRom};

use super::super::app::LauncherApp;
use super::badges::{status_badge, BadgeTone};
use super::buttons::{action_button, ButtonSize, ButtonVariant};
use super::data_table::{
    table_cell, table_column_gap, table_data_row_shell, table_header_shell, table_inset_strip,
    table_shell_inner_width, TableCellAlign,
};
use super::rom_library_columns::{rom_library_column_widths, ROM_LIBRARY_ROW_HEIGHT};
use crate::desktop::launcher::tabs::common::{
    display_game_name, format_last_played, rom_extension_platform_label,
};
use crate::desktop::launcher::theme::space_2;

pub(crate) fn render_rom_library_table(
    ui: &mut egui::Ui,
    avail: f32,
    app: &mut LauncherApp,
    filtered: &[RecentRom],
    launch_game: &mut Option<PathBuf>,
) {
    table_inset_strip(ui, avail, |ui, content_w| {
        let inner_row = table_shell_inner_width(content_w);

        table_header_shell(ui, |ui| {
            rom_library_header_cells(ui, inner_row);
        });

        ui.add_space(space_2());
        ui.separator();
        ui.add_space(space_2());

        for game in filtered {
            let selected = app
                .selected_rom_path
                .as_ref()
                .is_some_and(|p| p == &game.path);

            table_data_row_shell(ui, selected, |ui| {
                rom_library_body_cells(ui, app, game, inner_row, selected, launch_game);
            });

            ui.separator();
        }
    });
}

fn rom_library_header_cells(ui: &mut egui::Ui, inner_w: f32) {
    let c = rom_library_column_widths(inner_w);
    let h = ROM_LIBRARY_ROW_HEIGHT;
    ui.allocate_ui_with_layout(
        egui::vec2(inner_w, h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            table_cell(ui, c.game, h, TableCellAlign::Left, |ui| {
                ui.label(egui::RichText::new("Game").strong());
            });
            table_column_gap(ui);
            table_cell(ui, c.plat, h, TableCellAlign::Center, |ui| {
                ui.label(egui::RichText::new("Platform").strong());
            });
            table_column_gap(ui);
            table_cell(ui, c.played, h, TableCellAlign::Right, |ui| {
                ui.label(egui::RichText::new("Last played").strong());
            });
            table_column_gap(ui);
            table_cell(ui, c.fav, h, TableCellAlign::Center, |ui| {
                ui.label(egui::RichText::new("★").strong());
            });
            table_column_gap(ui);
            table_cell(ui, c.act, h, TableCellAlign::Center, |ui| {
                ui.label(egui::RichText::new("Play").strong());
            });
        },
    );
}

fn rom_library_body_cells(
    ui: &mut egui::Ui,
    app: &mut LauncherApp,
    game: &RecentRom,
    inner_w: f32,
    selected: bool,
    launch_game: &mut Option<PathBuf>,
) {
    let c = rom_library_column_widths(inner_w);
    let h = ROM_LIBRARY_ROW_HEIGHT;
    let ext = game
        .path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let is_favorite = storage::is_favorite(&game.path);

    ui.allocate_ui_with_layout(
        egui::vec2(inner_w, h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            table_cell(ui, c.game, h, TableCellAlign::Left, |ui| {
                let name = display_game_name(&game.path, 52);
                let title_resp = ui.selectable_label(selected, egui::RichText::new(name).strong());
                if title_resp.clicked() {
                    app.selected_rom_path = Some(game.path.clone());
                }
                title_resp.on_hover_text(game.path.display().to_string());
            });
            table_column_gap(ui);
            table_cell(ui, c.plat, h, TableCellAlign::Center, |ui| {
                status_badge(ui, rom_extension_platform_label(&ext), BadgeTone::Platform);
            });
            table_column_gap(ui);
            table_cell(ui, c.played, h, TableCellAlign::Right, |ui| {
                ui.label(format_last_played(game.last_played_unix_secs));
            });
            table_column_gap(ui);
            table_cell(ui, c.fav, h, TableCellAlign::Center, |ui| {
                let fav_text = if is_favorite { "★" } else { "☆" };
                if ui.small_button(fav_text).clicked() {
                    let _ = storage::toggle_favorite(&game.path);
                }
            });
            table_column_gap(ui);
            table_cell(ui, c.act, h, TableCellAlign::Center, |ui| {
                if action_button(ui, "Play", ButtonVariant::Primary, ButtonSize::Sm).clicked() {
                    *launch_game = Some(game.path.clone());
                }
            });
        },
    );
}
