use super::{time::Time, LogLevel};

#[derive(Clone, Debug)]
pub struct LogMessage {
    // level of the log
    level: LogLevel,
    // message of the log
    message: String,
    // time of the log
    pub(crate) time: Time,
    // position
    position: String,
}

impl LogMessage {
    /// Creates a new log message
    pub fn new(level: LogLevel, message: String, time_zone: i32, position: String) -> Self {
        Self {
            level,
            message,
            position,
            time: Time::now(time_zone),
        }
    }

    /// Formatting the log message
    pub fn print(&self) -> String {
        format!(
            "{} {} [{}] {}",
            self.time, self.level, self.position, self.message
        )
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
                position: self.position.clone(),
                level: self.level.clone(),
                message: line.to_string(),
                time: self.time.clone(),
            });
        }
        messages
    }
}

unsafe impl Send for LogMessage {}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn print_message() {
        let log = LogMessage {
            position: "test".to_string(),
            level: LogLevel::Info,
            message: "test message".to_string(),
            time: Time::now(0),
        };
        println!("{}", log)
    }
}
