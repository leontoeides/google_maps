use crate::roads::nearest_roads::request::Request;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Builds the query string for the Google Maps Nearest Roads request based
    /// on the input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &'a mut Request {
        // This section builds the "required parameters" portion of the query
        // string:

        // Convert `Vec<LatLng>` to `String`:
        let points: String = self
            .points
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("|");

        // URL encode path `String`:
        let points: String = utf8_percent_encode(&points, NON_ALPHANUMERIC).to_string();

        // Build "required parameters" portion of the query string:
        let query = format!("key={key}&points={points}", key = self.client.key,);

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self
    } // fn
} // impl
