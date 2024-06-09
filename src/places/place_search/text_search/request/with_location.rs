use crate::places::place_search::text_search::request::Request;
use crate::types::LatLng;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the location and radius parameters to the Places API _Text Search_
    /// query.
    ///
    /// ## Arguments
    ///
    /// * `location` ‧ The point around which to retrieve place information.
    ///   This must be specified as `latitude,longitude`.
    ///
    /// The `location` parameter may be overriden if the `query` contains an
    /// explicit location such as `Market in Barcelona`. Using quotes around the
    /// query may also influence the weight given to the `location` and
    /// `radius`.

    pub fn with_location(
        &'a mut self,
        location: impl Into<LatLng>
    ) -> &'a mut Self {
        // Set location in Request struct.
        self.location = Some(location.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
