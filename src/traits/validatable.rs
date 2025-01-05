// -----------------------------------------------------------------------------
//
/// Provides a way of validating a request `struct` (such as
/// `crate::directions::Request` or `crate::elevation::Request`) before the
/// request is convered into a [URL](https://en.wikipedia.org/wiki/Uniform_Resource_Locator)
/// [query string](https://en.wikipedia.org/wiki/Query_string) that
/// can be used in an [HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP)
/// [GET](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET) request.
pub trait Validatable {
    /// This function checks the combination of parameters to ensure that they
    /// make sense together and that Google Maps API will accept them.
    ///
    /// For example, Google Maps will not allow both a Positional Request and a
    /// Sampled Path Request in the same query.
    ///
    /// This function does not check parameter values for validity - i.e. it
    /// will not ensure Polylines or Latitudes/Longitudes are valid and
    /// well-formed.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// # Errors
    ///
    /// * This will fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Directions API cannot calculate alternative
    ///   routes if waypoints have been set. This will cause a validation
    ///   failure.
    fn validate(&self) -> Result<(), crate::Error>;
} // trait Validatable