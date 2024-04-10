use chrono::{DateTime, FixedOffset, Utc};
use std::env;

#[derive(Clone)]
pub struct Time {
    utc: DateTime<Utc>,
    time_offset: FixedOffset,
    pub detailed_display: bool, 
}

impl Time {
    pub fn new(utc: DateTime<Utc>, time_offset: FixedOffset) -> Self {
        Self { utc, time_offset, detailed_display: false }
    }

    pub fn now(time_zone: i32) -> Self {
        Self::new(Utc::now(), FixedOffset::east_opt(time_zone * 3600).unwrap())
    }

    pub fn now_auto_offset() -> Self {
        let time_zone = env::var("TIME_ZONE").unwrap_or("0".to_string());
        let time_zone = time_zone.parse::<i32>().unwrap();
        Self::now(time_zone)
    }

    fn to_string(&self) -> String {
        let format = env::var("TIME_FORMAT").unwrap_or("%Y-%m-%d %H:%M:%S%.3f".to_string());
        if self.detailed_display {
            format!("{} ({})", self.utc.format(&format), self.time_offset)
        } else {
            (self.utc.with_timezone(&self.time_offset)).format(&format).to_string()
        }
    }
}

impl From<&Time> for String {
    fn from(value: &Time) -> Self {
        value.to_string()
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}