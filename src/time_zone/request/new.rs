use crate::types::LatLng;
use chrono::{DateTime, Utc};

// =============================================================================

impl<'r> crate::time_zone::request::Request<'r> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Time Zone API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `location` ‧ Latitude & longitude of the desired time zone location.
    ///
    /// * `timestamp` ‧ Time is used to determine if Daylight Savings is
    ///   applicable.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use google_maps::prelude::TimeZoneRequest;
    /// use google_maps::{LatLng, NaiveDate};
    ///
    /// let time_zone = TimeZoneRequest::new(
    ///     &my_settings,
    ///     // St. Vitus Cathedral in Prague, Czechia
    ///     LatLng::try_from_dec(50.090_903, 14.400_512)?,
    ///     // Tuesday February 15, 2022 @ 6:00:00 pm
    ///     NaiveDate::from_ymd(2022, 2, 15).and_hms(18, 00, 0)
    /// ).execute();
    /// ```
    #[must_use]
    pub const fn new(
        client: &'r crate::Client,
        location: LatLng,
        timestamp: DateTime<Utc>
    ) -> Self {
        // Instantiate struct and return it to caller:
        Self {
            // Required parameters:
            client,
            location,
            timestamp,
            // Optional parameters:
            language: None,
        } // struct
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Time Zone API query with the
    /// required, non-optional parameters.
    ///
    /// This function is the same as `new` but it supports
    /// the [geo](https://crates.io/crates/geo) crate's
    /// [Coord](https://docs.rs/geo/latest/geo/geometry/struct.Coord.html) type.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    /// * `coordinate` - `Coord` of the desired time zone location.
    /// * `timestamp` - Time is used to determine if Daylight Savings is
    ///   applicable.
    #[cfg(feature = "geo")]
    pub fn try_new_coordinate<'g>(
        client: &'r crate::Client,
        coordinate: &'g geo_types::Coord,
        timestamp: DateTime<Utc>
    ) -> Result<Self, crate::error::Error> {
        // Instantiate struct and return it to caller:
        Ok(Self {
            // Required parameters:
            client,
            location: LatLng::try_from(coordinate)?,
            timestamp,
            // Optional parameters:
            language: None,
        }) // struct
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Time Zone API query with the
    /// required, non-optional parameters.
    ///
    /// This function is the same as `new` but it supports
    /// the [geo](https://crates.io/crates/geo) crate's
    /// [Point](https://docs.rs/geo/latest/geo/geometry/struct.Point.html) type.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    /// * `point` - `Point` of the desired time zone location.
    /// * `timestamp` - Time is used to determine if Daylight Savings is
    ///   applicable.
    #[cfg(feature = "geo")]
    pub fn try_new_point<'g>(
        client: &'r crate::Client,
        point: &'g geo_types::Point,
        timestamp: DateTime<Utc>
    ) -> Result<Self, crate::error::Error> {
        // Instantiate struct and return it to caller:
        Ok(Self {
            // Required parameters:
            client,
            location: LatLng::try_from(point)?,
            timestamp,
            // Optional parameters:
            language: None,
        }) // struct
    } // fn
} // impl