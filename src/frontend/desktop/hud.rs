//! HUD strip below the 160×144 game layer (`LCD_HEIGHT..`).

use crate::ppu::LCD_HEIGHT;

const SHELL_BG: [u8; 4] = [18, 18, 20, 245];
const SHELL_SURFACE: [u8; 4] = [44, 42, 57, 255];
const SHELL_BORDER: [u8; 4] = [84, 79, 96, 255];
const SHELL_FG: [u8; 4] = [243, 242, 247, 255];
const SHELL_MUTED: [u8; 4] = [162, 155, 178, 255];
const SHELL_SHADOW: [u8; 4] = [12, 10, 14, 255];
const SUCCESS_BG: [u8; 4] = [42, 74, 54, 255];
const SUCCESS_BORDER: [u8; 4] = [84, 160, 104, 255];
const WARNING_BG: [u8; 4] = [74, 58, 30, 255];
const WARNING_BORDER: [u8; 4] = [168, 130, 70, 255];
const FF_ACTIVE_BG: [u8; 4] = [200, 55, 115, 255];
const FF_ACTIVE_FG: [u8; 4] = [255, 245, 248, 255];

const HUD_MAX_W: usize = 320;

pub const HUD_STRIP_HEIGHT: u32 = 52;

#[inline]
pub const fn framebuffer_height() -> u32 {
    LCD_HEIGHT as u32 + HUD_STRIP_HEIGHT
}

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

    let bar_h = 34usize.min(buffer_height.saturating_sub(strip_top));
    let bar_y = buffer_height.saturating_sub(bar_h);
    debug_assert!(
        bar_y >= strip_top,
        "HUD strip too small — raise HUD_STRIP_HEIGHT"
    );

    fill_rect(
        frame,
        width,
        buffer_height,
        0,
        strip_top,
        w,
        buffer_height.saturating_sub(strip_top),
        SHELL_BG,
    );
    fill_rect(
        frame,
        width,
        buffer_height,
        0,
        strip_top,
        w,
        1,
        SHELL_BORDER,
    );

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
    let ff = if fast_forward_held { "ON" } else { "OFF" };
    let autosave = if autosave_enabled {
        "AUTO ON"
    } else {
        "AUTO OFF"
    };

    fill_rect(frame, width, buffer_height, 0, bar_y, w, bar_h, SHELL_BG);
    fill_rect(frame, width, buffer_height, 0, bar_y, w, 1, SHELL_BORDER);

    let r1 = bar_y + 3;
    let chip_h = 11;
    draw_chip(
        frame,
        width,
        buffer_height,
        3,
        r1,
        50,
        chip_h,
        chip_bg,
        chip_border,
        "F5 SAVE",
        fg,
        shadow,
    );
    draw_chip(
        frame,
        width,
        buffer_height,
        55,
        r1,
        50,
        chip_h,
        chip_bg,
        chip_border,
        "F9 LOAD",
        fg,
        shadow,
    );
    draw_chip(
        frame,
        width,
        buffer_height,
        107,
        r1,
        50,
        chip_h,
        chip_bg,
        chip_border,
        "ESC",
        fg,
        shadow,
    );

    let r2 = bar_y + 3 + chip_h + 2;
    let ff_label = format!("SPC {ff}");
    draw_chip(
        frame,
        width,
        buffer_height,
        3,
        r2,
        76,
        chip_h,
        ff_chip_bg,
        chip_border,
        &ff_label,
        ff_text,
        shadow,
    );
    draw_chip(
        frame,
        width,
        buffer_height,
        81,
        r2,
        76,
        chip_h,
        if autosave_enabled {
            SUCCESS_BG
        } else {
            WARNING_BG
        },
        if autosave_enabled {
            SUCCESS_BORDER
        } else {
            WARNING_BORDER
        },
        autosave,
        fg,
        shadow,
    );

    let hint = match (rendered_frames / 120) % 4 {
        0 => "F12 SHOT",
        1 => "F2  ROM",
        2 => "TAB HUD",
        _ => "F6 FOLDER",
    };

    let status = truncate_text(&hud_ascii(status_line), (w.saturating_sub(6)) / 4);
    let status_y = strip_top + 2;
    let hint_y = strip_top + 12;
    let hint_y = hint_y.min(bar_y.saturating_sub(8));

    draw_text(
        frame,
        width,
        buffer_height,
        3,
        status_y,
        &status,
        muted,
        shadow,
        1,
    );
    if hint_y > strip_top && hint_y + 6 < bar_y {
        draw_text(
            frame,
            width,
            buffer_height,
            3,
            hint_y,
            hint,
            muted,
            shadow,
            1,
        );
    }
}

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
