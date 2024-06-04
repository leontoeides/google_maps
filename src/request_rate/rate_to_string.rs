use crate::request_rate::duration_unit::DurationUnit;
use std::time::Duration;

/// Converts a numerator and denominator into an English expression of a rate.
///
/// ## Arguments
///
/// * `numerator` ‧ The numerator of the rate. i.e. _Files_ per second,
///   _requests_ per minute, etc.
///
/// * `duration` ‧ The denominator of the rate. i.e. Bytes per _second_,
///   pages per _hour_, etc.
///
/// * `numerator_singular` ‧ The singular form of the numerator's name as a
///   string to be displayed to the end-user. For example, `"request"` or
///   `"file"`.
///
/// * `numerator_plural` ‧ The plural form of the numerator's name as a string
///   to be displayed to the end-user. For example, `"bytes"` or `"queries"`.
///
/// ## Description
///
/// This function converts a numerator and Duration into an English expression
/// of a rate. For example, "1 file per month," "5.83 pages a minute," etc. The
/// unit of time (i.e. milliseconds or seconds) is automatically selected by
/// this function.

pub fn rate_to_string(
    numerator: u64,
    duration: &Duration,
    numerator_singular: &str,
    numerator_plural: &str,
) -> String {
    // Multiplication is much faster than division:

    const MILLISECONDS_IN_A_SECOND: f64 = 1.0 / 0.001;
    const SECONDS_IN_A_SECOND: f64 = 1.0;
    const MINUTES_IN_A_SECOND: f64 = 1.0 / 60.0;
    const HOURS_IN_A_SECOND: f64 = 1.0 / 3_600.0;
    const DAYS_IN_A_SECOND: f64 = 1.0 / 86_400.0;
    const WEEKS_IN_A_SECOND: f64 = 1.0 / 604_800.0;
    const MONTHS_IN_A_SECOND: f64 = 1.0 / 2_629_746.0;
    const YEARS_IN_A_SECOND: f64 = 1.0 / 31_556_952.0;

    // This match takes the duration passed by the caller and adjusts the
    // time/duration unit for better readability when presenting to an end user.
    // This match structure finds the smallest time unit that results in a value
    // over 1.0. It returns a tuple with - the adjusted duration quantity in
    // index 0, and the duration unit of time in index 1:

    let duration_in_secs = duration.as_secs_f64();
    let numerator = numerator as f64;

    let adjusted_units = match duration_in_secs {
        s if numerator / (s * MILLISECONDS_IN_A_SECOND) > 1.0 => (
            numerator / (s * MILLISECONDS_IN_A_SECOND),
            DurationUnit::Milliseconds,
        ),
        s if numerator / (s * SECONDS_IN_A_SECOND) > 1.0 => {
            (numerator / (s * SECONDS_IN_A_SECOND), DurationUnit::Seconds)
        }
        s if numerator / (s * MINUTES_IN_A_SECOND) > 1.0 => {
            (numerator / (s * MINUTES_IN_A_SECOND), DurationUnit::Minutes)
        }
        s if numerator / (s * HOURS_IN_A_SECOND) > 1.0 => {
            (numerator / (s * HOURS_IN_A_SECOND), DurationUnit::Hours)
        }
        s if numerator / (s * DAYS_IN_A_SECOND) > 1.0 => {
            (numerator / (s * DAYS_IN_A_SECOND), DurationUnit::Days)
        }
        s if numerator / (s * WEEKS_IN_A_SECOND) > 1.0 => {
            (numerator / (s * WEEKS_IN_A_SECOND), DurationUnit::Weeks)
        }
        s if numerator / (s * MONTHS_IN_A_SECOND) > 1.0 => {
            (numerator / (s * MONTHS_IN_A_SECOND), DurationUnit::Months)
        }
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

    // The rate type. For example it could be "_bytes_ per second," "_file_ per
    // minute." It will return singular if the quantity is exactly one. If the
    // quantity is not 1, or a fractional 1, it returns plural.

    let rate_type_string = if quantity_string == "1" {
        numerator_singular
    } else {
        numerator_plural
    }; // if

    // Returns the unit of time enum into a string that can be presented to the
    // user. It also returns the time unit's noun in singular if the value is
    // "1", and in plural if it is not.

    let units_string = match adjusted_units.1 {
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
    }; // match

    // Formats the final string and returns it to the caller:

    quantity_string + " " + rate_type_string + " per " + &units_string
} // fn
