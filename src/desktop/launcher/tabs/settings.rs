use eframe::egui;
use pocketemulator::storage::{AudioMode, VideoFilter};

use super::super::LauncherApp;
use super::common::key_picker;
use crate::desktop::launcher::components::badges::{status_badge, BadgeTone};
use crate::desktop::launcher::components::buttons::{
    action_button, action_button_row, ButtonSize, ButtonVariant,
};
use crate::desktop::launcher::components::cards::section_card;
use crate::desktop::launcher::components::form_controls::{
    keybind_row, segmented_row, settings_section_rule, slider_row, toggle_row,
};
use crate::desktop::launcher::components::page_headers::section_title_bar;
use crate::desktop::launcher::components::toasts::ToastTone;
use crate::desktop::launcher::components::tokens::{
    info_text, primary_color, text_caption, text_h3, text_mono,
};
use crate::desktop::launcher::theme::{space_2, space_3, space_4};

pub(crate) fn render_settings_tab(app: &mut LauncherApp, ui: &mut egui::Ui) {
    section_card(ui, |ui| {
        section_title_bar(
            ui,
            egui::RichText::new("Settings")
                .strong()
                .color(primary_color()),
            "Profile-scoped runtime and controls",
            |ui| {
                let selected = app
                    .selected_rom()
                    .and_then(|p| p.file_name().and_then(|s| s.to_str()))
                    .unwrap_or("No game selected");
                ui.small(text_caption(selected));
            },
        );
    });
    ui.separator();
    ui.add_space(space_4());
    ui.horizontal_top(|ui| {
        let gap = space_3();
        let col_w = ((ui.available_width() - gap).max(0.0)) / 2.0;
        ui.vertical(|ui| {
            ui.set_min_width(col_w);
            ui.set_max_width(col_w);
            section_card(ui, |ui| {
                ui.label(text_h3("Runtime").color(primary_color()));
                ui.small(text_caption("Per-game visual and performance settings"));
                ui.add_space(space_3() - 2.0);
                settings_section_rule(ui);
                toggle_row(
                    ui,
                    "Autosave",
                    "Every 30 seconds while playing",
                    &mut app.autosave_enabled,
                );
                settings_section_rule(ui);
                segmented_row_video_filter(ui, &mut app.selected_video_filter);
                settings_section_rule(ui);
                slider_row(ui, "Display scale", &mut app.scale, 1..=10);
                settings_section_rule(ui);
                segmented_row_audio_mode(ui, &mut app.selected_audio_mode);
            });
        });
        ui.separator();
        ui.vertical(|ui| {
            ui.set_min_width(col_w);
            ui.set_max_width(col_w);
            section_card(ui, |ui| {
                ui.label(text_h3("Keybinds").color(primary_color()));
                ui.small(text_caption("Customize all core controls and fast-forward"));
                ui.add_space(space_3() - 2.0);
                settings_section_rule(ui);
                keybind_row(ui, "A button", |ui| {
                    key_picker(ui, &mut app.controls.a, "bind_a")
                });
                settings_section_rule(ui);
                keybind_row(ui, "B button", |ui| {
                    key_picker(ui, &mut app.controls.b, "bind_b")
                });
                settings_section_rule(ui);
                keybind_row(ui, "Start", |ui| {
                    key_picker(ui, &mut app.controls.start, "bind_start")
                });
                settings_section_rule(ui);
                keybind_row(ui, "Select", |ui| {
                    key_picker(ui, &mut app.controls.select, "bind_select")
                });
                settings_section_rule(ui);
                keybind_row(ui, "Up", |ui| {
                    key_picker(ui, &mut app.controls.up, "bind_up")
                });
                settings_section_rule(ui);
                keybind_row(ui, "Down", |ui| {
                    key_picker(ui, &mut app.controls.down, "bind_down")
                });
                settings_section_rule(ui);
                keybind_row(ui, "Left", |ui| {
                    key_picker(ui, &mut app.controls.left, "bind_left")
                });
                settings_section_rule(ui);
                keybind_row(ui, "Right", |ui| {
                    key_picker(ui, &mut app.controls.right, "bind_right")
                });
                settings_section_rule(ui);
                keybind_row(ui, "Fast forward", |ui| {
                    key_picker(ui, &mut app.controls.fast_forward, "bind_ff")
                });
            });
        });
    });
    ui.add_space(space_4());
    section_card(ui, |ui| {
        action_button_row(ui, |ui| {
            if action_button(
                ui,
                "Reset controls",
                ButtonVariant::Secondary,
                ButtonSize::Lg,
            )
            .clicked()
            {
                app.controls = pocketemulator::frontend::Controls::default();
                app.push_toast(
                    "Controls reset".to_string(),
                    "Default keybinds restored".to_string(),
                    ToastTone::Info,
                    None,
                    3.0,
                );
            }
            if action_button(
                ui,
                "Reset defaults",
                ButtonVariant::Secondary,
                ButtonSize::Lg,
            )
            .clicked()
            {
                app.scale = 4;
                app.autosave_enabled = true;
                app.selected_video_filter = VideoFilter::Sharp;
                app.selected_audio_mode = AudioMode::Balanced;
                app.push_toast(
                    "Runtime defaults restored".to_string(),
                    "Scale, autosave, video and audio reset".to_string(),
                    ToastTone::Warning,
                    None,
                    3.0,
                );
            }
        });
        ui.add_space(space_2());
        ui.small(
            egui::RichText::new(
                "Settings are saved with the game profile when you launch from the library.",
            )
            .color(info_text()),
        );
        if let Some(path) = app.selected_rom() {
            ui.horizontal(|ui| {
                status_badge(ui, "INFO", BadgeTone::Info);
                ui.small(text_mono(path.display().to_string()));
            });
        }
    });
}

fn segmented_row_video_filter(ui: &mut egui::Ui, value: &mut VideoFilter) {
    let options = [
        ("DMG", *value == VideoFilter::Sharp),
        ("Pocket", *value == VideoFilter::Smooth),
    ];
    segmented_row(ui, "Color filter", &options, &mut |idx| {
        *value = if idx == 0 {
            VideoFilter::Sharp
        } else {
            VideoFilter::Smooth
        };
    });
}

fn segmented_row_audio_mode(ui: &mut egui::Ui, mode: &mut AudioMode) {
    let options = [
        ("Mono", *mode == AudioMode::LowLatency),
        ("Stereo", *mode == AudioMode::Balanced),
    ];
    segmented_row(ui, "Audio mode", &options, &mut |idx| {
        *mode = if idx == 0 {
            AudioMode::LowLatency
        } else {
            AudioMode::Balanced
        };
    });
}
