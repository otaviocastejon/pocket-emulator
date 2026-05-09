//! stderr + rolling log file under the app data directory (`logs/pocketemulator.log`).
//!
//! Level/filter: `RUST_LOG` (same as former `env_logger`), default **`info`**.

use flexi_logger::{Duplicate, FileSpec, Logger};

/// Initialize global logging: mirror to stderr and append to `logs_dir()/pocketemulator.log`.
///
/// If no project data directory exists, logs only go to stderr (same as before).
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let _ = pocketemulator::storage::ensure_data_dirs();

    if let Some(dir) = pocketemulator::storage::logs_dir() {
        Logger::try_with_env_or_str("info")?
            .log_to_file(
                FileSpec::default()
                    .directory(&dir)
                    .basename("pocketemulator")
                    .suffix("log"),
            )
            .duplicate_to_stderr(Duplicate::All)
            .start()?;

        log::info!("logging enabled — log directory: {}", dir.display());
    } else {
        Logger::try_with_env_or_str("info")?.start()?;
        log::info!("logging enabled — stderr only (no app data directory for log file)");
    }

    Ok(())
}
