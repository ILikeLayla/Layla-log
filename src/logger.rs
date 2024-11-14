use super::{msg::LogMessage, LogLevel};
use std::fs::{self, File};
use std::io::Write;

/// A writer for buffering the log and writing them into the suitable files.
pub struct Logger {
    // the file that is currently being written.
    file: Option<File>,
    // where stores the log files.
    dir_path: String,
    // the maximum number of logs in a single file.
    single_length: usize,
    // the current index of the file.
    current_index: usize,
    // a buffer for the log that are waiting to be written.
    log_buffer: Vec<LogMessage>,
    // define the minimum level of the log that should be written. (inclusive)
    file_record_level: LogLevel,
    // define the minimum level of the log that should be printed. (inclusive)
    terminal_print_level: LogLevel,
    // define to write the detailed time or not.
    time_detailed_display: bool,
    // the length of the log that have been written.
    used_length: usize,
    // the prefix of the time.
    time_prefix: String,
    // the time zone of the log.
    time_zone: i32,
    // setting whether to print the log to the terminal.
    print_out: bool,
    // check if the writer is initialized.
    init: bool,
}

impl Logger {
    /// Initialize the logger with all default setting.
    pub fn new() -> Self {
        let buffer = Self {
            dir_path: String::from("./logs/"),
            single_length: 0,
            current_index: 0,
            log_buffer: Vec::new(),
            file_record_level: LogLevel::Info,
            terminal_print_level: LogLevel::Warn,
            time_detailed_display: false,
            used_length: 0,
            time_prefix: format!("{}", chrono::Utc::now().format("%Y-%m-%d")),
            file: None,
            time_zone: 0,
            print_out: false,
            init: false,
        };
        buffer.check_dir();
        buffer
    }

    /// Customize and initialize the log writer.
    pub fn init(&mut self, dir_path: &str, single_length: usize, file_record_level: LogLevel, terminal_print_level: LogLevel, time_zone: i32, time_detailed_display: bool, print_out: bool) {
        if self.init {
            self.warn("Log writer has been initialized!");
            return;
        }
        
        self.dir_path = dir_path.to_string();
        self.single_length = single_length;
        self.file_record_level = file_record_level;
        self.terminal_print_level = terminal_print_level;
        self.log_buffer = Vec::with_capacity(single_length);
        self.time_detailed_display = time_detailed_display;
        self.time_prefix = format!("{}", chrono::Utc::now().format("%Y-%m-%d"));
        self.used_length = 0;
        self.file = Some(self.get_file());
        self.time_zone = time_zone;
        self.print_out = print_out;
        self.current_index = self.get_index(&self.time_prefix);
        self.check_dir();

        self.init = true;
        
    }

    /// Initialize the log writer with the default settings.
    pub fn default(&mut self, dir_path: &str) { 
        self.init(dir_path, 200, LogLevel::Trace, LogLevel::Debug, 0, false, true)
    }

    /// check the dir if it exists. if not, create it
    fn check_dir(&self) {
        if !std::path::Path::new(&self.dir_path).exists() {
            fs::create_dir(&self.dir_path).expect("Failed to create directory");
        }
    }
  
    /// Push a log into the buffer.
    pub fn push(&mut self, log_level: LogLevel, message: &str) {
        let log_message = LogMessage::new(log_level, message.to_string(), self.time_zone);
        self.log_buffer.push(log_message);
    }

    /// clear the log directory.
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
        for msg in self.log_buffer.clone().iter_mut() {
            if self.time_detailed_display { msg.set_detailed_time() } else { msg.set_rough_time() }
            self.write_single(msg);

            // check if the file is full
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

    /// Record a trace log.
    pub fn trace(&mut self, message: &str) {
        self.record(LogLevel::Trace, message);
    }

    /// Write a single log message to the file.
    fn write_single(&mut self, msg: &LogMessage) {
        for i in msg.split_enter() {
            // check if the time prefix has changed
            // (when a new day begins)
            let time_prefix = format!("{}", chrono::Utc::now().format("%Y-%m-%d"));
            if self.time_prefix != time_prefix {
                self.time_prefix = time_prefix;
                self.current_index = 0;
                self.used_length = 0;
                self.file = Some(self.get_file());
            };

            // check if should print to terminal.
            // requirement: print out is enabled and the level is high enough
            if self.print_out && self.terminal_print_level.get_level() <= i.get_level() { println!("{}", i.print()) };

            // check if should write to file.
            // requirement: the level is high enough
            if self.file_record_level.get_level() <= i.get_level() {
                self.file.as_mut().unwrap().write_all((i.print() + "\n").as_bytes()).expect("Cannot write into the log file.");
                self.used_length += 1;
            };
        }
    }

    /// Get the index of the current log file.
    /// This is used when resume the logging, since have to keep a continuos order of the log files.
    fn get_index(&self, time_prefix: &str) -> usize {
        let mut count = 1;
        loop {
            let path = self.get_path(time_prefix, count);
            // if the file exists, then the index is the next one
            if let Ok(_) = File::open(path) { 
                count += 1
            } else {
                return count
            }
        }
    }

    /// Get the path of the log file.
    fn get_path(&self, time_prefix: &str, index: usize) -> String {
        format!("{}/{}_{}.log", self.dir_path, time_prefix, index)
    }

    /// Get the file object of the log file.
    fn get_file(&self) -> File {
        let path = self.get_path(&self.time_prefix, self.current_index);
        // enable read and write and create a new file if not exist
        File::options().read(true).write(true).create_new(true).open(path).expect("Cannot create the log file.")
    }
}