//! Standard section headings (title + caption + optional trailing slot).

use eframe::egui;

use crate::desktop::launcher::components::tokens::text_caption;
use crate::desktop::launcher::theme::space_2;

/// H3-style title, muted caption, optional right-aligned trailing content (counts, selection summary).
pub(crate) fn section_title_bar(
    ui: &mut egui::Ui,
    title: egui::RichText,
    caption: impl Into<String>,
    trailing: impl FnOnce(&mut egui::Ui),
) {
    ui.horizontal(|ui| {
        ui.label(title);
        ui.separator();
        ui.small(text_caption(caption.into()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), trailing);
    });
    ui.add_space(space_2());
}
