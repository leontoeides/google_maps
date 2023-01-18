use crate::place_type::PlaceType;
use crate::places::place_search::text_search::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the types parameter to the Places API _Text Search_ query.
    ///
    /// ## Arguments:
    ///
    /// * `type` ‧ Restricts the results to places matching the specified type.
    /// Only one type may be specified. If more than one type is provided, all
    /// types following the first entry are ignored.
    ///
    /// `type=hospital|pharmacy|doctor` becomes `type=hospital`
    /// `type=hospital,pharmacy,doctor` is ignored entirely
    ///
    /// See the list of [supported types](https://developers.google.com/maps/documentation/places/web-service/supported_types).
    ///
    /// Note: Adding both `keyword` and `type` with the same value
    /// (`keyword=cafe&type=cafe` or `keyword=parking&type=parking`) can yield
    /// `ZERO_RESULTS`.

    pub fn with_type(&'a mut self, place_type: PlaceType) -> &'a mut Request {
        // Set location in Request struct.
        self.place_types = Some(vec![place_type]);
        // Return modified Request struct to caller.
        self
    } // fn

    /// Adds the types parameter to the Places API _Text Search_ query.
    ///
    /// ## Arguments:
    ///
    /// * `type` ‧ Restricts the results to places matching the specified type.
    /// Only one type may be specified. If more than one type is provided, all
    /// types following the first entry are ignored.
    ///
    /// `type=hospital|pharmacy|doctor` becomes `type=hospital`
    /// `type=hospital,pharmacy,doctor` is ignored entirely
    ///
    /// See the list of [supported types](https://developers.google.com/maps/documentation/places/web-service/supported_types).
    ///
    /// Note: Adding both `keyword` and `type` with the same value
    /// (`keyword=cafe&type=cafe` or `keyword=parking&type=parking`) can yield
    /// `ZERO_RESULTS`.

    pub fn with_types(&'a mut self, place_types_slice: &[PlaceType]) -> &'a mut Request {
        // Add place_types to Request struct.
        match &mut self.place_types {
            // If there are no filters in the request struct, initialize field:
            None => self.place_types = Some(place_types_slice.to_vec()),
            // If there are already filters, append to them:
            Some(place_types) => place_types_slice.iter().for_each(|component|
                place_types.push(component.to_owned())
            ), // iter
        } // match
        // Return modified Request struct to caller.
        self
    } // fn

} // impl