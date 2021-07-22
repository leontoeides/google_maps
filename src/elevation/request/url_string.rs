use crate::elevation::{
    OUTPUT_FORMAT,
    SERVICE_URL,
    error::Error,
    request::Request,
}; // crate::elevation

impl<'a> Request<'a> {

    /// Returns the URL query string that represents the query you've built.
    ///
    /// ## Description:
    ///
    /// Returns the query string that will be sent to the Google Maps API. It
    /// is the result of the builder pattern. This method could be useful for
    /// records or logging. It could also be used for passing to your HTTP
    /// client of choice and executing the HTTP GET request yourself.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn url_string(&'a mut self) -> Result<String, Error> {
        let query_string = match &self.query {
            // If query string has already been built, return it:
            Some(query_string) => query_string,
            // If it hasn't been built, build it:
            None => self.validate()?.build()?.query.as_ref().unwrap(),
        }; // match
        Ok(format!("{}/{}?{}", SERVICE_URL, OUTPUT_FORMAT, query_string))
    } // fn

} // impl