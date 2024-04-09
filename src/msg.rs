use super::Time;

#[derive(Copy, Clone)]
pub enum LogLevel {
    Info = 0,
    Debug = 1,
    Warn = 2,
    Error = 3,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Clone)]
pub struct LogMessage {
    level: LogLevel,
    message: String,
    time: Time,
}

impl LogMessage {
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            level,
            message,
            time: Time::now_auto_offset(),
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
}
