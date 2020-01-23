use time::PrimitiveDateTime;
use crate::{
    latlng::LatLng,
    time_zone::request::Request,
}; // use

impl Request {

    /// Initializes the builder pattern for a Time Zone API query with the
    /// required, non-optional parameters.
    ///
    /// # Arguments:
    ///
    /// * `key` - Your application's Google Cloud API key.
    /// * `location` - Latitude & longitude of the desired time zone location.
    /// * `time` - Time is used to determine if Daylight Savings is applicable.
    ///
    /// # Example:
    ///
    /// ```
    /// let time_zone = Request::new(
    ///     // St. Vitus Cathedral in Prague, Czechia
    ///     LatLng { lat: 50.090903, lng: 14.400512 },
    ///     PrimitiveDateTime::new(
    ///         // Tuesday February 15, 2022
    ///         Date::try_from_ymd(2022, 2, 15).unwrap(),
    ///         // 6:00:00 pm
    ///         Time::try_from_hms(18, 00, 0).unwrap(),
    ///     ),
    ///     // Your Google API Key
    ///     GOOGLE_API_KEY,
    /// // Build query string and then perform HTTP get request
    /// ).build().get();
    /// ```

    pub fn new(key: String, location: LatLng, time: PrimitiveDateTime) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            key,
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