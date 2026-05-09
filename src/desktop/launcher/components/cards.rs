use eframe::egui;

pub(crate) fn section_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    crate::desktop::launcher::theme::section_card(ui, add_contents);
}
