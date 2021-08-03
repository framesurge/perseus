use crate::errors::*;
use chrono::{Duration, Utc};

// Decodes time strings like '1w' into actual datetimes from the present moment. If you've ever used NodeJS's [`jsonwebtoken`](https://www.npmjs.com/package/jsonwebtoken) module, this is
/// very similar (based on Vercel's [`ms`](https://github.com/vercel/ms) module for JavaScript).
/// Accepts strings of the form 'xXyYzZ...', where the lower-case letters are numbers meaning a number of the intervals X/Y/Z (e.g. 1m4d -- one month four days).
/// The available intervals are:
///
/// - s: second,
/// - m: minute,
/// - h: hour,
/// - d: day,
/// - w: week,
/// - M: month (30 days used here, 12M ≠ 1y!),
/// - y: year (365 days always, leap years ignored, if you want them add them as days)
pub fn decode_time_str(time_str: &str) -> Result<String> {
    let mut duration_after_current = Duration::zero();
    // Get the current datetime since Unix epoch, we'll add to that
    let current = Utc::now();
    // A working variable to store the '123' part of an interval until we reach the idnicator and can do the full conversion
    let mut curr_duration_length = String::new();
    // Iterate through the time string's characters to get each interval
    for c in time_str.chars() {
        // If we have a number, append it to the working cache
        // If we have an indicator character, we'll match it to a duration
        if c.is_numeric() {
            curr_duration_length.push(c);
        } else {
            // Parse the working variable into an actual number
            let interval_length = curr_duration_length.parse::<i64>().unwrap(); // It's just a string of numbers, we know more than the compiler
            let duration = match c {
                's' => Duration::seconds(interval_length),
                'm' => Duration::minutes(interval_length),
                'h' => Duration::hours(interval_length),
                'd' => Duration::days(interval_length),
                'w' => Duration::weeks(interval_length),
                'M' => Duration::days(interval_length * 30), // Multiplying the number of months by 30 days (assumed length of a month)
                'y' => Duration::days(interval_length * 365), // Multiplying the number of years by 365 days (assumed length of a year)
                c => bail!(ErrorKind::InvalidDatetimeIntervalIndicator(c.to_string())),
            };
            duration_after_current = duration_after_current + duration;
            // Reset that working variable
            curr_duration_length = String::new();
        }
    }
    // Form the final duration by reducing the durations vector into one
    let datetime = current + duration_after_current;

    // We return an easily parsible format (RFC 3339)
    Ok(datetime.to_rfc3339())
}