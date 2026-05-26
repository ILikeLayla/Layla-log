use super::LogLevel;

/// the configuration of the logger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogSetting<'a: 'static> {
    /// where stores the log files.
    pub dir_path: &'a str,
    /// the maximum number of logs in a single file.
    pub single_length: usize,
    /// define the minimum [`LogLevel`] of the log that should be written. (inclusive)
    pub file_record_level: LogLevel,
    /// define the minimum [`LogLevel`] of the log that should be printed. (inclusive)
    pub terminal_print_level: LogLevel,
    /// define to show the detailed time or not.
    pub time_detailed_display: bool,
    /// the prefix of the time.
    pub file_time_format: &'a str,
    /// the time zone of the log.
    pub time_zone: i32,
    /// setting whether to print the log to the terminal.
    pub print_out: bool,
    /// set whether to print the scope of the log.
    pub display_scope: bool,
    /// set whether to print the path of the log.
    pub display_path: bool,
}

impl<'a> std::default::Default for LogSetting<'a> {
    /// Provide default settings, and the logger can use the default setting to initialize itself.
    fn default() -> Self {
        let terminal_print_level = if cfg!(debug_assertions) {
            LogLevel::Debug
        } else {
            LogLevel::Info
        };

        LogSetting {
            dir_path: "./logs",
            single_length: 0,
            file_record_level: LogLevel::Trace,
            terminal_print_level,
            time_detailed_display: false,
            file_time_format: "%Y-%m-%d",
            time_zone: 0,
            print_out: true,
            display_path: true,
            display_scope: true,
        }
    }
}

unsafe impl<'a> Send for LogSetting<'a> {}
