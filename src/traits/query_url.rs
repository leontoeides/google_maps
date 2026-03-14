// -----------------------------------------------------------------------------
//
/// Provides a way of converting a request `struct` (such as
/// `crate::directions::Request` or `crate::elevation::Request`) to a
/// [URL](https://en.wikipedia.org/wiki/Uniform_Resource_Locator) that
/// can be used as an [HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP)
/// [GET](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET) request.
pub trait QueryUrl {
    /// Converts a request `struct` (presumably a request type such as
    /// `crate::directions::Request`) to a
    /// [URL](https://en.wikipedia.org/wiki/Uniform_Resource_Locator) that
    /// can be used as an [HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP)
    /// [GET](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET)
    /// request.
    ///
    /// # Errors
    ///
    /// * This can fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Directions API cannot calculate alternative
    ///   routes if waypoints have been set. This will cause a validation
    ///   failure.
    fn query_url(&self) -> Result<String, crate::Error>;
} // trait QueryUrl

// -----------------------------------------------------------------------------

impl<T> QueryUrl for T
where
    T: crate::traits::Validatable + crate::traits::QueryString + crate::traits::EndPoint,
{
    /// Returns the URL query string that represents the request you've built.
    ///
    /// ## Description
    ///
    /// Returns the URL that will be used as the query to the Google Maps API.
    ///
    /// It is the result of the request builder pattern.
    ///
    /// This method can also be useful for records or logging. It can also be
    /// used for passing to your HTTP client of choice and executing the HTTP
    /// `GET` request yourself.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// # Errors
    ///
    /// * This can fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Directions API cannot calculate alternative
    ///   routes if waypoints have been set. This will cause a validation
    ///   failure.
    fn query_url(&self) -> Result<String, crate::Error> {
        // Validate the request before attempting to build a URL query string:
        self.validate()?;

        // If the request passes validation, build the full URL that will be
        // used as the query:
        #[allow(clippy::option_if_let_else)] // map_or_else is illegible
        if let Some(output_format) = T::output_format() {
            // An output format (i.e. `JSON` or `XML`) was defined for the
            // end-point. Render it in the query URL:
            Ok(format!(
                "{service_url}/{output_format}?{query_string}",
                service_url = T::service_url(),
                query_string = self.query_string()
            ))
        } else {
            // No output format was defined for the end-point. Don't render it
            // in the query URL:
            Ok(format!(
                "{service_url}?{query_string}",
                service_url = T::service_url(),
                query_string = self.query_string()
            ))
        } // if
    } // fn
} // impl