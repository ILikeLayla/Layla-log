use super::{msg::LogMessage, LogLevel};
use std::{fs::{self, File}, io::Write};

/// A writer for buffering the log and writing them into the suitable files.
pub struct Writer {
    // where stores the log files.
    dir_path: String,
    // the maximum number of logs in a single file.
    single_length: usize,
    // the current index of the file.
    current_index: usize,
    // a buffer for the log that are waiting to be written.
    log_buffer: Vec<LogMessage>,
    // define the minimum level of the log that should be written. (inclusive)
    log_level: LogLevel,
    // define to write the detailed time or not.
    time_details: bool,
    used_length: usize,
    time_prefix: String,
    file: Option<File>,
    time_zone: i32
}

// %y-%m-%d-%i
impl Writer {
    /// Customize and initialize the log writer.
    pub fn new(dir_path: &str, single_length: Option<usize>, log_level: Option<LogLevel>, time_zone: i32, time_details: bool) -> Writer {
        let single_length = single_length.unwrap_or(200);
        let time_prefix = format!("{}", chrono::Utc::now().format("%Y-%m-%d"));
        let mut buffer = Self {
            dir_path: dir_path.to_string(),
            single_length,
            current_index: 1,
            log_level: log_level.unwrap_or(LogLevel::Warn),
            log_buffer: Vec::with_capacity(single_length),
            time_details,
            time_prefix,
            used_length: 0,
            file: None,
            time_zone
        };
        buffer.current_index = buffer.get_index(&buffer.time_prefix);
        buffer.file = Some(buffer.get_file());
        buffer
    }
  
    /// Initialize the log writer with the default settings.
    pub fn default(dir_path: &str) -> Writer { Writer::new(dir_path, None, None, 0, false) }

    /// Push a log into the buffer.
    pub fn push(&mut self, log_level: LogLevel, message: &str) {
        let log_message = LogMessage::new(log_level, message.to_string(), self.time_zone);
        self.log_buffer.push(log_message);
    }

    pub fn clear_dir(&mut self) {
        fs::remove_dir_all(&self.dir_path).expect("Cannot remove the dir.");
        fs::create_dir(&self.dir_path).expect("Cannot create the dir.");
        self.current_index = 0;
        self.used_length = 0;
        self.file = Some(self.get_file())
    }
    /// Write all the logs in the buffer into the files.
    /// and also clear the buffer.
    pub fn write_all(&mut self) {
        let restriction = self.log_level as usize;
        for msg in self.log_buffer.clone().iter_mut() {
            if msg.get_level() < restriction { continue; }
            if self.time_details { msg.set_detailed_time() } else { msg.set_rough_time() }
            self.write_single(msg);
            if self.used_length >= self.single_length && self.single_length != 0 { 
                self.current_index += 1;
                self.used_length = 0;
                self.file = Some(self.get_file());
            }
        }
        self.log_buffer.clear();
    }

    /// Combine the push and write_all methods.
    pub fn record(&mut self, log_level: LogLevel, message: &str) {
        self.push(log_level, message);
        self.write_all();
    }

    /// Record an info log.
    pub fn info(&mut self, message: &str) {
        self.record(LogLevel::Info, message);
    }

    /// Record a debug log.
    pub fn debug(&mut self, message: &str) {
        self.record(LogLevel::Debug, message);
    }

    /// Record a warn log.
    pub fn warn(&mut self, message: &str) {
        self.record(LogLevel::Warn, message);
    }

    /// Record an error log.
    pub fn error(&mut self, message: &str) {
        self.record(LogLevel::Error, message);
    }

    fn write_single(&mut self, msg: &LogMessage) {
        for i in msg.split_enter() {
            let time_prefix = format!("{}", chrono::Utc::now().format("%Y-%m-%d"));
            if self.time_prefix != time_prefix {
                self.time_prefix = time_prefix;
                self.current_index = 0;
                self.used_length = 0;
                self.file = Some(self.get_file());
            };
            self.file.as_mut().unwrap().write_all((i.print() + "\n").as_bytes()).expect("Cannot write into the log file.");
            self.used_length += 1;
        }
    }

    fn get_index(&self, time_prefix: &str) -> usize {
        let mut count = 1;
        loop {
            let path = self.get_path(time_prefix, count);
            if let Ok(_) = File::open(path) { count += 1 }
            else {
                // let _ = File::create(self.get_path(&self.time_prefix, count)).expect("Cannot create the log file.");
                return count
            }
        }
    }

    fn get_path(&self, time_prefix: &str, index: usize) -> String {
        format!("{}\\{}-{}.log", self.dir_path, time_prefix, index)
    }

    fn get_file(&self) -> File {
        let path = self.get_path(&self.time_prefix, self.current_index);
        File::options().read(true).write(true).create_new(true).open(path).expect("Cannot create the log file.")
    }
}