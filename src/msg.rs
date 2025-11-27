#[cfg(feature = "async")]
use super::block_on;
use super::{time::Time, LogLevel, PositionTag, LOGSETTING};

#[derive(Clone, Debug)]
pub struct LogMessage {
    // level of the log
    level: LogLevel,
    // message of the log
    message: String,
    // time of the log
    time: Time,
    // position
    position: PositionTag,
}

impl LogMessage {
    /// Creates a new log message
    pub fn new(level: LogLevel, message: String, position: PositionTag) -> Self {
        #[cfg(not(feature = "async"))]
        let time_zone = LOGSETTING.lock().unwrap().time_zone;
        #[cfg(feature = "async")]
        let time_zone = block_on(async { LOGSETTING.lock().await.time_zone });
        Self {
            level,
            message,
            position,
            time: Time::now(time_zone),
        }
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
        #[cfg(not(feature = "async"))]
        let setting = LOGSETTING.lock().unwrap();
        #[cfg(feature = "async")]
        let setting = block_on(async { LOGSETTING.lock().await });
        let position_tag = match (setting.display_scope, setting.display_path) {
            (true, true) => format!(" [{} @ {}]", self.position.scope, self.position.path),
            (true, false) => format!(" [{}]", self.position.scope),
            (false, true) => format!(" [{}]", self.position.path),
            (false, false) => String::new(),
        };
        // drop the lock to allow the format function of self.time can get the lock
        drop(setting);
        write!(
            f,
            "{} {}{} {}",
            self.time, self.level, position_tag, self.message
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[cfg(not(feature = "async"))]
    use crate::{log_set, position};

    #[cfg(not(feature = "async"))]
    #[test]
    pub fn create_a_message() {
        log_set! {
            time_zone: 1,
            time_detailed_display: true
        };
        let log = LogMessage::new(LogLevel::Info, "test".to_string(), position!());
        println!("{}", log);
    }

    #[test]
    fn print_message() {
        let log = LogMessage {
            position: PositionTag {
                scope: "test_scoop".to_string(),
                path: "test_path".to_string(),
            },
            level: LogLevel::Info,
            message: "test message".to_string(),
            time: Time::now(0),
        };
        println!("{}", log)
    }
}
