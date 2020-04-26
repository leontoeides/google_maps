use crate::{client_settings::ClientSettings, latlng::LatLng, time_zone::request::Request};
use chrono::{DateTime, Utc}; // use

impl<'a> Request<'a> {
    /// Initializes the builder pattern for a Time Zone API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `key` - Your application's Google Cloud API key.
    /// * `location` - Latitude & longitude of the desired time zone location.
    /// * `timestamp` - Time is used to determine if Daylight Savings is
    /// applicable.
    ///
    /// ## Example:
    ///
    /// ```rust
    /// let time_zone = TimeZoneRequest::new(
    ///     &mut my_settings,
    ///     // St. Vitus Cathedral in Prague, Czechia
    ///     LatLng::try_from(50.090_903, 14.400_512).unwrap(),
    ///     // Tuesday February 15, 2022 @ 6:00:00 pm
    ///     NaiveDate::from_ymd(2022, 2, 15).and_hms(18, 00, 0)
    /// ).execute();
    /// ```

    pub fn new(
        client_settings: &mut ClientSettings,
        location: LatLng,
        timestamp: DateTime<Utc>,
    ) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client_settings,
            location,
            timestamp,
            // Optional parameters:
            language: None,
            // Internal use only:
            query: None,
            validated: false,
        } // struct
    } // fn
} // impl
