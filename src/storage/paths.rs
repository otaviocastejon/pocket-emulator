use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use directories::ProjectDirs;
use sha1::{Digest, Sha1};

fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "pocketemulator", "pocketemulator")
}

pub fn data_dir() -> Option<PathBuf> {
    project_dirs().map(|d| d.data_local_dir().to_path_buf())
}

pub fn ensure_data_dirs() -> io::Result<()> {
    let Some(base) = data_dir() else {
        return Ok(());
    };
    fs::create_dir_all(base.join("saves"))?;
    fs::create_dir_all(base.join("logs"))?;
    fs::create_dir_all(base.join("screenshots"))?;
    fs::create_dir_all(base.join("library").join("roms"))?;
    fs::create_dir_all(base.join("library").join("covers"))?;
    Ok(())
}

pub fn logs_dir() -> Option<PathBuf> {
    data_dir().map(|d| d.join("logs"))
}

pub fn screenshots_dir() -> Option<PathBuf> {
    data_dir().map(|d| d.join("screenshots"))
}

pub fn library_roms_dir() -> Option<PathBuf> {
    data_dir().map(|d| d.join("library").join("roms"))
}

pub fn library_covers_dir() -> Option<PathBuf> {
    data_dir().map(|d| d.join("library").join("covers"))
}

pub fn save_path_for_rom(rom_path: &Path) -> Option<PathBuf> {
    let base = data_dir()?;
    let saves_dir = base.join("saves");
    let display = rom_path.to_string_lossy();
    let mut hasher = Sha1::new();
    hasher.update(display.as_bytes());
    let digest = hasher.finalize();
    let id = format!("{:x}", digest);
    let stem = rom_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("game")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();
    Some(saves_dir.join(format!("{stem}-{}.sav", &id[..12])))
}

pub fn import_rom_into_library(rom_path: &Path) -> io::Result<PathBuf> {
    ensure_data_dirs()?;
    let ext = rom_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext != "gb" && ext != "gbc" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsupported rom extension: .{ext}"),
        ));
    }

    let src = rom_path
        .canonicalize()
        .unwrap_or_else(|_| rom_path.to_path_buf());
    let Some(roms_dir) = library_roms_dir() else {
        return Ok(src);
    };
    let roms_root = roms_dir.canonicalize().unwrap_or_else(|_| roms_dir.clone());
    if src.starts_with(&roms_root) {
        return Ok(src);
    }

    let bytes = fs::read(&src)?;
    let mut hasher = Sha1::new();
    hasher.update(&bytes);
    let digest = hasher.finalize();
    let hash = format!("{:x}", digest);
    let stem = src
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("game")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();
    let dst = roms_dir.join(format!("{stem}-{}.{}", &hash[..12], ext));
    if !dst.exists() {
        fs::write(&dst, bytes)?;
    }

    if let Some(covers_dir) = library_covers_dir() {
        let src_stem = src.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        let dst_stem = dst.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        for cover_ext in ["png", "jpg", "jpeg", "webp"] {
            let candidate = src.with_file_name(format!("{src_stem}.{cover_ext}"));
            if candidate.exists() {
                let _ = fs::copy(
                    &candidate,
                    covers_dir.join(format!("{dst_stem}.{cover_ext}")),
                );
            }
        }
    }

    Ok(dst)
}

#[derive(Debug, Clone, Copy)]
pub struct SaveHealth {
    pub has_save: bool,
    pub has_backup: bool,
    pub last_modified_unix_secs: Option<u64>,
}

pub fn save_health_for_rom(rom_path: &Path) -> Option<SaveHealth> {
    let save_path = save_path_for_rom(rom_path)?;
    let has_save = save_path.exists();
    let backup_path = save_path.with_extension("sav.bak");
    let has_backup = backup_path.exists();
    let last_modified_unix_secs = fs::metadata(&save_path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .or_else(|| {
            fs::metadata(&backup_path)
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
        });
    Some(SaveHealth {
        has_save,
        has_backup,
        last_modified_unix_secs,
    })
}
