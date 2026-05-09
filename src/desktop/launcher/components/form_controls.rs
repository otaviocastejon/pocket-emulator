use eframe::egui;

use super::buttons::{action_button, ButtonSize, ButtonVariant};
use super::tokens::{text_body, text_caption};
use crate::desktop::launcher::theme::{radius_sm, space_2, space_3};

pub(crate) fn control_row_frame(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(ui.visuals().widgets.inactive.bg_fill)
        .rounding(egui::Rounding::same(radius_sm()))
        .inner_margin(egui::Margin::same(space_3() + 2.0))
        .show(ui, |ui| add_contents(ui));
}

pub(crate) fn toggle_row(ui: &mut egui::Ui, title: &str, subtitle: &str, value: &mut bool) {
    control_row_frame(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(text_body(title));
                ui.small(text_caption(subtitle));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let label = if *value { "ON" } else { "OFF" };
                if action_button(ui, label, ButtonVariant::Success, ButtonSize::Sm).clicked() {
                    *value = !*value;
                }
            });
        });
    });
    ui.add_space(space_2());
}

pub(crate) fn segmented_row(
    ui: &mut egui::Ui,
    label: &str,
    options: &[(&str, bool)],
    on_select: &mut dyn FnMut(usize),
) {
    control_row_frame(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(text_body(label));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                for (idx, (name, selected)) in options.iter().enumerate().rev() {
                    if ui.selectable_label(*selected, *name).clicked() {
                        on_select(idx);
                    }
                }
            });
        });
    });
    ui.add_space(space_2());
}

pub(crate) fn slider_row(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut u32,
    range: std::ops::RangeInclusive<u32>,
) {
    control_row_frame(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(text_body(label));
            ui.add(
                egui::Slider::new(value, range)
                    .suffix("x")
                    .show_value(false),
            );
            ui.small(text_caption(format!("{value}x")));
        });
    });
    ui.add_space(space_2());
}

/// Separator between stacked settings controls (vertical rhythm).
pub(crate) fn settings_section_rule(ui: &mut egui::Ui) {
    ui.separator();
    ui.add_space(space_2());
}

pub(crate) fn keybind_row(
    ui: &mut egui::Ui,
    label: &str,
    add_binding_control: impl FnOnce(&mut egui::Ui),
) {
    control_row_frame(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(text_body(label));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                add_binding_control(ui);
            });
        });
    });
    ui.add_space(space_2());
}
