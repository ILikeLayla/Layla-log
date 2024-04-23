use super::{time::Time, LogLevel};

#[derive(Clone)]
pub struct LogMessage {
    level: LogLevel,
    message: String,
    time: Time,
}

impl LogMessage {
    pub fn new(level: LogLevel, message: String, time_zone:i32) -> Self {
        Self {
            level,
            message,
            time: Time::now(time_zone),
        }
    }

    pub fn set_detailed_time(&mut self) {
        self.time.detailed_display = true;
    }

    pub fn set_rough_time(&mut self) {
        self.time.detailed_display = false;
    }

    pub fn print(&self) -> String {
        format!("{} {} {}", self.time, self.level, self.message)
    }

    pub fn get_level(&self) -> usize {
        self.level as usize
    }

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
