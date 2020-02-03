use crate::rate_limit::duration_unit::DurationUnit;
use std::time::Duration;

pub fn rate_to_string(requests: u64, duration: Duration) -> String {

    let time_in_secs = duration.as_secs_f64();

    const MILLISECOND_IN_SECS: f64 = 0.001;
    const SECOND_IN_SECS: f64 = 1.0;
    const MINUTE_IN_SECS: f64 = 60.0;
    const HOUR_IN_SECS: f64 = 3_600.0;
    const DAY_IN_SECS: f64 = 86_400.0;
    const WEEK_IN_SECS: f64 = 604_800.0;
    const MONTH_IN_SECS: f64 = 2_629_746.0;
    const YEAR_IN_SECS: f64 = 31_556_952.0;

    // This match takes the duration passed by the caller and adjusts the
    // time/duration unit for better readability when presenting to an end user.
    // It returns a tuple with - the adjusted duration quantity in index 0, and
    // the duration unit of time in index 1:

    let adjusted_units = match time_in_secs {
        s if s < SECOND_IN_SECS => (requests as f64 / s * MILLISECOND_IN_SECS, DurationUnit::Milliseconds),
        s if s < MINUTE_IN_SECS => (requests as f64 / s * SECOND_IN_SECS, DurationUnit::Seconds),
        s if s < HOUR_IN_SECS => (requests as f64 / s * MINUTE_IN_SECS, DurationUnit::Minutes),
        s if s < DAY_IN_SECS => (requests as f64 / s * HOUR_IN_SECS, DurationUnit::Hours),
        s if s < WEEK_IN_SECS => (requests as f64 / s * DAY_IN_SECS, DurationUnit::Days),
        s if s < MONTH_IN_SECS => (requests as f64 / s * WEEK_IN_SECS, DurationUnit::Weeks),
        s if s < YEAR_IN_SECS => (requests as f64 / s * MONTH_IN_SECS, DurationUnit::Months),
        _ => (time_in_secs / YEAR_IN_SECS, DurationUnit::Years),
    }; // match

    // The fractional portion of a large value (i.e. 40075.14159) is less
    // significant compared to the same fractional portion of a tiny value
    // (i.e. 3.14159). This match suppresses the fractional portion for large
    // values and shows more of the fractional portion for small values:

    let mut quantity_string = match adjusted_units.0 {
        q if q < 0.001 => format!("{:.6}", q),
        q if q < 0.01 => format!("{:.5}", q),
        q if q < 0.1 => format!("{:.4}", q),
        q if q < 1.0 => format!("{:.3}", q),
        q if q < 10.0 => format!("{:.2}", q),
        q if q < 100.0 => format!("{:.1}", q),
        _ => format!("{:.0}", adjusted_units.0),
    }; // match

    // If the value has a fractional part, remove any insignificant digits:

    if quantity_string.contains(".") {
        quantity_string = quantity_string.trim_end_matches(|c| c == '0' || c == '.').to_string();
    }

    //

    let rate_type_string = if quantity_string == "1" {
        String::from("request")
    } else {
        String::from("requests")
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
        _ => String::from(&adjusted_units.1),
    }; // match

    // Formats the final string and returns it to the caller:

    quantity_string + &" " + &rate_type_string + &" per " + &units_string

} // fn