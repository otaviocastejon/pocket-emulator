use eframe::egui;

use super::tokens::{primary_color, success_text};
use crate::desktop::launcher::theme::{primary_hover_color, radius_sm, space_2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Ghost / Icon reserved for compact toolbars
pub(crate) enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum ButtonSize {
    Sm,
    Md,
    Lg,
    Icon,
}

pub(crate) fn action_button(
    ui: &mut egui::Ui,
    label: &str,
    variant: ButtonVariant,
    size: ButtonSize,
) -> egui::Response {
    let (w, h) = match size {
        ButtonSize::Sm => (88.0, 26.0),
        ButtonSize::Md => (112.0, 30.0),
        ButtonSize::Lg => (132.0, 34.0),
        ButtonSize::Icon => (30.0, 30.0),
    };
    if matches!(variant, ButtonVariant::Primary) {
        let desired = egui::vec2(w, h);
        let rounding = egui::Rounding::same(radius_sm());
        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
        let fill = if response.hovered() || response.clicked() {
            primary_hover_color()
        } else {
            primary_color()
        };
        let stroke = egui::Stroke::new(
            1.0,
            if response.hovered() {
                primary_color()
            } else {
                primary_hover_color()
            },
        );
        ui.painter().rect(rect, rounding, fill, stroke);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::TextStyle::Button.resolve(ui.style()),
            egui::Color32::WHITE,
        );
        return response;
    }
    let (fill, fg, stroke) = match variant {
        ButtonVariant::Primary => unreachable!(),
        // Use the theme border stroke so secondary actions read as real buttons (not flat text).
        ButtonVariant::Secondary => {
            let w = &ui.visuals().widgets.inactive;
            (w.bg_fill, w.fg_stroke.color, w.bg_stroke)
        }
        ButtonVariant::Success => (
            success_text(),
            ui.visuals().extreme_bg_color,
            egui::Stroke::new(1.0, success_text()),
        ),
        ButtonVariant::Ghost => (
            egui::Color32::TRANSPARENT,
            ui.visuals().widgets.inactive.fg_stroke.color,
            egui::Stroke::new(1.0, ui.visuals().widgets.inactive.bg_stroke.color),
        ),
    };
    ui.add_sized(
        [w, h],
        egui::Button::new(egui::RichText::new(label).color(fg))
            .fill(fill)
            .stroke(stroke)
            .rounding(egui::Rounding::same(radius_sm())),
    )
}

/// Horizontal row of actions with design-system spacing ([`crate::desktop::launcher::theme::space_2`]).
pub(crate) fn action_button_row(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = space_2();
        add_contents(ui);
    });
}
