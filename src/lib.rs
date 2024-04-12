// Import the files.
mod msg;
mod time;
mod writer;

// Re-export.
pub use writer::*;

/// Enumeration of log levels.
/// This defines the emergency of the log.
/// (the corresponding number is used to compare the log level to decide write to the log file or not.)
#[derive(Copy, Clone)]
pub enum LogLevel {
    Info = 0,
    Debug = 1,
    Warn = 2,
    Error = 3,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}
