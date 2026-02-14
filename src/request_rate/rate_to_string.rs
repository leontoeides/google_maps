use crate::request_rate::duration_unit::DurationUnit;
use std::time::Duration;

/// Converts a numerator and denominator into an English expression of a rate.
///
/// Takes a count (like number of requests or files) and a time duration, then
/// produces a human-readable string like "5.83 pages per minute" or "1 file per
/// month". The function automatically selects the most appropriate time unit
/// for readability—preferring "3 requests per second" over "180 requests per
/// minute".
///
/// ## Arguments
///
/// * `numerator` - The count portion of the rate (e.g., number of files,
///   requests, or bytes).
///
/// * `duration` - The time period over which the count occurs.
///
/// * `numerator_singular` - Singular form for display (e.g., "request").
///
/// * `numerator_plural` - Plural form for display (e.g., "requests").
pub fn rate_to_string(
    numerator: u64,
    duration: &Duration,
    numerator_singular: &str,
    numerator_plural: &str,
) -> String {
    // Seconds per time unit—used to convert the duration into various units
    // so we can pick the most human-readable one.
    const SECS_PER_MILLISECOND: f64 = 0.001;
    const SECS_PER_SECOND: f64 = 1.0;
    const SECS_PER_MINUTE: f64 = 60.0;
    const SECS_PER_HOUR: f64 = 3_600.0;
    const SECS_PER_DAY: f64 = 86_400.0;
    const SECS_PER_WEEK: f64 = 604_800.0;
    const SECS_PER_MONTH: f64 = 2_629_746.0;
    const SECS_PER_YEAR: f64 = 31_556_952.0;

    let duration_secs = duration.as_secs_f64();
    let numerator = numerator as f64;

    // Compute the rate (numerator per time-unit) for each possible unit.
    // We want the smallest unit that still gives us a rate >= 1.0 for
    // readability—"3 requests per second" beats "0.05 requests per minute".
    let rate_per = |secs_per_unit: f64| numerator * secs_per_unit / duration_secs;

    let (rate, unit) = if rate_per(SECS_PER_MILLISECOND) >= 1.0 {
        (rate_per(SECS_PER_MILLISECOND), DurationUnit::Milliseconds)
    } else if rate_per(SECS_PER_SECOND) >= 1.0 {
        (rate_per(SECS_PER_SECOND), DurationUnit::Seconds)
    } else if rate_per(SECS_PER_MINUTE) >= 1.0 {
        (rate_per(SECS_PER_MINUTE), DurationUnit::Minutes)
    } else if rate_per(SECS_PER_HOUR) >= 1.0 {
        (rate_per(SECS_PER_HOUR), DurationUnit::Hours)
    } else if rate_per(SECS_PER_DAY) >= 1.0 {
        (rate_per(SECS_PER_DAY), DurationUnit::Days)
    } else if rate_per(SECS_PER_WEEK) >= 1.0 {
        (rate_per(SECS_PER_WEEK), DurationUnit::Weeks)
    } else if rate_per(SECS_PER_MONTH) >= 1.0 {
        (rate_per(SECS_PER_MONTH), DurationUnit::Months)
    } else {
        (rate_per(SECS_PER_YEAR), DurationUnit::Years)
    };

    // Format the quantity with appropriate precision. Large values don't need
    // many decimal places, while small values benefit from more precision.
    let quantity = match rate {
        r if r < 0.001 => format!("{r:.6}"),
        r if r < 0.01 => format!("{r:.5}"),
        r if r < 0.1 => format!("{r:.4}"),
        r if r < 1.0 => format!("{r:.3}"),
        r if r < 10.0 => format!("{r:.2}"),
        r if r < 100.0 => format!("{r:.1}"),
        r => format!("{r:.0}"),
    };

    // Strip trailing zeros and a trailing decimal point for cleaner output.
    // "3.50" becomes "3.5", and "3.0" becomes "3".
    let quantity = quantity
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();

    // Use singular form only for exactly "1", otherwise plural.
    // "1.5 requests" is correct, not "1.5 request".
    let rate_noun = if quantity == "1" {
        numerator_singular
    } else {
        numerator_plural
    };

    // Convert the duration unit enum to its singular noun form.
    let unit_noun = match unit {
        DurationUnit::Milliseconds => "millisecond",
        DurationUnit::Seconds => "second",
        DurationUnit::Minutes => "minute",
        DurationUnit::Hours => "hour",
        DurationUnit::Days => "day",
        DurationUnit::Weeks => "week",
        DurationUnit::Months => "month",
        DurationUnit::Years => "year",
    };

    format!("{quantity} {rate_noun} per {unit_noun}")
}