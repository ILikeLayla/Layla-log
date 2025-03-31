use super::{msg::LogMessage, position, LogLevel, Setting};
use chrono::FixedOffset;
#[cfg(not(feature = "async"))]
use std::fs::{self, File};
#[cfg(not(feature = "async"))]
use std::io::Write;
#[cfg(feature = "async")]
use tokio::fs::{self, File};
#[cfg(feature = "async")]
use tokio::io::AsyncWriteExt;

/// A writer for buffering the log and writing them into the suitable files.
#[derive(Debug)]
pub struct Logger {
    /// the file that is currently being written.
    file: Option<File>,
    /// the current index of the file.
    current_index: usize,
    /// the length of the log that have been written.
    used_length: usize,
    /// a buffer to store the prefex of log files' name.
    current_file_prefix: String,
    /// check if the writer is initialized.
    init: bool,
    /// setting of the logger.
    setting: Setting,
}

impl Logger {
    /// Initialize the logger with all default setting.
    pub(crate) fn new() -> Self {
        let setting = Setting {
            ..Default::default()
        };

        let mut buffer = Self {
            file: None,
            current_index: 0,
            used_length: 0,
            init: false,
            current_file_prefix: format!(
                "{}",
                chrono::Utc::now()
                    .with_timezone(&FixedOffset::east_opt(setting.time_zone * 3600).unwrap())
                    .format(&setting.file_time_format)
            ),
            setting,
        };
        buffer.check_dir();
        buffer.current_index = buffer.get_index_not_async(&buffer.current_file_prefix);
        buffer
    }

    /// Get the path of the log file.
    fn get_path(&self, time_prefix: &str, index: usize) -> String {
        format!("{}/{}_{}.log", self.setting.dir_path, time_prefix, index)
    }

    /// Disable the logger.
    pub(crate) fn disable(&mut self) {
        self.setting.disabled = true;
    }

    /// Enable the logger.
    pub(crate) fn enable(&mut self) {
        self.setting.disabled = false;
    }

    /// Get the index of the current log file.
    /// This is used when resume the logging, since have to keep a continuos order of the log files.
    fn get_index_not_async(&self, time_prefix: &str) -> usize {
        let mut count = 0;
        loop {
            let path = self.get_path(time_prefix, count);
            // if the file exists, then the index is the next one
            if let Ok(_) = std::fs::File::open(path) {
                count += 1
            } else {
                return count;
            }
        }
    }

    /// check the dir if it exists. if not, create it
    fn check_dir(&self) {
        if !std::path::Path::new(&self.setting.dir_path).exists() {
            std::fs::create_dir(&self.setting.dir_path).expect("Failed to create directory");
        }
    }
}

#[cfg(feature = "async")]
impl Logger {
    /// Customize and initialize the log writer.
    pub(crate) async fn init(&mut self, setting: Setting) {
        if self.init {
            let position = position!().to_string();
            self.warn("Log writer had been initialized!", position)
                .await;
            return;
        }

        self.file = None;
        self.current_index = 0;
        self.used_length = 0;

        self.setting = setting;

        self.init = true;
        self.current_index = self.get_index(&self.current_file_prefix).await;
    }

    /// Change some setting after initialization.
    pub(crate) async fn set(&mut self, setting: Setting) {
        self.init = true;
        self.setting = setting;
        self.current_index = self.get_index(&self.current_file_prefix).await;
        self.file = Some(self.get_file().await);
    }

    /// clear the log directory. (remove all the log files in the directory)
    pub(crate) async fn clear_dir(&mut self) {
        fs::remove_dir_all(&self.setting.dir_path)
            .await
            .expect("Cannot remove the dir.");
        fs::create_dir(&self.setting.dir_path)
            .await
            .expect("Cannot create the dir.");
        self.current_index = 0;
        self.used_length = 0;
        self.file = None;
    }

    /// Write a single log message to the file.
    async fn write(&mut self, msg: &LogMessage) {
        // if the logger is disabled, return directly
        if self.setting.disabled {
            return;
        }

        for i in msg.split_enter() {
            if self.file.is_none() {
                self.file = Some(self.get_file().await);
            }

            // check if the time prefix has changed
            // (when a new day begins)
            let time_prefix = format!(
                "{}",
                chrono::Utc::now()
                    .with_timezone(&FixedOffset::east_opt(self.setting.time_zone * 3600).unwrap())
                    .format("%Y-%m-%d")
            );
            if self.current_file_prefix != time_prefix {
                self.current_file_prefix = time_prefix;
                self.current_index = 0;
                self.used_length = 0;
                self.file = Some(self.get_file().await);
            };

            // check if should print to terminal.
            // requirement: print out is enabled and the level is high enough
            if self.setting.print_out
                && self.setting.terminal_print_level.get_level() <= i.get_level()
            {
                println!("{}", i.print())
            };

            // check if should write to file.
            // requirement: the level is high enough
            if self.setting.file_record_level.get_level() <= i.get_level() {
                self.file
                    .as_mut()
                    .unwrap()
                    .write_all((i.print() + "\n").as_bytes())
                    .await
                    .expect("Cannot write into the log file.");
                self.used_length += 1;
            };
        }

        // check if the file is full or unlimited size
        if self.setting.single_length != 0 && self.used_length >= self.setting.single_length {
            self.current_index += 1;
            self.used_length = 0;
            self.file = None;
        }
    }

    /// provide a method to log something by only a given string and [`LogLevel`].
    pub async fn record(&mut self, log_level: LogLevel, message: &str, position: String) {
        if !self.init {
            self.init = true
        }
        let mut msg = LogMessage::new(
            log_level,
            message.to_string(),
            self.setting.time_zone,
            position,
        );
        msg.time.detailed_display = self.setting.time_detailed_display;
        self.write(&msg).await;
    }

    /// Record an info log.
    pub async fn info(&mut self, message: &str, position: String) {
        self.record(LogLevel::Info, message, position).await;
    }

    /// Record a debug log.
    pub async fn debug(&mut self, message: &str, position: String) {
        self.record(LogLevel::Debug, message, position).await;
    }

    /// Record a warn log.
    pub async fn warn(&mut self, message: &str, position: String) {
        self.record(LogLevel::Warn, message, position).await;
    }

    /// Record an error log.
    pub async fn error(&mut self, message: &str, position: String) {
        self.record(LogLevel::Error, message, position).await;
    }

    /// Record a trace log.
    pub async fn trace(&mut self, message: &str, position: String) {
        self.record(LogLevel::Trace, message, position).await;
    }

    /// Get the file object of the log file.
    async fn get_file(&self) -> File {
        let path = self.get_path(&self.current_file_prefix, self.current_index);
        // enable read and write and create a new file if not exist
        File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)
            .await
            .expect("Cannot create the log file.")
    }

    /// Get the index of the current log file.
    /// This is used when resume the logging, since have to keep a continuos order of the log files.
    async fn get_index(&self, time_prefix: &str) -> usize {
        let mut count = 0;
        loop {
            let path = self.get_path(time_prefix, count);
            // if the file exists, then the index is the next one
            if let Ok(_) = File::open(path).await {
                count += 1
            } else {
                return count;
            }
        }
    }
}

#[cfg(not(feature = "async"))]
impl Logger {
    /// Customize and initialize the log writer.
    pub(crate) fn init(&mut self, setting: Setting) {
        if self.init {
            let position = position!().to_string();
            self.warn("Log writer had been initialized!", position);
            return;
        }

        self.file = None;
        self.current_index = 0;
        self.used_length = 0;

        self.setting = setting;

        self.init = true;
        self.current_index = self.get_index(&self.current_file_prefix);
    }

    /// Change some setting after initialization.
    pub(crate) fn set(&mut self, setting: Setting) {
        self.init = true;
        self.setting = setting;
        self.current_index = self.get_index(&self.current_file_prefix);
        self.file = Some(self.get_file());
    }

    /// clear the log directory.
    pub(crate) fn clear_dir(&mut self) {
        fs::remove_dir_all(&self.setting.dir_path).expect("Cannot remove the dir.");
        fs::create_dir(&self.setting.dir_path).expect("Cannot create the dir.");
        self.current_index = 0;
        self.used_length = 0;
        self.file = None;
        self.current_file_prefix = format!(
            "{}",
            chrono::Utc::now()
                .with_timezone(&FixedOffset::east_opt(self.setting.time_zone * 3600).unwrap())
                .format(&self.setting.file_time_format)
        );
    }

    /// Write a single log message to the file.
    fn write(&mut self, msg: &LogMessage) {
        if self.setting.disabled {
            return;
        }

        if self.file.is_none() {
            self.file = Some(self.get_file());
        }

        for i in msg.split_enter() {
            // check if the time prefix has changed
            // (when a new day begins)
            let time_prefix = format!(
                "{}",
                chrono::Utc::now()
                    .with_timezone(&FixedOffset::east_opt(self.setting.time_zone * 3600).unwrap())
                    .format("%Y-%m-%d")
            );
            if self.current_file_prefix != time_prefix {
                self.current_file_prefix = time_prefix;
                self.current_index = 0;
                self.used_length = 0;
                self.file = Some(self.get_file());
            };

            // check if should print to terminal.
            // requirement: print out is enabled and the level is high enough
            if self.setting.print_out
                && self.setting.terminal_print_level.get_level() <= i.get_level()
            {
                println!("{}", i.print())
            };

            // check if should write to file.
            // requirement: the level is high enough
            if self.setting.file_record_level.get_level() <= i.get_level() {
                self.file
                    .as_mut()
                    .unwrap()
                    .write_all((i.print() + "\n").as_bytes())
                    .expect("Cannot write into the log file.");
                self.used_length += 1;
            };
        }

        if self.setting.single_length != 0 && self.used_length >= self.setting.single_length {
            self.current_index += 1;
            self.used_length = 0;
            self.file = None;
        }
    }

    /// provide a method to log something by only a given string and [`LogLevel`].
    pub fn record(&mut self, log_level: LogLevel, message: &str, position: String) {
        if !self.init {
            self.init = true
        }
        let mut msg = LogMessage::new(
            log_level,
            message.to_string(),
            self.setting.time_zone,
            position,
        );
        msg.time.detailed_display = self.setting.time_detailed_display;
        self.write(&msg);
    }

    /// Record an info log.
    pub fn info(&mut self, message: &str, position: String) {
        self.record(LogLevel::Info, message, position);
    }

    /// Record a debug log.
    pub fn debug(&mut self, message: &str, position: String) {
        self.record(LogLevel::Debug, message, position);
    }

    /// Record a warn log.
    pub fn warn(&mut self, message: &str, position: String) {
        self.record(LogLevel::Warn, message, position);
    }

    /// Record an error log.
    pub fn error(&mut self, message: &str, position: String) {
        self.record(LogLevel::Error, message, position);
    }

    /// Record a trace log.
    pub fn trace(&mut self, message: &str, position: String) {
        self.record(LogLevel::Trace, message, position);
    }

    /// Get the index of the current log file.
    /// This is used when resume the logging, since have to keep a continuos order of the log files.
    fn get_index(&self, time_prefix: &str) -> usize {
        let mut count = 0;
        loop {
            let path = self.get_path(time_prefix, count);
            // if the file exists, then the index is the next one
            if let Ok(_) = File::open(path) {
                count += 1
            } else {
                return count;
            }
        }
    }

    /// Get the file object of the log file.
    fn get_file(&self) -> File {
        let path = self.get_path(&self.current_file_prefix, self.current_index);
        // enable read and write and create a new file if not exist
        File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)
            .expect("Cannot create the log file.")
    }
}

unsafe impl Send for Logger {}
