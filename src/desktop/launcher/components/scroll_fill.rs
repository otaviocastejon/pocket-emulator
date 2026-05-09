//! Single policy for vertical scroll regions that consume remaining panel height.

use eframe::egui;

/// Minimum scroll viewport height so tiny windows still scroll meaningfully.
pub(crate) const FILL_SCROLL_MIN_HEIGHT: f32 = 120.0;

/// Vertical [`ScrollArea`] sized to [`egui::Ui::available_height`] (floored).
pub(crate) fn fill_vertical_scroll<R>(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let h = ui.available_height().max(FILL_SCROLL_MIN_HEIGHT);
    egui::ScrollArea::vertical()
        .id_source(id_source)
        .max_height(h)
        .auto_shrink([false, false])
        .show(ui, add_contents)
        .inner
}
