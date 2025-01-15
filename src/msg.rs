use super::{time::Time, LogLevel};

#[derive(Clone, Debug)]
pub(crate) struct LogMessage {
    // level of the log
    level: LogLevel,
    // message of the log
    message: String,
    // time of the log
    pub(crate) time: Time,
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

unsafe impl Send for LogMessage {}