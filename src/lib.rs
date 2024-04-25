// Import the files.
mod msg;
mod time;
mod writer;

// Re-export.
pub use writer::*;

use std::sync::{Arc, Mutex};
pub static mut LOGGER: Option<Arc<Mutex<Writer>>> = None;

/// Initialize the static logger with default setting.
pub fn default_init(dir_path: &str) {
    _init_block(Writer::default(dir_path))
}


/// Initialize the static logger with customized setting.
pub fn init(dir_path: &str, single_length: Option<usize>, log_level: Option<LogLevel>, time_zone: i32, time_details: bool, print_out: bool) {
    _init_block(Writer::new(dir_path, single_length, log_level, time_zone, time_details, print_out))
}

// A function provide a easier way to initialize the static logger by checking and replacing the writer in the static cell.
fn _init_block(aim_writer: Writer) {
    unsafe {
        let writer = std::ptr::addr_of!(LOGGER);
        match writer.as_ref() {
            Some(Some(writer)) => {
                writer.lock().unwrap().warn("The logger had already been initialized.")
            },
            Some(None) => {
                LOGGER = Some(Arc::new(Mutex::new(aim_writer)));
            },
            None => {
                panic!("Shouldn't be here.")
            }
        }
    }
}

/// Provide a easier way to clean all the existed logs.
pub fn clean_log() {
    unsafe {
        let mut writer = LOGGER.as_mut().expect("The logger haven't been initialized.").lock().unwrap();
        writer.clear_dir();
    }
}


// $crate::LOGGER => Option<Arc<Mutex<Writer>>>
//      .as_mut() => Option<&mut Arc<Mutex<Writer>, global>>
//      .expect() => &mut Arc<Mutex<Writer>>
//      .lock()   => Result<MutexGuard<'{error}, Writer>, PoisonError<MutexGuard<'{error}, Writer>>>
//      .unwrap() => MutexGuard<'{error}, Writer>

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        unsafe { $crate::LOGGER.as_mut().expect("The logger haven't been initialized.").lock().unwrap().error(&format!($($arg)*)) };
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        unsafe { $crate::LOGGER.as_mut().expect("The logger haven't been initialized.").lock().unwrap().warn(&format!($($arg)*)) };
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        unsafe { $crate::LOGGER.as_mut().expect("The logger haven't been initialized.").lock().unwrap().info(&format!($($arg)*)) };
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        unsafe { $crate::LOGGER.as_mut().expect("The logger haven't been initialized.").lock().unwrap().debug(&format!($($arg)*)) };
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        unsafe { $crate::LOGGER.as_mut().expect("The logger haven't been initialized.").lock().unwrap().trace(&format!($($arg)*)) };
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
