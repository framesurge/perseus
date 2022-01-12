use chrono::Utc;
use std::time;

/// Represents a duration that can be computed relative to the current time.
#[derive(Debug, Clone)]
pub struct ComputedDuration(chrono::Duration);

impl ComputedDuration {
    pub fn new<I: Into<time::Duration>>(duration: I) -> Self {
        let duration = chrono::Duration::from_std(duration.into()).unwrap();
        Self(duration)
    }

    /// Get the timestamp of the duration added to the current time.
    pub fn compute_timestamp(&self) -> String {
        let current = Utc::now();
        let datetime = current + self.0;
        datetime.to_rfc3339()
    }
}
