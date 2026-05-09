//! pixels + winit desktop frontend.

mod audio_output;
mod cheats;
mod framefx;
mod hud;
mod media;

use std::collections::VecDeque;
use std::process::Command;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Icon;
use winit::window::WindowBuilder;

use super::Controls;
use crate::gameboy::GameBoy;
use crate::joypad;
use crate::ppu::{LCD_HEIGHT, LCD_WIDTH};
use crate::runtime_env;
use crate::storage::{save_health_for_rom, AudioMode, VideoFilter};
use crate::ui_icon;
use audio_output::AudioOutput;
use cheats::{apply_cheats, load_cheats};
use framefx::copy_frame;
use hud::{clear_hud_strip, draw_controls_hud, framebuffer_height as hud_framebuffer_height};
use media::{save_screenshot_ppm, screenshot_output_path};

/// Open the ROM library / launcher again (`--menu`), then exit the game window.
pub(super) fn spawn_launcher_menu_process() {
    if let Ok(exe) = std::env::current_exe() {
        let _ = Command::new(exe).arg("--menu").spawn();
    }
}

const GB_W: u32 = LCD_WIDTH as u32;
const GB_H: u32 = LCD_HEIGHT as u32;
/// Includes game + dedicated HUD strip below (controls never overlap gameplay).
const FB_H: u32 = hud_framebuffer_height();
const BASE_TITLE: &str = "PocketEmulator";

pub fn run_window(
    mut gb: GameBoy,
    scale: u32,
    controls: Controls,
    autosave_enabled: bool,
    video_filter: VideoFilter,
    audio_mode: AudioMode,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let mut window_builder = WindowBuilder::new()
        .with_title(BASE_TITLE)
        .with_inner_size(LogicalSize::new(
            (GB_W * scale) as f64,
            (FB_H * scale) as f64,
        ))
        .with_min_inner_size(LogicalSize::new(GB_W as f64, FB_H as f64))
        .with_resizable(true);
    if let Some((rgba, w, h)) = ui_icon::load_icon_rgba() {
        if let Ok(icon) = Icon::from_rgba(rgba, w, h) {
            window_builder = window_builder.with_window_icon(Some(icon));
        }
    }
    let window = window_builder.build(&event_loop)?;
    let inner = window.inner_size();
    let surface_texture = SurfaceTexture::new(inner.width, inner.height, &window);
    let mut pixels = Pixels::new(GB_W, FB_H, surface_texture)?;
    let mut filtered_frame = vec![0u8; (GB_W * GB_H * 4) as usize];

    let cheats = load_cheats(gb.rom_path());
    let mut fast_forward_held = false;
    let mut frames_since_autosave: u32 = 0;
    let mut frames_since_rewind_snapshot: u32 = 0;
    let mut rewind_ram_snapshots: VecDeque<Vec<u8>> = VecDeque::new();
    let mut rendered_frames: u64 = 0;
    let frame_wait = std::time::Duration::from_secs_f64(1.0 / 59.7);
    let mut next_frame = std::time::Instant::now() + frame_wait;
    let toast_until_frame: u64 = 240;
    let launch_toast = format!(
        "mode: {} | audio: {}",
        match video_filter {
            VideoFilter::Sharp => "sharp",
            VideoFilter::Smooth => "smooth",
        },
        match audio_mode {
            AudioMode::Balanced => "balanced",
            AudioMode::LowLatency => "low-latency",
        }
    );
    let mut action_toast: Option<(String, u64)> = None;
    let mut hud_visible = true;

    let audio_out = AudioOutput::try_default();
    if audio_out.is_none() {
        log::warn!("could not open default audio output — continuing without sound");
    } else if let Some(ref ao) = audio_out {
        gb.bus.apu.set_playback_sample_rate(ao.sample_rate);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = match audio_mode {
            AudioMode::Balanced => {
                let now = std::time::Instant::now();
                if now < next_frame {
                    ControlFlow::WaitUntil(next_frame)
                } else {
                    next_frame = now + frame_wait;
                    ControlFlow::Poll
                }
            }
            AudioMode::LowLatency => ControlFlow::Poll,
        };
        match event {
            Event::RedrawRequested(_) => {
                let frames = if fast_forward_held { 6 } else { 1 };
                for _ in 0..frames {
                    gb.run_frame();
                    apply_cheats(&mut gb, &cheats);
                    frames_since_autosave = frames_since_autosave.saturating_add(1);
                    frames_since_rewind_snapshot = frames_since_rewind_snapshot.saturating_add(1);
                    if frames_since_rewind_snapshot >= 120 {
                        if let Some(ram) = gb.cartridge_ram_snapshot() {
                            rewind_ram_snapshots.push_back(ram);
                            while rewind_ram_snapshots.len() > 30 {
                                let _ = rewind_ram_snapshots.pop_front();
                            }
                        }
                        frames_since_rewind_snapshot = 0;
                    }
                    if autosave_enabled && frames_since_autosave >= 600 {
                        let _ = gb.persist_save();
                        frames_since_autosave = 0;
                    }
                    if let Some(ref ao) = audio_out {
                        ao.enqueue_interleaved(gb.bus.apu.take_pending_samples());
                    }
                }
                let frame = pixels.frame_mut();
                let game_bytes = (GB_W * GB_H * 4) as usize;
                copy_frame(
                    &gb.bus.ppu.framebuffer,
                    &mut frame[..game_bytes],
                    &mut filtered_frame,
                    video_filter,
                );
                if hud_visible {
                    let save_summary = gb
                        .rom_path()
                        .and_then(save_health_for_rom)
                        .map(|h| {
                            let suffix = h
                                .last_modified_unix_secs
                                .map(format_last_played_secs)
                                .unwrap_or_else(|| "never".to_string());
                            if h.has_save {
                                format!("save ok · updated {suffix}")
                            } else if h.has_backup {
                                format!("backup only · updated {suffix}")
                            } else {
                                "no save yet".to_string()
                            }
                        })
                        .unwrap_or_else(|| "save status unknown".to_string());
                    let status_line = format!(
                        "{} · autosave {} · {}",
                        match audio_mode {
                            AudioMode::Balanced => "stereo",
                            AudioMode::LowLatency => "mono",
                        },
                        if autosave_enabled { "on" } else { "off" },
                        save_summary
                    );
                    draw_controls_hud(
                        frame,
                        GB_W as usize,
                        FB_H as usize,
                        GB_H as usize,
                        fast_forward_held,
                        rendered_frames,
                        autosave_enabled,
                        &status_line,
                    );
                } else {
                    clear_hud_strip(frame, GB_W as usize, FB_H as usize, GB_H as usize);
                }
                let _ = pixels.render();
                rendered_frames = rendered_frames.saturating_add(1);
                if rendered_frames.is_multiple_of(300) {
                    log::info!(
                        "runtime: frames={} autosave={} rewind_points={} cheats={}",
                        rendered_frames,
                        autosave_enabled,
                        rewind_ram_snapshots.len(),
                        cheats.len()
                    );
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let _ = gb.persist_save();
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } if size.width > 0 && size.height > 0 => {
                let _ = pixels.resize_surface(size.width, size.height);
            }
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                ..
            } if new_inner_size.width > 0 && new_inner_size.height > 0 => {
                let _ = pixels.resize_surface(new_inner_size.width, new_inner_size.height);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state,
                                virtual_keycode,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                let pressed = state == ElementState::Pressed;
                if let Some(code) = virtual_keycode {
                    if code == controls.fast_forward {
                        fast_forward_held = pressed;
                    } else if code == controls.a {
                        set_button(pressed, &mut gb, joypad::BTN_A);
                    } else if code == controls.b {
                        set_button(pressed, &mut gb, joypad::BTN_B);
                    } else if code == controls.start {
                        set_button(pressed, &mut gb, joypad::BTN_START);
                    } else if code == controls.select {
                        set_button(pressed, &mut gb, joypad::BTN_SELECT);
                    } else if code == controls.up {
                        set_direction(pressed, &mut gb, joypad::DIR_UP);
                    } else if code == controls.down {
                        set_direction(pressed, &mut gb, joypad::DIR_DOWN);
                    } else if code == controls.left {
                        set_direction(pressed, &mut gb, joypad::DIR_LEFT);
                    } else if code == controls.right {
                        set_direction(pressed, &mut gb, joypad::DIR_RIGHT);
                    } else if code == VirtualKeyCode::F5 && pressed {
                        let _ = gb.persist_save();
                        action_toast = Some(("Saved SRAM".to_string(), rendered_frames + 180));
                    } else if code == VirtualKeyCode::F9 && pressed {
                        let _ = gb.reload_save();
                        action_toast = Some(("Loaded SRAM".to_string(), rendered_frames + 180));
                    } else if code == VirtualKeyCode::F7 && pressed {
                        if let Some(snapshot) = rewind_ram_snapshots.pop_back() {
                            gb.load_cartridge_ram_snapshot(&snapshot);
                            let _ = gb.persist_save();
                            action_toast = Some((
                                "Rewind snapshot restored".to_string(),
                                rendered_frames + 180,
                            ));
                        } else {
                            action_toast = Some((
                                "No rewind snapshot available".to_string(),
                                rendered_frames + 180,
                            ));
                        }
                    } else if code == VirtualKeyCode::F6 && pressed {
                        #[cfg(target_os = "macos")]
                        if let Some(dir) = gb.save_dir() {
                            let _ = Command::new("open").arg(dir).spawn();
                        }
                    } else if code == VirtualKeyCode::F2 && pressed {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Game Boy ROM", &["gb", "gbc"])
                            .pick_file()
                        {
                            if let Ok(exe) = std::env::current_exe() {
                                let vf = match video_filter {
                                    VideoFilter::Sharp => "sharp",
                                    VideoFilter::Smooth => "smooth",
                                };
                                let am = match audio_mode {
                                    AudioMode::Balanced => "balanced",
                                    AudioMode::LowLatency => "low-latency",
                                };
                                let save = if autosave_enabled { "1" } else { "0" };
                                let _ = Command::new(exe)
                                    .arg(path)
                                    .arg("--scale")
                                    .arg(scale.to_string())
                                    .env(runtime_env::CONTROLS.0, controls.to_env_string())
                                    .env(runtime_env::CONTROLS.1, controls.to_env_string())
                                    .env(runtime_env::AUTOSAVE.0, save)
                                    .env(runtime_env::AUTOSAVE.1, save)
                                    .env(runtime_env::VIDEO_FILTER.0, vf)
                                    .env(runtime_env::VIDEO_FILTER.1, vf)
                                    .env(runtime_env::AUDIO_MODE.0, am)
                                    .env(runtime_env::AUDIO_MODE.1, am)
                                    .spawn();
                                let _ = gb.persist_save();
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    } else if code == VirtualKeyCode::F12 && pressed {
                        let _ = save_screenshot_ppm(
                            &gb.bus.ppu.framebuffer,
                            GB_W as usize,
                            GB_H as usize,
                            screenshot_output_path(),
                        );
                        action_toast =
                            Some(("Screenshot saved".to_string(), rendered_frames + 180));
                    } else if code == VirtualKeyCode::Tab && pressed {
                        hud_visible = !hud_visible;
                        action_toast = Some((
                            if hud_visible {
                                "HUD shown".to_string()
                            } else {
                                "HUD hidden".to_string()
                            },
                            rendered_frames + 120,
                        ));
                    } else if code == VirtualKeyCode::Escape && pressed {
                        let _ = gb.persist_save();
                        spawn_launcher_menu_process();
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
            Event::MainEventsCleared => {
                if let Some((msg, until)) = &action_toast {
                    if rendered_frames < *until {
                        window.set_title(&format!("{BASE_TITLE}  |  {}", msg));
                    } else {
                        action_toast = None;
                        window.set_title(BASE_TITLE);
                    }
                } else if rendered_frames < toast_until_frame {
                    window.set_title(&format!("{BASE_TITLE}  |  {}", launch_toast));
                } else {
                    window.set_title(BASE_TITLE);
                }
                window.request_redraw();
            }
            _ => {}
        }
    });

    #[allow(unreachable_code)]
    Ok(())
}

fn set_button(pressed: bool, gb: &mut GameBoy, btn: u8) {
    if pressed {
        let if_ = &mut gb.bus.interrupts.if_;
        gb.bus.joypad.set_button_down(btn, if_);
    } else {
        gb.bus.joypad.set_button_up(btn);
    }
}

fn set_direction(pressed: bool, gb: &mut GameBoy, dir: u8) {
    if pressed {
        let if_ = &mut gb.bus.interrupts.if_;
        gb.bus.joypad.set_direction_down(dir, if_);
    } else {
        gb.bus.joypad.set_direction_up(dir);
    }
}

fn format_last_played_secs(last_played: u64) -> String {
    let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) else {
        return "unknown".to_string();
    };
    let now = now.as_secs();
    if now <= last_played + 60 {
        return "just now".to_string();
    }
    let delta = now.saturating_sub(last_played);
    if delta < 3600 {
        return format!("{}m ago", delta / 60);
    }
    if delta < 86_400 {
        return format!("{}h ago", delta / 3600);
    }
    format!("{}d ago", delta / 86_400)
}
