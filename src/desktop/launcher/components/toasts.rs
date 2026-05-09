use eframe::egui;

use crate::desktop::launcher::components::badges::{status_badge, BadgeTone};
use crate::desktop::launcher::components::buttons::{action_button, ButtonSize, ButtonVariant};
use crate::desktop::launcher::components::cards::section_card;
use crate::desktop::launcher::components::tokens::{
    destructive_text, info_text, success_text, warning_text,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToastTone {
    Success,
    Warning,
    Destructive,
    Info,
}

#[derive(Debug, Clone)]
pub(crate) struct ToastMessage {
    pub id: u64,
    pub title: String,
    pub detail: String,
    pub tone: ToastTone,
    pub action_label: Option<String>,
    /// TTL (seconds) applied once on next toast draw — converted to absolute `expires_at`.
    pub ttl_pending: Option<f64>,
    pub expires_at: f64,
}

pub(crate) fn show_toasts(
    ctx: &egui::Context,
    toasts: &mut Vec<ToastMessage>,
    mut on_action: impl FnMut(u64),
) {
    let now = ctx.input(|i| i.time);
    for t in toasts.iter_mut() {
        if let Some(ttl) = t.ttl_pending.take() {
            t.expires_at = now + ttl.max(0.25);
        }
    }
    toasts.retain(|t| t.expires_at > now);
    let max = 3usize;
    let visible: Vec<_> = toasts.iter().rev().take(max).cloned().collect();
    for (index, toast) in visible.into_iter().enumerate() {
        let y = -14.0 - index as f32 * 86.0;
        egui::Area::new(egui::Id::new(format!("toast_{}", toast.id)))
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-14.0, y))
            .interactable(true)
            .show(ctx, |ui| {
                section_card(ui, |ui| {
                    ui.set_width(420.0);
                    ui.horizontal(|ui| {
                        let (badge, color) = match toast.tone {
                            ToastTone::Success => (("Saved", BadgeTone::Success), success_text()),
                            ToastTone::Warning => (("Unsaved", BadgeTone::Warning), warning_text()),
                            ToastTone::Destructive => {
                                (("Save error", BadgeTone::Destructive), destructive_text())
                            }
                            ToastTone::Info => (("Tip", BadgeTone::Info), info_text()),
                        };
                        status_badge(ui, badge.0, badge.1);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(toast.title).strong().color(color));
                            ui.small(
                                egui::RichText::new(toast.detail)
                                    .color(ui.visuals().weak_text_color()),
                            );
                        });
                        if let Some(action) = &toast.action_label {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if action_button(
                                        ui,
                                        action,
                                        ButtonVariant::Secondary,
                                        ButtonSize::Md,
                                    )
                                    .clicked()
                                    {
                                        on_action(toast.id);
                                    }
                                },
                            );
                        }
                    });
                });
            });
    }
}
