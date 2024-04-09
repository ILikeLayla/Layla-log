use super::{LogMessage, LogLevel};
use std::{fs::File, io::Write};

pub struct Writer {
    dir_path: String,
    single_length: usize,
    used_length: usize,
    current_index: usize,
    log_buffer: Vec<LogMessage>,
    file_prefix: String,
    log_level: LogLevel,
    detailed_time: bool,
}

impl Writer {
    pub fn new(dir_path: &str, detailed_time: bool, log_level: Option<LogLevel>, single_length_buffer: Option<usize>, prefix: Option<&str>) -> Writer {
        let single_length = single_length_buffer.unwrap_or(100);
        let file_prefix = prefix.unwrap_or("LOG").to_string();
        let mut buffer = Self {
            dir_path: dir_path.to_string(),
            single_length,
            file_prefix,
            used_length: 0,
            current_index: 0,
            log_level: log_level.unwrap_or(LogLevel::Warn),
            log_buffer: Vec::with_capacity(single_length),
            detailed_time
        };
        buffer.check_current_index();
        buffer
    }

    pub fn default(dir_path: &str) -> Writer { Writer::new(dir_path, false, None, None, None) }

    pub fn push(&mut self, log_level: LogLevel, message: &str) {
        let log_message = LogMessage::new(log_level, message.to_string());
        self.log_buffer.push(log_message);
        println!("{}", self.log_buffer.len());
    }

    pub fn write_all(&mut self) {
        let restriction = self.log_level as usize;
        let file_path = self.dir_path.clone() + "\\" + &self.file_prefix.clone() + &self.current_index.to_string();
        let mut file = File::create(file_path).expect("Cannot create the log file.");
        for msg in self.log_buffer.clone().iter_mut() {
            if msg.get_level() < restriction { continue; }
            if self.detailed_time { msg.set_detailed_time() } else { msg.set_rough_time() }
            self.write_single(&mut file, msg);
            if self.used_length >= self.single_length { 
                self.current_index += 1;
                self.used_length = 0;
                let file_path = self.dir_path.clone() + "\\" + &self.file_prefix.clone() + &self.current_index.to_string();
                file = File::create(file_path).expect("Cannot create the log file.");
            }
        }
        self.log_buffer.clear();
    }

    fn write_single(&mut self, file: &mut File, msg: &LogMessage) {
        file.write_all((msg.print() + "\n").as_bytes()).expect("Cannot write into the log file.");
        self.used_length += 1;
    }

    fn check_current_index(&mut self) {
        let mut buffer = 0;
        loop {
            let file_path = self.dir_path.clone() + "\\" + &self.file_prefix.clone() + &buffer.to_string();
            if File::open(file_path).is_ok() {
                buffer += 1;
            } else {
                self.current_index = buffer;
                break;
            }
        }
    }

    pub fn record(&mut self, log_level: LogLevel, message: &str) {
        self.push(log_level, message);
        self.write_all();
    }
}