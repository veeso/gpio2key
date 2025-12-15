mod log_level;

use std::path::PathBuf;

pub use self::log_level::LogLevel;

/// gpio2key command line arguments
#[derive(Debug, argh::FromArgs)]
pub struct Args {
    /// path to configuration file
    #[argh(option, short = 'c', default = "PathBuf::from(\"config.toml\")")]
    pub config: PathBuf,
    /// chip device (e.g., /dev/gpiochip0)
    #[argh(option, short = 'd', default = "PathBuf::from(\"/dev/gpiochip0\")")]
    pub device: PathBuf,
    /// log level (error, warn, info, debug, trace)
    #[argh(option, short = 'l', default = "LogLevel::Info")]
    pub log_level: LogLevel,
}
