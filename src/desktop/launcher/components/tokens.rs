use eframe::egui;

pub(crate) fn surface_3_fill() -> egui::Color32 {
    crate::desktop::launcher::theme::surface_3_fill()
}

pub(crate) fn border_color() -> egui::Color32 {
    crate::desktop::launcher::theme::border_color()
}

pub(crate) fn primary_color() -> egui::Color32 {
    crate::desktop::launcher::theme::primary_color()
}

pub(crate) fn muted_text() -> egui::Color32 {
    crate::desktop::launcher::theme::muted_text()
}

pub(crate) fn success_text() -> egui::Color32 {
    crate::desktop::launcher::theme::success_text()
}

pub(crate) fn warning_text() -> egui::Color32 {
    crate::desktop::launcher::theme::warning_text()
}

pub(crate) fn destructive_text() -> egui::Color32 {
    crate::desktop::launcher::theme::destructive_text()
}

pub(crate) fn info_text() -> egui::Color32 {
    crate::desktop::launcher::theme::info_text()
}

pub(crate) fn success_bg() -> egui::Color32 {
    crate::desktop::launcher::theme::success_bg()
}

pub(crate) fn warning_bg() -> egui::Color32 {
    crate::desktop::launcher::theme::warning_bg()
}

pub(crate) fn destructive_bg() -> egui::Color32 {
    crate::desktop::launcher::theme::destructive_bg()
}

pub(crate) fn info_bg() -> egui::Color32 {
    crate::desktop::launcher::theme::info_bg()
}

pub(crate) fn platform_bg() -> egui::Color32 {
    crate::desktop::launcher::theme::platform_bg()
}

pub(crate) fn platform_text() -> egui::Color32 {
    crate::desktop::launcher::theme::platform_text()
}

pub(crate) fn text_display(s: impl Into<String>) -> egui::RichText {
    crate::desktop::launcher::theme::text_display(s)
}

pub(crate) fn text_h3(s: impl Into<String>) -> egui::RichText {
    crate::desktop::launcher::theme::text_h3(s)
}

pub(crate) fn text_body(s: impl Into<String>) -> egui::RichText {
    crate::desktop::launcher::theme::text_body(s)
}

pub(crate) fn text_caption(s: impl Into<String>) -> egui::RichText {
    crate::desktop::launcher::theme::text_caption(s)
}

pub(crate) fn text_eyebrow(s: impl Into<String>) -> egui::RichText {
    crate::desktop::launcher::theme::text_eyebrow(s)
}

pub(crate) fn text_mono(s: impl Into<String>) -> egui::RichText {
    crate::desktop::launcher::theme::text_mono(s)
}
