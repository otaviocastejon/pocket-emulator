use std::fs;
use std::io;
use std::path::PathBuf;

use pocketemulator::storage;

pub fn discover_roms(rom_dir: Option<&PathBuf>) -> io::Result<Vec<PathBuf>> {
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Some(dir) = rom_dir {
        roots.push(dir.clone());
    } else {
        roots.push(PathBuf::from("ROMS"));
        roots.push(PathBuf::from("roms"));
        if let Some(library) = storage::library_roms_dir() {
            roots.push(library);
        }
    }

    let mut out = Vec::new();
    for root in roots {
        if !root.exists() {
            continue;
        }
        collect_roms_recursive(&root, &mut out)?;
    }
    Ok(out)
}

fn collect_roms_recursive(dir: &PathBuf, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_roms_recursive(&path, out)?;
            continue;
        }
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_ascii_lowercase();
            if ext == "gb" || ext == "gbc" {
                out.push(path);
            }
        }
    }
    Ok(())
}
