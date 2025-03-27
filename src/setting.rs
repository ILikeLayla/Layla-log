use super::LogLevel;

/// the configuration of the logger.
#[derive(Debug)]
pub struct Setting {
    /// where stores the log files.
    pub dir_path: String,
    /// the maximum number of logs in a single file.
    pub single_length: usize,
    /// define the minimum [`LogLevel`] of the log that should be written. (inclusive)
    pub file_record_level: LogLevel,
    /// define the minimum [`LogLevel`] of the log that should be printed. (inclusive)
    pub terminal_print_level: LogLevel,
    /// define to show the detailed time or not.
    pub time_detailed_display: bool,
    /// the prefix of the time.
    pub file_time_format: String,
    /// the time zone of the log.
    pub time_zone: i32,
    /// setting whether to print the log to the terminal.
    pub print_out: bool,
    /// setting whether to log or not
    pub disabled: bool,
}

impl std::default::Default for Setting {
    /// Provide default settings, and the logger can use the default setting to initialize itself.
    fn default() -> Self {
        let terminal_print_level = if cfg!(debug_assertions) {
            LogLevel::Debug
        } else {
            LogLevel::Info
        };

        Setting {
            dir_path: "./logs".to_string(),
            single_length: 0,
            file_record_level: LogLevel::Trace,
            terminal_print_level,
            time_detailed_display: false,
            file_time_format: "%Y-%m-%d".to_string(),
            time_zone: 0,
            print_out: false,
            disabled: false,
        }
    }
}

unsafe impl Send for Setting {}
