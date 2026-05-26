use super::{msg::LogMessage, LogLevel, LogSetting, PositionTag, LOGSETTING};
use chrono::FixedOffset;
#[cfg(not(feature = "async"))]
use std::fs::{self, File};
#[cfg(not(feature = "async"))]
use std::io::Write;
#[cfg(feature = "async")]
use tokio::fs::{self, File};
#[cfg(feature = "async")]
use tokio::io::AsyncWriteExt;

fn get_path(dir_path: &str, time_prefix: &str, index: usize) -> String {
    format!("{}/{}_{}.log", dir_path, time_prefix, index)
}

#[cfg(not(feature = "async"))]
fn check_dir(dir_path: &str) {
    if !std::path::Path::new(dir_path).exists() {
        std::fs::create_dir(dir_path).expect("Failed to create directory");
    }
}

#[cfg(feature = "async")]
async fn check_dir(dir_path: &str) {
    if !tokio::fs::metadata(dir_path).await.is_ok() {
        tokio::fs::create_dir(dir_path)
            .await
            .expect("Failed to create directory");
    }
}

/// A writer for buffering the log and writing them into the suitable files.
#[derive(Debug)]
pub struct Logger<'a: 'static> {
    /// the file that is currently being written.
    file: Option<File>,
    /// the current index of the file.
    current_index: usize,
    /// the length of the log that have been written.
    used_length: usize,
    /// a buffer to store the prefix of log files' name.
    current_file_prefix: &'a str,
    /// to record down the logger is enabled or not.
    enable: bool,
}

impl<'a> Logger<'a> {
    /// Initialize the logger with all default setting.
    pub(crate) fn new() -> Self {
        let default_setting = LogSetting::default();

        let buffer = Self {
            file: None,
            current_index: 0,
            used_length: 0,
            current_file_prefix: Box::leak(
                format!(
                    "{}",
                    chrono::Utc::now()
                        .with_timezone(
                            &FixedOffset::east_opt(default_setting.time_zone * 3600).unwrap()
                        )
                        .format(&default_setting.file_time_format)
                )
                .into_boxed_str(),
            ),
            enable: true,
        };
        buffer
    }

    pub fn enable(&mut self) {
        self.enable = true;
    }

    pub fn disable(&mut self) {
        self.enable = false;
    }
}

#[cfg(feature = "async")]
impl<'a> Logger<'a> {
    /// clear the log directory. (remove all the log files in the directory)
    pub async fn clear_dir(&mut self) {
        let setting = LOGSETTING.lock().await;
        check_dir(&setting.dir_path).await;
        fs::remove_dir_all(&setting.dir_path)
            .await
            .expect("Cannot remove the dir.");
        fs::create_dir(&setting.dir_path)
            .await
            .expect("Cannot create the dir.");
        self.current_index = 0;
        self.used_length = 0;
        self.file = None;
    }

    /// Write a single log message to the file.
    async fn write(&mut self, msg: &LogMessage) {
        let setting = LOGSETTING.lock().await;
        if !self.enable {
            return;
        }

        if self.file.is_none() {
            self.current_index = self
                .get_index(&setting.dir_path, &self.current_file_prefix)
                .await;
            self.file = Some(self.get_file(&setting.dir_path).await);
        }

        // drop the lock to avoid deadlock
        drop(setting);

        for i in msg.split_enter() {
            let setting = LOGSETTING.lock().await;
            // check if the time prefix has changed
            // (when a new day begins)
            let time_prefix = format!(
                "{}",
                chrono::Utc::now()
                    .with_timezone(&FixedOffset::east_opt(setting.time_zone * 3600).unwrap())
                    .format("%Y-%m-%d")
            );
            if self.current_file_prefix != time_prefix {
                self.current_file_prefix = Box::leak(time_prefix.into_boxed_str());
                self.current_index = self
                    .get_index(&setting.dir_path, &self.current_file_prefix)
                    .await;
                self.used_length = 0;
                self.file = Some(self.get_file(&setting.dir_path).await);
            };

            // check if should print to terminal.
            // requirement: the log is significantly important and printout is enabled
            let printout_needed =
                setting.print_out && setting.terminal_print_level.get_level() <= i.get_level();
            // check if should write to file.
            // requirement: the log is significantly important and file record is enabled
            let file_needed = setting.file_record_level.get_level() <= i.get_level();

            drop(setting);

            if printout_needed {
                println!("{}", i)
            };

            if file_needed {
                self.file
                    .as_mut()
                    .unwrap()
                    .write_all(format!("{}\n", i).as_bytes())
                    .await
                    .expect("Cannot write into the log file.");
                self.used_length += 1;
            };
        }

        let setting = LOGSETTING.lock().await;
        // check if the file is full or unlimited size
        if setting.single_length != 0 && self.used_length >= setting.single_length {
            self.current_index += 1;
            self.used_length = 0;
            self.file = None;
        }
    }

    /// provide a method to log something by only a given string and [`LogLevel`].
    pub async fn record(&mut self, log_level: LogLevel, message: &str, position: PositionTag) {
        let msg = LogMessage::new(log_level, message.to_string(), position);
        self.write(&msg).await;
    }

    /// Record an info log.
    pub async fn info(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Info, message, position).await;
    }

    /// Record a debug log.
    pub async fn debug(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Debug, message, position).await;
    }

    /// Record a warn log.
    pub async fn warn(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Warn, message, position).await;
    }

    /// Record an error log.
    pub async fn error(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Error, message, position).await;
    }

    /// Record a trace log.
    pub async fn trace(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Trace, message, position).await;
    }

    /// Get the file object of the log file.
    async fn get_file(&self, dir_path: &str) -> File {
        let path = get_path(dir_path, &self.current_file_prefix, self.current_index);
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
    async fn get_index(&self, dir_path: &str, time_prefix: &str) -> usize {
        check_dir(dir_path).await;
        let mut count = 0;
        loop {
            let path = get_path(dir_path, time_prefix, count);
            // to find the last index which is not exist
            if let Ok(_) = File::open(path).await {
                count += 1
            } else {
                return count;
            }
        }
    }
}

#[cfg(not(feature = "async"))]
impl<'a> Logger<'a> {
    /// clear the log directory.
    pub fn clear_dir(&mut self) {
        let setting = LOGSETTING.lock().unwrap();
        check_dir(&setting.dir_path);
        fs::remove_dir_all(&setting.dir_path).expect("Cannot remove the dir.");
        fs::create_dir(&setting.dir_path).expect("Cannot create the dir.");
        self.current_index = 0;
        self.used_length = 0;
        self.file = None;
        self.current_file_prefix = Box::leak(
            format!(
                "{}",
                chrono::Utc::now()
                    .with_timezone(&FixedOffset::east_opt(setting.time_zone * 3600).unwrap())
                    .format(&setting.file_time_format)
            )
            .into_boxed_str(),
        );
    }

    /// Write a single log message to the file.
    fn write(&mut self, msg: &LogMessage) {
        let setting = LOGSETTING.lock().unwrap();
        if !self.enable {
            return;
        }

        if self.file.is_none() {
            self.current_index = self.get_index(&setting.dir_path, &self.current_file_prefix);
            self.file = Some(self.get_file(&setting.dir_path));
        }

        drop(setting);

        for i in msg.split_enter() {
            let setting = LOGSETTING.lock().unwrap();
            // check if the time prefix has changed
            // (when a new day begins)
            let time_prefix = format!(
                "{}",
                chrono::Utc::now()
                    .with_timezone(&FixedOffset::east_opt(setting.time_zone * 3600).unwrap())
                    .format("%Y-%m-%d")
            );
            if self.current_file_prefix != time_prefix {
                self.current_file_prefix = Box::leak(time_prefix.into_boxed_str());
                self.current_index = self.get_index(&setting.dir_path, &self.current_file_prefix);
                self.used_length = 0;
                self.file = Some(self.get_file(&setting.dir_path));
            };
            let printout_needed =
                setting.print_out && setting.terminal_print_level.get_level() <= i.get_level();
            let file_needed = setting.file_record_level.get_level() <= i.get_level();
            drop(setting);

            // check if should print to terminal.
            // requirement: print out is enabled and the level is high enough
            if printout_needed {
                println!("{}", i)
            };

            // check if should write to file.
            // requirement: the level is high enough
            if file_needed {
                self.file
                    .as_mut()
                    .unwrap()
                    .write_all((format!("{}\n", i)).as_bytes())
                    .expect("Cannot write into the log file.");
                self.used_length += 1;
            };
        }

        let setting = LOGSETTING.lock().unwrap();
        if setting.single_length != 0 && self.used_length >= setting.single_length {
            self.current_index += 1;
            self.used_length = 0;
            self.file = None;
        }
    }

    /// provide a method to log something by only a given string and [`LogLevel`].
    pub fn record(&mut self, log_level: LogLevel, message: &str, position: PositionTag) {
        let msg = LogMessage::new(log_level, message.to_string(), position);
        self.write(&msg);
    }

    /// Record an info log.
    pub fn info(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Info, message, position);
    }

    /// Record a debug log.
    pub fn debug(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Debug, message, position);
    }

    /// Record a warn log.
    pub fn warn(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Warn, message, position);
    }

    /// Record an error log.
    pub fn error(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Error, message, position);
    }

    /// Record a trace log.
    pub fn trace(&mut self, message: &str, position: PositionTag) {
        self.record(LogLevel::Trace, message, position);
    }

    /// Get the index of the current log file.
    /// This is used when resume the logging, since have to keep a continuos order of the log files.
    fn get_index(&self, dir_path: &str, time_prefix: &str) -> usize {
        check_dir(dir_path);
        let mut count = 0;
        loop {
            let path = get_path(dir_path, time_prefix, count);
            // if the file exists, then the index is the next one
            if let Ok(_) = File::open(path) {
                count += 1
            } else {
                return count;
            }
        }
    }

    /// Get the file object of the log file.
    fn get_file(&self, dir_path: &str) -> File {
        let path = get_path(dir_path, &self.current_file_prefix, self.current_index);
        // enable read and write and create a new file if not exist
        File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)
            .expect("Cannot create the log file.")
    }
}

unsafe impl<'a> Send for Logger<'a> {}
