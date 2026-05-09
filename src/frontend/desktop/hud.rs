//! Controls chrome drawn **below** the Game Boy picture (rows `LCD_HEIGHT..`).
//! The pixel buffer height is `LCD_HEIGHT + HUD_STRIP_HEIGHT` so gameplay is never covered.

use crate::ppu::LCD_HEIGHT;

const SHELL_BG: [u8; 4] = [18, 18, 20, 245];
const SHELL_SURFACE: [u8; 4] = [44, 42, 57, 255];
const SHELL_BORDER: [u8; 4] = [84, 79, 96, 255];
const SHELL_FG: [u8; 4] = [243, 242, 247, 255];
const SHELL_MUTED: [u8; 4] = [162, 155, 178, 255];
const SHELL_SHADOW: [u8; 4] = [12, 10, 14, 255];
const SUCCESS_BG: [u8; 4] = [42, 74, 54, 255];
const SUCCESS_BORDER: [u8; 4] = [84, 160, 104, 255];
/// Fast-forward “on” chip — matches launcher primary pink accent.
const FF_ACTIVE_BG: [u8; 4] = [200, 55, 115, 255];
const FF_ACTIVE_FG: [u8; 4] = [255, 245, 248, 255];

/// HUD chrome clips to this width (DMG 160px).
const HUD_MAX_W: usize = 320;

/// Vertical space reserved under the 160×144 game layer for dock + status.
pub const HUD_STRIP_HEIGHT: u32 = 76;

#[inline]
pub const fn framebuffer_height() -> u32 {
    LCD_HEIGHT as u32 + HUD_STRIP_HEIGHT
}

/// Paint the HUD strip when controls are hidden (solid bar under the game, no chrome).
pub fn clear_hud_strip(frame: &mut [u8], width: usize, buffer_height: usize, game_height: usize) {
    if width == 0 || buffer_height <= game_height {
        return;
    }
    let h = buffer_height.saturating_sub(game_height);
    fill_rect(
        frame,
        width,
        buffer_height,
        0,
        game_height,
        width.min(HUD_MAX_W),
        h,
        [18, 18, 20, 255],
    );
}

#[allow(clippy::too_many_arguments)]
pub fn draw_controls_hud(
    frame: &mut [u8],
    width: usize,
    buffer_height: usize,
    gameplay_height: usize,
    fast_forward_held: bool,
    rendered_frames: u64,
    autosave_enabled: bool,
    status_line: &str,
) {
    if width == 0 || buffer_height == 0 {
        return;
    }
    let game_height = gameplay_height;
    if buffer_height <= game_height {
        return;
    }

    let w = width.min(HUD_MAX_W);
    let strip_top = game_height;
    apply_screen_fx(frame, width, game_height, w, rendered_frames);

    // Top title bar (inside the pixel scene for a custom-window look).
    fill_rect(frame, width, buffer_height, 0, 0, w, 12, [24, 26, 38, 255]);
    fill_rect(frame, width, buffer_height, 0, 11, w, 1, SHELL_BORDER);
    fill_rect(frame, width, buffer_height, 4, 4, 3, 3, [232, 72, 66, 255]);
    fill_rect(frame, width, buffer_height, 9, 4, 3, 3, [225, 185, 70, 255]);
    fill_rect(frame, width, buffer_height, 14, 4, 3, 3, [82, 192, 105, 255]);
    draw_text(
        frame,
        width,
        buffer_height,
        24,
        4,
        "NOW PLAYING - POCKETEMULATOR",
        SHELL_MUTED,
        SHELL_SHADOW,
        1,
    );

    // Bezel around game viewport.
    fill_rect(frame, width, buffer_height, 0, 12, w, 1, SHELL_BORDER);
    fill_rect(frame, width, buffer_height, 0, game_height.saturating_sub(1), w, 1, SHELL_BORDER);
    fill_rect(frame, width, buffer_height, 0, 12, 1, game_height.saturating_sub(11), SHELL_BORDER);
    fill_rect(
        frame,
        width,
        buffer_height,
        w.saturating_sub(1),
        12,
        1,
        game_height.saturating_sub(11),
        SHELL_BORDER,
    );

    // Full shell under gameplay.
    fill_rect(frame, width, buffer_height, 0, strip_top, w, buffer_height.saturating_sub(strip_top), SHELL_BG);
    fill_rect(frame, width, buffer_height, 0, strip_top, w, 1, SHELL_BORDER);

    let fg = SHELL_FG;
    let muted = SHELL_MUTED;
    let shadow = SHELL_SHADOW;
    let chip_bg = SHELL_SURFACE;
    let chip_border = SHELL_BORDER;
    let ff_chip_bg = if fast_forward_held {
        FF_ACTIVE_BG
    } else {
        chip_bg
    };
    let ff_text = if fast_forward_held { FF_ACTIVE_FG } else { fg };
    let ff = if fast_forward_held { "FF ON" } else { "FF OFF" };
    let autosave = if autosave_enabled { "AUTO ON" } else { "AUTO OFF" };

    // Dock panel.
    let dock_y = strip_top + 6;
    let dock_h = 30usize.min(buffer_height.saturating_sub(dock_y + 2));
    fill_rect(frame, width, buffer_height, 4, dock_y, w.saturating_sub(8), dock_h, [27, 30, 44, 255]);
    fill_rect(frame, width, buffer_height, 4, dock_y, w.saturating_sub(8), 1, SHELL_BORDER);
    fill_rect(
        frame,
        width,
        buffer_height,
        4,
        dock_y + dock_h.saturating_sub(1),
        w.saturating_sub(8),
        1,
        SHELL_BORDER,
    );

    let r1 = dock_y + 9;
    draw_chip(frame, width, buffer_height, 10, r1, 24, 12, chip_bg, chip_border, "PA", fg, shadow);
    draw_chip(frame, width, buffer_height, 38, r1, 24, 12, SUCCESS_BG, SUCCESS_BORDER, "SV", fg, shadow);
    draw_chip(frame, width, buffer_height, 66, r1, 24, 12, chip_bg, chip_border, "LD", fg, shadow);
    draw_chip(frame, width, buffer_height, 94, r1, 24, 12, ff_chip_bg, chip_border, "FF", ff_text, shadow);
    draw_chip(frame, width, buffer_height, 122, r1, 24, 12, chip_bg, chip_border, "CFG", fg, shadow);
    draw_chip(
        frame,
        width,
        buffer_height,
        150,
        r1,
        24,
        12,
        [74, 38, 42, 255],
        [160, 78, 88, 255],
        "PWR",
        [250, 98, 116, 255],
        shadow,
    );

    let status = truncate_text(&hud_ascii(status_line), (w.saturating_sub(10)) / 4);
    let status_y = dock_y + dock_h + 8;
    draw_text(frame, width, buffer_height, 8, status_y, &status, muted, shadow, 1);
    draw_text(frame, width, buffer_height, 122, status_y, autosave, muted, shadow, 1);
    draw_text(frame, width, buffer_height, 150, status_y, ff, muted, shadow, 1);
}

fn apply_screen_fx(frame: &mut [u8], width: usize, game_height: usize, w: usize, rendered_frames: u64) {
    if width == 0 || game_height == 0 {
        return;
    }
    let cx = (w / 2) as i32;
    let cy = (game_height / 2) as i32;
    let tick = (rendered_frames & 1) as usize;
    for y in 12..game_height.saturating_sub(1) {
        for x in 1..w.saturating_sub(1) {
            let i = (y * width + x) * 4;
            let mut r = frame[i] as i32;
            let mut g = frame[i + 1] as i32;
            let mut b = frame[i + 2] as i32;

            // DMG-like green tint mix.
            let luma = (r * 30 + g * 59 + b * 11) / 100;
            r = (luma * 62) / 100;
            g = (luma * 125) / 100;
            b = (luma * 52) / 100;

            // Pixel matrix texture.
            if (x + tick).is_multiple_of(2) || (y + tick).is_multiple_of(2) {
                r = (r * 92) / 100;
                g = (g * 92) / 100;
                b = (b * 92) / 100;
            }

            // Gentle vignette.
            let dx = (x as i32 - cx).abs();
            let dy = (y as i32 - cy).abs();
            let v = (dx * 2 + dy * 3) / 7;
            let shade = (v / 7).min(24);
            r = (r - shade).max(0);
            g = (g - shade).max(0);
            b = (b - shade).max(0);

            frame[i] = r as u8;
            frame[i + 1] = g as u8;
            frame[i + 2] = b as u8;
        }
    }
}

/// Replace unsupported punctuation and force ASCII for the 3×5 font.
fn hud_ascii(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '·' | '•' => '-',
            '—' | '–' => '-',
            c if c.is_ascii() => c,
            _ => ' ',
        })
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn truncate_text(s: &str, max_chars: usize) -> String {
    let max_chars = max_chars.max(8);
    let count = s.chars().count();
    if count <= max_chars {
        return s.to_string();
    }
    let take = max_chars.saturating_sub(3);
    format!("{}...", s.chars().take(take).collect::<String>())
}

fn fill_rect(
    frame: &mut [u8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: [u8; 4],
) {
    let x2 = (x + w).min(width);
    let y2 = (y + h).min(height);
    for py in y..y2 {
        for px in x..x2 {
            let i = (py * width + px) * 4;
            frame[i] = color[0];
            frame[i + 1] = color[1];
            frame[i + 2] = color[2];
            frame[i + 3] = color[3];
        }
    }
}

fn draw_text(
    frame: &mut [u8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    text: &str,
    color: [u8; 4],
    shadow_color: [u8; 4],
    scale: usize,
) {
    let mut pen_x = x;
    for ch in text.chars() {
        draw_glyph(
            frame,
            width,
            height,
            pen_x + 1,
            y + 1,
            ch,
            shadow_color,
            scale,
        );
        draw_glyph(frame, width, height, pen_x, y, ch, color, scale);
        pen_x += 3 * scale + scale;
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_chip(
    frame: &mut [u8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    bg: [u8; 4],
    border: [u8; 4],
    text: &str,
    text_color: [u8; 4],
    shadow_color: [u8; 4],
) {
    fill_rect(frame, width, height, x, y, w, h, bg);
    fill_rect(frame, width, height, x, y, w, 1, border);
    fill_rect(
        frame,
        width,
        height,
        x,
        y + h.saturating_sub(1),
        w,
        1,
        border,
    );
    fill_rect(frame, width, height, x, y, 1, h, border);
    fill_rect(
        frame,
        width,
        height,
        x + w.saturating_sub(1),
        y,
        1,
        h,
        border,
    );
    draw_text(
        frame,
        width,
        height,
        x + 3,
        y + 2,
        text,
        text_color,
        shadow_color,
        1,
    );
}

fn draw_glyph(
    frame: &mut [u8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    ch: char,
    color: [u8; 4],
    scale: usize,
) {
    let rows = glyph_rows(ch);
    for (ry, row) in rows.iter().enumerate() {
        for (rx, b) in row.as_bytes().iter().enumerate() {
            if *b == b'1' {
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x + rx * scale + sx;
                        let py = y + ry * scale + sy;
                        if px < width && py < height {
                            let i = (py * width + px) * 4;
                            frame[i] = color[0];
                            frame[i + 1] = color[1];
                            frame[i + 2] = color[2];
                            frame[i + 3] = color[3];
                        }
                    }
                }
            }
        }
    }
}

fn glyph_rows(ch: char) -> [&'static str; 5] {
    let ch = if ch.is_ascii_lowercase() {
        char::from_u32(ch as u32 - 32).unwrap_or(ch)
    } else {
        ch
    };
    match ch {
        '·' | '•' => ["000", "000", "010", "000", "000"],
        _ => match ch {
            'A' => ["010", "101", "111", "101", "101"],
            'B' => ["110", "101", "110", "101", "110"],
            'C' => ["011", "100", "100", "100", "011"],
            'D' => ["110", "101", "101", "101", "110"],
            'E' => ["111", "100", "110", "100", "111"],
            'F' => ["111", "100", "110", "100", "100"],
            'G' => ["011", "100", "101", "101", "011"],
            'H' => ["101", "101", "111", "101", "101"],
            'I' => ["111", "010", "010", "010", "111"],
            'J' => ["011", "001", "001", "101", "011"],
            'K' => ["101", "101", "110", "101", "101"],
            'L' => ["100", "100", "100", "100", "111"],
            'M' => ["101", "111", "111", "101", "101"],
            'N' => ["101", "111", "111", "111", "101"],
            'O' => ["111", "101", "101", "101", "111"],
            'P' => ["110", "101", "110", "100", "100"],
            'Q' => ["111", "101", "101", "111", "001"],
            'R' => ["110", "101", "110", "101", "101"],
            'S' => ["011", "100", "010", "001", "110"],
            'T' => ["111", "010", "010", "010", "010"],
            'U' => ["101", "101", "101", "101", "111"],
            'V' => ["101", "101", "101", "101", "010"],
            'W' => ["101", "101", "111", "111", "101"],
            'X' => ["101", "101", "010", "101", "101"],
            'Y' => ["101", "101", "010", "010", "010"],
            'Z' => ["111", "001", "010", "100", "111"],
            '0' => ["111", "101", "101", "101", "111"],
            '1' => ["010", "110", "010", "010", "111"],
            '2' => ["110", "001", "111", "100", "111"],
            '3' => ["111", "001", "011", "001", "111"],
            '4' => ["101", "101", "111", "001", "001"],
            '5' => ["111", "100", "111", "001", "111"],
            '6' => ["011", "100", "111", "101", "111"],
            '7' => ["111", "001", "010", "010", "010"],
            '8' => ["111", "101", "111", "101", "111"],
            '9' => ["111", "101", "111", "001", "110"],
            ':' => ["000", "010", "000", "010", "000"],
            '.' => ["000", "000", "000", "000", "010"],
            ',' => ["000", "000", "000", "010", "010"],
            '-' => ["000", "000", "111", "000", "000"],
            '|' => ["010", "010", "010", "010", "010"],
            '/' => ["001", "001", "010", "100", "100"],
            ' ' => ["000", "000", "000", "000", "000"],
            _ => ["000", "000", "000", "000", "000"],
        },
    }
}
