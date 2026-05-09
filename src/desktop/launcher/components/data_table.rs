//! Shared table layout primitives (spacing from [`crate::desktop::launcher::theme`] tokens).

use eframe::egui;

use crate::desktop::launcher::components::tokens::surface_3_fill;
use crate::desktop::launcher::theme::{radius_sm, space_1, space_2, space_3};

/// Vertical rhythm inside header/data cells (spacing tokens, matches section cards).
pub(crate) fn table_cell_margin() -> egui::Margin {
    egui::Margin::symmetric(space_2(), space_1())
}

/// Inner width for row cells inside [`table_header_shell`] / [`table_data_row_shell`].
/// Callers must use this instead of [`egui::Ui::available_width`] so header/body share one width.
pub(crate) fn table_shell_inner_width(content_column_width: f32) -> f32 {
    let m = table_cell_margin();
    (content_column_width - m.left - m.right).max(80.0)
}

/// Horizontal inset for a table block inside a scroll/card.
pub(crate) fn table_horizontal_inset() -> f32 {
    space_3()
}

#[derive(Clone, Copy)]
pub(crate) enum TableCellAlign {
    Left,
    Center,
    Right,
}

pub(crate) fn table_column_gap(ui: &mut egui::Ui) {
    ui.add_space(space_2());
}

pub(crate) fn table_cell<R>(
    ui: &mut egui::Ui,
    w: f32,
    h: f32,
    align: TableCellAlign,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let layout = match align {
        TableCellAlign::Left => egui::Layout::top_down(egui::Align::LEFT),
        TableCellAlign::Center => egui::Layout::top_down(egui::Align::Center),
        TableCellAlign::Right => egui::Layout::top_down(egui::Align::RIGHT),
    };
    ui.allocate_ui_with_layout(egui::vec2(w, h), layout, |ui| {
        let rect = ui.max_rect();
        ui.set_clip_rect(rect);
        content(ui)
    })
    .inner
}

/// Centers table content with [`table_horizontal_inset`] padding left/right.
pub(crate) fn table_inset_strip<R>(
    ui: &mut egui::Ui,
    avail: f32,
    add_contents: impl FnOnce(&mut egui::Ui, f32) -> R,
) -> R {
    let side = table_horizontal_inset();
    let content_w = (avail - 2.0 * side).max(200.0);
    ui.horizontal(|ui| {
        ui.add_space(side);
        let inner = ui.vertical(|ui| {
            ui.set_width(content_w);
            ui.spacing_mut().item_spacing.y = space_1();
            add_contents(ui, content_w)
        });
        ui.add_space(side);
        inner.inner
    })
    .inner
}

pub(crate) fn table_header_shell(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(egui::Color32::TRANSPARENT)
        .inner_margin(table_cell_margin())
        .show(ui, add_contents);
}

pub(crate) fn table_data_row_shell(
    ui: &mut egui::Ui,
    selected: bool,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::none()
        .fill(if selected {
            surface_3_fill()
        } else {
            egui::Color32::TRANSPARENT
        })
        .rounding(radius_sm())
        .inner_margin(table_cell_margin())
        .show(ui, add_contents);
}
