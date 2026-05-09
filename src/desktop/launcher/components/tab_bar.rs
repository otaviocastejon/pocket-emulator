use eframe::egui;

use crate::desktop::launcher::components::tokens::{muted_text, text_caption};
use crate::desktop::launcher::theme::{
    border_color, primary_color, primary_hover_color, radius_lg, radius_sm, surface_2_fill,
    surface_fill,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TabIcon {
    Gamepad,
    Globe,
    Clock,
    Gear,
}

pub(crate) fn tab_button(
    ui: &mut egui::Ui,
    selected: bool,
    icon: TabIcon,
    title: &str,
) -> egui::Response {
    let enabled = ui.is_enabled();
    let selected = selected && enabled;
    let w = ui.available_width().clamp(100.0, 260.0);
    let desired = egui::vec2(w, 36.0);
    let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
    let rounding = if selected {
        egui::Rounding::same(radius_lg())
    } else {
        egui::Rounding::same(radius_sm())
    };
    let is_hovered = enabled && response.hovered();
    let hover_t = ui
        .ctx()
        .animate_bool(response.id.with("hover_anim"), is_hovered);
    let idle = surface_fill();
    let hover_bg = surface_2_fill();
    let mut bg = if enabled {
        lerp_color(idle, hover_bg, hover_t)
    } else {
        idle
    };
    if selected {
        bg = primary_color();
    }
    let stroke = if selected {
        egui::Stroke::new(1.0, primary_hover_color())
    } else if enabled {
        let c = lerp_color(border_color(), primary_hover_color(), hover_t * 0.35);
        egui::Stroke::new(1.0, c)
    } else {
        egui::Stroke::new(1.0, border_color().gamma_multiply(0.5))
    };
    ui.painter().rect(rect, rounding, bg, stroke);

    let text_color = if selected {
        egui::Color32::WHITE
    } else if enabled {
        lerp_color(muted_text(), ui.visuals().text_color(), hover_t)
    } else {
        muted_text().gamma_multiply(0.65)
    };
    let icon_rect = egui::Rect::from_min_size(
        egui::pos2(rect.min.x + 10.0, rect.center().y - 7.0),
        egui::vec2(14.0, 14.0),
    );
    draw_icon(ui.painter(), icon_rect, icon, text_color);
    ui.painter().text(
        egui::pos2(rect.min.x + 30.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        egui::TextStyle::Button.resolve(ui.style()),
        text_color,
    );
    response
}

/// LIBRARY subsection: folder icon + muted label (placeholders until wired).
pub(crate) fn library_folder_row(ui: &mut egui::Ui, label: &str) {
    let w = ui.available_width().clamp(100.0, 260.0);
    ui.horizontal(|ui| {
        ui.set_max_width(w);
        let (icon_rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
        draw_folder_glyph(ui.painter(), icon_rect, muted_text());
        ui.label(text_caption(label));
    });
}

fn draw_folder_glyph(painter: &egui::Painter, r: egui::Rect, color: egui::Color32) {
    let tab_h = r.height() * 0.28;
    let body = egui::Rect::from_min_max(egui::pos2(r.min.x, r.min.y + tab_h * 0.5), r.max);
    painter.rect_stroke(body, 2.0, egui::Stroke::new(1.0, color));
    let tab = egui::Rect::from_min_max(
        r.min,
        egui::pos2(r.min.x + r.width() * 0.45, r.min.y + tab_h + 1.0),
    );
    painter.rect_stroke(tab, 1.5, egui::Stroke::new(1.0, color));
}

fn lerp_color(a: egui::Color32, b: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    let [ar, ag, ab, aa] = a.to_array();
    let [br, bg, bb, ba] = b.to_array();
    egui::Color32::from_rgba_unmultiplied(
        ((ar as f32) + (br as f32 - ar as f32) * t) as u8,
        ((ag as f32) + (bg as f32 - ag as f32) * t) as u8,
        ((ab as f32) + (bb as f32 - ab as f32) * t) as u8,
        ((aa as f32) + (ba as f32 - aa as f32) * t) as u8,
    )
}

fn draw_icon(painter: &egui::Painter, rect: egui::Rect, icon: TabIcon, color: egui::Color32) {
    match icon {
        TabIcon::Gamepad => {
            painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.5, color));
            painter.line_segment(
                [
                    egui::pos2(rect.left() + 4.0, rect.center().y),
                    egui::pos2(rect.left() + 8.0, rect.center().y),
                ],
                egui::Stroke::new(1.5, color),
            );
            painter.line_segment(
                [
                    egui::pos2(rect.left() + 6.0, rect.center().y - 2.0),
                    egui::pos2(rect.left() + 6.0, rect.center().y + 2.0),
                ],
                egui::Stroke::new(1.5, color),
            );
            painter.circle_filled(
                egui::pos2(rect.right() - 5.0, rect.center().y - 2.0),
                1.2,
                color,
            );
            painter.circle_filled(
                egui::pos2(rect.right() - 3.0, rect.center().y + 1.5),
                1.2,
                color,
            );
        }
        TabIcon::Globe => {
            painter.circle_stroke(rect.center(), 7.0, egui::Stroke::new(1.4, color));
            painter.circle_stroke(rect.center(), 3.0, egui::Stroke::new(1.0, color));
            painter.line_segment(
                [
                    egui::pos2(rect.left() + 3.0, rect.center().y),
                    egui::pos2(rect.right() - 3.0, rect.center().y),
                ],
                egui::Stroke::new(1.0, color),
            );
            painter.line_segment(
                [
                    egui::pos2(rect.center().x, rect.top() + 3.0),
                    egui::pos2(rect.center().x, rect.bottom() - 3.0),
                ],
                egui::Stroke::new(1.0, color),
            );
        }
        TabIcon::Clock => {
            painter.circle_stroke(rect.center(), 6.0, egui::Stroke::new(1.5, color));
            painter.line_segment(
                [
                    rect.center(),
                    egui::pos2(rect.center().x, rect.center().y - 3.0),
                ],
                egui::Stroke::new(1.5, color),
            );
            painter.line_segment(
                [
                    rect.center(),
                    egui::pos2(rect.center().x + 3.0, rect.center().y + 1.5),
                ],
                egui::Stroke::new(1.5, color),
            );
        }
        TabIcon::Gear => {
            painter.circle_stroke(rect.center(), 5.0, egui::Stroke::new(1.5, color));
            painter.circle_filled(rect.center(), 1.4, color);
            for i in 0..6 {
                let a = i as f32 * std::f32::consts::TAU / 6.0;
                let p1 = egui::pos2(
                    rect.center().x + a.cos() * 6.0,
                    rect.center().y + a.sin() * 6.0,
                );
                let p2 = egui::pos2(
                    rect.center().x + a.cos() * 7.5,
                    rect.center().y + a.sin() * 7.5,
                );
                painter.line_segment([p1, p2], egui::Stroke::new(1.2, color));
            }
        }
    }
}
