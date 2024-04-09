use chrono::{DateTime, FixedOffset, Utc};
use std::env;

/// Provide a much easier time struct for the project.
/// This will used all the place where time stuff is used.
#[derive(Clone)]
pub struct Time {
    utc: DateTime<Utc>,
    time_offset: FixedOffset,

    /// False - print out the local time (after calculating the timezone).
    /// True  - print out the utc time with timezone.
    pub detailed_display: bool, 
}

impl Time {
    /// Need to manually provide the utc time, and timezone information.
    pub fn new(utc: DateTime<Utc>, time_offset: FixedOffset) -> Self {
        Self { utc, time_offset, detailed_display: false }
    }

    /// Auto get the utc time, but still need to manually give the timezone information.
    pub fn now(time_zone: i32) -> Self {
        Self::new(Utc::now(), FixedOffset::east_opt(time_zone * 3600).unwrap())
    }

    /// Auto get the utc time.
    /// Read the filed "TIME_ZONE" from environment variables. If failed to read, then automatically set as 0.
    pub fn now_auto_offset() -> Self {
        let time_zone = env::var("TIME_ZONE").unwrap_or("0".to_string());
        let time_zone = time_zone.parse::<i32>().unwrap();
        Self::now(time_zone)
    }

    /// Provide a public function to set the "detailed_display" field.
    /// Actually this field can be changed directly, not through using this function.
    pub fn set_detailed_display(&mut self, detailed_display: bool) {
        self.detailed_display = detailed_display;
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

impl From<Time> for String {
    fn from(value: Time) -> Self {
        value.to_string()
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