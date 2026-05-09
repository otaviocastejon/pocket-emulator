//! Third-party ROM listing in the system browser + auto-import from the Downloads folder.
//!
//! We cannot embed a web view in egui without heavy dependencies; opening the default browser is
//! the portable approach. New `.gb` / `.gbc` files are detected once they stop changing size/mtime
//! between polls (avoids importing incomplete downloads).

use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use directories::UserDirs;
use eframe::egui;

use super::super::app::LauncherApp;
use crate::desktop::launcher::components::buttons::{action_button, ButtonSize, ButtonVariant};
use crate::desktop::launcher::components::cards::section_card;
use crate::desktop::launcher::components::toasts::ToastTone;
use crate::desktop::launcher::components::tokens::{
    destructive_text, info_text, muted_text, primary_color, text_caption, text_h3,
};
use crate::desktop::launcher::theme::{space_2, space_3, space_4};

/// Catalog page the user asked to surface (opens in the system browser).
pub const GBC_ROMS_LIST_URL: &str = "https://www.romsgames.net/roms/gameboy-color/";

pub(crate) fn render_get_roms_tab(app: &mut LauncherApp, ui: &mut egui::Ui) {
    section_card(ui, |ui| {
        ui.label(text_h3("Game Boy Color downloads").color(primary_color()));
        ui.add_space(space_2());
        ui.label(text_caption(
            "ROM copyright belongs to publishers. Only download games you already own or that are \
             legally distributed. PocketEmulator does not host ROM files.",
        ));
        ui.add_space(space_3());
        if action_button(
            ui,
            "Open ROM catalog in browser",
            ButtonVariant::Primary,
            ButtonSize::Lg,
        )
        .clicked()
        {
            if let Err(e) = webbrowser::open(GBC_ROMS_LIST_URL) {
                app.error = Some(format!("Could not open browser: {e}"));
            } else {
                app.error = None;
                app.push_toast(
                    "Browser opened".to_string(),
                    "Download a .gb or .gbc file; it will be added to your library when the file finishes."
                        .to_string(),
                    ToastTone::Info,
                    None,
                    5.0,
                );
            }
        }
        ui.add_space(space_2());
        ui.small(
            egui::RichText::new(format!("URL · {GBC_ROMS_LIST_URL}"))
                .color(muted_text())
                .monospace(),
        );
    });

    ui.add_space(space_4());

    section_card(ui, |ui| {
        ui.label(text_h3("Automatic import").color(primary_color()));
        ui.add_space(space_2());
        ui.label(text_caption(
            "While this launcher stays open, we watch your Downloads folder. When a new Game Boy \
             ROM stops changing (download finished), it is copied into your PocketEmulator library.",
        ));
        ui.add_space(space_2());
        if let Some(dir) = downloads_dir() {
            ui.small(
                egui::RichText::new(format!("Watching · {}", dir.display())).color(info_text()),
            );
        } else {
            ui.colored_label(
                destructive_text(),
                "Could not locate your Downloads folder.",
            );
        }
        ui.add_space(space_2());
        ui.small(text_caption(
            "Tip · If nothing appears, confirm Safari/Chrome saves to the default Downloads path, \
             or use My Games → Import ROM.",
        ));
    });

    ui.add_space(space_4());

    section_card(ui, |ui| {
        ui.horizontal(|ui| {
            if action_button(
                ui,
                "Go to My Games",
                ButtonVariant::Secondary,
                ButtonSize::Md,
            )
            .clicked()
            {
                app.active_tab = super::LauncherTab::MyGames;
            }
        });
    });
}

fn downloads_dir() -> Option<PathBuf> {
    UserDirs::new().and_then(|d| d.download_dir().map(|p| p.to_path_buf()))
}

fn rom_extensions_ok(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            let e = e.to_ascii_lowercase();
            e == "gb" || e == "gbc"
        })
        .unwrap_or(false)
}

fn file_fingerprint(path: &Path) -> Option<(u64, u128)> {
    let meta = fs::metadata(path).ok()?;
    let len = meta.len();
    let mt = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_nanos() as u128)
        .unwrap_or(0);
    Some((len, mt))
}

/// Called from [`LauncherApp::update`] on a timer while the launcher is open.
pub(crate) fn poll_download_folder_imports(app: &mut LauncherApp, ctx: &egui::Context) {
    let now = ctx.input(|i| i.time);
    if now - app.last_download_poll_time < 0.85 {
        return;
    }
    app.last_download_poll_time = now;

    let Some(downloads) = downloads_dir() else {
        return;
    };

    let Ok(entries) = fs::read_dir(&downloads) else {
        return;
    };

    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && rom_extensions_ok(p))
        .collect();

    paths.sort();

    // Drop snapshots for files that disappeared (user moved/deleted).
    app.download_prev_snapshot
        .retain(|p, _| paths.iter().any(|x| x == p));

    for path in paths {
        let canon = path.canonicalize().unwrap_or_else(|_| path.clone());

        if app.download_imported_sources.contains(&canon) {
            continue;
        }

        let Some(fp) = file_fingerprint(&canon) else {
            continue;
        };
        // Ignore obviously incomplete tiny writes (adjust if you support mini ROM tests).
        if fp.0 < 256 {
            continue;
        }

        let stable = app
            .download_prev_snapshot
            .get(&canon)
            .is_some_and(|prev| prev == &fp);

        if stable {
            match app.import_rom_into_library(&canon) {
                Ok(imported) => {
                    app.download_imported_sources.insert(canon.clone());
                    app.download_prev_snapshot.remove(&canon);
                    let name = imported
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Game")
                        .to_string();
                    app.push_toast(
                        "Download imported".to_string(),
                        format!("{name} is in your library"),
                        ToastTone::Success,
                        None,
                        4.5,
                    );
                    app.error = None;
                    continue;
                }
                Err(e) => {
                    log::warn!(
                        "auto-import from Downloads failed for {}: {e}",
                        canon.display()
                    );
                }
            }
        }

        app.download_prev_snapshot.insert(canon, fp);
    }

    ctx.request_repaint_after(std::time::Duration::from_millis(400));
}
