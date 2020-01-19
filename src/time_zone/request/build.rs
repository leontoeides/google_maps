use crate::time_zone::request::Request;

impl Request {

    /// Builds the query string for the Google Maps Time Zone API based on the
    /// input provided by the client.
    ///
    /// # Example:
    ///
    /// ```
    /// let time_zone = Request::new(
    ///     // Eiffel Tower in Paris, France
    ///     LatLng { lat: 48.858379, lng: 2.294481 },
    ///     PrimitiveDateTime::new(
    ///         // Wednesday June 4, 2025
    ///         Date::try_from_ymd(2025, 6, 4).unwrap(),
    ///         // 8:30:00 am
    ///         Time::try_from_hms(8, 30, 0).unwrap(),
    ///     ),
    ///     // // Your Google API Key
    ///     GOOGLE_API_KEY,
    /// // Build query string and then perform HTTP get request
    /// ).build().get();
    /// ```

    pub fn build(&mut self) -> &mut Request {

        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = format!(
            "location={}&timestamp={}&key={}",
            String::from(&self.location),
            self.time.timestamp(),
            self.key
        );

        // This section builds the "optional parameters" portion of the query
        // string:

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        }

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self

    } // fn

} // impl