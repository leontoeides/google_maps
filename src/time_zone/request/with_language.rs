use crate::{
    language::Language,
    time_zone::request::Request,
}; // use

impl Request {

    /// Adds the language parameter to the Time Zone API query.
    ///
    /// # Arguments:
    ///
    /// * `language` - The language that the response will be presented in.
    ///
    /// # Example:
    ///
    /// ```
    /// let time_zone = Request::new(
    ///     // Monument to the Battle of the Nations in Leipzig, Germany
    ///     LatLng { lat: 51.312378, lng: 12.413269 },
    ///     PrimitiveDateTime::new(
    ///         // Saturday October 5, 2024
    ///         Date::try_from_ymd(2024, 10, 5).unwrap(),
    ///         // 1:30:00 pm
    ///         Time::try_from_hms(13, 30, 0).unwrap(),
    ///     ),
    ///     // Your Google API Key
    ///     GOOGLE_API_KEY,
    /// // Set Google's response to the French language
    /// ).with_language(Language::French)
    /// // Build query string and then perform HTTP get request
    /// .build().get();
    /// ```

    pub fn with_language(&mut self, language: Language) -> &mut Request {
        // Set language in Request struct.
        self.language = Some(language);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl