use crate::request_rate::duration_unit::DurationUnit;
use std::time::Duration;

/// Converts a `std::time::Duration` into an English expression of time.
///
/// ## Arguments
///
/// * `duration` ‧ The `std::time::Duration` that is to be converted into a
///   string.
///
/// ## Description
///
/// A `Duration` is a unit of time for the Rust programming language. This
/// function converts a Duration into an English expression of time. For
/// example, "1 month," "5.83 minutes," or "948 milliseconds." The unit of time
/// (i.e. milliseconds or seconds) is automatically selected by this function.

pub fn duration_to_string(duration: &Duration) -> String {
    const SECONDS_IN_A_SECOND: f64 = 1.0;
    const SECONDS_IN_A_MINUTE: f64 = 60.0;
    const SECONDS_IN_AN_HOUR: f64 = 3_600.0;
    const SECONDS_IN_A_DAY: f64 = 86_400.0;
    const SECONDS_IN_A_WEEK: f64 = 604_800.0;
    const SECONDS_IN_A_MONTH: f64 = 2_629_746.0;
    const SECONDS_IN_A_YEAR: f64 = 31_556_952.0;

    // Multiplication is much faster than division:

    const MILLISECONDS_IN_A_SECOND: f64 = 1.0 / 0.001;
    const MINUTES_IN_A_SECOND: f64 = 1.0 / 60.0;
    const HOURS_IN_A_SECOND: f64 = 1.0 / 3_600.0;
    const DAYS_IN_A_SECOND: f64 = 1.0 / 86_400.0;
    const WEEKS_IN_A_SECOND: f64 = 1.0 / 604_800.0;
    const MONTHS_IN_A_SECOND: f64 = 1.0 / 2_629_746.0;
    const YEARS_IN_A_SECOND: f64 = 1.0 / 31_556_952.0;

    // This match takes the duration passed by the caller and adjusts the
    // time/duration unit for better readability when presenting to an end user.
    // It returns a tuple with - the adjusted duration quantity in index 0, and
    // the duration unit of time in index 1:

    let duration_in_secs = duration.as_secs_f64();

    let adjusted_units = match duration_in_secs {
        s if s < SECONDS_IN_A_SECOND => (s * MILLISECONDS_IN_A_SECOND, DurationUnit::Milliseconds),
        s if s < SECONDS_IN_A_MINUTE => (s * SECONDS_IN_A_SECOND, DurationUnit::Seconds),
        s if s < SECONDS_IN_AN_HOUR => (s * MINUTES_IN_A_SECOND, DurationUnit::Minutes),
        s if s < SECONDS_IN_A_DAY => (s * HOURS_IN_A_SECOND, DurationUnit::Hours),
        s if s < SECONDS_IN_A_WEEK => (s * DAYS_IN_A_SECOND, DurationUnit::Days),
        s if s < SECONDS_IN_A_MONTH => (s * WEEKS_IN_A_SECOND, DurationUnit::Weeks),
        s if s < SECONDS_IN_A_YEAR => (s * MONTHS_IN_A_SECOND, DurationUnit::Months),
        _ => (duration_in_secs * YEARS_IN_A_SECOND, DurationUnit::Years),
    }; // match

    // The fractional portion of a large value (i.e. 40075.14159) is less
    // significant compared to the same fractional portion of a tiny value
    // (i.e. 3.14159). This match suppresses the fractional portion for large
    // values and shows more of the fractional portion for small values:

    let mut quantity_string = match adjusted_units.0 {
        q if q < 0.001 => format!("{q:.6}"),
        q if q < 0.01 => format!("{q:.5}"),
        q if q < 0.1 => format!("{q:.4}"),
        q if q < 1.0 => format!("{q:.3}"),
        q if q < 10.0 => format!("{q:.2}"),
        q if q < 100.0 => format!("{q:.1}"),
        _ => format!("{:.0}", adjusted_units.0),
    }; // match

    // If the value has a fractional part, remove any insignificant digits:

    if quantity_string.contains('.') {
        quantity_string = quantity_string.trim_end_matches('0').to_string();
        quantity_string = quantity_string.trim_end_matches('.').to_string();
    }

    // Returns the unit of time enum into a string that can be presented to the
    // user. It also returns the time unit's noun in singular if the value is
    // "1", and in plural if it is not.

    let units_string = if quantity_string == "1" {
        match adjusted_units.1 {
            DurationUnit::Days => String::from("day"),
            DurationUnit::Hours => String::from("hour"),
            DurationUnit::Milliseconds => String::from("millisecond"),
            DurationUnit::Minutes => String::from("minute"),
            DurationUnit::Months => String::from("month"),
            DurationUnit::Seconds => String::from("second"),
            DurationUnit::Weeks => String::from("week"),
            DurationUnit::Years => String::from("year"),
            // This can never be reached but it keeps the Rust compiler happy:
            _ => adjusted_units.1.to_string(),
        } // match
    } else {
        match adjusted_units.1 {
            DurationUnit::Days => String::from("days"),
            DurationUnit::Hours => String::from("hours"),
            DurationUnit::Milliseconds => String::from("milliseconds"),
            DurationUnit::Minutes => String::from("minutes"),
            DurationUnit::Months => String::from("months"),
            DurationUnit::Seconds => String::from("seconds"),
            DurationUnit::Weeks => String::from("weeks"),
            DurationUnit::Years => String::from("years"),
            // This can never be reached but it keeps the Rust compiler happy:
            _ => adjusted_units.1.to_string(),
        } // match
    }; // if

    // Formats the final string and returns it to the caller:

    quantity_string + " " + &units_string
} // fn
