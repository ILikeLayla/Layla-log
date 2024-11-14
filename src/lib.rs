mod msg;
mod time;
mod logger;

pub use logger::*;

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    /// The static logger.
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

/// Initialize the static logger with default setting.
pub fn default_init(dir_path: &str) {
    // _init_block(Writer::default(dir_path))
    let mut logger = LOGGER.lock().unwrap();
    logger.default(dir_path);
}

/// Initialize the static logger with customized setting.
pub fn init(dir_path: &str, single_length: usize, file_record: LogLevel, terminal_print: LogLevel, time_zone: i32, time_details: bool, print_out: bool) {
    let mut logger = LOGGER.lock().unwrap();
    logger.init(dir_path, single_length, file_record, terminal_print, time_zone, time_details, print_out);
}

/// Provide a easier way to clean all the existed logs.
pub fn clean_log() {
    let mut writer = LOGGER.lock().expect("Cannot lock the logger.");
    writer.clear_dir();
}


/// Macro to log error message.
/// First lock the logger in static, then log the message.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::LOGGER.lock().expect("Cannot lock the logger.").error(&format!($($arg)*));
    };
}

/// Macro to log warning message.
/// First lock the logger in static, then log the message.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::LOGGER.lock().expect("Cannot lock the logger.").warn(&format!($($arg)*));
    };
}

/// Macro to log info message.
/// First lock the logger in static, then log the message.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::LOGGER.lock().expect("Cannot lock the logger.").info(&format!($($arg)*));
    };
}

/// Macro to log debug message.
/// First lock the logger in static, then log the message.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::LOGGER.lock().expect("Cannot lock the logger.").debug(&format!($($arg)*));
    };
}

/// Macro to log trace message.
/// First lock the logger in static, then log the message.
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::LOGGER.lock().expect("Cannot lock the logger.").trace(&format!($($arg)*));
    };
}

/// Enumeration of log levels.
/// This defines the emergency of the log.
/// (the corresponding number is used to compare the log level to decide write to the log file or not.)
#[derive(Copy, Clone)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

impl LogLevel {
    pub fn get_level(&self) -> usize {
        *self as usize
    }
}
