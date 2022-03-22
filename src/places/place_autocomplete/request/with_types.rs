use crate::places::place_autocomplete::request::AutocompleteType;
use crate::places::place_autocomplete::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the types parameter to the Place API _Place Autocomplete_ query.
    ///
    /// ## Arguments:
    ///
    /// * `types` ‧ You may restrict results from a Place Autocomplete request to 
    /// be of a certain type by passing a types parameter. The parameter 
    /// specifies a type or a type collection, as listed in the supported types 
    /// below. If nothing is specified, all types are returned. In general only 
    /// a single type is allowed. The exception is that you can safely mix the 
    /// geocode and establishment types, but note that this will have the same 
    /// effect as specifying no types.
    ///
    /// * Multiple result type filters may be stacked together.

    pub fn with_type(&'a mut self, autocomplete_type: AutocompleteType) -> &'a mut Request {
        // Set types in Request struct.
        self.types.extend(vec![autocomplete_type]);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the types parameter to the Place API _Place Autocomplete_ query.
    ///
    /// ## Arguments:
    ///
    /// * `types` ‧ You may restrict results from a Place Autocomplete request to
    /// be of a certain type by passing a types parameter. The parameter
    /// specifies a type or a type collection, as listed in the supported types
    /// below. If nothing is specified, all types are returned. In general only
    /// a single type is allowed. The exception is that you can safely mix the
    /// geocode and establishment types, but note that this will have the same
    /// effect as specifying no types.
    ///
    /// * Multiple result type filters may be stacked together.

    pub fn with_types(&'a mut self, types: Vec<AutocompleteType>) -> &'a mut Request {
        // Set types in Request struct.
        self.types.extend(types);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl