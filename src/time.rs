#[cfg(feature = "async")]
use super::block_on;
use super::LOGSETTING;
use chrono::{DateTime, FixedOffset, Utc};

#[derive(Clone, Debug)]
pub(crate) struct Time {
    // UTC time
    utc: DateTime<Utc>,
    // Time zone offset
    time_offset: FixedOffset,
}

impl Time {
    /// Pass a time and a specified time zone
    pub fn new(utc: DateTime<Utc>, time_offset: FixedOffset) -> Self {
        Self { utc, time_offset }
    }

    /// Get the current time with specified time zone
    pub fn now(time_zone: i32) -> Self {
        Self::new(Utc::now(), FixedOffset::east_opt(time_zone * 3600).unwrap())
    }

    /// Format the time
    fn to_string(&self) -> String {
        let format = "%Y-%m-%d %H:%M:%S%.3f".to_string();
        #[cfg(not(feature = "async"))]
        let time_detailed_display = LOGSETTING.lock().unwrap().time_detailed_display;
        #[cfg(feature = "async")]
        let time_detailed_display =
            block_on(async { LOGSETTING.lock().await.time_detailed_display });
        // check the setting in the [`LOGSETTING`] (require lock)
        if time_detailed_display {
            format!(
                "{} ({})",
                self.utc.with_timezone(&self.time_offset).format(&format),
                self.time_offset
            )
        } else {
            (self.utc.with_timezone(&self.time_offset))
                .format(&format)
                .to_string()
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

unsafe impl Send for Time {}
