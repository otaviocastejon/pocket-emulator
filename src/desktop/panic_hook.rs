use std::fs;
use std::io::Write;

use crate::storage;

pub fn install_panic_hook() {
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        default(panic_info);
        let Some(logs_dir) = storage::logs_dir() else {
            return;
        };
        let _ = fs::create_dir_all(&logs_dir);
        let crash_path = logs_dir.join("crash.log");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let msg = format!("[{now}] panic: {panic_info}\n");
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(crash_path)
            .and_then(|mut f| f.write_all(msg.as_bytes()));
    }));
}
