use super::{time::Time, LogLevel};

#[derive(Clone)]
pub struct LogMessage {
    // level of the log
    level: LogLevel,
    // message of the log
    message: String,
    // time of the log
    time: Time,
}

impl LogMessage {

    /// Creates a new log message
    pub fn new(level: LogLevel, message: String, time_zone:i32) -> Self {
        Self {
            level,
            message,
            time: Time::now(time_zone),
        }
    }

    /// Sets the time of the log to detailed display
    pub fn set_detailed_time(&mut self) {
        self.time.detailed_display = true;
    }

    /// Sets the time of the log to rough display
    pub fn set_rough_time(&mut self) {
        self.time.detailed_display = false;
    }

    /// Formatting the log message
    pub fn print(&self) -> String {
        format!("{} {}\t{}", self.time, self.level, self.message)
    }

    /// Get the level of the log
    pub fn get_level(&self) -> usize {
        self.level as usize
    }

    /// Deal with the log with multiline.
    /// Convert multiline log into multiple single line log.
    pub fn split_enter(&self) -> Vec<Self> {
        let mut messages = Vec::new();
        for line in self.message.lines() {
            messages.push(Self {
                level: self.level.clone(),
                message: line.to_string(),
                time: self.time.clone(),
            });
        }
        messages
    }
}
