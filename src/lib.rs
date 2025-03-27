//! A simple logger library. This library provides a simple log writer and simple log level control.
//! It can be used to write logs in a program. The logs can be written to a dictionary.
//! The log level can be set to different levels (Error, Warn, Debug, Info and Trace).

mod logger;
mod msg;
mod setting;
mod time;

pub use logger::*;
pub use setting::Setting;

#[cfg(feature = "async")]
pub use async_log::*;
#[cfg(not(feature = "async"))]
pub use log::*;

use lazy_static::lazy_static;
#[cfg(not(feature = "async"))]
use std::sync::Mutex;
#[cfg(feature = "async")]
use tokio;
#[cfg(feature = "async")]
use tokio::sync::Mutex;

lazy_static! {
    /// The static logger.
    /// If async feature is enabled, the mutex used is [``tokio::sync::mutex``], otherwise it is [`std::sync::Mutex`].
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

/// A macro that returns the name of the function it is called in.
#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.rsplit("::")
            .find(|&part| part != "f" && part != "{{closure}}")
            .expect("Short function name")
    }};
}

/// A macro that returns the current position in the code.
#[macro_export]
macro_rules! position {
    () => {{
        let function = $crate::func!();
        let file = file!();
        let line = line!();
        format!("{}@{}:{}", function, file, line)
    }};
}

#[cfg(feature = "async")]
mod async_log {
    use super::*;

    /// Define a public asynchronous function named `init` that takes a `Setting` parameter.
    pub async fn init(setting: Setting) {
        // Acquire a mutable lock on the `LOGGER` (which is presumably a globally accessible logging utility).
        // The `await` keyword is used here to asynchronously wait for the lock to be acquired.
        let mut logger = LOGGER.lock().await;
        // Call the `init` method on the locked logger, passing in the `setting` parameter.
        // This initializes the logger with the provided settings.
        logger.init(setting).await;
    }

    /// Define a public asynchronous function named `clean_log`
    pub async fn clean_log() {
        // Acquire a mutable lock on the LOGGER, which is presumably a globally accessible logging utility
        // The `await` keyword is used here to asynchronously wait for the lock to be acquired
        let mut writer = LOGGER.lock().await;
        // Call the `clear_dir` method on the locked writer to clear the directory where logs are stored
        // This method is also awaited asynchronously, indicating it performs an I/O operation or similar
        writer.clear_dir().await;
    }

    /// Define a public asynchronous function named `set` that takes a `Setting` as a parameter.
    pub async fn set(setting: Setting) {
        // Acquire a mutable lock on the `LOGGER` using the `await` keyword to handle the asynchronous lock acquisition.
        // This ensures that only one writer can access the LOGGER at a time.
        let mut writer = LOGGER.lock().await;
        // Call the `set` method on the locked writer, passing the `setting` parameter to it.
        // This method call is also asynchronous and will await its completion.
        writer.set(setting).await;
    }

    /// Macro to log error message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().await.error(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Macro to log warning message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().await.warn(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Macro to log info message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().await.info(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Macro to log debug message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().await.debug(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Macro to log trace message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().await.trace(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Define a macro named `log` with two parameters: `$level` and `$($arg:tt)*`
    #[macro_export]
    macro_rules! log {
        // Match the macro invocation with a level expression and a variable number of arguments
        ($level:expr, $($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().await.record($level, &format!($($arg)*), position).await;
        }
    }

    /// Define a public asynchronous function named `enable_log`
    pub async fn enable_log() {
        // Acquire a mutable lock on the LOGGER, which is presumably a globally accessible logging utility
        // The `await` keyword is used here to asynchronously wait for the lock to be acquired
        let mut writer = LOGGER.lock().await;
        // Call the `enable` method on the locked writer to enable logging
        writer.enable();
    }

    /// Define a public asynchronous function named `disable_log`
    pub async fn disable_log() {
        // Acquire a mutable lock on the LOGGER, which is presumably a globally accessible logging mechanism
        // The `await` keyword is used here to asynchronously wait for the lock to be acquired
        let mut writer = LOGGER.lock().await;
        // Call the `disable` method on the locked writer to disable logging
        writer.disable();
    }
}

#[cfg(not(feature = "async"))]
mod log {
    use super::*;

    /// Macro to log error message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").error(&format!($($arg)*), position);
        };
    }

    /// Macro to log warning message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").warn(&format!($($arg)*), position);
        };
    }

    /// Macro to log info message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").info(&format!($($arg)*), position);
        };
    }

    /// Macro to log debug message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").debug(&format!($($arg)*), position);
        };
    }

    /// Macro to log trace message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").trace(&format!($($arg)*), position);
        };
    }

    #[macro_export]
    macro_rules! log {
        ($level:expr, $($arg:tt)*) => {
            let position = $crate::position!().to_string();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").record($level, &format!($($arg)*));
        }
    }

    /// Initialize the static logger with customized setting.
    pub fn init(setting: Setting) {
        let mut logger = LOGGER.lock().unwrap();
        logger.init(setting);
    }

    /// Provide a easier way to clean all the existed logs.
    pub fn clean_log() {
        let mut writer = LOGGER.lock().expect("Cannot lock the logger.");
        writer.clear_dir();
    }

    /// Public function to set a specific logging setting
    pub fn set(setting: Setting) {
        // Lock the LOGGER to ensure thread-safe access
        let mut writer = LOGGER.lock().expect("Cannot lock the logger.");
        // Set the provided logging setting using the writer
        writer.set(setting);
    }

    /// Public function to enable logging
    pub fn enable_log() {
        // Lock the LOGGER to ensure thread-safe access
        let mut writer = LOGGER.lock().expect("Cannot lock the logger.");
        // Enable logging using the writer
        writer.enable();
    }

    /// Public function to disable logging
    pub fn disable_log() {
        // Lock the LOGGER to ensure thread-safe access
        let mut writer = LOGGER.lock().expect("Cannot lock the logger.");
        // Disable logging using the writer
        writer.disable();
    }
}

/// Enumeration of log levels.
/// This defines the emergency of the log.
/// (the corresponding number is used to compare the log level to decide write to the log file or not.)
#[derive(Copy, Clone, Debug)]
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

unsafe impl Send for LogLevel {}
