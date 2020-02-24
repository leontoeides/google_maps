use chrono::NaiveDateTime;
use crate::{
    client_settings::ClientSettings,
    latlng::LatLng,
    time_zone::request::Request,
}; // use

impl<'a> Request<'a> {

    /// Initializes the builder pattern for a Time Zone API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `key` - Your application's Google Cloud API key.
    /// * `location` - Latitude & longitude of the desired time zone location.
    /// * `time` - Time is used to determine if Daylight Savings is applicable.
    ///
    /// ## Example:
    ///
    /// ```rust
    /// let time_zone = TimeZoneRequest::new(
    ///     &mut my_settings,
    ///     // St. Vitus Cathedral in Prague, Czechia
    ///     LatLng::try_from(50.090_903, 14.400_512).unwrap(),
    ///     PrimitiveDateTime::new(
    ///         // Tuesday February 15, 2022
    ///         Date::try_from_ymd(2022, 2, 15).unwrap(),
    ///         // 6:00:00 pm
    ///         Time::try_from_hms(18, 00, 0).unwrap(),
    ///     ),
    /// )
    /// ```

    pub fn new(client_settings: &mut ClientSettings, location: LatLng, time: NaiveDateTime) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client_settings,
            location,
            time,
            // Optional parameters:
            language: None,
            // Internal use only:
            query: None,
            validated: false,
        } // struct
    } // fn

} // impl