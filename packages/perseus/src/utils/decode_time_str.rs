#[cfg(not(target_arch = "wasm32"))]
mod engine {
    use super::InvalidDuration;
    use chrono::Utc;
    use std::time;

    /// Represents a duration that can be computed relative to the current time.
    /// This should be created through [`PerseusDuration::into_computed`] only.
    #[derive(Debug, Clone)]
    pub struct ComputedDuration(chrono::Duration);

    impl ComputedDuration {
        /// Get the timestamp of the duration added to the current time.
        pub fn compute_timestamp(&self) -> String {
            let current = Utc::now();
            let datetime = current + self.0;
            datetime.to_rfc3339()
        }
    }

    /// A trait that represents anything we'll accept for specifying durations
    /// for revalidation. Anything that implements thus must be able to be
    /// transformed into a [`ComputedDuration`];
    pub trait PerseusDuration {
        /// Converts this into a `[ComputedDuration]` for use in Perseus'
        /// internal revalidation systems.
        fn into_computed(self) -> Result<ComputedDuration, InvalidDuration>;
    }

    // We'll accept strings, and standard library durations.
    // We don't accept Chrono durations because that would create a difference to
    // the browser dummy API (since Chrono is only used on the engine side), and
    // Chrono durations can be trivially converted into standard ones.
    impl PerseusDuration for chrono::Duration {
        fn into_computed(self) -> Result<ComputedDuration, InvalidDuration> {
            Ok(ComputedDuration(self))
        }
    }
    impl PerseusDuration for &str {
        // NOTE: Logic to define how we parse time strings is here
        fn into_computed(self) -> Result<ComputedDuration, InvalidDuration> {
            let mut duration = chrono::Duration::zero();

            // A working variable to store the '123' part of an interval until we reach the
            // indicator and can do the full conversion
            let mut curr_duration_length = String::new();

            for c in self.chars() {
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
                        's' => duration = duration + chrono::Duration::seconds(interval_length),
                        'm' => duration = duration + chrono::Duration::minutes(interval_length),
                        'h' => duration = duration + chrono::Duration::hours(interval_length),
                        'd' => duration = duration + chrono::Duration::days(interval_length),
                        'w' => duration = duration + chrono::Duration::weeks(interval_length),
                        'M' => duration = duration + chrono::Duration::days(interval_length * 30), /* Assumed length of a month */
                        'y' => duration = duration + chrono::Duration::days(interval_length * 365), /* Assumed length of a year */
                        _ => return Err(InvalidDuration),
                    };

                    curr_duration_length = String::new();
                }
            }

            Ok(ComputedDuration(duration))
        }
    }
    impl PerseusDuration for time::Duration {
        fn into_computed(self) -> Result<ComputedDuration, InvalidDuration> {
            let duration = chrono::Duration::from_std(self).map_err(|_| InvalidDuration)?;
            Ok(ComputedDuration(duration))
        }
    }
}
#[cfg(target_arch = "wasm32")]
mod browser {
    use super::InvalidDuration;
    use std::time;

    /// Represents a duration that can be computed relative to the current time.
    /// This should be created through [`PerseusDuration::into_computed`] only.
    #[derive(Debug, Clone)]
    pub struct ComputedDuration;

    /// A trait that represents anything we'll accept for specifying durations
    /// for revalidation. Anything that implements thus must be able to be
    /// transformed into a [`ComputedDuration`];
    pub trait PerseusDuration {
        /// Converts this into a `[ComputedDuration]` for use in Perseus'
        /// internal revalidation systems.
        fn into_computed(self) -> Result<ComputedDuration, InvalidDuration>
        where
            Self: Sized,
        {
            // In the browser, this function should never be called
            unreachable!("computed durations can only be created on the engine-side")
        }
    }

    // Dummy implementations for the browser follow (we only need this for generic
    // validation)
    impl PerseusDuration for &str {}
    impl PerseusDuration for time::Duration {}
}

/// An error type for invalid durations.
#[derive(Debug)]
pub struct InvalidDuration;

#[cfg(target_arch = "wasm32")]
pub use browser::{ComputedDuration, PerseusDuration};
#[cfg(not(target_arch = "wasm32"))]
pub use engine::{ComputedDuration, PerseusDuration};

// // We can convert from our duration type into the standard library's
// impl From<Duration> for time::Duration {
//     fn from(duration: Duration) -> Self {
//         let duration = chrono::Duration::seconds(duration.seconds)
//             + chrono::Duration::minutes(duration.minutes)
//             + chrono::Duration::hours(duration.hours)
//             + chrono::Duration::days(duration.days)
//             + chrono::Duration::weeks(duration.weeks)
//             + chrono::Duration::days(duration.months * 30)  // Assumed length
// of a month             + chrono::Duration::days(duration.years * 365); //
// Assumed length of a year

//         duration.to_std().unwrap()
//     }
// }
