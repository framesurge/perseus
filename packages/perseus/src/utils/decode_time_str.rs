use chrono::Utc;
use std::{convert::TryFrom, time};

/// Represents a duration that can be computed relative to the current time.
#[derive(Debug, Clone)]
pub struct ComputedDuration(chrono::Duration);

impl ComputedDuration {
    /// Creates a new [`ComputedDuration`] from the given duration-like type.
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

/// A simpler representation of a duration based on individual components.
///
/// Note that months are assumed to be 30 days long, and years 365 days long.
#[derive(Default, Debug)]
pub struct Duration {
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
}

/// An error type for invalid `String` durations.
pub struct InvalidDuration;

impl From<Duration> for time::Duration {
    fn from(duration: Duration) -> Self {
        let duration = chrono::Duration::seconds(duration.seconds)
            + chrono::Duration::minutes(duration.minutes)
            + chrono::Duration::hours(duration.hours)
            + chrono::Duration::days(duration.days)
            + chrono::Duration::weeks(duration.weeks)
            + chrono::Duration::days(duration.months * 30)  // Assumed length of a month
            + chrono::Duration::days(duration.years * 365); // Assumed length of a year

        duration.to_std().unwrap()
    }
}

impl TryFrom<&str> for Duration {
    type Error = InvalidDuration;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut duration = Self::default();

        // A working variable to store the '123' part of an interval until we reach the indicator and can do the full conversion
        let mut curr_duration_length = String::new();

        for c in value.chars() {
            // If we have a number, append it to the working cache
            // If we have an indicator character, we'll match it to a duration
            if c.is_numeric() {
                curr_duration_length.push(c);
            } else {
                let interval_length: i64 = curr_duration_length.parse().unwrap(); // It's just a string of numbers, we know more than the compiler
                if interval_length <= 0 {
                    return Err(InvalidDuration);
                }

                match c {
                    's' if duration.seconds == 0 => duration.seconds = interval_length,
                    'm' if duration.minutes == 0 => duration.minutes = interval_length,
                    'h' if duration.hours == 0 => duration.hours = interval_length,
                    'd' if duration.days == 0 => duration.days = interval_length,
                    'w' if duration.weeks == 0 => duration.weeks = interval_length,
                    'M' if duration.months == 0 => duration.months = interval_length,
                    'y' if duration.years == 0 => duration.years = interval_length,
                    _ => return Err(InvalidDuration),
                };

                curr_duration_length = String::new();
            }
        }

        Ok(duration)
    }
}
