use crate::cli::LogLevel;

/// Initialize application logging with the specified log level
pub fn init_app_log(level: LogLevel) -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(level.into())
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))
}
