use eframe::egui;

use super::tokens::{
    border_color, destructive_bg, destructive_text, info_bg, info_text, platform_bg, platform_text,
    success_bg, success_text, warning_bg, warning_text,
};
use crate::desktop::launcher::theme::{radius_sm, space_1, space_2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BadgeTone {
    Success,
    Warning,
    Destructive,
    Info,
    Platform,
}

pub(crate) fn status_badge(ui: &mut egui::Ui, label: &str, tone: BadgeTone) {
    let (bg, fg) = match tone {
        BadgeTone::Success => (success_bg(), success_text()),
        BadgeTone::Warning => (warning_bg(), warning_text()),
        BadgeTone::Destructive => (destructive_bg(), destructive_text()),
        BadgeTone::Info => (info_bg(), info_text()),
        BadgeTone::Platform => (platform_bg(), platform_text()),
    };
    egui::Frame::none()
        .fill(bg)
        .rounding(egui::Rounding::same(radius_sm()))
        .stroke(egui::Stroke::new(1.0, border_color()))
        .inner_margin(egui::Margin::symmetric(space_2(), space_1()))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(label).size(11.0).color(fg).monospace());
        });
}
