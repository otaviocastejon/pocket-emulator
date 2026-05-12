//! Rolling log under app data; filter via `RUST_LOG` (default `info`).

use flexi_logger::{Duplicate, FileSpec, Logger};

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
