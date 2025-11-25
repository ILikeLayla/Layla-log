//! A simple logger library. This library provides a simple log writer and simple log level control.
//! It can be used to write logs in a program. The logs can be written to a dictionary.
//! The log level can be set to different levels (Error, Warn, Debug, Info and Trace).

// TODO: documentation

mod logger;
mod msg;
mod setting;
mod time;

pub use logger::*;
pub use setting::LogSetting;

#[cfg(feature = "async")]
use futures;
use lazy_static::lazy_static;
#[cfg(feature = "async")]
use std::future::Future;
use std::sync::Arc;
#[cfg(not(feature = "async"))]
use std::sync::Mutex;
#[cfg(feature = "async")]
use tokio;
#[cfg(feature = "async")]
use tokio::sync::Mutex;

lazy_static! {
    /// The static logger.
    /// If async feature is enabled, the mutex used is [``tokio::sync::mutex``], otherwise it is [`std::sync::Mutex`].
    pub static ref LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger::new()));
    pub static ref LOGSETTING: Arc<Mutex<LogSetting>> = Arc::new(Mutex::new(LogSetting::default()));
}

#[cfg(feature = "async")]
pub fn block_on<F, T>(future: F) -> F::Output
where
    F: Future<Output = T>,
{
    // let rt = tokio::runtime::Builder::new_current_thread().worker_threads(1).enable_all().build().unwrap();
    // rt.block_on(future)
    futures::executor::block_on(future)
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
        name.strip_suffix("::f")
            .unwrap()
            .rsplit("::")
            .find(|&part| part != "{{closure}}")
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
        let column = column!();
        // format!("{} @ {}:{}:{}", function, file, line, column)
        $crate::PositionTag {
            scoop: function.to_string(),
            path: format!("{}:{}:{}", file, line, column),
        }
    }};
}

#[derive(Clone, Debug)]
pub struct PositionTag {
    pub scoop: String,
    pub path: String,
}

#[cfg(feature = "async")]
mod async_log {
    #[macro_export]
    macro_rules! log_set {
        ($($key:ident : $value:expr),*) => {
            {
                let previous_setting = $crate::LOGSETTING.lock().await.clone();
                *($crate::LOGSETTING.lock().await) = $crate::LogSetting {
                    $($key: $value,)*
                    ..previous_setting
                };
            };
        };
    }

    #[macro_export]
    macro_rules! clean_log {
        () => {{
            let mut writer = $crate::LOGGER.lock().await;
            writer.clear_dir().await;
        };};
    }

    /// Macro to log error message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().await.error(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Macro to log warning message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().await.warn(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Macro to log info message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().await.info(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Macro to log debug message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().await.debug(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Macro to log trace message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().await.trace(format!($($arg)*).as_str(), position).await;
        };
    }

    /// Define a macro named `log` with two parameters: `$level` and `$($arg:tt)*`
    #[macro_export]
    macro_rules! log {
        // Match the macro invocation with a level expression and a variable number of arguments
        ($level:expr, $($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().await.record($level, &format!($($arg)*), position).await;
        }
    }

    #[macro_export]
    macro_rules! enable_log {
        () => {{
            let mut writer = $crate::LOGGER.lock().await;
            writer.enable();
        }};
    }

    #[macro_export]
    macro_rules! disable_log {
        () => {{
            let mut writer = $crate::LOGGER.lock().await;
            writer.disable();
        }};
    }
}

#[cfg(not(feature = "async"))]
mod sync_log {
    /// Macro to log error message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").error(&format!($($arg)*), position);
        };
    }

    /// Macro to log warning message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").warn(&format!($($arg)*), position);
        };
    }

    /// Macro to log info message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").info(&format!($($arg)*), position);
        };
    }

    /// Macro to log debug message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").debug(&format!($($arg)*), position);
        };
    }

    /// Macro to log trace message.
    /// First lock the logger in static, then log the message.
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").trace(&format!($($arg)*), position);
        };
    }

    #[macro_export]
    macro_rules! log {
        ($level:expr, $($arg:tt)*) => {
            let position = $crate::position!();
            $crate::LOGGER.lock().expect("Cannot lock the logger.").record($level, &format!($($arg)*), position);
        }
    }

    #[macro_export]
    macro_rules! log_set {
        ($($key:ident : $value:expr),*) => {
            {
                let previous_setting = $crate::LOGSETTING.lock().unwrap().clone();
                *($crate::LOGSETTING.lock().unwrap()) = $crate::LogSetting {
                    $($key: $value,)*
                    ..previous_setting
                };
            };
        };
    }

    #[macro_export]
    macro_rules! clean_log {
        () => {{
            let mut writer = $crate::LOGGER.lock().unwrap();
            writer.clear_dir();
        };};
    }

    #[macro_export]
    macro_rules! enable_log {
        () => {{
            let mut writer = $crate::LOGGER.lock().unwrap();
            writer.enable();
        }};
    }

    #[macro_export]
    macro_rules! disable_log {
        () => {{
            let mut writer = $crate::LOGGER.lock().unwrap();
            writer.disable();
        }};
    }
}

/// Enumeration of log levels.
/// This defines the emergency of the log.
/// (the corresponding number is used to compare the log level to decide write to the log file or not.)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
            LogLevel::Info => write!(f, "INFO "),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Warn => write!(f, "WARN "),
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
