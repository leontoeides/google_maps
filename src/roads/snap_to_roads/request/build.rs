use crate::roads::snap_to_roads::request::Request;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Builds the query string for the Google Maps Snap to Roads request based
    /// on the input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &'a mut Request {
        // This section builds the "required parameters" portion of the query
        // string:

        // Convert `Vec<LatLng>` to `String`:
        let path: String = self
            .path
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("|");

        // URL encode path `String`:
        let path: String = utf8_percent_encode(&path, NON_ALPHANUMERIC).to_string();

        // Build "required parameters" portion of the query string:
        let mut query = format!("key={key}&path={path}", key = self.client.key,);

        // This section builds the "optional parameters" portion of the query
        // string:

        // Language key/value pair:
        if let Some(interpolate) = &self.interpolate {
            query.push_str("&interpolate=");
            query.push_str(&interpolate.to_string())
        }

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self
    } // fn
} // impl
