//! Human-readable duration
//!
//! human-duration converts a [`std::time::Duration`] to a human readable string.
//!
//! Examples:
//!
//! ```
//! use human_duration::human_duration;
//!
//! let duration = std::time::Duration::new(120, 30_000_000);
//! assert_eq!(human_duration(&duration), "2m 0s 30ms");
//!
//! let duration = std::time::Duration::new(9000, 0);
//! assert_eq!(human_duration(&duration), "2h 30m 0s 0ms");
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::time::Duration;

static SECONDS_PER_YEAR: u64 = 31_536_000;
static SECONDS_PER_MONTH: u64 = 2_628_000;
static SECONDS_PER_DAY: u64 = 86400;
static SECONDS_PER_HOUR: u64 = 3600;
static SECONDS_PER_MINUTE: u64 = 60;

/// Takes a [`std::time::Duration`] and returns a formatted [`String`].
///
/// Examples:
/// ```
/// use human_duration::human_duration;
///
/// let duration = std::time::Duration::new(5, 0);
/// assert_eq!(human_duration(&duration), "5s 0ms");
///
/// let duration = std::time::Duration::new(125, 0);
/// assert_eq!(human_duration(&duration), "2m 5s 0ms");
///
/// let duration = std::time::Duration::new(45_000_000, 0);
/// assert_eq!(human_duration(&duration), "1y 5mon 3d 18h 0m 0s 0ms");
/// ```
pub fn human_duration(duration: &Duration) -> String {
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();

    let years = seconds / SECONDS_PER_YEAR;
    let mut remainder = seconds % SECONDS_PER_YEAR;

    let months = remainder / SECONDS_PER_MONTH;
    remainder %= SECONDS_PER_MONTH;

    let days = remainder / SECONDS_PER_DAY;
    remainder %= SECONDS_PER_DAY;

    let hours = remainder / SECONDS_PER_HOUR;
    remainder %= SECONDS_PER_HOUR;

    let minutes = remainder / SECONDS_PER_MINUTE;
    remainder %= SECONDS_PER_MINUTE;

    let mut output = Vec::with_capacity(7);

    if years > 0 {
        let f_years = format!("{years}y");
        output.push(f_years);
    }

    if !output.is_empty() || months > 0 {
        let f_months = format!("{months}mon");
        output.push(f_months);
    }

    if !output.is_empty() || days > 0 {
        let f_days = format!("{days}d");
        output.push(f_days);
    }

    if !output.is_empty() || hours > 0 {
        let f_hours = format!("{hours}h");
        output.push(f_hours);
    }

    if !output.is_empty() || minutes > 0 {
        let f_minutes = format!("{minutes}m");
        output.push(f_minutes);
    }

    if !output.is_empty() || seconds > 0 {
        let f_seconds = format!("{remainder}s");
        output.push(f_seconds);
    }

    let f_millis = format!("{millis}ms");
    output.push(f_millis);

    output.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_duration() {
        let result = human_duration(&std::time::Duration::new(0, 0));
        assert_eq!(result, "0ms");

        let result = human_duration(&std::time::Duration::new(30, 5_000_000));
        assert_eq!(result, "30s 5ms");

        let result = human_duration(&std::time::Duration::new(60, 50_000_000));
        assert_eq!(result, "1m 0s 50ms");

        let result = human_duration(&std::time::Duration::new(92, 0));
        assert_eq!(result, "1m 32s 0ms");

        let result = human_duration(&std::time::Duration::new(3600, 0));
        assert_eq!(result, "1h 0m 0s 0ms");

        let result = human_duration(&std::time::Duration::new(3666, 0));
        assert_eq!(result, "1h 1m 6s 0ms");

        let result = human_duration(&std::time::Duration::new(86400, 337_000_000));
        assert_eq!(result, "1d 0h 0m 0s 337ms");

        let result = human_duration(&std::time::Duration::new(86680, 0));
        assert_eq!(result, "1d 0h 4m 40s 0ms");

        let result = human_duration(&std::time::Duration::new(2_628_000, 0));
        assert_eq!(result, "1mon 0d 0h 0m 0s 0ms");

        let result = human_duration(&std::time::Duration::new(2_828_000, 0));
        assert_eq!(result, "1mon 2d 7h 33m 20s 0ms");

        let result = human_duration(&std::time::Duration::new(31_536_000, 0));
        assert_eq!(result, "1y 0mon 0d 0h 0m 0s 0ms");

        let result = human_duration(&std::time::Duration::new(34_536_000, 0));
        assert_eq!(result, "1y 1mon 4d 7h 20m 0s 0ms");
    }
}
