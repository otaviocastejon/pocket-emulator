use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::storage;

pub fn screenshot_output_path() -> PathBuf {
    let base = storage::screenshots_dir().unwrap_or_else(|| PathBuf::from("screenshots"));
    let _ = fs::create_dir_all(&base);
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    base.join(format!("shot-{ts}.ppm"))
}

pub fn save_screenshot_ppm(
    rgba: &[u8],
    width: usize,
    height: usize,
    path: PathBuf,
) -> std::io::Result<()> {
    let mut out = Vec::with_capacity(width * height * 3 + 64);
    out.extend_from_slice(format!("P6\n{width} {height}\n255\n").as_bytes());
    for px in rgba.chunks_exact(4) {
        out.push(px[0]);
        out.push(px[1]);
        out.push(px[2]);
    }
    fs::write(path, out)
}
