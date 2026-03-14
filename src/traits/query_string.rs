// -----------------------------------------------------------------------------
//
/// Provides a way of converting a request `struct` (such as
/// `crate::directions::Request` or `crate::elevation::Request`) to a
/// [URL](https://en.wikipedia.org/wiki/Uniform_Resource_Locator) 
/// [query string](https://en.wikipedia.org/wiki/Query_string) that
/// can be used in an [HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP)
/// [GET](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET) request.
pub trait QueryString {
    /// Converts a request `struct` (presumably a request type such as
    /// `crate::directions::Request`) to a
    /// [URL](https://en.wikipedia.org/wiki/Uniform_Resource_Locator) 
    /// [query string](https://en.wikipedia.org/wiki/Query_string) that
    /// can be used in an [HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP)
    /// [GET](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET)
    /// request.
    ///
    /// ## Notes
    ///
    /// * This function does not validate the request before generating the
    ///   _query string_. However, the superior method that generates the _query
    ///   URL_ does perform validation.
    ///
    /// * The query string is the part of the URL after the `?` question mark.
    ///   For example, in the URL `https://example.com/over/there?name=ferret`
    ///   the query string is `name=ferret`
    ///
    /// * There's no benefit to working on an owned `Request` struct (i.e. an
    ///   owned `self` versus an borrowed `&self`).
    ///   [percent-encoding](https://crates.io/crates/percent-encoding)
    ///   works on borrowed UTF-8 strings. Other types, such as enums and
    ///   numeric values are converted into strings. Therefore no zero-copy
    ///   operations are possible with an owned `self`.
    fn query_string(&self) -> String;
} // trait QueryString